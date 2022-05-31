#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CIMTYPE_ENUMERATION(pub i32);
pub const CIM_ILLEGAL: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(4095i32);
pub const CIM_EMPTY: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(0i32);
pub const CIM_SINT8: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(16i32);
pub const CIM_UINT8: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(17i32);
pub const CIM_SINT16: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(2i32);
pub const CIM_UINT16: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(18i32);
pub const CIM_SINT32: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(3i32);
pub const CIM_UINT32: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(19i32);
pub const CIM_SINT64: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(20i32);
pub const CIM_UINT64: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(21i32);
pub const CIM_REAL32: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(4i32);
pub const CIM_REAL64: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(5i32);
pub const CIM_BOOLEAN: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(11i32);
pub const CIM_STRING: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(8i32);
pub const CIM_DATETIME: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(101i32);
pub const CIM_REFERENCE: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(102i32);
pub const CIM_CHAR16: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(103i32);
pub const CIM_OBJECT: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(13i32);
pub const CIM_FLAG_ARRAY: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(8192i32);
impl ::core::marker::Copy for CIMTYPE_ENUMERATION {}
impl ::core::clone::Clone for CIMTYPE_ENUMERATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CIMTYPE_ENUMERATION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CIMTYPE_ENUMERATION {
    type Abi = Self;
}
impl ::core::fmt::Debug for CIMTYPE_ENUMERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CIMTYPE_ENUMERATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
pub struct IEnumWbemClassObject(::windows_core::IUnknown);
impl IEnumWbemClassObject {
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Next(&self, ltimeout: i32, apobjects: &mut [::core::option::Option<IWbemClassObject>], pureturned: *mut u32) -> ::windows_core::HRESULT {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ltimeout), apobjects.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(apobjects)), ::core::mem::transmute(pureturned)))
    }
    pub unsafe fn NextAsync<'a, Param1: ::windows_core::IntoParam<'a, IWbemObjectSink>>(&self, ucount: u32, psink: Param1) -> ::windows_core::HRESULT {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).NextAsync)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ucount), psink.into_param().abi()))
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumWbemClassObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumWbemClassObject>(result__)
    }
    pub unsafe fn Skip(&self, ltimeout: i32, ncount: u32) -> ::windows_core::HRESULT {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ltimeout), ::core::mem::transmute(ncount)))
    }
}
impl ::core::convert::From<IEnumWbemClassObject> for ::windows_core::IUnknown {
    fn from(value: IEnumWbemClassObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumWbemClassObject> for ::windows_core::IUnknown {
    fn from(value: &IEnumWbemClassObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumWbemClassObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumWbemClassObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumWbemClassObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumWbemClassObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumWbemClassObject {}
impl ::core::fmt::Debug for IEnumWbemClassObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumWbemClassObject").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumWbemClassObject {
    type Vtable = IEnumWbemClassObject_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x027947e1_d731_11ce_a357_000000000001);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumWbemClassObject_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ltimeout: i32, ucount: u32, apobjects: *mut ::windows_core::RawPtr, pureturned: *mut u32) -> ::windows_core::HRESULT,
    pub NextAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ucount: u32, psink: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ltimeout: i32, ncount: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IMofCompiler(::windows_core::IUnknown);
impl IMofCompiler {
    pub unsafe fn CompileFile<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param4: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, filename: Param0, serverandnamespace: Param1, user: Param2, authority: Param3, password: Param4, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CompileFile)(::windows_core::Interface::as_raw(self), filename.into_param().abi(), serverandnamespace.into_param().abi(), user.into_param().abi(), authority.into_param().abi(), password.into_param().abi(), ::core::mem::transmute(loptionflags), ::core::mem::transmute(lclassflags), ::core::mem::transmute(linstanceflags), ::core::mem::transmute(pinfo)).ok()
    }
    pub unsafe fn CompileBuffer<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param4: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param5: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, buffsize: i32, pbuffer: *const u8, serverandnamespace: Param2, user: Param3, authority: Param4, password: Param5, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CompileBuffer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(buffsize), ::core::mem::transmute(pbuffer), serverandnamespace.into_param().abi(), user.into_param().abi(), authority.into_param().abi(), password.into_param().abi(), ::core::mem::transmute(loptionflags), ::core::mem::transmute(lclassflags), ::core::mem::transmute(linstanceflags), ::core::mem::transmute(pinfo)).ok()
    }
    pub unsafe fn CreateBMOF<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, textfilename: Param0, bmoffilename: Param1, serverandnamespace: Param2, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreateBMOF)(::windows_core::Interface::as_raw(self), textfilename.into_param().abi(), bmoffilename.into_param().abi(), serverandnamespace.into_param().abi(), ::core::mem::transmute(loptionflags), ::core::mem::transmute(lclassflags), ::core::mem::transmute(linstanceflags), ::core::mem::transmute(pinfo)).ok()
    }
}
impl ::core::convert::From<IMofCompiler> for ::windows_core::IUnknown {
    fn from(value: IMofCompiler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMofCompiler> for ::windows_core::IUnknown {
    fn from(value: &IMofCompiler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMofCompiler {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMofCompiler {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMofCompiler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMofCompiler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMofCompiler {}
impl ::core::fmt::Debug for IMofCompiler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMofCompiler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IMofCompiler {
    type Vtable = IMofCompiler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6daf974e_2e37_11d2_aec9_00c04fb68820);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMofCompiler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub CompileFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, serverandnamespace: ::windows_core::PCWSTR, user: ::windows_core::PCWSTR, authority: ::windows_core::PCWSTR, password: ::windows_core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows_core::HRESULT,
    pub CompileBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffsize: i32, pbuffer: *const u8, serverandnamespace: ::windows_core::PCWSTR, user: ::windows_core::PCWSTR, authority: ::windows_core::PCWSTR, password: ::windows_core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows_core::HRESULT,
    pub CreateBMOF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, textfilename: ::windows_core::PCWSTR, bmoffilename: ::windows_core::PCWSTR, serverandnamespace: ::windows_core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ISWbemDateTime(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ISWbemDateTime {
    pub unsafe fn Value(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Value)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetValue<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, strvalue: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetValue)(::windows_core::Interface::as_raw(self), strvalue.into_param().abi()).ok()
    }
    pub unsafe fn Year(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Year)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetYear(&self, iyear: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetYear)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iyear)).ok()
    }
    pub unsafe fn YearSpecified(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).YearSpecified)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetYearSpecified(&self, byearspecified: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetYearSpecified)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(byearspecified)).ok()
    }
    pub unsafe fn Month(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Month)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMonth(&self, imonth: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMonth)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(imonth)).ok()
    }
    pub unsafe fn MonthSpecified(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).MonthSpecified)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetMonthSpecified(&self, bmonthspecified: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMonthSpecified)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(bmonthspecified)).ok()
    }
    pub unsafe fn Day(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Day)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetDay(&self, iday: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDay)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iday)).ok()
    }
    pub unsafe fn DaySpecified(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).DaySpecified)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetDaySpecified(&self, bdayspecified: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDaySpecified)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(bdayspecified)).ok()
    }
    pub unsafe fn Hours(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Hours)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetHours(&self, ihours: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetHours)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ihours)).ok()
    }
    pub unsafe fn HoursSpecified(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).HoursSpecified)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetHoursSpecified(&self, bhoursspecified: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetHoursSpecified)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(bhoursspecified)).ok()
    }
    pub unsafe fn Minutes(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Minutes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMinutes(&self, iminutes: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMinutes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iminutes)).ok()
    }
    pub unsafe fn MinutesSpecified(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).MinutesSpecified)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetMinutesSpecified(&self, bminutesspecified: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMinutesSpecified)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(bminutesspecified)).ok()
    }
    pub unsafe fn Seconds(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Seconds)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetSeconds(&self, iseconds: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSeconds)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iseconds)).ok()
    }
    pub unsafe fn SecondsSpecified(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).SecondsSpecified)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetSecondsSpecified(&self, bsecondsspecified: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSecondsSpecified)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(bsecondsspecified)).ok()
    }
    pub unsafe fn Microseconds(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Microseconds)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMicroseconds(&self, imicroseconds: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMicroseconds)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(imicroseconds)).ok()
    }
    pub unsafe fn MicrosecondsSpecified(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).MicrosecondsSpecified)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetMicrosecondsSpecified(&self, bmicrosecondsspecified: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMicrosecondsSpecified)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(bmicrosecondsspecified)).ok()
    }
    pub unsafe fn UTC(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).UTC)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetUTC(&self, iutc: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetUTC)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iutc)).ok()
    }
    pub unsafe fn UTCSpecified(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).UTCSpecified)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetUTCSpecified(&self, butcspecified: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetUTCSpecified)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(butcspecified)).ok()
    }
    pub unsafe fn IsInterval(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsInterval)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetIsInterval(&self, bisinterval: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIsInterval)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(bisinterval)).ok()
    }
    pub unsafe fn GetVarDate(&self, bislocal: i16) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).GetVarDate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(bislocal), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn SetVarDate(&self, dvardate: f64, bislocal: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetVarDate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dvardate), ::core::mem::transmute(bislocal)).ok()
    }
    pub unsafe fn GetFileTime(&self, bislocal: i16) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetFileTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(bislocal), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetFileTime<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, strfiletime: Param0, bislocal: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFileTime)(::windows_core::Interface::as_raw(self), strfiletime.into_param().abi(), ::core::mem::transmute(bislocal)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemDateTime> for ::windows_core::IUnknown {
    fn from(value: ISWbemDateTime) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemDateTime> for ::windows_core::IUnknown {
    fn from(value: &ISWbemDateTime) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISWbemDateTime {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISWbemDateTime {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemDateTime> for super::Com::IDispatch {
    fn from(value: ISWbemDateTime) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemDateTime> for super::Com::IDispatch {
    fn from(value: &ISWbemDateTime) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ISWbemDateTime {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ISWbemDateTime {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ISWbemDateTime {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ISWbemDateTime {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ISWbemDateTime {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ISWbemDateTime {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemDateTime").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ISWbemDateTime {
    type Vtable = ISWbemDateTime_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5e97458a_cf77_11d3_b38f_00105a1f473a);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemDateTime_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strvalue: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strvalue: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Year: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iyear: *mut i32) -> ::windows_core::HRESULT,
    pub SetYear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iyear: i32) -> ::windows_core::HRESULT,
    pub YearSpecified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, byearspecified: *mut i16) -> ::windows_core::HRESULT,
    pub SetYearSpecified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, byearspecified: i16) -> ::windows_core::HRESULT,
    pub Month: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imonth: *mut i32) -> ::windows_core::HRESULT,
    pub SetMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imonth: i32) -> ::windows_core::HRESULT,
    pub MonthSpecified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmonthspecified: *mut i16) -> ::windows_core::HRESULT,
    pub SetMonthSpecified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmonthspecified: i16) -> ::windows_core::HRESULT,
    pub Day: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iday: *mut i32) -> ::windows_core::HRESULT,
    pub SetDay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iday: i32) -> ::windows_core::HRESULT,
    pub DaySpecified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bdayspecified: *mut i16) -> ::windows_core::HRESULT,
    pub SetDaySpecified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bdayspecified: i16) -> ::windows_core::HRESULT,
    pub Hours: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ihours: *mut i32) -> ::windows_core::HRESULT,
    pub SetHours: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ihours: i32) -> ::windows_core::HRESULT,
    pub HoursSpecified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bhoursspecified: *mut i16) -> ::windows_core::HRESULT,
    pub SetHoursSpecified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bhoursspecified: i16) -> ::windows_core::HRESULT,
    pub Minutes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iminutes: *mut i32) -> ::windows_core::HRESULT,
    pub SetMinutes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iminutes: i32) -> ::windows_core::HRESULT,
    pub MinutesSpecified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bminutesspecified: *mut i16) -> ::windows_core::HRESULT,
    pub SetMinutesSpecified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bminutesspecified: i16) -> ::windows_core::HRESULT,
    pub Seconds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iseconds: *mut i32) -> ::windows_core::HRESULT,
    pub SetSeconds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iseconds: i32) -> ::windows_core::HRESULT,
    pub SecondsSpecified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bsecondsspecified: *mut i16) -> ::windows_core::HRESULT,
    pub SetSecondsSpecified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bsecondsspecified: i16) -> ::windows_core::HRESULT,
    pub Microseconds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imicroseconds: *mut i32) -> ::windows_core::HRESULT,
    pub SetMicroseconds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imicroseconds: i32) -> ::windows_core::HRESULT,
    pub MicrosecondsSpecified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmicrosecondsspecified: *mut i16) -> ::windows_core::HRESULT,
    pub SetMicrosecondsSpecified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmicrosecondsspecified: i16) -> ::windows_core::HRESULT,
    pub UTC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iutc: *mut i32) -> ::windows_core::HRESULT,
    pub SetUTC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iutc: i32) -> ::windows_core::HRESULT,
    pub UTCSpecified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, butcspecified: *mut i16) -> ::windows_core::HRESULT,
    pub SetUTCSpecified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, butcspecified: i16) -> ::windows_core::HRESULT,
    pub IsInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bisinterval: *mut i16) -> ::windows_core::HRESULT,
    pub SetIsInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bisinterval: i16) -> ::windows_core::HRESULT,
    pub GetVarDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bislocal: i16, dvardate: *mut f64) -> ::windows_core::HRESULT,
    pub SetVarDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dvardate: f64, bislocal: i16) -> ::windows_core::HRESULT,
    pub GetFileTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bislocal: i16, strfiletime: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetFileTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strfiletime: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bislocal: i16) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ISWbemEventSource(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ISWbemEventSource {
    #[cfg(feature = "win32-system")]
    pub unsafe fn NextEvent(&self, itimeoutms: i32) -> ::windows_core::Result<ISWbemObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).NextEvent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(itimeoutms), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObject>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Security_(&self) -> ::windows_core::Result<ISWbemSecurity> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Security_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemSecurity>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemEventSource> for ::windows_core::IUnknown {
    fn from(value: ISWbemEventSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemEventSource> for ::windows_core::IUnknown {
    fn from(value: &ISWbemEventSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISWbemEventSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISWbemEventSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemEventSource> for super::Com::IDispatch {
    fn from(value: ISWbemEventSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemEventSource> for super::Com::IDispatch {
    fn from(value: &ISWbemEventSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ISWbemEventSource {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ISWbemEventSource {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ISWbemEventSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ISWbemEventSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ISWbemEventSource {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ISWbemEventSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemEventSource").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ISWbemEventSource {
    type Vtable = ISWbemEventSource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27d54d92_0ebe_11d2_8b22_00600806d9b6);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemEventSource_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub NextEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itimeoutms: i32, objwbemobject: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    NextEvent: usize,
    #[cfg(feature = "win32-system")]
    pub Security_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Security_: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ISWbemLastError(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ISWbemLastError {
    #[cfg(feature = "win32-system")]
    pub unsafe fn Put_<'a, Param1: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, iflags: i32, objwbemnamedvalueset: Param1) -> ::windows_core::Result<ISWbemObjectPath> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Put_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObjectPath>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn PutAsync_<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param2: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param3: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, iflags: i32, objwbemnamedvalueset: Param2, objwbemasynccontext: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.PutAsync_)(::windows_core::Interface::as_raw(self), objwbemsink.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Delete_<'a, Param1: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, iflags: i32, objwbemnamedvalueset: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Delete_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn DeleteAsync_<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param2: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param3: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, iflags: i32, objwbemnamedvalueset: Param2, objwbemasynccontext: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteAsync_)(::windows_core::Interface::as_raw(self), objwbemsink.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Instances_<'a, Param1: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, iflags: i32, objwbemnamedvalueset: Param1) -> ::windows_core::Result<ISWbemObjectSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Instances_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObjectSet>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn InstancesAsync_<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param2: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param3: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, iflags: i32, objwbemnamedvalueset: Param2, objwbemasynccontext: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.InstancesAsync_)(::windows_core::Interface::as_raw(self), objwbemsink.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Subclasses_<'a, Param1: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, iflags: i32, objwbemnamedvalueset: Param1) -> ::windows_core::Result<ISWbemObjectSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Subclasses_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObjectSet>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SubclassesAsync_<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param2: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param3: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, iflags: i32, objwbemnamedvalueset: Param2, objwbemasynccontext: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SubclassesAsync_)(::windows_core::Interface::as_raw(self), objwbemsink.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Associators_<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param6: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param7: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param9: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, strassocclass: Param0, strresultclass: Param1, strresultrole: Param2, strrole: Param3, bclassesonly: i16, bschemaonly: i16, strrequiredassocqualifier: Param6, strrequiredqualifier: Param7, iflags: i32, objwbemnamedvalueset: Param9) -> ::windows_core::Result<ISWbemObjectSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Associators_)(::windows_core::Interface::as_raw(self), strassocclass.into_param().abi(), strresultclass.into_param().abi(), strresultrole.into_param().abi(), strrole.into_param().abi(), ::core::mem::transmute(bclassesonly), ::core::mem::transmute(bschemaonly), strrequiredassocqualifier.into_param().abi(), strrequiredqualifier.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObjectSet>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn AssociatorsAsync_<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param7: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param8: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param10: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param11: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(
        &self,
        objwbemsink: Param0,
        strassocclass: Param1,
        strresultclass: Param2,
        strresultrole: Param3,
        strrole: Param4,
        bclassesonly: i16,
        bschemaonly: i16,
        strrequiredassocqualifier: Param7,
        strrequiredqualifier: Param8,
        iflags: i32,
        objwbemnamedvalueset: Param10,
        objwbemasynccontext: Param11,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AssociatorsAsync_)(::windows_core::Interface::as_raw(self), objwbemsink.into_param().abi(), strassocclass.into_param().abi(), strresultclass.into_param().abi(), strresultrole.into_param().abi(), strrole.into_param().abi(), ::core::mem::transmute(bclassesonly), ::core::mem::transmute(bschemaonly), strrequiredassocqualifier.into_param().abi(), strrequiredqualifier.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn References_<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param6: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, strresultclass: Param0, strrole: Param1, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: Param4, iflags: i32, objwbemnamedvalueset: Param6) -> ::windows_core::Result<ISWbemObjectSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.References_)(::windows_core::Interface::as_raw(self), strresultclass.into_param().abi(), strrole.into_param().abi(), ::core::mem::transmute(bclassesonly), ::core::mem::transmute(bschemaonly), strrequiredqualifier.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObjectSet>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ReferencesAsync_<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param5: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param7: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param8: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strresultclass: Param1, strrole: Param2, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: Param5, iflags: i32, objwbemnamedvalueset: Param7, objwbemasynccontext: Param8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ReferencesAsync_)(::windows_core::Interface::as_raw(self), objwbemsink.into_param().abi(), strresultclass.into_param().abi(), strrole.into_param().abi(), ::core::mem::transmute(bclassesonly), ::core::mem::transmute(bschemaonly), strrequiredqualifier.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ExecMethod_<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param3: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, strmethodname: Param0, objwbeminparameters: Param1, iflags: i32, objwbemnamedvalueset: Param3) -> ::windows_core::Result<ISWbemObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ExecMethod_)(::windows_core::Interface::as_raw(self), strmethodname.into_param().abi(), objwbeminparameters.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObject>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ExecMethodAsync_<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param4: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param5: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strmethodname: Param1, objwbeminparameters: Param2, iflags: i32, objwbemnamedvalueset: Param4, objwbemasynccontext: Param5) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ExecMethodAsync_)(::windows_core::Interface::as_raw(self), objwbemsink.into_param().abi(), strmethodname.into_param().abi(), objwbeminparameters.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Clone_(&self) -> ::windows_core::Result<ISWbemObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Clone_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObject>(result__)
    }
    pub unsafe fn GetObjectText_(&self, iflags: i32) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetObjectText_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SpawnDerivedClass_(&self, iflags: i32) -> ::windows_core::Result<ISWbemObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SpawnDerivedClass_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObject>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SpawnInstance_(&self, iflags: i32) -> ::windows_core::Result<ISWbemObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SpawnInstance_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObject>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CompareTo_<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemobject: Param0, iflags: i32) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CompareTo_)(::windows_core::Interface::as_raw(self), objwbemobject.into_param().abi(), ::core::mem::transmute(iflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Qualifiers_(&self) -> ::windows_core::Result<ISWbemQualifierSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Qualifiers_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemQualifierSet>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Properties_(&self) -> ::windows_core::Result<ISWbemPropertySet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Properties_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemPropertySet>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Methods_(&self) -> ::windows_core::Result<ISWbemMethodSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Methods_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemMethodSet>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Derivation_(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Derivation_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Path_(&self) -> ::windows_core::Result<ISWbemObjectPath> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Path_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObjectPath>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Security_(&self) -> ::windows_core::Result<ISWbemSecurity> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Security_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemSecurity>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemLastError> for ::windows_core::IUnknown {
    fn from(value: ISWbemLastError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemLastError> for ::windows_core::IUnknown {
    fn from(value: &ISWbemLastError) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISWbemLastError {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISWbemLastError {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemLastError> for super::Com::IDispatch {
    fn from(value: ISWbemLastError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemLastError> for super::Com::IDispatch {
    fn from(value: &ISWbemLastError) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ISWbemLastError {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ISWbemLastError {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemLastError> for ISWbemObject {
    fn from(value: ISWbemLastError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemLastError> for ISWbemObject {
    fn from(value: &ISWbemLastError) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ISWbemObject> for ISWbemLastError {
    fn into_param(self) -> ::windows_core::Param<'a, ISWbemObject> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ISWbemObject> for &'a ISWbemLastError {
    fn into_param(self) -> ::windows_core::Param<'a, ISWbemObject> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ISWbemLastError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ISWbemLastError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ISWbemLastError {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ISWbemLastError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemLastError").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ISWbemLastError {
    type Vtable = ISWbemLastError_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd962db84_d4bb_11d1_8b09_00600806d9b6);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemLastError_Vtbl {
    pub base__: ISWbemObject_Vtbl,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ISWbemLocator(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ISWbemLocator {
    #[cfg(feature = "win32-system")]
    pub unsafe fn ConnectServer<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param5: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param7: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, strserver: Param0, strnamespace: Param1, struser: Param2, strpassword: Param3, strlocale: Param4, strauthority: Param5, isecurityflags: i32, objwbemnamedvalueset: Param7) -> ::windows_core::Result<ISWbemServices> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ConnectServer)(::windows_core::Interface::as_raw(self), strserver.into_param().abi(), strnamespace.into_param().abi(), struser.into_param().abi(), strpassword.into_param().abi(), strlocale.into_param().abi(), strauthority.into_param().abi(), ::core::mem::transmute(isecurityflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemServices>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Security_(&self) -> ::windows_core::Result<ISWbemSecurity> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Security_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemSecurity>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemLocator> for ::windows_core::IUnknown {
    fn from(value: ISWbemLocator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemLocator> for ::windows_core::IUnknown {
    fn from(value: &ISWbemLocator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISWbemLocator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISWbemLocator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemLocator> for super::Com::IDispatch {
    fn from(value: ISWbemLocator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemLocator> for super::Com::IDispatch {
    fn from(value: &ISWbemLocator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ISWbemLocator {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ISWbemLocator {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ISWbemLocator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ISWbemLocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ISWbemLocator {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ISWbemLocator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemLocator").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ISWbemLocator {
    type Vtable = ISWbemLocator_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x76a6415b_cb41_11d1_8b02_00600806d9b6);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemLocator_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub ConnectServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strserver: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strnamespace: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, struser: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strlocale: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strauthority: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, isecurityflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr, objwbemservices: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ConnectServer: usize,
    #[cfg(feature = "win32-system")]
    pub Security_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Security_: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ISWbemMethod(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ISWbemMethod {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Origin(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Origin)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn InParameters(&self) -> ::windows_core::Result<ISWbemObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).InParameters)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObject>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn OutParameters(&self) -> ::windows_core::Result<ISWbemObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).OutParameters)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObject>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Qualifiers_(&self) -> ::windows_core::Result<ISWbemQualifierSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Qualifiers_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemQualifierSet>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemMethod> for ::windows_core::IUnknown {
    fn from(value: ISWbemMethod) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemMethod> for ::windows_core::IUnknown {
    fn from(value: &ISWbemMethod) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISWbemMethod {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISWbemMethod {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemMethod> for super::Com::IDispatch {
    fn from(value: ISWbemMethod) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemMethod> for super::Com::IDispatch {
    fn from(value: &ISWbemMethod) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ISWbemMethod {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ISWbemMethod {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ISWbemMethod {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ISWbemMethod {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ISWbemMethod {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ISWbemMethod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemMethod").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ISWbemMethod {
    type Vtable = ISWbemMethod_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x422e8e90_d955_11d1_8b09_00600806d9b6);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemMethod_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Origin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strorigin: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub InParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbeminparameters: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    InParameters: usize,
    #[cfg(feature = "win32-system")]
    pub OutParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemoutparameters: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    OutParameters: usize,
    #[cfg(feature = "win32-system")]
    pub Qualifiers_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemqualifierset: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Qualifiers_: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ISWbemMethodSet(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ISWbemMethodSet {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Item<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, strname: Param0, iflags: i32) -> ::windows_core::Result<ISWbemMethod> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), strname.into_param().abi(), ::core::mem::transmute(iflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemMethod>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemMethodSet> for ::windows_core::IUnknown {
    fn from(value: ISWbemMethodSet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemMethodSet> for ::windows_core::IUnknown {
    fn from(value: &ISWbemMethodSet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISWbemMethodSet {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISWbemMethodSet {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemMethodSet> for super::Com::IDispatch {
    fn from(value: ISWbemMethodSet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemMethodSet> for super::Com::IDispatch {
    fn from(value: &ISWbemMethodSet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ISWbemMethodSet {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ISWbemMethodSet {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ISWbemMethodSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ISWbemMethodSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ISWbemMethodSet {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ISWbemMethodSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemMethodSet").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ISWbemMethodSet {
    type Vtable = ISWbemMethodSet_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc93ba292_d955_11d1_8b09_00600806d9b6);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemMethodSet_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, iflags: i32, objwbemmethod: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ISWbemNamedValue(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ISWbemNamedValue {
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Value(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Value)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetValue(&self, varvalue: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(varvalue)).ok()
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemNamedValue> for ::windows_core::IUnknown {
    fn from(value: ISWbemNamedValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemNamedValue> for ::windows_core::IUnknown {
    fn from(value: &ISWbemNamedValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISWbemNamedValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISWbemNamedValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemNamedValue> for super::Com::IDispatch {
    fn from(value: ISWbemNamedValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemNamedValue> for super::Com::IDispatch {
    fn from(value: &ISWbemNamedValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ISWbemNamedValue {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ISWbemNamedValue {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ISWbemNamedValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ISWbemNamedValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ISWbemNamedValue {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ISWbemNamedValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemNamedValue").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ISWbemNamedValue {
    type Vtable = ISWbemNamedValue_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x76a64164_cb41_11d1_8b02_00600806d9b6);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemNamedValue_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varvalue: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Value: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varvalue: *const super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetValue: usize,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ISWbemNamedValueSet(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ISWbemNamedValueSet {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Item<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, strname: Param0, iflags: i32) -> ::windows_core::Result<ISWbemNamedValue> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), strname.into_param().abi(), ::core::mem::transmute(iflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemNamedValue>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Add<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, strname: Param0, varvalue: *const super::Com::VARIANT, iflags: i32) -> ::windows_core::Result<ISWbemNamedValue> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), strname.into_param().abi(), ::core::mem::transmute(varvalue), ::core::mem::transmute(iflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemNamedValue>(result__)
    }
    pub unsafe fn Remove<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, strname: Param0, iflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), strname.into_param().abi(), ::core::mem::transmute(iflags)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Clone(&self) -> ::windows_core::Result<ISWbemNamedValueSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemNamedValueSet>(result__)
    }
    pub unsafe fn DeleteAll(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteAll)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemNamedValueSet> for ::windows_core::IUnknown {
    fn from(value: ISWbemNamedValueSet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemNamedValueSet> for ::windows_core::IUnknown {
    fn from(value: &ISWbemNamedValueSet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISWbemNamedValueSet {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISWbemNamedValueSet {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemNamedValueSet> for super::Com::IDispatch {
    fn from(value: ISWbemNamedValueSet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemNamedValueSet> for super::Com::IDispatch {
    fn from(value: &ISWbemNamedValueSet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ISWbemNamedValueSet {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ISWbemNamedValueSet {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ISWbemNamedValueSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ISWbemNamedValueSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ISWbemNamedValueSet {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ISWbemNamedValueSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemNamedValueSet").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ISWbemNamedValueSet {
    type Vtable = ISWbemNamedValueSet_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcf2376ea_ce8c_11d1_8b05_00600806d9b6);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemNamedValueSet_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, iflags: i32, objwbemnamedvalue: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varvalue: *const super::Com::VARIANT, iflags: i32, objwbemnamedvalue: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, iflags: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemnamedvalueset: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Clone: usize,
    pub DeleteAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ISWbemObject(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ISWbemObject {
    #[cfg(feature = "win32-system")]
    pub unsafe fn Put_<'a, Param1: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, iflags: i32, objwbemnamedvalueset: Param1) -> ::windows_core::Result<ISWbemObjectPath> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Put_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObjectPath>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn PutAsync_<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param2: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param3: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, iflags: i32, objwbemnamedvalueset: Param2, objwbemasynccontext: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PutAsync_)(::windows_core::Interface::as_raw(self), objwbemsink.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Delete_<'a, Param1: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, iflags: i32, objwbemnamedvalueset: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn DeleteAsync_<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param2: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param3: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, iflags: i32, objwbemnamedvalueset: Param2, objwbemasynccontext: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteAsync_)(::windows_core::Interface::as_raw(self), objwbemsink.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Instances_<'a, Param1: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, iflags: i32, objwbemnamedvalueset: Param1) -> ::windows_core::Result<ISWbemObjectSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Instances_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObjectSet>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn InstancesAsync_<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param2: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param3: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, iflags: i32, objwbemnamedvalueset: Param2, objwbemasynccontext: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InstancesAsync_)(::windows_core::Interface::as_raw(self), objwbemsink.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Subclasses_<'a, Param1: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, iflags: i32, objwbemnamedvalueset: Param1) -> ::windows_core::Result<ISWbemObjectSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Subclasses_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObjectSet>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SubclassesAsync_<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param2: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param3: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, iflags: i32, objwbemnamedvalueset: Param2, objwbemasynccontext: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SubclassesAsync_)(::windows_core::Interface::as_raw(self), objwbemsink.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Associators_<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param6: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param7: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param9: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, strassocclass: Param0, strresultclass: Param1, strresultrole: Param2, strrole: Param3, bclassesonly: i16, bschemaonly: i16, strrequiredassocqualifier: Param6, strrequiredqualifier: Param7, iflags: i32, objwbemnamedvalueset: Param9) -> ::windows_core::Result<ISWbemObjectSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Associators_)(::windows_core::Interface::as_raw(self), strassocclass.into_param().abi(), strresultclass.into_param().abi(), strresultrole.into_param().abi(), strrole.into_param().abi(), ::core::mem::transmute(bclassesonly), ::core::mem::transmute(bschemaonly), strrequiredassocqualifier.into_param().abi(), strrequiredqualifier.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObjectSet>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn AssociatorsAsync_<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param7: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param8: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param10: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param11: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(
        &self,
        objwbemsink: Param0,
        strassocclass: Param1,
        strresultclass: Param2,
        strresultrole: Param3,
        strrole: Param4,
        bclassesonly: i16,
        bschemaonly: i16,
        strrequiredassocqualifier: Param7,
        strrequiredqualifier: Param8,
        iflags: i32,
        objwbemnamedvalueset: Param10,
        objwbemasynccontext: Param11,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AssociatorsAsync_)(::windows_core::Interface::as_raw(self), objwbemsink.into_param().abi(), strassocclass.into_param().abi(), strresultclass.into_param().abi(), strresultrole.into_param().abi(), strrole.into_param().abi(), ::core::mem::transmute(bclassesonly), ::core::mem::transmute(bschemaonly), strrequiredassocqualifier.into_param().abi(), strrequiredqualifier.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn References_<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param6: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, strresultclass: Param0, strrole: Param1, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: Param4, iflags: i32, objwbemnamedvalueset: Param6) -> ::windows_core::Result<ISWbemObjectSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).References_)(::windows_core::Interface::as_raw(self), strresultclass.into_param().abi(), strrole.into_param().abi(), ::core::mem::transmute(bclassesonly), ::core::mem::transmute(bschemaonly), strrequiredqualifier.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObjectSet>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ReferencesAsync_<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param5: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param7: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param8: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strresultclass: Param1, strrole: Param2, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: Param5, iflags: i32, objwbemnamedvalueset: Param7, objwbemasynccontext: Param8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReferencesAsync_)(::windows_core::Interface::as_raw(self), objwbemsink.into_param().abi(), strresultclass.into_param().abi(), strrole.into_param().abi(), ::core::mem::transmute(bclassesonly), ::core::mem::transmute(bschemaonly), strrequiredqualifier.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ExecMethod_<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param3: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, strmethodname: Param0, objwbeminparameters: Param1, iflags: i32, objwbemnamedvalueset: Param3) -> ::windows_core::Result<ISWbemObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ExecMethod_)(::windows_core::Interface::as_raw(self), strmethodname.into_param().abi(), objwbeminparameters.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObject>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ExecMethodAsync_<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param4: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param5: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strmethodname: Param1, objwbeminparameters: Param2, iflags: i32, objwbemnamedvalueset: Param4, objwbemasynccontext: Param5) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ExecMethodAsync_)(::windows_core::Interface::as_raw(self), objwbemsink.into_param().abi(), strmethodname.into_param().abi(), objwbeminparameters.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Clone_(&self) -> ::windows_core::Result<ISWbemObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObject>(result__)
    }
    pub unsafe fn GetObjectText_(&self, iflags: i32) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetObjectText_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SpawnDerivedClass_(&self, iflags: i32) -> ::windows_core::Result<ISWbemObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).SpawnDerivedClass_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObject>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SpawnInstance_(&self, iflags: i32) -> ::windows_core::Result<ISWbemObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).SpawnInstance_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObject>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CompareTo_<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemobject: Param0, iflags: i32) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).CompareTo_)(::windows_core::Interface::as_raw(self), objwbemobject.into_param().abi(), ::core::mem::transmute(iflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Qualifiers_(&self) -> ::windows_core::Result<ISWbemQualifierSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Qualifiers_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemQualifierSet>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Properties_(&self) -> ::windows_core::Result<ISWbemPropertySet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Properties_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemPropertySet>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Methods_(&self) -> ::windows_core::Result<ISWbemMethodSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Methods_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemMethodSet>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Derivation_(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Derivation_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Path_(&self) -> ::windows_core::Result<ISWbemObjectPath> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Path_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObjectPath>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Security_(&self) -> ::windows_core::Result<ISWbemSecurity> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Security_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemSecurity>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemObject> for ::windows_core::IUnknown {
    fn from(value: ISWbemObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemObject> for ::windows_core::IUnknown {
    fn from(value: &ISWbemObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISWbemObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISWbemObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemObject> for super::Com::IDispatch {
    fn from(value: ISWbemObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemObject> for super::Com::IDispatch {
    fn from(value: &ISWbemObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ISWbemObject {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ISWbemObject {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ISWbemObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ISWbemObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ISWbemObject {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ISWbemObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemObject").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ISWbemObject {
    type Vtable = ISWbemObject_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x76a6415a_cb41_11d1_8b02_00600806d9b6);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemObject_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub Put_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr, objwbemobjectpath: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Put_: usize,
    #[cfg(feature = "win32-system")]
    pub PutAsync_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows_core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr, objwbemasynccontext: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    PutAsync_: usize,
    #[cfg(feature = "win32-system")]
    pub Delete_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Delete_: usize,
    #[cfg(feature = "win32-system")]
    pub DeleteAsync_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows_core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr, objwbemasynccontext: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    DeleteAsync_: usize,
    #[cfg(feature = "win32-system")]
    pub Instances_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr, objwbemobjectset: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Instances_: usize,
    #[cfg(feature = "win32-system")]
    pub InstancesAsync_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows_core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr, objwbemasynccontext: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    InstancesAsync_: usize,
    #[cfg(feature = "win32-system")]
    pub Subclasses_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr, objwbemobjectset: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Subclasses_: usize,
    #[cfg(feature = "win32-system")]
    pub SubclassesAsync_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows_core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr, objwbemasynccontext: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SubclassesAsync_: usize,
    #[cfg(feature = "win32-system")]
    pub Associators_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strassocclass: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strresultclass: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strresultrole: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredassocqualifier: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strrequiredqualifier: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr, objwbemobjectset: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Associators_: usize,
    #[cfg(feature = "win32-system")]
    pub AssociatorsAsync_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows_core::RawPtr, strassocclass: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strresultclass: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strresultrole: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredassocqualifier: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strrequiredqualifier: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr, objwbemasynccontext: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    AssociatorsAsync_: usize,
    #[cfg(feature = "win32-system")]
    pub References_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strresultclass: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr, objwbemobjectset: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    References_: usize,
    #[cfg(feature = "win32-system")]
    pub ReferencesAsync_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows_core::RawPtr, strresultclass: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr, objwbemasynccontext: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ReferencesAsync_: usize,
    #[cfg(feature = "win32-system")]
    pub ExecMethod_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strmethodname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, objwbeminparameters: ::windows_core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr, objwbemoutparameters: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ExecMethod_: usize,
    #[cfg(feature = "win32-system")]
    pub ExecMethodAsync_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows_core::RawPtr, strmethodname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, objwbeminparameters: ::windows_core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr, objwbemasynccontext: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ExecMethodAsync_: usize,
    #[cfg(feature = "win32-system")]
    pub Clone_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemobject: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Clone_: usize,
    pub GetObjectText_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32, strobjecttext: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub SpawnDerivedClass_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32, objwbemobject: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SpawnDerivedClass_: usize,
    #[cfg(feature = "win32-system")]
    pub SpawnInstance_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32, objwbemobject: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SpawnInstance_: usize,
    #[cfg(feature = "win32-system")]
    pub CompareTo_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemobject: ::windows_core::RawPtr, iflags: i32, bresult: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CompareTo_: usize,
    #[cfg(feature = "win32-system")]
    pub Qualifiers_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemqualifierset: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Qualifiers_: usize,
    #[cfg(feature = "win32-system")]
    pub Properties_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbempropertyset: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Properties_: usize,
    #[cfg(feature = "win32-system")]
    pub Methods_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemmethodset: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Methods_: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Derivation_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strclassnamearray: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Derivation_: usize,
    #[cfg(feature = "win32-system")]
    pub Path_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemobjectpath: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Path_: usize,
    #[cfg(feature = "win32-system")]
    pub Security_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Security_: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ISWbemObjectEx(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ISWbemObjectEx {
    #[cfg(feature = "win32-system")]
    pub unsafe fn Put_<'a, Param1: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, iflags: i32, objwbemnamedvalueset: Param1) -> ::windows_core::Result<ISWbemObjectPath> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Put_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObjectPath>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn PutAsync_<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param2: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param3: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, iflags: i32, objwbemnamedvalueset: Param2, objwbemasynccontext: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.PutAsync_)(::windows_core::Interface::as_raw(self), objwbemsink.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Delete_<'a, Param1: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, iflags: i32, objwbemnamedvalueset: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Delete_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn DeleteAsync_<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param2: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param3: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, iflags: i32, objwbemnamedvalueset: Param2, objwbemasynccontext: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteAsync_)(::windows_core::Interface::as_raw(self), objwbemsink.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Instances_<'a, Param1: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, iflags: i32, objwbemnamedvalueset: Param1) -> ::windows_core::Result<ISWbemObjectSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Instances_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObjectSet>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn InstancesAsync_<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param2: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param3: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, iflags: i32, objwbemnamedvalueset: Param2, objwbemasynccontext: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.InstancesAsync_)(::windows_core::Interface::as_raw(self), objwbemsink.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Subclasses_<'a, Param1: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, iflags: i32, objwbemnamedvalueset: Param1) -> ::windows_core::Result<ISWbemObjectSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Subclasses_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObjectSet>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SubclassesAsync_<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param2: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param3: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, iflags: i32, objwbemnamedvalueset: Param2, objwbemasynccontext: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SubclassesAsync_)(::windows_core::Interface::as_raw(self), objwbemsink.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Associators_<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param6: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param7: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param9: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, strassocclass: Param0, strresultclass: Param1, strresultrole: Param2, strrole: Param3, bclassesonly: i16, bschemaonly: i16, strrequiredassocqualifier: Param6, strrequiredqualifier: Param7, iflags: i32, objwbemnamedvalueset: Param9) -> ::windows_core::Result<ISWbemObjectSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Associators_)(::windows_core::Interface::as_raw(self), strassocclass.into_param().abi(), strresultclass.into_param().abi(), strresultrole.into_param().abi(), strrole.into_param().abi(), ::core::mem::transmute(bclassesonly), ::core::mem::transmute(bschemaonly), strrequiredassocqualifier.into_param().abi(), strrequiredqualifier.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObjectSet>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn AssociatorsAsync_<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param7: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param8: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param10: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param11: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(
        &self,
        objwbemsink: Param0,
        strassocclass: Param1,
        strresultclass: Param2,
        strresultrole: Param3,
        strrole: Param4,
        bclassesonly: i16,
        bschemaonly: i16,
        strrequiredassocqualifier: Param7,
        strrequiredqualifier: Param8,
        iflags: i32,
        objwbemnamedvalueset: Param10,
        objwbemasynccontext: Param11,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AssociatorsAsync_)(::windows_core::Interface::as_raw(self), objwbemsink.into_param().abi(), strassocclass.into_param().abi(), strresultclass.into_param().abi(), strresultrole.into_param().abi(), strrole.into_param().abi(), ::core::mem::transmute(bclassesonly), ::core::mem::transmute(bschemaonly), strrequiredassocqualifier.into_param().abi(), strrequiredqualifier.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn References_<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param6: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, strresultclass: Param0, strrole: Param1, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: Param4, iflags: i32, objwbemnamedvalueset: Param6) -> ::windows_core::Result<ISWbemObjectSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.References_)(::windows_core::Interface::as_raw(self), strresultclass.into_param().abi(), strrole.into_param().abi(), ::core::mem::transmute(bclassesonly), ::core::mem::transmute(bschemaonly), strrequiredqualifier.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObjectSet>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ReferencesAsync_<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param5: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param7: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param8: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strresultclass: Param1, strrole: Param2, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: Param5, iflags: i32, objwbemnamedvalueset: Param7, objwbemasynccontext: Param8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ReferencesAsync_)(::windows_core::Interface::as_raw(self), objwbemsink.into_param().abi(), strresultclass.into_param().abi(), strrole.into_param().abi(), ::core::mem::transmute(bclassesonly), ::core::mem::transmute(bschemaonly), strrequiredqualifier.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ExecMethod_<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param3: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, strmethodname: Param0, objwbeminparameters: Param1, iflags: i32, objwbemnamedvalueset: Param3) -> ::windows_core::Result<ISWbemObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ExecMethod_)(::windows_core::Interface::as_raw(self), strmethodname.into_param().abi(), objwbeminparameters.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObject>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ExecMethodAsync_<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param4: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param5: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strmethodname: Param1, objwbeminparameters: Param2, iflags: i32, objwbemnamedvalueset: Param4, objwbemasynccontext: Param5) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ExecMethodAsync_)(::windows_core::Interface::as_raw(self), objwbemsink.into_param().abi(), strmethodname.into_param().abi(), objwbeminparameters.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Clone_(&self) -> ::windows_core::Result<ISWbemObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Clone_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObject>(result__)
    }
    pub unsafe fn GetObjectText_(&self, iflags: i32) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetObjectText_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SpawnDerivedClass_(&self, iflags: i32) -> ::windows_core::Result<ISWbemObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SpawnDerivedClass_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObject>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SpawnInstance_(&self, iflags: i32) -> ::windows_core::Result<ISWbemObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SpawnInstance_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObject>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CompareTo_<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemobject: Param0, iflags: i32) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CompareTo_)(::windows_core::Interface::as_raw(self), objwbemobject.into_param().abi(), ::core::mem::transmute(iflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Qualifiers_(&self) -> ::windows_core::Result<ISWbemQualifierSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Qualifiers_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemQualifierSet>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Properties_(&self) -> ::windows_core::Result<ISWbemPropertySet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Properties_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemPropertySet>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Methods_(&self) -> ::windows_core::Result<ISWbemMethodSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Methods_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemMethodSet>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Derivation_(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Derivation_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Path_(&self) -> ::windows_core::Result<ISWbemObjectPath> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Path_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObjectPath>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Security_(&self) -> ::windows_core::Result<ISWbemSecurity> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Security_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemSecurity>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Refresh_<'a, Param1: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, iflags: i32, objwbemnamedvalueset: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SystemProperties_(&self) -> ::windows_core::Result<ISWbemPropertySet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).SystemProperties_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemPropertySet>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetText_<'a, Param2: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: Param2) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetText_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iobjecttextformat), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetFromText_<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, bstext: Param0, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFromText_)(::windows_core::Interface::as_raw(self), bstext.into_param().abi(), ::core::mem::transmute(iobjecttextformat), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemObjectEx> for ::windows_core::IUnknown {
    fn from(value: ISWbemObjectEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemObjectEx> for ::windows_core::IUnknown {
    fn from(value: &ISWbemObjectEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISWbemObjectEx {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISWbemObjectEx {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemObjectEx> for super::Com::IDispatch {
    fn from(value: ISWbemObjectEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemObjectEx> for super::Com::IDispatch {
    fn from(value: &ISWbemObjectEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ISWbemObjectEx {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ISWbemObjectEx {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemObjectEx> for ISWbemObject {
    fn from(value: ISWbemObjectEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemObjectEx> for ISWbemObject {
    fn from(value: &ISWbemObjectEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ISWbemObject> for ISWbemObjectEx {
    fn into_param(self) -> ::windows_core::Param<'a, ISWbemObject> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ISWbemObject> for &'a ISWbemObjectEx {
    fn into_param(self) -> ::windows_core::Param<'a, ISWbemObject> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ISWbemObjectEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ISWbemObjectEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ISWbemObjectEx {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ISWbemObjectEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemObjectEx").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ISWbemObjectEx {
    type Vtable = ISWbemObjectEx_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x269ad56a_8a67_4129_bc8c_0506dcfe9880);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemObjectEx_Vtbl {
    pub base__: ISWbemObject_Vtbl,
    #[cfg(feature = "win32-system")]
    pub Refresh_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Refresh_: usize,
    #[cfg(feature = "win32-system")]
    pub SystemProperties_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbempropertyset: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SystemProperties_: usize,
    #[cfg(feature = "win32-system")]
    pub GetText_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr, bstext: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetText_: usize,
    #[cfg(feature = "win32-system")]
    pub SetFromText_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstext: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetFromText_: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ISWbemObjectPath(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ISWbemObjectPath {
    pub unsafe fn Path(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Path)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetPath<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, strpath: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPath)(::windows_core::Interface::as_raw(self), strpath.into_param().abi()).ok()
    }
    pub unsafe fn RelPath(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).RelPath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetRelPath<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, strrelpath: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRelPath)(::windows_core::Interface::as_raw(self), strrelpath.into_param().abi()).ok()
    }
    pub unsafe fn Server(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Server)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetServer<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, strserver: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetServer)(::windows_core::Interface::as_raw(self), strserver.into_param().abi()).ok()
    }
    pub unsafe fn Namespace(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Namespace)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetNamespace<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, strnamespace: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetNamespace)(::windows_core::Interface::as_raw(self), strnamespace.into_param().abi()).ok()
    }
    pub unsafe fn ParentNamespace(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ParentNamespace)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn DisplayName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).DisplayName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDisplayName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, strdisplayname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDisplayName)(::windows_core::Interface::as_raw(self), strdisplayname.into_param().abi()).ok()
    }
    pub unsafe fn Class(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Class)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetClass<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, strclass: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetClass)(::windows_core::Interface::as_raw(self), strclass.into_param().abi()).ok()
    }
    pub unsafe fn IsClass(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsClass)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetAsClass(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAsClass)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn IsSingleton(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsSingleton)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetAsSingleton(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAsSingleton)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Keys(&self) -> ::windows_core::Result<ISWbemNamedValueSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Keys)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemNamedValueSet>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Security_(&self) -> ::windows_core::Result<ISWbemSecurity> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Security_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemSecurity>(result__)
    }
    pub unsafe fn Locale(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Locale)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetLocale<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, strlocale: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLocale)(::windows_core::Interface::as_raw(self), strlocale.into_param().abi()).ok()
    }
    pub unsafe fn Authority(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Authority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetAuthority<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, strauthority: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAuthority)(::windows_core::Interface::as_raw(self), strauthority.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemObjectPath> for ::windows_core::IUnknown {
    fn from(value: ISWbemObjectPath) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemObjectPath> for ::windows_core::IUnknown {
    fn from(value: &ISWbemObjectPath) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISWbemObjectPath {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISWbemObjectPath {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemObjectPath> for super::Com::IDispatch {
    fn from(value: ISWbemObjectPath) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemObjectPath> for super::Com::IDispatch {
    fn from(value: &ISWbemObjectPath) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ISWbemObjectPath {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ISWbemObjectPath {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ISWbemObjectPath {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ISWbemObjectPath {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ISWbemObjectPath {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ISWbemObjectPath {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemObjectPath").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ISWbemObjectPath {
    type Vtable = ISWbemObjectPath_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5791bc27_ce9c_11d1_97bf_0000f81e849c);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemObjectPath_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strpath: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strpath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub RelPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strrelpath: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetRelPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strrelpath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Server: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strserver: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strserver: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Namespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strnamespace: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strnamespace: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub ParentNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strparentnamespace: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strdisplayname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strdisplayname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Class: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strclass: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strclass: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub IsClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bisclass: *mut i16) -> ::windows_core::HRESULT,
    pub SetAsClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub IsSingleton: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bissingleton: *mut i16) -> ::windows_core::HRESULT,
    pub SetAsSingleton: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Keys: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemnamedvalueset: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Keys: usize,
    #[cfg(feature = "win32-system")]
    pub Security_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Security_: usize,
    pub Locale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strlocale: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetLocale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strlocale: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Authority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strauthority: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetAuthority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strauthority: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ISWbemObjectSet(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ISWbemObjectSet {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Item<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, strobjectpath: Param0, iflags: i32) -> ::windows_core::Result<ISWbemObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), strobjectpath.into_param().abi(), ::core::mem::transmute(iflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObject>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Security_(&self) -> ::windows_core::Result<ISWbemSecurity> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Security_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemSecurity>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ItemIndex(&self, lindex: i32) -> ::windows_core::Result<ISWbemObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ItemIndex)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObject>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemObjectSet> for ::windows_core::IUnknown {
    fn from(value: ISWbemObjectSet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemObjectSet> for ::windows_core::IUnknown {
    fn from(value: &ISWbemObjectSet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISWbemObjectSet {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISWbemObjectSet {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemObjectSet> for super::Com::IDispatch {
    fn from(value: ISWbemObjectSet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemObjectSet> for super::Com::IDispatch {
    fn from(value: &ISWbemObjectSet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ISWbemObjectSet {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ISWbemObjectSet {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ISWbemObjectSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ISWbemObjectSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ISWbemObjectSet {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ISWbemObjectSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemObjectSet").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ISWbemObjectSet {
    type Vtable = ISWbemObjectSet_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x76a6415f_cb41_11d1_8b02_00600806d9b6);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemObjectSet_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, iflags: i32, objwbemobject: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Security_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Security_: usize,
    #[cfg(feature = "win32-system")]
    pub ItemIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, objwbemobject: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ItemIndex: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ISWbemPrivilege(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ISWbemPrivilege {
    pub unsafe fn IsEnabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetIsEnabled(&self, bisenabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIsEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(bisenabled)).ok()
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn DisplayName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).DisplayName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Identifier(&self) -> ::windows_core::Result<WbemPrivilegeEnum> {
        let mut result__ = ::core::mem::MaybeUninit::<WbemPrivilegeEnum>::zeroed();
        (::windows_core::Interface::vtable(self).Identifier)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WbemPrivilegeEnum>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemPrivilege> for ::windows_core::IUnknown {
    fn from(value: ISWbemPrivilege) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemPrivilege> for ::windows_core::IUnknown {
    fn from(value: &ISWbemPrivilege) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISWbemPrivilege {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISWbemPrivilege {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemPrivilege> for super::Com::IDispatch {
    fn from(value: ISWbemPrivilege) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemPrivilege> for super::Com::IDispatch {
    fn from(value: &ISWbemPrivilege) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ISWbemPrivilege {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ISWbemPrivilege {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ISWbemPrivilege {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ISWbemPrivilege {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ISWbemPrivilege {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ISWbemPrivilege {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemPrivilege").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ISWbemPrivilege {
    type Vtable = ISWbemPrivilege_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x26ee67bd_5804_11d2_8b4a_00600806d9b6);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemPrivilege_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bisenabled: *mut i16) -> ::windows_core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bisenabled: i16) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strdisplayname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strdisplayname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Identifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iprivilege: *mut WbemPrivilegeEnum) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ISWbemPrivilegeSet(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ISWbemPrivilegeSet {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Item(&self, iprivilege: WbemPrivilegeEnum) -> ::windows_core::Result<ISWbemPrivilege> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iprivilege), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemPrivilege>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Add(&self, iprivilege: WbemPrivilegeEnum, bisenabled: i16) -> ::windows_core::Result<ISWbemPrivilege> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iprivilege), ::core::mem::transmute(bisenabled), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemPrivilege>(result__)
    }
    pub unsafe fn Remove(&self, iprivilege: WbemPrivilegeEnum) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iprivilege)).ok()
    }
    pub unsafe fn DeleteAll(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteAll)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn AddAsString<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, strprivilege: Param0, bisenabled: i16) -> ::windows_core::Result<ISWbemPrivilege> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).AddAsString)(::windows_core::Interface::as_raw(self), strprivilege.into_param().abi(), ::core::mem::transmute(bisenabled), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemPrivilege>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemPrivilegeSet> for ::windows_core::IUnknown {
    fn from(value: ISWbemPrivilegeSet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemPrivilegeSet> for ::windows_core::IUnknown {
    fn from(value: &ISWbemPrivilegeSet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISWbemPrivilegeSet {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISWbemPrivilegeSet {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemPrivilegeSet> for super::Com::IDispatch {
    fn from(value: ISWbemPrivilegeSet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemPrivilegeSet> for super::Com::IDispatch {
    fn from(value: &ISWbemPrivilegeSet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ISWbemPrivilegeSet {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ISWbemPrivilegeSet {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ISWbemPrivilegeSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ISWbemPrivilegeSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ISWbemPrivilegeSet {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ISWbemPrivilegeSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemPrivilegeSet").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ISWbemPrivilegeSet {
    type Vtable = ISWbemPrivilegeSet_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x26ee67bf_5804_11d2_8b4a_00600806d9b6);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemPrivilegeSet_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iprivilege: WbemPrivilegeEnum, objwbemprivilege: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iprivilege: WbemPrivilegeEnum, bisenabled: i16, objwbemprivilege: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iprivilege: WbemPrivilegeEnum) -> ::windows_core::HRESULT,
    pub DeleteAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub AddAsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strprivilege: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bisenabled: i16, objwbemprivilege: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    AddAsString: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ISWbemProperty(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ISWbemProperty {
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Value(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Value)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetValue(&self, varvalue: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(varvalue)).ok()
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn IsLocal(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsLocal)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn Origin(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Origin)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn CIMType(&self) -> ::windows_core::Result<WbemCimtypeEnum> {
        let mut result__ = ::core::mem::MaybeUninit::<WbemCimtypeEnum>::zeroed();
        (::windows_core::Interface::vtable(self).CIMType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WbemCimtypeEnum>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Qualifiers_(&self) -> ::windows_core::Result<ISWbemQualifierSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Qualifiers_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemQualifierSet>(result__)
    }
    pub unsafe fn IsArray(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsArray)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemProperty> for ::windows_core::IUnknown {
    fn from(value: ISWbemProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemProperty> for ::windows_core::IUnknown {
    fn from(value: &ISWbemProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISWbemProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISWbemProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemProperty> for super::Com::IDispatch {
    fn from(value: ISWbemProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemProperty> for super::Com::IDispatch {
    fn from(value: &ISWbemProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ISWbemProperty {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ISWbemProperty {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ISWbemProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ISWbemProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ISWbemProperty {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ISWbemProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemProperty").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ISWbemProperty {
    type Vtable = ISWbemProperty_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1a388f98_d4ba_11d1_8b09_00600806d9b6);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemProperty_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varvalue: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Value: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varvalue: *const super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetValue: usize,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub IsLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bislocal: *mut i16) -> ::windows_core::HRESULT,
    pub Origin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strorigin: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub CIMType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icimtype: *mut WbemCimtypeEnum) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Qualifiers_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemqualifierset: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Qualifiers_: usize,
    pub IsArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bisarray: *mut i16) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ISWbemPropertySet(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ISWbemPropertySet {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Item<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, strname: Param0, iflags: i32) -> ::windows_core::Result<ISWbemProperty> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), strname.into_param().abi(), ::core::mem::transmute(iflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemProperty>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Add<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, strname: Param0, icimtype: WbemCimtypeEnum, bisarray: i16, iflags: i32) -> ::windows_core::Result<ISWbemProperty> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), strname.into_param().abi(), ::core::mem::transmute(icimtype), ::core::mem::transmute(bisarray), ::core::mem::transmute(iflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemProperty>(result__)
    }
    pub unsafe fn Remove<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, strname: Param0, iflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), strname.into_param().abi(), ::core::mem::transmute(iflags)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemPropertySet> for ::windows_core::IUnknown {
    fn from(value: ISWbemPropertySet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemPropertySet> for ::windows_core::IUnknown {
    fn from(value: &ISWbemPropertySet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISWbemPropertySet {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISWbemPropertySet {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemPropertySet> for super::Com::IDispatch {
    fn from(value: ISWbemPropertySet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemPropertySet> for super::Com::IDispatch {
    fn from(value: &ISWbemPropertySet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ISWbemPropertySet {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ISWbemPropertySet {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ISWbemPropertySet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ISWbemPropertySet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ISWbemPropertySet {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ISWbemPropertySet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemPropertySet").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ISWbemPropertySet {
    type Vtable = ISWbemPropertySet_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdea0a7b2_d4ba_11d1_8b09_00600806d9b6);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemPropertySet_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, iflags: i32, objwbemproperty: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, icimtype: WbemCimtypeEnum, bisarray: i16, iflags: i32, objwbemproperty: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, iflags: i32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ISWbemQualifier(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ISWbemQualifier {
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Value(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Value)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetValue(&self, varvalue: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(varvalue)).ok()
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn IsLocal(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsLocal)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn PropagatesToSubclass(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).PropagatesToSubclass)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetPropagatesToSubclass(&self, bpropagatestosubclass: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPropagatesToSubclass)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(bpropagatestosubclass)).ok()
    }
    pub unsafe fn PropagatesToInstance(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).PropagatesToInstance)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetPropagatesToInstance(&self, bpropagatestoinstance: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPropagatesToInstance)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(bpropagatestoinstance)).ok()
    }
    pub unsafe fn IsOverridable(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsOverridable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetIsOverridable(&self, bisoverridable: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIsOverridable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(bisoverridable)).ok()
    }
    pub unsafe fn IsAmended(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsAmended)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemQualifier> for ::windows_core::IUnknown {
    fn from(value: ISWbemQualifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemQualifier> for ::windows_core::IUnknown {
    fn from(value: &ISWbemQualifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISWbemQualifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISWbemQualifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemQualifier> for super::Com::IDispatch {
    fn from(value: ISWbemQualifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemQualifier> for super::Com::IDispatch {
    fn from(value: &ISWbemQualifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ISWbemQualifier {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ISWbemQualifier {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ISWbemQualifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ISWbemQualifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ISWbemQualifier {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ISWbemQualifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemQualifier").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ISWbemQualifier {
    type Vtable = ISWbemQualifier_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79b05932_d3b7_11d1_8b06_00600806d9b6);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemQualifier_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varvalue: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Value: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varvalue: *const super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetValue: usize,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub IsLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bislocal: *mut i16) -> ::windows_core::HRESULT,
    pub PropagatesToSubclass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bpropagatestosubclass: *mut i16) -> ::windows_core::HRESULT,
    pub SetPropagatesToSubclass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bpropagatestosubclass: i16) -> ::windows_core::HRESULT,
    pub PropagatesToInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bpropagatestoinstance: *mut i16) -> ::windows_core::HRESULT,
    pub SetPropagatesToInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bpropagatestoinstance: i16) -> ::windows_core::HRESULT,
    pub IsOverridable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bisoverridable: *mut i16) -> ::windows_core::HRESULT,
    pub SetIsOverridable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bisoverridable: i16) -> ::windows_core::HRESULT,
    pub IsAmended: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bisamended: *mut i16) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ISWbemQualifierSet(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ISWbemQualifierSet {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Item<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0, iflags: i32) -> ::windows_core::Result<ISWbemQualifier> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), name.into_param().abi(), ::core::mem::transmute(iflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemQualifier>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Add<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, strname: Param0, varval: *const super::Com::VARIANT, bpropagatestosubclass: i16, bpropagatestoinstance: i16, bisoverridable: i16, iflags: i32) -> ::windows_core::Result<ISWbemQualifier> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), strname.into_param().abi(), ::core::mem::transmute(varval), ::core::mem::transmute(bpropagatestosubclass), ::core::mem::transmute(bpropagatestoinstance), ::core::mem::transmute(bisoverridable), ::core::mem::transmute(iflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemQualifier>(result__)
    }
    pub unsafe fn Remove<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, strname: Param0, iflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), strname.into_param().abi(), ::core::mem::transmute(iflags)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemQualifierSet> for ::windows_core::IUnknown {
    fn from(value: ISWbemQualifierSet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemQualifierSet> for ::windows_core::IUnknown {
    fn from(value: &ISWbemQualifierSet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISWbemQualifierSet {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISWbemQualifierSet {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemQualifierSet> for super::Com::IDispatch {
    fn from(value: ISWbemQualifierSet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemQualifierSet> for super::Com::IDispatch {
    fn from(value: &ISWbemQualifierSet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ISWbemQualifierSet {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ISWbemQualifierSet {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ISWbemQualifierSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ISWbemQualifierSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ISWbemQualifierSet {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ISWbemQualifierSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemQualifierSet").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ISWbemQualifierSet {
    type Vtable = ISWbemQualifierSet_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9b16ed16_d3df_11d1_8b08_00600806d9b6);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemQualifierSet_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, iflags: i32, objwbemqualifier: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varval: *const super::Com::VARIANT, bpropagatestosubclass: i16, bpropagatestoinstance: i16, bisoverridable: i16, iflags: i32, objwbemqualifier: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, iflags: i32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ISWbemRefreshableItem(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ISWbemRefreshableItem {
    pub unsafe fn Index(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Index)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Refresher(&self) -> ::windows_core::Result<ISWbemRefresher> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Refresher)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemRefresher>(result__)
    }
    pub unsafe fn IsSet(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsSet)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Object(&self) -> ::windows_core::Result<ISWbemObjectEx> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Object)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObjectEx>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ObjectSet(&self) -> ::windows_core::Result<ISWbemObjectSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ObjectSet)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObjectSet>(result__)
    }
    pub unsafe fn Remove(&self, iflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iflags)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemRefreshableItem> for ::windows_core::IUnknown {
    fn from(value: ISWbemRefreshableItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemRefreshableItem> for ::windows_core::IUnknown {
    fn from(value: &ISWbemRefreshableItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISWbemRefreshableItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISWbemRefreshableItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemRefreshableItem> for super::Com::IDispatch {
    fn from(value: ISWbemRefreshableItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemRefreshableItem> for super::Com::IDispatch {
    fn from(value: &ISWbemRefreshableItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ISWbemRefreshableItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ISWbemRefreshableItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ISWbemRefreshableItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ISWbemRefreshableItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ISWbemRefreshableItem {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ISWbemRefreshableItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemRefreshableItem").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ISWbemRefreshableItem {
    type Vtable = ISWbemRefreshableItem_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5ad4bf92_daab_11d3_b38f_00105a1f473a);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemRefreshableItem_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Index: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iindex: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Refresher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemrefresher: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Refresher: usize,
    pub IsSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bisset: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Object: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemobject: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Object: usize,
    #[cfg(feature = "win32-system")]
    pub ObjectSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemobjectset: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ObjectSet: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ISWbemRefresher(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ISWbemRefresher {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Item(&self, iindex: i32) -> ::windows_core::Result<ISWbemRefreshableItem> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemRefreshableItem>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Add<'a, Param0: ::windows_core::IntoParam<'a, ISWbemServicesEx>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemservices: Param0, bsinstancepath: Param1, iflags: i32, objwbemnamedvalueset: Param3) -> ::windows_core::Result<ISWbemRefreshableItem> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), objwbemservices.into_param().abi(), bsinstancepath.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemRefreshableItem>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn AddEnum<'a, Param0: ::windows_core::IntoParam<'a, ISWbemServicesEx>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemservices: Param0, bsclassname: Param1, iflags: i32, objwbemnamedvalueset: Param3) -> ::windows_core::Result<ISWbemRefreshableItem> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).AddEnum)(::windows_core::Interface::as_raw(self), objwbemservices.into_param().abi(), bsclassname.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemRefreshableItem>(result__)
    }
    pub unsafe fn Remove(&self, iindex: i32, iflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iindex), ::core::mem::transmute(iflags)).ok()
    }
    pub unsafe fn Refresh(&self, iflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iflags)).ok()
    }
    pub unsafe fn AutoReconnect(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).AutoReconnect)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetAutoReconnect(&self, bcount: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAutoReconnect)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(bcount)).ok()
    }
    pub unsafe fn DeleteAll(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteAll)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemRefresher> for ::windows_core::IUnknown {
    fn from(value: ISWbemRefresher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemRefresher> for ::windows_core::IUnknown {
    fn from(value: &ISWbemRefresher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISWbemRefresher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISWbemRefresher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemRefresher> for super::Com::IDispatch {
    fn from(value: ISWbemRefresher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemRefresher> for super::Com::IDispatch {
    fn from(value: &ISWbemRefresher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ISWbemRefresher {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ISWbemRefresher {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ISWbemRefresher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ISWbemRefresher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ISWbemRefresher {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ISWbemRefresher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemRefresher").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ISWbemRefresher {
    type Vtable = ISWbemRefresher_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x14d8250e_d9c2_11d3_b38f_00105a1f473a);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemRefresher_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iindex: i32, objwbemrefreshableitem: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemservices: ::windows_core::RawPtr, bsinstancepath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr, objwbemrefreshableitem: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Add: usize,
    #[cfg(feature = "win32-system")]
    pub AddEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemservices: ::windows_core::RawPtr, bsclassname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr, objwbemrefreshableitem: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    AddEnum: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iindex: i32, iflags: i32) -> ::windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32) -> ::windows_core::HRESULT,
    pub AutoReconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bcount: *mut i16) -> ::windows_core::HRESULT,
    pub SetAutoReconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bcount: i16) -> ::windows_core::HRESULT,
    pub DeleteAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ISWbemSecurity(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ISWbemSecurity {
    pub unsafe fn ImpersonationLevel(&self) -> ::windows_core::Result<WbemImpersonationLevelEnum> {
        let mut result__ = ::core::mem::MaybeUninit::<WbemImpersonationLevelEnum>::zeroed();
        (::windows_core::Interface::vtable(self).ImpersonationLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WbemImpersonationLevelEnum>(result__)
    }
    pub unsafe fn SetImpersonationLevel(&self, iimpersonationlevel: WbemImpersonationLevelEnum) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetImpersonationLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iimpersonationlevel)).ok()
    }
    pub unsafe fn AuthenticationLevel(&self) -> ::windows_core::Result<WbemAuthenticationLevelEnum> {
        let mut result__ = ::core::mem::MaybeUninit::<WbemAuthenticationLevelEnum>::zeroed();
        (::windows_core::Interface::vtable(self).AuthenticationLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WbemAuthenticationLevelEnum>(result__)
    }
    pub unsafe fn SetAuthenticationLevel(&self, iauthenticationlevel: WbemAuthenticationLevelEnum) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAuthenticationLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iauthenticationlevel)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Privileges(&self) -> ::windows_core::Result<ISWbemPrivilegeSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Privileges)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemPrivilegeSet>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemSecurity> for ::windows_core::IUnknown {
    fn from(value: ISWbemSecurity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemSecurity> for ::windows_core::IUnknown {
    fn from(value: &ISWbemSecurity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISWbemSecurity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISWbemSecurity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemSecurity> for super::Com::IDispatch {
    fn from(value: ISWbemSecurity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemSecurity> for super::Com::IDispatch {
    fn from(value: &ISWbemSecurity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ISWbemSecurity {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ISWbemSecurity {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ISWbemSecurity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ISWbemSecurity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ISWbemSecurity {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ISWbemSecurity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemSecurity").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ISWbemSecurity {
    type Vtable = ISWbemSecurity_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb54d66e6_2287_11d2_8b33_00600806d9b6);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemSecurity_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ImpersonationLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iimpersonationlevel: *mut WbemImpersonationLevelEnum) -> ::windows_core::HRESULT,
    pub SetImpersonationLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iimpersonationlevel: WbemImpersonationLevelEnum) -> ::windows_core::HRESULT,
    pub AuthenticationLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iauthenticationlevel: *mut WbemAuthenticationLevelEnum) -> ::windows_core::HRESULT,
    pub SetAuthenticationLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iauthenticationlevel: WbemAuthenticationLevelEnum) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Privileges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemprivilegeset: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Privileges: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ISWbemServices(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ISWbemServices {
    #[cfg(feature = "win32-system")]
    pub unsafe fn Get<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, strobjectpath: Param0, iflags: i32, objwbemnamedvalueset: Param2) -> ::windows_core::Result<ISWbemObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Get)(::windows_core::Interface::as_raw(self), strobjectpath.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObject>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param4: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strobjectpath: Param1, iflags: i32, objwbemnamedvalueset: Param3, objwbemasynccontext: Param4) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetAsync)(::windows_core::Interface::as_raw(self), objwbemsink.into_param().abi(), strobjectpath.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Delete<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, strobjectpath: Param0, iflags: i32, objwbemnamedvalueset: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self), strobjectpath.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn DeleteAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param4: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strobjectpath: Param1, iflags: i32, objwbemnamedvalueset: Param3, objwbemasynccontext: Param4) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteAsync)(::windows_core::Interface::as_raw(self), objwbemsink.into_param().abi(), strobjectpath.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn InstancesOf<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, strclass: Param0, iflags: i32, objwbemnamedvalueset: Param2) -> ::windows_core::Result<ISWbemObjectSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).InstancesOf)(::windows_core::Interface::as_raw(self), strclass.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObjectSet>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn InstancesOfAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param4: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strclass: Param1, iflags: i32, objwbemnamedvalueset: Param3, objwbemasynccontext: Param4) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InstancesOfAsync)(::windows_core::Interface::as_raw(self), objwbemsink.into_param().abi(), strclass.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SubclassesOf<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, strsuperclass: Param0, iflags: i32, objwbemnamedvalueset: Param2) -> ::windows_core::Result<ISWbemObjectSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).SubclassesOf)(::windows_core::Interface::as_raw(self), strsuperclass.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObjectSet>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SubclassesOfAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param4: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strsuperclass: Param1, iflags: i32, objwbemnamedvalueset: Param3, objwbemasynccontext: Param4) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SubclassesOfAsync)(::windows_core::Interface::as_raw(self), objwbemsink.into_param().abi(), strsuperclass.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ExecQuery<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, strquery: Param0, strquerylanguage: Param1, iflags: i32, objwbemnamedvalueset: Param3) -> ::windows_core::Result<ISWbemObjectSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ExecQuery)(::windows_core::Interface::as_raw(self), strquery.into_param().abi(), strquerylanguage.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObjectSet>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ExecQueryAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param5: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strquery: Param1, strquerylanguage: Param2, lflags: i32, objwbemnamedvalueset: Param4, objwbemasynccontext: Param5) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ExecQueryAsync)(::windows_core::Interface::as_raw(self), objwbemsink.into_param().abi(), strquery.into_param().abi(), strquerylanguage.into_param().abi(), ::core::mem::transmute(lflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn AssociatorsOf<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param7: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param8: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param10: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(
        &self,
        strobjectpath: Param0,
        strassocclass: Param1,
        strresultclass: Param2,
        strresultrole: Param3,
        strrole: Param4,
        bclassesonly: i16,
        bschemaonly: i16,
        strrequiredassocqualifier: Param7,
        strrequiredqualifier: Param8,
        iflags: i32,
        objwbemnamedvalueset: Param10,
    ) -> ::windows_core::Result<ISWbemObjectSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).AssociatorsOf)(
            ::windows_core::Interface::as_raw(self),
            strobjectpath.into_param().abi(),
            strassocclass.into_param().abi(),
            strresultclass.into_param().abi(),
            strresultrole.into_param().abi(),
            strrole.into_param().abi(),
            ::core::mem::transmute(bclassesonly),
            ::core::mem::transmute(bschemaonly),
            strrequiredassocqualifier.into_param().abi(),
            strrequiredqualifier.into_param().abi(),
            ::core::mem::transmute(iflags),
            objwbemnamedvalueset.into_param().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<ISWbemObjectSet>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn AssociatorsOfAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param5: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param8: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param9: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param11: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param12: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(
        &self,
        objwbemsink: Param0,
        strobjectpath: Param1,
        strassocclass: Param2,
        strresultclass: Param3,
        strresultrole: Param4,
        strrole: Param5,
        bclassesonly: i16,
        bschemaonly: i16,
        strrequiredassocqualifier: Param8,
        strrequiredqualifier: Param9,
        iflags: i32,
        objwbemnamedvalueset: Param11,
        objwbemasynccontext: Param12,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AssociatorsOfAsync)(
            ::windows_core::Interface::as_raw(self),
            objwbemsink.into_param().abi(),
            strobjectpath.into_param().abi(),
            strassocclass.into_param().abi(),
            strresultclass.into_param().abi(),
            strresultrole.into_param().abi(),
            strrole.into_param().abi(),
            ::core::mem::transmute(bclassesonly),
            ::core::mem::transmute(bschemaonly),
            strrequiredassocqualifier.into_param().abi(),
            strrequiredqualifier.into_param().abi(),
            ::core::mem::transmute(iflags),
            objwbemnamedvalueset.into_param().abi(),
            objwbemasynccontext.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ReferencesTo<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param5: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param7: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, strobjectpath: Param0, strresultclass: Param1, strrole: Param2, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: Param5, iflags: i32, objwbemnamedvalueset: Param7) -> ::windows_core::Result<ISWbemObjectSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ReferencesTo)(::windows_core::Interface::as_raw(self), strobjectpath.into_param().abi(), strresultclass.into_param().abi(), strrole.into_param().abi(), ::core::mem::transmute(bclassesonly), ::core::mem::transmute(bschemaonly), strrequiredqualifier.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObjectSet>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ReferencesToAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param6: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param8: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param9: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strobjectpath: Param1, strresultclass: Param2, strrole: Param3, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: Param6, iflags: i32, objwbemnamedvalueset: Param8, objwbemasynccontext: Param9) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReferencesToAsync)(::windows_core::Interface::as_raw(self), objwbemsink.into_param().abi(), strobjectpath.into_param().abi(), strresultclass.into_param().abi(), strrole.into_param().abi(), ::core::mem::transmute(bclassesonly), ::core::mem::transmute(bschemaonly), strrequiredqualifier.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ExecNotificationQuery<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, strquery: Param0, strquerylanguage: Param1, iflags: i32, objwbemnamedvalueset: Param3) -> ::windows_core::Result<ISWbemEventSource> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ExecNotificationQuery)(::windows_core::Interface::as_raw(self), strquery.into_param().abi(), strquerylanguage.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemEventSource>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ExecNotificationQueryAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param5: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strquery: Param1, strquerylanguage: Param2, iflags: i32, objwbemnamedvalueset: Param4, objwbemasynccontext: Param5) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ExecNotificationQueryAsync)(::windows_core::Interface::as_raw(self), objwbemsink.into_param().abi(), strquery.into_param().abi(), strquerylanguage.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ExecMethod<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param4: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, strobjectpath: Param0, strmethodname: Param1, objwbeminparameters: Param2, iflags: i32, objwbemnamedvalueset: Param4) -> ::windows_core::Result<ISWbemObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ExecMethod)(::windows_core::Interface::as_raw(self), strobjectpath.into_param().abi(), strmethodname.into_param().abi(), objwbeminparameters.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObject>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ExecMethodAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param5: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param6: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strobjectpath: Param1, strmethodname: Param2, objwbeminparameters: Param3, iflags: i32, objwbemnamedvalueset: Param5, objwbemasynccontext: Param6) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ExecMethodAsync)(::windows_core::Interface::as_raw(self), objwbemsink.into_param().abi(), strobjectpath.into_param().abi(), strmethodname.into_param().abi(), objwbeminparameters.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Security_(&self) -> ::windows_core::Result<ISWbemSecurity> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Security_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemSecurity>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemServices> for ::windows_core::IUnknown {
    fn from(value: ISWbemServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemServices> for ::windows_core::IUnknown {
    fn from(value: &ISWbemServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISWbemServices {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISWbemServices {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemServices> for super::Com::IDispatch {
    fn from(value: ISWbemServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemServices> for super::Com::IDispatch {
    fn from(value: &ISWbemServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ISWbemServices {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ISWbemServices {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ISWbemServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ISWbemServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ISWbemServices {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ISWbemServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemServices").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ISWbemServices {
    type Vtable = ISWbemServices_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x76a6415c_cb41_11d1_8b02_00600806d9b6);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemServices_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub Get: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr, objwbemobject: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Get: usize,
    #[cfg(feature = "win32-system")]
    pub GetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows_core::RawPtr, strobjectpath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr, objwbemasynccontext: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetAsync: usize,
    #[cfg(feature = "win32-system")]
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Delete: usize,
    #[cfg(feature = "win32-system")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows_core::RawPtr, strobjectpath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr, objwbemasynccontext: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    DeleteAsync: usize,
    #[cfg(feature = "win32-system")]
    pub InstancesOf: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strclass: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr, objwbemobjectset: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    InstancesOf: usize,
    #[cfg(feature = "win32-system")]
    pub InstancesOfAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows_core::RawPtr, strclass: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr, objwbemasynccontext: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    InstancesOfAsync: usize,
    #[cfg(feature = "win32-system")]
    pub SubclassesOf: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strsuperclass: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr, objwbemobjectset: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SubclassesOf: usize,
    #[cfg(feature = "win32-system")]
    pub SubclassesOfAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows_core::RawPtr, strsuperclass: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr, objwbemasynccontext: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SubclassesOfAsync: usize,
    #[cfg(feature = "win32-system")]
    pub ExecQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strquery: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strquerylanguage: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr, objwbemobjectset: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ExecQuery: usize,
    #[cfg(feature = "win32-system")]
    pub ExecQueryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows_core::RawPtr, strquery: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strquerylanguage: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, lflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr, objwbemasynccontext: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ExecQueryAsync: usize,
    #[cfg(feature = "win32-system")]
    pub AssociatorsOf: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strassocclass: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strresultclass: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strresultrole: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredassocqualifier: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strrequiredqualifier: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr, objwbemobjectset: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    AssociatorsOf: usize,
    #[cfg(feature = "win32-system")]
    pub AssociatorsOfAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows_core::RawPtr, strobjectpath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strassocclass: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strresultclass: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strresultrole: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredassocqualifier: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strrequiredqualifier: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr, objwbemasynccontext: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    AssociatorsOfAsync: usize,
    #[cfg(feature = "win32-system")]
    pub ReferencesTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strresultclass: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr, objwbemobjectset: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ReferencesTo: usize,
    #[cfg(feature = "win32-system")]
    pub ReferencesToAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows_core::RawPtr, strobjectpath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strresultclass: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr, objwbemasynccontext: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ReferencesToAsync: usize,
    #[cfg(feature = "win32-system")]
    pub ExecNotificationQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strquery: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strquerylanguage: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr, objwbemeventsource: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ExecNotificationQuery: usize,
    #[cfg(feature = "win32-system")]
    pub ExecNotificationQueryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows_core::RawPtr, strquery: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strquerylanguage: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr, objwbemasynccontext: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ExecNotificationQueryAsync: usize,
    #[cfg(feature = "win32-system")]
    pub ExecMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strmethodname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, objwbeminparameters: ::windows_core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr, objwbemoutparameters: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ExecMethod: usize,
    #[cfg(feature = "win32-system")]
    pub ExecMethodAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows_core::RawPtr, strobjectpath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strmethodname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, objwbeminparameters: ::windows_core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr, objwbemasynccontext: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ExecMethodAsync: usize,
    #[cfg(feature = "win32-system")]
    pub Security_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Security_: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ISWbemServicesEx(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ISWbemServicesEx {
    #[cfg(feature = "win32-system")]
    pub unsafe fn Get<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, strobjectpath: Param0, iflags: i32, objwbemnamedvalueset: Param2) -> ::windows_core::Result<ISWbemObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Get)(::windows_core::Interface::as_raw(self), strobjectpath.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObject>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param4: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strobjectpath: Param1, iflags: i32, objwbemnamedvalueset: Param3, objwbemasynccontext: Param4) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetAsync)(::windows_core::Interface::as_raw(self), objwbemsink.into_param().abi(), strobjectpath.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Delete<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, strobjectpath: Param0, iflags: i32, objwbemnamedvalueset: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Delete)(::windows_core::Interface::as_raw(self), strobjectpath.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn DeleteAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param4: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strobjectpath: Param1, iflags: i32, objwbemnamedvalueset: Param3, objwbemasynccontext: Param4) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteAsync)(::windows_core::Interface::as_raw(self), objwbemsink.into_param().abi(), strobjectpath.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn InstancesOf<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, strclass: Param0, iflags: i32, objwbemnamedvalueset: Param2) -> ::windows_core::Result<ISWbemObjectSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.InstancesOf)(::windows_core::Interface::as_raw(self), strclass.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObjectSet>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn InstancesOfAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param4: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strclass: Param1, iflags: i32, objwbemnamedvalueset: Param3, objwbemasynccontext: Param4) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.InstancesOfAsync)(::windows_core::Interface::as_raw(self), objwbemsink.into_param().abi(), strclass.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SubclassesOf<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, strsuperclass: Param0, iflags: i32, objwbemnamedvalueset: Param2) -> ::windows_core::Result<ISWbemObjectSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SubclassesOf)(::windows_core::Interface::as_raw(self), strsuperclass.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObjectSet>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SubclassesOfAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param4: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strsuperclass: Param1, iflags: i32, objwbemnamedvalueset: Param3, objwbemasynccontext: Param4) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SubclassesOfAsync)(::windows_core::Interface::as_raw(self), objwbemsink.into_param().abi(), strsuperclass.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ExecQuery<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, strquery: Param0, strquerylanguage: Param1, iflags: i32, objwbemnamedvalueset: Param3) -> ::windows_core::Result<ISWbemObjectSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ExecQuery)(::windows_core::Interface::as_raw(self), strquery.into_param().abi(), strquerylanguage.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObjectSet>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ExecQueryAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param5: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strquery: Param1, strquerylanguage: Param2, lflags: i32, objwbemnamedvalueset: Param4, objwbemasynccontext: Param5) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ExecQueryAsync)(::windows_core::Interface::as_raw(self), objwbemsink.into_param().abi(), strquery.into_param().abi(), strquerylanguage.into_param().abi(), ::core::mem::transmute(lflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn AssociatorsOf<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param7: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param8: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param10: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(
        &self,
        strobjectpath: Param0,
        strassocclass: Param1,
        strresultclass: Param2,
        strresultrole: Param3,
        strrole: Param4,
        bclassesonly: i16,
        bschemaonly: i16,
        strrequiredassocqualifier: Param7,
        strrequiredqualifier: Param8,
        iflags: i32,
        objwbemnamedvalueset: Param10,
    ) -> ::windows_core::Result<ISWbemObjectSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.AssociatorsOf)(
            ::windows_core::Interface::as_raw(self),
            strobjectpath.into_param().abi(),
            strassocclass.into_param().abi(),
            strresultclass.into_param().abi(),
            strresultrole.into_param().abi(),
            strrole.into_param().abi(),
            ::core::mem::transmute(bclassesonly),
            ::core::mem::transmute(bschemaonly),
            strrequiredassocqualifier.into_param().abi(),
            strrequiredqualifier.into_param().abi(),
            ::core::mem::transmute(iflags),
            objwbemnamedvalueset.into_param().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<ISWbemObjectSet>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn AssociatorsOfAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param5: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param8: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param9: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param11: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param12: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(
        &self,
        objwbemsink: Param0,
        strobjectpath: Param1,
        strassocclass: Param2,
        strresultclass: Param3,
        strresultrole: Param4,
        strrole: Param5,
        bclassesonly: i16,
        bschemaonly: i16,
        strrequiredassocqualifier: Param8,
        strrequiredqualifier: Param9,
        iflags: i32,
        objwbemnamedvalueset: Param11,
        objwbemasynccontext: Param12,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AssociatorsOfAsync)(
            ::windows_core::Interface::as_raw(self),
            objwbemsink.into_param().abi(),
            strobjectpath.into_param().abi(),
            strassocclass.into_param().abi(),
            strresultclass.into_param().abi(),
            strresultrole.into_param().abi(),
            strrole.into_param().abi(),
            ::core::mem::transmute(bclassesonly),
            ::core::mem::transmute(bschemaonly),
            strrequiredassocqualifier.into_param().abi(),
            strrequiredqualifier.into_param().abi(),
            ::core::mem::transmute(iflags),
            objwbemnamedvalueset.into_param().abi(),
            objwbemasynccontext.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ReferencesTo<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param5: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param7: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, strobjectpath: Param0, strresultclass: Param1, strrole: Param2, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: Param5, iflags: i32, objwbemnamedvalueset: Param7) -> ::windows_core::Result<ISWbemObjectSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ReferencesTo)(::windows_core::Interface::as_raw(self), strobjectpath.into_param().abi(), strresultclass.into_param().abi(), strrole.into_param().abi(), ::core::mem::transmute(bclassesonly), ::core::mem::transmute(bschemaonly), strrequiredqualifier.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObjectSet>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ReferencesToAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param6: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param8: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param9: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strobjectpath: Param1, strresultclass: Param2, strrole: Param3, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: Param6, iflags: i32, objwbemnamedvalueset: Param8, objwbemasynccontext: Param9) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ReferencesToAsync)(::windows_core::Interface::as_raw(self), objwbemsink.into_param().abi(), strobjectpath.into_param().abi(), strresultclass.into_param().abi(), strrole.into_param().abi(), ::core::mem::transmute(bclassesonly), ::core::mem::transmute(bschemaonly), strrequiredqualifier.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ExecNotificationQuery<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, strquery: Param0, strquerylanguage: Param1, iflags: i32, objwbemnamedvalueset: Param3) -> ::windows_core::Result<ISWbemEventSource> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ExecNotificationQuery)(::windows_core::Interface::as_raw(self), strquery.into_param().abi(), strquerylanguage.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemEventSource>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ExecNotificationQueryAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param5: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strquery: Param1, strquerylanguage: Param2, iflags: i32, objwbemnamedvalueset: Param4, objwbemasynccontext: Param5) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ExecNotificationQueryAsync)(::windows_core::Interface::as_raw(self), objwbemsink.into_param().abi(), strquery.into_param().abi(), strquerylanguage.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ExecMethod<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param4: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, strobjectpath: Param0, strmethodname: Param1, objwbeminparameters: Param2, iflags: i32, objwbemnamedvalueset: Param4) -> ::windows_core::Result<ISWbemObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ExecMethod)(::windows_core::Interface::as_raw(self), strobjectpath.into_param().abi(), strmethodname.into_param().abi(), objwbeminparameters.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObject>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ExecMethodAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param5: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param6: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strobjectpath: Param1, strmethodname: Param2, objwbeminparameters: Param3, iflags: i32, objwbemnamedvalueset: Param5, objwbemasynccontext: Param6) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ExecMethodAsync)(::windows_core::Interface::as_raw(self), objwbemsink.into_param().abi(), strobjectpath.into_param().abi(), strmethodname.into_param().abi(), objwbeminparameters.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Security_(&self) -> ::windows_core::Result<ISWbemSecurity> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Security_)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemSecurity>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Put<'a, Param0: ::windows_core::IntoParam<'a, ISWbemObjectEx>, Param2: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemobject: Param0, iflags: i32, objwbemnamedvalueset: Param2) -> ::windows_core::Result<ISWbemObjectPath> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Put)(::windows_core::Interface::as_raw(self), objwbemobject.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObjectPath>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn PutAsync<'a, Param0: ::windows_core::IntoParam<'a, ISWbemSink>, Param1: ::windows_core::IntoParam<'a, ISWbemObjectEx>, Param3: ::windows_core::IntoParam<'a, super::Com::IDispatch>, Param4: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, objwbemobject: Param1, iflags: i32, objwbemnamedvalueset: Param3, objwbemasynccontext: Param4) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PutAsync)(::windows_core::Interface::as_raw(self), objwbemsink.into_param().abi(), objwbemobject.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemServicesEx> for ::windows_core::IUnknown {
    fn from(value: ISWbemServicesEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemServicesEx> for ::windows_core::IUnknown {
    fn from(value: &ISWbemServicesEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISWbemServicesEx {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISWbemServicesEx {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemServicesEx> for super::Com::IDispatch {
    fn from(value: ISWbemServicesEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemServicesEx> for super::Com::IDispatch {
    fn from(value: &ISWbemServicesEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ISWbemServicesEx {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ISWbemServicesEx {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemServicesEx> for ISWbemServices {
    fn from(value: ISWbemServicesEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemServicesEx> for ISWbemServices {
    fn from(value: &ISWbemServicesEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ISWbemServices> for ISWbemServicesEx {
    fn into_param(self) -> ::windows_core::Param<'a, ISWbemServices> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ISWbemServices> for &'a ISWbemServicesEx {
    fn into_param(self) -> ::windows_core::Param<'a, ISWbemServices> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ISWbemServicesEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ISWbemServicesEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ISWbemServicesEx {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ISWbemServicesEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemServicesEx").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ISWbemServicesEx {
    type Vtable = ISWbemServicesEx_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd2f68443_85dc_427e_91d8_366554cc754c);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemServicesEx_Vtbl {
    pub base__: ISWbemServices_Vtbl,
    #[cfg(feature = "win32-system")]
    pub Put: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemobject: ::windows_core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr, objwbemobjectpath: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Put: usize,
    #[cfg(feature = "win32-system")]
    pub PutAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows_core::RawPtr, objwbemobject: ::windows_core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows_core::RawPtr, objwbemasynccontext: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    PutAsync: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ISWbemSink(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ISWbemSink {
    pub unsafe fn Cancel(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cancel)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemSink> for ::windows_core::IUnknown {
    fn from(value: ISWbemSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemSink> for ::windows_core::IUnknown {
    fn from(value: &ISWbemSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISWbemSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISWbemSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemSink> for super::Com::IDispatch {
    fn from(value: ISWbemSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemSink> for super::Com::IDispatch {
    fn from(value: &ISWbemSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ISWbemSink {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ISWbemSink {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ISWbemSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ISWbemSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ISWbemSink {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ISWbemSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemSink").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ISWbemSink {
    type Vtable = ISWbemSink_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x75718c9f_f029_11d1_a1ac_00c04fb6c223);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemSink_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ISWbemSinkEvents(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ISWbemSinkEvents {}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemSinkEvents> for ::windows_core::IUnknown {
    fn from(value: ISWbemSinkEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemSinkEvents> for ::windows_core::IUnknown {
    fn from(value: &ISWbemSinkEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISWbemSinkEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISWbemSinkEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISWbemSinkEvents> for super::Com::IDispatch {
    fn from(value: ISWbemSinkEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISWbemSinkEvents> for super::Com::IDispatch {
    fn from(value: &ISWbemSinkEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ISWbemSinkEvents {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ISWbemSinkEvents {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ISWbemSinkEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ISWbemSinkEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ISWbemSinkEvents {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ISWbemSinkEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemSinkEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ISWbemSinkEvents {
    type Vtable = ISWbemSinkEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x75718ca0_f029_11d1_a1ac_00c04fb6c223);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemSinkEvents_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
#[repr(transparent)]
pub struct IUnsecuredApartment(::windows_core::IUnknown);
impl IUnsecuredApartment {
    pub unsafe fn CreateObjectStub<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, pobject: Param0) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).CreateObjectStub)(::windows_core::Interface::as_raw(self), pobject.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
}
impl ::core::convert::From<IUnsecuredApartment> for ::windows_core::IUnknown {
    fn from(value: IUnsecuredApartment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUnsecuredApartment> for ::windows_core::IUnknown {
    fn from(value: &IUnsecuredApartment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUnsecuredApartment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUnsecuredApartment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUnsecuredApartment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUnsecuredApartment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUnsecuredApartment {}
impl ::core::fmt::Debug for IUnsecuredApartment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUnsecuredApartment").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IUnsecuredApartment {
    type Vtable = IUnsecuredApartment_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1cfaba8c_1523_11d1_ad79_00c04fd8fdff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnsecuredApartment_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub CreateObjectStub: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobject: *mut ::core::ffi::c_void, ppstub: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IWMIExtension(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IWMIExtension {
    pub unsafe fn WMIObjectPath(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).WMIObjectPath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetWMIObject(&self) -> ::windows_core::Result<ISWbemObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetWMIObject)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemObject>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetWMIServices(&self) -> ::windows_core::Result<ISWbemServices> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetWMIServices)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISWbemServices>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IWMIExtension> for ::windows_core::IUnknown {
    fn from(value: IWMIExtension) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IWMIExtension> for ::windows_core::IUnknown {
    fn from(value: &IWMIExtension) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMIExtension {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMIExtension {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IWMIExtension> for super::Com::IDispatch {
    fn from(value: IWMIExtension) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IWMIExtension> for super::Com::IDispatch {
    fn from(value: &IWMIExtension) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IWMIExtension {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IWMIExtension {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IWMIExtension {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IWMIExtension {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IWMIExtension {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IWMIExtension {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMIExtension").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IWMIExtension {
    type Vtable = IWMIExtension_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xadc1f06e_5c7e_11d2_8b74_00104b2afb41);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMIExtension_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub WMIObjectPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strwmiobjectpath: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub GetWMIObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwmiobject: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetWMIObject: usize,
    #[cfg(feature = "win32-system")]
    pub GetWMIServices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwmiservices: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetWMIServices: usize,
}
#[repr(transparent)]
pub struct IWbemAddressResolution(::windows_core::IUnknown);
impl IWbemAddressResolution {
    pub unsafe fn Resolve<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wsznamespacepath: Param0, wszaddresstype: ::windows_core::PWSTR, pdwaddresslength: *mut u32, pabbinaryaddress: *mut *mut u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Resolve)(::windows_core::Interface::as_raw(self), wsznamespacepath.into_param().abi(), ::core::mem::transmute(wszaddresstype), ::core::mem::transmute(pdwaddresslength), ::core::mem::transmute(pabbinaryaddress)).ok()
    }
}
impl ::core::convert::From<IWbemAddressResolution> for ::windows_core::IUnknown {
    fn from(value: IWbemAddressResolution) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemAddressResolution> for ::windows_core::IUnknown {
    fn from(value: &IWbemAddressResolution) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemAddressResolution {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemAddressResolution {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemAddressResolution {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemAddressResolution {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemAddressResolution {}
impl ::core::fmt::Debug for IWbemAddressResolution {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemAddressResolution").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemAddressResolution {
    type Vtable = IWbemAddressResolution_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf7ce2e12_8c90_11d1_9e7b_00c04fc324a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemAddressResolution_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Resolve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wsznamespacepath: ::windows_core::PCWSTR, wszaddresstype: ::windows_core::PWSTR, pdwaddresslength: *mut u32, pabbinaryaddress: *mut *mut u8) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWbemBackupRestore(::windows_core::IUnknown);
impl IWbemBackupRestore {
    pub unsafe fn Backup<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, strbackuptofile: Param0, lflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Backup)(::windows_core::Interface::as_raw(self), strbackuptofile.into_param().abi(), ::core::mem::transmute(lflags)).ok()
    }
    pub unsafe fn Restore<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, strrestorefromfile: Param0, lflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Restore)(::windows_core::Interface::as_raw(self), strrestorefromfile.into_param().abi(), ::core::mem::transmute(lflags)).ok()
    }
}
impl ::core::convert::From<IWbemBackupRestore> for ::windows_core::IUnknown {
    fn from(value: IWbemBackupRestore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemBackupRestore> for ::windows_core::IUnknown {
    fn from(value: &IWbemBackupRestore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemBackupRestore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemBackupRestore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemBackupRestore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemBackupRestore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemBackupRestore {}
impl ::core::fmt::Debug for IWbemBackupRestore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemBackupRestore").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemBackupRestore {
    type Vtable = IWbemBackupRestore_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc49e32c7_bc8b_11d2_85d4_00105a1f8304);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemBackupRestore_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Backup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strbackuptofile: ::windows_core::PCWSTR, lflags: i32) -> ::windows_core::HRESULT,
    pub Restore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strrestorefromfile: ::windows_core::PCWSTR, lflags: i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWbemBackupRestoreEx(::windows_core::IUnknown);
impl IWbemBackupRestoreEx {
    pub unsafe fn Backup<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, strbackuptofile: Param0, lflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Backup)(::windows_core::Interface::as_raw(self), strbackuptofile.into_param().abi(), ::core::mem::transmute(lflags)).ok()
    }
    pub unsafe fn Restore<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, strrestorefromfile: Param0, lflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Restore)(::windows_core::Interface::as_raw(self), strrestorefromfile.into_param().abi(), ::core::mem::transmute(lflags)).ok()
    }
    pub unsafe fn Pause(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Pause)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Resume)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IWbemBackupRestoreEx> for ::windows_core::IUnknown {
    fn from(value: IWbemBackupRestoreEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemBackupRestoreEx> for ::windows_core::IUnknown {
    fn from(value: &IWbemBackupRestoreEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemBackupRestoreEx {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemBackupRestoreEx {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWbemBackupRestoreEx> for IWbemBackupRestore {
    fn from(value: IWbemBackupRestoreEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemBackupRestoreEx> for IWbemBackupRestore {
    fn from(value: &IWbemBackupRestoreEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWbemBackupRestore> for IWbemBackupRestoreEx {
    fn into_param(self) -> ::windows_core::Param<'a, IWbemBackupRestore> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWbemBackupRestore> for &'a IWbemBackupRestoreEx {
    fn into_param(self) -> ::windows_core::Param<'a, IWbemBackupRestore> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemBackupRestoreEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemBackupRestoreEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemBackupRestoreEx {}
impl ::core::fmt::Debug for IWbemBackupRestoreEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemBackupRestoreEx").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemBackupRestoreEx {
    type Vtable = IWbemBackupRestoreEx_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa359dec5_e813_4834_8a2a_ba7f1d777d76);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemBackupRestoreEx_Vtbl {
    pub base__: IWbemBackupRestore_Vtbl,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWbemCallResult(::windows_core::IUnknown);
impl IWbemCallResult {
    pub unsafe fn GetResultObject(&self, ltimeout: i32) -> ::windows_core::Result<IWbemClassObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetResultObject)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ltimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWbemClassObject>(result__)
    }
    pub unsafe fn GetResultString(&self, ltimeout: i32) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetResultString)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ltimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetResultServices(&self, ltimeout: i32) -> ::windows_core::Result<IWbemServices> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetResultServices)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ltimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWbemServices>(result__)
    }
    pub unsafe fn GetCallStatus(&self, ltimeout: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).GetCallStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ltimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
impl ::core::convert::From<IWbemCallResult> for ::windows_core::IUnknown {
    fn from(value: IWbemCallResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemCallResult> for ::windows_core::IUnknown {
    fn from(value: &IWbemCallResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemCallResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemCallResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemCallResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemCallResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemCallResult {}
impl ::core::fmt::Debug for IWbemCallResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemCallResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemCallResult {
    type Vtable = IWbemCallResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x44aca675_e8fc_11d0_a07c_00c04fb68820);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemCallResult_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetResultObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ltimeout: i32, ppresultobject: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetResultString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ltimeout: i32, pstrresultstring: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetResultServices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ltimeout: i32, ppservices: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetCallStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ltimeout: i32, plstatus: *mut i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWbemClassObject(::windows_core::IUnknown);
impl IWbemClassObject {
    pub unsafe fn GetQualifierSet(&self) -> ::windows_core::Result<IWbemQualifierSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetQualifierSet)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWbemQualifierSet>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Get<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszname: Param0, lflags: i32, pval: *mut super::Com::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Get)(::windows_core::Interface::as_raw(self), wszname.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(pval), ::core::mem::transmute(ptype), ::core::mem::transmute(plflavor)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Put<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszname: Param0, lflags: i32, pval: *const super::Com::VARIANT, r#type: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Put)(::windows_core::Interface::as_raw(self), wszname.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(pval), ::core::mem::transmute(r#type)).ok()
    }
    pub unsafe fn Delete<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self), wszname.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn GetNames<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszqualifiername: Param0, lflags: i32, pqualifierval: *const super::Com::VARIANT) -> ::windows_core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut super::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).GetNames)(::windows_core::Interface::as_raw(self), wszqualifiername.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(pqualifierval), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn BeginEnumeration(&self, lenumflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginEnumeration)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lenumflags)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Next(&self, lflags: i32, strname: *mut ::win32_foundation::BSTR, pval: *mut super::Com::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), ::core::mem::transmute(strname), ::core::mem::transmute(pval), ::core::mem::transmute(ptype), ::core::mem::transmute(plflavor)).ok()
    }
    pub unsafe fn EndEnumeration(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndEnumeration)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetPropertyQualifierSet<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszproperty: Param0) -> ::windows_core::Result<IWbemQualifierSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetPropertyQualifierSet)(::windows_core::Interface::as_raw(self), wszproperty.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWbemQualifierSet>(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IWbemClassObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWbemClassObject>(result__)
    }
    pub unsafe fn GetObjectText(&self, lflags: i32) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetObjectText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SpawnDerivedClass(&self, lflags: i32) -> ::windows_core::Result<IWbemClassObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).SpawnDerivedClass)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWbemClassObject>(result__)
    }
    pub unsafe fn SpawnInstance(&self, lflags: i32) -> ::windows_core::Result<IWbemClassObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).SpawnInstance)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWbemClassObject>(result__)
    }
    pub unsafe fn CompareTo<'a, Param1: ::windows_core::IntoParam<'a, IWbemClassObject>>(&self, lflags: i32, pcompareto: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CompareTo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), pcompareto.into_param().abi()).ok()
    }
    pub unsafe fn GetPropertyOrigin<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszname: Param0) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetPropertyOrigin)(::windows_core::Interface::as_raw(self), wszname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn InheritsFrom<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, strancestor: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InheritsFrom)(::windows_core::Interface::as_raw(self), strancestor.into_param().abi()).ok()
    }
    pub unsafe fn GetMethod<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszname: Param0, lflags: i32, ppinsignature: *mut ::core::option::Option<IWbemClassObject>, ppoutsignature: *mut ::core::option::Option<IWbemClassObject>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetMethod)(::windows_core::Interface::as_raw(self), wszname.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(ppinsignature), ::core::mem::transmute(ppoutsignature)).ok()
    }
    pub unsafe fn PutMethod<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, IWbemClassObject>, Param3: ::windows_core::IntoParam<'a, IWbemClassObject>>(&self, wszname: Param0, lflags: i32, pinsignature: Param2, poutsignature: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PutMethod)(::windows_core::Interface::as_raw(self), wszname.into_param().abi(), ::core::mem::transmute(lflags), pinsignature.into_param().abi(), poutsignature.into_param().abi()).ok()
    }
    pub unsafe fn DeleteMethod<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteMethod)(::windows_core::Interface::as_raw(self), wszname.into_param().abi()).ok()
    }
    pub unsafe fn BeginMethodEnumeration(&self, lenumflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginMethodEnumeration)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lenumflags)).ok()
    }
    pub unsafe fn NextMethod(&self, lflags: i32, pstrname: *mut ::win32_foundation::BSTR, ppinsignature: *mut ::core::option::Option<IWbemClassObject>, ppoutsignature: *mut ::core::option::Option<IWbemClassObject>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NextMethod)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), ::core::mem::transmute(pstrname), ::core::mem::transmute(ppinsignature), ::core::mem::transmute(ppoutsignature)).ok()
    }
    pub unsafe fn EndMethodEnumeration(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndMethodEnumeration)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetMethodQualifierSet<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszmethod: Param0) -> ::windows_core::Result<IWbemQualifierSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetMethodQualifierSet)(::windows_core::Interface::as_raw(self), wszmethod.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWbemQualifierSet>(result__)
    }
    pub unsafe fn GetMethodOrigin<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszmethodname: Param0) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetMethodOrigin)(::windows_core::Interface::as_raw(self), wszmethodname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<IWbemClassObject> for ::windows_core::IUnknown {
    fn from(value: IWbemClassObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemClassObject> for ::windows_core::IUnknown {
    fn from(value: &IWbemClassObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemClassObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemClassObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemClassObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemClassObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemClassObject {}
impl ::core::fmt::Debug for IWbemClassObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemClassObject").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemClassObject {
    type Vtable = IWbemClassObject_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdc12a681_737f_11cf_884d_00aa004b2e24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemClassObject_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetQualifierSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqualset: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Get: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: ::windows_core::PCWSTR, lflags: i32, pval: *mut super::Com::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Get: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Put: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: ::windows_core::PCWSTR, lflags: i32, pval: *const super::Com::VARIANT, r#type: i32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Put: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub GetNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszqualifiername: ::windows_core::PCWSTR, lflags: i32, pqualifierval: *const super::Com::VARIANT, pnames: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    GetNames: usize,
    pub BeginEnumeration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lenumflags: i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, strname: *mut ::win32_foundation::BSTR, pval: *mut super::Com::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Next: usize,
    pub EndEnumeration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetPropertyQualifierSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszproperty: ::windows_core::PCWSTR, ppqualset: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcopy: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetObjectText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pstrobjecttext: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SpawnDerivedClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, ppnewclass: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SpawnInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, ppnewinstance: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CompareTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pcompareto: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetPropertyOrigin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: ::windows_core::PCWSTR, pstrclassname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub InheritsFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strancestor: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: ::windows_core::PCWSTR, lflags: i32, ppinsignature: *mut ::windows_core::RawPtr, ppoutsignature: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PutMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: ::windows_core::PCWSTR, lflags: i32, pinsignature: ::windows_core::RawPtr, poutsignature: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DeleteMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub BeginMethodEnumeration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lenumflags: i32) -> ::windows_core::HRESULT,
    pub NextMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pstrname: *mut ::win32_foundation::BSTR, ppinsignature: *mut ::windows_core::RawPtr, ppoutsignature: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EndMethodEnumeration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetMethodQualifierSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszmethod: ::windows_core::PCWSTR, ppqualset: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetMethodOrigin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszmethodname: ::windows_core::PCWSTR, pstrclassname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWbemClientConnectionTransport(::windows_core::IUnknown);
impl IWbemClientConnectionTransport {
    pub unsafe fn Open<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param5: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param6: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param8: ::windows_core::IntoParam<'a, IWbemContext>>(&self, straddresstype: Param0, abbinaryaddress: &[u8], strobject: Param3, struser: Param4, strpassword: Param5, strlocale: Param6, lflags: i32, pctx: Param8, riid: *const ::windows_core::GUID, pinterface: *mut *mut ::core::ffi::c_void, pcallres: *mut ::core::option::Option<IWbemCallResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Open)(::windows_core::Interface::as_raw(self), straddresstype.into_param().abi(), abbinaryaddress.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(abbinaryaddress)), strobject.into_param().abi(), struser.into_param().abi(), strpassword.into_param().abi(), strlocale.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(pinterface), ::core::mem::transmute(pcallres)).ok()
    }
    pub unsafe fn OpenAsync<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param5: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param6: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param8: ::windows_core::IntoParam<'a, IWbemContext>, Param10: ::windows_core::IntoParam<'a, IWbemObjectSink>>(&self, straddresstype: Param0, abbinaryaddress: &[u8], strobject: Param3, struser: Param4, strpassword: Param5, strlocale: Param6, lflags: i32, pctx: Param8, riid: *const ::windows_core::GUID, presponsehandler: Param10) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OpenAsync)(::windows_core::Interface::as_raw(self), straddresstype.into_param().abi(), abbinaryaddress.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(abbinaryaddress)), strobject.into_param().abi(), struser.into_param().abi(), strpassword.into_param().abi(), strlocale.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), ::core::mem::transmute(riid), presponsehandler.into_param().abi()).ok()
    }
    pub unsafe fn Cancel<'a, Param1: ::windows_core::IntoParam<'a, IWbemObjectSink>>(&self, lflags: i32, phandler: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cancel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), phandler.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWbemClientConnectionTransport> for ::windows_core::IUnknown {
    fn from(value: IWbemClientConnectionTransport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemClientConnectionTransport> for ::windows_core::IUnknown {
    fn from(value: &IWbemClientConnectionTransport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemClientConnectionTransport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemClientConnectionTransport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemClientConnectionTransport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemClientConnectionTransport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemClientConnectionTransport {}
impl ::core::fmt::Debug for IWbemClientConnectionTransport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemClientConnectionTransport").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemClientConnectionTransport {
    type Vtable = IWbemClientConnectionTransport_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa889c72a_fcc1_4a9e_af61_ed071333fb5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemClientConnectionTransport_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, straddresstype: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, struser: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strlocale: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, lflags: i32, pctx: ::windows_core::RawPtr, riid: *const ::windows_core::GUID, pinterface: *mut *mut ::core::ffi::c_void, pcallres: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OpenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, straddresstype: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, struser: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strlocale: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, lflags: i32, pctx: ::windows_core::RawPtr, riid: *const ::windows_core::GUID, presponsehandler: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, phandler: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWbemClientTransport(::windows_core::IUnknown);
impl IWbemClientTransport {
    pub unsafe fn ConnectServer<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param5: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param6: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param8: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param9: ::windows_core::IntoParam<'a, IWbemContext>>(&self, straddresstype: Param0, abbinaryaddress: &[u8], strnetworkresource: Param3, struser: Param4, strpassword: Param5, strlocale: Param6, lsecurityflags: i32, strauthority: Param8, pctx: Param9) -> ::windows_core::Result<IWbemServices> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ConnectServer)(::windows_core::Interface::as_raw(self), straddresstype.into_param().abi(), abbinaryaddress.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(abbinaryaddress)), strnetworkresource.into_param().abi(), struser.into_param().abi(), strpassword.into_param().abi(), strlocale.into_param().abi(), ::core::mem::transmute(lsecurityflags), strauthority.into_param().abi(), pctx.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWbemServices>(result__)
    }
}
impl ::core::convert::From<IWbemClientTransport> for ::windows_core::IUnknown {
    fn from(value: IWbemClientTransport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemClientTransport> for ::windows_core::IUnknown {
    fn from(value: &IWbemClientTransport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemClientTransport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemClientTransport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemClientTransport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemClientTransport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemClientTransport {}
impl ::core::fmt::Debug for IWbemClientTransport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemClientTransport").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemClientTransport {
    type Vtable = IWbemClientTransport_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf7ce2e11_8c90_11d1_9e7b_00c04fc324a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemClientTransport_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub ConnectServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, straddresstype: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strnetworkresource: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, struser: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strlocale: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, lsecurityflags: i32, strauthority: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pctx: ::windows_core::RawPtr, ppnamespace: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWbemConfigureRefresher(::windows_core::IUnknown);
impl IWbemConfigureRefresher {
    pub unsafe fn AddObjectByPath<'a, Param0: ::windows_core::IntoParam<'a, IWbemServices>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, IWbemContext>>(&self, pnamespace: Param0, wszpath: Param1, lflags: i32, pcontext: Param3, pprefreshable: *mut ::core::option::Option<IWbemClassObject>, plid: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddObjectByPath)(::windows_core::Interface::as_raw(self), pnamespace.into_param().abi(), wszpath.into_param().abi(), ::core::mem::transmute(lflags), pcontext.into_param().abi(), ::core::mem::transmute(pprefreshable), ::core::mem::transmute(plid)).ok()
    }
    pub unsafe fn AddObjectByTemplate<'a, Param0: ::windows_core::IntoParam<'a, IWbemServices>, Param1: ::windows_core::IntoParam<'a, IWbemClassObject>, Param3: ::windows_core::IntoParam<'a, IWbemContext>>(&self, pnamespace: Param0, ptemplate: Param1, lflags: i32, pcontext: Param3, pprefreshable: *mut ::core::option::Option<IWbemClassObject>, plid: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddObjectByTemplate)(::windows_core::Interface::as_raw(self), pnamespace.into_param().abi(), ptemplate.into_param().abi(), ::core::mem::transmute(lflags), pcontext.into_param().abi(), ::core::mem::transmute(pprefreshable), ::core::mem::transmute(plid)).ok()
    }
    pub unsafe fn AddRefresher<'a, Param0: ::windows_core::IntoParam<'a, IWbemRefresher>>(&self, prefresher: Param0, lflags: i32, plid: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddRefresher)(::windows_core::Interface::as_raw(self), prefresher.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(plid)).ok()
    }
    pub unsafe fn Remove(&self, lid: i32, lflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lid), ::core::mem::transmute(lflags)).ok()
    }
    pub unsafe fn AddEnum<'a, Param0: ::windows_core::IntoParam<'a, IWbemServices>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, IWbemContext>>(&self, pnamespace: Param0, wszclassname: Param1, lflags: i32, pcontext: Param3, ppenum: *mut ::core::option::Option<IWbemHiPerfEnum>, plid: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddEnum)(::windows_core::Interface::as_raw(self), pnamespace.into_param().abi(), wszclassname.into_param().abi(), ::core::mem::transmute(lflags), pcontext.into_param().abi(), ::core::mem::transmute(ppenum), ::core::mem::transmute(plid)).ok()
    }
}
impl ::core::convert::From<IWbemConfigureRefresher> for ::windows_core::IUnknown {
    fn from(value: IWbemConfigureRefresher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemConfigureRefresher> for ::windows_core::IUnknown {
    fn from(value: &IWbemConfigureRefresher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemConfigureRefresher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemConfigureRefresher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemConfigureRefresher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemConfigureRefresher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemConfigureRefresher {}
impl ::core::fmt::Debug for IWbemConfigureRefresher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemConfigureRefresher").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemConfigureRefresher {
    type Vtable = IWbemConfigureRefresher_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x49353c92_516b_11d1_aea6_00c04fb68820);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemConfigureRefresher_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub AddObjectByPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnamespace: ::windows_core::RawPtr, wszpath: ::windows_core::PCWSTR, lflags: i32, pcontext: ::windows_core::RawPtr, pprefreshable: *mut ::windows_core::RawPtr, plid: *mut i32) -> ::windows_core::HRESULT,
    pub AddObjectByTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnamespace: ::windows_core::RawPtr, ptemplate: ::windows_core::RawPtr, lflags: i32, pcontext: ::windows_core::RawPtr, pprefreshable: *mut ::windows_core::RawPtr, plid: *mut i32) -> ::windows_core::HRESULT,
    pub AddRefresher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prefresher: ::windows_core::RawPtr, lflags: i32, plid: *mut i32) -> ::windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lid: i32, lflags: i32) -> ::windows_core::HRESULT,
    pub AddEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnamespace: ::windows_core::RawPtr, wszclassname: ::windows_core::PCWSTR, lflags: i32, pcontext: ::windows_core::RawPtr, ppenum: *mut ::windows_core::RawPtr, plid: *mut i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWbemConnectorLogin(::windows_core::IUnknown);
impl IWbemConnectorLogin {
    pub unsafe fn ConnectorLogin<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, IWbemContext>, T: ::windows_core::Interface>(&self, wsznetworkresource: Param0, wszpreferredlocale: Param1, lflags: i32, pctx: Param3) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).ConnectorLogin)(::windows_core::Interface::as_raw(self), wsznetworkresource.into_param().abi(), wszpreferredlocale.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IWbemConnectorLogin> for ::windows_core::IUnknown {
    fn from(value: IWbemConnectorLogin) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemConnectorLogin> for ::windows_core::IUnknown {
    fn from(value: &IWbemConnectorLogin) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemConnectorLogin {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemConnectorLogin {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemConnectorLogin {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemConnectorLogin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemConnectorLogin {}
impl ::core::fmt::Debug for IWbemConnectorLogin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemConnectorLogin").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemConnectorLogin {
    type Vtable = IWbemConnectorLogin_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd8ec9cb1_b135_4f10_8b1b_c7188bb0d186);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemConnectorLogin_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub ConnectorLogin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wsznetworkresource: ::windows_core::PCWSTR, wszpreferredlocale: ::windows_core::PCWSTR, lflags: i32, pctx: ::windows_core::RawPtr, riid: *const ::windows_core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWbemConstructClassObject(::windows_core::IUnknown);
impl IWbemConstructClassObject {
    pub unsafe fn SetInheritanceChain(&self, lnumantecedents: i32, awszantecedents: *const ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInheritanceChain)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lnumantecedents), ::core::mem::transmute(awszantecedents)).ok()
    }
    pub unsafe fn SetPropertyOrigin<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszpropertyname: Param0, loriginindex: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPropertyOrigin)(::windows_core::Interface::as_raw(self), wszpropertyname.into_param().abi(), ::core::mem::transmute(loriginindex)).ok()
    }
    pub unsafe fn SetMethodOrigin<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszmethodname: Param0, loriginindex: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMethodOrigin)(::windows_core::Interface::as_raw(self), wszmethodname.into_param().abi(), ::core::mem::transmute(loriginindex)).ok()
    }
    pub unsafe fn SetServerNamespace<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszserver: Param0, wsznamespace: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetServerNamespace)(::windows_core::Interface::as_raw(self), wszserver.into_param().abi(), wsznamespace.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWbemConstructClassObject> for ::windows_core::IUnknown {
    fn from(value: IWbemConstructClassObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemConstructClassObject> for ::windows_core::IUnknown {
    fn from(value: &IWbemConstructClassObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemConstructClassObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemConstructClassObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemConstructClassObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemConstructClassObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemConstructClassObject {}
impl ::core::fmt::Debug for IWbemConstructClassObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemConstructClassObject").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemConstructClassObject {
    type Vtable = IWbemConstructClassObject_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9ef76194_70d5_11d1_ad90_00c04fd8fdff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemConstructClassObject_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetInheritanceChain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnumantecedents: i32, awszantecedents: *const ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetPropertyOrigin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszpropertyname: ::windows_core::PCWSTR, loriginindex: i32) -> ::windows_core::HRESULT,
    pub SetMethodOrigin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszmethodname: ::windows_core::PCWSTR, loriginindex: i32) -> ::windows_core::HRESULT,
    pub SetServerNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszserver: ::windows_core::PCWSTR, wsznamespace: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWbemContext(::windows_core::IUnknown);
impl IWbemContext {
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IWbemContext> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWbemContext>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetNames(&self, lflags: i32) -> ::windows_core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut super::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).GetNames)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn BeginEnumeration(&self, lflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginEnumeration)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Next(&self, lflags: i32, pstrname: *mut ::win32_foundation::BSTR, pvalue: *mut super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), ::core::mem::transmute(pstrname), ::core::mem::transmute(pvalue)).ok()
    }
    pub unsafe fn EndEnumeration(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndEnumeration)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszname: Param0, lflags: i32, pvalue: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetValue)(::windows_core::Interface::as_raw(self), wszname.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(pvalue)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn GetValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszname: Param0, lflags: i32) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetValue)(::windows_core::Interface::as_raw(self), wszname.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn DeleteValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszname: Param0, lflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteValue)(::windows_core::Interface::as_raw(self), wszname.into_param().abi(), ::core::mem::transmute(lflags)).ok()
    }
    pub unsafe fn DeleteAll(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteAll)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IWbemContext> for ::windows_core::IUnknown {
    fn from(value: IWbemContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemContext> for ::windows_core::IUnknown {
    fn from(value: &IWbemContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemContext {}
impl ::core::fmt::Debug for IWbemContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemContext").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemContext {
    type Vtable = IWbemContext_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x44aca674_e8fc_11d0_a07c_00c04fb68820);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemContext_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnewcopy: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub GetNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pnames: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetNames: usize,
    pub BeginEnumeration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pstrname: *mut ::win32_foundation::BSTR, pvalue: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Next: usize,
    pub EndEnumeration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: ::windows_core::PCWSTR, lflags: i32, pvalue: *const super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetValue: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: ::windows_core::PCWSTR, lflags: i32, pvalue: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    GetValue: usize,
    pub DeleteValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: ::windows_core::PCWSTR, lflags: i32) -> ::windows_core::HRESULT,
    pub DeleteAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWbemDecoupledBasicEventProvider(::windows_core::IUnknown);
impl IWbemDecoupledBasicEventProvider {
    pub unsafe fn Register<'a, Param1: ::windows_core::IntoParam<'a, IWbemContext>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param4: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param5: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param6: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, a_flags: i32, a_context: Param1, a_user: Param2, a_locale: Param3, a_scope: Param4, a_registration: Param5, piunknown: Param6) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Register)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(a_flags), a_context.into_param().abi(), a_user.into_param().abi(), a_locale.into_param().abi(), a_scope.into_param().abi(), a_registration.into_param().abi(), piunknown.into_param().abi()).ok()
    }
    pub unsafe fn UnRegister(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.UnRegister)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetSink<'a, Param1: ::windows_core::IntoParam<'a, IWbemContext>>(&self, a_flags: i32, a_context: Param1) -> ::windows_core::Result<IWbemObjectSink> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSink)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(a_flags), a_context.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWbemObjectSink>(result__)
    }
    pub unsafe fn GetService<'a, Param1: ::windows_core::IntoParam<'a, IWbemContext>>(&self, a_flags: i32, a_context: Param1) -> ::windows_core::Result<IWbemServices> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetService)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(a_flags), a_context.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWbemServices>(result__)
    }
}
impl ::core::convert::From<IWbemDecoupledBasicEventProvider> for ::windows_core::IUnknown {
    fn from(value: IWbemDecoupledBasicEventProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemDecoupledBasicEventProvider> for ::windows_core::IUnknown {
    fn from(value: &IWbemDecoupledBasicEventProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemDecoupledBasicEventProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemDecoupledBasicEventProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWbemDecoupledBasicEventProvider> for IWbemDecoupledRegistrar {
    fn from(value: IWbemDecoupledBasicEventProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemDecoupledBasicEventProvider> for IWbemDecoupledRegistrar {
    fn from(value: &IWbemDecoupledBasicEventProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWbemDecoupledRegistrar> for IWbemDecoupledBasicEventProvider {
    fn into_param(self) -> ::windows_core::Param<'a, IWbemDecoupledRegistrar> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWbemDecoupledRegistrar> for &'a IWbemDecoupledBasicEventProvider {
    fn into_param(self) -> ::windows_core::Param<'a, IWbemDecoupledRegistrar> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemDecoupledBasicEventProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemDecoupledBasicEventProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemDecoupledBasicEventProvider {}
impl ::core::fmt::Debug for IWbemDecoupledBasicEventProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemDecoupledBasicEventProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemDecoupledBasicEventProvider {
    type Vtable = IWbemDecoupledBasicEventProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x86336d20_ca11_4786_9ef1_bc8a946b42fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemDecoupledBasicEventProvider_Vtbl {
    pub base__: IWbemDecoupledRegistrar_Vtbl,
    pub GetSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, a_flags: i32, a_context: ::windows_core::RawPtr, a_sink: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, a_flags: i32, a_context: ::windows_core::RawPtr, a_service: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWbemDecoupledRegistrar(::windows_core::IUnknown);
impl IWbemDecoupledRegistrar {
    pub unsafe fn Register<'a, Param1: ::windows_core::IntoParam<'a, IWbemContext>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param4: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param5: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param6: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, a_flags: i32, a_context: Param1, a_user: Param2, a_locale: Param3, a_scope: Param4, a_registration: Param5, piunknown: Param6) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Register)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(a_flags), a_context.into_param().abi(), a_user.into_param().abi(), a_locale.into_param().abi(), a_scope.into_param().abi(), a_registration.into_param().abi(), piunknown.into_param().abi()).ok()
    }
    pub unsafe fn UnRegister(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnRegister)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IWbemDecoupledRegistrar> for ::windows_core::IUnknown {
    fn from(value: IWbemDecoupledRegistrar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemDecoupledRegistrar> for ::windows_core::IUnknown {
    fn from(value: &IWbemDecoupledRegistrar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemDecoupledRegistrar {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemDecoupledRegistrar {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemDecoupledRegistrar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemDecoupledRegistrar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemDecoupledRegistrar {}
impl ::core::fmt::Debug for IWbemDecoupledRegistrar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemDecoupledRegistrar").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemDecoupledRegistrar {
    type Vtable = IWbemDecoupledRegistrar_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1005cbcf_e64f_4646_bcd3_3a089d8a84b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemDecoupledRegistrar_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Register: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, a_flags: i32, a_context: ::windows_core::RawPtr, a_user: ::windows_core::PCWSTR, a_locale: ::windows_core::PCWSTR, a_scope: ::windows_core::PCWSTR, a_registration: ::windows_core::PCWSTR, piunknown: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub UnRegister: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWbemEventConsumerProvider(::windows_core::IUnknown);
impl IWbemEventConsumerProvider {
    pub unsafe fn FindConsumer<'a, Param0: ::windows_core::IntoParam<'a, IWbemClassObject>>(&self, plogicalconsumer: Param0) -> ::windows_core::Result<IWbemUnboundObjectSink> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).FindConsumer)(::windows_core::Interface::as_raw(self), plogicalconsumer.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWbemUnboundObjectSink>(result__)
    }
}
impl ::core::convert::From<IWbemEventConsumerProvider> for ::windows_core::IUnknown {
    fn from(value: IWbemEventConsumerProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemEventConsumerProvider> for ::windows_core::IUnknown {
    fn from(value: &IWbemEventConsumerProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemEventConsumerProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemEventConsumerProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemEventConsumerProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemEventConsumerProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemEventConsumerProvider {}
impl ::core::fmt::Debug for IWbemEventConsumerProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemEventConsumerProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemEventConsumerProvider {
    type Vtable = IWbemEventConsumerProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe246107a_b06e_11d0_ad61_00c04fd8fdff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemEventConsumerProvider_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub FindConsumer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plogicalconsumer: ::windows_core::RawPtr, ppconsumer: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWbemEventProvider(::windows_core::IUnknown);
impl IWbemEventProvider {
    pub unsafe fn ProvideEvents<'a, Param0: ::windows_core::IntoParam<'a, IWbemObjectSink>>(&self, psink: Param0, lflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ProvideEvents)(::windows_core::Interface::as_raw(self), psink.into_param().abi(), ::core::mem::transmute(lflags)).ok()
    }
}
impl ::core::convert::From<IWbemEventProvider> for ::windows_core::IUnknown {
    fn from(value: IWbemEventProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemEventProvider> for ::windows_core::IUnknown {
    fn from(value: &IWbemEventProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemEventProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemEventProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemEventProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemEventProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemEventProvider {}
impl ::core::fmt::Debug for IWbemEventProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemEventProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemEventProvider {
    type Vtable = IWbemEventProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe245105b_b06e_11d0_ad61_00c04fd8fdff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemEventProvider_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub ProvideEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: ::windows_core::RawPtr, lflags: i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWbemEventProviderQuerySink(::windows_core::IUnknown);
impl IWbemEventProviderQuerySink {
    pub unsafe fn NewQuery(&self, dwid: u32, wszquerylanguage: *const u16, wszquery: *const u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NewQuery)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwid), ::core::mem::transmute(wszquerylanguage), ::core::mem::transmute(wszquery)).ok()
    }
    pub unsafe fn CancelQuery(&self, dwid: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CancelQuery)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwid)).ok()
    }
}
impl ::core::convert::From<IWbemEventProviderQuerySink> for ::windows_core::IUnknown {
    fn from(value: IWbemEventProviderQuerySink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemEventProviderQuerySink> for ::windows_core::IUnknown {
    fn from(value: &IWbemEventProviderQuerySink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemEventProviderQuerySink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemEventProviderQuerySink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemEventProviderQuerySink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemEventProviderQuerySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemEventProviderQuerySink {}
impl ::core::fmt::Debug for IWbemEventProviderQuerySink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemEventProviderQuerySink").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemEventProviderQuerySink {
    type Vtable = IWbemEventProviderQuerySink_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x580acaf8_fa1c_11d0_ad72_00c04fd8fdff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemEventProviderQuerySink_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub NewQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwid: u32, wszquerylanguage: *const u16, wszquery: *const u16) -> ::windows_core::HRESULT,
    pub CancelQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwid: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWbemEventProviderSecurity(::windows_core::IUnknown);
impl IWbemEventProviderSecurity {
    pub unsafe fn AccessCheck(&self, wszquerylanguage: *const u16, wszquery: *const u16, psid: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AccessCheck)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wszquerylanguage), ::core::mem::transmute(wszquery), psid.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(psid))).ok()
    }
}
impl ::core::convert::From<IWbemEventProviderSecurity> for ::windows_core::IUnknown {
    fn from(value: IWbemEventProviderSecurity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemEventProviderSecurity> for ::windows_core::IUnknown {
    fn from(value: &IWbemEventProviderSecurity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemEventProviderSecurity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemEventProviderSecurity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemEventProviderSecurity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemEventProviderSecurity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemEventProviderSecurity {}
impl ::core::fmt::Debug for IWbemEventProviderSecurity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemEventProviderSecurity").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemEventProviderSecurity {
    type Vtable = IWbemEventProviderSecurity_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x631f7d96_d993_11d2_b339_00105a1f4aaf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemEventProviderSecurity_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub AccessCheck: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszquerylanguage: *const u16, wszquery: *const u16, lsidlength: i32, psid: *const u8) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWbemEventSink(::windows_core::IUnknown);
impl IWbemEventSink {
    pub unsafe fn Indicate(&self, apobjarray: &[::core::option::Option<IWbemClassObject>]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Indicate)(::windows_core::Interface::as_raw(self), apobjarray.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(apobjarray))).ok()
    }
    pub unsafe fn SetStatus<'a, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, IWbemClassObject>>(&self, lflags: i32, hresult: ::windows_core::HRESULT, strparam: Param2, pobjparam: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), ::core::mem::transmute(hresult), strparam.into_param().abi(), pobjparam.into_param().abi()).ok()
    }
    pub unsafe fn SetSinkSecurity(&self, psd: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSinkSecurity)(::windows_core::Interface::as_raw(self), psd.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(psd))).ok()
    }
    pub unsafe fn IsActive(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsActive)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetRestrictedSink<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, awszqueries: &[::windows_core::PWSTR], pcallback: Param2) -> ::windows_core::Result<IWbemEventSink> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetRestrictedSink)(::windows_core::Interface::as_raw(self), awszqueries.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(awszqueries)), pcallback.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWbemEventSink>(result__)
    }
    pub unsafe fn SetBatchingParameters(&self, lflags: i32, dwmaxbuffersize: u32, dwmaxsendlatency: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBatchingParameters)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), ::core::mem::transmute(dwmaxbuffersize), ::core::mem::transmute(dwmaxsendlatency)).ok()
    }
}
impl ::core::convert::From<IWbemEventSink> for ::windows_core::IUnknown {
    fn from(value: IWbemEventSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemEventSink> for ::windows_core::IUnknown {
    fn from(value: &IWbemEventSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemEventSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemEventSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWbemEventSink> for IWbemObjectSink {
    fn from(value: IWbemEventSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemEventSink> for IWbemObjectSink {
    fn from(value: &IWbemEventSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWbemObjectSink> for IWbemEventSink {
    fn into_param(self) -> ::windows_core::Param<'a, IWbemObjectSink> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWbemObjectSink> for &'a IWbemEventSink {
    fn into_param(self) -> ::windows_core::Param<'a, IWbemObjectSink> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemEventSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemEventSink {}
impl ::core::fmt::Debug for IWbemEventSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemEventSink").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemEventSink {
    type Vtable = IWbemEventSink_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3ae0080a_7e3a_4366_bf89_0feedc931659);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemEventSink_Vtbl {
    pub base__: IWbemObjectSink_Vtbl,
    pub SetSinkSecurity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lsdlength: i32, psd: *const u8) -> ::windows_core::HRESULT,
    pub IsActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetRestrictedSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnumqueries: i32, awszqueries: *const ::windows_core::PWSTR, pcallback: *mut ::core::ffi::c_void, ppsink: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetBatchingParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, dwmaxbuffersize: u32, dwmaxsendlatency: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWbemHiPerfEnum(::windows_core::IUnknown);
impl IWbemHiPerfEnum {
    pub unsafe fn AddObjects(&self, lflags: i32, unumobjects: u32, apids: *const i32, apobj: *const ::core::option::Option<IWbemObjectAccess>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddObjects)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), ::core::mem::transmute(unumobjects), ::core::mem::transmute(apids), ::core::mem::transmute(apobj)).ok()
    }
    pub unsafe fn RemoveObjects(&self, lflags: i32, apids: &[i32]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveObjects)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), apids.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(apids))).ok()
    }
    pub unsafe fn GetObjects(&self, lflags: i32, apobj: &mut [::core::option::Option<IWbemObjectAccess>], pureturned: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetObjects)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), apobj.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(apobj)), ::core::mem::transmute(pureturned)).ok()
    }
    pub unsafe fn RemoveAll(&self, lflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAll)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags)).ok()
    }
}
impl ::core::convert::From<IWbemHiPerfEnum> for ::windows_core::IUnknown {
    fn from(value: IWbemHiPerfEnum) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemHiPerfEnum> for ::windows_core::IUnknown {
    fn from(value: &IWbemHiPerfEnum) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemHiPerfEnum {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemHiPerfEnum {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemHiPerfEnum {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemHiPerfEnum {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemHiPerfEnum {}
impl ::core::fmt::Debug for IWbemHiPerfEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemHiPerfEnum").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemHiPerfEnum {
    type Vtable = IWbemHiPerfEnum_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2705c288_79ae_11d2_b348_00105a1f8177);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemHiPerfEnum_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub AddObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, unumobjects: u32, apids: *const i32, apobj: *const ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, unumobjects: u32, apids: *const i32) -> ::windows_core::HRESULT,
    pub GetObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, unumobjects: u32, apobj: *mut ::windows_core::RawPtr, pureturned: *mut u32) -> ::windows_core::HRESULT,
    pub RemoveAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWbemHiPerfProvider(::windows_core::IUnknown);
impl IWbemHiPerfProvider {
    pub unsafe fn QueryInstances<'a, Param0: ::windows_core::IntoParam<'a, IWbemServices>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, IWbemContext>, Param4: ::windows_core::IntoParam<'a, IWbemObjectSink>>(&self, pnamespace: Param0, wszclass: Param1, lflags: i32, pctx: Param3, psink: Param4) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).QueryInstances)(::windows_core::Interface::as_raw(self), pnamespace.into_param().abi(), wszclass.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), psink.into_param().abi()).ok()
    }
    pub unsafe fn CreateRefresher<'a, Param0: ::windows_core::IntoParam<'a, IWbemServices>>(&self, pnamespace: Param0, lflags: i32) -> ::windows_core::Result<IWbemRefresher> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateRefresher)(::windows_core::Interface::as_raw(self), pnamespace.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWbemRefresher>(result__)
    }
    pub unsafe fn CreateRefreshableObject<'a, Param0: ::windows_core::IntoParam<'a, IWbemServices>, Param1: ::windows_core::IntoParam<'a, IWbemObjectAccess>, Param2: ::windows_core::IntoParam<'a, IWbemRefresher>, Param4: ::windows_core::IntoParam<'a, IWbemContext>>(&self, pnamespace: Param0, ptemplate: Param1, prefresher: Param2, lflags: i32, pcontext: Param4, pprefreshable: *mut ::core::option::Option<IWbemObjectAccess>, plid: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreateRefreshableObject)(::windows_core::Interface::as_raw(self), pnamespace.into_param().abi(), ptemplate.into_param().abi(), prefresher.into_param().abi(), ::core::mem::transmute(lflags), pcontext.into_param().abi(), ::core::mem::transmute(pprefreshable), ::core::mem::transmute(plid)).ok()
    }
    pub unsafe fn StopRefreshing<'a, Param0: ::windows_core::IntoParam<'a, IWbemRefresher>>(&self, prefresher: Param0, lid: i32, lflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StopRefreshing)(::windows_core::Interface::as_raw(self), prefresher.into_param().abi(), ::core::mem::transmute(lid), ::core::mem::transmute(lflags)).ok()
    }
    pub unsafe fn CreateRefreshableEnum<'a, Param0: ::windows_core::IntoParam<'a, IWbemServices>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, IWbemRefresher>, Param4: ::windows_core::IntoParam<'a, IWbemContext>, Param5: ::windows_core::IntoParam<'a, IWbemHiPerfEnum>>(&self, pnamespace: Param0, wszclass: Param1, prefresher: Param2, lflags: i32, pcontext: Param4, phiperfenum: Param5) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).CreateRefreshableEnum)(::windows_core::Interface::as_raw(self), pnamespace.into_param().abi(), wszclass.into_param().abi(), prefresher.into_param().abi(), ::core::mem::transmute(lflags), pcontext.into_param().abi(), phiperfenum.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn GetObjects<'a, Param0: ::windows_core::IntoParam<'a, IWbemServices>, Param4: ::windows_core::IntoParam<'a, IWbemContext>>(&self, pnamespace: Param0, apobj: &mut [::core::option::Option<IWbemObjectAccess>], lflags: i32, pcontext: Param4) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetObjects)(::windows_core::Interface::as_raw(self), pnamespace.into_param().abi(), apobj.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(apobj)), ::core::mem::transmute(lflags), pcontext.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWbemHiPerfProvider> for ::windows_core::IUnknown {
    fn from(value: IWbemHiPerfProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemHiPerfProvider> for ::windows_core::IUnknown {
    fn from(value: &IWbemHiPerfProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemHiPerfProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemHiPerfProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemHiPerfProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemHiPerfProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemHiPerfProvider {}
impl ::core::fmt::Debug for IWbemHiPerfProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemHiPerfProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemHiPerfProvider {
    type Vtable = IWbemHiPerfProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x49353c93_516b_11d1_aea6_00c04fb68820);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemHiPerfProvider_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub QueryInstances: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnamespace: ::windows_core::RawPtr, wszclass: ::windows_core::PCWSTR, lflags: i32, pctx: ::windows_core::RawPtr, psink: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateRefresher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnamespace: ::windows_core::RawPtr, lflags: i32, pprefresher: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateRefreshableObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnamespace: ::windows_core::RawPtr, ptemplate: ::windows_core::RawPtr, prefresher: ::windows_core::RawPtr, lflags: i32, pcontext: ::windows_core::RawPtr, pprefreshable: *mut ::windows_core::RawPtr, plid: *mut i32) -> ::windows_core::HRESULT,
    pub StopRefreshing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prefresher: ::windows_core::RawPtr, lid: i32, lflags: i32) -> ::windows_core::HRESULT,
    pub CreateRefreshableEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnamespace: ::windows_core::RawPtr, wszclass: ::windows_core::PCWSTR, prefresher: ::windows_core::RawPtr, lflags: i32, pcontext: ::windows_core::RawPtr, phiperfenum: ::windows_core::RawPtr, plid: *mut i32) -> ::windows_core::HRESULT,
    pub GetObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnamespace: ::windows_core::RawPtr, lnumobjects: i32, apobj: *mut ::windows_core::RawPtr, lflags: i32, pcontext: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWbemLevel1Login(::windows_core::IUnknown);
impl IWbemLevel1Login {
    pub unsafe fn EstablishPosition<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszlocalelist: Param0, dwnumlocales: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).EstablishPosition)(::windows_core::Interface::as_raw(self), wszlocalelist.into_param().abi(), ::core::mem::transmute(dwnumlocales), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn RequestChallenge<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wsznetworkresource: Param0, wszuser: Param1) -> ::windows_core::Result<u8> {
        let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
        (::windows_core::Interface::vtable(self).RequestChallenge)(::windows_core::Interface::as_raw(self), wsznetworkresource.into_param().abi(), wszuser.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u8>(result__)
    }
    pub unsafe fn WBEMLogin<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, IWbemContext>>(&self, wszpreferredlocale: Param0, accesstoken: *const u8, lflags: i32, pctx: Param3) -> ::windows_core::Result<IWbemServices> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).WBEMLogin)(::windows_core::Interface::as_raw(self), wszpreferredlocale.into_param().abi(), ::core::mem::transmute(accesstoken), ::core::mem::transmute(lflags), pctx.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWbemServices>(result__)
    }
    pub unsafe fn NTLMLogin<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, IWbemContext>>(&self, wsznetworkresource: Param0, wszpreferredlocale: Param1, lflags: i32, pctx: Param3) -> ::windows_core::Result<IWbemServices> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).NTLMLogin)(::windows_core::Interface::as_raw(self), wsznetworkresource.into_param().abi(), wszpreferredlocale.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWbemServices>(result__)
    }
}
impl ::core::convert::From<IWbemLevel1Login> for ::windows_core::IUnknown {
    fn from(value: IWbemLevel1Login) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemLevel1Login> for ::windows_core::IUnknown {
    fn from(value: &IWbemLevel1Login) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemLevel1Login {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemLevel1Login {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemLevel1Login {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemLevel1Login {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemLevel1Login {}
impl ::core::fmt::Debug for IWbemLevel1Login {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemLevel1Login").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemLevel1Login {
    type Vtable = IWbemLevel1Login_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf309ad18_d86a_11d0_a075_00c04fb68820);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemLevel1Login_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub EstablishPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszlocalelist: ::windows_core::PCWSTR, dwnumlocales: u32, reserved: *mut u32) -> ::windows_core::HRESULT,
    pub RequestChallenge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wsznetworkresource: ::windows_core::PCWSTR, wszuser: ::windows_core::PCWSTR, nonce: *mut u8) -> ::windows_core::HRESULT,
    pub WBEMLogin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszpreferredlocale: ::windows_core::PCWSTR, accesstoken: *const u8, lflags: i32, pctx: ::windows_core::RawPtr, ppnamespace: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub NTLMLogin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wsznetworkresource: ::windows_core::PCWSTR, wszpreferredlocale: ::windows_core::PCWSTR, lflags: i32, pctx: ::windows_core::RawPtr, ppnamespace: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWbemLocator(::windows_core::IUnknown);
impl IWbemLocator {
    pub unsafe fn ConnectServer<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param5: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param6: ::windows_core::IntoParam<'a, IWbemContext>>(&self, strnetworkresource: Param0, struser: Param1, strpassword: Param2, strlocale: Param3, lsecurityflags: i32, strauthority: Param5, pctx: Param6) -> ::windows_core::Result<IWbemServices> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ConnectServer)(::windows_core::Interface::as_raw(self), strnetworkresource.into_param().abi(), struser.into_param().abi(), strpassword.into_param().abi(), strlocale.into_param().abi(), ::core::mem::transmute(lsecurityflags), strauthority.into_param().abi(), pctx.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWbemServices>(result__)
    }
}
impl ::core::convert::From<IWbemLocator> for ::windows_core::IUnknown {
    fn from(value: IWbemLocator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemLocator> for ::windows_core::IUnknown {
    fn from(value: &IWbemLocator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemLocator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemLocator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemLocator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemLocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemLocator {}
impl ::core::fmt::Debug for IWbemLocator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemLocator").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemLocator {
    type Vtable = IWbemLocator_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdc12a687_737f_11cf_884d_00aa004b2e24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemLocator_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub ConnectServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strnetworkresource: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, struser: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strlocale: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, lsecurityflags: i32, strauthority: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pctx: ::windows_core::RawPtr, ppnamespace: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWbemObjectAccess(::windows_core::IUnknown);
impl IWbemObjectAccess {
    pub unsafe fn GetQualifierSet(&self) -> ::windows_core::Result<IWbemQualifierSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetQualifierSet)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWbemQualifierSet>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Get<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszname: Param0, lflags: i32, pval: *mut super::Com::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Get)(::windows_core::Interface::as_raw(self), wszname.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(pval), ::core::mem::transmute(ptype), ::core::mem::transmute(plflavor)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Put<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszname: Param0, lflags: i32, pval: *const super::Com::VARIANT, r#type: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Put)(::windows_core::Interface::as_raw(self), wszname.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(pval), ::core::mem::transmute(r#type)).ok()
    }
    pub unsafe fn Delete<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Delete)(::windows_core::Interface::as_raw(self), wszname.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn GetNames<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszqualifiername: Param0, lflags: i32, pqualifierval: *const super::Com::VARIANT) -> ::windows_core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut super::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetNames)(::windows_core::Interface::as_raw(self), wszqualifiername.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(pqualifierval), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn BeginEnumeration(&self, lenumflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.BeginEnumeration)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lenumflags)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Next(&self, lflags: i32, strname: *mut ::win32_foundation::BSTR, pval: *mut super::Com::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Next)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), ::core::mem::transmute(strname), ::core::mem::transmute(pval), ::core::mem::transmute(ptype), ::core::mem::transmute(plflavor)).ok()
    }
    pub unsafe fn EndEnumeration(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EndEnumeration)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetPropertyQualifierSet<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszproperty: Param0) -> ::windows_core::Result<IWbemQualifierSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPropertyQualifierSet)(::windows_core::Interface::as_raw(self), wszproperty.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWbemQualifierSet>(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IWbemClassObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWbemClassObject>(result__)
    }
    pub unsafe fn GetObjectText(&self, lflags: i32) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetObjectText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SpawnDerivedClass(&self, lflags: i32) -> ::windows_core::Result<IWbemClassObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SpawnDerivedClass)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWbemClassObject>(result__)
    }
    pub unsafe fn SpawnInstance(&self, lflags: i32) -> ::windows_core::Result<IWbemClassObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SpawnInstance)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWbemClassObject>(result__)
    }
    pub unsafe fn CompareTo<'a, Param1: ::windows_core::IntoParam<'a, IWbemClassObject>>(&self, lflags: i32, pcompareto: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CompareTo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), pcompareto.into_param().abi()).ok()
    }
    pub unsafe fn GetPropertyOrigin<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszname: Param0) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPropertyOrigin)(::windows_core::Interface::as_raw(self), wszname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn InheritsFrom<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, strancestor: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.InheritsFrom)(::windows_core::Interface::as_raw(self), strancestor.into_param().abi()).ok()
    }
    pub unsafe fn GetMethod<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszname: Param0, lflags: i32, ppinsignature: *mut ::core::option::Option<IWbemClassObject>, ppoutsignature: *mut ::core::option::Option<IWbemClassObject>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetMethod)(::windows_core::Interface::as_raw(self), wszname.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(ppinsignature), ::core::mem::transmute(ppoutsignature)).ok()
    }
    pub unsafe fn PutMethod<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, IWbemClassObject>, Param3: ::windows_core::IntoParam<'a, IWbemClassObject>>(&self, wszname: Param0, lflags: i32, pinsignature: Param2, poutsignature: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.PutMethod)(::windows_core::Interface::as_raw(self), wszname.into_param().abi(), ::core::mem::transmute(lflags), pinsignature.into_param().abi(), poutsignature.into_param().abi()).ok()
    }
    pub unsafe fn DeleteMethod<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteMethod)(::windows_core::Interface::as_raw(self), wszname.into_param().abi()).ok()
    }
    pub unsafe fn BeginMethodEnumeration(&self, lenumflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.BeginMethodEnumeration)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lenumflags)).ok()
    }
    pub unsafe fn NextMethod(&self, lflags: i32, pstrname: *mut ::win32_foundation::BSTR, ppinsignature: *mut ::core::option::Option<IWbemClassObject>, ppoutsignature: *mut ::core::option::Option<IWbemClassObject>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.NextMethod)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), ::core::mem::transmute(pstrname), ::core::mem::transmute(ppinsignature), ::core::mem::transmute(ppoutsignature)).ok()
    }
    pub unsafe fn EndMethodEnumeration(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EndMethodEnumeration)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetMethodQualifierSet<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszmethod: Param0) -> ::windows_core::Result<IWbemQualifierSet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetMethodQualifierSet)(::windows_core::Interface::as_raw(self), wszmethod.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWbemQualifierSet>(result__)
    }
    pub unsafe fn GetMethodOrigin<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszmethodname: Param0) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetMethodOrigin)(::windows_core::Interface::as_raw(self), wszmethodname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetPropertyHandle<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszpropertyname: Param0, ptype: *mut i32, plhandle: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPropertyHandle)(::windows_core::Interface::as_raw(self), wszpropertyname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(plhandle)).ok()
    }
    pub unsafe fn WritePropertyValue(&self, lhandle: i32, adata: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WritePropertyValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lhandle), adata.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(adata))).ok()
    }
    pub unsafe fn ReadPropertyValue(&self, lhandle: i32, plnumbytes: *mut i32, adata: &mut [u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReadPropertyValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lhandle), adata.len() as _, ::core::mem::transmute(plnumbytes), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(adata))).ok()
    }
    pub unsafe fn ReadDWORD(&self, lhandle: i32) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).ReadDWORD)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lhandle), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn WriteDWORD(&self, lhandle: i32, dw: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteDWORD)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lhandle), ::core::mem::transmute(dw)).ok()
    }
    pub unsafe fn ReadQWORD(&self, lhandle: i32) -> ::windows_core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        (::windows_core::Interface::vtable(self).ReadQWORD)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lhandle), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    pub unsafe fn WriteQWORD(&self, lhandle: i32, pw: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteQWORD)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lhandle), ::core::mem::transmute(pw)).ok()
    }
    pub unsafe fn GetPropertyInfoByHandle(&self, lhandle: i32, pstrname: *mut ::win32_foundation::BSTR, ptype: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPropertyInfoByHandle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lhandle), ::core::mem::transmute(pstrname), ::core::mem::transmute(ptype)).ok()
    }
    pub unsafe fn Lock(&self, lflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Lock)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags)).ok()
    }
    pub unsafe fn Unlock(&self, lflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Unlock)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags)).ok()
    }
}
impl ::core::convert::From<IWbemObjectAccess> for ::windows_core::IUnknown {
    fn from(value: IWbemObjectAccess) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemObjectAccess> for ::windows_core::IUnknown {
    fn from(value: &IWbemObjectAccess) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemObjectAccess {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemObjectAccess {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWbemObjectAccess> for IWbemClassObject {
    fn from(value: IWbemObjectAccess) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemObjectAccess> for IWbemClassObject {
    fn from(value: &IWbemObjectAccess) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWbemClassObject> for IWbemObjectAccess {
    fn into_param(self) -> ::windows_core::Param<'a, IWbemClassObject> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWbemClassObject> for &'a IWbemObjectAccess {
    fn into_param(self) -> ::windows_core::Param<'a, IWbemClassObject> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemObjectAccess {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemObjectAccess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemObjectAccess {}
impl ::core::fmt::Debug for IWbemObjectAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemObjectAccess").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemObjectAccess {
    type Vtable = IWbemObjectAccess_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x49353c9a_516b_11d1_aea6_00c04fb68820);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemObjectAccess_Vtbl {
    pub base__: IWbemClassObject_Vtbl,
    pub GetPropertyHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszpropertyname: ::windows_core::PCWSTR, ptype: *mut i32, plhandle: *mut i32) -> ::windows_core::HRESULT,
    pub WritePropertyValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lhandle: i32, lnumbytes: i32, adata: *const u8) -> ::windows_core::HRESULT,
    pub ReadPropertyValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lhandle: i32, lbuffersize: i32, plnumbytes: *mut i32, adata: *mut u8) -> ::windows_core::HRESULT,
    pub ReadDWORD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lhandle: i32, pdw: *mut u32) -> ::windows_core::HRESULT,
    pub WriteDWORD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lhandle: i32, dw: u32) -> ::windows_core::HRESULT,
    pub ReadQWORD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lhandle: i32, pqw: *mut u64) -> ::windows_core::HRESULT,
    pub WriteQWORD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lhandle: i32, pw: u64) -> ::windows_core::HRESULT,
    pub GetPropertyInfoByHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lhandle: i32, pstrname: *mut ::win32_foundation::BSTR, ptype: *mut i32) -> ::windows_core::HRESULT,
    pub Lock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows_core::HRESULT,
    pub Unlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWbemObjectSink(::windows_core::IUnknown);
impl IWbemObjectSink {
    pub unsafe fn Indicate(&self, apobjarray: &[::core::option::Option<IWbemClassObject>]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Indicate)(::windows_core::Interface::as_raw(self), apobjarray.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(apobjarray))).ok()
    }
    pub unsafe fn SetStatus<'a, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, IWbemClassObject>>(&self, lflags: i32, hresult: ::windows_core::HRESULT, strparam: Param2, pobjparam: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), ::core::mem::transmute(hresult), strparam.into_param().abi(), pobjparam.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWbemObjectSink> for ::windows_core::IUnknown {
    fn from(value: IWbemObjectSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemObjectSink> for ::windows_core::IUnknown {
    fn from(value: &IWbemObjectSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemObjectSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemObjectSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemObjectSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemObjectSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemObjectSink {}
impl ::core::fmt::Debug for IWbemObjectSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemObjectSink").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemObjectSink {
    type Vtable = IWbemObjectSink_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7c857801_7381_11cf_884d_00aa004b2e24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemObjectSink_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Indicate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lobjectcount: i32, apobjarray: *const ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, hresult: ::windows_core::HRESULT, strparam: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pobjparam: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWbemObjectSinkEx(::windows_core::IUnknown);
impl IWbemObjectSinkEx {
    pub unsafe fn Indicate(&self, apobjarray: &[::core::option::Option<IWbemClassObject>]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Indicate)(::windows_core::Interface::as_raw(self), apobjarray.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(apobjarray))).ok()
    }
    pub unsafe fn SetStatus<'a, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, IWbemClassObject>>(&self, lflags: i32, hresult: ::windows_core::HRESULT, strparam: Param2, pobjparam: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), ::core::mem::transmute(hresult), strparam.into_param().abi(), pobjparam.into_param().abi()).ok()
    }
    pub unsafe fn WriteMessage<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, uchannel: u32, strmessage: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteMessage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uchannel), strmessage.into_param().abi()).ok()
    }
    pub unsafe fn WriteError<'a, Param0: ::windows_core::IntoParam<'a, IWbemClassObject>>(&self, pobjerror: Param0) -> ::windows_core::Result<u8> {
        let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
        (::windows_core::Interface::vtable(self).WriteError)(::windows_core::Interface::as_raw(self), pobjerror.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u8>(result__)
    }
    pub unsafe fn PromptUser<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, strmessage: Param0, uprompttype: u8) -> ::windows_core::Result<u8> {
        let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
        (::windows_core::Interface::vtable(self).PromptUser)(::windows_core::Interface::as_raw(self), strmessage.into_param().abi(), ::core::mem::transmute(uprompttype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u8>(result__)
    }
    pub unsafe fn WriteProgress<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, stractivity: Param0, strcurrentoperation: Param1, strstatusdescription: Param2, upercentcomplete: u32, usecondsremaining: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteProgress)(::windows_core::Interface::as_raw(self), stractivity.into_param().abi(), strcurrentoperation.into_param().abi(), strstatusdescription.into_param().abi(), ::core::mem::transmute(upercentcomplete), ::core::mem::transmute(usecondsremaining)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn WriteStreamParameter<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, strname: Param0, vtvalue: *const super::Com::VARIANT, ultype: u32, ulflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteStreamParameter)(::windows_core::Interface::as_raw(self), strname.into_param().abi(), ::core::mem::transmute(vtvalue), ::core::mem::transmute(ultype), ::core::mem::transmute(ulflags)).ok()
    }
}
impl ::core::convert::From<IWbemObjectSinkEx> for ::windows_core::IUnknown {
    fn from(value: IWbemObjectSinkEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemObjectSinkEx> for ::windows_core::IUnknown {
    fn from(value: &IWbemObjectSinkEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemObjectSinkEx {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemObjectSinkEx {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWbemObjectSinkEx> for IWbemObjectSink {
    fn from(value: IWbemObjectSinkEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemObjectSinkEx> for IWbemObjectSink {
    fn from(value: &IWbemObjectSinkEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWbemObjectSink> for IWbemObjectSinkEx {
    fn into_param(self) -> ::windows_core::Param<'a, IWbemObjectSink> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWbemObjectSink> for &'a IWbemObjectSinkEx {
    fn into_param(self) -> ::windows_core::Param<'a, IWbemObjectSink> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemObjectSinkEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemObjectSinkEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemObjectSinkEx {}
impl ::core::fmt::Debug for IWbemObjectSinkEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemObjectSinkEx").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemObjectSinkEx {
    type Vtable = IWbemObjectSinkEx_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe7d35cfa_348b_485e_b524_252725d697ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemObjectSinkEx_Vtbl {
    pub base__: IWbemObjectSink_Vtbl,
    pub WriteMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uchannel: u32, strmessage: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub WriteError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobjerror: ::windows_core::RawPtr, pureturned: *mut u8) -> ::windows_core::HRESULT,
    pub PromptUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strmessage: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, uprompttype: u8, pureturned: *mut u8) -> ::windows_core::HRESULT,
    pub WriteProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stractivity: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strcurrentoperation: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strstatusdescription: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, upercentcomplete: u32, usecondsremaining: u32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub WriteStreamParameter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, vtvalue: *const super::Com::VARIANT, ultype: u32, ulflags: u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    WriteStreamParameter: usize,
}
#[repr(transparent)]
pub struct IWbemObjectTextSrc(::windows_core::IUnknown);
impl IWbemObjectTextSrc {
    pub unsafe fn GetText<'a, Param1: ::windows_core::IntoParam<'a, IWbemClassObject>, Param3: ::windows_core::IntoParam<'a, IWbemContext>>(&self, lflags: i32, pobj: Param1, uobjtextformat: u32, pctx: Param3) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), pobj.into_param().abi(), ::core::mem::transmute(uobjtextformat), pctx.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn CreateFromText<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, IWbemContext>>(&self, lflags: i32, strtext: Param1, uobjtextformat: u32, pctx: Param3) -> ::windows_core::Result<IWbemClassObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateFromText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), strtext.into_param().abi(), ::core::mem::transmute(uobjtextformat), pctx.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWbemClassObject>(result__)
    }
}
impl ::core::convert::From<IWbemObjectTextSrc> for ::windows_core::IUnknown {
    fn from(value: IWbemObjectTextSrc) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemObjectTextSrc> for ::windows_core::IUnknown {
    fn from(value: &IWbemObjectTextSrc) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemObjectTextSrc {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemObjectTextSrc {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemObjectTextSrc {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemObjectTextSrc {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemObjectTextSrc {}
impl ::core::fmt::Debug for IWbemObjectTextSrc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemObjectTextSrc").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemObjectTextSrc {
    type Vtable = IWbemObjectTextSrc_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbfbf883a_cad7_11d3_a11b_00105a1f515a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemObjectTextSrc_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pobj: ::windows_core::RawPtr, uobjtextformat: u32, pctx: ::windows_core::RawPtr, strtext: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub CreateFromText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, strtext: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, uobjtextformat: u32, pctx: ::windows_core::RawPtr, pnewobj: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWbemPath(::windows_core::IUnknown);
impl IWbemPath {
    pub unsafe fn SetText<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, umode: u32, pszpath: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(umode), pszpath.into_param().abi()).ok()
    }
    pub unsafe fn GetText(&self, lflags: i32, pubufflength: *mut u32, psztext: ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), ::core::mem::transmute(pubufflength), ::core::mem::transmute(psztext)).ok()
    }
    pub unsafe fn GetInfo(&self, urequestedinfo: u32) -> ::windows_core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        (::windows_core::Interface::vtable(self).GetInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(urequestedinfo), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    pub unsafe fn SetServer<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, name: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetServer)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn GetServer(&self, punamebuflength: *mut u32, pname: ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetServer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(punamebuflength), ::core::mem::transmute(pname)).ok()
    }
    pub unsafe fn GetNamespaceCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetNamespaceCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetNamespaceAt<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, uindex: u32, pszname: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetNamespaceAt)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uindex), pszname.into_param().abi()).ok()
    }
    pub unsafe fn GetNamespaceAt(&self, uindex: u32, punamebuflength: *mut u32, pname: ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetNamespaceAt)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uindex), ::core::mem::transmute(punamebuflength), ::core::mem::transmute(pname)).ok()
    }
    pub unsafe fn RemoveNamespaceAt(&self, uindex: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveNamespaceAt)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uindex)).ok()
    }
    pub unsafe fn RemoveAllNamespaces(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAllNamespaces)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetScopeCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetScopeCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetScope<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, uindex: u32, pszclass: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetScope)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uindex), pszclass.into_param().abi()).ok()
    }
    pub unsafe fn SetScopeFromText<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, uindex: u32, psztext: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetScopeFromText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uindex), psztext.into_param().abi()).ok()
    }
    pub unsafe fn GetScope(&self, uindex: u32, puclassnamebufsize: *mut u32, pszclass: ::windows_core::PWSTR, pkeylist: *mut ::core::option::Option<IWbemPathKeyList>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetScope)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uindex), ::core::mem::transmute(puclassnamebufsize), ::core::mem::transmute(pszclass), ::core::mem::transmute(pkeylist)).ok()
    }
    pub unsafe fn GetScopeAsText(&self, uindex: u32, putextbufsize: *mut u32, psztext: ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetScopeAsText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uindex), ::core::mem::transmute(putextbufsize), ::core::mem::transmute(psztext)).ok()
    }
    pub unsafe fn RemoveScope(&self, uindex: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveScope)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uindex)).ok()
    }
    pub unsafe fn RemoveAllScopes(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAllScopes)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetClassName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, name: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetClassName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn GetClassName(&self, pubufflength: *mut u32, pszname: ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetClassName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pubufflength), ::core::mem::transmute(pszname)).ok()
    }
    pub unsafe fn GetKeyList(&self) -> ::windows_core::Result<IWbemPathKeyList> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetKeyList)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWbemPathKeyList>(result__)
    }
    pub unsafe fn CreateClassPart<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, lflags: i32, name: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreateClassPart)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), name.into_param().abi()).ok()
    }
    pub unsafe fn DeleteClassPart(&self, lflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteClassPart)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags)).ok()
    }
    pub unsafe fn IsRelative<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszmachine: Param0, wsznamespace: Param1) -> ::win32_foundation::BOOL {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).IsRelative)(::windows_core::Interface::as_raw(self), wszmachine.into_param().abi(), wsznamespace.into_param().abi()))
    }
    pub unsafe fn IsRelativeOrChild<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszmachine: Param0, wsznamespace: Param1, lflags: i32) -> ::win32_foundation::BOOL {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).IsRelativeOrChild)(::windows_core::Interface::as_raw(self), wszmachine.into_param().abi(), wsznamespace.into_param().abi(), ::core::mem::transmute(lflags)))
    }
    pub unsafe fn IsLocal<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszmachine: Param0) -> ::win32_foundation::BOOL {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).IsLocal)(::windows_core::Interface::as_raw(self), wszmachine.into_param().abi()))
    }
    pub unsafe fn IsSameClassName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszclass: Param0) -> ::win32_foundation::BOOL {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).IsSameClassName)(::windows_core::Interface::as_raw(self), wszclass.into_param().abi()))
    }
}
impl ::core::convert::From<IWbemPath> for ::windows_core::IUnknown {
    fn from(value: IWbemPath) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemPath> for ::windows_core::IUnknown {
    fn from(value: &IWbemPath) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemPath {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemPath {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemPath {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemPath {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemPath {}
impl ::core::fmt::Debug for IWbemPath {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemPath").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemPath {
    type Vtable = IWbemPath_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3bc15af2_736c_477e_9e51_238af8667dcc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemPath_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, umode: u32, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pubufflength: *mut u32, psztext: ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, urequestedinfo: u32, puresponse: *mut u64) -> ::windows_core::HRESULT,
    pub SetServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punamebuflength: *mut u32, pname: ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetNamespaceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows_core::HRESULT,
    pub SetNamespaceAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, pszname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetNamespaceAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, punamebuflength: *mut u32, pname: ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub RemoveNamespaceAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32) -> ::windows_core::HRESULT,
    pub RemoveAllNamespaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetScopeCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows_core::HRESULT,
    pub SetScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, pszclass: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub SetScopeFromText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, psztext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, puclassnamebufsize: *mut u32, pszclass: ::windows_core::PWSTR, pkeylist: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetScopeAsText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, putextbufsize: *mut u32, psztext: ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub RemoveScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32) -> ::windows_core::HRESULT,
    pub RemoveAllScopes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetClassName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetClassName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pubufflength: *mut u32, pszname: ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetKeyList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pout: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateClassPart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, name: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub DeleteClassPart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows_core::HRESULT,
    pub IsRelative: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszmachine: ::windows_core::PCWSTR, wsznamespace: ::windows_core::PCWSTR) -> ::win32_foundation::BOOL,
    pub IsRelativeOrChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszmachine: ::windows_core::PCWSTR, wsznamespace: ::windows_core::PCWSTR, lflags: i32) -> ::win32_foundation::BOOL,
    pub IsLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszmachine: ::windows_core::PCWSTR) -> ::win32_foundation::BOOL,
    pub IsSameClassName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszclass: ::windows_core::PCWSTR) -> ::win32_foundation::BOOL,
}
#[repr(transparent)]
pub struct IWbemPathKeyList(::windows_core::IUnknown);
impl IWbemPathKeyList {
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetKey<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszname: Param0, uflags: u32, ucimtype: u32, pkeyval: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetKey)(::windows_core::Interface::as_raw(self), wszname.into_param().abi(), ::core::mem::transmute(uflags), ::core::mem::transmute(ucimtype), ::core::mem::transmute(pkeyval)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetKey2<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszname: Param0, uflags: u32, ucimtype: u32, pkeyval: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetKey2)(::windows_core::Interface::as_raw(self), wszname.into_param().abi(), ::core::mem::transmute(uflags), ::core::mem::transmute(ucimtype), ::core::mem::transmute(pkeyval)).ok()
    }
    pub unsafe fn GetKey(&self, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: ::windows_core::PWSTR, pukeyvalbufsize: *mut u32, pkeyval: *mut ::core::ffi::c_void, puapparentcimtype: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetKey)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ukeyix), ::core::mem::transmute(uflags), ::core::mem::transmute(punamebufsize), ::core::mem::transmute(pszkeyname), ::core::mem::transmute(pukeyvalbufsize), ::core::mem::transmute(pkeyval), ::core::mem::transmute(puapparentcimtype)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn GetKey2(&self, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: ::windows_core::PWSTR, pkeyvalue: *mut super::Com::VARIANT, puapparentcimtype: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetKey2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ukeyix), ::core::mem::transmute(uflags), ::core::mem::transmute(punamebufsize), ::core::mem::transmute(pszkeyname), ::core::mem::transmute(pkeyvalue), ::core::mem::transmute(puapparentcimtype)).ok()
    }
    pub unsafe fn RemoveKey<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszname: Param0, uflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveKey)(::windows_core::Interface::as_raw(self), wszname.into_param().abi(), ::core::mem::transmute(uflags)).ok()
    }
    pub unsafe fn RemoveAllKeys(&self, uflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAllKeys)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uflags)).ok()
    }
    pub unsafe fn MakeSingleton(&self, bset: u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MakeSingleton)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(bset)).ok()
    }
    pub unsafe fn GetInfo(&self, urequestedinfo: u32) -> ::windows_core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        (::windows_core::Interface::vtable(self).GetInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(urequestedinfo), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    pub unsafe fn GetText(&self, lflags: i32, pubufflength: *mut u32, psztext: ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), ::core::mem::transmute(pubufflength), ::core::mem::transmute(psztext)).ok()
    }
}
impl ::core::convert::From<IWbemPathKeyList> for ::windows_core::IUnknown {
    fn from(value: IWbemPathKeyList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemPathKeyList> for ::windows_core::IUnknown {
    fn from(value: &IWbemPathKeyList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemPathKeyList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemPathKeyList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemPathKeyList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemPathKeyList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemPathKeyList {}
impl ::core::fmt::Debug for IWbemPathKeyList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemPathKeyList").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemPathKeyList {
    type Vtable = IWbemPathKeyList_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9ae62877_7544_4bb0_aa26_a13824659ed6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemPathKeyList_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pukeycount: *mut u32) -> ::windows_core::HRESULT,
    pub SetKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: ::windows_core::PCWSTR, uflags: u32, ucimtype: u32, pkeyval: *const ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetKey2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: ::windows_core::PCWSTR, uflags: u32, ucimtype: u32, pkeyval: *const super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetKey2: usize,
    pub GetKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: ::windows_core::PWSTR, pukeyvalbufsize: *mut u32, pkeyval: *mut ::core::ffi::c_void, puapparentcimtype: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub GetKey2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: ::windows_core::PWSTR, pkeyvalue: *mut super::Com::VARIANT, puapparentcimtype: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    GetKey2: usize,
    pub RemoveKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: ::windows_core::PCWSTR, uflags: u32) -> ::windows_core::HRESULT,
    pub RemoveAllKeys: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uflags: u32) -> ::windows_core::HRESULT,
    pub MakeSingleton: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bset: u8) -> ::windows_core::HRESULT,
    pub GetInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, urequestedinfo: u32, puresponse: *mut u64) -> ::windows_core::HRESULT,
    pub GetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pubufflength: *mut u32, psztext: ::windows_core::PWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWbemPropertyProvider(::windows_core::IUnknown);
impl IWbemPropertyProvider {
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn GetProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, lflags: i32, strlocale: Param1, strclassmapping: Param2, strinstmapping: Param3, strpropmapping: Param4) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), strlocale.into_param().abi(), strclassmapping.into_param().abi(), strinstmapping.into_param().abi(), strpropmapping.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn PutProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, lflags: i32, strlocale: Param1, strclassmapping: Param2, strinstmapping: Param3, strpropmapping: Param4, pvvalue: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PutProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), strlocale.into_param().abi(), strclassmapping.into_param().abi(), strinstmapping.into_param().abi(), strpropmapping.into_param().abi(), ::core::mem::transmute(pvvalue)).ok()
    }
}
impl ::core::convert::From<IWbemPropertyProvider> for ::windows_core::IUnknown {
    fn from(value: IWbemPropertyProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemPropertyProvider> for ::windows_core::IUnknown {
    fn from(value: &IWbemPropertyProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemPropertyProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemPropertyProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemPropertyProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemPropertyProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemPropertyProvider {}
impl ::core::fmt::Debug for IWbemPropertyProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemPropertyProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemPropertyProvider {
    type Vtable = IWbemPropertyProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xce61e841_65bc_11d0_b6bd_00aa003240c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemPropertyProvider_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, strlocale: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strclassmapping: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strinstmapping: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strpropmapping: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pvvalue: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    GetProperty: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub PutProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, strlocale: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strclassmapping: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strinstmapping: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strpropmapping: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pvvalue: *const super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    PutProperty: usize,
}
#[repr(transparent)]
pub struct IWbemProviderIdentity(::windows_core::IUnknown);
impl IWbemProviderIdentity {
    pub unsafe fn SetRegistrationObject<'a, Param1: ::windows_core::IntoParam<'a, IWbemClassObject>>(&self, lflags: i32, pprovreg: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRegistrationObject)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), pprovreg.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWbemProviderIdentity> for ::windows_core::IUnknown {
    fn from(value: IWbemProviderIdentity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemProviderIdentity> for ::windows_core::IUnknown {
    fn from(value: &IWbemProviderIdentity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemProviderIdentity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemProviderIdentity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemProviderIdentity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemProviderIdentity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemProviderIdentity {}
impl ::core::fmt::Debug for IWbemProviderIdentity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemProviderIdentity").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemProviderIdentity {
    type Vtable = IWbemProviderIdentity_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x631f7d97_d993_11d2_b339_00105a1f4aaf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemProviderIdentity_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetRegistrationObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pprovreg: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWbemProviderInit(::windows_core::IUnknown);
impl IWbemProviderInit {
    pub unsafe fn Initialize<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param4: ::windows_core::IntoParam<'a, IWbemServices>, Param5: ::windows_core::IntoParam<'a, IWbemContext>, Param6: ::windows_core::IntoParam<'a, IWbemProviderInitSink>>(&self, wszuser: Param0, lflags: i32, wsznamespace: Param2, wszlocale: Param3, pnamespace: Param4, pctx: Param5, pinitsink: Param6) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), wszuser.into_param().abi(), ::core::mem::transmute(lflags), wsznamespace.into_param().abi(), wszlocale.into_param().abi(), pnamespace.into_param().abi(), pctx.into_param().abi(), pinitsink.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWbemProviderInit> for ::windows_core::IUnknown {
    fn from(value: IWbemProviderInit) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemProviderInit> for ::windows_core::IUnknown {
    fn from(value: &IWbemProviderInit) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemProviderInit {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemProviderInit {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemProviderInit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemProviderInit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemProviderInit {}
impl ::core::fmt::Debug for IWbemProviderInit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemProviderInit").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemProviderInit {
    type Vtable = IWbemProviderInit_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1be41572_91dd_11d1_aeb2_00c04fb68820);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemProviderInit_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszuser: ::windows_core::PCWSTR, lflags: i32, wsznamespace: ::windows_core::PCWSTR, wszlocale: ::windows_core::PCWSTR, pnamespace: ::windows_core::RawPtr, pctx: ::windows_core::RawPtr, pinitsink: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWbemProviderInitSink(::windows_core::IUnknown);
impl IWbemProviderInitSink {
    pub unsafe fn SetStatus(&self, lstatus: i32, lflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lstatus), ::core::mem::transmute(lflags)).ok()
    }
}
impl ::core::convert::From<IWbemProviderInitSink> for ::windows_core::IUnknown {
    fn from(value: IWbemProviderInitSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemProviderInitSink> for ::windows_core::IUnknown {
    fn from(value: &IWbemProviderInitSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemProviderInitSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemProviderInitSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemProviderInitSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemProviderInitSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemProviderInitSink {}
impl ::core::fmt::Debug for IWbemProviderInitSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemProviderInitSink").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemProviderInitSink {
    type Vtable = IWbemProviderInitSink_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1be41571_91dd_11d1_aeb2_00c04fb68820);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemProviderInitSink_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lstatus: i32, lflags: i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWbemQualifierSet(::windows_core::IUnknown);
impl IWbemQualifierSet {
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Get<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszname: Param0, lflags: i32, pval: *mut super::Com::VARIANT, plflavor: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Get)(::windows_core::Interface::as_raw(self), wszname.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(pval), ::core::mem::transmute(plflavor)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Put<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszname: Param0, pval: *const super::Com::VARIANT, lflavor: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Put)(::windows_core::Interface::as_raw(self), wszname.into_param().abi(), ::core::mem::transmute(pval), ::core::mem::transmute(lflavor)).ok()
    }
    pub unsafe fn Delete<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self), wszname.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetNames(&self, lflags: i32) -> ::windows_core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut super::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).GetNames)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn BeginEnumeration(&self, lflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginEnumeration)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Next(&self, lflags: i32, pstrname: *mut ::win32_foundation::BSTR, pval: *mut super::Com::VARIANT, plflavor: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), ::core::mem::transmute(pstrname), ::core::mem::transmute(pval), ::core::mem::transmute(plflavor)).ok()
    }
    pub unsafe fn EndEnumeration(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndEnumeration)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IWbemQualifierSet> for ::windows_core::IUnknown {
    fn from(value: IWbemQualifierSet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemQualifierSet> for ::windows_core::IUnknown {
    fn from(value: &IWbemQualifierSet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemQualifierSet {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemQualifierSet {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemQualifierSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemQualifierSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemQualifierSet {}
impl ::core::fmt::Debug for IWbemQualifierSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemQualifierSet").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemQualifierSet {
    type Vtable = IWbemQualifierSet_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdc12a680_737f_11cf_884d_00aa004b2e24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemQualifierSet_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Get: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: ::windows_core::PCWSTR, lflags: i32, pval: *mut super::Com::VARIANT, plflavor: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Get: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Put: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: ::windows_core::PCWSTR, pval: *const super::Com::VARIANT, lflavor: i32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Put: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub GetNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pnames: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetNames: usize,
    pub BeginEnumeration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pstrname: *mut ::win32_foundation::BSTR, pval: *mut super::Com::VARIANT, plflavor: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Next: usize,
    pub EndEnumeration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWbemQuery(::windows_core::IUnknown);
impl IWbemQuery {
    pub unsafe fn Empty(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Empty)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetLanguageFeatures(&self, uflags: u32, uarraysize: u32, pufeatures: *const u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLanguageFeatures)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uflags), ::core::mem::transmute(uarraysize), ::core::mem::transmute(pufeatures)).ok()
    }
    pub unsafe fn TestLanguageFeatures(&self, uflags: u32, uarraysize: *mut u32, pufeatures: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TestLanguageFeatures)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uflags), ::core::mem::transmute(uarraysize), ::core::mem::transmute(pufeatures)).ok()
    }
    pub unsafe fn Parse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszlang: Param0, pszquery: Param1, uflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Parse)(::windows_core::Interface::as_raw(self), pszlang.into_param().abi(), pszquery.into_param().abi(), ::core::mem::transmute(uflags)).ok()
    }
    pub unsafe fn GetAnalysis(&self, uanalysistype: u32, uflags: u32, panalysis: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetAnalysis)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uanalysistype), ::core::mem::transmute(uflags), ::core::mem::transmute(panalysis)).ok()
    }
    pub unsafe fn FreeMemory(&self, pmem: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FreeMemory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pmem)).ok()
    }
    pub unsafe fn GetQueryInfo(&self, uanalysistype: u32, uinfoid: u32, ubufsize: u32, pdestbuf: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetQueryInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uanalysistype), ::core::mem::transmute(uinfoid), ::core::mem::transmute(ubufsize), ::core::mem::transmute(pdestbuf)).ok()
    }
}
impl ::core::convert::From<IWbemQuery> for ::windows_core::IUnknown {
    fn from(value: IWbemQuery) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemQuery> for ::windows_core::IUnknown {
    fn from(value: &IWbemQuery) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemQuery {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemQuery {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemQuery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemQuery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemQuery {}
impl ::core::fmt::Debug for IWbemQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemQuery").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemQuery {
    type Vtable = IWbemQuery_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x81166f58_dd98_11d3_a120_00105a1f515a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemQuery_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Empty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetLanguageFeatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uflags: u32, uarraysize: u32, pufeatures: *const u32) -> ::windows_core::HRESULT,
    pub TestLanguageFeatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uflags: u32, uarraysize: *mut u32, pufeatures: *mut u32) -> ::windows_core::HRESULT,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszlang: ::windows_core::PCWSTR, pszquery: ::windows_core::PCWSTR, uflags: u32) -> ::windows_core::HRESULT,
    pub GetAnalysis: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uanalysistype: u32, uflags: u32, panalysis: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FreeMemory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmem: *const ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetQueryInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uanalysistype: u32, uinfoid: u32, ubufsize: u32, pdestbuf: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWbemRefresher(::windows_core::IUnknown);
impl IWbemRefresher {
    pub unsafe fn Refresh(&self, lflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags)).ok()
    }
}
impl ::core::convert::From<IWbemRefresher> for ::windows_core::IUnknown {
    fn from(value: IWbemRefresher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemRefresher> for ::windows_core::IUnknown {
    fn from(value: &IWbemRefresher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemRefresher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemRefresher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemRefresher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemRefresher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemRefresher {}
impl ::core::fmt::Debug for IWbemRefresher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemRefresher").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemRefresher {
    type Vtable = IWbemRefresher_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x49353c99_516b_11d1_aea6_00c04fb68820);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemRefresher_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWbemServices(::windows_core::IUnknown);
impl IWbemServices {
    pub unsafe fn OpenNamespace<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, IWbemContext>>(&self, strnamespace: Param0, lflags: i32, pctx: Param2, ppworkingnamespace: *mut ::core::option::Option<IWbemServices>, ppresult: *mut ::core::option::Option<IWbemCallResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OpenNamespace)(::windows_core::Interface::as_raw(self), strnamespace.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), ::core::mem::transmute(ppworkingnamespace), ::core::mem::transmute(ppresult)).ok()
    }
    pub unsafe fn CancelAsyncCall<'a, Param0: ::windows_core::IntoParam<'a, IWbemObjectSink>>(&self, psink: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CancelAsyncCall)(::windows_core::Interface::as_raw(self), psink.into_param().abi()).ok()
    }
    pub unsafe fn QueryObjectSink(&self, lflags: i32) -> ::windows_core::Result<IWbemObjectSink> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).QueryObjectSink)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWbemObjectSink>(result__)
    }
    pub unsafe fn GetObject<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, IWbemContext>>(&self, strobjectpath: Param0, lflags: i32, pctx: Param2, ppobject: *mut ::core::option::Option<IWbemClassObject>, ppcallresult: *mut ::core::option::Option<IWbemCallResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetObject)(::windows_core::Interface::as_raw(self), strobjectpath.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), ::core::mem::transmute(ppobject), ::core::mem::transmute(ppcallresult)).ok()
    }
    pub unsafe fn GetObjectAsync<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, IWbemContext>, Param3: ::windows_core::IntoParam<'a, IWbemObjectSink>>(&self, strobjectpath: Param0, lflags: i32, pctx: Param2, presponsehandler: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetObjectAsync)(::windows_core::Interface::as_raw(self), strobjectpath.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), presponsehandler.into_param().abi()).ok()
    }
    pub unsafe fn PutClass<'a, Param0: ::windows_core::IntoParam<'a, IWbemClassObject>, Param2: ::windows_core::IntoParam<'a, IWbemContext>>(&self, pobject: Param0, lflags: i32, pctx: Param2, ppcallresult: *mut ::core::option::Option<IWbemCallResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PutClass)(::windows_core::Interface::as_raw(self), pobject.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), ::core::mem::transmute(ppcallresult)).ok()
    }
    pub unsafe fn PutClassAsync<'a, Param0: ::windows_core::IntoParam<'a, IWbemClassObject>, Param2: ::windows_core::IntoParam<'a, IWbemContext>, Param3: ::windows_core::IntoParam<'a, IWbemObjectSink>>(&self, pobject: Param0, lflags: i32, pctx: Param2, presponsehandler: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PutClassAsync)(::windows_core::Interface::as_raw(self), pobject.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), presponsehandler.into_param().abi()).ok()
    }
    pub unsafe fn DeleteClass<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, IWbemContext>>(&self, strclass: Param0, lflags: i32, pctx: Param2, ppcallresult: *mut ::core::option::Option<IWbemCallResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteClass)(::windows_core::Interface::as_raw(self), strclass.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), ::core::mem::transmute(ppcallresult)).ok()
    }
    pub unsafe fn DeleteClassAsync<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, IWbemContext>, Param3: ::windows_core::IntoParam<'a, IWbemObjectSink>>(&self, strclass: Param0, lflags: i32, pctx: Param2, presponsehandler: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteClassAsync)(::windows_core::Interface::as_raw(self), strclass.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), presponsehandler.into_param().abi()).ok()
    }
    pub unsafe fn CreateClassEnum<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, IWbemContext>>(&self, strsuperclass: Param0, lflags: i32, pctx: Param2) -> ::windows_core::Result<IEnumWbemClassObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateClassEnum)(::windows_core::Interface::as_raw(self), strsuperclass.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumWbemClassObject>(result__)
    }
    pub unsafe fn CreateClassEnumAsync<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, IWbemContext>, Param3: ::windows_core::IntoParam<'a, IWbemObjectSink>>(&self, strsuperclass: Param0, lflags: i32, pctx: Param2, presponsehandler: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreateClassEnumAsync)(::windows_core::Interface::as_raw(self), strsuperclass.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), presponsehandler.into_param().abi()).ok()
    }
    pub unsafe fn PutInstance<'a, Param0: ::windows_core::IntoParam<'a, IWbemClassObject>, Param2: ::windows_core::IntoParam<'a, IWbemContext>>(&self, pinst: Param0, lflags: i32, pctx: Param2, ppcallresult: *mut ::core::option::Option<IWbemCallResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PutInstance)(::windows_core::Interface::as_raw(self), pinst.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), ::core::mem::transmute(ppcallresult)).ok()
    }
    pub unsafe fn PutInstanceAsync<'a, Param0: ::windows_core::IntoParam<'a, IWbemClassObject>, Param2: ::windows_core::IntoParam<'a, IWbemContext>, Param3: ::windows_core::IntoParam<'a, IWbemObjectSink>>(&self, pinst: Param0, lflags: i32, pctx: Param2, presponsehandler: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PutInstanceAsync)(::windows_core::Interface::as_raw(self), pinst.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), presponsehandler.into_param().abi()).ok()
    }
    pub unsafe fn DeleteInstance<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, IWbemContext>>(&self, strobjectpath: Param0, lflags: i32, pctx: Param2, ppcallresult: *mut ::core::option::Option<IWbemCallResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteInstance)(::windows_core::Interface::as_raw(self), strobjectpath.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), ::core::mem::transmute(ppcallresult)).ok()
    }
    pub unsafe fn DeleteInstanceAsync<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, IWbemContext>, Param3: ::windows_core::IntoParam<'a, IWbemObjectSink>>(&self, strobjectpath: Param0, lflags: i32, pctx: Param2, presponsehandler: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteInstanceAsync)(::windows_core::Interface::as_raw(self), strobjectpath.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), presponsehandler.into_param().abi()).ok()
    }
    pub unsafe fn CreateInstanceEnum<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, IWbemContext>>(&self, strfilter: Param0, lflags: i32, pctx: Param2) -> ::windows_core::Result<IEnumWbemClassObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateInstanceEnum)(::windows_core::Interface::as_raw(self), strfilter.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumWbemClassObject>(result__)
    }
    pub unsafe fn CreateInstanceEnumAsync<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, IWbemContext>, Param3: ::windows_core::IntoParam<'a, IWbemObjectSink>>(&self, strfilter: Param0, lflags: i32, pctx: Param2, presponsehandler: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreateInstanceEnumAsync)(::windows_core::Interface::as_raw(self), strfilter.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), presponsehandler.into_param().abi()).ok()
    }
    pub unsafe fn ExecQuery<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, IWbemContext>>(&self, strquerylanguage: Param0, strquery: Param1, lflags: i32, pctx: Param3) -> ::windows_core::Result<IEnumWbemClassObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ExecQuery)(::windows_core::Interface::as_raw(self), strquerylanguage.into_param().abi(), strquery.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumWbemClassObject>(result__)
    }
    pub unsafe fn ExecQueryAsync<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, IWbemContext>, Param4: ::windows_core::IntoParam<'a, IWbemObjectSink>>(&self, strquerylanguage: Param0, strquery: Param1, lflags: i32, pctx: Param3, presponsehandler: Param4) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ExecQueryAsync)(::windows_core::Interface::as_raw(self), strquerylanguage.into_param().abi(), strquery.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), presponsehandler.into_param().abi()).ok()
    }
    pub unsafe fn ExecNotificationQuery<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, IWbemContext>>(&self, strquerylanguage: Param0, strquery: Param1, lflags: i32, pctx: Param3) -> ::windows_core::Result<IEnumWbemClassObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ExecNotificationQuery)(::windows_core::Interface::as_raw(self), strquerylanguage.into_param().abi(), strquery.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumWbemClassObject>(result__)
    }
    pub unsafe fn ExecNotificationQueryAsync<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, IWbemContext>, Param4: ::windows_core::IntoParam<'a, IWbemObjectSink>>(&self, strquerylanguage: Param0, strquery: Param1, lflags: i32, pctx: Param3, presponsehandler: Param4) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ExecNotificationQueryAsync)(::windows_core::Interface::as_raw(self), strquerylanguage.into_param().abi(), strquery.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), presponsehandler.into_param().abi()).ok()
    }
    pub unsafe fn ExecMethod<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, IWbemContext>, Param4: ::windows_core::IntoParam<'a, IWbemClassObject>>(&self, strobjectpath: Param0, strmethodname: Param1, lflags: i32, pctx: Param3, pinparams: Param4, ppoutparams: *mut ::core::option::Option<IWbemClassObject>, ppcallresult: *mut ::core::option::Option<IWbemCallResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ExecMethod)(::windows_core::Interface::as_raw(self), strobjectpath.into_param().abi(), strmethodname.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), pinparams.into_param().abi(), ::core::mem::transmute(ppoutparams), ::core::mem::transmute(ppcallresult)).ok()
    }
    pub unsafe fn ExecMethodAsync<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, IWbemContext>, Param4: ::windows_core::IntoParam<'a, IWbemClassObject>, Param5: ::windows_core::IntoParam<'a, IWbemObjectSink>>(&self, strobjectpath: Param0, strmethodname: Param1, lflags: i32, pctx: Param3, pinparams: Param4, presponsehandler: Param5) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ExecMethodAsync)(::windows_core::Interface::as_raw(self), strobjectpath.into_param().abi(), strmethodname.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), pinparams.into_param().abi(), presponsehandler.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWbemServices> for ::windows_core::IUnknown {
    fn from(value: IWbemServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemServices> for ::windows_core::IUnknown {
    fn from(value: &IWbemServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemServices {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemServices {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemServices {}
impl ::core::fmt::Debug for IWbemServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemServices").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemServices {
    type Vtable = IWbemServices_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9556dc99_828c_11cf_a37e_00aa003240c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemServices_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OpenNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strnamespace: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, lflags: i32, pctx: ::windows_core::RawPtr, ppworkingnamespace: *mut ::windows_core::RawPtr, ppresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CancelAsyncCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub QueryObjectSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, ppresponsehandler: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, lflags: i32, pctx: ::windows_core::RawPtr, ppobject: *mut ::windows_core::RawPtr, ppcallresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetObjectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, lflags: i32, pctx: ::windows_core::RawPtr, presponsehandler: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PutClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobject: ::windows_core::RawPtr, lflags: i32, pctx: ::windows_core::RawPtr, ppcallresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PutClassAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobject: ::windows_core::RawPtr, lflags: i32, pctx: ::windows_core::RawPtr, presponsehandler: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DeleteClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strclass: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, lflags: i32, pctx: ::windows_core::RawPtr, ppcallresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DeleteClassAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strclass: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, lflags: i32, pctx: ::windows_core::RawPtr, presponsehandler: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateClassEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strsuperclass: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, lflags: i32, pctx: ::windows_core::RawPtr, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateClassEnumAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strsuperclass: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, lflags: i32, pctx: ::windows_core::RawPtr, presponsehandler: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PutInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinst: ::windows_core::RawPtr, lflags: i32, pctx: ::windows_core::RawPtr, ppcallresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PutInstanceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinst: ::windows_core::RawPtr, lflags: i32, pctx: ::windows_core::RawPtr, presponsehandler: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DeleteInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, lflags: i32, pctx: ::windows_core::RawPtr, ppcallresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DeleteInstanceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, lflags: i32, pctx: ::windows_core::RawPtr, presponsehandler: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateInstanceEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strfilter: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, lflags: i32, pctx: ::windows_core::RawPtr, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateInstanceEnumAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strfilter: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, lflags: i32, pctx: ::windows_core::RawPtr, presponsehandler: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ExecQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strquerylanguage: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strquery: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, lflags: i32, pctx: ::windows_core::RawPtr, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ExecQueryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strquerylanguage: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strquery: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, lflags: i32, pctx: ::windows_core::RawPtr, presponsehandler: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ExecNotificationQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strquerylanguage: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strquery: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, lflags: i32, pctx: ::windows_core::RawPtr, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ExecNotificationQueryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strquerylanguage: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strquery: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, lflags: i32, pctx: ::windows_core::RawPtr, presponsehandler: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ExecMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strmethodname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, lflags: i32, pctx: ::windows_core::RawPtr, pinparams: ::windows_core::RawPtr, ppoutparams: *mut ::windows_core::RawPtr, ppcallresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ExecMethodAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strmethodname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, lflags: i32, pctx: ::windows_core::RawPtr, pinparams: ::windows_core::RawPtr, presponsehandler: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWbemShutdown(::windows_core::IUnknown);
impl IWbemShutdown {
    pub unsafe fn Shutdown<'a, Param2: ::windows_core::IntoParam<'a, IWbemContext>>(&self, ureason: i32, umaxmilliseconds: u32, pctx: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Shutdown)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ureason), ::core::mem::transmute(umaxmilliseconds), pctx.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWbemShutdown> for ::windows_core::IUnknown {
    fn from(value: IWbemShutdown) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemShutdown> for ::windows_core::IUnknown {
    fn from(value: &IWbemShutdown) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemShutdown {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemShutdown {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemShutdown {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemShutdown {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemShutdown {}
impl ::core::fmt::Debug for IWbemShutdown {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemShutdown").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemShutdown {
    type Vtable = IWbemShutdown_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb7b31df9_d515_11d3_a11c_00105a1f515a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemShutdown_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Shutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ureason: i32, umaxmilliseconds: u32, pctx: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWbemStatusCodeText(::windows_core::IUnknown);
impl IWbemStatusCodeText {
    pub unsafe fn GetErrorCodeText(&self, hres: ::windows_core::HRESULT, localeid: u32, lflags: i32) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetErrorCodeText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hres), ::core::mem::transmute(localeid), ::core::mem::transmute(lflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetFacilityCodeText(&self, hres: ::windows_core::HRESULT, localeid: u32, lflags: i32) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetFacilityCodeText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hres), ::core::mem::transmute(localeid), ::core::mem::transmute(lflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<IWbemStatusCodeText> for ::windows_core::IUnknown {
    fn from(value: IWbemStatusCodeText) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemStatusCodeText> for ::windows_core::IUnknown {
    fn from(value: &IWbemStatusCodeText) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemStatusCodeText {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemStatusCodeText {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemStatusCodeText {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemStatusCodeText {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemStatusCodeText {}
impl ::core::fmt::Debug for IWbemStatusCodeText {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemStatusCodeText").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemStatusCodeText {
    type Vtable = IWbemStatusCodeText_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeb87e1bc_3233_11d2_aec9_00c04fb68820);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemStatusCodeText_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetErrorCodeText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hres: ::windows_core::HRESULT, localeid: u32, lflags: i32, messagetext: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetFacilityCodeText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hres: ::windows_core::HRESULT, localeid: u32, lflags: i32, messagetext: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWbemTransport(::windows_core::IUnknown);
impl IWbemTransport {
    pub unsafe fn Initialize(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IWbemTransport> for ::windows_core::IUnknown {
    fn from(value: IWbemTransport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemTransport> for ::windows_core::IUnknown {
    fn from(value: &IWbemTransport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemTransport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemTransport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemTransport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemTransport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemTransport {}
impl ::core::fmt::Debug for IWbemTransport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemTransport").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemTransport {
    type Vtable = IWbemTransport_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x553fe584_2156_11d0_b6ae_00aa003240c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemTransport_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWbemUnboundObjectSink(::windows_core::IUnknown);
impl IWbemUnboundObjectSink {
    pub unsafe fn IndicateToConsumer<'a, Param0: ::windows_core::IntoParam<'a, IWbemClassObject>>(&self, plogicalconsumer: Param0, apobjects: &[::core::option::Option<IWbemClassObject>]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IndicateToConsumer)(::windows_core::Interface::as_raw(self), plogicalconsumer.into_param().abi(), apobjects.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(apobjects))).ok()
    }
}
impl ::core::convert::From<IWbemUnboundObjectSink> for ::windows_core::IUnknown {
    fn from(value: IWbemUnboundObjectSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemUnboundObjectSink> for ::windows_core::IUnknown {
    fn from(value: &IWbemUnboundObjectSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemUnboundObjectSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemUnboundObjectSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemUnboundObjectSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemUnboundObjectSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemUnboundObjectSink {}
impl ::core::fmt::Debug for IWbemUnboundObjectSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemUnboundObjectSink").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemUnboundObjectSink {
    type Vtable = IWbemUnboundObjectSink_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe246107b_b06e_11d0_ad61_00c04fd8fdff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemUnboundObjectSink_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub IndicateToConsumer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plogicalconsumer: ::windows_core::RawPtr, lnumobjects: i32, apobjects: *const ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWbemUnsecuredApartment(::windows_core::IUnknown);
impl IWbemUnsecuredApartment {
    pub unsafe fn CreateObjectStub<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, pobject: Param0) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateObjectStub)(::windows_core::Interface::as_raw(self), pobject.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn CreateSinkStub<'a, Param0: ::windows_core::IntoParam<'a, IWbemObjectSink>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, psink: Param0, dwflags: u32, wszreserved: Param2) -> ::windows_core::Result<IWbemObjectSink> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateSinkStub)(::windows_core::Interface::as_raw(self), psink.into_param().abi(), ::core::mem::transmute(dwflags), wszreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWbemObjectSink>(result__)
    }
}
impl ::core::convert::From<IWbemUnsecuredApartment> for ::windows_core::IUnknown {
    fn from(value: IWbemUnsecuredApartment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemUnsecuredApartment> for ::windows_core::IUnknown {
    fn from(value: &IWbemUnsecuredApartment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWbemUnsecuredApartment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWbemUnsecuredApartment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWbemUnsecuredApartment> for IUnsecuredApartment {
    fn from(value: IWbemUnsecuredApartment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemUnsecuredApartment> for IUnsecuredApartment {
    fn from(value: &IWbemUnsecuredApartment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IUnsecuredApartment> for IWbemUnsecuredApartment {
    fn into_param(self) -> ::windows_core::Param<'a, IUnsecuredApartment> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IUnsecuredApartment> for &'a IWbemUnsecuredApartment {
    fn into_param(self) -> ::windows_core::Param<'a, IUnsecuredApartment> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemUnsecuredApartment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemUnsecuredApartment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemUnsecuredApartment {}
impl ::core::fmt::Debug for IWbemUnsecuredApartment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemUnsecuredApartment").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWbemUnsecuredApartment {
    type Vtable = IWbemUnsecuredApartment_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x31739d04_3471_4cf4_9a7c_57a44ae71956);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemUnsecuredApartment_Vtbl {
    pub base__: IUnsecuredApartment_Vtbl,
    pub CreateSinkStub: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: ::windows_core::RawPtr, dwflags: u32, wszreserved: ::windows_core::PCWSTR, ppstub: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(C)]
pub struct MI_Application {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_ApplicationFT,
}
impl ::core::marker::Copy for MI_Application {}
impl ::core::clone::Clone for MI_Application {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Application {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Application").field("reserved1", &self.reserved1).field("reserved2", &self.reserved2).field("ft", &self.ft).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Application {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Application {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Application>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Application {}
impl ::core::default::Default for MI_Application {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ApplicationFT {
    pub Close: isize,
    pub NewSession: isize,
    pub NewHostedProvider: isize,
    pub NewInstance: isize,
    pub NewDestinationOptions: isize,
    pub NewOperationOptions: isize,
    pub NewSubscriptionDeliveryOptions: isize,
    pub NewSerializer: isize,
    pub NewDeserializer: isize,
    pub NewInstanceFromClass: isize,
    pub NewClass: isize,
}
impl ::core::marker::Copy for MI_ApplicationFT {}
impl ::core::clone::Clone for MI_ApplicationFT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ApplicationFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ApplicationFT")
            .field("Close", &self.Close)
            .field("NewSession", &self.NewSession)
            .field("NewHostedProvider", &self.NewHostedProvider)
            .field("NewInstance", &self.NewInstance)
            .field("NewDestinationOptions", &self.NewDestinationOptions)
            .field("NewOperationOptions", &self.NewOperationOptions)
            .field("NewSubscriptionDeliveryOptions", &self.NewSubscriptionDeliveryOptions)
            .field("NewSerializer", &self.NewSerializer)
            .field("NewDeserializer", &self.NewDeserializer)
            .field("NewInstanceFromClass", &self.NewInstanceFromClass)
            .field("NewClass", &self.NewClass)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ApplicationFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ApplicationFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ApplicationFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ApplicationFT {}
impl ::core::default::Default for MI_ApplicationFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[inline]
pub unsafe fn MI_Application_InitializeV1(flags: u32, applicationid: *const u16, extendederror: *mut *mut MI_Instance, application: *mut MI_Application) -> MI_Result {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MI_Application_InitializeV1(flags: u32, applicationid: *const u16, extendederror: *mut *mut MI_Instance, application: *mut MI_Application) -> MI_Result;
        }
        ::core::mem::transmute(MI_Application_InitializeV1(::core::mem::transmute(flags), ::core::mem::transmute(applicationid), ::core::mem::transmute(extendederror), ::core::mem::transmute(application)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
pub struct MI_Array {
    pub data: *mut ::core::ffi::c_void,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Array {}
impl ::core::clone::Clone for MI_Array {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Array {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Array").field("data", &self.data).field("size", &self.size).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Array {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Array {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Array>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Array {}
impl ::core::default::Default for MI_Array {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ArrayField {
    pub value: MI_Array,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ArrayField {}
impl ::core::clone::Clone for MI_ArrayField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ArrayField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ArrayField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ArrayField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ArrayField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ArrayField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ArrayField {}
impl ::core::default::Default for MI_ArrayField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_BooleanA {
    pub data: *mut u8,
    pub size: u32,
}
impl ::core::marker::Copy for MI_BooleanA {}
impl ::core::clone::Clone for MI_BooleanA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_BooleanA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_BooleanA").field("data", &self.data).field("size", &self.size).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_BooleanA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_BooleanA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_BooleanA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_BooleanA {}
impl ::core::default::Default for MI_BooleanA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_BooleanAField {
    pub value: MI_BooleanA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_BooleanAField {}
impl ::core::clone::Clone for MI_BooleanAField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_BooleanAField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_BooleanAField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_BooleanAField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_BooleanAField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_BooleanAField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_BooleanAField {}
impl ::core::default::Default for MI_BooleanAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_BooleanField {
    pub value: u8,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_BooleanField {}
impl ::core::clone::Clone for MI_BooleanField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_BooleanField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_BooleanField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_BooleanField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_BooleanField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_BooleanField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_BooleanField {}
impl ::core::default::Default for MI_BooleanField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const MI_CALL_VERSION: u32 = 1u32;
pub const MI_CHAR_TYPE: u32 = 2u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MI_CallbackMode(pub i32);
pub const MI_CALLBACKMODE_REPORT: MI_CallbackMode = MI_CallbackMode(0i32);
pub const MI_CALLBACKMODE_INQUIRE: MI_CallbackMode = MI_CallbackMode(1i32);
pub const MI_CALLBACKMODE_IGNORE: MI_CallbackMode = MI_CallbackMode(2i32);
impl ::core::marker::Copy for MI_CallbackMode {}
impl ::core::clone::Clone for MI_CallbackMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MI_CallbackMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MI_CallbackMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for MI_CallbackMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MI_CallbackMode").field(&self.0).finish()
    }
}
pub type MI_CancelCallback = ::core::option::Option<unsafe extern "system" fn(reason: MI_CancellationReason, callbackdata: *const ::core::ffi::c_void)>;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MI_CancellationReason(pub i32);
pub const MI_REASON_NONE: MI_CancellationReason = MI_CancellationReason(0i32);
pub const MI_REASON_TIMEOUT: MI_CancellationReason = MI_CancellationReason(1i32);
pub const MI_REASON_SHUTDOWN: MI_CancellationReason = MI_CancellationReason(2i32);
pub const MI_REASON_SERVICESTOP: MI_CancellationReason = MI_CancellationReason(3i32);
impl ::core::marker::Copy for MI_CancellationReason {}
impl ::core::clone::Clone for MI_CancellationReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MI_CancellationReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MI_CancellationReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for MI_CancellationReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MI_CancellationReason").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct MI_Char16A {
    pub data: *mut u16,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Char16A {}
impl ::core::clone::Clone for MI_Char16A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Char16A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Char16A").field("data", &self.data).field("size", &self.size).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Char16A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Char16A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Char16A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Char16A {}
impl ::core::default::Default for MI_Char16A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Char16AField {
    pub value: MI_Char16A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Char16AField {}
impl ::core::clone::Clone for MI_Char16AField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Char16AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Char16AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Char16AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Char16AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Char16AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Char16AField {}
impl ::core::default::Default for MI_Char16AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Char16Field {
    pub value: u16,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Char16Field {}
impl ::core::clone::Clone for MI_Char16Field {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Char16Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Char16Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Char16Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Char16Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Char16Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Char16Field {}
impl ::core::default::Default for MI_Char16Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Class {
    pub ft: *const MI_ClassFT,
    pub classDecl: *const MI_ClassDecl,
    pub namespaceName: *const u16,
    pub serverName: *const u16,
    pub reserved: [isize; 4],
}
impl ::core::marker::Copy for MI_Class {}
impl ::core::clone::Clone for MI_Class {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Class {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Class").field("ft", &self.ft).field("classDecl", &self.classDecl).field("namespaceName", &self.namespaceName).field("serverName", &self.serverName).field("reserved", &self.reserved).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Class {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Class {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Class>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Class {}
impl ::core::default::Default for MI_Class {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ClassDecl {
    pub flags: u32,
    pub code: u32,
    pub name: *const u16,
    pub qualifiers: *const *const MI_Qualifier,
    pub numQualifiers: u32,
    pub properties: *const *const MI_PropertyDecl,
    pub numProperties: u32,
    pub size: u32,
    pub superClass: *const u16,
    pub superClassDecl: *const MI_ClassDecl,
    pub methods: *const *const MI_MethodDecl,
    pub numMethods: u32,
    pub schema: *const MI_SchemaDecl,
    pub providerFT: *const MI_ProviderFT,
    pub owningClass: *mut MI_Class,
}
impl ::core::marker::Copy for MI_ClassDecl {}
impl ::core::clone::Clone for MI_ClassDecl {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ClassDecl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ClassDecl")
            .field("flags", &self.flags)
            .field("code", &self.code)
            .field("name", &self.name)
            .field("qualifiers", &self.qualifiers)
            .field("numQualifiers", &self.numQualifiers)
            .field("properties", &self.properties)
            .field("numProperties", &self.numProperties)
            .field("size", &self.size)
            .field("superClass", &self.superClass)
            .field("superClassDecl", &self.superClassDecl)
            .field("methods", &self.methods)
            .field("numMethods", &self.numMethods)
            .field("schema", &self.schema)
            .field("providerFT", &self.providerFT)
            .field("owningClass", &self.owningClass)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ClassDecl {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ClassDecl {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ClassDecl>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ClassDecl {}
impl ::core::default::Default for MI_ClassDecl {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ClassFT {
    pub GetClassNameA: isize,
    pub GetNameSpace: isize,
    pub GetServerName: isize,
    pub GetElementCount: isize,
    pub GetElement: isize,
    pub GetElementAt: isize,
    pub GetClassQualifierSet: isize,
    pub GetMethodCount: isize,
    pub GetMethodAt: isize,
    pub GetMethod: isize,
    pub GetParentClassName: isize,
    pub GetParentClass: isize,
    pub Delete: isize,
    pub Clone: isize,
}
impl ::core::marker::Copy for MI_ClassFT {}
impl ::core::clone::Clone for MI_ClassFT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ClassFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ClassFT")
            .field("GetClassNameA", &self.GetClassNameA)
            .field("GetNameSpace", &self.GetNameSpace)
            .field("GetServerName", &self.GetServerName)
            .field("GetElementCount", &self.GetElementCount)
            .field("GetElement", &self.GetElement)
            .field("GetElementAt", &self.GetElementAt)
            .field("GetClassQualifierSet", &self.GetClassQualifierSet)
            .field("GetMethodCount", &self.GetMethodCount)
            .field("GetMethodAt", &self.GetMethodAt)
            .field("GetMethod", &self.GetMethod)
            .field("GetParentClassName", &self.GetParentClassName)
            .field("GetParentClass", &self.GetParentClass)
            .field("Delete", &self.Delete)
            .field("Clone", &self.Clone)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ClassFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ClassFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ClassFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ClassFT {}
impl ::core::default::Default for MI_ClassFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ClientFT_V1 {
    pub applicationFT: *const MI_ApplicationFT,
    pub sessionFT: *const MI_SessionFT,
    pub operationFT: *const MI_OperationFT,
    pub hostedProviderFT: *const MI_HostedProviderFT,
    pub serializerFT: *const MI_SerializerFT,
    pub deserializerFT: *const MI_DeserializerFT,
    pub subscribeDeliveryOptionsFT: *const MI_SubscriptionDeliveryOptionsFT,
    pub destinationOptionsFT: *const MI_DestinationOptionsFT,
    pub operationOptionsFT: *const MI_OperationOptionsFT,
    pub utilitiesFT: *const MI_UtilitiesFT,
}
impl ::core::marker::Copy for MI_ClientFT_V1 {}
impl ::core::clone::Clone for MI_ClientFT_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ClientFT_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ClientFT_V1")
            .field("applicationFT", &self.applicationFT)
            .field("sessionFT", &self.sessionFT)
            .field("operationFT", &self.operationFT)
            .field("hostedProviderFT", &self.hostedProviderFT)
            .field("serializerFT", &self.serializerFT)
            .field("deserializerFT", &self.deserializerFT)
            .field("subscribeDeliveryOptionsFT", &self.subscribeDeliveryOptionsFT)
            .field("destinationOptionsFT", &self.destinationOptionsFT)
            .field("operationOptionsFT", &self.operationOptionsFT)
            .field("utilitiesFT", &self.utilitiesFT)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ClientFT_V1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ClientFT_V1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ClientFT_V1>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ClientFT_V1 {}
impl ::core::default::Default for MI_ClientFT_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstBooleanA {
    pub data: *const u8,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstBooleanA {}
impl ::core::clone::Clone for MI_ConstBooleanA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstBooleanA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstBooleanA").field("data", &self.data).field("size", &self.size).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstBooleanA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstBooleanA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstBooleanA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstBooleanA {}
impl ::core::default::Default for MI_ConstBooleanA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstBooleanAField {
    pub value: MI_ConstBooleanA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstBooleanAField {}
impl ::core::clone::Clone for MI_ConstBooleanAField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstBooleanAField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstBooleanAField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstBooleanAField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstBooleanAField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstBooleanAField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstBooleanAField {}
impl ::core::default::Default for MI_ConstBooleanAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstBooleanField {
    pub value: u8,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstBooleanField {}
impl ::core::clone::Clone for MI_ConstBooleanField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstBooleanField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstBooleanField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstBooleanField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstBooleanField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstBooleanField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstBooleanField {}
impl ::core::default::Default for MI_ConstBooleanField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstChar16A {
    pub data: *const u16,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstChar16A {}
impl ::core::clone::Clone for MI_ConstChar16A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstChar16A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstChar16A").field("data", &self.data).field("size", &self.size).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstChar16A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstChar16A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstChar16A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstChar16A {}
impl ::core::default::Default for MI_ConstChar16A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstChar16AField {
    pub value: MI_ConstChar16A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstChar16AField {}
impl ::core::clone::Clone for MI_ConstChar16AField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstChar16AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstChar16AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstChar16AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstChar16AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstChar16AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstChar16AField {}
impl ::core::default::Default for MI_ConstChar16AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstChar16Field {
    pub value: u16,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstChar16Field {}
impl ::core::clone::Clone for MI_ConstChar16Field {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstChar16Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstChar16Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstChar16Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstChar16Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstChar16Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstChar16Field {}
impl ::core::default::Default for MI_ConstChar16Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstDatetimeA {
    pub data: *const MI_Datetime,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstDatetimeA {}
impl ::core::clone::Clone for MI_ConstDatetimeA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstDatetimeA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstDatetimeA").field("data", &self.data).field("size", &self.size).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstDatetimeA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstDatetimeA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstDatetimeA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstDatetimeA {}
impl ::core::default::Default for MI_ConstDatetimeA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstDatetimeAField {
    pub value: MI_ConstDatetimeA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstDatetimeAField {}
impl ::core::clone::Clone for MI_ConstDatetimeAField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstDatetimeAField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstDatetimeAField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstDatetimeAField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstDatetimeAField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstDatetimeAField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstDatetimeAField {}
impl ::core::default::Default for MI_ConstDatetimeAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstDatetimeField {
    pub value: MI_Datetime,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstDatetimeField {}
impl ::core::clone::Clone for MI_ConstDatetimeField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for MI_ConstDatetimeField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstDatetimeField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstDatetimeField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstDatetimeField {}
impl ::core::default::Default for MI_ConstDatetimeField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstInstanceA {
    pub data: *const *const MI_Instance,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstInstanceA {}
impl ::core::clone::Clone for MI_ConstInstanceA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstInstanceA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstInstanceA").field("data", &self.data).field("size", &self.size).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstInstanceA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstInstanceA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstInstanceA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstInstanceA {}
impl ::core::default::Default for MI_ConstInstanceA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstInstanceAField {
    pub value: MI_ConstInstanceA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstInstanceAField {}
impl ::core::clone::Clone for MI_ConstInstanceAField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstInstanceAField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstInstanceAField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstInstanceAField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstInstanceAField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstInstanceAField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstInstanceAField {}
impl ::core::default::Default for MI_ConstInstanceAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstInstanceField {
    pub value: *const MI_Instance,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstInstanceField {}
impl ::core::clone::Clone for MI_ConstInstanceField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstInstanceField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstInstanceField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstInstanceField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstInstanceField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstInstanceField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstInstanceField {}
impl ::core::default::Default for MI_ConstInstanceField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstReal32A {
    pub data: *const f32,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstReal32A {}
impl ::core::clone::Clone for MI_ConstReal32A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstReal32A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstReal32A").field("data", &self.data).field("size", &self.size).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstReal32A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstReal32A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstReal32A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstReal32A {}
impl ::core::default::Default for MI_ConstReal32A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstReal32AField {
    pub value: MI_ConstReal32A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstReal32AField {}
impl ::core::clone::Clone for MI_ConstReal32AField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstReal32AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstReal32AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstReal32AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstReal32AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstReal32AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstReal32AField {}
impl ::core::default::Default for MI_ConstReal32AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstReal32Field {
    pub value: f32,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstReal32Field {}
impl ::core::clone::Clone for MI_ConstReal32Field {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstReal32Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstReal32Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstReal32Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstReal32Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstReal32Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstReal32Field {}
impl ::core::default::Default for MI_ConstReal32Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstReal64A {
    pub data: *const f64,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstReal64A {}
impl ::core::clone::Clone for MI_ConstReal64A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstReal64A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstReal64A").field("data", &self.data).field("size", &self.size).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstReal64A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstReal64A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstReal64A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstReal64A {}
impl ::core::default::Default for MI_ConstReal64A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstReal64AField {
    pub value: MI_ConstReal64A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstReal64AField {}
impl ::core::clone::Clone for MI_ConstReal64AField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstReal64AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstReal64AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstReal64AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstReal64AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstReal64AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstReal64AField {}
impl ::core::default::Default for MI_ConstReal64AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstReal64Field {
    pub value: f64,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstReal64Field {}
impl ::core::clone::Clone for MI_ConstReal64Field {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstReal64Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstReal64Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstReal64Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstReal64Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstReal64Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstReal64Field {}
impl ::core::default::Default for MI_ConstReal64Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstReferenceA {
    pub data: *const *const MI_Instance,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstReferenceA {}
impl ::core::clone::Clone for MI_ConstReferenceA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstReferenceA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstReferenceA").field("data", &self.data).field("size", &self.size).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstReferenceA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstReferenceA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstReferenceA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstReferenceA {}
impl ::core::default::Default for MI_ConstReferenceA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstReferenceAField {
    pub value: MI_ConstReferenceA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstReferenceAField {}
impl ::core::clone::Clone for MI_ConstReferenceAField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstReferenceAField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstReferenceAField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstReferenceAField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstReferenceAField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstReferenceAField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstReferenceAField {}
impl ::core::default::Default for MI_ConstReferenceAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstReferenceField {
    pub value: *const MI_Instance,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstReferenceField {}
impl ::core::clone::Clone for MI_ConstReferenceField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstReferenceField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstReferenceField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstReferenceField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstReferenceField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstReferenceField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstReferenceField {}
impl ::core::default::Default for MI_ConstReferenceField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstSint16A {
    pub data: *const i16,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstSint16A {}
impl ::core::clone::Clone for MI_ConstSint16A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstSint16A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstSint16A").field("data", &self.data).field("size", &self.size).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstSint16A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstSint16A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstSint16A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstSint16A {}
impl ::core::default::Default for MI_ConstSint16A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstSint16AField {
    pub value: MI_ConstSint16A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstSint16AField {}
impl ::core::clone::Clone for MI_ConstSint16AField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstSint16AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstSint16AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstSint16AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstSint16AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstSint16AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstSint16AField {}
impl ::core::default::Default for MI_ConstSint16AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstSint16Field {
    pub value: i16,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstSint16Field {}
impl ::core::clone::Clone for MI_ConstSint16Field {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstSint16Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstSint16Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstSint16Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstSint16Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstSint16Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstSint16Field {}
impl ::core::default::Default for MI_ConstSint16Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstSint32A {
    pub data: *const i32,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstSint32A {}
impl ::core::clone::Clone for MI_ConstSint32A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstSint32A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstSint32A").field("data", &self.data).field("size", &self.size).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstSint32A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstSint32A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstSint32A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstSint32A {}
impl ::core::default::Default for MI_ConstSint32A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstSint32AField {
    pub value: MI_ConstSint32A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstSint32AField {}
impl ::core::clone::Clone for MI_ConstSint32AField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstSint32AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstSint32AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstSint32AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstSint32AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstSint32AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstSint32AField {}
impl ::core::default::Default for MI_ConstSint32AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstSint32Field {
    pub value: i32,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstSint32Field {}
impl ::core::clone::Clone for MI_ConstSint32Field {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstSint32Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstSint32Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstSint32Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstSint32Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstSint32Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstSint32Field {}
impl ::core::default::Default for MI_ConstSint32Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstSint64A {
    pub data: *const i64,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstSint64A {}
impl ::core::clone::Clone for MI_ConstSint64A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstSint64A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstSint64A").field("data", &self.data).field("size", &self.size).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstSint64A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstSint64A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstSint64A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstSint64A {}
impl ::core::default::Default for MI_ConstSint64A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstSint64AField {
    pub value: MI_ConstSint64A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstSint64AField {}
impl ::core::clone::Clone for MI_ConstSint64AField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstSint64AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstSint64AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstSint64AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstSint64AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstSint64AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstSint64AField {}
impl ::core::default::Default for MI_ConstSint64AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstSint64Field {
    pub value: i64,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstSint64Field {}
impl ::core::clone::Clone for MI_ConstSint64Field {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstSint64Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstSint64Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstSint64Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstSint64Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstSint64Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstSint64Field {}
impl ::core::default::Default for MI_ConstSint64Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstSint8A {
    pub data: *const i8,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstSint8A {}
impl ::core::clone::Clone for MI_ConstSint8A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstSint8A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstSint8A").field("data", &self.data).field("size", &self.size).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstSint8A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstSint8A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstSint8A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstSint8A {}
impl ::core::default::Default for MI_ConstSint8A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstSint8AField {
    pub value: MI_ConstSint8A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstSint8AField {}
impl ::core::clone::Clone for MI_ConstSint8AField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstSint8AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstSint8AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstSint8AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstSint8AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstSint8AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstSint8AField {}
impl ::core::default::Default for MI_ConstSint8AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstSint8Field {
    pub value: i8,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstSint8Field {}
impl ::core::clone::Clone for MI_ConstSint8Field {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstSint8Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstSint8Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstSint8Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstSint8Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstSint8Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstSint8Field {}
impl ::core::default::Default for MI_ConstSint8Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstStringA {
    pub data: *const *const u16,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstStringA {}
impl ::core::clone::Clone for MI_ConstStringA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstStringA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstStringA").field("data", &self.data).field("size", &self.size).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstStringA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstStringA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstStringA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstStringA {}
impl ::core::default::Default for MI_ConstStringA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstStringAField {
    pub value: MI_ConstStringA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstStringAField {}
impl ::core::clone::Clone for MI_ConstStringAField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstStringAField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstStringAField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstStringAField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstStringAField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstStringAField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstStringAField {}
impl ::core::default::Default for MI_ConstStringAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstStringField {
    pub value: *const u16,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstStringField {}
impl ::core::clone::Clone for MI_ConstStringField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstStringField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstStringField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstStringField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstStringField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstStringField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstStringField {}
impl ::core::default::Default for MI_ConstStringField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstUint16A {
    pub data: *const u16,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstUint16A {}
impl ::core::clone::Clone for MI_ConstUint16A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstUint16A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstUint16A").field("data", &self.data).field("size", &self.size).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstUint16A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstUint16A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstUint16A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstUint16A {}
impl ::core::default::Default for MI_ConstUint16A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstUint16AField {
    pub value: MI_ConstUint16A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstUint16AField {}
impl ::core::clone::Clone for MI_ConstUint16AField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstUint16AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstUint16AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstUint16AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstUint16AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstUint16AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstUint16AField {}
impl ::core::default::Default for MI_ConstUint16AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstUint16Field {
    pub value: u16,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstUint16Field {}
impl ::core::clone::Clone for MI_ConstUint16Field {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstUint16Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstUint16Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstUint16Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstUint16Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstUint16Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstUint16Field {}
impl ::core::default::Default for MI_ConstUint16Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstUint32A {
    pub data: *const u32,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstUint32A {}
impl ::core::clone::Clone for MI_ConstUint32A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstUint32A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstUint32A").field("data", &self.data).field("size", &self.size).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstUint32A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstUint32A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstUint32A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstUint32A {}
impl ::core::default::Default for MI_ConstUint32A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstUint32AField {
    pub value: MI_ConstUint32A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstUint32AField {}
impl ::core::clone::Clone for MI_ConstUint32AField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstUint32AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstUint32AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstUint32AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstUint32AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstUint32AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstUint32AField {}
impl ::core::default::Default for MI_ConstUint32AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstUint32Field {
    pub value: u32,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstUint32Field {}
impl ::core::clone::Clone for MI_ConstUint32Field {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstUint32Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstUint32Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstUint32Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstUint32Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstUint32Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstUint32Field {}
impl ::core::default::Default for MI_ConstUint32Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstUint64A {
    pub data: *const u64,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstUint64A {}
impl ::core::clone::Clone for MI_ConstUint64A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstUint64A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstUint64A").field("data", &self.data).field("size", &self.size).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstUint64A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstUint64A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstUint64A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstUint64A {}
impl ::core::default::Default for MI_ConstUint64A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstUint64AField {
    pub value: MI_ConstUint64A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstUint64AField {}
impl ::core::clone::Clone for MI_ConstUint64AField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstUint64AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstUint64AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstUint64AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstUint64AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstUint64AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstUint64AField {}
impl ::core::default::Default for MI_ConstUint64AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstUint64Field {
    pub value: u64,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstUint64Field {}
impl ::core::clone::Clone for MI_ConstUint64Field {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstUint64Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstUint64Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstUint64Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstUint64Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstUint64Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstUint64Field {}
impl ::core::default::Default for MI_ConstUint64Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstUint8A {
    pub data: *const u8,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstUint8A {}
impl ::core::clone::Clone for MI_ConstUint8A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstUint8A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstUint8A").field("data", &self.data).field("size", &self.size).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstUint8A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstUint8A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstUint8A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstUint8A {}
impl ::core::default::Default for MI_ConstUint8A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstUint8AField {
    pub value: MI_ConstUint8A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstUint8AField {}
impl ::core::clone::Clone for MI_ConstUint8AField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstUint8AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstUint8AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstUint8AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstUint8AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstUint8AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstUint8AField {}
impl ::core::default::Default for MI_ConstUint8AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ConstUint8Field {
    pub value: u8,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstUint8Field {}
impl ::core::clone::Clone for MI_ConstUint8Field {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstUint8Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstUint8Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ConstUint8Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstUint8Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstUint8Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstUint8Field {}
impl ::core::default::Default for MI_ConstUint8Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Context {
    pub ft: *const MI_ContextFT,
    pub reserved: [isize; 3],
}
impl ::core::marker::Copy for MI_Context {}
impl ::core::clone::Clone for MI_Context {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Context {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Context").field("ft", &self.ft).field("reserved", &self.reserved).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Context {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Context {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Context>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Context {}
impl ::core::default::Default for MI_Context {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ContextFT {
    pub PostResult: isize,
    pub PostInstance: isize,
    pub PostIndication: isize,
    pub ConstructInstance: isize,
    pub ConstructParameters: isize,
    pub NewInstance: isize,
    pub NewDynamicInstance: isize,
    pub NewParameters: isize,
    pub Canceled: isize,
    pub GetLocale: isize,
    pub RegisterCancel: isize,
    pub RequestUnload: isize,
    pub RefuseUnload: isize,
    pub GetLocalSession: isize,
    pub SetStringOption: isize,
    pub GetStringOption: isize,
    pub GetNumberOption: isize,
    pub GetCustomOption: isize,
    pub GetCustomOptionCount: isize,
    pub GetCustomOptionAt: isize,
    pub WriteMessage: isize,
    pub WriteProgress: isize,
    pub WriteStreamParameter: isize,
    pub WriteCimError: isize,
    pub PromptUser: isize,
    pub ShouldProcess: isize,
    pub ShouldContinue: isize,
    pub PostError: isize,
    pub PostCimError: isize,
    pub WriteError: isize,
}
impl ::core::marker::Copy for MI_ContextFT {}
impl ::core::clone::Clone for MI_ContextFT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ContextFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ContextFT")
            .field("PostResult", &self.PostResult)
            .field("PostInstance", &self.PostInstance)
            .field("PostIndication", &self.PostIndication)
            .field("ConstructInstance", &self.ConstructInstance)
            .field("ConstructParameters", &self.ConstructParameters)
            .field("NewInstance", &self.NewInstance)
            .field("NewDynamicInstance", &self.NewDynamicInstance)
            .field("NewParameters", &self.NewParameters)
            .field("Canceled", &self.Canceled)
            .field("GetLocale", &self.GetLocale)
            .field("RegisterCancel", &self.RegisterCancel)
            .field("RequestUnload", &self.RequestUnload)
            .field("RefuseUnload", &self.RefuseUnload)
            .field("GetLocalSession", &self.GetLocalSession)
            .field("SetStringOption", &self.SetStringOption)
            .field("GetStringOption", &self.GetStringOption)
            .field("GetNumberOption", &self.GetNumberOption)
            .field("GetCustomOption", &self.GetCustomOption)
            .field("GetCustomOptionCount", &self.GetCustomOptionCount)
            .field("GetCustomOptionAt", &self.GetCustomOptionAt)
            .field("WriteMessage", &self.WriteMessage)
            .field("WriteProgress", &self.WriteProgress)
            .field("WriteStreamParameter", &self.WriteStreamParameter)
            .field("WriteCimError", &self.WriteCimError)
            .field("PromptUser", &self.PromptUser)
            .field("ShouldProcess", &self.ShouldProcess)
            .field("ShouldContinue", &self.ShouldContinue)
            .field("PostError", &self.PostError)
            .field("PostCimError", &self.PostCimError)
            .field("WriteError", &self.WriteError)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ContextFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ContextFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ContextFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ContextFT {}
impl ::core::default::Default for MI_ContextFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Datetime {
    pub isTimestamp: u32,
    pub u: MI_Datetime_0,
}
impl ::core::marker::Copy for MI_Datetime {}
impl ::core::clone::Clone for MI_Datetime {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for MI_Datetime {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Datetime {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Datetime>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Datetime {}
impl ::core::default::Default for MI_Datetime {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union MI_Datetime_0 {
    pub timestamp: MI_Timestamp,
    pub interval: MI_Interval,
}
impl ::core::marker::Copy for MI_Datetime_0 {}
impl ::core::clone::Clone for MI_Datetime_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for MI_Datetime_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Datetime_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Datetime_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Datetime_0 {}
impl ::core::default::Default for MI_Datetime_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_DatetimeA {
    pub data: *mut MI_Datetime,
    pub size: u32,
}
impl ::core::marker::Copy for MI_DatetimeA {}
impl ::core::clone::Clone for MI_DatetimeA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_DatetimeA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_DatetimeA").field("data", &self.data).field("size", &self.size).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_DatetimeA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_DatetimeA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_DatetimeA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_DatetimeA {}
impl ::core::default::Default for MI_DatetimeA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_DatetimeAField {
    pub value: MI_DatetimeA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_DatetimeAField {}
impl ::core::clone::Clone for MI_DatetimeAField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_DatetimeAField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_DatetimeAField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_DatetimeAField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_DatetimeAField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_DatetimeAField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_DatetimeAField {}
impl ::core::default::Default for MI_DatetimeAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_DatetimeField {
    pub value: MI_Datetime,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_DatetimeField {}
impl ::core::clone::Clone for MI_DatetimeField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for MI_DatetimeField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_DatetimeField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_DatetimeField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_DatetimeField {}
impl ::core::default::Default for MI_DatetimeField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Deserializer {
    pub reserved1: u64,
    pub reserved2: isize,
}
impl ::core::marker::Copy for MI_Deserializer {}
impl ::core::clone::Clone for MI_Deserializer {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Deserializer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Deserializer").field("reserved1", &self.reserved1).field("reserved2", &self.reserved2).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Deserializer {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Deserializer {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Deserializer>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Deserializer {}
impl ::core::default::Default for MI_Deserializer {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_DeserializerFT {
    pub Close: isize,
    pub DeserializeClass: isize,
    pub Class_GetClassName: isize,
    pub Class_GetParentClassName: isize,
    pub DeserializeInstance: isize,
    pub Instance_GetClassName: isize,
}
impl ::core::marker::Copy for MI_DeserializerFT {}
impl ::core::clone::Clone for MI_DeserializerFT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_DeserializerFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_DeserializerFT").field("Close", &self.Close).field("DeserializeClass", &self.DeserializeClass).field("Class_GetClassName", &self.Class_GetClassName).field("Class_GetParentClassName", &self.Class_GetParentClassName).field("DeserializeInstance", &self.DeserializeInstance).field("Instance_GetClassName", &self.Instance_GetClassName).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_DeserializerFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_DeserializerFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_DeserializerFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_DeserializerFT {}
impl ::core::default::Default for MI_DeserializerFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type MI_Deserializer_ClassObjectNeeded = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, servername: *const u16, namespacename: *const u16, classname: *const u16, requestedclassobject: *mut *mut MI_Class) -> MI_Result>;
#[repr(C)]
pub struct MI_DestinationOptions {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_DestinationOptionsFT,
}
impl ::core::marker::Copy for MI_DestinationOptions {}
impl ::core::clone::Clone for MI_DestinationOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_DestinationOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_DestinationOptions").field("reserved1", &self.reserved1).field("reserved2", &self.reserved2).field("ft", &self.ft).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_DestinationOptions {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_DestinationOptions {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_DestinationOptions>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_DestinationOptions {}
impl ::core::default::Default for MI_DestinationOptions {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_DestinationOptionsFT {
    pub Delete: isize,
    pub SetString: isize,
    pub SetNumber: isize,
    pub AddCredentials: isize,
    pub GetString: isize,
    pub GetNumber: isize,
    pub GetOptionCount: isize,
    pub GetOptionAt: isize,
    pub GetOption: isize,
    pub GetCredentialsCount: isize,
    pub GetCredentialsAt: isize,
    pub GetCredentialsPasswordAt: isize,
    pub Clone: isize,
    pub SetInterval: isize,
    pub GetInterval: isize,
}
impl ::core::marker::Copy for MI_DestinationOptionsFT {}
impl ::core::clone::Clone for MI_DestinationOptionsFT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_DestinationOptionsFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_DestinationOptionsFT")
            .field("Delete", &self.Delete)
            .field("SetString", &self.SetString)
            .field("SetNumber", &self.SetNumber)
            .field("AddCredentials", &self.AddCredentials)
            .field("GetString", &self.GetString)
            .field("GetNumber", &self.GetNumber)
            .field("GetOptionCount", &self.GetOptionCount)
            .field("GetOptionAt", &self.GetOptionAt)
            .field("GetOption", &self.GetOption)
            .field("GetCredentialsCount", &self.GetCredentialsCount)
            .field("GetCredentialsAt", &self.GetCredentialsAt)
            .field("GetCredentialsPasswordAt", &self.GetCredentialsPasswordAt)
            .field("Clone", &self.Clone)
            .field("SetInterval", &self.SetInterval)
            .field("GetInterval", &self.GetInterval)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for MI_DestinationOptionsFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_DestinationOptionsFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_DestinationOptionsFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_DestinationOptionsFT {}
impl ::core::default::Default for MI_DestinationOptionsFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MI_DestinationOptions_ImpersonationType(pub i32);
pub const MI_DestinationOptions_ImpersonationType_Default: MI_DestinationOptions_ImpersonationType = MI_DestinationOptions_ImpersonationType(0i32);
pub const MI_DestinationOptions_ImpersonationType_None: MI_DestinationOptions_ImpersonationType = MI_DestinationOptions_ImpersonationType(1i32);
pub const MI_DestinationOptions_ImpersonationType_Identify: MI_DestinationOptions_ImpersonationType = MI_DestinationOptions_ImpersonationType(2i32);
pub const MI_DestinationOptions_ImpersonationType_Impersonate: MI_DestinationOptions_ImpersonationType = MI_DestinationOptions_ImpersonationType(3i32);
pub const MI_DestinationOptions_ImpersonationType_Delegate: MI_DestinationOptions_ImpersonationType = MI_DestinationOptions_ImpersonationType(4i32);
impl ::core::marker::Copy for MI_DestinationOptions_ImpersonationType {}
impl ::core::clone::Clone for MI_DestinationOptions_ImpersonationType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MI_DestinationOptions_ImpersonationType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MI_DestinationOptions_ImpersonationType {
    type Abi = Self;
}
impl ::core::fmt::Debug for MI_DestinationOptions_ImpersonationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MI_DestinationOptions_ImpersonationType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MI_ErrorCategory(pub i32);
pub const MI_ERRORCATEGORY_NOT_SPECIFIED: MI_ErrorCategory = MI_ErrorCategory(0i32);
pub const MI_ERRORCATEGORY_OPEN_ERROR: MI_ErrorCategory = MI_ErrorCategory(1i32);
pub const MI_ERRORCATEGORY_CLOS_EERROR: MI_ErrorCategory = MI_ErrorCategory(2i32);
pub const MI_ERRORCATEGORY_DEVICE_ERROR: MI_ErrorCategory = MI_ErrorCategory(3i32);
pub const MI_ERRORCATEGORY_DEADLOCK_DETECTED: MI_ErrorCategory = MI_ErrorCategory(4i32);
pub const MI_ERRORCATEGORY_INVALID_ARGUMENT: MI_ErrorCategory = MI_ErrorCategory(5i32);
pub const MI_ERRORCATEGORY_INVALID_DATA: MI_ErrorCategory = MI_ErrorCategory(6i32);
pub const MI_ERRORCATEGORY_INVALID_OPERATION: MI_ErrorCategory = MI_ErrorCategory(7i32);
pub const MI_ERRORCATEGORY_INVALID_RESULT: MI_ErrorCategory = MI_ErrorCategory(8i32);
pub const MI_ERRORCATEGORY_INVALID_TYPE: MI_ErrorCategory = MI_ErrorCategory(9i32);
pub const MI_ERRORCATEGORY_METADATA_ERROR: MI_ErrorCategory = MI_ErrorCategory(10i32);
pub const MI_ERRORCATEGORY_NOT_IMPLEMENTED: MI_ErrorCategory = MI_ErrorCategory(11i32);
pub const MI_ERRORCATEGORY_NOT_INSTALLED: MI_ErrorCategory = MI_ErrorCategory(12i32);
pub const MI_ERRORCATEGORY_OBJECT_NOT_FOUND: MI_ErrorCategory = MI_ErrorCategory(13i32);
pub const MI_ERRORCATEGORY_OPERATION_STOPPED: MI_ErrorCategory = MI_ErrorCategory(14i32);
pub const MI_ERRORCATEGORY_OPERATION_TIMEOUT: MI_ErrorCategory = MI_ErrorCategory(15i32);
pub const MI_ERRORCATEGORY_SYNTAX_ERROR: MI_ErrorCategory = MI_ErrorCategory(16i32);
pub const MI_ERRORCATEGORY_PARSER_ERROR: MI_ErrorCategory = MI_ErrorCategory(17i32);
pub const MI_ERRORCATEGORY_ACCESS_DENIED: MI_ErrorCategory = MI_ErrorCategory(18i32);
pub const MI_ERRORCATEGORY_RESOURCE_BUSY: MI_ErrorCategory = MI_ErrorCategory(19i32);
pub const MI_ERRORCATEGORY_RESOURCE_EXISTS: MI_ErrorCategory = MI_ErrorCategory(20i32);
pub const MI_ERRORCATEGORY_RESOURCE_UNAVAILABLE: MI_ErrorCategory = MI_ErrorCategory(21i32);
pub const MI_ERRORCATEGORY_READ_ERROR: MI_ErrorCategory = MI_ErrorCategory(22i32);
pub const MI_ERRORCATEGORY_WRITE_ERROR: MI_ErrorCategory = MI_ErrorCategory(23i32);
pub const MI_ERRORCATEGORY_FROM_STDERR: MI_ErrorCategory = MI_ErrorCategory(24i32);
pub const MI_ERRORCATEGORY_SECURITY_ERROR: MI_ErrorCategory = MI_ErrorCategory(25i32);
pub const MI_ERRORCATEGORY_PROTOCOL_ERROR: MI_ErrorCategory = MI_ErrorCategory(26i32);
pub const MI_ERRORCATEGORY_CONNECTION_ERROR: MI_ErrorCategory = MI_ErrorCategory(27i32);
pub const MI_ERRORCATEGORY_AUTHENTICATION_ERROR: MI_ErrorCategory = MI_ErrorCategory(28i32);
pub const MI_ERRORCATEGORY_LIMITS_EXCEEDED: MI_ErrorCategory = MI_ErrorCategory(29i32);
pub const MI_ERRORCATEGORY_QUOTA_EXCEEDED: MI_ErrorCategory = MI_ErrorCategory(30i32);
pub const MI_ERRORCATEGORY_NOT_ENABLED: MI_ErrorCategory = MI_ErrorCategory(31i32);
impl ::core::marker::Copy for MI_ErrorCategory {}
impl ::core::clone::Clone for MI_ErrorCategory {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MI_ErrorCategory {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MI_ErrorCategory {
    type Abi = Self;
}
impl ::core::fmt::Debug for MI_ErrorCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MI_ErrorCategory").field(&self.0).finish()
    }
}
pub const MI_FLAG_ABSTRACT: u32 = 131072u32;
pub const MI_FLAG_ADOPT: u32 = 2147483648u32;
pub const MI_FLAG_ANY: u32 = 127u32;
pub const MI_FLAG_ASSOCIATION: u32 = 16u32;
pub const MI_FLAG_BORROW: u32 = 1073741824u32;
pub const MI_FLAG_CLASS: u32 = 1u32;
pub const MI_FLAG_DISABLEOVERRIDE: u32 = 256u32;
pub const MI_FLAG_ENABLEOVERRIDE: u32 = 128u32;
pub const MI_FLAG_EXPENSIVE: u32 = 524288u32;
pub const MI_FLAG_EXTENDED: u32 = 4096u32;
pub const MI_FLAG_IN: u32 = 8192u32;
pub const MI_FLAG_INDICATION: u32 = 32u32;
pub const MI_FLAG_KEY: u32 = 4096u32;
pub const MI_FLAG_METHOD: u32 = 2u32;
pub const MI_FLAG_NOT_MODIFIED: u32 = 33554432u32;
pub const MI_FLAG_NULL: u32 = 536870912u32;
pub const MI_FLAG_OUT: u32 = 16384u32;
pub const MI_FLAG_PARAMETER: u32 = 8u32;
pub const MI_FLAG_PROPERTY: u32 = 4u32;
pub const MI_FLAG_READONLY: u32 = 2097152u32;
pub const MI_FLAG_REFERENCE: u32 = 64u32;
pub const MI_FLAG_REQUIRED: u32 = 32768u32;
pub const MI_FLAG_RESTRICTED: u32 = 512u32;
pub const MI_FLAG_STATIC: u32 = 65536u32;
pub const MI_FLAG_STREAM: u32 = 1048576u32;
pub const MI_FLAG_TERMINAL: u32 = 262144u32;
pub const MI_FLAG_TOSUBCLASS: u32 = 1024u32;
pub const MI_FLAG_TRANSLATABLE: u32 = 2048u32;
pub const MI_FLAG_VERSION: u32 = 469762048u32;
#[repr(C)]
pub struct MI_FeatureDecl {
    pub flags: u32,
    pub code: u32,
    pub name: *const u16,
    pub qualifiers: *const *const MI_Qualifier,
    pub numQualifiers: u32,
}
impl ::core::marker::Copy for MI_FeatureDecl {}
impl ::core::clone::Clone for MI_FeatureDecl {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_FeatureDecl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_FeatureDecl").field("flags", &self.flags).field("code", &self.code).field("name", &self.name).field("qualifiers", &self.qualifiers).field("numQualifiers", &self.numQualifiers).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_FeatureDecl {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_FeatureDecl {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_FeatureDecl>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_FeatureDecl {}
impl ::core::default::Default for MI_FeatureDecl {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Filter {
    pub ft: *const MI_FilterFT,
    pub reserved: [isize; 3],
}
impl ::core::marker::Copy for MI_Filter {}
impl ::core::clone::Clone for MI_Filter {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Filter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Filter").field("ft", &self.ft).field("reserved", &self.reserved).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Filter {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Filter {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Filter>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Filter {}
impl ::core::default::Default for MI_Filter {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_FilterFT {
    pub Evaluate: isize,
    pub GetExpression: isize,
}
impl ::core::marker::Copy for MI_FilterFT {}
impl ::core::clone::Clone for MI_FilterFT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_FilterFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_FilterFT").field("Evaluate", &self.Evaluate).field("GetExpression", &self.GetExpression).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_FilterFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_FilterFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_FilterFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_FilterFT {}
impl ::core::default::Default for MI_FilterFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_HostedProvider {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_HostedProviderFT,
}
impl ::core::marker::Copy for MI_HostedProvider {}
impl ::core::clone::Clone for MI_HostedProvider {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_HostedProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_HostedProvider").field("reserved1", &self.reserved1).field("reserved2", &self.reserved2).field("ft", &self.ft).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_HostedProvider {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_HostedProvider {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_HostedProvider>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_HostedProvider {}
impl ::core::default::Default for MI_HostedProvider {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_HostedProviderFT {
    pub Close: isize,
    pub GetApplication: isize,
}
impl ::core::marker::Copy for MI_HostedProviderFT {}
impl ::core::clone::Clone for MI_HostedProviderFT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_HostedProviderFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_HostedProviderFT").field("Close", &self.Close).field("GetApplication", &self.GetApplication).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_HostedProviderFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_HostedProviderFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_HostedProviderFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_HostedProviderFT {}
impl ::core::default::Default for MI_HostedProviderFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Instance {
    pub ft: *const MI_InstanceFT,
    pub classDecl: *const MI_ClassDecl,
    pub serverName: *const u16,
    pub nameSpace: *const u16,
    pub reserved: [isize; 4],
}
impl ::core::marker::Copy for MI_Instance {}
impl ::core::clone::Clone for MI_Instance {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Instance {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Instance").field("ft", &self.ft).field("classDecl", &self.classDecl).field("serverName", &self.serverName).field("nameSpace", &self.nameSpace).field("reserved", &self.reserved).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Instance {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Instance {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Instance>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Instance {}
impl ::core::default::Default for MI_Instance {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_InstanceA {
    pub data: *mut *mut MI_Instance,
    pub size: u32,
}
impl ::core::marker::Copy for MI_InstanceA {}
impl ::core::clone::Clone for MI_InstanceA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_InstanceA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_InstanceA").field("data", &self.data).field("size", &self.size).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_InstanceA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_InstanceA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_InstanceA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_InstanceA {}
impl ::core::default::Default for MI_InstanceA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_InstanceAField {
    pub value: MI_InstanceA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_InstanceAField {}
impl ::core::clone::Clone for MI_InstanceAField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_InstanceAField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_InstanceAField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_InstanceAField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_InstanceAField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_InstanceAField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_InstanceAField {}
impl ::core::default::Default for MI_InstanceAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_InstanceExFT {
    pub parent: MI_InstanceFT,
    pub Normalize: isize,
}
impl ::core::marker::Copy for MI_InstanceExFT {}
impl ::core::clone::Clone for MI_InstanceExFT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_InstanceExFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_InstanceExFT").field("parent", &self.parent).field("Normalize", &self.Normalize).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_InstanceExFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_InstanceExFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_InstanceExFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_InstanceExFT {}
impl ::core::default::Default for MI_InstanceExFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_InstanceFT {
    pub Clone: isize,
    pub Destruct: isize,
    pub Delete: isize,
    pub IsA: isize,
    pub GetClassNameA: isize,
    pub SetNameSpace: isize,
    pub GetNameSpace: isize,
    pub GetElementCount: isize,
    pub AddElement: isize,
    pub SetElement: isize,
    pub SetElementAt: isize,
    pub GetElement: isize,
    pub GetElementAt: isize,
    pub ClearElement: isize,
    pub ClearElementAt: isize,
    pub GetServerName: isize,
    pub SetServerName: isize,
    pub GetClass: isize,
}
impl ::core::marker::Copy for MI_InstanceFT {}
impl ::core::clone::Clone for MI_InstanceFT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_InstanceFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_InstanceFT")
            .field("Clone", &self.Clone)
            .field("Destruct", &self.Destruct)
            .field("Delete", &self.Delete)
            .field("IsA", &self.IsA)
            .field("GetClassNameA", &self.GetClassNameA)
            .field("SetNameSpace", &self.SetNameSpace)
            .field("GetNameSpace", &self.GetNameSpace)
            .field("GetElementCount", &self.GetElementCount)
            .field("AddElement", &self.AddElement)
            .field("SetElement", &self.SetElement)
            .field("SetElementAt", &self.SetElementAt)
            .field("GetElement", &self.GetElement)
            .field("GetElementAt", &self.GetElementAt)
            .field("ClearElement", &self.ClearElement)
            .field("ClearElementAt", &self.ClearElementAt)
            .field("GetServerName", &self.GetServerName)
            .field("SetServerName", &self.SetServerName)
            .field("GetClass", &self.GetClass)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for MI_InstanceFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_InstanceFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_InstanceFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_InstanceFT {}
impl ::core::default::Default for MI_InstanceFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_InstanceField {
    pub value: *mut MI_Instance,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_InstanceField {}
impl ::core::clone::Clone for MI_InstanceField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_InstanceField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_InstanceField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_InstanceField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_InstanceField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_InstanceField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_InstanceField {}
impl ::core::default::Default for MI_InstanceField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Interval {
    pub days: u32,
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    pub microseconds: u32,
    pub __padding1: u32,
    pub __padding2: u32,
    pub __padding3: u32,
}
impl ::core::marker::Copy for MI_Interval {}
impl ::core::clone::Clone for MI_Interval {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Interval {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Interval").field("days", &self.days).field("hours", &self.hours).field("minutes", &self.minutes).field("seconds", &self.seconds).field("microseconds", &self.microseconds).field("__padding1", &self.__padding1).field("__padding2", &self.__padding2).field("__padding3", &self.__padding3).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Interval {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Interval {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Interval>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Interval {}
impl ::core::default::Default for MI_Interval {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MI_LocaleType(pub i32);
pub const MI_LOCALE_TYPE_REQUESTED_UI: MI_LocaleType = MI_LocaleType(0i32);
pub const MI_LOCALE_TYPE_REQUESTED_DATA: MI_LocaleType = MI_LocaleType(1i32);
pub const MI_LOCALE_TYPE_CLOSEST_UI: MI_LocaleType = MI_LocaleType(2i32);
pub const MI_LOCALE_TYPE_CLOSEST_DATA: MI_LocaleType = MI_LocaleType(3i32);
impl ::core::marker::Copy for MI_LocaleType {}
impl ::core::clone::Clone for MI_LocaleType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MI_LocaleType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MI_LocaleType {
    type Abi = Self;
}
impl ::core::fmt::Debug for MI_LocaleType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MI_LocaleType").field(&self.0).finish()
    }
}
pub const MI_MAX_LOCALE_SIZE: u32 = 128u32;
pub const MI_MODULE_FLAG_BOOLEANS: u32 = 16u32;
pub const MI_MODULE_FLAG_CPLUSPLUS: u32 = 32u32;
pub const MI_MODULE_FLAG_DESCRIPTIONS: u32 = 2u32;
pub const MI_MODULE_FLAG_FILTER_SUPPORT: u32 = 128u32;
pub const MI_MODULE_FLAG_LOCALIZED: u32 = 64u32;
pub const MI_MODULE_FLAG_MAPPING_STRINGS: u32 = 8u32;
pub const MI_MODULE_FLAG_STANDARD_QUALIFIERS: u32 = 1u32;
pub const MI_MODULE_FLAG_VALUES: u32 = 4u32;
pub type MI_MainFunction = ::core::option::Option<unsafe extern "system" fn(server: *mut MI_Server) -> *mut MI_Module>;
#[repr(C)]
pub struct MI_MethodDecl {
    pub flags: u32,
    pub code: u32,
    pub name: *const u16,
    pub qualifiers: *const *const MI_Qualifier,
    pub numQualifiers: u32,
    pub parameters: *const *const MI_ParameterDecl,
    pub numParameters: u32,
    pub size: u32,
    pub returnType: u32,
    pub origin: *const u16,
    pub propagator: *const u16,
    pub schema: *const MI_SchemaDecl,
    pub function: MI_MethodDecl_Invoke,
}
impl ::core::marker::Copy for MI_MethodDecl {}
impl ::core::clone::Clone for MI_MethodDecl {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_MethodDecl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_MethodDecl")
            .field("flags", &self.flags)
            .field("code", &self.code)
            .field("name", &self.name)
            .field("qualifiers", &self.qualifiers)
            .field("numQualifiers", &self.numQualifiers)
            .field("parameters", &self.parameters)
            .field("numParameters", &self.numParameters)
            .field("size", &self.size)
            .field("returnType", &self.returnType)
            .field("origin", &self.origin)
            .field("propagator", &self.propagator)
            .field("schema", &self.schema)
            .field("function", &self.function.map(|f| f as usize))
            .finish()
    }
}
unsafe impl ::windows_core::Abi for MI_MethodDecl {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_MethodDecl {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_MethodDecl>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_MethodDecl {}
impl ::core::default::Default for MI_MethodDecl {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type MI_MethodDecl_Invoke = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, methodname: *const u16, instancename: *const MI_Instance, parameters: *const MI_Instance)>;
#[repr(C)]
pub struct MI_Module {
    pub version: u32,
    pub generatorVersion: u32,
    pub flags: u32,
    pub charSize: u32,
    pub schemaDecl: *mut MI_SchemaDecl,
    pub Load: MI_Module_Load,
    pub Unload: MI_Module_Unload,
    pub dynamicProviderFT: *const MI_ProviderFT,
}
impl ::core::marker::Copy for MI_Module {}
impl ::core::clone::Clone for MI_Module {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Module {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Module").field("version", &self.version).field("generatorVersion", &self.generatorVersion).field("flags", &self.flags).field("charSize", &self.charSize).field("schemaDecl", &self.schemaDecl).field("Load", &self.Load.map(|f| f as usize)).field("Unload", &self.Unload.map(|f| f as usize)).field("dynamicProviderFT", &self.dynamicProviderFT).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Module {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Module {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Module>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Module {}
impl ::core::default::Default for MI_Module {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type MI_Module_Load = ::core::option::Option<unsafe extern "system" fn(self_: *mut *mut MI_Module_Self, context: *const MI_Context)>;
#[repr(C)]
pub struct MI_Module_Self(pub u8);
pub type MI_Module_Unload = ::core::option::Option<unsafe extern "system" fn(self_: *const MI_Module_Self, context: *const MI_Context)>;
pub const MI_OPERATIONFLAGS_BASIC_RTTI: u32 = 2u32;
pub const MI_OPERATIONFLAGS_DEFAULT_RTTI: u32 = 0u32;
pub const MI_OPERATIONFLAGS_EXPENSIVE_PROPERTIES: u32 = 64u32;
pub const MI_OPERATIONFLAGS_FULL_RTTI: u32 = 4u32;
pub const MI_OPERATIONFLAGS_LOCALIZED_QUALIFIERS: u32 = 8u32;
pub const MI_OPERATIONFLAGS_MANUAL_ACK_RESULTS: u32 = 1u32;
pub const MI_OPERATIONFLAGS_NO_RTTI: u32 = 1024u32;
pub const MI_OPERATIONFLAGS_POLYMORPHISM_DEEP_BASE_PROPS_ONLY: u32 = 384u32;
pub const MI_OPERATIONFLAGS_POLYMORPHISM_SHALLOW: u32 = 128u32;
pub const MI_OPERATIONFLAGS_REPORT_OPERATION_STARTED: u32 = 512u32;
pub const MI_OPERATIONFLAGS_STANDARD_RTTI: u32 = 2048u32;
#[repr(C)]
pub struct MI_ObjectDecl {
    pub flags: u32,
    pub code: u32,
    pub name: *const u16,
    pub qualifiers: *const *const MI_Qualifier,
    pub numQualifiers: u32,
    pub properties: *const *const MI_PropertyDecl,
    pub numProperties: u32,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ObjectDecl {}
impl ::core::clone::Clone for MI_ObjectDecl {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ObjectDecl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ObjectDecl").field("flags", &self.flags).field("code", &self.code).field("name", &self.name).field("qualifiers", &self.qualifiers).field("numQualifiers", &self.numQualifiers).field("properties", &self.properties).field("numProperties", &self.numProperties).field("size", &self.size).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ObjectDecl {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ObjectDecl {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ObjectDecl>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ObjectDecl {}
impl ::core::default::Default for MI_ObjectDecl {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Operation {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_OperationFT,
}
impl ::core::marker::Copy for MI_Operation {}
impl ::core::clone::Clone for MI_Operation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Operation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Operation").field("reserved1", &self.reserved1).field("reserved2", &self.reserved2).field("ft", &self.ft).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Operation {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Operation {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Operation>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Operation {}
impl ::core::default::Default for MI_Operation {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type MI_OperationCallback_Class = ::core::option::Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, classresult: *const MI_Class, moreresults: u8, resultcode: MI_Result, errorstring: *const u16, errordetails: *const MI_Instance, resultacknowledgement: isize)>;
pub type MI_OperationCallback_Indication = ::core::option::Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, instance: *const MI_Instance, bookmark: *const u16, machineid: *const u16, moreresults: u8, resultcode: MI_Result, errorstring: *const u16, errordetails: *const MI_Instance, resultacknowledgement: isize)>;
pub type MI_OperationCallback_Instance = ::core::option::Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, instance: *const MI_Instance, moreresults: u8, resultcode: MI_Result, errorstring: *const u16, errordetails: *const MI_Instance, resultacknowledgement: isize)>;
pub type MI_OperationCallback_PromptUser = ::core::option::Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, message: *const u16, prompttype: MI_PromptType, promptuserresult: isize)>;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MI_OperationCallback_ResponseType(pub i32);
pub const MI_OperationCallback_ResponseType_No: MI_OperationCallback_ResponseType = MI_OperationCallback_ResponseType(0i32);
pub const MI_OperationCallback_ResponseType_Yes: MI_OperationCallback_ResponseType = MI_OperationCallback_ResponseType(1i32);
pub const MI_OperationCallback_ResponseType_NoToAll: MI_OperationCallback_ResponseType = MI_OperationCallback_ResponseType(2i32);
pub const MI_OperationCallback_ResponseType_YesToAll: MI_OperationCallback_ResponseType = MI_OperationCallback_ResponseType(3i32);
impl ::core::marker::Copy for MI_OperationCallback_ResponseType {}
impl ::core::clone::Clone for MI_OperationCallback_ResponseType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MI_OperationCallback_ResponseType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MI_OperationCallback_ResponseType {
    type Abi = Self;
}
impl ::core::fmt::Debug for MI_OperationCallback_ResponseType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MI_OperationCallback_ResponseType").field(&self.0).finish()
    }
}
pub type MI_OperationCallback_StreamedParameter = ::core::option::Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, parametername: *const u16, resulttype: MI_Type, result: *const MI_Value, resultacknowledgement: isize)>;
pub type MI_OperationCallback_WriteError = ::core::option::Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, instance: *const MI_Instance, writeerrorresult: isize)>;
pub type MI_OperationCallback_WriteMessage = ::core::option::Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, channel: u32, message: *const u16)>;
pub type MI_OperationCallback_WriteProgress = ::core::option::Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, activity: *const u16, currentoperation: *const u16, statusdescription: *const u16, percentagecomplete: u32, secondsremaining: u32)>;
#[repr(C)]
pub struct MI_OperationCallbacks {
    pub callbackContext: *mut ::core::ffi::c_void,
    pub promptUser: MI_OperationCallback_PromptUser,
    pub writeError: MI_OperationCallback_WriteError,
    pub writeMessage: MI_OperationCallback_WriteMessage,
    pub writeProgress: MI_OperationCallback_WriteProgress,
    pub instanceResult: MI_OperationCallback_Instance,
    pub indicationResult: MI_OperationCallback_Indication,
    pub classResult: MI_OperationCallback_Class,
    pub streamedParameterResult: MI_OperationCallback_StreamedParameter,
}
impl ::core::marker::Copy for MI_OperationCallbacks {}
impl ::core::clone::Clone for MI_OperationCallbacks {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_OperationCallbacks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_OperationCallbacks")
            .field("callbackContext", &self.callbackContext)
            .field("promptUser", &self.promptUser.map(|f| f as usize))
            .field("writeError", &self.writeError.map(|f| f as usize))
            .field("writeMessage", &self.writeMessage.map(|f| f as usize))
            .field("writeProgress", &self.writeProgress.map(|f| f as usize))
            .field("instanceResult", &self.instanceResult.map(|f| f as usize))
            .field("indicationResult", &self.indicationResult.map(|f| f as usize))
            .field("classResult", &self.classResult.map(|f| f as usize))
            .field("streamedParameterResult", &self.streamedParameterResult.map(|f| f as usize))
            .finish()
    }
}
unsafe impl ::windows_core::Abi for MI_OperationCallbacks {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_OperationCallbacks {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_OperationCallbacks>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_OperationCallbacks {}
impl ::core::default::Default for MI_OperationCallbacks {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_OperationFT {
    pub Close: isize,
    pub Cancel: isize,
    pub GetSession: isize,
    pub GetInstance: isize,
    pub GetIndication: isize,
    pub GetClass: isize,
}
impl ::core::marker::Copy for MI_OperationFT {}
impl ::core::clone::Clone for MI_OperationFT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_OperationFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_OperationFT").field("Close", &self.Close).field("Cancel", &self.Cancel).field("GetSession", &self.GetSession).field("GetInstance", &self.GetInstance).field("GetIndication", &self.GetIndication).field("GetClass", &self.GetClass).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_OperationFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_OperationFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_OperationFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_OperationFT {}
impl ::core::default::Default for MI_OperationFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_OperationOptions {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_OperationOptionsFT,
}
impl ::core::marker::Copy for MI_OperationOptions {}
impl ::core::clone::Clone for MI_OperationOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_OperationOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_OperationOptions").field("reserved1", &self.reserved1).field("reserved2", &self.reserved2).field("ft", &self.ft).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_OperationOptions {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_OperationOptions {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_OperationOptions>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_OperationOptions {}
impl ::core::default::Default for MI_OperationOptions {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_OperationOptionsFT {
    pub Delete: isize,
    pub SetString: isize,
    pub SetNumber: isize,
    pub SetCustomOption: isize,
    pub GetString: isize,
    pub GetNumber: isize,
    pub GetOptionCount: isize,
    pub GetOptionAt: isize,
    pub GetOption: isize,
    pub GetEnabledChannels: isize,
    pub Clone: isize,
    pub SetInterval: isize,
    pub GetInterval: isize,
}
impl ::core::marker::Copy for MI_OperationOptionsFT {}
impl ::core::clone::Clone for MI_OperationOptionsFT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_OperationOptionsFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_OperationOptionsFT")
            .field("Delete", &self.Delete)
            .field("SetString", &self.SetString)
            .field("SetNumber", &self.SetNumber)
            .field("SetCustomOption", &self.SetCustomOption)
            .field("GetString", &self.GetString)
            .field("GetNumber", &self.GetNumber)
            .field("GetOptionCount", &self.GetOptionCount)
            .field("GetOptionAt", &self.GetOptionAt)
            .field("GetOption", &self.GetOption)
            .field("GetEnabledChannels", &self.GetEnabledChannels)
            .field("Clone", &self.Clone)
            .field("SetInterval", &self.SetInterval)
            .field("GetInterval", &self.GetInterval)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for MI_OperationOptionsFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_OperationOptionsFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_OperationOptionsFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_OperationOptionsFT {}
impl ::core::default::Default for MI_OperationOptionsFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ParameterDecl {
    pub flags: u32,
    pub code: u32,
    pub name: *const u16,
    pub qualifiers: *const *const MI_Qualifier,
    pub numQualifiers: u32,
    pub r#type: u32,
    pub className: *const u16,
    pub subscript: u32,
    pub offset: u32,
}
impl ::core::marker::Copy for MI_ParameterDecl {}
impl ::core::clone::Clone for MI_ParameterDecl {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ParameterDecl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ParameterDecl").field("flags", &self.flags).field("code", &self.code).field("name", &self.name).field("qualifiers", &self.qualifiers).field("numQualifiers", &self.numQualifiers).field("type", &self.r#type).field("className", &self.className).field("subscript", &self.subscript).field("offset", &self.offset).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ParameterDecl {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ParameterDecl {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ParameterDecl>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ParameterDecl {}
impl ::core::default::Default for MI_ParameterDecl {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ParameterSet {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_ParameterSetFT,
}
impl ::core::marker::Copy for MI_ParameterSet {}
impl ::core::clone::Clone for MI_ParameterSet {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ParameterSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ParameterSet").field("reserved1", &self.reserved1).field("reserved2", &self.reserved2).field("ft", &self.ft).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ParameterSet {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ParameterSet {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ParameterSet>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ParameterSet {}
impl ::core::default::Default for MI_ParameterSet {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ParameterSetFT {
    pub GetMethodReturnType: isize,
    pub GetParameterCount: isize,
    pub GetParameterAt: isize,
    pub GetParameter: isize,
}
impl ::core::marker::Copy for MI_ParameterSetFT {}
impl ::core::clone::Clone for MI_ParameterSetFT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ParameterSetFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ParameterSetFT").field("GetMethodReturnType", &self.GetMethodReturnType).field("GetParameterCount", &self.GetParameterCount).field("GetParameterAt", &self.GetParameterAt).field("GetParameter", &self.GetParameter).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ParameterSetFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ParameterSetFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ParameterSetFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ParameterSetFT {}
impl ::core::default::Default for MI_ParameterSetFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MI_PromptType(pub i32);
pub const MI_PROMPTTYPE_NORMAL: MI_PromptType = MI_PromptType(0i32);
pub const MI_PROMPTTYPE_CRITICAL: MI_PromptType = MI_PromptType(1i32);
impl ::core::marker::Copy for MI_PromptType {}
impl ::core::clone::Clone for MI_PromptType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MI_PromptType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MI_PromptType {
    type Abi = Self;
}
impl ::core::fmt::Debug for MI_PromptType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MI_PromptType").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct MI_PropertyDecl {
    pub flags: u32,
    pub code: u32,
    pub name: *const u16,
    pub qualifiers: *const *const MI_Qualifier,
    pub numQualifiers: u32,
    pub r#type: u32,
    pub className: *const u16,
    pub subscript: u32,
    pub offset: u32,
    pub origin: *const u16,
    pub propagator: *const u16,
    pub value: *const ::core::ffi::c_void,
}
impl ::core::marker::Copy for MI_PropertyDecl {}
impl ::core::clone::Clone for MI_PropertyDecl {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_PropertyDecl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_PropertyDecl").field("flags", &self.flags).field("code", &self.code).field("name", &self.name).field("qualifiers", &self.qualifiers).field("numQualifiers", &self.numQualifiers).field("type", &self.r#type).field("className", &self.className).field("subscript", &self.subscript).field("offset", &self.offset).field("origin", &self.origin).field("propagator", &self.propagator).field("value", &self.value).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_PropertyDecl {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_PropertyDecl {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_PropertyDecl>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_PropertyDecl {}
impl ::core::default::Default for MI_PropertyDecl {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_PropertySet {
    pub ft: *const MI_PropertySetFT,
    pub reserved: [isize; 3],
}
impl ::core::marker::Copy for MI_PropertySet {}
impl ::core::clone::Clone for MI_PropertySet {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_PropertySet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_PropertySet").field("ft", &self.ft).field("reserved", &self.reserved).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_PropertySet {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_PropertySet {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_PropertySet>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_PropertySet {}
impl ::core::default::Default for MI_PropertySet {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_PropertySetFT {
    pub GetElementCount: isize,
    pub ContainsElement: isize,
    pub AddElement: isize,
    pub GetElementAt: isize,
    pub Clear: isize,
    pub Destruct: isize,
    pub Delete: isize,
    pub Clone: isize,
}
impl ::core::marker::Copy for MI_PropertySetFT {}
impl ::core::clone::Clone for MI_PropertySetFT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_PropertySetFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_PropertySetFT").field("GetElementCount", &self.GetElementCount).field("ContainsElement", &self.ContainsElement).field("AddElement", &self.AddElement).field("GetElementAt", &self.GetElementAt).field("Clear", &self.Clear).field("Destruct", &self.Destruct).field("Delete", &self.Delete).field("Clone", &self.Clone).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_PropertySetFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_PropertySetFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_PropertySetFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_PropertySetFT {}
impl ::core::default::Default for MI_PropertySetFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MI_ProviderArchitecture(pub i32);
pub const MI_PROVIDER_ARCHITECTURE_32BIT: MI_ProviderArchitecture = MI_ProviderArchitecture(0i32);
pub const MI_PROVIDER_ARCHITECTURE_64BIT: MI_ProviderArchitecture = MI_ProviderArchitecture(1i32);
impl ::core::marker::Copy for MI_ProviderArchitecture {}
impl ::core::clone::Clone for MI_ProviderArchitecture {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MI_ProviderArchitecture {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MI_ProviderArchitecture {
    type Abi = Self;
}
impl ::core::fmt::Debug for MI_ProviderArchitecture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MI_ProviderArchitecture").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct MI_ProviderFT {
    pub Load: MI_ProviderFT_Load,
    pub Unload: MI_ProviderFT_Unload,
    pub GetInstance: MI_ProviderFT_GetInstance,
    pub EnumerateInstances: MI_ProviderFT_EnumerateInstances,
    pub CreateInstance: MI_ProviderFT_CreateInstance,
    pub ModifyInstance: MI_ProviderFT_ModifyInstance,
    pub DeleteInstance: MI_ProviderFT_DeleteInstance,
    pub AssociatorInstances: MI_ProviderFT_AssociatorInstances,
    pub ReferenceInstances: MI_ProviderFT_ReferenceInstances,
    pub EnableIndications: MI_ProviderFT_EnableIndications,
    pub DisableIndications: MI_ProviderFT_DisableIndications,
    pub Subscribe: MI_ProviderFT_Subscribe,
    pub Unsubscribe: MI_ProviderFT_Unsubscribe,
    pub Invoke: MI_ProviderFT_Invoke,
}
impl ::core::marker::Copy for MI_ProviderFT {}
impl ::core::clone::Clone for MI_ProviderFT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ProviderFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ProviderFT")
            .field("Load", &self.Load.map(|f| f as usize))
            .field("Unload", &self.Unload.map(|f| f as usize))
            .field("GetInstance", &self.GetInstance.map(|f| f as usize))
            .field("EnumerateInstances", &self.EnumerateInstances.map(|f| f as usize))
            .field("CreateInstance", &self.CreateInstance.map(|f| f as usize))
            .field("ModifyInstance", &self.ModifyInstance.map(|f| f as usize))
            .field("DeleteInstance", &self.DeleteInstance.map(|f| f as usize))
            .field("AssociatorInstances", &self.AssociatorInstances.map(|f| f as usize))
            .field("ReferenceInstances", &self.ReferenceInstances.map(|f| f as usize))
            .field("EnableIndications", &self.EnableIndications.map(|f| f as usize))
            .field("DisableIndications", &self.DisableIndications.map(|f| f as usize))
            .field("Subscribe", &self.Subscribe.map(|f| f as usize))
            .field("Unsubscribe", &self.Unsubscribe.map(|f| f as usize))
            .field("Invoke", &self.Invoke.map(|f| f as usize))
            .finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ProviderFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ProviderFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ProviderFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ProviderFT {}
impl ::core::default::Default for MI_ProviderFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type MI_ProviderFT_AssociatorInstances = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, instancename: *const MI_Instance, resultclass: *const u16, role: *const u16, resultrole: *const u16, propertyset: *const MI_PropertySet, keysonly: u8, filter: *const MI_Filter)>;
pub type MI_ProviderFT_CreateInstance = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, newinstance: *const MI_Instance)>;
pub type MI_ProviderFT_DeleteInstance = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, instancename: *const MI_Instance)>;
pub type MI_ProviderFT_DisableIndications = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, indicationscontext: *const MI_Context, namespace: *const u16, classname: *const u16)>;
pub type MI_ProviderFT_EnableIndications = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, indicationscontext: *const MI_Context, namespace: *const u16, classname: *const u16)>;
pub type MI_ProviderFT_EnumerateInstances = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, propertyset: *const MI_PropertySet, keysonly: u8, filter: *const MI_Filter)>;
pub type MI_ProviderFT_GetInstance = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, instancename: *const MI_Instance, propertyset: *const MI_PropertySet)>;
pub type MI_ProviderFT_Invoke = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, methodname: *const u16, instancename: *const MI_Instance, inputparameters: *const MI_Instance)>;
pub type MI_ProviderFT_Load = ::core::option::Option<unsafe extern "system" fn(self_: *mut *mut ::core::ffi::c_void, selfmodule: *const MI_Module_Self, context: *const MI_Context)>;
pub type MI_ProviderFT_ModifyInstance = ::core::option::Option<unsafe extern "system" fn(self_: *mut ::core::ffi::c_void, context: *mut MI_Context, namespace: *const u16, classname: *const u16, modifiedinstance: *const MI_Instance, propertyset: *const MI_PropertySet)>;
pub type MI_ProviderFT_ReferenceInstances = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, instancename: *const MI_Instance, role: *const u16, propertyset: *const MI_PropertySet, keysonly: u8, filter: *const MI_Filter)>;
pub type MI_ProviderFT_Subscribe = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, filter: *const MI_Filter, bookmark: *const u16, subscriptionid: u64, subscriptionself: *mut *mut ::core::ffi::c_void)>;
pub type MI_ProviderFT_Unload = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context)>;
pub type MI_ProviderFT_Unsubscribe = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, subscriptionid: u64, subscriptionself: *const ::core::ffi::c_void)>;
#[repr(C)]
pub struct MI_Qualifier {
    pub name: *const u16,
    pub r#type: u32,
    pub flavor: u32,
    pub value: *const ::core::ffi::c_void,
}
impl ::core::marker::Copy for MI_Qualifier {}
impl ::core::clone::Clone for MI_Qualifier {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Qualifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Qualifier").field("name", &self.name).field("type", &self.r#type).field("flavor", &self.flavor).field("value", &self.value).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Qualifier {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Qualifier {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Qualifier>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Qualifier {}
impl ::core::default::Default for MI_Qualifier {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_QualifierDecl {
    pub name: *const u16,
    pub r#type: u32,
    pub scope: u32,
    pub flavor: u32,
    pub subscript: u32,
    pub value: *const ::core::ffi::c_void,
}
impl ::core::marker::Copy for MI_QualifierDecl {}
impl ::core::clone::Clone for MI_QualifierDecl {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_QualifierDecl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_QualifierDecl").field("name", &self.name).field("type", &self.r#type).field("scope", &self.scope).field("flavor", &self.flavor).field("subscript", &self.subscript).field("value", &self.value).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_QualifierDecl {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_QualifierDecl {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_QualifierDecl>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_QualifierDecl {}
impl ::core::default::Default for MI_QualifierDecl {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_QualifierSet {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_QualifierSetFT,
}
impl ::core::marker::Copy for MI_QualifierSet {}
impl ::core::clone::Clone for MI_QualifierSet {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_QualifierSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_QualifierSet").field("reserved1", &self.reserved1).field("reserved2", &self.reserved2).field("ft", &self.ft).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_QualifierSet {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_QualifierSet {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_QualifierSet>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_QualifierSet {}
impl ::core::default::Default for MI_QualifierSet {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_QualifierSetFT {
    pub GetQualifierCount: isize,
    pub GetQualifierAt: isize,
    pub GetQualifier: isize,
}
impl ::core::marker::Copy for MI_QualifierSetFT {}
impl ::core::clone::Clone for MI_QualifierSetFT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_QualifierSetFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_QualifierSetFT").field("GetQualifierCount", &self.GetQualifierCount).field("GetQualifierAt", &self.GetQualifierAt).field("GetQualifier", &self.GetQualifier).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_QualifierSetFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_QualifierSetFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_QualifierSetFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_QualifierSetFT {}
impl ::core::default::Default for MI_QualifierSetFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Real32A {
    pub data: *mut f32,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Real32A {}
impl ::core::clone::Clone for MI_Real32A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Real32A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Real32A").field("data", &self.data).field("size", &self.size).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Real32A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Real32A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Real32A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Real32A {}
impl ::core::default::Default for MI_Real32A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Real32AField {
    pub value: MI_Real32A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Real32AField {}
impl ::core::clone::Clone for MI_Real32AField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Real32AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Real32AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Real32AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Real32AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Real32AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Real32AField {}
impl ::core::default::Default for MI_Real32AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Real32Field {
    pub value: f32,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Real32Field {}
impl ::core::clone::Clone for MI_Real32Field {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Real32Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Real32Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Real32Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Real32Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Real32Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Real32Field {}
impl ::core::default::Default for MI_Real32Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Real64A {
    pub data: *mut f64,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Real64A {}
impl ::core::clone::Clone for MI_Real64A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Real64A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Real64A").field("data", &self.data).field("size", &self.size).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Real64A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Real64A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Real64A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Real64A {}
impl ::core::default::Default for MI_Real64A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Real64AField {
    pub value: MI_Real64A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Real64AField {}
impl ::core::clone::Clone for MI_Real64AField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Real64AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Real64AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Real64AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Real64AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Real64AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Real64AField {}
impl ::core::default::Default for MI_Real64AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Real64Field {
    pub value: f64,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Real64Field {}
impl ::core::clone::Clone for MI_Real64Field {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Real64Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Real64Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Real64Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Real64Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Real64Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Real64Field {}
impl ::core::default::Default for MI_Real64Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ReferenceA {
    pub data: *mut *mut MI_Instance,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ReferenceA {}
impl ::core::clone::Clone for MI_ReferenceA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ReferenceA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ReferenceA").field("data", &self.data).field("size", &self.size).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ReferenceA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ReferenceA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ReferenceA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ReferenceA {}
impl ::core::default::Default for MI_ReferenceA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ReferenceAField {
    pub value: MI_ReferenceA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ReferenceAField {}
impl ::core::clone::Clone for MI_ReferenceAField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ReferenceAField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ReferenceAField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ReferenceAField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ReferenceAField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ReferenceAField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ReferenceAField {}
impl ::core::default::Default for MI_ReferenceAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ReferenceField {
    pub value: *mut MI_Instance,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ReferenceField {}
impl ::core::clone::Clone for MI_ReferenceField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ReferenceField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ReferenceField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ReferenceField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ReferenceField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ReferenceField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ReferenceField {}
impl ::core::default::Default for MI_ReferenceField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MI_Result(pub i32);
pub const MI_RESULT_OK: MI_Result = MI_Result(0i32);
pub const MI_RESULT_FAILED: MI_Result = MI_Result(1i32);
pub const MI_RESULT_ACCESS_DENIED: MI_Result = MI_Result(2i32);
pub const MI_RESULT_INVALID_NAMESPACE: MI_Result = MI_Result(3i32);
pub const MI_RESULT_INVALID_PARAMETER: MI_Result = MI_Result(4i32);
pub const MI_RESULT_INVALID_CLASS: MI_Result = MI_Result(5i32);
pub const MI_RESULT_NOT_FOUND: MI_Result = MI_Result(6i32);
pub const MI_RESULT_NOT_SUPPORTED: MI_Result = MI_Result(7i32);
pub const MI_RESULT_CLASS_HAS_CHILDREN: MI_Result = MI_Result(8i32);
pub const MI_RESULT_CLASS_HAS_INSTANCES: MI_Result = MI_Result(9i32);
pub const MI_RESULT_INVALID_SUPERCLASS: MI_Result = MI_Result(10i32);
pub const MI_RESULT_ALREADY_EXISTS: MI_Result = MI_Result(11i32);
pub const MI_RESULT_NO_SUCH_PROPERTY: MI_Result = MI_Result(12i32);
pub const MI_RESULT_TYPE_MISMATCH: MI_Result = MI_Result(13i32);
pub const MI_RESULT_QUERY_LANGUAGE_NOT_SUPPORTED: MI_Result = MI_Result(14i32);
pub const MI_RESULT_INVALID_QUERY: MI_Result = MI_Result(15i32);
pub const MI_RESULT_METHOD_NOT_AVAILABLE: MI_Result = MI_Result(16i32);
pub const MI_RESULT_METHOD_NOT_FOUND: MI_Result = MI_Result(17i32);
pub const MI_RESULT_NAMESPACE_NOT_EMPTY: MI_Result = MI_Result(20i32);
pub const MI_RESULT_INVALID_ENUMERATION_CONTEXT: MI_Result = MI_Result(21i32);
pub const MI_RESULT_INVALID_OPERATION_TIMEOUT: MI_Result = MI_Result(22i32);
pub const MI_RESULT_PULL_HAS_BEEN_ABANDONED: MI_Result = MI_Result(23i32);
pub const MI_RESULT_PULL_CANNOT_BE_ABANDONED: MI_Result = MI_Result(24i32);
pub const MI_RESULT_FILTERED_ENUMERATION_NOT_SUPPORTED: MI_Result = MI_Result(25i32);
pub const MI_RESULT_CONTINUATION_ON_ERROR_NOT_SUPPORTED: MI_Result = MI_Result(26i32);
pub const MI_RESULT_SERVER_LIMITS_EXCEEDED: MI_Result = MI_Result(27i32);
pub const MI_RESULT_SERVER_IS_SHUTTING_DOWN: MI_Result = MI_Result(28i32);
impl ::core::marker::Copy for MI_Result {}
impl ::core::clone::Clone for MI_Result {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MI_Result {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MI_Result {
    type Abi = Self;
}
impl ::core::fmt::Debug for MI_Result {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MI_Result").field(&self.0).finish()
    }
}
pub const MI_SERIALIZER_FLAGS_CLASS_DEEP: u32 = 1u32;
pub const MI_SERIALIZER_FLAGS_INSTANCE_WITH_CLASS: u32 = 1u32;
pub const MI_SUBSCRIBE_BOOKMARK_NEWEST: &str = "MI_SUBSCRIBE_BOOKMARK_NEWEST";
pub const MI_SUBSCRIBE_BOOKMARK_OLDEST: &str = "MI_SUBSCRIBE_BOOKMARK_OLDEST";
#[repr(C)]
pub struct MI_SchemaDecl {
    pub qualifierDecls: *const *const MI_QualifierDecl,
    pub numQualifierDecls: u32,
    pub classDecls: *const *const MI_ClassDecl,
    pub numClassDecls: u32,
}
impl ::core::marker::Copy for MI_SchemaDecl {}
impl ::core::clone::Clone for MI_SchemaDecl {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_SchemaDecl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_SchemaDecl").field("qualifierDecls", &self.qualifierDecls).field("numQualifierDecls", &self.numQualifierDecls).field("classDecls", &self.classDecls).field("numClassDecls", &self.numClassDecls).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_SchemaDecl {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_SchemaDecl {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_SchemaDecl>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_SchemaDecl {}
impl ::core::default::Default for MI_SchemaDecl {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Serializer {
    pub reserved1: u64,
    pub reserved2: isize,
}
impl ::core::marker::Copy for MI_Serializer {}
impl ::core::clone::Clone for MI_Serializer {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Serializer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Serializer").field("reserved1", &self.reserved1).field("reserved2", &self.reserved2).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Serializer {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Serializer {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Serializer>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Serializer {}
impl ::core::default::Default for MI_Serializer {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_SerializerFT {
    pub Close: isize,
    pub SerializeClass: isize,
    pub SerializeInstance: isize,
}
impl ::core::marker::Copy for MI_SerializerFT {}
impl ::core::clone::Clone for MI_SerializerFT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_SerializerFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_SerializerFT").field("Close", &self.Close).field("SerializeClass", &self.SerializeClass).field("SerializeInstance", &self.SerializeInstance).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_SerializerFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_SerializerFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_SerializerFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_SerializerFT {}
impl ::core::default::Default for MI_SerializerFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Server {
    pub serverFT: *const MI_ServerFT,
    pub contextFT: *const MI_ContextFT,
    pub instanceFT: *const MI_InstanceFT,
    pub propertySetFT: *const MI_PropertySetFT,
    pub filterFT: *const MI_FilterFT,
}
impl ::core::marker::Copy for MI_Server {}
impl ::core::clone::Clone for MI_Server {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Server {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Server").field("serverFT", &self.serverFT).field("contextFT", &self.contextFT).field("instanceFT", &self.instanceFT).field("propertySetFT", &self.propertySetFT).field("filterFT", &self.filterFT).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Server {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Server {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Server>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Server {}
impl ::core::default::Default for MI_Server {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_ServerFT {
    pub GetVersion: isize,
    pub GetSystemName: isize,
}
impl ::core::marker::Copy for MI_ServerFT {}
impl ::core::clone::Clone for MI_ServerFT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ServerFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ServerFT").field("GetVersion", &self.GetVersion).field("GetSystemName", &self.GetSystemName).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_ServerFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ServerFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ServerFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ServerFT {}
impl ::core::default::Default for MI_ServerFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Session {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_SessionFT,
}
impl ::core::marker::Copy for MI_Session {}
impl ::core::clone::Clone for MI_Session {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Session {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Session").field("reserved1", &self.reserved1).field("reserved2", &self.reserved2).field("ft", &self.ft).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Session {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Session {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Session>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Session {}
impl ::core::default::Default for MI_Session {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_SessionCallbacks {
    pub callbackContext: *mut ::core::ffi::c_void,
    pub writeMessage: isize,
    pub writeError: isize,
}
impl ::core::marker::Copy for MI_SessionCallbacks {}
impl ::core::clone::Clone for MI_SessionCallbacks {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_SessionCallbacks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_SessionCallbacks").field("callbackContext", &self.callbackContext).field("writeMessage", &self.writeMessage).field("writeError", &self.writeError).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_SessionCallbacks {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_SessionCallbacks {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_SessionCallbacks>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_SessionCallbacks {}
impl ::core::default::Default for MI_SessionCallbacks {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_SessionFT {
    pub Close: isize,
    pub GetApplication: isize,
    pub GetInstance: isize,
    pub ModifyInstance: isize,
    pub CreateInstance: isize,
    pub DeleteInstance: isize,
    pub Invoke: isize,
    pub EnumerateInstances: isize,
    pub QueryInstances: isize,
    pub AssociatorInstances: isize,
    pub ReferenceInstances: isize,
    pub Subscribe: isize,
    pub GetClass: isize,
    pub EnumerateClasses: isize,
    pub TestConnection: isize,
}
impl ::core::marker::Copy for MI_SessionFT {}
impl ::core::clone::Clone for MI_SessionFT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_SessionFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_SessionFT")
            .field("Close", &self.Close)
            .field("GetApplication", &self.GetApplication)
            .field("GetInstance", &self.GetInstance)
            .field("ModifyInstance", &self.ModifyInstance)
            .field("CreateInstance", &self.CreateInstance)
            .field("DeleteInstance", &self.DeleteInstance)
            .field("Invoke", &self.Invoke)
            .field("EnumerateInstances", &self.EnumerateInstances)
            .field("QueryInstances", &self.QueryInstances)
            .field("AssociatorInstances", &self.AssociatorInstances)
            .field("ReferenceInstances", &self.ReferenceInstances)
            .field("Subscribe", &self.Subscribe)
            .field("GetClass", &self.GetClass)
            .field("EnumerateClasses", &self.EnumerateClasses)
            .field("TestConnection", &self.TestConnection)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for MI_SessionFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_SessionFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_SessionFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_SessionFT {}
impl ::core::default::Default for MI_SessionFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Sint16A {
    pub data: *mut i16,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Sint16A {}
impl ::core::clone::Clone for MI_Sint16A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Sint16A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Sint16A").field("data", &self.data).field("size", &self.size).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Sint16A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Sint16A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Sint16A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Sint16A {}
impl ::core::default::Default for MI_Sint16A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Sint16AField {
    pub value: MI_Sint16A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Sint16AField {}
impl ::core::clone::Clone for MI_Sint16AField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Sint16AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Sint16AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Sint16AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Sint16AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Sint16AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Sint16AField {}
impl ::core::default::Default for MI_Sint16AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Sint16Field {
    pub value: i16,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Sint16Field {}
impl ::core::clone::Clone for MI_Sint16Field {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Sint16Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Sint16Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Sint16Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Sint16Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Sint16Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Sint16Field {}
impl ::core::default::Default for MI_Sint16Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Sint32A {
    pub data: *mut i32,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Sint32A {}
impl ::core::clone::Clone for MI_Sint32A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Sint32A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Sint32A").field("data", &self.data).field("size", &self.size).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Sint32A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Sint32A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Sint32A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Sint32A {}
impl ::core::default::Default for MI_Sint32A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Sint32AField {
    pub value: MI_Sint32A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Sint32AField {}
impl ::core::clone::Clone for MI_Sint32AField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Sint32AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Sint32AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Sint32AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Sint32AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Sint32AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Sint32AField {}
impl ::core::default::Default for MI_Sint32AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Sint32Field {
    pub value: i32,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Sint32Field {}
impl ::core::clone::Clone for MI_Sint32Field {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Sint32Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Sint32Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Sint32Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Sint32Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Sint32Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Sint32Field {}
impl ::core::default::Default for MI_Sint32Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Sint64A {
    pub data: *mut i64,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Sint64A {}
impl ::core::clone::Clone for MI_Sint64A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Sint64A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Sint64A").field("data", &self.data).field("size", &self.size).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Sint64A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Sint64A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Sint64A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Sint64A {}
impl ::core::default::Default for MI_Sint64A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Sint64AField {
    pub value: MI_Sint64A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Sint64AField {}
impl ::core::clone::Clone for MI_Sint64AField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Sint64AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Sint64AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Sint64AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Sint64AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Sint64AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Sint64AField {}
impl ::core::default::Default for MI_Sint64AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Sint64Field {
    pub value: i64,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Sint64Field {}
impl ::core::clone::Clone for MI_Sint64Field {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Sint64Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Sint64Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Sint64Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Sint64Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Sint64Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Sint64Field {}
impl ::core::default::Default for MI_Sint64Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Sint8A {
    pub data: *mut i8,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Sint8A {}
impl ::core::clone::Clone for MI_Sint8A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Sint8A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Sint8A").field("data", &self.data).field("size", &self.size).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Sint8A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Sint8A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Sint8A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Sint8A {}
impl ::core::default::Default for MI_Sint8A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Sint8AField {
    pub value: MI_Sint8A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Sint8AField {}
impl ::core::clone::Clone for MI_Sint8AField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Sint8AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Sint8AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Sint8AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Sint8AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Sint8AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Sint8AField {}
impl ::core::default::Default for MI_Sint8AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Sint8Field {
    pub value: i8,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Sint8Field {}
impl ::core::clone::Clone for MI_Sint8Field {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Sint8Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Sint8Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Sint8Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Sint8Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Sint8Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Sint8Field {}
impl ::core::default::Default for MI_Sint8Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_StringA {
    pub data: *mut *mut u16,
    pub size: u32,
}
impl ::core::marker::Copy for MI_StringA {}
impl ::core::clone::Clone for MI_StringA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_StringA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_StringA").field("data", &self.data).field("size", &self.size).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_StringA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_StringA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_StringA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_StringA {}
impl ::core::default::Default for MI_StringA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_StringAField {
    pub value: MI_StringA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_StringAField {}
impl ::core::clone::Clone for MI_StringAField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_StringAField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_StringAField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_StringAField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_StringAField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_StringAField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_StringAField {}
impl ::core::default::Default for MI_StringAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_StringField {
    pub value: *mut u16,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_StringField {}
impl ::core::clone::Clone for MI_StringField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_StringField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_StringField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_StringField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_StringField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_StringField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_StringField {}
impl ::core::default::Default for MI_StringField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_SubscriptionDeliveryOptions {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_SubscriptionDeliveryOptionsFT,
}
impl ::core::marker::Copy for MI_SubscriptionDeliveryOptions {}
impl ::core::clone::Clone for MI_SubscriptionDeliveryOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_SubscriptionDeliveryOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_SubscriptionDeliveryOptions").field("reserved1", &self.reserved1).field("reserved2", &self.reserved2).field("ft", &self.ft).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_SubscriptionDeliveryOptions {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_SubscriptionDeliveryOptions {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_SubscriptionDeliveryOptions>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_SubscriptionDeliveryOptions {}
impl ::core::default::Default for MI_SubscriptionDeliveryOptions {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_SubscriptionDeliveryOptionsFT {
    pub SetString: isize,
    pub SetNumber: isize,
    pub SetDateTime: isize,
    pub SetInterval: isize,
    pub AddCredentials: isize,
    pub Delete: isize,
    pub GetString: isize,
    pub GetNumber: isize,
    pub GetDateTime: isize,
    pub GetInterval: isize,
    pub GetOptionCount: isize,
    pub GetOptionAt: isize,
    pub GetOption: isize,
    pub GetCredentialsCount: isize,
    pub GetCredentialsAt: isize,
    pub GetCredentialsPasswordAt: isize,
    pub Clone: isize,
}
impl ::core::marker::Copy for MI_SubscriptionDeliveryOptionsFT {}
impl ::core::clone::Clone for MI_SubscriptionDeliveryOptionsFT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_SubscriptionDeliveryOptionsFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_SubscriptionDeliveryOptionsFT")
            .field("SetString", &self.SetString)
            .field("SetNumber", &self.SetNumber)
            .field("SetDateTime", &self.SetDateTime)
            .field("SetInterval", &self.SetInterval)
            .field("AddCredentials", &self.AddCredentials)
            .field("Delete", &self.Delete)
            .field("GetString", &self.GetString)
            .field("GetNumber", &self.GetNumber)
            .field("GetDateTime", &self.GetDateTime)
            .field("GetInterval", &self.GetInterval)
            .field("GetOptionCount", &self.GetOptionCount)
            .field("GetOptionAt", &self.GetOptionAt)
            .field("GetOption", &self.GetOption)
            .field("GetCredentialsCount", &self.GetCredentialsCount)
            .field("GetCredentialsAt", &self.GetCredentialsAt)
            .field("GetCredentialsPasswordAt", &self.GetCredentialsPasswordAt)
            .field("Clone", &self.Clone)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for MI_SubscriptionDeliveryOptionsFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_SubscriptionDeliveryOptionsFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_SubscriptionDeliveryOptionsFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_SubscriptionDeliveryOptionsFT {}
impl ::core::default::Default for MI_SubscriptionDeliveryOptionsFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MI_SubscriptionDeliveryType(pub i32);
pub const MI_SubscriptionDeliveryType_Pull: MI_SubscriptionDeliveryType = MI_SubscriptionDeliveryType(1i32);
pub const MI_SubscriptionDeliveryType_Push: MI_SubscriptionDeliveryType = MI_SubscriptionDeliveryType(2i32);
impl ::core::marker::Copy for MI_SubscriptionDeliveryType {}
impl ::core::clone::Clone for MI_SubscriptionDeliveryType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MI_SubscriptionDeliveryType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MI_SubscriptionDeliveryType {
    type Abi = Self;
}
impl ::core::fmt::Debug for MI_SubscriptionDeliveryType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MI_SubscriptionDeliveryType").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct MI_Timestamp {
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub microseconds: u32,
    pub utc: i32,
}
impl ::core::marker::Copy for MI_Timestamp {}
impl ::core::clone::Clone for MI_Timestamp {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Timestamp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Timestamp").field("year", &self.year).field("month", &self.month).field("day", &self.day).field("hour", &self.hour).field("minute", &self.minute).field("second", &self.second).field("microseconds", &self.microseconds).field("utc", &self.utc).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Timestamp {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Timestamp {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Timestamp>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Timestamp {}
impl ::core::default::Default for MI_Timestamp {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MI_Type(pub i32);
pub const MI_BOOLEAN: MI_Type = MI_Type(0i32);
pub const MI_UINT8: MI_Type = MI_Type(1i32);
pub const MI_SINT8: MI_Type = MI_Type(2i32);
pub const MI_UINT16: MI_Type = MI_Type(3i32);
pub const MI_SINT16: MI_Type = MI_Type(4i32);
pub const MI_UINT32: MI_Type = MI_Type(5i32);
pub const MI_SINT32: MI_Type = MI_Type(6i32);
pub const MI_UINT64: MI_Type = MI_Type(7i32);
pub const MI_SINT64: MI_Type = MI_Type(8i32);
pub const MI_REAL32: MI_Type = MI_Type(9i32);
pub const MI_REAL64: MI_Type = MI_Type(10i32);
pub const MI_CHAR16: MI_Type = MI_Type(11i32);
pub const MI_DATETIME: MI_Type = MI_Type(12i32);
pub const MI_STRING: MI_Type = MI_Type(13i32);
pub const MI_REFERENCE: MI_Type = MI_Type(14i32);
pub const MI_INSTANCE: MI_Type = MI_Type(15i32);
pub const MI_BOOLEANA: MI_Type = MI_Type(16i32);
pub const MI_UINT8A: MI_Type = MI_Type(17i32);
pub const MI_SINT8A: MI_Type = MI_Type(18i32);
pub const MI_UINT16A: MI_Type = MI_Type(19i32);
pub const MI_SINT16A: MI_Type = MI_Type(20i32);
pub const MI_UINT32A: MI_Type = MI_Type(21i32);
pub const MI_SINT32A: MI_Type = MI_Type(22i32);
pub const MI_UINT64A: MI_Type = MI_Type(23i32);
pub const MI_SINT64A: MI_Type = MI_Type(24i32);
pub const MI_REAL32A: MI_Type = MI_Type(25i32);
pub const MI_REAL64A: MI_Type = MI_Type(26i32);
pub const MI_CHAR16A: MI_Type = MI_Type(27i32);
pub const MI_DATETIMEA: MI_Type = MI_Type(28i32);
pub const MI_STRINGA: MI_Type = MI_Type(29i32);
pub const MI_REFERENCEA: MI_Type = MI_Type(30i32);
pub const MI_INSTANCEA: MI_Type = MI_Type(31i32);
pub const MI_ARRAY: MI_Type = MI_Type(16i32);
impl ::core::marker::Copy for MI_Type {}
impl ::core::clone::Clone for MI_Type {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MI_Type {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MI_Type {
    type Abi = Self;
}
impl ::core::fmt::Debug for MI_Type {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MI_Type").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct MI_Uint16A {
    pub data: *mut u16,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Uint16A {}
impl ::core::clone::Clone for MI_Uint16A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Uint16A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Uint16A").field("data", &self.data).field("size", &self.size).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Uint16A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Uint16A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Uint16A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Uint16A {}
impl ::core::default::Default for MI_Uint16A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Uint16AField {
    pub value: MI_Uint16A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Uint16AField {}
impl ::core::clone::Clone for MI_Uint16AField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Uint16AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Uint16AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Uint16AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Uint16AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Uint16AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Uint16AField {}
impl ::core::default::Default for MI_Uint16AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Uint16Field {
    pub value: u16,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Uint16Field {}
impl ::core::clone::Clone for MI_Uint16Field {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Uint16Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Uint16Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Uint16Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Uint16Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Uint16Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Uint16Field {}
impl ::core::default::Default for MI_Uint16Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Uint32A {
    pub data: *mut u32,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Uint32A {}
impl ::core::clone::Clone for MI_Uint32A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Uint32A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Uint32A").field("data", &self.data).field("size", &self.size).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Uint32A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Uint32A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Uint32A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Uint32A {}
impl ::core::default::Default for MI_Uint32A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Uint32AField {
    pub value: MI_Uint32A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Uint32AField {}
impl ::core::clone::Clone for MI_Uint32AField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Uint32AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Uint32AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Uint32AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Uint32AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Uint32AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Uint32AField {}
impl ::core::default::Default for MI_Uint32AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Uint32Field {
    pub value: u32,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Uint32Field {}
impl ::core::clone::Clone for MI_Uint32Field {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Uint32Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Uint32Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Uint32Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Uint32Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Uint32Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Uint32Field {}
impl ::core::default::Default for MI_Uint32Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Uint64A {
    pub data: *mut u64,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Uint64A {}
impl ::core::clone::Clone for MI_Uint64A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Uint64A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Uint64A").field("data", &self.data).field("size", &self.size).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Uint64A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Uint64A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Uint64A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Uint64A {}
impl ::core::default::Default for MI_Uint64A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Uint64AField {
    pub value: MI_Uint64A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Uint64AField {}
impl ::core::clone::Clone for MI_Uint64AField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Uint64AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Uint64AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Uint64AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Uint64AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Uint64AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Uint64AField {}
impl ::core::default::Default for MI_Uint64AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Uint64Field {
    pub value: u64,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Uint64Field {}
impl ::core::clone::Clone for MI_Uint64Field {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Uint64Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Uint64Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Uint64Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Uint64Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Uint64Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Uint64Field {}
impl ::core::default::Default for MI_Uint64Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Uint8A {
    pub data: *mut u8,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Uint8A {}
impl ::core::clone::Clone for MI_Uint8A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Uint8A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Uint8A").field("data", &self.data).field("size", &self.size).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Uint8A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Uint8A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Uint8A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Uint8A {}
impl ::core::default::Default for MI_Uint8A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Uint8AField {
    pub value: MI_Uint8A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Uint8AField {}
impl ::core::clone::Clone for MI_Uint8AField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Uint8AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Uint8AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Uint8AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Uint8AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Uint8AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Uint8AField {}
impl ::core::default::Default for MI_Uint8AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Uint8Field {
    pub value: u8,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Uint8Field {}
impl ::core::clone::Clone for MI_Uint8Field {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Uint8Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Uint8Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_Uint8Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Uint8Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Uint8Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Uint8Field {}
impl ::core::default::Default for MI_Uint8Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_UserCredentials {
    pub authenticationType: *const u16,
    pub credentials: MI_UserCredentials_0,
}
impl ::core::marker::Copy for MI_UserCredentials {}
impl ::core::clone::Clone for MI_UserCredentials {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for MI_UserCredentials {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_UserCredentials {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_UserCredentials>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_UserCredentials {}
impl ::core::default::Default for MI_UserCredentials {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union MI_UserCredentials_0 {
    pub usernamePassword: MI_UsernamePasswordCreds,
    pub certificateThumbprint: *const u16,
}
impl ::core::marker::Copy for MI_UserCredentials_0 {}
impl ::core::clone::Clone for MI_UserCredentials_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for MI_UserCredentials_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_UserCredentials_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_UserCredentials_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_UserCredentials_0 {}
impl ::core::default::Default for MI_UserCredentials_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_UsernamePasswordCreds {
    pub domain: *const u16,
    pub username: *const u16,
    pub password: *const u16,
}
impl ::core::marker::Copy for MI_UsernamePasswordCreds {}
impl ::core::clone::Clone for MI_UsernamePasswordCreds {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_UsernamePasswordCreds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_UsernamePasswordCreds").field("domain", &self.domain).field("username", &self.username).field("password", &self.password).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_UsernamePasswordCreds {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_UsernamePasswordCreds {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_UsernamePasswordCreds>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_UsernamePasswordCreds {}
impl ::core::default::Default for MI_UsernamePasswordCreds {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_UtilitiesFT {
    pub MapErrorToMiErrorCategory: isize,
    pub CimErrorFromErrorCode: isize,
}
impl ::core::marker::Copy for MI_UtilitiesFT {}
impl ::core::clone::Clone for MI_UtilitiesFT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_UtilitiesFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_UtilitiesFT").field("MapErrorToMiErrorCategory", &self.MapErrorToMiErrorCategory).field("CimErrorFromErrorCode", &self.CimErrorFromErrorCode).finish()
    }
}
unsafe impl ::windows_core::Abi for MI_UtilitiesFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_UtilitiesFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_UtilitiesFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_UtilitiesFT {}
impl ::core::default::Default for MI_UtilitiesFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union MI_Value {
    pub boolean: u8,
    pub uint8: u8,
    pub sint8: i8,
    pub uint16: u16,
    pub sint16: i16,
    pub uint32: u32,
    pub sint32: i32,
    pub uint64: u64,
    pub sint64: i64,
    pub real32: f32,
    pub real64: f64,
    pub char16: u16,
    pub datetime: MI_Datetime,
    pub string: *mut u16,
    pub instance: *mut MI_Instance,
    pub reference: *mut MI_Instance,
    pub booleana: MI_BooleanA,
    pub uint8a: MI_Uint8A,
    pub sint8a: MI_Sint8A,
    pub uint16a: MI_Uint16A,
    pub sint16a: MI_Sint16A,
    pub uint32a: MI_Uint32A,
    pub sint32a: MI_Sint32A,
    pub uint64a: MI_Uint64A,
    pub sint64a: MI_Sint64A,
    pub real32a: MI_Real32A,
    pub real64a: MI_Real64A,
    pub char16a: MI_Char16A,
    pub datetimea: MI_DatetimeA,
    pub stringa: MI_StringA,
    pub referencea: MI_ReferenceA,
    pub instancea: MI_InstanceA,
    pub array: MI_Array,
}
impl ::core::marker::Copy for MI_Value {}
impl ::core::clone::Clone for MI_Value {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for MI_Value {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Value {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Value>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Value {}
impl ::core::default::Default for MI_Value {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const MI_WRITEMESSAGE_CHANNEL_DEBUG: u32 = 2u32;
pub const MI_WRITEMESSAGE_CHANNEL_VERBOSE: u32 = 1u32;
pub const MI_WRITEMESSAGE_CHANNEL_WARNING: u32 = 0u32;
pub const MofCompiler: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6daf9757_2e37_11d2_aec9_00c04fb68820);
#[repr(C)]
pub struct SWbemAnalysisMatrix {
    pub m_uVersion: u32,
    pub m_uMatrixType: u32,
    pub m_pszProperty: ::windows_core::PCWSTR,
    pub m_uPropertyType: u32,
    pub m_uEntries: u32,
    pub m_pValues: *mut *mut ::core::ffi::c_void,
    pub m_pbTruthTable: *mut ::win32_foundation::BOOL,
}
impl ::core::marker::Copy for SWbemAnalysisMatrix {}
impl ::core::clone::Clone for SWbemAnalysisMatrix {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SWbemAnalysisMatrix {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SWbemAnalysisMatrix").field("m_uVersion", &self.m_uVersion).field("m_uMatrixType", &self.m_uMatrixType).field("m_pszProperty", &self.m_pszProperty).field("m_uPropertyType", &self.m_uPropertyType).field("m_uEntries", &self.m_uEntries).field("m_pValues", &self.m_pValues).field("m_pbTruthTable", &self.m_pbTruthTable).finish()
    }
}
unsafe impl ::windows_core::Abi for SWbemAnalysisMatrix {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SWbemAnalysisMatrix {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SWbemAnalysisMatrix>()) == 0 }
    }
}
impl ::core::cmp::Eq for SWbemAnalysisMatrix {}
impl ::core::default::Default for SWbemAnalysisMatrix {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SWbemAnalysisMatrixList {
    pub m_uVersion: u32,
    pub m_uMatrixType: u32,
    pub m_uNumMatrices: u32,
    pub m_pMatrices: *mut SWbemAnalysisMatrix,
}
impl ::core::marker::Copy for SWbemAnalysisMatrixList {}
impl ::core::clone::Clone for SWbemAnalysisMatrixList {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SWbemAnalysisMatrixList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SWbemAnalysisMatrixList").field("m_uVersion", &self.m_uVersion).field("m_uMatrixType", &self.m_uMatrixType).field("m_uNumMatrices", &self.m_uNumMatrices).field("m_pMatrices", &self.m_pMatrices).finish()
    }
}
unsafe impl ::windows_core::Abi for SWbemAnalysisMatrixList {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SWbemAnalysisMatrixList {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SWbemAnalysisMatrixList>()) == 0 }
    }
}
impl ::core::cmp::Eq for SWbemAnalysisMatrixList {}
impl ::core::default::Default for SWbemAnalysisMatrixList {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SWbemAssocQueryInf {
    pub m_uVersion: u32,
    pub m_uAnalysisType: u32,
    pub m_uFeatureMask: u32,
    pub m_pPath: ::core::option::Option<IWbemPath>,
    pub m_pszPath: ::windows_core::PWSTR,
    pub m_pszQueryText: ::windows_core::PWSTR,
    pub m_pszResultClass: ::windows_core::PWSTR,
    pub m_pszAssocClass: ::windows_core::PWSTR,
    pub m_pszRole: ::windows_core::PWSTR,
    pub m_pszResultRole: ::windows_core::PWSTR,
    pub m_pszRequiredQualifier: ::windows_core::PWSTR,
    pub m_pszRequiredAssocQualifier: ::windows_core::PWSTR,
}
impl ::core::clone::Clone for SWbemAssocQueryInf {
    fn clone(&self) -> Self {
        Self {
            m_uVersion: self.m_uVersion,
            m_uAnalysisType: self.m_uAnalysisType,
            m_uFeatureMask: self.m_uFeatureMask,
            m_pPath: self.m_pPath.clone(),
            m_pszPath: self.m_pszPath,
            m_pszQueryText: self.m_pszQueryText,
            m_pszResultClass: self.m_pszResultClass,
            m_pszAssocClass: self.m_pszAssocClass,
            m_pszRole: self.m_pszRole,
            m_pszResultRole: self.m_pszResultRole,
            m_pszRequiredQualifier: self.m_pszRequiredQualifier,
            m_pszRequiredAssocQualifier: self.m_pszRequiredAssocQualifier,
        }
    }
}
impl ::core::fmt::Debug for SWbemAssocQueryInf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SWbemAssocQueryInf")
            .field("m_uVersion", &self.m_uVersion)
            .field("m_uAnalysisType", &self.m_uAnalysisType)
            .field("m_uFeatureMask", &self.m_uFeatureMask)
            .field("m_pPath", &self.m_pPath)
            .field("m_pszPath", &self.m_pszPath)
            .field("m_pszQueryText", &self.m_pszQueryText)
            .field("m_pszResultClass", &self.m_pszResultClass)
            .field("m_pszAssocClass", &self.m_pszAssocClass)
            .field("m_pszRole", &self.m_pszRole)
            .field("m_pszResultRole", &self.m_pszResultRole)
            .field("m_pszRequiredQualifier", &self.m_pszRequiredQualifier)
            .field("m_pszRequiredAssocQualifier", &self.m_pszRequiredAssocQualifier)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for SWbemAssocQueryInf {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for SWbemAssocQueryInf {
    fn eq(&self, other: &Self) -> bool {
        self.m_uVersion == other.m_uVersion && self.m_uAnalysisType == other.m_uAnalysisType && self.m_uFeatureMask == other.m_uFeatureMask && self.m_pPath == other.m_pPath && self.m_pszPath == other.m_pszPath && self.m_pszQueryText == other.m_pszQueryText && self.m_pszResultClass == other.m_pszResultClass && self.m_pszAssocClass == other.m_pszAssocClass && self.m_pszRole == other.m_pszRole && self.m_pszResultRole == other.m_pszResultRole && self.m_pszRequiredQualifier == other.m_pszRequiredQualifier && self.m_pszRequiredAssocQualifier == other.m_pszRequiredAssocQualifier
    }
}
impl ::core::cmp::Eq for SWbemAssocQueryInf {}
impl ::core::default::Default for SWbemAssocQueryInf {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const SWbemDateTime: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x47dfbe54_cf76_11d3_b38f_00105a1f473a);
pub const SWbemEventSource: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x04b83d58_21ae_11d2_8b33_00600806d9b6);
pub const SWbemLastError: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc2feeeac_cfcd_11d1_8b05_00600806d9b6);
pub const SWbemLocator: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x76a64158_cb41_11d1_8b02_00600806d9b6);
pub const SWbemMethod: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x04b83d5b_21ae_11d2_8b33_00600806d9b6);
pub const SWbemMethodSet: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x04b83d5a_21ae_11d2_8b33_00600806d9b6);
pub const SWbemNamedValue: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x04b83d60_21ae_11d2_8b33_00600806d9b6);
pub const SWbemNamedValueSet: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9aed384e_ce8b_11d1_8b05_00600806d9b6);
pub const SWbemObject: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x04b83d62_21ae_11d2_8b33_00600806d9b6);
pub const SWbemObjectEx: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd6bdafb2_9435_491f_bb87_6aa0f0bc31a2);
pub const SWbemObjectPath: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5791bc26_ce9c_11d1_97bf_0000f81e849c);
pub const SWbemObjectSet: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x04b83d61_21ae_11d2_8b33_00600806d9b6);
pub const SWbemPrivilege: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x26ee67bc_5804_11d2_8b4a_00600806d9b6);
pub const SWbemPrivilegeSet: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x26ee67be_5804_11d2_8b4a_00600806d9b6);
pub const SWbemProperty: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x04b83d5d_21ae_11d2_8b33_00600806d9b6);
pub const SWbemPropertySet: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x04b83d5c_21ae_11d2_8b33_00600806d9b6);
pub const SWbemQualifier: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x04b83d5f_21ae_11d2_8b33_00600806d9b6);
pub const SWbemQualifierSet: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x04b83d5e_21ae_11d2_8b33_00600806d9b6);
#[repr(C)]
pub struct SWbemQueryQualifiedName {
    pub m_uVersion: u32,
    pub m_uTokenType: u32,
    pub m_uNameListSize: u32,
    pub m_ppszNameList: *mut ::windows_core::PWSTR,
    pub m_bArraysUsed: ::win32_foundation::BOOL,
    pub m_pbArrayElUsed: *mut ::win32_foundation::BOOL,
    pub m_puArrayIndex: *mut u32,
}
impl ::core::marker::Copy for SWbemQueryQualifiedName {}
impl ::core::clone::Clone for SWbemQueryQualifiedName {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SWbemQueryQualifiedName {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SWbemQueryQualifiedName").field("m_uVersion", &self.m_uVersion).field("m_uTokenType", &self.m_uTokenType).field("m_uNameListSize", &self.m_uNameListSize).field("m_ppszNameList", &self.m_ppszNameList).field("m_bArraysUsed", &self.m_bArraysUsed).field("m_pbArrayElUsed", &self.m_pbArrayElUsed).field("m_puArrayIndex", &self.m_puArrayIndex).finish()
    }
}
unsafe impl ::windows_core::Abi for SWbemQueryQualifiedName {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SWbemQueryQualifiedName {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SWbemQueryQualifiedName>()) == 0 }
    }
}
impl ::core::cmp::Eq for SWbemQueryQualifiedName {}
impl ::core::default::Default for SWbemQueryQualifiedName {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const SWbemRefreshableItem: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8c6854bc_de4b_11d3_b390_00105a1f473a);
pub const SWbemRefresher: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd269bf5c_d9c1_11d3_b38f_00105a1f473a);
#[repr(C)]
pub union SWbemRpnConst {
    pub m_pszStrVal: ::windows_core::PCWSTR,
    pub m_bBoolVal: ::win32_foundation::BOOL,
    pub m_lLongVal: i32,
    pub m_uLongVal: u32,
    pub m_dblVal: f64,
    pub m_lVal64: i64,
    pub m_uVal64: i64,
}
impl ::core::marker::Copy for SWbemRpnConst {}
impl ::core::clone::Clone for SWbemRpnConst {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for SWbemRpnConst {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SWbemRpnConst {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SWbemRpnConst>()) == 0 }
    }
}
impl ::core::cmp::Eq for SWbemRpnConst {}
impl ::core::default::Default for SWbemRpnConst {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SWbemRpnEncodedQuery {
    pub m_uVersion: u32,
    pub m_uTokenType: u32,
    pub m_uParsedFeatureMask: u64,
    pub m_uDetectedArraySize: u32,
    pub m_puDetectedFeatures: *mut u32,
    pub m_uSelectListSize: u32,
    pub m_ppSelectList: *mut *mut SWbemQueryQualifiedName,
    pub m_uFromTargetType: u32,
    pub m_pszOptionalFromPath: ::windows_core::PCWSTR,
    pub m_uFromListSize: u32,
    pub m_ppszFromList: *mut ::windows_core::PWSTR,
    pub m_uWhereClauseSize: u32,
    pub m_ppRpnWhereClause: *mut *mut SWbemRpnQueryToken,
    pub m_dblWithinPolling: f64,
    pub m_dblWithinWindow: f64,
    pub m_uOrderByListSize: u32,
    pub m_ppszOrderByList: *mut ::windows_core::PWSTR,
    pub m_uOrderDirectionEl: *mut u32,
}
impl ::core::marker::Copy for SWbemRpnEncodedQuery {}
impl ::core::clone::Clone for SWbemRpnEncodedQuery {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SWbemRpnEncodedQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SWbemRpnEncodedQuery")
            .field("m_uVersion", &self.m_uVersion)
            .field("m_uTokenType", &self.m_uTokenType)
            .field("m_uParsedFeatureMask", &self.m_uParsedFeatureMask)
            .field("m_uDetectedArraySize", &self.m_uDetectedArraySize)
            .field("m_puDetectedFeatures", &self.m_puDetectedFeatures)
            .field("m_uSelectListSize", &self.m_uSelectListSize)
            .field("m_ppSelectList", &self.m_ppSelectList)
            .field("m_uFromTargetType", &self.m_uFromTargetType)
            .field("m_pszOptionalFromPath", &self.m_pszOptionalFromPath)
            .field("m_uFromListSize", &self.m_uFromListSize)
            .field("m_ppszFromList", &self.m_ppszFromList)
            .field("m_uWhereClauseSize", &self.m_uWhereClauseSize)
            .field("m_ppRpnWhereClause", &self.m_ppRpnWhereClause)
            .field("m_dblWithinPolling", &self.m_dblWithinPolling)
            .field("m_dblWithinWindow", &self.m_dblWithinWindow)
            .field("m_uOrderByListSize", &self.m_uOrderByListSize)
            .field("m_ppszOrderByList", &self.m_ppszOrderByList)
            .field("m_uOrderDirectionEl", &self.m_uOrderDirectionEl)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for SWbemRpnEncodedQuery {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SWbemRpnEncodedQuery {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SWbemRpnEncodedQuery>()) == 0 }
    }
}
impl ::core::cmp::Eq for SWbemRpnEncodedQuery {}
impl ::core::default::Default for SWbemRpnEncodedQuery {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SWbemRpnQueryToken {
    pub m_uVersion: u32,
    pub m_uTokenType: u32,
    pub m_uSubexpressionShape: u32,
    pub m_uOperator: u32,
    pub m_pRightIdent: *mut SWbemQueryQualifiedName,
    pub m_pLeftIdent: *mut SWbemQueryQualifiedName,
    pub m_uConstApparentType: u32,
    pub m_Const: SWbemRpnConst,
    pub m_uConst2ApparentType: u32,
    pub m_Const2: SWbemRpnConst,
    pub m_pszRightFunc: ::windows_core::PCWSTR,
    pub m_pszLeftFunc: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for SWbemRpnQueryToken {}
impl ::core::clone::Clone for SWbemRpnQueryToken {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for SWbemRpnQueryToken {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SWbemRpnQueryToken {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SWbemRpnQueryToken>()) == 0 }
    }
}
impl ::core::cmp::Eq for SWbemRpnQueryToken {}
impl ::core::default::Default for SWbemRpnQueryToken {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SWbemRpnTokenList {
    pub m_uVersion: u32,
    pub m_uTokenType: u32,
    pub m_uNumTokens: u32,
}
impl ::core::marker::Copy for SWbemRpnTokenList {}
impl ::core::clone::Clone for SWbemRpnTokenList {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SWbemRpnTokenList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SWbemRpnTokenList").field("m_uVersion", &self.m_uVersion).field("m_uTokenType", &self.m_uTokenType).field("m_uNumTokens", &self.m_uNumTokens).finish()
    }
}
unsafe impl ::windows_core::Abi for SWbemRpnTokenList {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SWbemRpnTokenList {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SWbemRpnTokenList>()) == 0 }
    }
}
impl ::core::cmp::Eq for SWbemRpnTokenList {}
impl ::core::default::Default for SWbemRpnTokenList {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const SWbemSecurity: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb54d66e9_2287_11d2_8b33_00600806d9b6);
pub const SWbemServices: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x04b83d63_21ae_11d2_8b33_00600806d9b6);
pub const SWbemServicesEx: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x62e522dc_8cf3_40a8_8b2e_37d595651e40);
pub const SWbemSink: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x75718c9a_f029_11d1_a1ac_00c04fb6c223);
pub const UnsecuredApartment: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x49bd2028_1523_11d1_ad79_00c04fd8fdff);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WBEMSTATUS(pub i32);
pub const WBEM_NO_ERROR: WBEMSTATUS = WBEMSTATUS(0i32);
pub const WBEM_S_NO_ERROR: WBEMSTATUS = WBEMSTATUS(0i32);
pub const WBEM_S_SAME: WBEMSTATUS = WBEMSTATUS(0i32);
pub const WBEM_S_FALSE: WBEMSTATUS = WBEMSTATUS(1i32);
pub const WBEM_S_ALREADY_EXISTS: WBEMSTATUS = WBEMSTATUS(262145i32);
pub const WBEM_S_RESET_TO_DEFAULT: WBEMSTATUS = WBEMSTATUS(262146i32);
pub const WBEM_S_DIFFERENT: WBEMSTATUS = WBEMSTATUS(262147i32);
pub const WBEM_S_TIMEDOUT: WBEMSTATUS = WBEMSTATUS(262148i32);
pub const WBEM_S_NO_MORE_DATA: WBEMSTATUS = WBEMSTATUS(262149i32);
pub const WBEM_S_OPERATION_CANCELLED: WBEMSTATUS = WBEMSTATUS(262150i32);
pub const WBEM_S_PENDING: WBEMSTATUS = WBEMSTATUS(262151i32);
pub const WBEM_S_DUPLICATE_OBJECTS: WBEMSTATUS = WBEMSTATUS(262152i32);
pub const WBEM_S_ACCESS_DENIED: WBEMSTATUS = WBEMSTATUS(262153i32);
pub const WBEM_S_PARTIAL_RESULTS: WBEMSTATUS = WBEMSTATUS(262160i32);
pub const WBEM_S_SOURCE_NOT_AVAILABLE: WBEMSTATUS = WBEMSTATUS(262167i32);
pub const WBEM_E_FAILED: WBEMSTATUS = WBEMSTATUS(-2147217407i32);
pub const WBEM_E_NOT_FOUND: WBEMSTATUS = WBEMSTATUS(-2147217406i32);
pub const WBEM_E_ACCESS_DENIED: WBEMSTATUS = WBEMSTATUS(-2147217405i32);
pub const WBEM_E_PROVIDER_FAILURE: WBEMSTATUS = WBEMSTATUS(-2147217404i32);
pub const WBEM_E_TYPE_MISMATCH: WBEMSTATUS = WBEMSTATUS(-2147217403i32);
pub const WBEM_E_OUT_OF_MEMORY: WBEMSTATUS = WBEMSTATUS(-2147217402i32);
pub const WBEM_E_INVALID_CONTEXT: WBEMSTATUS = WBEMSTATUS(-2147217401i32);
pub const WBEM_E_INVALID_PARAMETER: WBEMSTATUS = WBEMSTATUS(-2147217400i32);
pub const WBEM_E_NOT_AVAILABLE: WBEMSTATUS = WBEMSTATUS(-2147217399i32);
pub const WBEM_E_CRITICAL_ERROR: WBEMSTATUS = WBEMSTATUS(-2147217398i32);
pub const WBEM_E_INVALID_STREAM: WBEMSTATUS = WBEMSTATUS(-2147217397i32);
pub const WBEM_E_NOT_SUPPORTED: WBEMSTATUS = WBEMSTATUS(-2147217396i32);
pub const WBEM_E_INVALID_SUPERCLASS: WBEMSTATUS = WBEMSTATUS(-2147217395i32);
pub const WBEM_E_INVALID_NAMESPACE: WBEMSTATUS = WBEMSTATUS(-2147217394i32);
pub const WBEM_E_INVALID_OBJECT: WBEMSTATUS = WBEMSTATUS(-2147217393i32);
pub const WBEM_E_INVALID_CLASS: WBEMSTATUS = WBEMSTATUS(-2147217392i32);
pub const WBEM_E_PROVIDER_NOT_FOUND: WBEMSTATUS = WBEMSTATUS(-2147217391i32);
pub const WBEM_E_INVALID_PROVIDER_REGISTRATION: WBEMSTATUS = WBEMSTATUS(-2147217390i32);
pub const WBEM_E_PROVIDER_LOAD_FAILURE: WBEMSTATUS = WBEMSTATUS(-2147217389i32);
pub const WBEM_E_INITIALIZATION_FAILURE: WBEMSTATUS = WBEMSTATUS(-2147217388i32);
pub const WBEM_E_TRANSPORT_FAILURE: WBEMSTATUS = WBEMSTATUS(-2147217387i32);
pub const WBEM_E_INVALID_OPERATION: WBEMSTATUS = WBEMSTATUS(-2147217386i32);
pub const WBEM_E_INVALID_QUERY: WBEMSTATUS = WBEMSTATUS(-2147217385i32);
pub const WBEM_E_INVALID_QUERY_TYPE: WBEMSTATUS = WBEMSTATUS(-2147217384i32);
pub const WBEM_E_ALREADY_EXISTS: WBEMSTATUS = WBEMSTATUS(-2147217383i32);
pub const WBEM_E_OVERRIDE_NOT_ALLOWED: WBEMSTATUS = WBEMSTATUS(-2147217382i32);
pub const WBEM_E_PROPAGATED_QUALIFIER: WBEMSTATUS = WBEMSTATUS(-2147217381i32);
pub const WBEM_E_PROPAGATED_PROPERTY: WBEMSTATUS = WBEMSTATUS(-2147217380i32);
pub const WBEM_E_UNEXPECTED: WBEMSTATUS = WBEMSTATUS(-2147217379i32);
pub const WBEM_E_ILLEGAL_OPERATION: WBEMSTATUS = WBEMSTATUS(-2147217378i32);
pub const WBEM_E_CANNOT_BE_KEY: WBEMSTATUS = WBEMSTATUS(-2147217377i32);
pub const WBEM_E_INCOMPLETE_CLASS: WBEMSTATUS = WBEMSTATUS(-2147217376i32);
pub const WBEM_E_INVALID_SYNTAX: WBEMSTATUS = WBEMSTATUS(-2147217375i32);
pub const WBEM_E_NONDECORATED_OBJECT: WBEMSTATUS = WBEMSTATUS(-2147217374i32);
pub const WBEM_E_READ_ONLY: WBEMSTATUS = WBEMSTATUS(-2147217373i32);
pub const WBEM_E_PROVIDER_NOT_CAPABLE: WBEMSTATUS = WBEMSTATUS(-2147217372i32);
pub const WBEM_E_CLASS_HAS_CHILDREN: WBEMSTATUS = WBEMSTATUS(-2147217371i32);
pub const WBEM_E_CLASS_HAS_INSTANCES: WBEMSTATUS = WBEMSTATUS(-2147217370i32);
pub const WBEM_E_QUERY_NOT_IMPLEMENTED: WBEMSTATUS = WBEMSTATUS(-2147217369i32);
pub const WBEM_E_ILLEGAL_NULL: WBEMSTATUS = WBEMSTATUS(-2147217368i32);
pub const WBEM_E_INVALID_QUALIFIER_TYPE: WBEMSTATUS = WBEMSTATUS(-2147217367i32);
pub const WBEM_E_INVALID_PROPERTY_TYPE: WBEMSTATUS = WBEMSTATUS(-2147217366i32);
pub const WBEM_E_VALUE_OUT_OF_RANGE: WBEMSTATUS = WBEMSTATUS(-2147217365i32);
pub const WBEM_E_CANNOT_BE_SINGLETON: WBEMSTATUS = WBEMSTATUS(-2147217364i32);
pub const WBEM_E_INVALID_CIM_TYPE: WBEMSTATUS = WBEMSTATUS(-2147217363i32);
pub const WBEM_E_INVALID_METHOD: WBEMSTATUS = WBEMSTATUS(-2147217362i32);
pub const WBEM_E_INVALID_METHOD_PARAMETERS: WBEMSTATUS = WBEMSTATUS(-2147217361i32);
pub const WBEM_E_SYSTEM_PROPERTY: WBEMSTATUS = WBEMSTATUS(-2147217360i32);
pub const WBEM_E_INVALID_PROPERTY: WBEMSTATUS = WBEMSTATUS(-2147217359i32);
pub const WBEM_E_CALL_CANCELLED: WBEMSTATUS = WBEMSTATUS(-2147217358i32);
pub const WBEM_E_SHUTTING_DOWN: WBEMSTATUS = WBEMSTATUS(-2147217357i32);
pub const WBEM_E_PROPAGATED_METHOD: WBEMSTATUS = WBEMSTATUS(-2147217356i32);
pub const WBEM_E_UNSUPPORTED_PARAMETER: WBEMSTATUS = WBEMSTATUS(-2147217355i32);
pub const WBEM_E_MISSING_PARAMETER_ID: WBEMSTATUS = WBEMSTATUS(-2147217354i32);
pub const WBEM_E_INVALID_PARAMETER_ID: WBEMSTATUS = WBEMSTATUS(-2147217353i32);
pub const WBEM_E_NONCONSECUTIVE_PARAMETER_IDS: WBEMSTATUS = WBEMSTATUS(-2147217352i32);
pub const WBEM_E_PARAMETER_ID_ON_RETVAL: WBEMSTATUS = WBEMSTATUS(-2147217351i32);
pub const WBEM_E_INVALID_OBJECT_PATH: WBEMSTATUS = WBEMSTATUS(-2147217350i32);
pub const WBEM_E_OUT_OF_DISK_SPACE: WBEMSTATUS = WBEMSTATUS(-2147217349i32);
pub const WBEM_E_BUFFER_TOO_SMALL: WBEMSTATUS = WBEMSTATUS(-2147217348i32);
pub const WBEM_E_UNSUPPORTED_PUT_EXTENSION: WBEMSTATUS = WBEMSTATUS(-2147217347i32);
pub const WBEM_E_UNKNOWN_OBJECT_TYPE: WBEMSTATUS = WBEMSTATUS(-2147217346i32);
pub const WBEM_E_UNKNOWN_PACKET_TYPE: WBEMSTATUS = WBEMSTATUS(-2147217345i32);
pub const WBEM_E_MARSHAL_VERSION_MISMATCH: WBEMSTATUS = WBEMSTATUS(-2147217344i32);
pub const WBEM_E_MARSHAL_INVALID_SIGNATURE: WBEMSTATUS = WBEMSTATUS(-2147217343i32);
pub const WBEM_E_INVALID_QUALIFIER: WBEMSTATUS = WBEMSTATUS(-2147217342i32);
pub const WBEM_E_INVALID_DUPLICATE_PARAMETER: WBEMSTATUS = WBEMSTATUS(-2147217341i32);
pub const WBEM_E_TOO_MUCH_DATA: WBEMSTATUS = WBEMSTATUS(-2147217340i32);
pub const WBEM_E_SERVER_TOO_BUSY: WBEMSTATUS = WBEMSTATUS(-2147217339i32);
pub const WBEM_E_INVALID_FLAVOR: WBEMSTATUS = WBEMSTATUS(-2147217338i32);
pub const WBEM_E_CIRCULAR_REFERENCE: WBEMSTATUS = WBEMSTATUS(-2147217337i32);
pub const WBEM_E_UNSUPPORTED_CLASS_UPDATE: WBEMSTATUS = WBEMSTATUS(-2147217336i32);
pub const WBEM_E_CANNOT_CHANGE_KEY_INHERITANCE: WBEMSTATUS = WBEMSTATUS(-2147217335i32);
pub const WBEM_E_CANNOT_CHANGE_INDEX_INHERITANCE: WBEMSTATUS = WBEMSTATUS(-2147217328i32);
pub const WBEM_E_TOO_MANY_PROPERTIES: WBEMSTATUS = WBEMSTATUS(-2147217327i32);
pub const WBEM_E_UPDATE_TYPE_MISMATCH: WBEMSTATUS = WBEMSTATUS(-2147217326i32);
pub const WBEM_E_UPDATE_OVERRIDE_NOT_ALLOWED: WBEMSTATUS = WBEMSTATUS(-2147217325i32);
pub const WBEM_E_UPDATE_PROPAGATED_METHOD: WBEMSTATUS = WBEMSTATUS(-2147217324i32);
pub const WBEM_E_METHOD_NOT_IMPLEMENTED: WBEMSTATUS = WBEMSTATUS(-2147217323i32);
pub const WBEM_E_METHOD_DISABLED: WBEMSTATUS = WBEMSTATUS(-2147217322i32);
pub const WBEM_E_REFRESHER_BUSY: WBEMSTATUS = WBEMSTATUS(-2147217321i32);
pub const WBEM_E_UNPARSABLE_QUERY: WBEMSTATUS = WBEMSTATUS(-2147217320i32);
pub const WBEM_E_NOT_EVENT_CLASS: WBEMSTATUS = WBEMSTATUS(-2147217319i32);
pub const WBEM_E_MISSING_GROUP_WITHIN: WBEMSTATUS = WBEMSTATUS(-2147217318i32);
pub const WBEM_E_MISSING_AGGREGATION_LIST: WBEMSTATUS = WBEMSTATUS(-2147217317i32);
pub const WBEM_E_PROPERTY_NOT_AN_OBJECT: WBEMSTATUS = WBEMSTATUS(-2147217316i32);
pub const WBEM_E_AGGREGATING_BY_OBJECT: WBEMSTATUS = WBEMSTATUS(-2147217315i32);
pub const WBEM_E_UNINTERPRETABLE_PROVIDER_QUERY: WBEMSTATUS = WBEMSTATUS(-2147217313i32);
pub const WBEM_E_BACKUP_RESTORE_WINMGMT_RUNNING: WBEMSTATUS = WBEMSTATUS(-2147217312i32);
pub const WBEM_E_QUEUE_OVERFLOW: WBEMSTATUS = WBEMSTATUS(-2147217311i32);
pub const WBEM_E_PRIVILEGE_NOT_HELD: WBEMSTATUS = WBEMSTATUS(-2147217310i32);
pub const WBEM_E_INVALID_OPERATOR: WBEMSTATUS = WBEMSTATUS(-2147217309i32);
pub const WBEM_E_LOCAL_CREDENTIALS: WBEMSTATUS = WBEMSTATUS(-2147217308i32);
pub const WBEM_E_CANNOT_BE_ABSTRACT: WBEMSTATUS = WBEMSTATUS(-2147217307i32);
pub const WBEM_E_AMENDED_OBJECT: WBEMSTATUS = WBEMSTATUS(-2147217306i32);
pub const WBEM_E_CLIENT_TOO_SLOW: WBEMSTATUS = WBEMSTATUS(-2147217305i32);
pub const WBEM_E_NULL_SECURITY_DESCRIPTOR: WBEMSTATUS = WBEMSTATUS(-2147217304i32);
pub const WBEM_E_TIMED_OUT: WBEMSTATUS = WBEMSTATUS(-2147217303i32);
pub const WBEM_E_INVALID_ASSOCIATION: WBEMSTATUS = WBEMSTATUS(-2147217302i32);
pub const WBEM_E_AMBIGUOUS_OPERATION: WBEMSTATUS = WBEMSTATUS(-2147217301i32);
pub const WBEM_E_QUOTA_VIOLATION: WBEMSTATUS = WBEMSTATUS(-2147217300i32);
pub const WBEM_E_RESERVED_001: WBEMSTATUS = WBEMSTATUS(-2147217299i32);
pub const WBEM_E_RESERVED_002: WBEMSTATUS = WBEMSTATUS(-2147217298i32);
pub const WBEM_E_UNSUPPORTED_LOCALE: WBEMSTATUS = WBEMSTATUS(-2147217297i32);
pub const WBEM_E_HANDLE_OUT_OF_DATE: WBEMSTATUS = WBEMSTATUS(-2147217296i32);
pub const WBEM_E_CONNECTION_FAILED: WBEMSTATUS = WBEMSTATUS(-2147217295i32);
pub const WBEM_E_INVALID_HANDLE_REQUEST: WBEMSTATUS = WBEMSTATUS(-2147217294i32);
pub const WBEM_E_PROPERTY_NAME_TOO_WIDE: WBEMSTATUS = WBEMSTATUS(-2147217293i32);
pub const WBEM_E_CLASS_NAME_TOO_WIDE: WBEMSTATUS = WBEMSTATUS(-2147217292i32);
pub const WBEM_E_METHOD_NAME_TOO_WIDE: WBEMSTATUS = WBEMSTATUS(-2147217291i32);
pub const WBEM_E_QUALIFIER_NAME_TOO_WIDE: WBEMSTATUS = WBEMSTATUS(-2147217290i32);
pub const WBEM_E_RERUN_COMMAND: WBEMSTATUS = WBEMSTATUS(-2147217289i32);
pub const WBEM_E_DATABASE_VER_MISMATCH: WBEMSTATUS = WBEMSTATUS(-2147217288i32);
pub const WBEM_E_VETO_DELETE: WBEMSTATUS = WBEMSTATUS(-2147217287i32);
pub const WBEM_E_VETO_PUT: WBEMSTATUS = WBEMSTATUS(-2147217286i32);
pub const WBEM_E_INVALID_LOCALE: WBEMSTATUS = WBEMSTATUS(-2147217280i32);
pub const WBEM_E_PROVIDER_SUSPENDED: WBEMSTATUS = WBEMSTATUS(-2147217279i32);
pub const WBEM_E_SYNCHRONIZATION_REQUIRED: WBEMSTATUS = WBEMSTATUS(-2147217278i32);
pub const WBEM_E_NO_SCHEMA: WBEMSTATUS = WBEMSTATUS(-2147217277i32);
pub const WBEM_E_PROVIDER_ALREADY_REGISTERED: WBEMSTATUS = WBEMSTATUS(-2147217276i32);
pub const WBEM_E_PROVIDER_NOT_REGISTERED: WBEMSTATUS = WBEMSTATUS(-2147217275i32);
pub const WBEM_E_FATAL_TRANSPORT_ERROR: WBEMSTATUS = WBEMSTATUS(-2147217274i32);
pub const WBEM_E_ENCRYPTED_CONNECTION_REQUIRED: WBEMSTATUS = WBEMSTATUS(-2147217273i32);
pub const WBEM_E_PROVIDER_TIMED_OUT: WBEMSTATUS = WBEMSTATUS(-2147217272i32);
pub const WBEM_E_NO_KEY: WBEMSTATUS = WBEMSTATUS(-2147217271i32);
pub const WBEM_E_PROVIDER_DISABLED: WBEMSTATUS = WBEMSTATUS(-2147217270i32);
pub const WBEMESS_E_REGISTRATION_TOO_BROAD: WBEMSTATUS = WBEMSTATUS(-2147213311i32);
pub const WBEMESS_E_REGISTRATION_TOO_PRECISE: WBEMSTATUS = WBEMSTATUS(-2147213310i32);
pub const WBEMESS_E_AUTHZ_NOT_PRIVILEGED: WBEMSTATUS = WBEMSTATUS(-2147213309i32);
pub const WBEMMOF_E_EXPECTED_QUALIFIER_NAME: WBEMSTATUS = WBEMSTATUS(-2147205119i32);
pub const WBEMMOF_E_EXPECTED_SEMI: WBEMSTATUS = WBEMSTATUS(-2147205118i32);
pub const WBEMMOF_E_EXPECTED_OPEN_BRACE: WBEMSTATUS = WBEMSTATUS(-2147205117i32);
pub const WBEMMOF_E_EXPECTED_CLOSE_BRACE: WBEMSTATUS = WBEMSTATUS(-2147205116i32);
pub const WBEMMOF_E_EXPECTED_CLOSE_BRACKET: WBEMSTATUS = WBEMSTATUS(-2147205115i32);
pub const WBEMMOF_E_EXPECTED_CLOSE_PAREN: WBEMSTATUS = WBEMSTATUS(-2147205114i32);
pub const WBEMMOF_E_ILLEGAL_CONSTANT_VALUE: WBEMSTATUS = WBEMSTATUS(-2147205113i32);
pub const WBEMMOF_E_EXPECTED_TYPE_IDENTIFIER: WBEMSTATUS = WBEMSTATUS(-2147205112i32);
pub const WBEMMOF_E_EXPECTED_OPEN_PAREN: WBEMSTATUS = WBEMSTATUS(-2147205111i32);
pub const WBEMMOF_E_UNRECOGNIZED_TOKEN: WBEMSTATUS = WBEMSTATUS(-2147205110i32);
pub const WBEMMOF_E_UNRECOGNIZED_TYPE: WBEMSTATUS = WBEMSTATUS(-2147205109i32);
pub const WBEMMOF_E_EXPECTED_PROPERTY_NAME: WBEMSTATUS = WBEMSTATUS(-2147205108i32);
pub const WBEMMOF_E_TYPEDEF_NOT_SUPPORTED: WBEMSTATUS = WBEMSTATUS(-2147205107i32);
pub const WBEMMOF_E_UNEXPECTED_ALIAS: WBEMSTATUS = WBEMSTATUS(-2147205106i32);
pub const WBEMMOF_E_UNEXPECTED_ARRAY_INIT: WBEMSTATUS = WBEMSTATUS(-2147205105i32);
pub const WBEMMOF_E_INVALID_AMENDMENT_SYNTAX: WBEMSTATUS = WBEMSTATUS(-2147205104i32);
pub const WBEMMOF_E_INVALID_DUPLICATE_AMENDMENT: WBEMSTATUS = WBEMSTATUS(-2147205103i32);
pub const WBEMMOF_E_INVALID_PRAGMA: WBEMSTATUS = WBEMSTATUS(-2147205102i32);
pub const WBEMMOF_E_INVALID_NAMESPACE_SYNTAX: WBEMSTATUS = WBEMSTATUS(-2147205101i32);
pub const WBEMMOF_E_EXPECTED_CLASS_NAME: WBEMSTATUS = WBEMSTATUS(-2147205100i32);
pub const WBEMMOF_E_TYPE_MISMATCH: WBEMSTATUS = WBEMSTATUS(-2147205099i32);
pub const WBEMMOF_E_EXPECTED_ALIAS_NAME: WBEMSTATUS = WBEMSTATUS(-2147205098i32);
pub const WBEMMOF_E_INVALID_CLASS_DECLARATION: WBEMSTATUS = WBEMSTATUS(-2147205097i32);
pub const WBEMMOF_E_INVALID_INSTANCE_DECLARATION: WBEMSTATUS = WBEMSTATUS(-2147205096i32);
pub const WBEMMOF_E_EXPECTED_DOLLAR: WBEMSTATUS = WBEMSTATUS(-2147205095i32);
pub const WBEMMOF_E_CIMTYPE_QUALIFIER: WBEMSTATUS = WBEMSTATUS(-2147205094i32);
pub const WBEMMOF_E_DUPLICATE_PROPERTY: WBEMSTATUS = WBEMSTATUS(-2147205093i32);
pub const WBEMMOF_E_INVALID_NAMESPACE_SPECIFICATION: WBEMSTATUS = WBEMSTATUS(-2147205092i32);
pub const WBEMMOF_E_OUT_OF_RANGE: WBEMSTATUS = WBEMSTATUS(-2147205091i32);
pub const WBEMMOF_E_INVALID_FILE: WBEMSTATUS = WBEMSTATUS(-2147205090i32);
pub const WBEMMOF_E_ALIASES_IN_EMBEDDED: WBEMSTATUS = WBEMSTATUS(-2147205089i32);
pub const WBEMMOF_E_NULL_ARRAY_ELEM: WBEMSTATUS = WBEMSTATUS(-2147205088i32);
pub const WBEMMOF_E_DUPLICATE_QUALIFIER: WBEMSTATUS = WBEMSTATUS(-2147205087i32);
pub const WBEMMOF_E_EXPECTED_FLAVOR_TYPE: WBEMSTATUS = WBEMSTATUS(-2147205086i32);
pub const WBEMMOF_E_INCOMPATIBLE_FLAVOR_TYPES: WBEMSTATUS = WBEMSTATUS(-2147205085i32);
pub const WBEMMOF_E_MULTIPLE_ALIASES: WBEMSTATUS = WBEMSTATUS(-2147205084i32);
pub const WBEMMOF_E_INCOMPATIBLE_FLAVOR_TYPES2: WBEMSTATUS = WBEMSTATUS(-2147205083i32);
pub const WBEMMOF_E_NO_ARRAYS_RETURNED: WBEMSTATUS = WBEMSTATUS(-2147205082i32);
pub const WBEMMOF_E_MUST_BE_IN_OR_OUT: WBEMSTATUS = WBEMSTATUS(-2147205081i32);
pub const WBEMMOF_E_INVALID_FLAGS_SYNTAX: WBEMSTATUS = WBEMSTATUS(-2147205080i32);
pub const WBEMMOF_E_EXPECTED_BRACE_OR_BAD_TYPE: WBEMSTATUS = WBEMSTATUS(-2147205079i32);
pub const WBEMMOF_E_UNSUPPORTED_CIMV22_QUAL_VALUE: WBEMSTATUS = WBEMSTATUS(-2147205078i32);
pub const WBEMMOF_E_UNSUPPORTED_CIMV22_DATA_TYPE: WBEMSTATUS = WBEMSTATUS(-2147205077i32);
pub const WBEMMOF_E_INVALID_DELETEINSTANCE_SYNTAX: WBEMSTATUS = WBEMSTATUS(-2147205076i32);
pub const WBEMMOF_E_INVALID_QUALIFIER_SYNTAX: WBEMSTATUS = WBEMSTATUS(-2147205075i32);
pub const WBEMMOF_E_QUALIFIER_USED_OUTSIDE_SCOPE: WBEMSTATUS = WBEMSTATUS(-2147205074i32);
pub const WBEMMOF_E_ERROR_CREATING_TEMP_FILE: WBEMSTATUS = WBEMSTATUS(-2147205073i32);
pub const WBEMMOF_E_ERROR_INVALID_INCLUDE_FILE: WBEMSTATUS = WBEMSTATUS(-2147205072i32);
pub const WBEMMOF_E_INVALID_DELETECLASS_SYNTAX: WBEMSTATUS = WBEMSTATUS(-2147205071i32);
impl ::core::marker::Copy for WBEMSTATUS {}
impl ::core::clone::Clone for WBEMSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEMSTATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WBEMSTATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WBEMSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEMSTATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WBEMSTATUS_FORMAT(pub i32);
pub const WBEMSTATUS_FORMAT_NEWLINE: WBEMSTATUS_FORMAT = WBEMSTATUS_FORMAT(0i32);
pub const WBEMSTATUS_FORMAT_NO_NEWLINE: WBEMSTATUS_FORMAT = WBEMSTATUS_FORMAT(1i32);
impl ::core::marker::Copy for WBEMSTATUS_FORMAT {}
impl ::core::clone::Clone for WBEMSTATUS_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEMSTATUS_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WBEMSTATUS_FORMAT {
    type Abi = Self;
}
impl ::core::fmt::Debug for WBEMSTATUS_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEMSTATUS_FORMAT").field(&self.0).finish()
    }
}
pub const WBEMS_DISPID_COMPLETED: u32 = 2u32;
pub const WBEMS_DISPID_CONNECTION_READY: u32 = 5u32;
pub const WBEMS_DISPID_DERIVATION: u32 = 23u32;
pub const WBEMS_DISPID_OBJECT_PUT: u32 = 4u32;
pub const WBEMS_DISPID_OBJECT_READY: u32 = 1u32;
pub const WBEMS_DISPID_PROGRESS: u32 = 3u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WBEM_BACKUP_RESTORE_FLAGS(pub i32);
pub const WBEM_FLAG_BACKUP_RESTORE_DEFAULT: WBEM_BACKUP_RESTORE_FLAGS = WBEM_BACKUP_RESTORE_FLAGS(0i32);
pub const WBEM_FLAG_BACKUP_RESTORE_FORCE_SHUTDOWN: WBEM_BACKUP_RESTORE_FLAGS = WBEM_BACKUP_RESTORE_FLAGS(1i32);
impl ::core::marker::Copy for WBEM_BACKUP_RESTORE_FLAGS {}
impl ::core::clone::Clone for WBEM_BACKUP_RESTORE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_BACKUP_RESTORE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WBEM_BACKUP_RESTORE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WBEM_BACKUP_RESTORE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_BACKUP_RESTORE_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WBEM_BATCH_TYPE(pub i32);
pub const WBEM_FLAG_BATCH_IF_NEEDED: WBEM_BATCH_TYPE = WBEM_BATCH_TYPE(0i32);
pub const WBEM_FLAG_MUST_BATCH: WBEM_BATCH_TYPE = WBEM_BATCH_TYPE(1i32);
pub const WBEM_FLAG_MUST_NOT_BATCH: WBEM_BATCH_TYPE = WBEM_BATCH_TYPE(2i32);
impl ::core::marker::Copy for WBEM_BATCH_TYPE {}
impl ::core::clone::Clone for WBEM_BATCH_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_BATCH_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WBEM_BATCH_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WBEM_BATCH_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_BATCH_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WBEM_CHANGE_FLAG_TYPE(pub i32);
pub const WBEM_FLAG_CREATE_OR_UPDATE: WBEM_CHANGE_FLAG_TYPE = WBEM_CHANGE_FLAG_TYPE(0i32);
pub const WBEM_FLAG_UPDATE_ONLY: WBEM_CHANGE_FLAG_TYPE = WBEM_CHANGE_FLAG_TYPE(1i32);
pub const WBEM_FLAG_CREATE_ONLY: WBEM_CHANGE_FLAG_TYPE = WBEM_CHANGE_FLAG_TYPE(2i32);
pub const WBEM_FLAG_UPDATE_COMPATIBLE: WBEM_CHANGE_FLAG_TYPE = WBEM_CHANGE_FLAG_TYPE(0i32);
pub const WBEM_FLAG_UPDATE_SAFE_MODE: WBEM_CHANGE_FLAG_TYPE = WBEM_CHANGE_FLAG_TYPE(32i32);
pub const WBEM_FLAG_UPDATE_FORCE_MODE: WBEM_CHANGE_FLAG_TYPE = WBEM_CHANGE_FLAG_TYPE(64i32);
pub const WBEM_MASK_UPDATE_MODE: WBEM_CHANGE_FLAG_TYPE = WBEM_CHANGE_FLAG_TYPE(96i32);
pub const WBEM_FLAG_ADVISORY: WBEM_CHANGE_FLAG_TYPE = WBEM_CHANGE_FLAG_TYPE(65536i32);
impl ::core::marker::Copy for WBEM_CHANGE_FLAG_TYPE {}
impl ::core::clone::Clone for WBEM_CHANGE_FLAG_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_CHANGE_FLAG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WBEM_CHANGE_FLAG_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WBEM_CHANGE_FLAG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_CHANGE_FLAG_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WBEM_COMPARISON_FLAG(pub i32);
pub const WBEM_COMPARISON_INCLUDE_ALL: WBEM_COMPARISON_FLAG = WBEM_COMPARISON_FLAG(0i32);
pub const WBEM_FLAG_IGNORE_QUALIFIERS: WBEM_COMPARISON_FLAG = WBEM_COMPARISON_FLAG(1i32);
pub const WBEM_FLAG_IGNORE_OBJECT_SOURCE: WBEM_COMPARISON_FLAG = WBEM_COMPARISON_FLAG(2i32);
pub const WBEM_FLAG_IGNORE_DEFAULT_VALUES: WBEM_COMPARISON_FLAG = WBEM_COMPARISON_FLAG(4i32);
pub const WBEM_FLAG_IGNORE_CLASS: WBEM_COMPARISON_FLAG = WBEM_COMPARISON_FLAG(8i32);
pub const WBEM_FLAG_IGNORE_CASE: WBEM_COMPARISON_FLAG = WBEM_COMPARISON_FLAG(16i32);
pub const WBEM_FLAG_IGNORE_FLAVOR: WBEM_COMPARISON_FLAG = WBEM_COMPARISON_FLAG(32i32);
impl ::core::marker::Copy for WBEM_COMPARISON_FLAG {}
impl ::core::clone::Clone for WBEM_COMPARISON_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_COMPARISON_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WBEM_COMPARISON_FLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for WBEM_COMPARISON_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_COMPARISON_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WBEM_COMPILER_OPTIONS(pub i32);
pub const WBEM_FLAG_CHECK_ONLY: WBEM_COMPILER_OPTIONS = WBEM_COMPILER_OPTIONS(1i32);
pub const WBEM_FLAG_AUTORECOVER: WBEM_COMPILER_OPTIONS = WBEM_COMPILER_OPTIONS(2i32);
pub const WBEM_FLAG_WMI_CHECK: WBEM_COMPILER_OPTIONS = WBEM_COMPILER_OPTIONS(4i32);
pub const WBEM_FLAG_CONSOLE_PRINT: WBEM_COMPILER_OPTIONS = WBEM_COMPILER_OPTIONS(8i32);
pub const WBEM_FLAG_DONT_ADD_TO_LIST: WBEM_COMPILER_OPTIONS = WBEM_COMPILER_OPTIONS(16i32);
pub const WBEM_FLAG_SPLIT_FILES: WBEM_COMPILER_OPTIONS = WBEM_COMPILER_OPTIONS(32i32);
pub const WBEM_FLAG_STORE_FILE: WBEM_COMPILER_OPTIONS = WBEM_COMPILER_OPTIONS(256i32);
impl ::core::marker::Copy for WBEM_COMPILER_OPTIONS {}
impl ::core::clone::Clone for WBEM_COMPILER_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_COMPILER_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WBEM_COMPILER_OPTIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WBEM_COMPILER_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_COMPILER_OPTIONS").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct WBEM_COMPILE_STATUS_INFO {
    pub lPhaseError: i32,
    pub hRes: ::windows_core::HRESULT,
    pub ObjectNum: i32,
    pub FirstLine: i32,
    pub LastLine: i32,
    pub dwOutFlags: u32,
}
impl ::core::marker::Copy for WBEM_COMPILE_STATUS_INFO {}
impl ::core::clone::Clone for WBEM_COMPILE_STATUS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WBEM_COMPILE_STATUS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WBEM_COMPILE_STATUS_INFO").field("lPhaseError", &self.lPhaseError).field("hRes", &self.hRes).field("ObjectNum", &self.ObjectNum).field("FirstLine", &self.FirstLine).field("LastLine", &self.LastLine).field("dwOutFlags", &self.dwOutFlags).finish()
    }
}
unsafe impl ::windows_core::Abi for WBEM_COMPILE_STATUS_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WBEM_COMPILE_STATUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WBEM_COMPILE_STATUS_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WBEM_COMPILE_STATUS_INFO {}
impl ::core::default::Default for WBEM_COMPILE_STATUS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WBEM_CONDITION_FLAG_TYPE(pub i32);
pub const WBEM_FLAG_ALWAYS: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(0i32);
pub const WBEM_FLAG_ONLY_IF_TRUE: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(1i32);
pub const WBEM_FLAG_ONLY_IF_FALSE: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(2i32);
pub const WBEM_FLAG_ONLY_IF_IDENTICAL: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(3i32);
pub const WBEM_MASK_PRIMARY_CONDITION: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(3i32);
pub const WBEM_FLAG_KEYS_ONLY: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(4i32);
pub const WBEM_FLAG_REFS_ONLY: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(8i32);
pub const WBEM_FLAG_LOCAL_ONLY: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(16i32);
pub const WBEM_FLAG_PROPAGATED_ONLY: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(32i32);
pub const WBEM_FLAG_SYSTEM_ONLY: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(48i32);
pub const WBEM_FLAG_NONSYSTEM_ONLY: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(64i32);
pub const WBEM_MASK_CONDITION_ORIGIN: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(112i32);
pub const WBEM_FLAG_CLASS_OVERRIDES_ONLY: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(256i32);
pub const WBEM_FLAG_CLASS_LOCAL_AND_OVERRIDES: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(512i32);
pub const WBEM_MASK_CLASS_CONDITION: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(768i32);
impl ::core::marker::Copy for WBEM_CONDITION_FLAG_TYPE {}
impl ::core::clone::Clone for WBEM_CONDITION_FLAG_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_CONDITION_FLAG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WBEM_CONDITION_FLAG_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WBEM_CONDITION_FLAG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_CONDITION_FLAG_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WBEM_CONNECT_OPTIONS(pub i32);
pub const WBEM_FLAG_CONNECT_REPOSITORY_ONLY: WBEM_CONNECT_OPTIONS = WBEM_CONNECT_OPTIONS(64i32);
pub const WBEM_FLAG_CONNECT_USE_MAX_WAIT: WBEM_CONNECT_OPTIONS = WBEM_CONNECT_OPTIONS(128i32);
pub const WBEM_FLAG_CONNECT_PROVIDERS: WBEM_CONNECT_OPTIONS = WBEM_CONNECT_OPTIONS(256i32);
impl ::core::marker::Copy for WBEM_CONNECT_OPTIONS {}
impl ::core::clone::Clone for WBEM_CONNECT_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_CONNECT_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WBEM_CONNECT_OPTIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WBEM_CONNECT_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_CONNECT_OPTIONS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WBEM_EXTRA_RETURN_CODES(pub i32);
pub const WBEM_S_INITIALIZED: WBEM_EXTRA_RETURN_CODES = WBEM_EXTRA_RETURN_CODES(0i32);
pub const WBEM_S_LIMITED_SERVICE: WBEM_EXTRA_RETURN_CODES = WBEM_EXTRA_RETURN_CODES(274433i32);
pub const WBEM_S_INDIRECTLY_UPDATED: WBEM_EXTRA_RETURN_CODES = WBEM_EXTRA_RETURN_CODES(274434i32);
pub const WBEM_S_SUBJECT_TO_SDS: WBEM_EXTRA_RETURN_CODES = WBEM_EXTRA_RETURN_CODES(274435i32);
pub const WBEM_E_RETRY_LATER: WBEM_EXTRA_RETURN_CODES = WBEM_EXTRA_RETURN_CODES(-2147209215i32);
pub const WBEM_E_RESOURCE_CONTENTION: WBEM_EXTRA_RETURN_CODES = WBEM_EXTRA_RETURN_CODES(-2147209214i32);
impl ::core::marker::Copy for WBEM_EXTRA_RETURN_CODES {}
impl ::core::clone::Clone for WBEM_EXTRA_RETURN_CODES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_EXTRA_RETURN_CODES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WBEM_EXTRA_RETURN_CODES {
    type Abi = Self;
}
impl ::core::fmt::Debug for WBEM_EXTRA_RETURN_CODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_EXTRA_RETURN_CODES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WBEM_FLAVOR_TYPE(pub i32);
pub const WBEM_FLAVOR_DONT_PROPAGATE: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(0i32);
pub const WBEM_FLAVOR_FLAG_PROPAGATE_TO_INSTANCE: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(1i32);
pub const WBEM_FLAVOR_FLAG_PROPAGATE_TO_DERIVED_CLASS: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(2i32);
pub const WBEM_FLAVOR_MASK_PROPAGATION: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(15i32);
pub const WBEM_FLAVOR_OVERRIDABLE: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(0i32);
pub const WBEM_FLAVOR_NOT_OVERRIDABLE: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(16i32);
pub const WBEM_FLAVOR_MASK_PERMISSIONS: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(16i32);
pub const WBEM_FLAVOR_ORIGIN_LOCAL: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(0i32);
pub const WBEM_FLAVOR_ORIGIN_PROPAGATED: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(32i32);
pub const WBEM_FLAVOR_ORIGIN_SYSTEM: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(64i32);
pub const WBEM_FLAVOR_MASK_ORIGIN: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(96i32);
pub const WBEM_FLAVOR_NOT_AMENDED: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(0i32);
pub const WBEM_FLAVOR_AMENDED: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(128i32);
pub const WBEM_FLAVOR_MASK_AMENDED: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(128i32);
impl ::core::marker::Copy for WBEM_FLAVOR_TYPE {}
impl ::core::clone::Clone for WBEM_FLAVOR_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_FLAVOR_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WBEM_FLAVOR_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WBEM_FLAVOR_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_FLAVOR_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WBEM_GENERIC_FLAG_TYPE(pub i32);
pub const WBEM_FLAG_RETURN_IMMEDIATELY: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(16i32);
pub const WBEM_FLAG_RETURN_WBEM_COMPLETE: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(0i32);
pub const WBEM_FLAG_BIDIRECTIONAL: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(0i32);
pub const WBEM_FLAG_FORWARD_ONLY: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(32i32);
pub const WBEM_FLAG_NO_ERROR_OBJECT: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(64i32);
pub const WBEM_FLAG_RETURN_ERROR_OBJECT: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(0i32);
pub const WBEM_FLAG_SEND_STATUS: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(128i32);
pub const WBEM_FLAG_DONT_SEND_STATUS: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(0i32);
pub const WBEM_FLAG_ENSURE_LOCATABLE: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(256i32);
pub const WBEM_FLAG_DIRECT_READ: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(512i32);
pub const WBEM_FLAG_SEND_ONLY_SELECTED: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(0i32);
pub const WBEM_RETURN_WHEN_COMPLETE: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(0i32);
pub const WBEM_RETURN_IMMEDIATELY: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(16i32);
pub const WBEM_MASK_RESERVED_FLAGS: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(126976i32);
pub const WBEM_FLAG_USE_AMENDED_QUALIFIERS: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(131072i32);
pub const WBEM_FLAG_STRONG_VALIDATION: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(1048576i32);
impl ::core::marker::Copy for WBEM_GENERIC_FLAG_TYPE {}
impl ::core::clone::Clone for WBEM_GENERIC_FLAG_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_GENERIC_FLAG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WBEM_GENERIC_FLAG_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WBEM_GENERIC_FLAG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_GENERIC_FLAG_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WBEM_GENUS_TYPE(pub i32);
pub const WBEM_GENUS_CLASS: WBEM_GENUS_TYPE = WBEM_GENUS_TYPE(1i32);
pub const WBEM_GENUS_INSTANCE: WBEM_GENUS_TYPE = WBEM_GENUS_TYPE(2i32);
impl ::core::marker::Copy for WBEM_GENUS_TYPE {}
impl ::core::clone::Clone for WBEM_GENUS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_GENUS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WBEM_GENUS_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WBEM_GENUS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_GENUS_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WBEM_GET_KEY_FLAGS(pub i32);
pub const WBEMPATH_TEXT: WBEM_GET_KEY_FLAGS = WBEM_GET_KEY_FLAGS(1i32);
pub const WBEMPATH_QUOTEDTEXT: WBEM_GET_KEY_FLAGS = WBEM_GET_KEY_FLAGS(2i32);
impl ::core::marker::Copy for WBEM_GET_KEY_FLAGS {}
impl ::core::clone::Clone for WBEM_GET_KEY_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_GET_KEY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WBEM_GET_KEY_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WBEM_GET_KEY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_GET_KEY_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WBEM_GET_TEXT_FLAGS(pub i32);
pub const WBEMPATH_COMPRESSED: WBEM_GET_TEXT_FLAGS = WBEM_GET_TEXT_FLAGS(1i32);
pub const WBEMPATH_GET_RELATIVE_ONLY: WBEM_GET_TEXT_FLAGS = WBEM_GET_TEXT_FLAGS(2i32);
pub const WBEMPATH_GET_SERVER_TOO: WBEM_GET_TEXT_FLAGS = WBEM_GET_TEXT_FLAGS(4i32);
pub const WBEMPATH_GET_SERVER_AND_NAMESPACE_ONLY: WBEM_GET_TEXT_FLAGS = WBEM_GET_TEXT_FLAGS(8i32);
pub const WBEMPATH_GET_NAMESPACE_ONLY: WBEM_GET_TEXT_FLAGS = WBEM_GET_TEXT_FLAGS(16i32);
pub const WBEMPATH_GET_ORIGINAL: WBEM_GET_TEXT_FLAGS = WBEM_GET_TEXT_FLAGS(32i32);
impl ::core::marker::Copy for WBEM_GET_TEXT_FLAGS {}
impl ::core::clone::Clone for WBEM_GET_TEXT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_GET_TEXT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WBEM_GET_TEXT_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WBEM_GET_TEXT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_GET_TEXT_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WBEM_INFORMATION_FLAG_TYPE(pub i32);
pub const WBEM_FLAG_SHORT_NAME: WBEM_INFORMATION_FLAG_TYPE = WBEM_INFORMATION_FLAG_TYPE(1i32);
pub const WBEM_FLAG_LONG_NAME: WBEM_INFORMATION_FLAG_TYPE = WBEM_INFORMATION_FLAG_TYPE(2i32);
impl ::core::marker::Copy for WBEM_INFORMATION_FLAG_TYPE {}
impl ::core::clone::Clone for WBEM_INFORMATION_FLAG_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_INFORMATION_FLAG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WBEM_INFORMATION_FLAG_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WBEM_INFORMATION_FLAG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_INFORMATION_FLAG_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WBEM_LIMITATION_FLAG_TYPE(pub i32);
pub const WBEM_FLAG_EXCLUDE_OBJECT_QUALIFIERS: WBEM_LIMITATION_FLAG_TYPE = WBEM_LIMITATION_FLAG_TYPE(16i32);
pub const WBEM_FLAG_EXCLUDE_PROPERTY_QUALIFIERS: WBEM_LIMITATION_FLAG_TYPE = WBEM_LIMITATION_FLAG_TYPE(32i32);
impl ::core::marker::Copy for WBEM_LIMITATION_FLAG_TYPE {}
impl ::core::clone::Clone for WBEM_LIMITATION_FLAG_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_LIMITATION_FLAG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WBEM_LIMITATION_FLAG_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WBEM_LIMITATION_FLAG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_LIMITATION_FLAG_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WBEM_LIMITS(pub i32);
pub const WBEM_MAX_IDENTIFIER: WBEM_LIMITS = WBEM_LIMITS(4096i32);
pub const WBEM_MAX_QUERY: WBEM_LIMITS = WBEM_LIMITS(16384i32);
pub const WBEM_MAX_PATH: WBEM_LIMITS = WBEM_LIMITS(8192i32);
pub const WBEM_MAX_OBJECT_NESTING: WBEM_LIMITS = WBEM_LIMITS(64i32);
pub const WBEM_MAX_USER_PROPERTIES: WBEM_LIMITS = WBEM_LIMITS(1024i32);
impl ::core::marker::Copy for WBEM_LIMITS {}
impl ::core::clone::Clone for WBEM_LIMITS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_LIMITS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WBEM_LIMITS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WBEM_LIMITS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_LIMITS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WBEM_LOCKING(pub i32);
pub const WBEM_FLAG_ALLOW_READ: WBEM_LOCKING = WBEM_LOCKING(1i32);
impl ::core::marker::Copy for WBEM_LOCKING {}
impl ::core::clone::Clone for WBEM_LOCKING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_LOCKING {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WBEM_LOCKING {
    type Abi = Self;
}
impl ::core::fmt::Debug for WBEM_LOCKING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_LOCKING").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WBEM_PATH_CREATE_FLAG(pub i32);
pub const WBEMPATH_CREATE_ACCEPT_RELATIVE: WBEM_PATH_CREATE_FLAG = WBEM_PATH_CREATE_FLAG(1i32);
pub const WBEMPATH_CREATE_ACCEPT_ABSOLUTE: WBEM_PATH_CREATE_FLAG = WBEM_PATH_CREATE_FLAG(2i32);
pub const WBEMPATH_CREATE_ACCEPT_ALL: WBEM_PATH_CREATE_FLAG = WBEM_PATH_CREATE_FLAG(4i32);
pub const WBEMPATH_TREAT_SINGLE_IDENT_AS_NS: WBEM_PATH_CREATE_FLAG = WBEM_PATH_CREATE_FLAG(8i32);
impl ::core::marker::Copy for WBEM_PATH_CREATE_FLAG {}
impl ::core::clone::Clone for WBEM_PATH_CREATE_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_PATH_CREATE_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WBEM_PATH_CREATE_FLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for WBEM_PATH_CREATE_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_PATH_CREATE_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WBEM_PATH_STATUS_FLAG(pub i32);
pub const WBEMPATH_INFO_ANON_LOCAL_MACHINE: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(1i32);
pub const WBEMPATH_INFO_HAS_MACHINE_NAME: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(2i32);
pub const WBEMPATH_INFO_IS_CLASS_REF: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(4i32);
pub const WBEMPATH_INFO_IS_INST_REF: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(8i32);
pub const WBEMPATH_INFO_HAS_SUBSCOPES: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(16i32);
pub const WBEMPATH_INFO_IS_COMPOUND: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(32i32);
pub const WBEMPATH_INFO_HAS_V2_REF_PATHS: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(64i32);
pub const WBEMPATH_INFO_HAS_IMPLIED_KEY: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(128i32);
pub const WBEMPATH_INFO_CONTAINS_SINGLETON: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(256i32);
pub const WBEMPATH_INFO_V1_COMPLIANT: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(512i32);
pub const WBEMPATH_INFO_V2_COMPLIANT: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(1024i32);
pub const WBEMPATH_INFO_CIM_COMPLIANT: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(2048i32);
pub const WBEMPATH_INFO_IS_SINGLETON: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(4096i32);
pub const WBEMPATH_INFO_IS_PARENT: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(8192i32);
pub const WBEMPATH_INFO_SERVER_NAMESPACE_ONLY: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(16384i32);
pub const WBEMPATH_INFO_NATIVE_PATH: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(32768i32);
pub const WBEMPATH_INFO_WMI_PATH: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(65536i32);
pub const WBEMPATH_INFO_PATH_HAD_SERVER: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(131072i32);
impl ::core::marker::Copy for WBEM_PATH_STATUS_FLAG {}
impl ::core::clone::Clone for WBEM_PATH_STATUS_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_PATH_STATUS_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WBEM_PATH_STATUS_FLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for WBEM_PATH_STATUS_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_PATH_STATUS_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WBEM_PROVIDER_FLAGS(pub i32);
pub const WBEM_FLAG_OWNER_UPDATE: WBEM_PROVIDER_FLAGS = WBEM_PROVIDER_FLAGS(65536i32);
impl ::core::marker::Copy for WBEM_PROVIDER_FLAGS {}
impl ::core::clone::Clone for WBEM_PROVIDER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_PROVIDER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WBEM_PROVIDER_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WBEM_PROVIDER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_PROVIDER_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WBEM_PROVIDER_REQUIREMENTS_TYPE(pub i32);
pub const WBEM_REQUIREMENTS_START_POSTFILTER: WBEM_PROVIDER_REQUIREMENTS_TYPE = WBEM_PROVIDER_REQUIREMENTS_TYPE(0i32);
pub const WBEM_REQUIREMENTS_STOP_POSTFILTER: WBEM_PROVIDER_REQUIREMENTS_TYPE = WBEM_PROVIDER_REQUIREMENTS_TYPE(1i32);
pub const WBEM_REQUIREMENTS_RECHECK_SUBSCRIPTIONS: WBEM_PROVIDER_REQUIREMENTS_TYPE = WBEM_PROVIDER_REQUIREMENTS_TYPE(2i32);
impl ::core::marker::Copy for WBEM_PROVIDER_REQUIREMENTS_TYPE {}
impl ::core::clone::Clone for WBEM_PROVIDER_REQUIREMENTS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_PROVIDER_REQUIREMENTS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WBEM_PROVIDER_REQUIREMENTS_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WBEM_PROVIDER_REQUIREMENTS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_PROVIDER_REQUIREMENTS_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WBEM_QUERY_FLAG_TYPE(pub i32);
pub const WBEM_FLAG_DEEP: WBEM_QUERY_FLAG_TYPE = WBEM_QUERY_FLAG_TYPE(0i32);
pub const WBEM_FLAG_SHALLOW: WBEM_QUERY_FLAG_TYPE = WBEM_QUERY_FLAG_TYPE(1i32);
pub const WBEM_FLAG_PROTOTYPE: WBEM_QUERY_FLAG_TYPE = WBEM_QUERY_FLAG_TYPE(2i32);
impl ::core::marker::Copy for WBEM_QUERY_FLAG_TYPE {}
impl ::core::clone::Clone for WBEM_QUERY_FLAG_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_QUERY_FLAG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WBEM_QUERY_FLAG_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WBEM_QUERY_FLAG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_QUERY_FLAG_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WBEM_REFRESHER_FLAGS(pub i32);
pub const WBEM_FLAG_REFRESH_AUTO_RECONNECT: WBEM_REFRESHER_FLAGS = WBEM_REFRESHER_FLAGS(0i32);
pub const WBEM_FLAG_REFRESH_NO_AUTO_RECONNECT: WBEM_REFRESHER_FLAGS = WBEM_REFRESHER_FLAGS(1i32);
impl ::core::marker::Copy for WBEM_REFRESHER_FLAGS {}
impl ::core::clone::Clone for WBEM_REFRESHER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_REFRESHER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WBEM_REFRESHER_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WBEM_REFRESHER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_REFRESHER_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WBEM_SECURITY_FLAGS(pub i32);
pub const WBEM_ENABLE: WBEM_SECURITY_FLAGS = WBEM_SECURITY_FLAGS(1i32);
pub const WBEM_METHOD_EXECUTE: WBEM_SECURITY_FLAGS = WBEM_SECURITY_FLAGS(2i32);
pub const WBEM_FULL_WRITE_REP: WBEM_SECURITY_FLAGS = WBEM_SECURITY_FLAGS(4i32);
pub const WBEM_PARTIAL_WRITE_REP: WBEM_SECURITY_FLAGS = WBEM_SECURITY_FLAGS(8i32);
pub const WBEM_WRITE_PROVIDER: WBEM_SECURITY_FLAGS = WBEM_SECURITY_FLAGS(16i32);
pub const WBEM_REMOTE_ACCESS: WBEM_SECURITY_FLAGS = WBEM_SECURITY_FLAGS(32i32);
pub const WBEM_RIGHT_SUBSCRIBE: WBEM_SECURITY_FLAGS = WBEM_SECURITY_FLAGS(64i32);
pub const WBEM_RIGHT_PUBLISH: WBEM_SECURITY_FLAGS = WBEM_SECURITY_FLAGS(128i32);
impl ::core::marker::Copy for WBEM_SECURITY_FLAGS {}
impl ::core::clone::Clone for WBEM_SECURITY_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_SECURITY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WBEM_SECURITY_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WBEM_SECURITY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_SECURITY_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WBEM_SHUTDOWN_FLAGS(pub i32);
pub const WBEM_SHUTDOWN_UNLOAD_COMPONENT: WBEM_SHUTDOWN_FLAGS = WBEM_SHUTDOWN_FLAGS(1i32);
pub const WBEM_SHUTDOWN_WMI: WBEM_SHUTDOWN_FLAGS = WBEM_SHUTDOWN_FLAGS(2i32);
pub const WBEM_SHUTDOWN_OS: WBEM_SHUTDOWN_FLAGS = WBEM_SHUTDOWN_FLAGS(3i32);
impl ::core::marker::Copy for WBEM_SHUTDOWN_FLAGS {}
impl ::core::clone::Clone for WBEM_SHUTDOWN_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_SHUTDOWN_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WBEM_SHUTDOWN_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WBEM_SHUTDOWN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_SHUTDOWN_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WBEM_STATUS_TYPE(pub i32);
pub const WBEM_STATUS_COMPLETE: WBEM_STATUS_TYPE = WBEM_STATUS_TYPE(0i32);
pub const WBEM_STATUS_REQUIREMENTS: WBEM_STATUS_TYPE = WBEM_STATUS_TYPE(1i32);
pub const WBEM_STATUS_PROGRESS: WBEM_STATUS_TYPE = WBEM_STATUS_TYPE(2i32);
pub const WBEM_STATUS_LOGGING_INFORMATION: WBEM_STATUS_TYPE = WBEM_STATUS_TYPE(256i32);
pub const WBEM_STATUS_LOGGING_INFORMATION_PROVIDER: WBEM_STATUS_TYPE = WBEM_STATUS_TYPE(512i32);
pub const WBEM_STATUS_LOGGING_INFORMATION_HOST: WBEM_STATUS_TYPE = WBEM_STATUS_TYPE(1024i32);
pub const WBEM_STATUS_LOGGING_INFORMATION_REPOSITORY: WBEM_STATUS_TYPE = WBEM_STATUS_TYPE(2048i32);
pub const WBEM_STATUS_LOGGING_INFORMATION_ESS: WBEM_STATUS_TYPE = WBEM_STATUS_TYPE(4096i32);
impl ::core::marker::Copy for WBEM_STATUS_TYPE {}
impl ::core::clone::Clone for WBEM_STATUS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_STATUS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WBEM_STATUS_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WBEM_STATUS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_STATUS_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WBEM_TEXT_FLAG_TYPE(pub i32);
pub const WBEM_FLAG_NO_FLAVORS: WBEM_TEXT_FLAG_TYPE = WBEM_TEXT_FLAG_TYPE(1i32);
impl ::core::marker::Copy for WBEM_TEXT_FLAG_TYPE {}
impl ::core::clone::Clone for WBEM_TEXT_FLAG_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_TEXT_FLAG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WBEM_TEXT_FLAG_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WBEM_TEXT_FLAG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_TEXT_FLAG_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WBEM_TIMEOUT_TYPE(pub i32);
pub const WBEM_NO_WAIT: WBEM_TIMEOUT_TYPE = WBEM_TIMEOUT_TYPE(0i32);
pub const WBEM_INFINITE: WBEM_TIMEOUT_TYPE = WBEM_TIMEOUT_TYPE(-1i32);
impl ::core::marker::Copy for WBEM_TIMEOUT_TYPE {}
impl ::core::clone::Clone for WBEM_TIMEOUT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_TIMEOUT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WBEM_TIMEOUT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WBEM_TIMEOUT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_TIMEOUT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WBEM_UNSECAPP_FLAG_TYPE(pub i32);
pub const WBEM_FLAG_UNSECAPP_DEFAULT_CHECK_ACCESS: WBEM_UNSECAPP_FLAG_TYPE = WBEM_UNSECAPP_FLAG_TYPE(0i32);
pub const WBEM_FLAG_UNSECAPP_CHECK_ACCESS: WBEM_UNSECAPP_FLAG_TYPE = WBEM_UNSECAPP_FLAG_TYPE(1i32);
pub const WBEM_FLAG_UNSECAPP_DONT_CHECK_ACCESS: WBEM_UNSECAPP_FLAG_TYPE = WBEM_UNSECAPP_FLAG_TYPE(2i32);
impl ::core::marker::Copy for WBEM_UNSECAPP_FLAG_TYPE {}
impl ::core::clone::Clone for WBEM_UNSECAPP_FLAG_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_UNSECAPP_FLAG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WBEM_UNSECAPP_FLAG_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WBEM_UNSECAPP_FLAG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_UNSECAPP_FLAG_TYPE").field(&self.0).finish()
    }
}
pub const WMIExtension: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf0975afe_5c7f_11d2_8b74_00104b2afb41);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMIQ_ANALYSIS_TYPE(pub i32);
pub const WMIQ_ANALYSIS_RPN_SEQUENCE: WMIQ_ANALYSIS_TYPE = WMIQ_ANALYSIS_TYPE(1i32);
pub const WMIQ_ANALYSIS_ASSOC_QUERY: WMIQ_ANALYSIS_TYPE = WMIQ_ANALYSIS_TYPE(2i32);
pub const WMIQ_ANALYSIS_PROP_ANALYSIS_MATRIX: WMIQ_ANALYSIS_TYPE = WMIQ_ANALYSIS_TYPE(3i32);
pub const WMIQ_ANALYSIS_QUERY_TEXT: WMIQ_ANALYSIS_TYPE = WMIQ_ANALYSIS_TYPE(4i32);
pub const WMIQ_ANALYSIS_RESERVED: WMIQ_ANALYSIS_TYPE = WMIQ_ANALYSIS_TYPE(134217728i32);
impl ::core::marker::Copy for WMIQ_ANALYSIS_TYPE {}
impl ::core::clone::Clone for WMIQ_ANALYSIS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMIQ_ANALYSIS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WMIQ_ANALYSIS_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMIQ_ANALYSIS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMIQ_ANALYSIS_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMIQ_ASSOCQ_FLAGS(pub i32);
pub const WMIQ_ASSOCQ_ASSOCIATORS: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(1i32);
pub const WMIQ_ASSOCQ_REFERENCES: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(2i32);
pub const WMIQ_ASSOCQ_RESULTCLASS: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(4i32);
pub const WMIQ_ASSOCQ_ASSOCCLASS: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(8i32);
pub const WMIQ_ASSOCQ_ROLE: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(16i32);
pub const WMIQ_ASSOCQ_RESULTROLE: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(32i32);
pub const WMIQ_ASSOCQ_REQUIREDQUALIFIER: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(64i32);
pub const WMIQ_ASSOCQ_REQUIREDASSOCQUALIFIER: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(128i32);
pub const WMIQ_ASSOCQ_CLASSDEFSONLY: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(256i32);
pub const WMIQ_ASSOCQ_KEYSONLY: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(512i32);
pub const WMIQ_ASSOCQ_SCHEMAONLY: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(1024i32);
pub const WMIQ_ASSOCQ_CLASSREFSONLY: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(2048i32);
impl ::core::marker::Copy for WMIQ_ASSOCQ_FLAGS {}
impl ::core::clone::Clone for WMIQ_ASSOCQ_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMIQ_ASSOCQ_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WMIQ_ASSOCQ_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMIQ_ASSOCQ_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMIQ_ASSOCQ_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMIQ_LANGUAGE_FEATURES(pub i32);
pub const WMIQ_LF1_BASIC_SELECT: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(1i32);
pub const WMIQ_LF2_CLASS_NAME_IN_QUERY: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(2i32);
pub const WMIQ_LF3_STRING_CASE_FUNCTIONS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(3i32);
pub const WMIQ_LF4_PROP_TO_PROP_TESTS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(4i32);
pub const WMIQ_LF5_COUNT_STAR: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(5i32);
pub const WMIQ_LF6_ORDER_BY: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(6i32);
pub const WMIQ_LF7_DISTINCT: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(7i32);
pub const WMIQ_LF8_ISA: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(8i32);
pub const WMIQ_LF9_THIS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(9i32);
pub const WMIQ_LF10_COMPEX_SUBEXPRESSIONS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(10i32);
pub const WMIQ_LF11_ALIASING: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(11i32);
pub const WMIQ_LF12_GROUP_BY_HAVING: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(12i32);
pub const WMIQ_LF13_WMI_WITHIN: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(13i32);
pub const WMIQ_LF14_SQL_WRITE_OPERATIONS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(14i32);
pub const WMIQ_LF15_GO: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(15i32);
pub const WMIQ_LF16_SINGLE_LEVEL_TRANSACTIONS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(16i32);
pub const WMIQ_LF17_QUALIFIED_NAMES: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(17i32);
pub const WMIQ_LF18_ASSOCIATONS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(18i32);
pub const WMIQ_LF19_SYSTEM_PROPERTIES: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(19i32);
pub const WMIQ_LF20_EXTENDED_SYSTEM_PROPERTIES: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(20i32);
pub const WMIQ_LF21_SQL89_JOINS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(21i32);
pub const WMIQ_LF22_SQL92_JOINS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(22i32);
pub const WMIQ_LF23_SUBSELECTS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(23i32);
pub const WMIQ_LF24_UMI_EXTENSIONS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(24i32);
pub const WMIQ_LF25_DATEPART: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(25i32);
pub const WMIQ_LF26_LIKE: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(26i32);
pub const WMIQ_LF27_CIM_TEMPORAL_CONSTRUCTS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(27i32);
pub const WMIQ_LF28_STANDARD_AGGREGATES: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(28i32);
pub const WMIQ_LF29_MULTI_LEVEL_ORDER_BY: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(29i32);
pub const WMIQ_LF30_WMI_PRAGMAS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(30i32);
pub const WMIQ_LF31_QUALIFIER_TESTS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(31i32);
pub const WMIQ_LF32_SP_EXECUTE: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(32i32);
pub const WMIQ_LF33_ARRAY_ACCESS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(33i32);
pub const WMIQ_LF34_UNION: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(34i32);
pub const WMIQ_LF35_COMPLEX_SELECT_TARGET: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(35i32);
pub const WMIQ_LF36_REFERENCE_TESTS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(36i32);
pub const WMIQ_LF37_SELECT_INTO: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(37i32);
pub const WMIQ_LF38_BASIC_DATETIME_TESTS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(38i32);
pub const WMIQ_LF39_COUNT_COLUMN: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(39i32);
pub const WMIQ_LF40_BETWEEN: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(40i32);
pub const WMIQ_LF_LAST: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(40i32);
impl ::core::marker::Copy for WMIQ_LANGUAGE_FEATURES {}
impl ::core::clone::Clone for WMIQ_LANGUAGE_FEATURES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMIQ_LANGUAGE_FEATURES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WMIQ_LANGUAGE_FEATURES {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMIQ_LANGUAGE_FEATURES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMIQ_LANGUAGE_FEATURES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMIQ_RPNQ_FEATURE(pub i32);
pub const WMIQ_RPNF_WHERE_CLAUSE_PRESENT: WMIQ_RPNQ_FEATURE = WMIQ_RPNQ_FEATURE(1i32);
pub const WMIQ_RPNF_QUERY_IS_CONJUNCTIVE: WMIQ_RPNQ_FEATURE = WMIQ_RPNQ_FEATURE(2i32);
pub const WMIQ_RPNF_QUERY_IS_DISJUNCTIVE: WMIQ_RPNQ_FEATURE = WMIQ_RPNQ_FEATURE(4i32);
pub const WMIQ_RPNF_PROJECTION: WMIQ_RPNQ_FEATURE = WMIQ_RPNQ_FEATURE(8i32);
pub const WMIQ_RPNF_FEATURE_SELECT_STAR: WMIQ_RPNQ_FEATURE = WMIQ_RPNQ_FEATURE(16i32);
pub const WMIQ_RPNF_EQUALITY_TESTS_ONLY: WMIQ_RPNQ_FEATURE = WMIQ_RPNQ_FEATURE(32i32);
pub const WMIQ_RPNF_COUNT_STAR: WMIQ_RPNQ_FEATURE = WMIQ_RPNQ_FEATURE(64i32);
pub const WMIQ_RPNF_QUALIFIED_NAMES_USED: WMIQ_RPNQ_FEATURE = WMIQ_RPNQ_FEATURE(128i32);
pub const WMIQ_RPNF_SYSPROP_CLASS_USED: WMIQ_RPNQ_FEATURE = WMIQ_RPNQ_FEATURE(256i32);
pub const WMIQ_RPNF_PROP_TO_PROP_TESTS: WMIQ_RPNQ_FEATURE = WMIQ_RPNQ_FEATURE(512i32);
pub const WMIQ_RPNF_ORDER_BY: WMIQ_RPNQ_FEATURE = WMIQ_RPNQ_FEATURE(1024i32);
pub const WMIQ_RPNF_ISA_USED: WMIQ_RPNQ_FEATURE = WMIQ_RPNQ_FEATURE(2048i32);
pub const WMIQ_RPNF_GROUP_BY_HAVING: WMIQ_RPNQ_FEATURE = WMIQ_RPNQ_FEATURE(4096i32);
pub const WMIQ_RPNF_ARRAY_ACCESS_USED: WMIQ_RPNQ_FEATURE = WMIQ_RPNQ_FEATURE(8192i32);
impl ::core::marker::Copy for WMIQ_RPNQ_FEATURE {}
impl ::core::clone::Clone for WMIQ_RPNQ_FEATURE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMIQ_RPNQ_FEATURE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WMIQ_RPNQ_FEATURE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMIQ_RPNQ_FEATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMIQ_RPNQ_FEATURE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMIQ_RPN_TOKEN_FLAGS(pub i32);
pub const WMIQ_RPN_TOKEN_EXPRESSION: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(1i32);
pub const WMIQ_RPN_TOKEN_AND: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(2i32);
pub const WMIQ_RPN_TOKEN_OR: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(3i32);
pub const WMIQ_RPN_TOKEN_NOT: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(4i32);
pub const WMIQ_RPN_OP_UNDEFINED: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(0i32);
pub const WMIQ_RPN_OP_EQ: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(1i32);
pub const WMIQ_RPN_OP_NE: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(2i32);
pub const WMIQ_RPN_OP_GE: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(3i32);
pub const WMIQ_RPN_OP_LE: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(4i32);
pub const WMIQ_RPN_OP_LT: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(5i32);
pub const WMIQ_RPN_OP_GT: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(6i32);
pub const WMIQ_RPN_OP_LIKE: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(7i32);
pub const WMIQ_RPN_OP_ISA: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(8i32);
pub const WMIQ_RPN_OP_ISNOTA: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(9i32);
pub const WMIQ_RPN_OP_ISNULL: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(10i32);
pub const WMIQ_RPN_OP_ISNOTNULL: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(11i32);
pub const WMIQ_RPN_LEFT_PROPERTY_NAME: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(1i32);
pub const WMIQ_RPN_RIGHT_PROPERTY_NAME: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(2i32);
pub const WMIQ_RPN_CONST2: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(4i32);
pub const WMIQ_RPN_CONST: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(8i32);
pub const WMIQ_RPN_RELOP: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(16i32);
pub const WMIQ_RPN_LEFT_FUNCTION: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(32i32);
pub const WMIQ_RPN_RIGHT_FUNCTION: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(64i32);
pub const WMIQ_RPN_GET_TOKEN_TYPE: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(1i32);
pub const WMIQ_RPN_GET_EXPR_SHAPE: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(2i32);
pub const WMIQ_RPN_GET_LEFT_FUNCTION: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(3i32);
pub const WMIQ_RPN_GET_RIGHT_FUNCTION: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(4i32);
pub const WMIQ_RPN_GET_RELOP: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(5i32);
pub const WMIQ_RPN_NEXT_TOKEN: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(1i32);
pub const WMIQ_RPN_FROM_UNARY: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(1i32);
pub const WMIQ_RPN_FROM_PATH: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(2i32);
pub const WMIQ_RPN_FROM_CLASS_LIST: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(4i32);
pub const WMIQ_RPN_FROM_MULTIPLE: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(8i32);
impl ::core::marker::Copy for WMIQ_RPN_TOKEN_FLAGS {}
impl ::core::clone::Clone for WMIQ_RPN_TOKEN_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMIQ_RPN_TOKEN_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WMIQ_RPN_TOKEN_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMIQ_RPN_TOKEN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMIQ_RPN_TOKEN_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMI_OBJ_TEXT(pub i32);
pub const WMI_OBJ_TEXT_CIM_DTD_2_0: WMI_OBJ_TEXT = WMI_OBJ_TEXT(1i32);
pub const WMI_OBJ_TEXT_WMI_DTD_2_0: WMI_OBJ_TEXT = WMI_OBJ_TEXT(2i32);
pub const WMI_OBJ_TEXT_WMI_EXT1: WMI_OBJ_TEXT = WMI_OBJ_TEXT(3i32);
pub const WMI_OBJ_TEXT_WMI_EXT2: WMI_OBJ_TEXT = WMI_OBJ_TEXT(4i32);
pub const WMI_OBJ_TEXT_WMI_EXT3: WMI_OBJ_TEXT = WMI_OBJ_TEXT(5i32);
pub const WMI_OBJ_TEXT_WMI_EXT4: WMI_OBJ_TEXT = WMI_OBJ_TEXT(6i32);
pub const WMI_OBJ_TEXT_WMI_EXT5: WMI_OBJ_TEXT = WMI_OBJ_TEXT(7i32);
pub const WMI_OBJ_TEXT_WMI_EXT6: WMI_OBJ_TEXT = WMI_OBJ_TEXT(8i32);
pub const WMI_OBJ_TEXT_WMI_EXT7: WMI_OBJ_TEXT = WMI_OBJ_TEXT(9i32);
pub const WMI_OBJ_TEXT_WMI_EXT8: WMI_OBJ_TEXT = WMI_OBJ_TEXT(10i32);
pub const WMI_OBJ_TEXT_WMI_EXT9: WMI_OBJ_TEXT = WMI_OBJ_TEXT(11i32);
pub const WMI_OBJ_TEXT_WMI_EXT10: WMI_OBJ_TEXT = WMI_OBJ_TEXT(12i32);
pub const WMI_OBJ_TEXT_LAST: WMI_OBJ_TEXT = WMI_OBJ_TEXT(13i32);
impl ::core::marker::Copy for WMI_OBJ_TEXT {}
impl ::core::clone::Clone for WMI_OBJ_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMI_OBJ_TEXT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WMI_OBJ_TEXT {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMI_OBJ_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMI_OBJ_TEXT").field(&self.0).finish()
    }
}
pub const WbemAdministrativeLocator: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcb8555cc_9128_11d1_ad9b_00c04fd8fdff);
pub const WbemAuthenticatedLocator: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcd184336_9128_11d1_ad9b_00c04fd8fdff);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WbemAuthenticationLevelEnum(pub i32);
pub const wbemAuthenticationLevelDefault: WbemAuthenticationLevelEnum = WbemAuthenticationLevelEnum(0i32);
pub const wbemAuthenticationLevelNone: WbemAuthenticationLevelEnum = WbemAuthenticationLevelEnum(1i32);
pub const wbemAuthenticationLevelConnect: WbemAuthenticationLevelEnum = WbemAuthenticationLevelEnum(2i32);
pub const wbemAuthenticationLevelCall: WbemAuthenticationLevelEnum = WbemAuthenticationLevelEnum(3i32);
pub const wbemAuthenticationLevelPkt: WbemAuthenticationLevelEnum = WbemAuthenticationLevelEnum(4i32);
pub const wbemAuthenticationLevelPktIntegrity: WbemAuthenticationLevelEnum = WbemAuthenticationLevelEnum(5i32);
pub const wbemAuthenticationLevelPktPrivacy: WbemAuthenticationLevelEnum = WbemAuthenticationLevelEnum(6i32);
impl ::core::marker::Copy for WbemAuthenticationLevelEnum {}
impl ::core::clone::Clone for WbemAuthenticationLevelEnum {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WbemAuthenticationLevelEnum {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WbemAuthenticationLevelEnum {
    type Abi = Self;
}
impl ::core::fmt::Debug for WbemAuthenticationLevelEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemAuthenticationLevelEnum").field(&self.0).finish()
    }
}
pub const WbemBackupRestore: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc49e32c6_bc8b_11d2_85d4_00105a1f8304);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WbemChangeFlagEnum(pub i32);
pub const wbemChangeFlagCreateOrUpdate: WbemChangeFlagEnum = WbemChangeFlagEnum(0i32);
pub const wbemChangeFlagUpdateOnly: WbemChangeFlagEnum = WbemChangeFlagEnum(1i32);
pub const wbemChangeFlagCreateOnly: WbemChangeFlagEnum = WbemChangeFlagEnum(2i32);
pub const wbemChangeFlagUpdateCompatible: WbemChangeFlagEnum = WbemChangeFlagEnum(0i32);
pub const wbemChangeFlagUpdateSafeMode: WbemChangeFlagEnum = WbemChangeFlagEnum(32i32);
pub const wbemChangeFlagUpdateForceMode: WbemChangeFlagEnum = WbemChangeFlagEnum(64i32);
pub const wbemChangeFlagStrongValidation: WbemChangeFlagEnum = WbemChangeFlagEnum(128i32);
pub const wbemChangeFlagAdvisory: WbemChangeFlagEnum = WbemChangeFlagEnum(65536i32);
impl ::core::marker::Copy for WbemChangeFlagEnum {}
impl ::core::clone::Clone for WbemChangeFlagEnum {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WbemChangeFlagEnum {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WbemChangeFlagEnum {
    type Abi = Self;
}
impl ::core::fmt::Debug for WbemChangeFlagEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemChangeFlagEnum").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WbemCimtypeEnum(pub i32);
pub const wbemCimtypeSint8: WbemCimtypeEnum = WbemCimtypeEnum(16i32);
pub const wbemCimtypeUint8: WbemCimtypeEnum = WbemCimtypeEnum(17i32);
pub const wbemCimtypeSint16: WbemCimtypeEnum = WbemCimtypeEnum(2i32);
pub const wbemCimtypeUint16: WbemCimtypeEnum = WbemCimtypeEnum(18i32);
pub const wbemCimtypeSint32: WbemCimtypeEnum = WbemCimtypeEnum(3i32);
pub const wbemCimtypeUint32: WbemCimtypeEnum = WbemCimtypeEnum(19i32);
pub const wbemCimtypeSint64: WbemCimtypeEnum = WbemCimtypeEnum(20i32);
pub const wbemCimtypeUint64: WbemCimtypeEnum = WbemCimtypeEnum(21i32);
pub const wbemCimtypeReal32: WbemCimtypeEnum = WbemCimtypeEnum(4i32);
pub const wbemCimtypeReal64: WbemCimtypeEnum = WbemCimtypeEnum(5i32);
pub const wbemCimtypeBoolean: WbemCimtypeEnum = WbemCimtypeEnum(11i32);
pub const wbemCimtypeString: WbemCimtypeEnum = WbemCimtypeEnum(8i32);
pub const wbemCimtypeDatetime: WbemCimtypeEnum = WbemCimtypeEnum(101i32);
pub const wbemCimtypeReference: WbemCimtypeEnum = WbemCimtypeEnum(102i32);
pub const wbemCimtypeChar16: WbemCimtypeEnum = WbemCimtypeEnum(103i32);
pub const wbemCimtypeObject: WbemCimtypeEnum = WbemCimtypeEnum(13i32);
impl ::core::marker::Copy for WbemCimtypeEnum {}
impl ::core::clone::Clone for WbemCimtypeEnum {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WbemCimtypeEnum {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WbemCimtypeEnum {
    type Abi = Self;
}
impl ::core::fmt::Debug for WbemCimtypeEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemCimtypeEnum").field(&self.0).finish()
    }
}
pub const WbemClassObject: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9a653086_174f_11d2_b5f9_00104b703efd);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WbemComparisonFlagEnum(pub i32);
pub const wbemComparisonFlagIncludeAll: WbemComparisonFlagEnum = WbemComparisonFlagEnum(0i32);
pub const wbemComparisonFlagIgnoreQualifiers: WbemComparisonFlagEnum = WbemComparisonFlagEnum(1i32);
pub const wbemComparisonFlagIgnoreObjectSource: WbemComparisonFlagEnum = WbemComparisonFlagEnum(2i32);
pub const wbemComparisonFlagIgnoreDefaultValues: WbemComparisonFlagEnum = WbemComparisonFlagEnum(4i32);
pub const wbemComparisonFlagIgnoreClass: WbemComparisonFlagEnum = WbemComparisonFlagEnum(8i32);
pub const wbemComparisonFlagIgnoreCase: WbemComparisonFlagEnum = WbemComparisonFlagEnum(16i32);
pub const wbemComparisonFlagIgnoreFlavor: WbemComparisonFlagEnum = WbemComparisonFlagEnum(32i32);
impl ::core::marker::Copy for WbemComparisonFlagEnum {}
impl ::core::clone::Clone for WbemComparisonFlagEnum {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WbemComparisonFlagEnum {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WbemComparisonFlagEnum {
    type Abi = Self;
}
impl ::core::fmt::Debug for WbemComparisonFlagEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemComparisonFlagEnum").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WbemConnectOptionsEnum(pub i32);
pub const wbemConnectFlagUseMaxWait: WbemConnectOptionsEnum = WbemConnectOptionsEnum(128i32);
impl ::core::marker::Copy for WbemConnectOptionsEnum {}
impl ::core::clone::Clone for WbemConnectOptionsEnum {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WbemConnectOptionsEnum {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WbemConnectOptionsEnum {
    type Abi = Self;
}
impl ::core::fmt::Debug for WbemConnectOptionsEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemConnectOptionsEnum").field(&self.0).finish()
    }
}
pub const WbemContext: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x674b6698_ee92_11d0_ad71_00c04fd8fdff);
pub const WbemDCOMTransport: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf7ce2e13_8c90_11d1_9e7b_00c04fc324a8);
pub const WbemDecoupledBasicEventProvider: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf5f75737_2843_4f22_933d_c76a97cda62f);
pub const WbemDecoupledRegistrar: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4cfc7932_0f9d_4bef_9c32_8ea2a6b56fcb);
pub const WbemDefPath: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcf4cc405_e2c5_4ddd_b3ce_5e7582d8c9fa);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WbemErrorEnum(pub i32);
pub const wbemNoErr: WbemErrorEnum = WbemErrorEnum(0i32);
pub const wbemErrFailed: WbemErrorEnum = WbemErrorEnum(-2147217407i32);
pub const wbemErrNotFound: WbemErrorEnum = WbemErrorEnum(-2147217406i32);
pub const wbemErrAccessDenied: WbemErrorEnum = WbemErrorEnum(-2147217405i32);
pub const wbemErrProviderFailure: WbemErrorEnum = WbemErrorEnum(-2147217404i32);
pub const wbemErrTypeMismatch: WbemErrorEnum = WbemErrorEnum(-2147217403i32);
pub const wbemErrOutOfMemory: WbemErrorEnum = WbemErrorEnum(-2147217402i32);
pub const wbemErrInvalidContext: WbemErrorEnum = WbemErrorEnum(-2147217401i32);
pub const wbemErrInvalidParameter: WbemErrorEnum = WbemErrorEnum(-2147217400i32);
pub const wbemErrNotAvailable: WbemErrorEnum = WbemErrorEnum(-2147217399i32);
pub const wbemErrCriticalError: WbemErrorEnum = WbemErrorEnum(-2147217398i32);
pub const wbemErrInvalidStream: WbemErrorEnum = WbemErrorEnum(-2147217397i32);
pub const wbemErrNotSupported: WbemErrorEnum = WbemErrorEnum(-2147217396i32);
pub const wbemErrInvalidSuperclass: WbemErrorEnum = WbemErrorEnum(-2147217395i32);
pub const wbemErrInvalidNamespace: WbemErrorEnum = WbemErrorEnum(-2147217394i32);
pub const wbemErrInvalidObject: WbemErrorEnum = WbemErrorEnum(-2147217393i32);
pub const wbemErrInvalidClass: WbemErrorEnum = WbemErrorEnum(-2147217392i32);
pub const wbemErrProviderNotFound: WbemErrorEnum = WbemErrorEnum(-2147217391i32);
pub const wbemErrInvalidProviderRegistration: WbemErrorEnum = WbemErrorEnum(-2147217390i32);
pub const wbemErrProviderLoadFailure: WbemErrorEnum = WbemErrorEnum(-2147217389i32);
pub const wbemErrInitializationFailure: WbemErrorEnum = WbemErrorEnum(-2147217388i32);
pub const wbemErrTransportFailure: WbemErrorEnum = WbemErrorEnum(-2147217387i32);
pub const wbemErrInvalidOperation: WbemErrorEnum = WbemErrorEnum(-2147217386i32);
pub const wbemErrInvalidQuery: WbemErrorEnum = WbemErrorEnum(-2147217385i32);
pub const wbemErrInvalidQueryType: WbemErrorEnum = WbemErrorEnum(-2147217384i32);
pub const wbemErrAlreadyExists: WbemErrorEnum = WbemErrorEnum(-2147217383i32);
pub const wbemErrOverrideNotAllowed: WbemErrorEnum = WbemErrorEnum(-2147217382i32);
pub const wbemErrPropagatedQualifier: WbemErrorEnum = WbemErrorEnum(-2147217381i32);
pub const wbemErrPropagatedProperty: WbemErrorEnum = WbemErrorEnum(-2147217380i32);
pub const wbemErrUnexpected: WbemErrorEnum = WbemErrorEnum(-2147217379i32);
pub const wbemErrIllegalOperation: WbemErrorEnum = WbemErrorEnum(-2147217378i32);
pub const wbemErrCannotBeKey: WbemErrorEnum = WbemErrorEnum(-2147217377i32);
pub const wbemErrIncompleteClass: WbemErrorEnum = WbemErrorEnum(-2147217376i32);
pub const wbemErrInvalidSyntax: WbemErrorEnum = WbemErrorEnum(-2147217375i32);
pub const wbemErrNondecoratedObject: WbemErrorEnum = WbemErrorEnum(-2147217374i32);
pub const wbemErrReadOnly: WbemErrorEnum = WbemErrorEnum(-2147217373i32);
pub const wbemErrProviderNotCapable: WbemErrorEnum = WbemErrorEnum(-2147217372i32);
pub const wbemErrClassHasChildren: WbemErrorEnum = WbemErrorEnum(-2147217371i32);
pub const wbemErrClassHasInstances: WbemErrorEnum = WbemErrorEnum(-2147217370i32);
pub const wbemErrQueryNotImplemented: WbemErrorEnum = WbemErrorEnum(-2147217369i32);
pub const wbemErrIllegalNull: WbemErrorEnum = WbemErrorEnum(-2147217368i32);
pub const wbemErrInvalidQualifierType: WbemErrorEnum = WbemErrorEnum(-2147217367i32);
pub const wbemErrInvalidPropertyType: WbemErrorEnum = WbemErrorEnum(-2147217366i32);
pub const wbemErrValueOutOfRange: WbemErrorEnum = WbemErrorEnum(-2147217365i32);
pub const wbemErrCannotBeSingleton: WbemErrorEnum = WbemErrorEnum(-2147217364i32);
pub const wbemErrInvalidCimType: WbemErrorEnum = WbemErrorEnum(-2147217363i32);
pub const wbemErrInvalidMethod: WbemErrorEnum = WbemErrorEnum(-2147217362i32);
pub const wbemErrInvalidMethodParameters: WbemErrorEnum = WbemErrorEnum(-2147217361i32);
pub const wbemErrSystemProperty: WbemErrorEnum = WbemErrorEnum(-2147217360i32);
pub const wbemErrInvalidProperty: WbemErrorEnum = WbemErrorEnum(-2147217359i32);
pub const wbemErrCallCancelled: WbemErrorEnum = WbemErrorEnum(-2147217358i32);
pub const wbemErrShuttingDown: WbemErrorEnum = WbemErrorEnum(-2147217357i32);
pub const wbemErrPropagatedMethod: WbemErrorEnum = WbemErrorEnum(-2147217356i32);
pub const wbemErrUnsupportedParameter: WbemErrorEnum = WbemErrorEnum(-2147217355i32);
pub const wbemErrMissingParameter: WbemErrorEnum = WbemErrorEnum(-2147217354i32);
pub const wbemErrInvalidParameterId: WbemErrorEnum = WbemErrorEnum(-2147217353i32);
pub const wbemErrNonConsecutiveParameterIds: WbemErrorEnum = WbemErrorEnum(-2147217352i32);
pub const wbemErrParameterIdOnRetval: WbemErrorEnum = WbemErrorEnum(-2147217351i32);
pub const wbemErrInvalidObjectPath: WbemErrorEnum = WbemErrorEnum(-2147217350i32);
pub const wbemErrOutOfDiskSpace: WbemErrorEnum = WbemErrorEnum(-2147217349i32);
pub const wbemErrBufferTooSmall: WbemErrorEnum = WbemErrorEnum(-2147217348i32);
pub const wbemErrUnsupportedPutExtension: WbemErrorEnum = WbemErrorEnum(-2147217347i32);
pub const wbemErrUnknownObjectType: WbemErrorEnum = WbemErrorEnum(-2147217346i32);
pub const wbemErrUnknownPacketType: WbemErrorEnum = WbemErrorEnum(-2147217345i32);
pub const wbemErrMarshalVersionMismatch: WbemErrorEnum = WbemErrorEnum(-2147217344i32);
pub const wbemErrMarshalInvalidSignature: WbemErrorEnum = WbemErrorEnum(-2147217343i32);
pub const wbemErrInvalidQualifier: WbemErrorEnum = WbemErrorEnum(-2147217342i32);
pub const wbemErrInvalidDuplicateParameter: WbemErrorEnum = WbemErrorEnum(-2147217341i32);
pub const wbemErrTooMuchData: WbemErrorEnum = WbemErrorEnum(-2147217340i32);
pub const wbemErrServerTooBusy: WbemErrorEnum = WbemErrorEnum(-2147217339i32);
pub const wbemErrInvalidFlavor: WbemErrorEnum = WbemErrorEnum(-2147217338i32);
pub const wbemErrCircularReference: WbemErrorEnum = WbemErrorEnum(-2147217337i32);
pub const wbemErrUnsupportedClassUpdate: WbemErrorEnum = WbemErrorEnum(-2147217336i32);
pub const wbemErrCannotChangeKeyInheritance: WbemErrorEnum = WbemErrorEnum(-2147217335i32);
pub const wbemErrCannotChangeIndexInheritance: WbemErrorEnum = WbemErrorEnum(-2147217328i32);
pub const wbemErrTooManyProperties: WbemErrorEnum = WbemErrorEnum(-2147217327i32);
pub const wbemErrUpdateTypeMismatch: WbemErrorEnum = WbemErrorEnum(-2147217326i32);
pub const wbemErrUpdateOverrideNotAllowed: WbemErrorEnum = WbemErrorEnum(-2147217325i32);
pub const wbemErrUpdatePropagatedMethod: WbemErrorEnum = WbemErrorEnum(-2147217324i32);
pub const wbemErrMethodNotImplemented: WbemErrorEnum = WbemErrorEnum(-2147217323i32);
pub const wbemErrMethodDisabled: WbemErrorEnum = WbemErrorEnum(-2147217322i32);
pub const wbemErrRefresherBusy: WbemErrorEnum = WbemErrorEnum(-2147217321i32);
pub const wbemErrUnparsableQuery: WbemErrorEnum = WbemErrorEnum(-2147217320i32);
pub const wbemErrNotEventClass: WbemErrorEnum = WbemErrorEnum(-2147217319i32);
pub const wbemErrMissingGroupWithin: WbemErrorEnum = WbemErrorEnum(-2147217318i32);
pub const wbemErrMissingAggregationList: WbemErrorEnum = WbemErrorEnum(-2147217317i32);
pub const wbemErrPropertyNotAnObject: WbemErrorEnum = WbemErrorEnum(-2147217316i32);
pub const wbemErrAggregatingByObject: WbemErrorEnum = WbemErrorEnum(-2147217315i32);
pub const wbemErrUninterpretableProviderQuery: WbemErrorEnum = WbemErrorEnum(-2147217313i32);
pub const wbemErrBackupRestoreWinmgmtRunning: WbemErrorEnum = WbemErrorEnum(-2147217312i32);
pub const wbemErrQueueOverflow: WbemErrorEnum = WbemErrorEnum(-2147217311i32);
pub const wbemErrPrivilegeNotHeld: WbemErrorEnum = WbemErrorEnum(-2147217310i32);
pub const wbemErrInvalidOperator: WbemErrorEnum = WbemErrorEnum(-2147217309i32);
pub const wbemErrLocalCredentials: WbemErrorEnum = WbemErrorEnum(-2147217308i32);
pub const wbemErrCannotBeAbstract: WbemErrorEnum = WbemErrorEnum(-2147217307i32);
pub const wbemErrAmendedObject: WbemErrorEnum = WbemErrorEnum(-2147217306i32);
pub const wbemErrClientTooSlow: WbemErrorEnum = WbemErrorEnum(-2147217305i32);
pub const wbemErrNullSecurityDescriptor: WbemErrorEnum = WbemErrorEnum(-2147217304i32);
pub const wbemErrTimeout: WbemErrorEnum = WbemErrorEnum(-2147217303i32);
pub const wbemErrInvalidAssociation: WbemErrorEnum = WbemErrorEnum(-2147217302i32);
pub const wbemErrAmbiguousOperation: WbemErrorEnum = WbemErrorEnum(-2147217301i32);
pub const wbemErrQuotaViolation: WbemErrorEnum = WbemErrorEnum(-2147217300i32);
pub const wbemErrTransactionConflict: WbemErrorEnum = WbemErrorEnum(-2147217299i32);
pub const wbemErrForcedRollback: WbemErrorEnum = WbemErrorEnum(-2147217298i32);
pub const wbemErrUnsupportedLocale: WbemErrorEnum = WbemErrorEnum(-2147217297i32);
pub const wbemErrHandleOutOfDate: WbemErrorEnum = WbemErrorEnum(-2147217296i32);
pub const wbemErrConnectionFailed: WbemErrorEnum = WbemErrorEnum(-2147217295i32);
pub const wbemErrInvalidHandleRequest: WbemErrorEnum = WbemErrorEnum(-2147217294i32);
pub const wbemErrPropertyNameTooWide: WbemErrorEnum = WbemErrorEnum(-2147217293i32);
pub const wbemErrClassNameTooWide: WbemErrorEnum = WbemErrorEnum(-2147217292i32);
pub const wbemErrMethodNameTooWide: WbemErrorEnum = WbemErrorEnum(-2147217291i32);
pub const wbemErrQualifierNameTooWide: WbemErrorEnum = WbemErrorEnum(-2147217290i32);
pub const wbemErrRerunCommand: WbemErrorEnum = WbemErrorEnum(-2147217289i32);
pub const wbemErrDatabaseVerMismatch: WbemErrorEnum = WbemErrorEnum(-2147217288i32);
pub const wbemErrVetoPut: WbemErrorEnum = WbemErrorEnum(-2147217287i32);
pub const wbemErrVetoDelete: WbemErrorEnum = WbemErrorEnum(-2147217286i32);
pub const wbemErrInvalidLocale: WbemErrorEnum = WbemErrorEnum(-2147217280i32);
pub const wbemErrProviderSuspended: WbemErrorEnum = WbemErrorEnum(-2147217279i32);
pub const wbemErrSynchronizationRequired: WbemErrorEnum = WbemErrorEnum(-2147217278i32);
pub const wbemErrNoSchema: WbemErrorEnum = WbemErrorEnum(-2147217277i32);
pub const wbemErrProviderAlreadyRegistered: WbemErrorEnum = WbemErrorEnum(-2147217276i32);
pub const wbemErrProviderNotRegistered: WbemErrorEnum = WbemErrorEnum(-2147217275i32);
pub const wbemErrFatalTransportError: WbemErrorEnum = WbemErrorEnum(-2147217274i32);
pub const wbemErrEncryptedConnectionRequired: WbemErrorEnum = WbemErrorEnum(-2147217273i32);
pub const wbemErrRegistrationTooBroad: WbemErrorEnum = WbemErrorEnum(-2147213311i32);
pub const wbemErrRegistrationTooPrecise: WbemErrorEnum = WbemErrorEnum(-2147213310i32);
pub const wbemErrTimedout: WbemErrorEnum = WbemErrorEnum(-2147209215i32);
pub const wbemErrResetToDefault: WbemErrorEnum = WbemErrorEnum(-2147209214i32);
impl ::core::marker::Copy for WbemErrorEnum {}
impl ::core::clone::Clone for WbemErrorEnum {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WbemErrorEnum {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WbemErrorEnum {
    type Abi = Self;
}
impl ::core::fmt::Debug for WbemErrorEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemErrorEnum").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WbemFlagEnum(pub i32);
pub const wbemFlagReturnImmediately: WbemFlagEnum = WbemFlagEnum(16i32);
pub const wbemFlagReturnWhenComplete: WbemFlagEnum = WbemFlagEnum(0i32);
pub const wbemFlagBidirectional: WbemFlagEnum = WbemFlagEnum(0i32);
pub const wbemFlagForwardOnly: WbemFlagEnum = WbemFlagEnum(32i32);
pub const wbemFlagNoErrorObject: WbemFlagEnum = WbemFlagEnum(64i32);
pub const wbemFlagReturnErrorObject: WbemFlagEnum = WbemFlagEnum(0i32);
pub const wbemFlagSendStatus: WbemFlagEnum = WbemFlagEnum(128i32);
pub const wbemFlagDontSendStatus: WbemFlagEnum = WbemFlagEnum(0i32);
pub const wbemFlagEnsureLocatable: WbemFlagEnum = WbemFlagEnum(256i32);
pub const wbemFlagDirectRead: WbemFlagEnum = WbemFlagEnum(512i32);
pub const wbemFlagSendOnlySelected: WbemFlagEnum = WbemFlagEnum(0i32);
pub const wbemFlagUseAmendedQualifiers: WbemFlagEnum = WbemFlagEnum(131072i32);
pub const wbemFlagGetDefault: WbemFlagEnum = WbemFlagEnum(0i32);
pub const wbemFlagSpawnInstance: WbemFlagEnum = WbemFlagEnum(1i32);
pub const wbemFlagUseCurrentTime: WbemFlagEnum = WbemFlagEnum(1i32);
impl ::core::marker::Copy for WbemFlagEnum {}
impl ::core::clone::Clone for WbemFlagEnum {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WbemFlagEnum {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WbemFlagEnum {
    type Abi = Self;
}
impl ::core::fmt::Debug for WbemFlagEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemFlagEnum").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WbemImpersonationLevelEnum(pub i32);
pub const wbemImpersonationLevelAnonymous: WbemImpersonationLevelEnum = WbemImpersonationLevelEnum(1i32);
pub const wbemImpersonationLevelIdentify: WbemImpersonationLevelEnum = WbemImpersonationLevelEnum(2i32);
pub const wbemImpersonationLevelImpersonate: WbemImpersonationLevelEnum = WbemImpersonationLevelEnum(3i32);
pub const wbemImpersonationLevelDelegate: WbemImpersonationLevelEnum = WbemImpersonationLevelEnum(4i32);
impl ::core::marker::Copy for WbemImpersonationLevelEnum {}
impl ::core::clone::Clone for WbemImpersonationLevelEnum {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WbemImpersonationLevelEnum {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WbemImpersonationLevelEnum {
    type Abi = Self;
}
impl ::core::fmt::Debug for WbemImpersonationLevelEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemImpersonationLevelEnum").field(&self.0).finish()
    }
}
pub const WbemLevel1Login: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8bc3f05e_d86b_11d0_a075_00c04fb68820);
pub const WbemLocalAddrRes: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa1044801_8f7e_11d1_9e7c_00c04fc324a8);
pub const WbemLocator: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4590f811_1d3a_11d0_891f_00aa004b2e24);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WbemObjectTextFormatEnum(pub i32);
pub const wbemObjectTextFormatCIMDTD20: WbemObjectTextFormatEnum = WbemObjectTextFormatEnum(1i32);
pub const wbemObjectTextFormatWMIDTD20: WbemObjectTextFormatEnum = WbemObjectTextFormatEnum(2i32);
impl ::core::marker::Copy for WbemObjectTextFormatEnum {}
impl ::core::clone::Clone for WbemObjectTextFormatEnum {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WbemObjectTextFormatEnum {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WbemObjectTextFormatEnum {
    type Abi = Self;
}
impl ::core::fmt::Debug for WbemObjectTextFormatEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemObjectTextFormatEnum").field(&self.0).finish()
    }
}
pub const WbemObjectTextSrc: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8d1c559d_84f0_4bb3_a7d5_56a7435a9ba6);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WbemPrivilegeEnum(pub i32);
pub const wbemPrivilegeCreateToken: WbemPrivilegeEnum = WbemPrivilegeEnum(1i32);
pub const wbemPrivilegePrimaryToken: WbemPrivilegeEnum = WbemPrivilegeEnum(2i32);
pub const wbemPrivilegeLockMemory: WbemPrivilegeEnum = WbemPrivilegeEnum(3i32);
pub const wbemPrivilegeIncreaseQuota: WbemPrivilegeEnum = WbemPrivilegeEnum(4i32);
pub const wbemPrivilegeMachineAccount: WbemPrivilegeEnum = WbemPrivilegeEnum(5i32);
pub const wbemPrivilegeTcb: WbemPrivilegeEnum = WbemPrivilegeEnum(6i32);
pub const wbemPrivilegeSecurity: WbemPrivilegeEnum = WbemPrivilegeEnum(7i32);
pub const wbemPrivilegeTakeOwnership: WbemPrivilegeEnum = WbemPrivilegeEnum(8i32);
pub const wbemPrivilegeLoadDriver: WbemPrivilegeEnum = WbemPrivilegeEnum(9i32);
pub const wbemPrivilegeSystemProfile: WbemPrivilegeEnum = WbemPrivilegeEnum(10i32);
pub const wbemPrivilegeSystemtime: WbemPrivilegeEnum = WbemPrivilegeEnum(11i32);
pub const wbemPrivilegeProfileSingleProcess: WbemPrivilegeEnum = WbemPrivilegeEnum(12i32);
pub const wbemPrivilegeIncreaseBasePriority: WbemPrivilegeEnum = WbemPrivilegeEnum(13i32);
pub const wbemPrivilegeCreatePagefile: WbemPrivilegeEnum = WbemPrivilegeEnum(14i32);
pub const wbemPrivilegeCreatePermanent: WbemPrivilegeEnum = WbemPrivilegeEnum(15i32);
pub const wbemPrivilegeBackup: WbemPrivilegeEnum = WbemPrivilegeEnum(16i32);
pub const wbemPrivilegeRestore: WbemPrivilegeEnum = WbemPrivilegeEnum(17i32);
pub const wbemPrivilegeShutdown: WbemPrivilegeEnum = WbemPrivilegeEnum(18i32);
pub const wbemPrivilegeDebug: WbemPrivilegeEnum = WbemPrivilegeEnum(19i32);
pub const wbemPrivilegeAudit: WbemPrivilegeEnum = WbemPrivilegeEnum(20i32);
pub const wbemPrivilegeSystemEnvironment: WbemPrivilegeEnum = WbemPrivilegeEnum(21i32);
pub const wbemPrivilegeChangeNotify: WbemPrivilegeEnum = WbemPrivilegeEnum(22i32);
pub const wbemPrivilegeRemoteShutdown: WbemPrivilegeEnum = WbemPrivilegeEnum(23i32);
pub const wbemPrivilegeUndock: WbemPrivilegeEnum = WbemPrivilegeEnum(24i32);
pub const wbemPrivilegeSyncAgent: WbemPrivilegeEnum = WbemPrivilegeEnum(25i32);
pub const wbemPrivilegeEnableDelegation: WbemPrivilegeEnum = WbemPrivilegeEnum(26i32);
pub const wbemPrivilegeManageVolume: WbemPrivilegeEnum = WbemPrivilegeEnum(27i32);
impl ::core::marker::Copy for WbemPrivilegeEnum {}
impl ::core::clone::Clone for WbemPrivilegeEnum {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WbemPrivilegeEnum {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WbemPrivilegeEnum {
    type Abi = Self;
}
impl ::core::fmt::Debug for WbemPrivilegeEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemPrivilegeEnum").field(&self.0).finish()
    }
}
pub const WbemQuery: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeac8a024_21e2_4523_ad73_a71a0aa2f56a);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WbemQueryFlagEnum(pub i32);
pub const wbemQueryFlagDeep: WbemQueryFlagEnum = WbemQueryFlagEnum(0i32);
pub const wbemQueryFlagShallow: WbemQueryFlagEnum = WbemQueryFlagEnum(1i32);
pub const wbemQueryFlagPrototype: WbemQueryFlagEnum = WbemQueryFlagEnum(2i32);
impl ::core::marker::Copy for WbemQueryFlagEnum {}
impl ::core::clone::Clone for WbemQueryFlagEnum {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WbemQueryFlagEnum {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WbemQueryFlagEnum {
    type Abi = Self;
}
impl ::core::fmt::Debug for WbemQueryFlagEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemQueryFlagEnum").field(&self.0).finish()
    }
}
pub const WbemRefresher: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc71566f2_561e_11d1_ad87_00c04fd8fdff);
pub const WbemStatusCodeText: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeb87e1bd_3233_11d2_aec9_00c04fb68820);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WbemTextFlagEnum(pub i32);
pub const wbemTextFlagNoFlavors: WbemTextFlagEnum = WbemTextFlagEnum(1i32);
impl ::core::marker::Copy for WbemTextFlagEnum {}
impl ::core::clone::Clone for WbemTextFlagEnum {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WbemTextFlagEnum {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WbemTextFlagEnum {
    type Abi = Self;
}
impl ::core::fmt::Debug for WbemTextFlagEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemTextFlagEnum").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WbemTimeout(pub i32);
pub const wbemTimeoutInfinite: WbemTimeout = WbemTimeout(-1i32);
impl ::core::marker::Copy for WbemTimeout {}
impl ::core::clone::Clone for WbemTimeout {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WbemTimeout {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WbemTimeout {
    type Abi = Self;
}
impl ::core::fmt::Debug for WbemTimeout {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemTimeout").field(&self.0).finish()
    }
}
pub const WbemUnauthenticatedLocator: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x443e7b79_de31_11d2_b340_00104bcc4b4a);
pub const WbemUninitializedClassObject: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7a0227f6_7108_11d1_ad90_00c04fd8fdff);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct tag_WBEM_LOGIN_TYPE(pub i32);
pub const WBEM_FLAG_INPROC_LOGIN: tag_WBEM_LOGIN_TYPE = tag_WBEM_LOGIN_TYPE(0i32);
pub const WBEM_FLAG_LOCAL_LOGIN: tag_WBEM_LOGIN_TYPE = tag_WBEM_LOGIN_TYPE(1i32);
pub const WBEM_FLAG_REMOTE_LOGIN: tag_WBEM_LOGIN_TYPE = tag_WBEM_LOGIN_TYPE(2i32);
pub const WBEM_AUTHENTICATION_METHOD_MASK: tag_WBEM_LOGIN_TYPE = tag_WBEM_LOGIN_TYPE(15i32);
pub const WBEM_FLAG_USE_MULTIPLE_CHALLENGES: tag_WBEM_LOGIN_TYPE = tag_WBEM_LOGIN_TYPE(16i32);
impl ::core::marker::Copy for tag_WBEM_LOGIN_TYPE {}
impl ::core::clone::Clone for tag_WBEM_LOGIN_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for tag_WBEM_LOGIN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for tag_WBEM_LOGIN_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for tag_WBEM_LOGIN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("tag_WBEM_LOGIN_TYPE").field(&self.0).finish()
    }
}
