pub const CLSID_CTask: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x148bd520_a2ab_11ce_b11f_00aa00530503);
pub const CLSID_CTaskScheduler: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x148bd52a_a2ab_11ce_b11f_00aa00530503);
#[repr(C)]
pub struct DAILY {
    pub DaysInterval: u16,
}
impl ::core::marker::Copy for DAILY {}
impl ::core::clone::Clone for DAILY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DAILY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DAILY").field("DaysInterval", &self.DaysInterval).finish()
    }
}
unsafe impl ::windows_core::Abi for DAILY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DAILY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DAILY>()) == 0 }
    }
}
impl ::core::cmp::Eq for DAILY {}
impl ::core::default::Default for DAILY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IAction(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IAction {
    pub unsafe fn Id(&self, pid: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    pub unsafe fn SetId<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, id: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetId)(::windows_core::Interface::as_raw(self), id.into_param().abi()).ok()
    }
    pub unsafe fn Type(&self, ptype: *mut TASK_ACTION_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IAction> for ::windows_core::IUnknown {
    fn from(value: IAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IAction> for ::windows_core::IUnknown {
    fn from(value: &IAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IAction> for super::Com::IDispatch {
    fn from(value: IAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IAction> for super::Com::IDispatch {
    fn from(value: &IAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IAction {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IAction {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IAction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IAction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IAction {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAction").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IAction {
    type Vtable = IAction_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbae54997_48b1_4cbe_9965_d6be263ebea4);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IAction_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut TASK_ACTION_TYPE) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IActionCollection(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IActionCollection {
    pub unsafe fn Count(&self, pcount: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcount)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows_core::Result<IAction> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAction>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn XmlText(&self, ptext: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).XmlText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptext)).ok()
    }
    pub unsafe fn SetXmlText<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, text: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetXmlText)(::windows_core::Interface::as_raw(self), text.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Create(&self, r#type: TASK_ACTION_TYPE) -> ::windows_core::Result<IAction> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Create)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(r#type), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAction>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Remove<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), index.into_param().abi()).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Clear)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Context(&self, pcontext: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Context)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcontext)).ok()
    }
    pub unsafe fn SetContext<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, context: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetContext)(::windows_core::Interface::as_raw(self), context.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IActionCollection> for ::windows_core::IUnknown {
    fn from(value: IActionCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IActionCollection> for ::windows_core::IUnknown {
    fn from(value: &IActionCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IActionCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IActionCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IActionCollection> for super::Com::IDispatch {
    fn from(value: IActionCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IActionCollection> for super::Com::IDispatch {
    fn from(value: &IActionCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IActionCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IActionCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IActionCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IActionCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IActionCollection {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IActionCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActionCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IActionCollection {
    type Vtable = IActionCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x02820e19_7b98_4ed2_b2e8_fdccceff619b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IActionCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, ppaction: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub XmlText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptext: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetXmlText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: TASK_ACTION_TYPE, ppaction: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Create: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Context: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IBootTrigger(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IBootTrigger {
    pub unsafe fn Type(&self, ptype: *mut TASK_TRIGGER_TYPE2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    pub unsafe fn Id(&self, pid: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    pub unsafe fn SetId<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, id: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetId)(::windows_core::Interface::as_raw(self), id.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Repetition(&self) -> ::windows_core::Result<IRepetitionPattern> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Repetition)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRepetitionPattern>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetRepetition<'a, Param0: ::windows_core::IntoParam<'a, IRepetitionPattern>>(&self, prepeat: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRepetition)(::windows_core::Interface::as_raw(self), prepeat.into_param().abi()).ok()
    }
    pub unsafe fn ExecutionTimeLimit(&self, ptimelimit: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ExecutionTimeLimit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptimelimit)).ok()
    }
    pub unsafe fn SetExecutionTimeLimit<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, timelimit: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetExecutionTimeLimit)(::windows_core::Interface::as_raw(self), timelimit.into_param().abi()).ok()
    }
    pub unsafe fn StartBoundary(&self, pstart: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StartBoundary)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pstart)).ok()
    }
    pub unsafe fn SetStartBoundary<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, start: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetStartBoundary)(::windows_core::Interface::as_raw(self), start.into_param().abi()).ok()
    }
    pub unsafe fn EndBoundary(&self, pend: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EndBoundary)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pend)).ok()
    }
    pub unsafe fn SetEndBoundary<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, end: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEndBoundary)(::windows_core::Interface::as_raw(self), end.into_param().abi()).ok()
    }
    pub unsafe fn Enabled(&self, penabled: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Enabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(penabled)).ok()
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enabled)).ok()
    }
    pub unsafe fn Delay(&self, pdelay: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delay)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdelay)).ok()
    }
    pub unsafe fn SetDelay<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, delay: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDelay)(::windows_core::Interface::as_raw(self), delay.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IBootTrigger> for ::windows_core::IUnknown {
    fn from(value: IBootTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IBootTrigger> for ::windows_core::IUnknown {
    fn from(value: &IBootTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IBootTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IBootTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IBootTrigger> for super::Com::IDispatch {
    fn from(value: IBootTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IBootTrigger> for super::Com::IDispatch {
    fn from(value: &IBootTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IBootTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IBootTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IBootTrigger> for ITrigger {
    fn from(value: IBootTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IBootTrigger> for ITrigger {
    fn from(value: &IBootTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ITrigger> for IBootTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ITrigger> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ITrigger> for &'a IBootTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ITrigger> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IBootTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IBootTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IBootTrigger {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IBootTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBootTrigger").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IBootTrigger {
    type Vtable = IBootTrigger_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2a9c35da_d357_41f4_bbc1_207ac1b1f3cb);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IBootTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub Delay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdelay: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, delay: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IComHandlerAction(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IComHandlerAction {
    pub unsafe fn Id(&self, pid: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    pub unsafe fn SetId<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, id: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetId)(::windows_core::Interface::as_raw(self), id.into_param().abi()).ok()
    }
    pub unsafe fn Type(&self, ptype: *mut TASK_ACTION_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    pub unsafe fn ClassId(&self, pclsid: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ClassId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pclsid)).ok()
    }
    pub unsafe fn SetClassId<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, clsid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetClassId)(::windows_core::Interface::as_raw(self), clsid.into_param().abi()).ok()
    }
    pub unsafe fn Data(&self, pdata: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Data)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetData<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, data: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetData)(::windows_core::Interface::as_raw(self), data.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IComHandlerAction> for ::windows_core::IUnknown {
    fn from(value: IComHandlerAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IComHandlerAction> for ::windows_core::IUnknown {
    fn from(value: &IComHandlerAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IComHandlerAction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IComHandlerAction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IComHandlerAction> for super::Com::IDispatch {
    fn from(value: IComHandlerAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IComHandlerAction> for super::Com::IDispatch {
    fn from(value: &IComHandlerAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IComHandlerAction {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IComHandlerAction {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IComHandlerAction> for IAction {
    fn from(value: IComHandlerAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IComHandlerAction> for IAction {
    fn from(value: &IComHandlerAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IAction> for IComHandlerAction {
    fn into_param(self) -> ::windows_core::Param<'a, IAction> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IAction> for &'a IComHandlerAction {
    fn into_param(self) -> ::windows_core::Param<'a, IAction> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IComHandlerAction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IComHandlerAction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IComHandlerAction {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IComHandlerAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComHandlerAction").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IComHandlerAction {
    type Vtable = IComHandlerAction_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6d2fd252_75c5_4f66_90ba_2a7d8cc3039f);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IComHandlerAction_Vtbl {
    pub base__: IAction_Vtbl,
    pub ClassId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetClassId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IDailyTrigger(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IDailyTrigger {
    pub unsafe fn Type(&self, ptype: *mut TASK_TRIGGER_TYPE2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    pub unsafe fn Id(&self, pid: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    pub unsafe fn SetId<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, id: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetId)(::windows_core::Interface::as_raw(self), id.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Repetition(&self) -> ::windows_core::Result<IRepetitionPattern> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Repetition)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRepetitionPattern>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetRepetition<'a, Param0: ::windows_core::IntoParam<'a, IRepetitionPattern>>(&self, prepeat: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRepetition)(::windows_core::Interface::as_raw(self), prepeat.into_param().abi()).ok()
    }
    pub unsafe fn ExecutionTimeLimit(&self, ptimelimit: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ExecutionTimeLimit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptimelimit)).ok()
    }
    pub unsafe fn SetExecutionTimeLimit<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, timelimit: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetExecutionTimeLimit)(::windows_core::Interface::as_raw(self), timelimit.into_param().abi()).ok()
    }
    pub unsafe fn StartBoundary(&self, pstart: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StartBoundary)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pstart)).ok()
    }
    pub unsafe fn SetStartBoundary<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, start: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetStartBoundary)(::windows_core::Interface::as_raw(self), start.into_param().abi()).ok()
    }
    pub unsafe fn EndBoundary(&self, pend: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EndBoundary)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pend)).ok()
    }
    pub unsafe fn SetEndBoundary<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, end: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEndBoundary)(::windows_core::Interface::as_raw(self), end.into_param().abi()).ok()
    }
    pub unsafe fn Enabled(&self, penabled: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Enabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(penabled)).ok()
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enabled)).ok()
    }
    pub unsafe fn DaysInterval(&self, pdays: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DaysInterval)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdays)).ok()
    }
    pub unsafe fn SetDaysInterval(&self, days: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDaysInterval)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(days)).ok()
    }
    pub unsafe fn RandomDelay(&self, prandomdelay: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RandomDelay)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(prandomdelay)).ok()
    }
    pub unsafe fn SetRandomDelay<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, randomdelay: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRandomDelay)(::windows_core::Interface::as_raw(self), randomdelay.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IDailyTrigger> for ::windows_core::IUnknown {
    fn from(value: IDailyTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IDailyTrigger> for ::windows_core::IUnknown {
    fn from(value: &IDailyTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDailyTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDailyTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IDailyTrigger> for super::Com::IDispatch {
    fn from(value: IDailyTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IDailyTrigger> for super::Com::IDispatch {
    fn from(value: &IDailyTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IDailyTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IDailyTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IDailyTrigger> for ITrigger {
    fn from(value: IDailyTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IDailyTrigger> for ITrigger {
    fn from(value: &IDailyTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ITrigger> for IDailyTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ITrigger> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ITrigger> for &'a IDailyTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ITrigger> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IDailyTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IDailyTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IDailyTrigger {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IDailyTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDailyTrigger").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IDailyTrigger {
    type Vtable = IDailyTrigger_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x126c5cd8_b288_41d5_8dbf_e491446adc5c);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IDailyTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub DaysInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdays: *mut i16) -> ::windows_core::HRESULT,
    pub SetDaysInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, days: i16) -> ::windows_core::HRESULT,
    pub RandomDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prandomdelay: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetRandomDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, randomdelay: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IEmailAction(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IEmailAction {
    pub unsafe fn Id(&self, pid: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    pub unsafe fn SetId<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, id: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetId)(::windows_core::Interface::as_raw(self), id.into_param().abi()).ok()
    }
    pub unsafe fn Type(&self, ptype: *mut TASK_ACTION_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    pub unsafe fn Server(&self, pserver: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Server)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pserver)).ok()
    }
    pub unsafe fn SetServer<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, server: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetServer)(::windows_core::Interface::as_raw(self), server.into_param().abi()).ok()
    }
    pub unsafe fn Subject(&self, psubject: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Subject)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(psubject)).ok()
    }
    pub unsafe fn SetSubject<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, subject: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSubject)(::windows_core::Interface::as_raw(self), subject.into_param().abi()).ok()
    }
    pub unsafe fn To(&self, pto: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).To)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pto)).ok()
    }
    pub unsafe fn SetTo<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, to: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTo)(::windows_core::Interface::as_raw(self), to.into_param().abi()).ok()
    }
    pub unsafe fn Cc(&self, pcc: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cc)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcc)).ok()
    }
    pub unsafe fn SetCc<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, cc: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCc)(::windows_core::Interface::as_raw(self), cc.into_param().abi()).ok()
    }
    pub unsafe fn Bcc(&self, pbcc: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Bcc)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbcc)).ok()
    }
    pub unsafe fn SetBcc<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bcc: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBcc)(::windows_core::Interface::as_raw(self), bcc.into_param().abi()).ok()
    }
    pub unsafe fn ReplyTo(&self, preplyto: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReplyTo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(preplyto)).ok()
    }
    pub unsafe fn SetReplyTo<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, replyto: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetReplyTo)(::windows_core::Interface::as_raw(self), replyto.into_param().abi()).ok()
    }
    pub unsafe fn From(&self, pfrom: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).From)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pfrom)).ok()
    }
    pub unsafe fn SetFrom<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, from: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFrom)(::windows_core::Interface::as_raw(self), from.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn HeaderFields(&self) -> ::windows_core::Result<ITaskNamedValueCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).HeaderFields)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITaskNamedValueCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetHeaderFields<'a, Param0: ::windows_core::IntoParam<'a, ITaskNamedValueCollection>>(&self, pheaderfields: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetHeaderFields)(::windows_core::Interface::as_raw(self), pheaderfields.into_param().abi()).ok()
    }
    pub unsafe fn Body(&self, pbody: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Body)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbody)).ok()
    }
    pub unsafe fn SetBody<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, body: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBody)(::windows_core::Interface::as_raw(self), body.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Attachments(&self, pattachements: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Attachments)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pattachements)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetAttachments(&self, pattachements: *mut super::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAttachments)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pattachements)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IEmailAction> for ::windows_core::IUnknown {
    fn from(value: IEmailAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IEmailAction> for ::windows_core::IUnknown {
    fn from(value: &IEmailAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEmailAction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEmailAction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IEmailAction> for super::Com::IDispatch {
    fn from(value: IEmailAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IEmailAction> for super::Com::IDispatch {
    fn from(value: &IEmailAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IEmailAction {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IEmailAction {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IEmailAction> for IAction {
    fn from(value: IEmailAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IEmailAction> for IAction {
    fn from(value: &IEmailAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IAction> for IEmailAction {
    fn into_param(self) -> ::windows_core::Param<'a, IAction> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IAction> for &'a IEmailAction {
    fn into_param(self) -> ::windows_core::Param<'a, IAction> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IEmailAction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IEmailAction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IEmailAction {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IEmailAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEmailAction").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IEmailAction {
    type Vtable = IEmailAction_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x10f62c64_7e16_4314_a0c2_0c3683f99d40);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IEmailAction_Vtbl {
    pub base__: IAction_Vtbl,
    pub Server: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pserver: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, server: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Subject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psubject: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetSubject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subject: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub To: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pto: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, to: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Cc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcc: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetCc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cc: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Bcc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbcc: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetBcc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bcc: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub ReplyTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preplyto: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetReplyTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, replyto: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub From: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfrom: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, from: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub HeaderFields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppheaderfields: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    HeaderFields: usize,
    #[cfg(feature = "win32-system")]
    pub SetHeaderFields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pheaderfields: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetHeaderFields: usize,
    pub Body: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbody: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, body: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Attachments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pattachements: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Attachments: usize,
    #[cfg(feature = "win32-system")]
    pub SetAttachments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pattachements: *mut super::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetAttachments: usize,
}
#[repr(transparent)]
pub struct IEnumWorkItems(::windows_core::IUnknown);
impl IEnumWorkItems {
    pub unsafe fn Next(&self, celt: u32, rgpwsznames: *mut *mut ::windows_core::PWSTR, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgpwsznames), ::core::mem::transmute(pceltfetched)))
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::HRESULT {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(celt)))
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumWorkItems> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumWorkItems>(result__)
    }
}
impl ::core::convert::From<IEnumWorkItems> for ::windows_core::IUnknown {
    fn from(value: IEnumWorkItems) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumWorkItems> for ::windows_core::IUnknown {
    fn from(value: &IEnumWorkItems) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumWorkItems {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumWorkItems {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumWorkItems {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumWorkItems {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumWorkItems {}
impl ::core::fmt::Debug for IEnumWorkItems {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumWorkItems").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumWorkItems {
    type Vtable = IEnumWorkItems_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x148bd528_a2ab_11ce_b11f_00aa00530503);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumWorkItems_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgpwsznames: *mut *mut ::windows_core::PWSTR, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumworkitems: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IEventTrigger(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IEventTrigger {
    pub unsafe fn Type(&self, ptype: *mut TASK_TRIGGER_TYPE2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    pub unsafe fn Id(&self, pid: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    pub unsafe fn SetId<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, id: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetId)(::windows_core::Interface::as_raw(self), id.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Repetition(&self) -> ::windows_core::Result<IRepetitionPattern> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Repetition)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRepetitionPattern>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetRepetition<'a, Param0: ::windows_core::IntoParam<'a, IRepetitionPattern>>(&self, prepeat: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRepetition)(::windows_core::Interface::as_raw(self), prepeat.into_param().abi()).ok()
    }
    pub unsafe fn ExecutionTimeLimit(&self, ptimelimit: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ExecutionTimeLimit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptimelimit)).ok()
    }
    pub unsafe fn SetExecutionTimeLimit<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, timelimit: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetExecutionTimeLimit)(::windows_core::Interface::as_raw(self), timelimit.into_param().abi()).ok()
    }
    pub unsafe fn StartBoundary(&self, pstart: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StartBoundary)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pstart)).ok()
    }
    pub unsafe fn SetStartBoundary<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, start: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetStartBoundary)(::windows_core::Interface::as_raw(self), start.into_param().abi()).ok()
    }
    pub unsafe fn EndBoundary(&self, pend: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EndBoundary)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pend)).ok()
    }
    pub unsafe fn SetEndBoundary<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, end: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEndBoundary)(::windows_core::Interface::as_raw(self), end.into_param().abi()).ok()
    }
    pub unsafe fn Enabled(&self, penabled: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Enabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(penabled)).ok()
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enabled)).ok()
    }
    pub unsafe fn Subscription(&self, pquery: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Subscription)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pquery)).ok()
    }
    pub unsafe fn SetSubscription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, query: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSubscription)(::windows_core::Interface::as_raw(self), query.into_param().abi()).ok()
    }
    pub unsafe fn Delay(&self, pdelay: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delay)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdelay)).ok()
    }
    pub unsafe fn SetDelay<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, delay: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDelay)(::windows_core::Interface::as_raw(self), delay.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ValueQueries(&self) -> ::windows_core::Result<ITaskNamedValueCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ValueQueries)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITaskNamedValueCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetValueQueries<'a, Param0: ::windows_core::IntoParam<'a, ITaskNamedValueCollection>>(&self, pnamedxpaths: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetValueQueries)(::windows_core::Interface::as_raw(self), pnamedxpaths.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IEventTrigger> for ::windows_core::IUnknown {
    fn from(value: IEventTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IEventTrigger> for ::windows_core::IUnknown {
    fn from(value: &IEventTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEventTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEventTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IEventTrigger> for super::Com::IDispatch {
    fn from(value: IEventTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IEventTrigger> for super::Com::IDispatch {
    fn from(value: &IEventTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IEventTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IEventTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IEventTrigger> for ITrigger {
    fn from(value: IEventTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IEventTrigger> for ITrigger {
    fn from(value: &IEventTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ITrigger> for IEventTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ITrigger> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ITrigger> for &'a IEventTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ITrigger> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IEventTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IEventTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IEventTrigger {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IEventTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventTrigger").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IEventTrigger {
    type Vtable = IEventTrigger_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd45b0167_9653_4eef_b94f_0732ca7af251);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IEventTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub Subscription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pquery: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetSubscription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Delay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdelay: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, delay: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub ValueQueries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnamedxpaths: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ValueQueries: usize,
    #[cfg(feature = "win32-system")]
    pub SetValueQueries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnamedxpaths: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetValueQueries: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IExecAction(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IExecAction {
    pub unsafe fn Id(&self, pid: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    pub unsafe fn SetId<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, id: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetId)(::windows_core::Interface::as_raw(self), id.into_param().abi()).ok()
    }
    pub unsafe fn Type(&self, ptype: *mut TASK_ACTION_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    pub unsafe fn Path(&self, ppath: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Path)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppath)).ok()
    }
    pub unsafe fn SetPath<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, path: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPath)(::windows_core::Interface::as_raw(self), path.into_param().abi()).ok()
    }
    pub unsafe fn Arguments(&self, pargument: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Arguments)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pargument)).ok()
    }
    pub unsafe fn SetArguments<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, argument: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetArguments)(::windows_core::Interface::as_raw(self), argument.into_param().abi()).ok()
    }
    pub unsafe fn WorkingDirectory(&self, pworkingdirectory: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WorkingDirectory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pworkingdirectory)).ok()
    }
    pub unsafe fn SetWorkingDirectory<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, workingdirectory: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetWorkingDirectory)(::windows_core::Interface::as_raw(self), workingdirectory.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IExecAction> for ::windows_core::IUnknown {
    fn from(value: IExecAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IExecAction> for ::windows_core::IUnknown {
    fn from(value: &IExecAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IExecAction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IExecAction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IExecAction> for super::Com::IDispatch {
    fn from(value: IExecAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IExecAction> for super::Com::IDispatch {
    fn from(value: &IExecAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IExecAction {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IExecAction {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IExecAction> for IAction {
    fn from(value: IExecAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IExecAction> for IAction {
    fn from(value: &IExecAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IAction> for IExecAction {
    fn into_param(self) -> ::windows_core::Param<'a, IAction> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IAction> for &'a IExecAction {
    fn into_param(self) -> ::windows_core::Param<'a, IAction> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IExecAction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IExecAction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IExecAction {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IExecAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExecAction").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IExecAction {
    type Vtable = IExecAction_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4c3d624d_fd6b_49a3_b9b7_09cb3cd3f047);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IExecAction_Vtbl {
    pub base__: IAction_Vtbl,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppath: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Arguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pargument: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, argument: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub WorkingDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pworkingdirectory: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetWorkingDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, workingdirectory: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IExecAction2(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IExecAction2 {
    pub unsafe fn Id(&self, pid: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    pub unsafe fn SetId<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, id: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetId)(::windows_core::Interface::as_raw(self), id.into_param().abi()).ok()
    }
    pub unsafe fn Type(&self, ptype: *mut TASK_ACTION_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    pub unsafe fn Path(&self, ppath: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Path)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppath)).ok()
    }
    pub unsafe fn SetPath<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, path: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetPath)(::windows_core::Interface::as_raw(self), path.into_param().abi()).ok()
    }
    pub unsafe fn Arguments(&self, pargument: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Arguments)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pargument)).ok()
    }
    pub unsafe fn SetArguments<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, argument: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetArguments)(::windows_core::Interface::as_raw(self), argument.into_param().abi()).ok()
    }
    pub unsafe fn WorkingDirectory(&self, pworkingdirectory: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.WorkingDirectory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pworkingdirectory)).ok()
    }
    pub unsafe fn SetWorkingDirectory<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, workingdirectory: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetWorkingDirectory)(::windows_core::Interface::as_raw(self), workingdirectory.into_param().abi()).ok()
    }
    pub unsafe fn HideAppWindow(&self, phideappwindow: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).HideAppWindow)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(phideappwindow)).ok()
    }
    pub unsafe fn SetHideAppWindow(&self, hideappwindow: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetHideAppWindow)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hideappwindow)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IExecAction2> for ::windows_core::IUnknown {
    fn from(value: IExecAction2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IExecAction2> for ::windows_core::IUnknown {
    fn from(value: &IExecAction2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IExecAction2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IExecAction2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IExecAction2> for super::Com::IDispatch {
    fn from(value: IExecAction2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IExecAction2> for super::Com::IDispatch {
    fn from(value: &IExecAction2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IExecAction2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IExecAction2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IExecAction2> for IAction {
    fn from(value: IExecAction2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IExecAction2> for IAction {
    fn from(value: &IExecAction2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IAction> for IExecAction2 {
    fn into_param(self) -> ::windows_core::Param<'a, IAction> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IAction> for &'a IExecAction2 {
    fn into_param(self) -> ::windows_core::Param<'a, IAction> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IExecAction2> for IExecAction {
    fn from(value: IExecAction2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IExecAction2> for IExecAction {
    fn from(value: &IExecAction2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IExecAction> for IExecAction2 {
    fn into_param(self) -> ::windows_core::Param<'a, IExecAction> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IExecAction> for &'a IExecAction2 {
    fn into_param(self) -> ::windows_core::Param<'a, IExecAction> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IExecAction2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IExecAction2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IExecAction2 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IExecAction2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExecAction2").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IExecAction2 {
    type Vtable = IExecAction2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf2a82542_bda5_4e6b_9143_e2bf4f8987b6);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IExecAction2_Vtbl {
    pub base__: IExecAction_Vtbl,
    pub HideAppWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phideappwindow: *mut i16) -> ::windows_core::HRESULT,
    pub SetHideAppWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hideappwindow: i16) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IIdleSettings(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IIdleSettings {
    pub unsafe fn IdleDuration(&self, pdelay: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IdleDuration)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdelay)).ok()
    }
    pub unsafe fn SetIdleDuration<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, delay: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIdleDuration)(::windows_core::Interface::as_raw(self), delay.into_param().abi()).ok()
    }
    pub unsafe fn WaitTimeout(&self, ptimeout: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WaitTimeout)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptimeout)).ok()
    }
    pub unsafe fn SetWaitTimeout<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, timeout: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetWaitTimeout)(::windows_core::Interface::as_raw(self), timeout.into_param().abi()).ok()
    }
    pub unsafe fn StopOnIdleEnd(&self, pstop: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StopOnIdleEnd)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pstop)).ok()
    }
    pub unsafe fn SetStopOnIdleEnd(&self, stop: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStopOnIdleEnd)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(stop)).ok()
    }
    pub unsafe fn RestartOnIdle(&self, prestart: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RestartOnIdle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(prestart)).ok()
    }
    pub unsafe fn SetRestartOnIdle(&self, restart: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRestartOnIdle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(restart)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IIdleSettings> for ::windows_core::IUnknown {
    fn from(value: IIdleSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IIdleSettings> for ::windows_core::IUnknown {
    fn from(value: &IIdleSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IIdleSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IIdleSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IIdleSettings> for super::Com::IDispatch {
    fn from(value: IIdleSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IIdleSettings> for super::Com::IDispatch {
    fn from(value: &IIdleSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IIdleSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IIdleSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IIdleSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IIdleSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IIdleSettings {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IIdleSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IIdleSettings").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IIdleSettings {
    type Vtable = IIdleSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x84594461_0053_4342_a8fd_088fabf11f32);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IIdleSettings_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub IdleDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdelay: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetIdleDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, delay: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub WaitTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptimeout: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetWaitTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeout: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub StopOnIdleEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstop: *mut i16) -> ::windows_core::HRESULT,
    pub SetStopOnIdleEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stop: i16) -> ::windows_core::HRESULT,
    pub RestartOnIdle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prestart: *mut i16) -> ::windows_core::HRESULT,
    pub SetRestartOnIdle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, restart: i16) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IIdleTrigger(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IIdleTrigger {
    pub unsafe fn Type(&self, ptype: *mut TASK_TRIGGER_TYPE2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    pub unsafe fn Id(&self, pid: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    pub unsafe fn SetId<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, id: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetId)(::windows_core::Interface::as_raw(self), id.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Repetition(&self) -> ::windows_core::Result<IRepetitionPattern> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Repetition)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRepetitionPattern>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetRepetition<'a, Param0: ::windows_core::IntoParam<'a, IRepetitionPattern>>(&self, prepeat: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRepetition)(::windows_core::Interface::as_raw(self), prepeat.into_param().abi()).ok()
    }
    pub unsafe fn ExecutionTimeLimit(&self, ptimelimit: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ExecutionTimeLimit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptimelimit)).ok()
    }
    pub unsafe fn SetExecutionTimeLimit<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, timelimit: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetExecutionTimeLimit)(::windows_core::Interface::as_raw(self), timelimit.into_param().abi()).ok()
    }
    pub unsafe fn StartBoundary(&self, pstart: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StartBoundary)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pstart)).ok()
    }
    pub unsafe fn SetStartBoundary<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, start: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetStartBoundary)(::windows_core::Interface::as_raw(self), start.into_param().abi()).ok()
    }
    pub unsafe fn EndBoundary(&self, pend: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EndBoundary)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pend)).ok()
    }
    pub unsafe fn SetEndBoundary<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, end: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEndBoundary)(::windows_core::Interface::as_raw(self), end.into_param().abi()).ok()
    }
    pub unsafe fn Enabled(&self, penabled: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Enabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(penabled)).ok()
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enabled)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IIdleTrigger> for ::windows_core::IUnknown {
    fn from(value: IIdleTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IIdleTrigger> for ::windows_core::IUnknown {
    fn from(value: &IIdleTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IIdleTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IIdleTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IIdleTrigger> for super::Com::IDispatch {
    fn from(value: IIdleTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IIdleTrigger> for super::Com::IDispatch {
    fn from(value: &IIdleTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IIdleTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IIdleTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IIdleTrigger> for ITrigger {
    fn from(value: IIdleTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IIdleTrigger> for ITrigger {
    fn from(value: &IIdleTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ITrigger> for IIdleTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ITrigger> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ITrigger> for &'a IIdleTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ITrigger> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IIdleTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IIdleTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IIdleTrigger {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IIdleTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IIdleTrigger").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IIdleTrigger {
    type Vtable = IIdleTrigger_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd537d2b0_9fb3_4d34_9739_1ff5ce7b1ef3);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IIdleTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ILogonTrigger(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ILogonTrigger {
    pub unsafe fn Type(&self, ptype: *mut TASK_TRIGGER_TYPE2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    pub unsafe fn Id(&self, pid: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    pub unsafe fn SetId<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, id: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetId)(::windows_core::Interface::as_raw(self), id.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Repetition(&self) -> ::windows_core::Result<IRepetitionPattern> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Repetition)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRepetitionPattern>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetRepetition<'a, Param0: ::windows_core::IntoParam<'a, IRepetitionPattern>>(&self, prepeat: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRepetition)(::windows_core::Interface::as_raw(self), prepeat.into_param().abi()).ok()
    }
    pub unsafe fn ExecutionTimeLimit(&self, ptimelimit: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ExecutionTimeLimit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptimelimit)).ok()
    }
    pub unsafe fn SetExecutionTimeLimit<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, timelimit: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetExecutionTimeLimit)(::windows_core::Interface::as_raw(self), timelimit.into_param().abi()).ok()
    }
    pub unsafe fn StartBoundary(&self, pstart: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StartBoundary)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pstart)).ok()
    }
    pub unsafe fn SetStartBoundary<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, start: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetStartBoundary)(::windows_core::Interface::as_raw(self), start.into_param().abi()).ok()
    }
    pub unsafe fn EndBoundary(&self, pend: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EndBoundary)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pend)).ok()
    }
    pub unsafe fn SetEndBoundary<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, end: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEndBoundary)(::windows_core::Interface::as_raw(self), end.into_param().abi()).ok()
    }
    pub unsafe fn Enabled(&self, penabled: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Enabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(penabled)).ok()
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enabled)).ok()
    }
    pub unsafe fn Delay(&self, pdelay: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delay)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdelay)).ok()
    }
    pub unsafe fn SetDelay<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, delay: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDelay)(::windows_core::Interface::as_raw(self), delay.into_param().abi()).ok()
    }
    pub unsafe fn UserId(&self, puser: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UserId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(puser)).ok()
    }
    pub unsafe fn SetUserId<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, user: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetUserId)(::windows_core::Interface::as_raw(self), user.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ILogonTrigger> for ::windows_core::IUnknown {
    fn from(value: ILogonTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ILogonTrigger> for ::windows_core::IUnknown {
    fn from(value: &ILogonTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ILogonTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ILogonTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ILogonTrigger> for super::Com::IDispatch {
    fn from(value: ILogonTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ILogonTrigger> for super::Com::IDispatch {
    fn from(value: &ILogonTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ILogonTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ILogonTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ILogonTrigger> for ITrigger {
    fn from(value: ILogonTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ILogonTrigger> for ITrigger {
    fn from(value: &ILogonTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ITrigger> for ILogonTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ITrigger> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ITrigger> for &'a ILogonTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ITrigger> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ILogonTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ILogonTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ILogonTrigger {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ILogonTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILogonTrigger").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ILogonTrigger {
    type Vtable = ILogonTrigger_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x72dade38_fae4_4b3e_baf4_5d009af02b1c);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ILogonTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub Delay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdelay: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, delay: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub UserId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puser: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetUserId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMaintenanceSettings(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMaintenanceSettings {
    pub unsafe fn SetPeriod<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPeriod)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    pub unsafe fn Period(&self, target: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Period)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(target)).ok()
    }
    pub unsafe fn SetDeadline<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDeadline)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    pub unsafe fn Deadline(&self, target: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Deadline)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(target)).ok()
    }
    pub unsafe fn SetExclusive(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetExclusive)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    pub unsafe fn Exclusive(&self, target: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Exclusive)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(target)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMaintenanceSettings> for ::windows_core::IUnknown {
    fn from(value: IMaintenanceSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMaintenanceSettings> for ::windows_core::IUnknown {
    fn from(value: &IMaintenanceSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMaintenanceSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMaintenanceSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMaintenanceSettings> for super::Com::IDispatch {
    fn from(value: IMaintenanceSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMaintenanceSettings> for super::Com::IDispatch {
    fn from(value: &IMaintenanceSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMaintenanceSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMaintenanceSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMaintenanceSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMaintenanceSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMaintenanceSettings {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMaintenanceSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMaintenanceSettings").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMaintenanceSettings {
    type Vtable = IMaintenanceSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa6024fa8_9652_4adb_a6bf_5cfcd877a7ba);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMaintenanceSettings_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub SetPeriod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Period: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetDeadline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Deadline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetExclusive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows_core::HRESULT,
    pub Exclusive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: *mut i16) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMonthlyDOWTrigger(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMonthlyDOWTrigger {
    pub unsafe fn Type(&self, ptype: *mut TASK_TRIGGER_TYPE2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    pub unsafe fn Id(&self, pid: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    pub unsafe fn SetId<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, id: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetId)(::windows_core::Interface::as_raw(self), id.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Repetition(&self) -> ::windows_core::Result<IRepetitionPattern> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Repetition)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRepetitionPattern>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetRepetition<'a, Param0: ::windows_core::IntoParam<'a, IRepetitionPattern>>(&self, prepeat: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRepetition)(::windows_core::Interface::as_raw(self), prepeat.into_param().abi()).ok()
    }
    pub unsafe fn ExecutionTimeLimit(&self, ptimelimit: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ExecutionTimeLimit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptimelimit)).ok()
    }
    pub unsafe fn SetExecutionTimeLimit<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, timelimit: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetExecutionTimeLimit)(::windows_core::Interface::as_raw(self), timelimit.into_param().abi()).ok()
    }
    pub unsafe fn StartBoundary(&self, pstart: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StartBoundary)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pstart)).ok()
    }
    pub unsafe fn SetStartBoundary<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, start: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetStartBoundary)(::windows_core::Interface::as_raw(self), start.into_param().abi()).ok()
    }
    pub unsafe fn EndBoundary(&self, pend: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EndBoundary)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pend)).ok()
    }
    pub unsafe fn SetEndBoundary<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, end: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEndBoundary)(::windows_core::Interface::as_raw(self), end.into_param().abi()).ok()
    }
    pub unsafe fn Enabled(&self, penabled: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Enabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(penabled)).ok()
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enabled)).ok()
    }
    pub unsafe fn DaysOfWeek(&self, pdays: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DaysOfWeek)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdays)).ok()
    }
    pub unsafe fn SetDaysOfWeek(&self, days: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDaysOfWeek)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(days)).ok()
    }
    pub unsafe fn WeeksOfMonth(&self, pweeks: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WeeksOfMonth)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pweeks)).ok()
    }
    pub unsafe fn SetWeeksOfMonth(&self, weeks: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetWeeksOfMonth)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(weeks)).ok()
    }
    pub unsafe fn MonthsOfYear(&self, pmonths: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MonthsOfYear)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pmonths)).ok()
    }
    pub unsafe fn SetMonthsOfYear(&self, months: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMonthsOfYear)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(months)).ok()
    }
    pub unsafe fn RunOnLastWeekOfMonth(&self, plastweek: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RunOnLastWeekOfMonth)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(plastweek)).ok()
    }
    pub unsafe fn SetRunOnLastWeekOfMonth(&self, lastweek: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRunOnLastWeekOfMonth)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lastweek)).ok()
    }
    pub unsafe fn RandomDelay(&self, prandomdelay: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RandomDelay)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(prandomdelay)).ok()
    }
    pub unsafe fn SetRandomDelay<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, randomdelay: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRandomDelay)(::windows_core::Interface::as_raw(self), randomdelay.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMonthlyDOWTrigger> for ::windows_core::IUnknown {
    fn from(value: IMonthlyDOWTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMonthlyDOWTrigger> for ::windows_core::IUnknown {
    fn from(value: &IMonthlyDOWTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMonthlyDOWTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMonthlyDOWTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMonthlyDOWTrigger> for super::Com::IDispatch {
    fn from(value: IMonthlyDOWTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMonthlyDOWTrigger> for super::Com::IDispatch {
    fn from(value: &IMonthlyDOWTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMonthlyDOWTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMonthlyDOWTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMonthlyDOWTrigger> for ITrigger {
    fn from(value: IMonthlyDOWTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMonthlyDOWTrigger> for ITrigger {
    fn from(value: &IMonthlyDOWTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ITrigger> for IMonthlyDOWTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ITrigger> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ITrigger> for &'a IMonthlyDOWTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ITrigger> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMonthlyDOWTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMonthlyDOWTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMonthlyDOWTrigger {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMonthlyDOWTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMonthlyDOWTrigger").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMonthlyDOWTrigger {
    type Vtable = IMonthlyDOWTrigger_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x77d025a3_90fa_43aa_b52e_cda5499b946a);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMonthlyDOWTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub DaysOfWeek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdays: *mut i16) -> ::windows_core::HRESULT,
    pub SetDaysOfWeek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, days: i16) -> ::windows_core::HRESULT,
    pub WeeksOfMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pweeks: *mut i16) -> ::windows_core::HRESULT,
    pub SetWeeksOfMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, weeks: i16) -> ::windows_core::HRESULT,
    pub MonthsOfYear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmonths: *mut i16) -> ::windows_core::HRESULT,
    pub SetMonthsOfYear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, months: i16) -> ::windows_core::HRESULT,
    pub RunOnLastWeekOfMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plastweek: *mut i16) -> ::windows_core::HRESULT,
    pub SetRunOnLastWeekOfMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastweek: i16) -> ::windows_core::HRESULT,
    pub RandomDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prandomdelay: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetRandomDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, randomdelay: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMonthlyTrigger(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMonthlyTrigger {
    pub unsafe fn Type(&self, ptype: *mut TASK_TRIGGER_TYPE2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    pub unsafe fn Id(&self, pid: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    pub unsafe fn SetId<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, id: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetId)(::windows_core::Interface::as_raw(self), id.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Repetition(&self) -> ::windows_core::Result<IRepetitionPattern> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Repetition)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRepetitionPattern>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetRepetition<'a, Param0: ::windows_core::IntoParam<'a, IRepetitionPattern>>(&self, prepeat: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRepetition)(::windows_core::Interface::as_raw(self), prepeat.into_param().abi()).ok()
    }
    pub unsafe fn ExecutionTimeLimit(&self, ptimelimit: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ExecutionTimeLimit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptimelimit)).ok()
    }
    pub unsafe fn SetExecutionTimeLimit<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, timelimit: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetExecutionTimeLimit)(::windows_core::Interface::as_raw(self), timelimit.into_param().abi()).ok()
    }
    pub unsafe fn StartBoundary(&self, pstart: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StartBoundary)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pstart)).ok()
    }
    pub unsafe fn SetStartBoundary<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, start: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetStartBoundary)(::windows_core::Interface::as_raw(self), start.into_param().abi()).ok()
    }
    pub unsafe fn EndBoundary(&self, pend: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EndBoundary)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pend)).ok()
    }
    pub unsafe fn SetEndBoundary<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, end: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEndBoundary)(::windows_core::Interface::as_raw(self), end.into_param().abi()).ok()
    }
    pub unsafe fn Enabled(&self, penabled: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Enabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(penabled)).ok()
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enabled)).ok()
    }
    pub unsafe fn DaysOfMonth(&self, pdays: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DaysOfMonth)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdays)).ok()
    }
    pub unsafe fn SetDaysOfMonth(&self, days: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDaysOfMonth)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(days)).ok()
    }
    pub unsafe fn MonthsOfYear(&self, pmonths: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MonthsOfYear)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pmonths)).ok()
    }
    pub unsafe fn SetMonthsOfYear(&self, months: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMonthsOfYear)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(months)).ok()
    }
    pub unsafe fn RunOnLastDayOfMonth(&self, plastday: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RunOnLastDayOfMonth)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(plastday)).ok()
    }
    pub unsafe fn SetRunOnLastDayOfMonth(&self, lastday: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRunOnLastDayOfMonth)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lastday)).ok()
    }
    pub unsafe fn RandomDelay(&self, prandomdelay: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RandomDelay)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(prandomdelay)).ok()
    }
    pub unsafe fn SetRandomDelay<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, randomdelay: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRandomDelay)(::windows_core::Interface::as_raw(self), randomdelay.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMonthlyTrigger> for ::windows_core::IUnknown {
    fn from(value: IMonthlyTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMonthlyTrigger> for ::windows_core::IUnknown {
    fn from(value: &IMonthlyTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMonthlyTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMonthlyTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMonthlyTrigger> for super::Com::IDispatch {
    fn from(value: IMonthlyTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMonthlyTrigger> for super::Com::IDispatch {
    fn from(value: &IMonthlyTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMonthlyTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMonthlyTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMonthlyTrigger> for ITrigger {
    fn from(value: IMonthlyTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMonthlyTrigger> for ITrigger {
    fn from(value: &IMonthlyTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ITrigger> for IMonthlyTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ITrigger> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ITrigger> for &'a IMonthlyTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ITrigger> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMonthlyTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMonthlyTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMonthlyTrigger {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMonthlyTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMonthlyTrigger").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMonthlyTrigger {
    type Vtable = IMonthlyTrigger_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x97c45ef1_6b02_4a1a_9c0e_1ebfba1500ac);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMonthlyTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub DaysOfMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdays: *mut i32) -> ::windows_core::HRESULT,
    pub SetDaysOfMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, days: i32) -> ::windows_core::HRESULT,
    pub MonthsOfYear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmonths: *mut i16) -> ::windows_core::HRESULT,
    pub SetMonthsOfYear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, months: i16) -> ::windows_core::HRESULT,
    pub RunOnLastDayOfMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plastday: *mut i16) -> ::windows_core::HRESULT,
    pub SetRunOnLastDayOfMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastday: i16) -> ::windows_core::HRESULT,
    pub RandomDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prandomdelay: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetRandomDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, randomdelay: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct INetworkSettings(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl INetworkSettings {
    pub unsafe fn Name(&self, pname: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pname)).ok()
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn Id(&self, pid: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    pub unsafe fn SetId<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, id: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetId)(::windows_core::Interface::as_raw(self), id.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetworkSettings> for ::windows_core::IUnknown {
    fn from(value: INetworkSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetworkSettings> for ::windows_core::IUnknown {
    fn from(value: &INetworkSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INetworkSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INetworkSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetworkSettings> for super::Com::IDispatch {
    fn from(value: INetworkSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetworkSettings> for super::Com::IDispatch {
    fn from(value: &INetworkSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for INetworkSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a INetworkSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for INetworkSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for INetworkSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for INetworkSettings {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for INetworkSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetworkSettings").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for INetworkSettings {
    type Vtable = INetworkSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f7dea84_c30b_4245_80b6_00e9f646f1b4);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct INetworkSettings_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IPrincipal(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IPrincipal {
    pub unsafe fn Id(&self, pid: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    pub unsafe fn SetId<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, id: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetId)(::windows_core::Interface::as_raw(self), id.into_param().abi()).ok()
    }
    pub unsafe fn DisplayName(&self, pname: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DisplayName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pname)).ok()
    }
    pub unsafe fn SetDisplayName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDisplayName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn UserId(&self, puser: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UserId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(puser)).ok()
    }
    pub unsafe fn SetUserId<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, user: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetUserId)(::windows_core::Interface::as_raw(self), user.into_param().abi()).ok()
    }
    pub unsafe fn LogonType(&self, plogon: *mut TASK_LOGON_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LogonType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(plogon)).ok()
    }
    pub unsafe fn SetLogonType(&self, logon: TASK_LOGON_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLogonType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(logon)).ok()
    }
    pub unsafe fn GroupId(&self, pgroup: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GroupId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pgroup)).ok()
    }
    pub unsafe fn SetGroupId<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, group: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetGroupId)(::windows_core::Interface::as_raw(self), group.into_param().abi()).ok()
    }
    pub unsafe fn RunLevel(&self, prunlevel: *mut TASK_RUNLEVEL_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RunLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(prunlevel)).ok()
    }
    pub unsafe fn SetRunLevel(&self, runlevel: TASK_RUNLEVEL_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRunLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(runlevel)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IPrincipal> for ::windows_core::IUnknown {
    fn from(value: IPrincipal) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IPrincipal> for ::windows_core::IUnknown {
    fn from(value: &IPrincipal) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPrincipal {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPrincipal {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IPrincipal> for super::Com::IDispatch {
    fn from(value: IPrincipal) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IPrincipal> for super::Com::IDispatch {
    fn from(value: &IPrincipal) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IPrincipal {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IPrincipal {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IPrincipal {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IPrincipal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IPrincipal {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IPrincipal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrincipal").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IPrincipal {
    type Vtable = IPrincipal_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd98d51e5_c9b4_496a_a9c1_18980261cf0f);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrincipal_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub UserId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puser: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetUserId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub LogonType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plogon: *mut TASK_LOGON_TYPE) -> ::windows_core::HRESULT,
    pub SetLogonType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logon: TASK_LOGON_TYPE) -> ::windows_core::HRESULT,
    pub GroupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgroup: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetGroupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, group: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub RunLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prunlevel: *mut TASK_RUNLEVEL_TYPE) -> ::windows_core::HRESULT,
    pub SetRunLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, runlevel: TASK_RUNLEVEL_TYPE) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IPrincipal2(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IPrincipal2 {
    pub unsafe fn ProcessTokenSidType(&self, pprocesstokensidtype: *mut TASK_PROCESSTOKENSID_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ProcessTokenSidType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pprocesstokensidtype)).ok()
    }
    pub unsafe fn SetProcessTokenSidType(&self, processtokensidtype: TASK_PROCESSTOKENSID_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProcessTokenSidType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(processtokensidtype)).ok()
    }
    pub unsafe fn RequiredPrivilegeCount(&self, pcount: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RequiredPrivilegeCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcount)).ok()
    }
    pub unsafe fn get_RequiredPrivilege(&self, index: i32, pprivilege: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).get_RequiredPrivilege)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(pprivilege)).ok()
    }
    pub unsafe fn AddRequiredPrivilege<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, privilege: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddRequiredPrivilege)(::windows_core::Interface::as_raw(self), privilege.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IPrincipal2> for ::windows_core::IUnknown {
    fn from(value: IPrincipal2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IPrincipal2> for ::windows_core::IUnknown {
    fn from(value: &IPrincipal2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPrincipal2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPrincipal2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IPrincipal2> for super::Com::IDispatch {
    fn from(value: IPrincipal2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IPrincipal2> for super::Com::IDispatch {
    fn from(value: &IPrincipal2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IPrincipal2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IPrincipal2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IPrincipal2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IPrincipal2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IPrincipal2 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IPrincipal2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrincipal2").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IPrincipal2 {
    type Vtable = IPrincipal2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x248919ae_e345_4a6d_8aeb_e0d3165c904e);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrincipal2_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ProcessTokenSidType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprocesstokensidtype: *mut TASK_PROCESSTOKENSID_TYPE) -> ::windows_core::HRESULT,
    pub SetProcessTokenSidType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, processtokensidtype: TASK_PROCESSTOKENSID_TYPE) -> ::windows_core::HRESULT,
    pub RequiredPrivilegeCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows_core::HRESULT,
    pub get_RequiredPrivilege: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pprivilege: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub AddRequiredPrivilege: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, privilege: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IProvideTaskPage(::windows_core::IUnknown);
impl IProvideTaskPage {
    #[cfg(feature = "win32-ui")]
    pub unsafe fn GetPage<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, tptype: TASKPAGE, fpersistchanges: Param1) -> ::windows_core::Result<::win32_ui::Controls::HPROPSHEETPAGE> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_ui::Controls::HPROPSHEETPAGE>::zeroed();
        (::windows_core::Interface::vtable(self).GetPage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(tptype), fpersistchanges.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_ui::Controls::HPROPSHEETPAGE>(result__)
    }
}
impl ::core::convert::From<IProvideTaskPage> for ::windows_core::IUnknown {
    fn from(value: IProvideTaskPage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProvideTaskPage> for ::windows_core::IUnknown {
    fn from(value: &IProvideTaskPage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IProvideTaskPage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IProvideTaskPage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IProvideTaskPage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProvideTaskPage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProvideTaskPage {}
impl ::core::fmt::Debug for IProvideTaskPage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProvideTaskPage").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IProvideTaskPage {
    type Vtable = IProvideTaskPage_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4086658a_cbbb_11cf_b604_00c04fd8d565);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvideTaskPage_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "win32-ui")]
    pub GetPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tptype: TASKPAGE, fpersistchanges: ::win32_foundation::BOOL, phpage: *mut ::win32_ui::Controls::HPROPSHEETPAGE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    GetPage: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IRegisteredTask(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IRegisteredTask {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Path(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Path)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn State(&self) -> ::windows_core::Result<TASK_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::<TASK_STATE>::zeroed();
        (::windows_core::Interface::vtable(self).State)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<TASK_STATE>(result__)
    }
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).Enabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enabled)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Run<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, params: Param0) -> ::windows_core::Result<IRunningTask> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Run)(::windows_core::Interface::as_raw(self), params.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRunningTask>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn RunEx<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, params: Param0, flags: i32, sessionid: i32, user: Param3) -> ::windows_core::Result<IRunningTask> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).RunEx)(::windows_core::Interface::as_raw(self), params.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(sessionid), user.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRunningTask>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetInstances(&self, flags: i32) -> ::windows_core::Result<IRunningTaskCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetInstances)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(flags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRunningTaskCollection>(result__)
    }
    pub unsafe fn LastRunTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).LastRunTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn LastTaskResult(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).LastTaskResult)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn NumberOfMissedRuns(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).NumberOfMissedRuns)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn NextRunTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).NextRunTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Definition(&self) -> ::windows_core::Result<ITaskDefinition> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Definition)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITaskDefinition>(result__)
    }
    pub unsafe fn Xml(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Xml)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetSecurityDescriptor(&self, securityinformation: i32) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetSecurityDescriptor)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(securityinformation), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetSecurityDescriptor<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, sddl: Param0, flags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSecurityDescriptor)(::windows_core::Interface::as_raw(self), sddl.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    pub unsafe fn Stop(&self, flags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Stop)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(flags)).ok()
    }
    pub unsafe fn GetRunTimes(&self, pststart: *const ::win32_foundation::SYSTEMTIME, pstend: *const ::win32_foundation::SYSTEMTIME, pcount: *mut u32, pruntimes: *mut *mut ::win32_foundation::SYSTEMTIME) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRunTimes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pststart), ::core::mem::transmute(pstend), ::core::mem::transmute(pcount), ::core::mem::transmute(pruntimes)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRegisteredTask> for ::windows_core::IUnknown {
    fn from(value: IRegisteredTask) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRegisteredTask> for ::windows_core::IUnknown {
    fn from(value: &IRegisteredTask) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRegisteredTask {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRegisteredTask {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRegisteredTask> for super::Com::IDispatch {
    fn from(value: IRegisteredTask) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRegisteredTask> for super::Com::IDispatch {
    fn from(value: &IRegisteredTask) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IRegisteredTask {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IRegisteredTask {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IRegisteredTask {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IRegisteredTask {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IRegisteredTask {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IRegisteredTask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRegisteredTask").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IRegisteredTask {
    type Vtable = IRegisteredTask_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9c86f320_dee3_4dd1_b972_a303f26b061e);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IRegisteredTask_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppath: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstate: *mut TASK_STATE) -> ::windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penabled: *mut i16) -> ::windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Run: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, params: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pprunningtask: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Run: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub RunEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, params: ::core::mem::ManuallyDrop<super::Com::VARIANT>, flags: i32, sessionid: i32, user: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pprunningtask: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    RunEx: usize,
    #[cfg(feature = "win32-system")]
    pub GetInstances: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: i32, pprunningtasks: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetInstances: usize,
    pub LastRunTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plastruntime: *mut f64) -> ::windows_core::HRESULT,
    pub LastTaskResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plasttaskresult: *mut i32) -> ::windows_core::HRESULT,
    pub NumberOfMissedRuns: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnumberofmissedruns: *mut i32) -> ::windows_core::HRESULT,
    pub NextRunTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnextruntime: *mut f64) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Definition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdefinition: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Definition: usize,
    pub Xml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pxml: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetSecurityDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, securityinformation: i32, psddl: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetSecurityDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sddl: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, flags: i32) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows_core::HRESULT,
    pub GetRunTimes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pststart: *const ::win32_foundation::SYSTEMTIME, pstend: *const ::win32_foundation::SYSTEMTIME, pcount: *mut u32, pruntimes: *mut *mut ::win32_foundation::SYSTEMTIME) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IRegisteredTaskCollection(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IRegisteredTaskCollection {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn get_Item<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows_core::Result<IRegisteredTask> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), index.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRegisteredTask>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRegisteredTaskCollection> for ::windows_core::IUnknown {
    fn from(value: IRegisteredTaskCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRegisteredTaskCollection> for ::windows_core::IUnknown {
    fn from(value: &IRegisteredTaskCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRegisteredTaskCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRegisteredTaskCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRegisteredTaskCollection> for super::Com::IDispatch {
    fn from(value: IRegisteredTaskCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRegisteredTaskCollection> for super::Com::IDispatch {
    fn from(value: &IRegisteredTaskCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IRegisteredTaskCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IRegisteredTaskCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IRegisteredTaskCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IRegisteredTaskCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IRegisteredTaskCollection {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IRegisteredTaskCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRegisteredTaskCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IRegisteredTaskCollection {
    type Vtable = IRegisteredTaskCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x86627eb4_42a7_41e4_a4d9_ac33a72f2d52);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IRegisteredTaskCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppregisteredtask: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IRegistrationInfo(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IRegistrationInfo {
    pub unsafe fn Description(&self, pdescription: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdescription)).ok()
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, description: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Author(&self, pauthor: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Author)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pauthor)).ok()
    }
    pub unsafe fn SetAuthor<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, author: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAuthor)(::windows_core::Interface::as_raw(self), author.into_param().abi()).ok()
    }
    pub unsafe fn Version(&self, pversion: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Version)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pversion)).ok()
    }
    pub unsafe fn SetVersion<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, version: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetVersion)(::windows_core::Interface::as_raw(self), version.into_param().abi()).ok()
    }
    pub unsafe fn Date(&self, pdate: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Date)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdate)).ok()
    }
    pub unsafe fn SetDate<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, date: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDate)(::windows_core::Interface::as_raw(self), date.into_param().abi()).ok()
    }
    pub unsafe fn Documentation(&self, pdocumentation: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Documentation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdocumentation)).ok()
    }
    pub unsafe fn SetDocumentation<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, documentation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDocumentation)(::windows_core::Interface::as_raw(self), documentation.into_param().abi()).ok()
    }
    pub unsafe fn XmlText(&self, ptext: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).XmlText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptext)).ok()
    }
    pub unsafe fn SetXmlText<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, text: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetXmlText)(::windows_core::Interface::as_raw(self), text.into_param().abi()).ok()
    }
    pub unsafe fn URI(&self, puri: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).URI)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(puri)).ok()
    }
    pub unsafe fn SetURI<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, uri: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetURI)(::windows_core::Interface::as_raw(self), uri.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SecurityDescriptor(&self, psddl: *mut super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SecurityDescriptor)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(psddl)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetSecurityDescriptor<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, sddl: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSecurityDescriptor)(::windows_core::Interface::as_raw(self), sddl.into_param().abi()).ok()
    }
    pub unsafe fn Source(&self, psource: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Source)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(psource)).ok()
    }
    pub unsafe fn SetSource<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, source: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSource)(::windows_core::Interface::as_raw(self), source.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRegistrationInfo> for ::windows_core::IUnknown {
    fn from(value: IRegistrationInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRegistrationInfo> for ::windows_core::IUnknown {
    fn from(value: &IRegistrationInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRegistrationInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRegistrationInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRegistrationInfo> for super::Com::IDispatch {
    fn from(value: IRegistrationInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRegistrationInfo> for super::Com::IDispatch {
    fn from(value: &IRegistrationInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IRegistrationInfo {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IRegistrationInfo {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IRegistrationInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IRegistrationInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IRegistrationInfo {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IRegistrationInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRegistrationInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IRegistrationInfo {
    type Vtable = IRegistrationInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x416d8b73_cb41_4ea1_805c_9be9a5ac4a74);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IRegistrationInfo_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdescription: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Author: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pauthor: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetAuthor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, author: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pversion: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, version: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Date: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdate: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, date: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Documentation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdocumentation: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetDocumentation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentation: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub XmlText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptext: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetXmlText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub URI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puri: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SecurityDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psddl: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SecurityDescriptor: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetSecurityDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sddl: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetSecurityDescriptor: usize,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psource: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IRegistrationTrigger(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IRegistrationTrigger {
    pub unsafe fn Type(&self, ptype: *mut TASK_TRIGGER_TYPE2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    pub unsafe fn Id(&self, pid: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    pub unsafe fn SetId<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, id: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetId)(::windows_core::Interface::as_raw(self), id.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Repetition(&self) -> ::windows_core::Result<IRepetitionPattern> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Repetition)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRepetitionPattern>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetRepetition<'a, Param0: ::windows_core::IntoParam<'a, IRepetitionPattern>>(&self, prepeat: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRepetition)(::windows_core::Interface::as_raw(self), prepeat.into_param().abi()).ok()
    }
    pub unsafe fn ExecutionTimeLimit(&self, ptimelimit: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ExecutionTimeLimit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptimelimit)).ok()
    }
    pub unsafe fn SetExecutionTimeLimit<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, timelimit: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetExecutionTimeLimit)(::windows_core::Interface::as_raw(self), timelimit.into_param().abi()).ok()
    }
    pub unsafe fn StartBoundary(&self, pstart: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StartBoundary)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pstart)).ok()
    }
    pub unsafe fn SetStartBoundary<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, start: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetStartBoundary)(::windows_core::Interface::as_raw(self), start.into_param().abi()).ok()
    }
    pub unsafe fn EndBoundary(&self, pend: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EndBoundary)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pend)).ok()
    }
    pub unsafe fn SetEndBoundary<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, end: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEndBoundary)(::windows_core::Interface::as_raw(self), end.into_param().abi()).ok()
    }
    pub unsafe fn Enabled(&self, penabled: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Enabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(penabled)).ok()
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enabled)).ok()
    }
    pub unsafe fn Delay(&self, pdelay: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delay)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdelay)).ok()
    }
    pub unsafe fn SetDelay<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, delay: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDelay)(::windows_core::Interface::as_raw(self), delay.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRegistrationTrigger> for ::windows_core::IUnknown {
    fn from(value: IRegistrationTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRegistrationTrigger> for ::windows_core::IUnknown {
    fn from(value: &IRegistrationTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRegistrationTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRegistrationTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRegistrationTrigger> for super::Com::IDispatch {
    fn from(value: IRegistrationTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRegistrationTrigger> for super::Com::IDispatch {
    fn from(value: &IRegistrationTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IRegistrationTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IRegistrationTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRegistrationTrigger> for ITrigger {
    fn from(value: IRegistrationTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRegistrationTrigger> for ITrigger {
    fn from(value: &IRegistrationTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ITrigger> for IRegistrationTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ITrigger> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ITrigger> for &'a IRegistrationTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ITrigger> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IRegistrationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IRegistrationTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IRegistrationTrigger {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IRegistrationTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRegistrationTrigger").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IRegistrationTrigger {
    type Vtable = IRegistrationTrigger_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4c8fec3a_c218_4e0c_b23d_629024db91a2);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IRegistrationTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub Delay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdelay: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, delay: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IRepetitionPattern(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IRepetitionPattern {
    pub unsafe fn Interval(&self, pinterval: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Interval)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinterval)).ok()
    }
    pub unsafe fn SetInterval<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, interval: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInterval)(::windows_core::Interface::as_raw(self), interval.into_param().abi()).ok()
    }
    pub unsafe fn Duration(&self, pduration: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Duration)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pduration)).ok()
    }
    pub unsafe fn SetDuration<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, duration: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDuration)(::windows_core::Interface::as_raw(self), duration.into_param().abi()).ok()
    }
    pub unsafe fn StopAtDurationEnd(&self, pstop: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StopAtDurationEnd)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pstop)).ok()
    }
    pub unsafe fn SetStopAtDurationEnd(&self, stop: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStopAtDurationEnd)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(stop)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRepetitionPattern> for ::windows_core::IUnknown {
    fn from(value: IRepetitionPattern) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRepetitionPattern> for ::windows_core::IUnknown {
    fn from(value: &IRepetitionPattern) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRepetitionPattern {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRepetitionPattern {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRepetitionPattern> for super::Com::IDispatch {
    fn from(value: IRepetitionPattern) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRepetitionPattern> for super::Com::IDispatch {
    fn from(value: &IRepetitionPattern) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IRepetitionPattern {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IRepetitionPattern {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IRepetitionPattern {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IRepetitionPattern {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IRepetitionPattern {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IRepetitionPattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRepetitionPattern").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IRepetitionPattern {
    type Vtable = IRepetitionPattern_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7fb9acf1_26be_400e_85b5_294b9c75dfd6);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IRepetitionPattern_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Interval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinterval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interval: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pduration: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub StopAtDurationEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstop: *mut i16) -> ::windows_core::HRESULT,
    pub SetStopAtDurationEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stop: i16) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IRunningTask(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IRunningTask {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn InstanceGuid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).InstanceGuid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Path(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Path)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn State(&self) -> ::windows_core::Result<TASK_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::<TASK_STATE>::zeroed();
        (::windows_core::Interface::vtable(self).State)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<TASK_STATE>(result__)
    }
    pub unsafe fn CurrentAction(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).CurrentAction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Stop(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Stop)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnginePID(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).EnginePID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRunningTask> for ::windows_core::IUnknown {
    fn from(value: IRunningTask) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRunningTask> for ::windows_core::IUnknown {
    fn from(value: &IRunningTask) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRunningTask {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRunningTask {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRunningTask> for super::Com::IDispatch {
    fn from(value: IRunningTask) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRunningTask> for super::Com::IDispatch {
    fn from(value: &IRunningTask) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IRunningTask {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IRunningTask {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IRunningTask {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IRunningTask {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IRunningTask {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IRunningTask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRunningTask").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IRunningTask {
    type Vtable = IRunningTask_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x653758fb_7b9a_4f1e_a471_beeb8e9b834e);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IRunningTask_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub InstanceGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppath: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstate: *mut TASK_STATE) -> ::windows_core::HRESULT,
    pub CurrentAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EnginePID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppid: *mut u32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IRunningTaskCollection(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IRunningTaskCollection {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn get_Item<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows_core::Result<IRunningTask> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), index.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRunningTask>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRunningTaskCollection> for ::windows_core::IUnknown {
    fn from(value: IRunningTaskCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRunningTaskCollection> for ::windows_core::IUnknown {
    fn from(value: &IRunningTaskCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRunningTaskCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRunningTaskCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRunningTaskCollection> for super::Com::IDispatch {
    fn from(value: IRunningTaskCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRunningTaskCollection> for super::Com::IDispatch {
    fn from(value: &IRunningTaskCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IRunningTaskCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IRunningTaskCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IRunningTaskCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IRunningTaskCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IRunningTaskCollection {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IRunningTaskCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRunningTaskCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IRunningTaskCollection {
    type Vtable = IRunningTaskCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6a67614b_6828_4fec_aa54_6d52e8f1f2db);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IRunningTaskCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pprunningtask: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IScheduledWorkItem(::windows_core::IUnknown);
impl IScheduledWorkItem {
    pub unsafe fn CreateTrigger(&self, pinewtrigger: *mut u16, pptrigger: *mut ::core::option::Option<ITaskTrigger>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreateTrigger)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinewtrigger), ::core::mem::transmute(pptrigger)).ok()
    }
    pub unsafe fn DeleteTrigger(&self, itrigger: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteTrigger)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(itrigger)).ok()
    }
    pub unsafe fn GetTriggerCount(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).GetTriggerCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn GetTrigger(&self, itrigger: u16) -> ::windows_core::Result<ITaskTrigger> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetTrigger)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(itrigger), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITaskTrigger>(result__)
    }
    pub unsafe fn GetTriggerString(&self, itrigger: u16) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetTriggerString)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(itrigger), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetRunTimes(&self, pstbegin: *const ::win32_foundation::SYSTEMTIME, pstend: *const ::win32_foundation::SYSTEMTIME, pcount: *mut u16, rgsttasktimes: *mut *mut ::win32_foundation::SYSTEMTIME) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRunTimes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pstbegin), ::core::mem::transmute(pstend), ::core::mem::transmute(pcount), ::core::mem::transmute(rgsttasktimes)).ok()
    }
    pub unsafe fn GetNextRunTime(&self, pstnextrun: *mut ::win32_foundation::SYSTEMTIME) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetNextRunTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pstnextrun)).ok()
    }
    pub unsafe fn SetIdleWait(&self, widleminutes: u16, wdeadlineminutes: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIdleWait)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(widleminutes), ::core::mem::transmute(wdeadlineminutes)).ok()
    }
    pub unsafe fn GetIdleWait(&self, pwidleminutes: *mut u16, pwdeadlineminutes: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetIdleWait)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwidleminutes), ::core::mem::transmute(pwdeadlineminutes)).ok()
    }
    pub unsafe fn Run(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Run)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Terminate(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Terminate)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EditWorkItem<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(&self, hparent: Param0, dwreserved: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EditWorkItem)(::windows_core::Interface::as_raw(self), hparent.into_param().abi(), ::core::mem::transmute(dwreserved)).ok()
    }
    pub unsafe fn GetMostRecentRunTime(&self) -> ::windows_core::Result<::win32_foundation::SYSTEMTIME> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::SYSTEMTIME>::zeroed();
        (::windows_core::Interface::vtable(self).GetMostRecentRunTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::SYSTEMTIME>(result__)
    }
    pub unsafe fn GetStatus(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
        (::windows_core::Interface::vtable(self).GetStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::HRESULT>(result__)
    }
    pub unsafe fn GetExitCode(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetExitCode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetComment<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszcomment: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetComment)(::windows_core::Interface::as_raw(self), pwszcomment.into_param().abi()).ok()
    }
    pub unsafe fn GetComment(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetComment)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn SetCreator<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszcreator: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCreator)(::windows_core::Interface::as_raw(self), pwszcreator.into_param().abi()).ok()
    }
    pub unsafe fn GetCreator(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetCreator)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn SetWorkItemData(&self, cbdata: u16, rgbdata: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetWorkItemData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cbdata), ::core::mem::transmute(rgbdata)).ok()
    }
    pub unsafe fn GetWorkItemData(&self, pcbdata: *mut u16, prgbdata: *mut *mut u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetWorkItemData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcbdata), ::core::mem::transmute(prgbdata)).ok()
    }
    pub unsafe fn SetErrorRetryCount(&self, wretrycount: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetErrorRetryCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wretrycount)).ok()
    }
    pub unsafe fn GetErrorRetryCount(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).GetErrorRetryCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn SetErrorRetryInterval(&self, wretryinterval: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetErrorRetryInterval)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wretryinterval)).ok()
    }
    pub unsafe fn GetErrorRetryInterval(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).GetErrorRetryInterval)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn SetFlags(&self, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn GetFlags(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetAccountInformation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszaccountname: Param0, pwszpassword: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAccountInformation)(::windows_core::Interface::as_raw(self), pwszaccountname.into_param().abi(), pwszpassword.into_param().abi()).ok()
    }
    pub unsafe fn GetAccountInformation(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetAccountInformation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
}
impl ::core::convert::From<IScheduledWorkItem> for ::windows_core::IUnknown {
    fn from(value: IScheduledWorkItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IScheduledWorkItem> for ::windows_core::IUnknown {
    fn from(value: &IScheduledWorkItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IScheduledWorkItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IScheduledWorkItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IScheduledWorkItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IScheduledWorkItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IScheduledWorkItem {}
impl ::core::fmt::Debug for IScheduledWorkItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IScheduledWorkItem").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IScheduledWorkItem {
    type Vtable = IScheduledWorkItem_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa6b952f0_a4b1_11d0_997d_00aa006887ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScheduledWorkItem_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub CreateTrigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinewtrigger: *mut u16, pptrigger: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DeleteTrigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itrigger: u16) -> ::windows_core::HRESULT,
    pub GetTriggerCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcount: *mut u16) -> ::windows_core::HRESULT,
    pub GetTrigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itrigger: u16, pptrigger: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetTriggerString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itrigger: u16, ppwsztrigger: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetRunTimes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstbegin: *const ::win32_foundation::SYSTEMTIME, pstend: *const ::win32_foundation::SYSTEMTIME, pcount: *mut u16, rgsttasktimes: *mut *mut ::win32_foundation::SYSTEMTIME) -> ::windows_core::HRESULT,
    pub GetNextRunTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstnextrun: *mut ::win32_foundation::SYSTEMTIME) -> ::windows_core::HRESULT,
    pub SetIdleWait: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, widleminutes: u16, wdeadlineminutes: u16) -> ::windows_core::HRESULT,
    pub GetIdleWait: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwidleminutes: *mut u16, pwdeadlineminutes: *mut u16) -> ::windows_core::HRESULT,
    pub Run: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Terminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EditWorkItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hparent: ::win32_foundation::HWND, dwreserved: u32) -> ::windows_core::HRESULT,
    pub GetMostRecentRunTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstlastrun: *mut ::win32_foundation::SYSTEMTIME) -> ::windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phrstatus: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub GetExitCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwexitcode: *mut u32) -> ::windows_core::HRESULT,
    pub SetComment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszcomment: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetComment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszcomment: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetCreator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszcreator: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetCreator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszcreator: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetWorkItemData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbdata: u16, rgbdata: *const u8) -> ::windows_core::HRESULT,
    pub GetWorkItemData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbdata: *mut u16, prgbdata: *mut *mut u8) -> ::windows_core::HRESULT,
    pub SetErrorRetryCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wretrycount: u16) -> ::windows_core::HRESULT,
    pub GetErrorRetryCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwretrycount: *mut u16) -> ::windows_core::HRESULT,
    pub SetErrorRetryInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wretryinterval: u16) -> ::windows_core::HRESULT,
    pub GetErrorRetryInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwretryinterval: *mut u16) -> ::windows_core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT,
    pub SetAccountInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszaccountname: ::windows_core::PCWSTR, pwszpassword: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetAccountInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszaccountname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ISessionStateChangeTrigger(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ISessionStateChangeTrigger {
    pub unsafe fn Type(&self, ptype: *mut TASK_TRIGGER_TYPE2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    pub unsafe fn Id(&self, pid: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    pub unsafe fn SetId<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, id: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetId)(::windows_core::Interface::as_raw(self), id.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Repetition(&self) -> ::windows_core::Result<IRepetitionPattern> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Repetition)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRepetitionPattern>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetRepetition<'a, Param0: ::windows_core::IntoParam<'a, IRepetitionPattern>>(&self, prepeat: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRepetition)(::windows_core::Interface::as_raw(self), prepeat.into_param().abi()).ok()
    }
    pub unsafe fn ExecutionTimeLimit(&self, ptimelimit: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ExecutionTimeLimit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptimelimit)).ok()
    }
    pub unsafe fn SetExecutionTimeLimit<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, timelimit: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetExecutionTimeLimit)(::windows_core::Interface::as_raw(self), timelimit.into_param().abi()).ok()
    }
    pub unsafe fn StartBoundary(&self, pstart: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StartBoundary)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pstart)).ok()
    }
    pub unsafe fn SetStartBoundary<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, start: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetStartBoundary)(::windows_core::Interface::as_raw(self), start.into_param().abi()).ok()
    }
    pub unsafe fn EndBoundary(&self, pend: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EndBoundary)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pend)).ok()
    }
    pub unsafe fn SetEndBoundary<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, end: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEndBoundary)(::windows_core::Interface::as_raw(self), end.into_param().abi()).ok()
    }
    pub unsafe fn Enabled(&self, penabled: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Enabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(penabled)).ok()
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enabled)).ok()
    }
    pub unsafe fn Delay(&self, pdelay: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delay)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdelay)).ok()
    }
    pub unsafe fn SetDelay<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, delay: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDelay)(::windows_core::Interface::as_raw(self), delay.into_param().abi()).ok()
    }
    pub unsafe fn UserId(&self, puser: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UserId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(puser)).ok()
    }
    pub unsafe fn SetUserId<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, user: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetUserId)(::windows_core::Interface::as_raw(self), user.into_param().abi()).ok()
    }
    pub unsafe fn StateChange(&self, ptype: *mut TASK_SESSION_STATE_CHANGE_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StateChange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    pub unsafe fn SetStateChange(&self, r#type: TASK_SESSION_STATE_CHANGE_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStateChange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(r#type)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISessionStateChangeTrigger> for ::windows_core::IUnknown {
    fn from(value: ISessionStateChangeTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISessionStateChangeTrigger> for ::windows_core::IUnknown {
    fn from(value: &ISessionStateChangeTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISessionStateChangeTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISessionStateChangeTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISessionStateChangeTrigger> for super::Com::IDispatch {
    fn from(value: ISessionStateChangeTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISessionStateChangeTrigger> for super::Com::IDispatch {
    fn from(value: &ISessionStateChangeTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ISessionStateChangeTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ISessionStateChangeTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISessionStateChangeTrigger> for ITrigger {
    fn from(value: ISessionStateChangeTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISessionStateChangeTrigger> for ITrigger {
    fn from(value: &ISessionStateChangeTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ITrigger> for ISessionStateChangeTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ITrigger> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ITrigger> for &'a ISessionStateChangeTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ITrigger> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ISessionStateChangeTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ISessionStateChangeTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ISessionStateChangeTrigger {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ISessionStateChangeTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISessionStateChangeTrigger").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ISessionStateChangeTrigger {
    type Vtable = ISessionStateChangeTrigger_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x754da71b_4385_4475_9dd9_598294fa3641);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ISessionStateChangeTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub Delay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdelay: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, delay: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub UserId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puser: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetUserId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub StateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut TASK_SESSION_STATE_CHANGE_TYPE) -> ::windows_core::HRESULT,
    pub SetStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: TASK_SESSION_STATE_CHANGE_TYPE) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IShowMessageAction(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IShowMessageAction {
    pub unsafe fn Id(&self, pid: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    pub unsafe fn SetId<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, id: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetId)(::windows_core::Interface::as_raw(self), id.into_param().abi()).ok()
    }
    pub unsafe fn Type(&self, ptype: *mut TASK_ACTION_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    pub unsafe fn Title(&self, ptitle: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Title)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptitle)).ok()
    }
    pub unsafe fn SetTitle<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, title: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTitle)(::windows_core::Interface::as_raw(self), title.into_param().abi()).ok()
    }
    pub unsafe fn MessageBody(&self, pmessagebody: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MessageBody)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pmessagebody)).ok()
    }
    pub unsafe fn SetMessageBody<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, messagebody: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMessageBody)(::windows_core::Interface::as_raw(self), messagebody.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IShowMessageAction> for ::windows_core::IUnknown {
    fn from(value: IShowMessageAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IShowMessageAction> for ::windows_core::IUnknown {
    fn from(value: &IShowMessageAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IShowMessageAction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IShowMessageAction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IShowMessageAction> for super::Com::IDispatch {
    fn from(value: IShowMessageAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IShowMessageAction> for super::Com::IDispatch {
    fn from(value: &IShowMessageAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IShowMessageAction {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IShowMessageAction {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IShowMessageAction> for IAction {
    fn from(value: IShowMessageAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IShowMessageAction> for IAction {
    fn from(value: &IShowMessageAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IAction> for IShowMessageAction {
    fn into_param(self) -> ::windows_core::Param<'a, IAction> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IAction> for &'a IShowMessageAction {
    fn into_param(self) -> ::windows_core::Param<'a, IAction> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IShowMessageAction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IShowMessageAction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IShowMessageAction {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IShowMessageAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShowMessageAction").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IShowMessageAction {
    type Vtable = IShowMessageAction_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x505e9e68_af89_46b8_a30f_56162a83d537);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IShowMessageAction_Vtbl {
    pub base__: IAction_Vtbl,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptitle: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub MessageBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmessagebody: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetMessageBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagebody: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITask(::windows_core::IUnknown);
impl ITask {
    pub unsafe fn CreateTrigger(&self, pinewtrigger: *mut u16, pptrigger: *mut ::core::option::Option<ITaskTrigger>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CreateTrigger)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinewtrigger), ::core::mem::transmute(pptrigger)).ok()
    }
    pub unsafe fn DeleteTrigger(&self, itrigger: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteTrigger)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(itrigger)).ok()
    }
    pub unsafe fn GetTriggerCount(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTriggerCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn GetTrigger(&self, itrigger: u16) -> ::windows_core::Result<ITaskTrigger> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTrigger)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(itrigger), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITaskTrigger>(result__)
    }
    pub unsafe fn GetTriggerString(&self, itrigger: u16) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTriggerString)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(itrigger), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetRunTimes(&self, pstbegin: *const ::win32_foundation::SYSTEMTIME, pstend: *const ::win32_foundation::SYSTEMTIME, pcount: *mut u16, rgsttasktimes: *mut *mut ::win32_foundation::SYSTEMTIME) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetRunTimes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pstbegin), ::core::mem::transmute(pstend), ::core::mem::transmute(pcount), ::core::mem::transmute(rgsttasktimes)).ok()
    }
    pub unsafe fn GetNextRunTime(&self, pstnextrun: *mut ::win32_foundation::SYSTEMTIME) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetNextRunTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pstnextrun)).ok()
    }
    pub unsafe fn SetIdleWait(&self, widleminutes: u16, wdeadlineminutes: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetIdleWait)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(widleminutes), ::core::mem::transmute(wdeadlineminutes)).ok()
    }
    pub unsafe fn GetIdleWait(&self, pwidleminutes: *mut u16, pwdeadlineminutes: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetIdleWait)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwidleminutes), ::core::mem::transmute(pwdeadlineminutes)).ok()
    }
    pub unsafe fn Run(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Run)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Terminate(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Terminate)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EditWorkItem<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(&self, hparent: Param0, dwreserved: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EditWorkItem)(::windows_core::Interface::as_raw(self), hparent.into_param().abi(), ::core::mem::transmute(dwreserved)).ok()
    }
    pub unsafe fn GetMostRecentRunTime(&self) -> ::windows_core::Result<::win32_foundation::SYSTEMTIME> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::SYSTEMTIME>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetMostRecentRunTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::SYSTEMTIME>(result__)
    }
    pub unsafe fn GetStatus(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::HRESULT>(result__)
    }
    pub unsafe fn GetExitCode(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetExitCode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetComment<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszcomment: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetComment)(::windows_core::Interface::as_raw(self), pwszcomment.into_param().abi()).ok()
    }
    pub unsafe fn GetComment(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetComment)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn SetCreator<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszcreator: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetCreator)(::windows_core::Interface::as_raw(self), pwszcreator.into_param().abi()).ok()
    }
    pub unsafe fn GetCreator(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetCreator)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn SetWorkItemData(&self, cbdata: u16, rgbdata: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetWorkItemData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cbdata), ::core::mem::transmute(rgbdata)).ok()
    }
    pub unsafe fn GetWorkItemData(&self, pcbdata: *mut u16, prgbdata: *mut *mut u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetWorkItemData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcbdata), ::core::mem::transmute(prgbdata)).ok()
    }
    pub unsafe fn SetErrorRetryCount(&self, wretrycount: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetErrorRetryCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wretrycount)).ok()
    }
    pub unsafe fn GetErrorRetryCount(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetErrorRetryCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn SetErrorRetryInterval(&self, wretryinterval: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetErrorRetryInterval)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wretryinterval)).ok()
    }
    pub unsafe fn GetErrorRetryInterval(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetErrorRetryInterval)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn SetFlags(&self, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn GetFlags(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetAccountInformation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszaccountname: Param0, pwszpassword: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetAccountInformation)(::windows_core::Interface::as_raw(self), pwszaccountname.into_param().abi(), pwszpassword.into_param().abi()).ok()
    }
    pub unsafe fn GetAccountInformation(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetAccountInformation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn SetApplicationName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszapplicationname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetApplicationName)(::windows_core::Interface::as_raw(self), pwszapplicationname.into_param().abi()).ok()
    }
    pub unsafe fn GetApplicationName(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetApplicationName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn SetParameters<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszparameters: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetParameters)(::windows_core::Interface::as_raw(self), pwszparameters.into_param().abi()).ok()
    }
    pub unsafe fn GetParameters(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetParameters)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn SetWorkingDirectory<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszworkingdirectory: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetWorkingDirectory)(::windows_core::Interface::as_raw(self), pwszworkingdirectory.into_param().abi()).ok()
    }
    pub unsafe fn GetWorkingDirectory(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetWorkingDirectory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn SetPriority(&self, dwpriority: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPriority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwpriority)).ok()
    }
    pub unsafe fn GetPriority(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetPriority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetTaskFlags(&self, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTaskFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn GetTaskFlags(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetTaskFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetMaxRunTime(&self, dwmaxruntimems: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMaxRunTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwmaxruntimems)).ok()
    }
    pub unsafe fn GetMaxRunTime(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetMaxRunTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<ITask> for ::windows_core::IUnknown {
    fn from(value: ITask) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITask> for ::windows_core::IUnknown {
    fn from(value: &ITask) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITask {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITask {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITask> for IScheduledWorkItem {
    fn from(value: ITask) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITask> for IScheduledWorkItem {
    fn from(value: &ITask) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IScheduledWorkItem> for ITask {
    fn into_param(self) -> ::windows_core::Param<'a, IScheduledWorkItem> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IScheduledWorkItem> for &'a ITask {
    fn into_param(self) -> ::windows_core::Param<'a, IScheduledWorkItem> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITask {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITask {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITask {}
impl ::core::fmt::Debug for ITask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITask").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITask {
    type Vtable = ITask_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x148bd524_a2ab_11ce_b11f_00aa00530503);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITask_Vtbl {
    pub base__: IScheduledWorkItem_Vtbl,
    pub SetApplicationName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszapplicationname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetApplicationName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszapplicationname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszparameters: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszparameters: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetWorkingDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszworkingdirectory: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetWorkingDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszworkingdirectory: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwpriority: u32) -> ::windows_core::HRESULT,
    pub GetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpriority: *mut u32) -> ::windows_core::HRESULT,
    pub SetTaskFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT,
    pub GetTaskFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT,
    pub SetMaxRunTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmaxruntimems: u32) -> ::windows_core::HRESULT,
    pub GetMaxRunTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwmaxruntimems: *mut u32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ITaskDefinition(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ITaskDefinition {
    #[cfg(feature = "win32-system")]
    pub unsafe fn RegistrationInfo(&self) -> ::windows_core::Result<IRegistrationInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).RegistrationInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRegistrationInfo>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetRegistrationInfo<'a, Param0: ::windows_core::IntoParam<'a, IRegistrationInfo>>(&self, pregistrationinfo: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRegistrationInfo)(::windows_core::Interface::as_raw(self), pregistrationinfo.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Triggers(&self) -> ::windows_core::Result<ITriggerCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Triggers)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITriggerCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetTriggers<'a, Param0: ::windows_core::IntoParam<'a, ITriggerCollection>>(&self, ptriggers: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTriggers)(::windows_core::Interface::as_raw(self), ptriggers.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Settings(&self) -> ::windows_core::Result<ITaskSettings> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Settings)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITaskSettings>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetSettings<'a, Param0: ::windows_core::IntoParam<'a, ITaskSettings>>(&self, psettings: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSettings)(::windows_core::Interface::as_raw(self), psettings.into_param().abi()).ok()
    }
    pub unsafe fn Data(&self, pdata: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Data)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetData<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, data: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetData)(::windows_core::Interface::as_raw(self), data.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Principal(&self) -> ::windows_core::Result<IPrincipal> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Principal)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPrincipal>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetPrincipal<'a, Param0: ::windows_core::IntoParam<'a, IPrincipal>>(&self, pprincipal: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPrincipal)(::windows_core::Interface::as_raw(self), pprincipal.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Actions(&self) -> ::windows_core::Result<IActionCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Actions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IActionCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetActions<'a, Param0: ::windows_core::IntoParam<'a, IActionCollection>>(&self, pactions: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetActions)(::windows_core::Interface::as_raw(self), pactions.into_param().abi()).ok()
    }
    pub unsafe fn XmlText(&self, pxml: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).XmlText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pxml)).ok()
    }
    pub unsafe fn SetXmlText<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, xml: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetXmlText)(::windows_core::Interface::as_raw(self), xml.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ITaskDefinition> for ::windows_core::IUnknown {
    fn from(value: ITaskDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ITaskDefinition> for ::windows_core::IUnknown {
    fn from(value: &ITaskDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITaskDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITaskDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ITaskDefinition> for super::Com::IDispatch {
    fn from(value: ITaskDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ITaskDefinition> for super::Com::IDispatch {
    fn from(value: &ITaskDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ITaskDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ITaskDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ITaskDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ITaskDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ITaskDefinition {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ITaskDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITaskDefinition").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ITaskDefinition {
    type Vtable = ITaskDefinition_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf5bc8fc5_536d_4f77_b852_fbc1356fdeb6);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ITaskDefinition_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub RegistrationInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppregistrationinfo: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    RegistrationInfo: usize,
    #[cfg(feature = "win32-system")]
    pub SetRegistrationInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pregistrationinfo: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetRegistrationInfo: usize,
    #[cfg(feature = "win32-system")]
    pub Triggers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptriggers: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Triggers: usize,
    #[cfg(feature = "win32-system")]
    pub SetTriggers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptriggers: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetTriggers: usize,
    #[cfg(feature = "win32-system")]
    pub Settings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsettings: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Settings: usize,
    #[cfg(feature = "win32-system")]
    pub SetSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psettings: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetSettings: usize,
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Principal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprincipal: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Principal: usize,
    #[cfg(feature = "win32-system")]
    pub SetPrincipal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprincipal: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetPrincipal: usize,
    #[cfg(feature = "win32-system")]
    pub Actions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppactions: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Actions: usize,
    #[cfg(feature = "win32-system")]
    pub SetActions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pactions: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetActions: usize,
    pub XmlText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pxml: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetXmlText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xml: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ITaskFolder(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ITaskFolder {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Path(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Path)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetFolder<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, path: Param0) -> ::windows_core::Result<ITaskFolder> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetFolder)(::windows_core::Interface::as_raw(self), path.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITaskFolder>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetFolders(&self, flags: i32) -> ::windows_core::Result<ITaskFolderCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetFolders)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(flags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITaskFolderCollection>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn CreateFolder<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, subfoldername: Param0, sddl: Param1) -> ::windows_core::Result<ITaskFolder> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateFolder)(::windows_core::Interface::as_raw(self), subfoldername.into_param().abi(), sddl.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITaskFolder>(result__)
    }
    pub unsafe fn DeleteFolder<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, subfoldername: Param0, flags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteFolder)(::windows_core::Interface::as_raw(self), subfoldername.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetTask<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, path: Param0) -> ::windows_core::Result<IRegisteredTask> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetTask)(::windows_core::Interface::as_raw(self), path.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRegisteredTask>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetTasks(&self, flags: i32) -> ::windows_core::Result<IRegisteredTaskCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetTasks)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(flags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRegisteredTaskCollection>(result__)
    }
    pub unsafe fn DeleteTask<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0, flags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteTask)(::windows_core::Interface::as_raw(self), name.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn RegisterTask<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, super::Com::VARIANT>, Param4: ::windows_core::IntoParam<'a, super::Com::VARIANT>, Param6: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, path: Param0, xmltext: Param1, flags: i32, userid: Param3, password: Param4, logontype: TASK_LOGON_TYPE, sddl: Param6) -> ::windows_core::Result<IRegisteredTask> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).RegisterTask)(::windows_core::Interface::as_raw(self), path.into_param().abi(), xmltext.into_param().abi(), ::core::mem::transmute(flags), userid.into_param().abi(), password.into_param().abi(), ::core::mem::transmute(logontype), sddl.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRegisteredTask>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn RegisterTaskDefinition<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ITaskDefinition>, Param3: ::windows_core::IntoParam<'a, super::Com::VARIANT>, Param4: ::windows_core::IntoParam<'a, super::Com::VARIANT>, Param6: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, path: Param0, pdefinition: Param1, flags: i32, userid: Param3, password: Param4, logontype: TASK_LOGON_TYPE, sddl: Param6) -> ::windows_core::Result<IRegisteredTask> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).RegisterTaskDefinition)(::windows_core::Interface::as_raw(self), path.into_param().abi(), pdefinition.into_param().abi(), ::core::mem::transmute(flags), userid.into_param().abi(), password.into_param().abi(), ::core::mem::transmute(logontype), sddl.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRegisteredTask>(result__)
    }
    pub unsafe fn GetSecurityDescriptor(&self, securityinformation: i32) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetSecurityDescriptor)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(securityinformation), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetSecurityDescriptor<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, sddl: Param0, flags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSecurityDescriptor)(::windows_core::Interface::as_raw(self), sddl.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ITaskFolder> for ::windows_core::IUnknown {
    fn from(value: ITaskFolder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ITaskFolder> for ::windows_core::IUnknown {
    fn from(value: &ITaskFolder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITaskFolder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITaskFolder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ITaskFolder> for super::Com::IDispatch {
    fn from(value: ITaskFolder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ITaskFolder> for super::Com::IDispatch {
    fn from(value: &ITaskFolder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ITaskFolder {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ITaskFolder {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ITaskFolder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ITaskFolder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ITaskFolder {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ITaskFolder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITaskFolder").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ITaskFolder {
    type Vtable = ITaskFolder_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8cfac062_a080_4c15_9a88_aa7c2af80dfc);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ITaskFolder_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppath: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub GetFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppfolder: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetFolder: usize,
    #[cfg(feature = "win32-system")]
    pub GetFolders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: i32, ppfolders: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetFolders: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub CreateFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subfoldername: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, sddl: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppfolder: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    CreateFolder: usize,
    pub DeleteFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subfoldername: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, flags: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub GetTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pptask: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetTask: usize,
    #[cfg(feature = "win32-system")]
    pub GetTasks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: i32, pptasks: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetTasks: usize,
    pub DeleteTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, flags: i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub RegisterTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, xmltext: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, flags: i32, userid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, password: ::core::mem::ManuallyDrop<super::Com::VARIANT>, logontype: TASK_LOGON_TYPE, sddl: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pptask: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    RegisterTask: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub RegisterTaskDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pdefinition: ::windows_core::RawPtr, flags: i32, userid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, password: ::core::mem::ManuallyDrop<super::Com::VARIANT>, logontype: TASK_LOGON_TYPE, sddl: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pptask: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    RegisterTaskDefinition: usize,
    pub GetSecurityDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, securityinformation: i32, psddl: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetSecurityDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sddl: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, flags: i32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ITaskFolderCollection(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ITaskFolderCollection {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn get_Item<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows_core::Result<ITaskFolder> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), index.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITaskFolder>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ITaskFolderCollection> for ::windows_core::IUnknown {
    fn from(value: ITaskFolderCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ITaskFolderCollection> for ::windows_core::IUnknown {
    fn from(value: &ITaskFolderCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITaskFolderCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITaskFolderCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ITaskFolderCollection> for super::Com::IDispatch {
    fn from(value: ITaskFolderCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ITaskFolderCollection> for super::Com::IDispatch {
    fn from(value: &ITaskFolderCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ITaskFolderCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ITaskFolderCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ITaskFolderCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ITaskFolderCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ITaskFolderCollection {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ITaskFolderCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITaskFolderCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ITaskFolderCollection {
    type Vtable = ITaskFolderCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79184a66_8664_423f_97f1_637356a5d812);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ITaskFolderCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppfolder: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITaskHandler(::windows_core::IUnknown);
impl ITaskHandler {
    pub unsafe fn Start<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, phandlerservices: Param0, data: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Start)(::windows_core::Interface::as_raw(self), phandlerservices.into_param().abi(), data.into_param().abi()).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
        (::windows_core::Interface::vtable(self).Stop)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::HRESULT>(result__)
    }
    pub unsafe fn Pause(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Pause)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Resume)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<ITaskHandler> for ::windows_core::IUnknown {
    fn from(value: ITaskHandler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITaskHandler> for ::windows_core::IUnknown {
    fn from(value: &ITaskHandler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITaskHandler {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITaskHandler {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITaskHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITaskHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITaskHandler {}
impl ::core::fmt::Debug for ITaskHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITaskHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITaskHandler {
    type Vtable = ITaskHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x839d7762_5121_4009_9234_4f0d19394f04);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITaskHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phandlerservices: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretcode: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITaskHandlerStatus(::windows_core::IUnknown);
impl ITaskHandlerStatus {
    pub unsafe fn UpdateStatus<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, percentcomplete: i16, statusmessage: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UpdateStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(percentcomplete), statusmessage.into_param().abi()).ok()
    }
    pub unsafe fn TaskCompleted(&self, taskerrcode: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TaskCompleted)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(taskerrcode)).ok()
    }
}
impl ::core::convert::From<ITaskHandlerStatus> for ::windows_core::IUnknown {
    fn from(value: ITaskHandlerStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITaskHandlerStatus> for ::windows_core::IUnknown {
    fn from(value: &ITaskHandlerStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITaskHandlerStatus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITaskHandlerStatus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITaskHandlerStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITaskHandlerStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITaskHandlerStatus {}
impl ::core::fmt::Debug for ITaskHandlerStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITaskHandlerStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITaskHandlerStatus {
    type Vtable = ITaskHandlerStatus_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeaec7a8f_27a0_4ddc_8675_14726a01a38a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITaskHandlerStatus_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub UpdateStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, percentcomplete: i16, statusmessage: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub TaskCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskerrcode: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ITaskNamedValueCollection(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ITaskNamedValueCollection {
    pub unsafe fn Count(&self, pcount: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcount)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows_core::Result<ITaskNamedValuePair> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITaskNamedValuePair>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0, value: Param1) -> ::windows_core::Result<ITaskNamedValuePair> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Create)(::windows_core::Interface::as_raw(self), name.into_param().abi(), value.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITaskNamedValuePair>(result__)
    }
    pub unsafe fn Remove(&self, index: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index)).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Clear)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ITaskNamedValueCollection> for ::windows_core::IUnknown {
    fn from(value: ITaskNamedValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ITaskNamedValueCollection> for ::windows_core::IUnknown {
    fn from(value: &ITaskNamedValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITaskNamedValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITaskNamedValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ITaskNamedValueCollection> for super::Com::IDispatch {
    fn from(value: ITaskNamedValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ITaskNamedValueCollection> for super::Com::IDispatch {
    fn from(value: &ITaskNamedValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ITaskNamedValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ITaskNamedValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ITaskNamedValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ITaskNamedValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ITaskNamedValueCollection {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ITaskNamedValueCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITaskNamedValueCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ITaskNamedValueCollection {
    type Vtable = ITaskNamedValueCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb4ef826b_63c3_46e4_a504_ef69e4f7ea4d);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ITaskNamedValueCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pppair: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, value: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pppair: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Create: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32) -> ::windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ITaskNamedValuePair(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ITaskNamedValuePair {
    pub unsafe fn Name(&self, pname: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pname)).ok()
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn Value(&self, pvalue: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Value)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pvalue)).ok()
    }
    pub unsafe fn SetValue<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetValue)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ITaskNamedValuePair> for ::windows_core::IUnknown {
    fn from(value: ITaskNamedValuePair) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ITaskNamedValuePair> for ::windows_core::IUnknown {
    fn from(value: &ITaskNamedValuePair) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITaskNamedValuePair {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITaskNamedValuePair {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ITaskNamedValuePair> for super::Com::IDispatch {
    fn from(value: ITaskNamedValuePair) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ITaskNamedValuePair> for super::Com::IDispatch {
    fn from(value: &ITaskNamedValuePair) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ITaskNamedValuePair {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ITaskNamedValuePair {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ITaskNamedValuePair {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ITaskNamedValuePair {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ITaskNamedValuePair {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ITaskNamedValuePair {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITaskNamedValuePair").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ITaskNamedValuePair {
    type Vtable = ITaskNamedValuePair_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x39038068_2b46_4afd_8662_7bb6f868d221);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ITaskNamedValuePair_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITaskScheduler(::windows_core::IUnknown);
impl ITaskScheduler {
    pub unsafe fn SetTargetComputer<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszcomputer: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTargetComputer)(::windows_core::Interface::as_raw(self), pwszcomputer.into_param().abi()).ok()
    }
    pub unsafe fn GetTargetComputer(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetTargetComputer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn Enum(&self) -> ::windows_core::Result<IEnumWorkItems> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Enum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumWorkItems>(result__)
    }
    pub unsafe fn Activate<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszname: Param0, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).Activate)(::windows_core::Interface::as_raw(self), pwszname.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn Delete<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self), pwszname.into_param().abi()).ok()
    }
    pub unsafe fn NewWorkItem<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwsztaskname: Param0, rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).NewWorkItem)(::windows_core::Interface::as_raw(self), pwsztaskname.into_param().abi(), ::core::mem::transmute(rclsid), ::core::mem::transmute(riid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn AddWorkItem<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, IScheduledWorkItem>>(&self, pwsztaskname: Param0, pworkitem: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddWorkItem)(::windows_core::Interface::as_raw(self), pwsztaskname.into_param().abi(), pworkitem.into_param().abi()).ok()
    }
    pub unsafe fn IsOfType<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszname: Param0, riid: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsOfType)(::windows_core::Interface::as_raw(self), pwszname.into_param().abi(), ::core::mem::transmute(riid)).ok()
    }
}
impl ::core::convert::From<ITaskScheduler> for ::windows_core::IUnknown {
    fn from(value: ITaskScheduler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITaskScheduler> for ::windows_core::IUnknown {
    fn from(value: &ITaskScheduler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITaskScheduler {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITaskScheduler {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITaskScheduler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITaskScheduler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITaskScheduler {}
impl ::core::fmt::Debug for ITaskScheduler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITaskScheduler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITaskScheduler {
    type Vtable = ITaskScheduler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x148bd527_a2ab_11ce_b11f_00aa00530503);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITaskScheduler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetTargetComputer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszcomputer: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetTargetComputer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszcomputer: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub Enum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumworkitems: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Activate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub NewWorkItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsztaskname: ::windows_core::PCWSTR, rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AddWorkItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsztaskname: ::windows_core::PCWSTR, pworkitem: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsOfType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ITaskService(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ITaskService {
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetFolder<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, path: Param0) -> ::windows_core::Result<ITaskFolder> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetFolder)(::windows_core::Interface::as_raw(self), path.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITaskFolder>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetRunningTasks(&self, flags: i32) -> ::windows_core::Result<IRunningTaskCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetRunningTasks)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(flags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRunningTaskCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn NewTask(&self, flags: u32) -> ::windows_core::Result<ITaskDefinition> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).NewTask)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(flags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITaskDefinition>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Connect<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>, Param1: ::windows_core::IntoParam<'a, super::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, super::Com::VARIANT>, Param3: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, servername: Param0, user: Param1, domain: Param2, password: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Connect)(::windows_core::Interface::as_raw(self), servername.into_param().abi(), user.into_param().abi(), domain.into_param().abi(), password.into_param().abi()).ok()
    }
    pub unsafe fn Connected(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).Connected)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn TargetServer(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).TargetServer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ConnectedUser(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ConnectedUser)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ConnectedDomain(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ConnectedDomain)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn HighestVersion(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).HighestVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ITaskService> for ::windows_core::IUnknown {
    fn from(value: ITaskService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ITaskService> for ::windows_core::IUnknown {
    fn from(value: &ITaskService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITaskService {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITaskService {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ITaskService> for super::Com::IDispatch {
    fn from(value: ITaskService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ITaskService> for super::Com::IDispatch {
    fn from(value: &ITaskService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ITaskService {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ITaskService {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ITaskService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ITaskService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ITaskService {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ITaskService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITaskService").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ITaskService {
    type Vtable = ITaskService_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2faba4c7_4da9_4013_9697_20cc3fd40f85);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ITaskService_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub GetFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppfolder: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetFolder: usize,
    #[cfg(feature = "win32-system")]
    pub GetRunningTasks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: i32, pprunningtasks: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetRunningTasks: usize,
    #[cfg(feature = "win32-system")]
    pub NewTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: u32, ppdefinition: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    NewTask: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servername: ::core::mem::ManuallyDrop<super::Com::VARIANT>, user: ::core::mem::ManuallyDrop<super::Com::VARIANT>, domain: ::core::mem::ManuallyDrop<super::Com::VARIANT>, password: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Connect: usize,
    pub Connected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnected: *mut i16) -> ::windows_core::HRESULT,
    pub TargetServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pserver: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub ConnectedUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puser: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub ConnectedDomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdomain: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub HighestVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pversion: *mut u32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ITaskSettings(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ITaskSettings {
    pub unsafe fn AllowDemandStart(&self, pallowdemandstart: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AllowDemandStart)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pallowdemandstart)).ok()
    }
    pub unsafe fn SetAllowDemandStart(&self, allowdemandstart: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAllowDemandStart)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(allowdemandstart)).ok()
    }
    pub unsafe fn RestartInterval(&self, prestartinterval: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RestartInterval)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(prestartinterval)).ok()
    }
    pub unsafe fn SetRestartInterval<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, restartinterval: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRestartInterval)(::windows_core::Interface::as_raw(self), restartinterval.into_param().abi()).ok()
    }
    pub unsafe fn RestartCount(&self, prestartcount: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RestartCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(prestartcount)).ok()
    }
    pub unsafe fn SetRestartCount(&self, restartcount: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRestartCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(restartcount)).ok()
    }
    pub unsafe fn MultipleInstances(&self, ppolicy: *mut TASK_INSTANCES_POLICY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MultipleInstances)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppolicy)).ok()
    }
    pub unsafe fn SetMultipleInstances(&self, policy: TASK_INSTANCES_POLICY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMultipleInstances)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(policy)).ok()
    }
    pub unsafe fn StopIfGoingOnBatteries(&self, pstopifonbatteries: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StopIfGoingOnBatteries)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pstopifonbatteries)).ok()
    }
    pub unsafe fn SetStopIfGoingOnBatteries(&self, stopifonbatteries: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStopIfGoingOnBatteries)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(stopifonbatteries)).ok()
    }
    pub unsafe fn DisallowStartIfOnBatteries(&self, pdisallowstart: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DisallowStartIfOnBatteries)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdisallowstart)).ok()
    }
    pub unsafe fn SetDisallowStartIfOnBatteries(&self, disallowstart: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDisallowStartIfOnBatteries)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(disallowstart)).ok()
    }
    pub unsafe fn AllowHardTerminate(&self, pallowhardterminate: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AllowHardTerminate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pallowhardterminate)).ok()
    }
    pub unsafe fn SetAllowHardTerminate(&self, allowhardterminate: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAllowHardTerminate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(allowhardterminate)).ok()
    }
    pub unsafe fn StartWhenAvailable(&self, pstartwhenavailable: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StartWhenAvailable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pstartwhenavailable)).ok()
    }
    pub unsafe fn SetStartWhenAvailable(&self, startwhenavailable: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStartWhenAvailable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(startwhenavailable)).ok()
    }
    pub unsafe fn XmlText(&self, ptext: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).XmlText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptext)).ok()
    }
    pub unsafe fn SetXmlText<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, text: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetXmlText)(::windows_core::Interface::as_raw(self), text.into_param().abi()).ok()
    }
    pub unsafe fn RunOnlyIfNetworkAvailable(&self, prunonlyifnetworkavailable: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RunOnlyIfNetworkAvailable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(prunonlyifnetworkavailable)).ok()
    }
    pub unsafe fn SetRunOnlyIfNetworkAvailable(&self, runonlyifnetworkavailable: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRunOnlyIfNetworkAvailable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(runonlyifnetworkavailable)).ok()
    }
    pub unsafe fn ExecutionTimeLimit(&self, pexecutiontimelimit: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ExecutionTimeLimit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pexecutiontimelimit)).ok()
    }
    pub unsafe fn SetExecutionTimeLimit<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, executiontimelimit: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetExecutionTimeLimit)(::windows_core::Interface::as_raw(self), executiontimelimit.into_param().abi()).ok()
    }
    pub unsafe fn Enabled(&self, penabled: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Enabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(penabled)).ok()
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enabled)).ok()
    }
    pub unsafe fn DeleteExpiredTaskAfter(&self, pexpirationdelay: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteExpiredTaskAfter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pexpirationdelay)).ok()
    }
    pub unsafe fn SetDeleteExpiredTaskAfter<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, expirationdelay: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDeleteExpiredTaskAfter)(::windows_core::Interface::as_raw(self), expirationdelay.into_param().abi()).ok()
    }
    pub unsafe fn Priority(&self, ppriority: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Priority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppriority)).ok()
    }
    pub unsafe fn SetPriority(&self, priority: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPriority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(priority)).ok()
    }
    pub unsafe fn Compatibility(&self, pcompatlevel: *mut TASK_COMPATIBILITY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Compatibility)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcompatlevel)).ok()
    }
    pub unsafe fn SetCompatibility(&self, compatlevel: TASK_COMPATIBILITY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCompatibility)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(compatlevel)).ok()
    }
    pub unsafe fn Hidden(&self, phidden: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Hidden)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(phidden)).ok()
    }
    pub unsafe fn SetHidden(&self, hidden: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetHidden)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hidden)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn IdleSettings(&self) -> ::windows_core::Result<IIdleSettings> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).IdleSettings)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IIdleSettings>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetIdleSettings<'a, Param0: ::windows_core::IntoParam<'a, IIdleSettings>>(&self, pidlesettings: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIdleSettings)(::windows_core::Interface::as_raw(self), pidlesettings.into_param().abi()).ok()
    }
    pub unsafe fn RunOnlyIfIdle(&self, prunonlyifidle: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RunOnlyIfIdle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(prunonlyifidle)).ok()
    }
    pub unsafe fn SetRunOnlyIfIdle(&self, runonlyifidle: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRunOnlyIfIdle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(runonlyifidle)).ok()
    }
    pub unsafe fn WakeToRun(&self, pwake: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WakeToRun)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwake)).ok()
    }
    pub unsafe fn SetWakeToRun(&self, wake: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetWakeToRun)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wake)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn NetworkSettings(&self) -> ::windows_core::Result<INetworkSettings> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).NetworkSettings)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INetworkSettings>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetNetworkSettings<'a, Param0: ::windows_core::IntoParam<'a, INetworkSettings>>(&self, pnetworksettings: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetNetworkSettings)(::windows_core::Interface::as_raw(self), pnetworksettings.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ITaskSettings> for ::windows_core::IUnknown {
    fn from(value: ITaskSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ITaskSettings> for ::windows_core::IUnknown {
    fn from(value: &ITaskSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITaskSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITaskSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ITaskSettings> for super::Com::IDispatch {
    fn from(value: ITaskSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ITaskSettings> for super::Com::IDispatch {
    fn from(value: &ITaskSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ITaskSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ITaskSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ITaskSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ITaskSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ITaskSettings {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ITaskSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITaskSettings").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ITaskSettings {
    type Vtable = ITaskSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8fd4711d_2d02_4c8c_87e3_eff699de127e);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ITaskSettings_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub AllowDemandStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pallowdemandstart: *mut i16) -> ::windows_core::HRESULT,
    pub SetAllowDemandStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allowdemandstart: i16) -> ::windows_core::HRESULT,
    pub RestartInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prestartinterval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetRestartInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, restartinterval: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub RestartCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prestartcount: *mut i32) -> ::windows_core::HRESULT,
    pub SetRestartCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, restartcount: i32) -> ::windows_core::HRESULT,
    pub MultipleInstances: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppolicy: *mut TASK_INSTANCES_POLICY) -> ::windows_core::HRESULT,
    pub SetMultipleInstances: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, policy: TASK_INSTANCES_POLICY) -> ::windows_core::HRESULT,
    pub StopIfGoingOnBatteries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstopifonbatteries: *mut i16) -> ::windows_core::HRESULT,
    pub SetStopIfGoingOnBatteries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stopifonbatteries: i16) -> ::windows_core::HRESULT,
    pub DisallowStartIfOnBatteries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdisallowstart: *mut i16) -> ::windows_core::HRESULT,
    pub SetDisallowStartIfOnBatteries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disallowstart: i16) -> ::windows_core::HRESULT,
    pub AllowHardTerminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pallowhardterminate: *mut i16) -> ::windows_core::HRESULT,
    pub SetAllowHardTerminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allowhardterminate: i16) -> ::windows_core::HRESULT,
    pub StartWhenAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstartwhenavailable: *mut i16) -> ::windows_core::HRESULT,
    pub SetStartWhenAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startwhenavailable: i16) -> ::windows_core::HRESULT,
    pub XmlText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptext: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetXmlText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub RunOnlyIfNetworkAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prunonlyifnetworkavailable: *mut i16) -> ::windows_core::HRESULT,
    pub SetRunOnlyIfNetworkAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, runonlyifnetworkavailable: i16) -> ::windows_core::HRESULT,
    pub ExecutionTimeLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pexecutiontimelimit: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetExecutionTimeLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, executiontimelimit: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penabled: *mut i16) -> ::windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows_core::HRESULT,
    pub DeleteExpiredTaskAfter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pexpirationdelay: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetDeleteExpiredTaskAfter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, expirationdelay: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppriority: *mut i32) -> ::windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, priority: i32) -> ::windows_core::HRESULT,
    pub Compatibility: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcompatlevel: *mut TASK_COMPATIBILITY) -> ::windows_core::HRESULT,
    pub SetCompatibility: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compatlevel: TASK_COMPATIBILITY) -> ::windows_core::HRESULT,
    pub Hidden: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phidden: *mut i16) -> ::windows_core::HRESULT,
    pub SetHidden: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hidden: i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub IdleSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppidlesettings: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    IdleSettings: usize,
    #[cfg(feature = "win32-system")]
    pub SetIdleSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidlesettings: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetIdleSettings: usize,
    pub RunOnlyIfIdle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prunonlyifidle: *mut i16) -> ::windows_core::HRESULT,
    pub SetRunOnlyIfIdle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, runonlyifidle: i16) -> ::windows_core::HRESULT,
    pub WakeToRun: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwake: *mut i16) -> ::windows_core::HRESULT,
    pub SetWakeToRun: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wake: i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub NetworkSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnetworksettings: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    NetworkSettings: usize,
    #[cfg(feature = "win32-system")]
    pub SetNetworkSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnetworksettings: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetNetworkSettings: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ITaskSettings2(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ITaskSettings2 {
    pub unsafe fn DisallowStartOnRemoteAppSession(&self, pdisallowstart: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DisallowStartOnRemoteAppSession)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdisallowstart)).ok()
    }
    pub unsafe fn SetDisallowStartOnRemoteAppSession(&self, disallowstart: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDisallowStartOnRemoteAppSession)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(disallowstart)).ok()
    }
    pub unsafe fn UseUnifiedSchedulingEngine(&self, puseunifiedengine: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UseUnifiedSchedulingEngine)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(puseunifiedengine)).ok()
    }
    pub unsafe fn SetUseUnifiedSchedulingEngine(&self, useunifiedengine: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetUseUnifiedSchedulingEngine)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(useunifiedengine)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ITaskSettings2> for ::windows_core::IUnknown {
    fn from(value: ITaskSettings2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ITaskSettings2> for ::windows_core::IUnknown {
    fn from(value: &ITaskSettings2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITaskSettings2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITaskSettings2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ITaskSettings2> for super::Com::IDispatch {
    fn from(value: ITaskSettings2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ITaskSettings2> for super::Com::IDispatch {
    fn from(value: &ITaskSettings2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ITaskSettings2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ITaskSettings2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ITaskSettings2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ITaskSettings2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ITaskSettings2 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ITaskSettings2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITaskSettings2").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ITaskSettings2 {
    type Vtable = ITaskSettings2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c05c3f0_6eed_4c05_a15f_ed7d7a98a369);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ITaskSettings2_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub DisallowStartOnRemoteAppSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdisallowstart: *mut i16) -> ::windows_core::HRESULT,
    pub SetDisallowStartOnRemoteAppSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disallowstart: i16) -> ::windows_core::HRESULT,
    pub UseUnifiedSchedulingEngine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puseunifiedengine: *mut i16) -> ::windows_core::HRESULT,
    pub SetUseUnifiedSchedulingEngine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useunifiedengine: i16) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ITaskSettings3(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ITaskSettings3 {
    pub unsafe fn AllowDemandStart(&self, pallowdemandstart: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AllowDemandStart)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pallowdemandstart)).ok()
    }
    pub unsafe fn SetAllowDemandStart(&self, allowdemandstart: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetAllowDemandStart)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(allowdemandstart)).ok()
    }
    pub unsafe fn RestartInterval(&self, prestartinterval: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RestartInterval)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(prestartinterval)).ok()
    }
    pub unsafe fn SetRestartInterval<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, restartinterval: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRestartInterval)(::windows_core::Interface::as_raw(self), restartinterval.into_param().abi()).ok()
    }
    pub unsafe fn RestartCount(&self, prestartcount: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RestartCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(prestartcount)).ok()
    }
    pub unsafe fn SetRestartCount(&self, restartcount: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRestartCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(restartcount)).ok()
    }
    pub unsafe fn MultipleInstances(&self, ppolicy: *mut TASK_INSTANCES_POLICY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.MultipleInstances)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppolicy)).ok()
    }
    pub unsafe fn SetMultipleInstances(&self, policy: TASK_INSTANCES_POLICY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetMultipleInstances)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(policy)).ok()
    }
    pub unsafe fn StopIfGoingOnBatteries(&self, pstopifonbatteries: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StopIfGoingOnBatteries)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pstopifonbatteries)).ok()
    }
    pub unsafe fn SetStopIfGoingOnBatteries(&self, stopifonbatteries: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetStopIfGoingOnBatteries)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(stopifonbatteries)).ok()
    }
    pub unsafe fn DisallowStartIfOnBatteries(&self, pdisallowstart: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DisallowStartIfOnBatteries)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdisallowstart)).ok()
    }
    pub unsafe fn SetDisallowStartIfOnBatteries(&self, disallowstart: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDisallowStartIfOnBatteries)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(disallowstart)).ok()
    }
    pub unsafe fn AllowHardTerminate(&self, pallowhardterminate: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AllowHardTerminate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pallowhardterminate)).ok()
    }
    pub unsafe fn SetAllowHardTerminate(&self, allowhardterminate: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetAllowHardTerminate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(allowhardterminate)).ok()
    }
    pub unsafe fn StartWhenAvailable(&self, pstartwhenavailable: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StartWhenAvailable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pstartwhenavailable)).ok()
    }
    pub unsafe fn SetStartWhenAvailable(&self, startwhenavailable: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetStartWhenAvailable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(startwhenavailable)).ok()
    }
    pub unsafe fn XmlText(&self, ptext: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.XmlText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptext)).ok()
    }
    pub unsafe fn SetXmlText<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, text: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetXmlText)(::windows_core::Interface::as_raw(self), text.into_param().abi()).ok()
    }
    pub unsafe fn RunOnlyIfNetworkAvailable(&self, prunonlyifnetworkavailable: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RunOnlyIfNetworkAvailable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(prunonlyifnetworkavailable)).ok()
    }
    pub unsafe fn SetRunOnlyIfNetworkAvailable(&self, runonlyifnetworkavailable: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRunOnlyIfNetworkAvailable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(runonlyifnetworkavailable)).ok()
    }
    pub unsafe fn ExecutionTimeLimit(&self, pexecutiontimelimit: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ExecutionTimeLimit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pexecutiontimelimit)).ok()
    }
    pub unsafe fn SetExecutionTimeLimit<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, executiontimelimit: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetExecutionTimeLimit)(::windows_core::Interface::as_raw(self), executiontimelimit.into_param().abi()).ok()
    }
    pub unsafe fn Enabled(&self, penabled: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Enabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(penabled)).ok()
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enabled)).ok()
    }
    pub unsafe fn DeleteExpiredTaskAfter(&self, pexpirationdelay: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteExpiredTaskAfter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pexpirationdelay)).ok()
    }
    pub unsafe fn SetDeleteExpiredTaskAfter<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, expirationdelay: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDeleteExpiredTaskAfter)(::windows_core::Interface::as_raw(self), expirationdelay.into_param().abi()).ok()
    }
    pub unsafe fn Priority(&self, ppriority: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Priority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppriority)).ok()
    }
    pub unsafe fn SetPriority(&self, priority: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetPriority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(priority)).ok()
    }
    pub unsafe fn Compatibility(&self, pcompatlevel: *mut TASK_COMPATIBILITY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Compatibility)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcompatlevel)).ok()
    }
    pub unsafe fn SetCompatibility(&self, compatlevel: TASK_COMPATIBILITY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetCompatibility)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(compatlevel)).ok()
    }
    pub unsafe fn Hidden(&self, phidden: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Hidden)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(phidden)).ok()
    }
    pub unsafe fn SetHidden(&self, hidden: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetHidden)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hidden)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn IdleSettings(&self) -> ::windows_core::Result<IIdleSettings> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IdleSettings)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IIdleSettings>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetIdleSettings<'a, Param0: ::windows_core::IntoParam<'a, IIdleSettings>>(&self, pidlesettings: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetIdleSettings)(::windows_core::Interface::as_raw(self), pidlesettings.into_param().abi()).ok()
    }
    pub unsafe fn RunOnlyIfIdle(&self, prunonlyifidle: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RunOnlyIfIdle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(prunonlyifidle)).ok()
    }
    pub unsafe fn SetRunOnlyIfIdle(&self, runonlyifidle: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRunOnlyIfIdle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(runonlyifidle)).ok()
    }
    pub unsafe fn WakeToRun(&self, pwake: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.WakeToRun)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwake)).ok()
    }
    pub unsafe fn SetWakeToRun(&self, wake: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetWakeToRun)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wake)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn NetworkSettings(&self) -> ::windows_core::Result<INetworkSettings> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.NetworkSettings)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INetworkSettings>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetNetworkSettings<'a, Param0: ::windows_core::IntoParam<'a, INetworkSettings>>(&self, pnetworksettings: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetNetworkSettings)(::windows_core::Interface::as_raw(self), pnetworksettings.into_param().abi()).ok()
    }
    pub unsafe fn DisallowStartOnRemoteAppSession(&self, pdisallowstart: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DisallowStartOnRemoteAppSession)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdisallowstart)).ok()
    }
    pub unsafe fn SetDisallowStartOnRemoteAppSession(&self, disallowstart: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDisallowStartOnRemoteAppSession)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(disallowstart)).ok()
    }
    pub unsafe fn UseUnifiedSchedulingEngine(&self, puseunifiedengine: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UseUnifiedSchedulingEngine)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(puseunifiedengine)).ok()
    }
    pub unsafe fn SetUseUnifiedSchedulingEngine(&self, useunifiedengine: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetUseUnifiedSchedulingEngine)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(useunifiedengine)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn MaintenanceSettings(&self) -> ::windows_core::Result<IMaintenanceSettings> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).MaintenanceSettings)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMaintenanceSettings>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetMaintenanceSettings<'a, Param0: ::windows_core::IntoParam<'a, IMaintenanceSettings>>(&self, pmaintenancesettings: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMaintenanceSettings)(::windows_core::Interface::as_raw(self), pmaintenancesettings.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateMaintenanceSettings(&self) -> ::windows_core::Result<IMaintenanceSettings> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateMaintenanceSettings)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMaintenanceSettings>(result__)
    }
    pub unsafe fn Volatile(&self, pvolatile: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Volatile)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pvolatile)).ok()
    }
    pub unsafe fn SetVolatile(&self, volatile: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetVolatile)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(volatile)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ITaskSettings3> for ::windows_core::IUnknown {
    fn from(value: ITaskSettings3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ITaskSettings3> for ::windows_core::IUnknown {
    fn from(value: &ITaskSettings3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITaskSettings3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITaskSettings3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ITaskSettings3> for super::Com::IDispatch {
    fn from(value: ITaskSettings3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ITaskSettings3> for super::Com::IDispatch {
    fn from(value: &ITaskSettings3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ITaskSettings3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ITaskSettings3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ITaskSettings3> for ITaskSettings {
    fn from(value: ITaskSettings3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ITaskSettings3> for ITaskSettings {
    fn from(value: &ITaskSettings3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ITaskSettings> for ITaskSettings3 {
    fn into_param(self) -> ::windows_core::Param<'a, ITaskSettings> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ITaskSettings> for &'a ITaskSettings3 {
    fn into_param(self) -> ::windows_core::Param<'a, ITaskSettings> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ITaskSettings3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ITaskSettings3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ITaskSettings3 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ITaskSettings3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITaskSettings3").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ITaskSettings3 {
    type Vtable = ITaskSettings3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0ad9d0d7_0c7f_4ebb_9a5f_d1c648dca528);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ITaskSettings3_Vtbl {
    pub base__: ITaskSettings_Vtbl,
    pub DisallowStartOnRemoteAppSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdisallowstart: *mut i16) -> ::windows_core::HRESULT,
    pub SetDisallowStartOnRemoteAppSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disallowstart: i16) -> ::windows_core::HRESULT,
    pub UseUnifiedSchedulingEngine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puseunifiedengine: *mut i16) -> ::windows_core::HRESULT,
    pub SetUseUnifiedSchedulingEngine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useunifiedengine: i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub MaintenanceSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmaintenancesettings: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    MaintenanceSettings: usize,
    #[cfg(feature = "win32-system")]
    pub SetMaintenanceSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmaintenancesettings: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetMaintenanceSettings: usize,
    #[cfg(feature = "win32-system")]
    pub CreateMaintenanceSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmaintenancesettings: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CreateMaintenanceSettings: usize,
    pub Volatile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvolatile: *mut i16) -> ::windows_core::HRESULT,
    pub SetVolatile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, volatile: i16) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITaskTrigger(::windows_core::IUnknown);
impl ITaskTrigger {
    pub unsafe fn SetTrigger(&self, ptrigger: *const TASK_TRIGGER) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTrigger)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptrigger)).ok()
    }
    pub unsafe fn GetTrigger(&self) -> ::windows_core::Result<TASK_TRIGGER> {
        let mut result__ = ::core::mem::MaybeUninit::<TASK_TRIGGER>::zeroed();
        (::windows_core::Interface::vtable(self).GetTrigger)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<TASK_TRIGGER>(result__)
    }
    pub unsafe fn GetTriggerString(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetTriggerString)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
}
impl ::core::convert::From<ITaskTrigger> for ::windows_core::IUnknown {
    fn from(value: ITaskTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITaskTrigger> for ::windows_core::IUnknown {
    fn from(value: &ITaskTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITaskTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITaskTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITaskTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITaskTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITaskTrigger {}
impl ::core::fmt::Debug for ITaskTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITaskTrigger").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITaskTrigger {
    type Vtable = ITaskTrigger_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x148bd52b_a2ab_11ce_b11f_00aa00530503);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITaskTrigger_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetTrigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptrigger: *const TASK_TRIGGER) -> ::windows_core::HRESULT,
    pub GetTrigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptrigger: *mut TASK_TRIGGER) -> ::windows_core::HRESULT,
    pub GetTriggerString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwsztrigger: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITaskVariables(::windows_core::IUnknown);
impl ITaskVariables {
    pub unsafe fn GetInput(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetInput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetOutput<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, input: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOutput)(::windows_core::Interface::as_raw(self), input.into_param().abi()).ok()
    }
    pub unsafe fn GetContext(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetContext)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<ITaskVariables> for ::windows_core::IUnknown {
    fn from(value: ITaskVariables) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITaskVariables> for ::windows_core::IUnknown {
    fn from(value: &ITaskVariables) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITaskVariables {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITaskVariables {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITaskVariables {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITaskVariables {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITaskVariables {}
impl ::core::fmt::Debug for ITaskVariables {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITaskVariables").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITaskVariables {
    type Vtable = ITaskVariables_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3e4c9351_d966_4b8b_bb87_ceba68bb0107);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITaskVariables_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinput: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub GetContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ITimeTrigger(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ITimeTrigger {
    pub unsafe fn Type(&self, ptype: *mut TASK_TRIGGER_TYPE2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    pub unsafe fn Id(&self, pid: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    pub unsafe fn SetId<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, id: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetId)(::windows_core::Interface::as_raw(self), id.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Repetition(&self) -> ::windows_core::Result<IRepetitionPattern> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Repetition)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRepetitionPattern>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetRepetition<'a, Param0: ::windows_core::IntoParam<'a, IRepetitionPattern>>(&self, prepeat: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRepetition)(::windows_core::Interface::as_raw(self), prepeat.into_param().abi()).ok()
    }
    pub unsafe fn ExecutionTimeLimit(&self, ptimelimit: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ExecutionTimeLimit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptimelimit)).ok()
    }
    pub unsafe fn SetExecutionTimeLimit<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, timelimit: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetExecutionTimeLimit)(::windows_core::Interface::as_raw(self), timelimit.into_param().abi()).ok()
    }
    pub unsafe fn StartBoundary(&self, pstart: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StartBoundary)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pstart)).ok()
    }
    pub unsafe fn SetStartBoundary<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, start: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetStartBoundary)(::windows_core::Interface::as_raw(self), start.into_param().abi()).ok()
    }
    pub unsafe fn EndBoundary(&self, pend: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EndBoundary)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pend)).ok()
    }
    pub unsafe fn SetEndBoundary<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, end: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEndBoundary)(::windows_core::Interface::as_raw(self), end.into_param().abi()).ok()
    }
    pub unsafe fn Enabled(&self, penabled: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Enabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(penabled)).ok()
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enabled)).ok()
    }
    pub unsafe fn RandomDelay(&self, prandomdelay: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RandomDelay)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(prandomdelay)).ok()
    }
    pub unsafe fn SetRandomDelay<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, randomdelay: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRandomDelay)(::windows_core::Interface::as_raw(self), randomdelay.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ITimeTrigger> for ::windows_core::IUnknown {
    fn from(value: ITimeTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ITimeTrigger> for ::windows_core::IUnknown {
    fn from(value: &ITimeTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITimeTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITimeTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ITimeTrigger> for super::Com::IDispatch {
    fn from(value: ITimeTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ITimeTrigger> for super::Com::IDispatch {
    fn from(value: &ITimeTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ITimeTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ITimeTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ITimeTrigger> for ITrigger {
    fn from(value: ITimeTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ITimeTrigger> for ITrigger {
    fn from(value: &ITimeTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ITrigger> for ITimeTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ITrigger> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ITrigger> for &'a ITimeTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ITrigger> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ITimeTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ITimeTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ITimeTrigger {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ITimeTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITimeTrigger").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ITimeTrigger {
    type Vtable = ITimeTrigger_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb45747e0_eba7_4276_9f29_85c5bb300006);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ITimeTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub RandomDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prandomdelay: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetRandomDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, randomdelay: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ITrigger(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ITrigger {
    pub unsafe fn Type(&self, ptype: *mut TASK_TRIGGER_TYPE2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    pub unsafe fn Id(&self, pid: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    pub unsafe fn SetId<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, id: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetId)(::windows_core::Interface::as_raw(self), id.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Repetition(&self) -> ::windows_core::Result<IRepetitionPattern> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Repetition)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRepetitionPattern>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetRepetition<'a, Param0: ::windows_core::IntoParam<'a, IRepetitionPattern>>(&self, prepeat: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRepetition)(::windows_core::Interface::as_raw(self), prepeat.into_param().abi()).ok()
    }
    pub unsafe fn ExecutionTimeLimit(&self, ptimelimit: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ExecutionTimeLimit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptimelimit)).ok()
    }
    pub unsafe fn SetExecutionTimeLimit<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, timelimit: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetExecutionTimeLimit)(::windows_core::Interface::as_raw(self), timelimit.into_param().abi()).ok()
    }
    pub unsafe fn StartBoundary(&self, pstart: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StartBoundary)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pstart)).ok()
    }
    pub unsafe fn SetStartBoundary<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, start: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStartBoundary)(::windows_core::Interface::as_raw(self), start.into_param().abi()).ok()
    }
    pub unsafe fn EndBoundary(&self, pend: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndBoundary)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pend)).ok()
    }
    pub unsafe fn SetEndBoundary<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, end: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEndBoundary)(::windows_core::Interface::as_raw(self), end.into_param().abi()).ok()
    }
    pub unsafe fn Enabled(&self, penabled: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Enabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(penabled)).ok()
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enabled)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ITrigger> for ::windows_core::IUnknown {
    fn from(value: ITrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ITrigger> for ::windows_core::IUnknown {
    fn from(value: &ITrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ITrigger> for super::Com::IDispatch {
    fn from(value: ITrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ITrigger> for super::Com::IDispatch {
    fn from(value: &ITrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ITrigger {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ITrigger {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ITrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ITrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ITrigger {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ITrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITrigger").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ITrigger {
    type Vtable = ITrigger_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x09941815_ea89_4b5b_89e0_2a773801fac3);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ITrigger_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut TASK_TRIGGER_TYPE2) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Repetition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprepeat: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Repetition: usize,
    #[cfg(feature = "win32-system")]
    pub SetRepetition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prepeat: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetRepetition: usize,
    pub ExecutionTimeLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptimelimit: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetExecutionTimeLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timelimit: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub StartBoundary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstart: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetStartBoundary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, start: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub EndBoundary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pend: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetEndBoundary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, end: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penabled: *mut i16) -> ::windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ITriggerCollection(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ITriggerCollection {
    pub unsafe fn Count(&self, pcount: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcount)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows_core::Result<ITrigger> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITrigger>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Create(&self, r#type: TASK_TRIGGER_TYPE2) -> ::windows_core::Result<ITrigger> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Create)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(r#type), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITrigger>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Remove<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), index.into_param().abi()).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Clear)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ITriggerCollection> for ::windows_core::IUnknown {
    fn from(value: ITriggerCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ITriggerCollection> for ::windows_core::IUnknown {
    fn from(value: &ITriggerCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITriggerCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITriggerCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ITriggerCollection> for super::Com::IDispatch {
    fn from(value: ITriggerCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ITriggerCollection> for super::Com::IDispatch {
    fn from(value: &ITriggerCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ITriggerCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ITriggerCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ITriggerCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ITriggerCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ITriggerCollection {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ITriggerCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITriggerCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ITriggerCollection {
    type Vtable = ITriggerCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x85df5081_1b24_4f32_878a_d9d14df4cb77);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ITriggerCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pptrigger: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: TASK_TRIGGER_TYPE2, pptrigger: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Create: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IWeeklyTrigger(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IWeeklyTrigger {
    pub unsafe fn Type(&self, ptype: *mut TASK_TRIGGER_TYPE2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    pub unsafe fn Id(&self, pid: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    pub unsafe fn SetId<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, id: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetId)(::windows_core::Interface::as_raw(self), id.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Repetition(&self) -> ::windows_core::Result<IRepetitionPattern> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Repetition)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRepetitionPattern>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetRepetition<'a, Param0: ::windows_core::IntoParam<'a, IRepetitionPattern>>(&self, prepeat: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRepetition)(::windows_core::Interface::as_raw(self), prepeat.into_param().abi()).ok()
    }
    pub unsafe fn ExecutionTimeLimit(&self, ptimelimit: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ExecutionTimeLimit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptimelimit)).ok()
    }
    pub unsafe fn SetExecutionTimeLimit<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, timelimit: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetExecutionTimeLimit)(::windows_core::Interface::as_raw(self), timelimit.into_param().abi()).ok()
    }
    pub unsafe fn StartBoundary(&self, pstart: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StartBoundary)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pstart)).ok()
    }
    pub unsafe fn SetStartBoundary<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, start: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetStartBoundary)(::windows_core::Interface::as_raw(self), start.into_param().abi()).ok()
    }
    pub unsafe fn EndBoundary(&self, pend: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EndBoundary)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pend)).ok()
    }
    pub unsafe fn SetEndBoundary<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, end: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEndBoundary)(::windows_core::Interface::as_raw(self), end.into_param().abi()).ok()
    }
    pub unsafe fn Enabled(&self, penabled: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Enabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(penabled)).ok()
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enabled)).ok()
    }
    pub unsafe fn DaysOfWeek(&self, pdays: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DaysOfWeek)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdays)).ok()
    }
    pub unsafe fn SetDaysOfWeek(&self, days: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDaysOfWeek)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(days)).ok()
    }
    pub unsafe fn WeeksInterval(&self, pweeks: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WeeksInterval)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pweeks)).ok()
    }
    pub unsafe fn SetWeeksInterval(&self, weeks: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetWeeksInterval)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(weeks)).ok()
    }
    pub unsafe fn RandomDelay(&self, prandomdelay: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RandomDelay)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(prandomdelay)).ok()
    }
    pub unsafe fn SetRandomDelay<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, randomdelay: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRandomDelay)(::windows_core::Interface::as_raw(self), randomdelay.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IWeeklyTrigger> for ::windows_core::IUnknown {
    fn from(value: IWeeklyTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IWeeklyTrigger> for ::windows_core::IUnknown {
    fn from(value: &IWeeklyTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWeeklyTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWeeklyTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IWeeklyTrigger> for super::Com::IDispatch {
    fn from(value: IWeeklyTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IWeeklyTrigger> for super::Com::IDispatch {
    fn from(value: &IWeeklyTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IWeeklyTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IWeeklyTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IWeeklyTrigger> for ITrigger {
    fn from(value: IWeeklyTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IWeeklyTrigger> for ITrigger {
    fn from(value: &IWeeklyTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ITrigger> for IWeeklyTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ITrigger> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ITrigger> for &'a IWeeklyTrigger {
    fn into_param(self) -> ::windows_core::Param<'a, ITrigger> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IWeeklyTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IWeeklyTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IWeeklyTrigger {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IWeeklyTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWeeklyTrigger").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IWeeklyTrigger {
    type Vtable = IWeeklyTrigger_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5038fc98_82ff_436d_8728_a512a57c9dc1);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IWeeklyTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub DaysOfWeek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdays: *mut i16) -> ::windows_core::HRESULT,
    pub SetDaysOfWeek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, days: i16) -> ::windows_core::HRESULT,
    pub WeeksInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pweeks: *mut i16) -> ::windows_core::HRESULT,
    pub SetWeeksInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, weeks: i16) -> ::windows_core::HRESULT,
    pub RandomDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prandomdelay: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetRandomDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, randomdelay: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(C)]
pub struct MONTHLYDATE {
    pub rgfDays: u32,
    pub rgfMonths: u16,
}
impl ::core::marker::Copy for MONTHLYDATE {}
impl ::core::clone::Clone for MONTHLYDATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MONTHLYDATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MONTHLYDATE").field("rgfDays", &self.rgfDays).field("rgfMonths", &self.rgfMonths).finish()
    }
}
unsafe impl ::windows_core::Abi for MONTHLYDATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MONTHLYDATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MONTHLYDATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for MONTHLYDATE {}
impl ::core::default::Default for MONTHLYDATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MONTHLYDOW {
    pub wWhichWeek: u16,
    pub rgfDaysOfTheWeek: u16,
    pub rgfMonths: u16,
}
impl ::core::marker::Copy for MONTHLYDOW {}
impl ::core::clone::Clone for MONTHLYDOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MONTHLYDOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MONTHLYDOW").field("wWhichWeek", &self.wWhichWeek).field("rgfDaysOfTheWeek", &self.rgfDaysOfTheWeek).field("rgfMonths", &self.rgfMonths).finish()
    }
}
unsafe impl ::windows_core::Abi for MONTHLYDOW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MONTHLYDOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MONTHLYDOW>()) == 0 }
    }
}
impl ::core::cmp::Eq for MONTHLYDOW {}
impl ::core::default::Default for MONTHLYDOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TASKPAGE(pub i32);
pub const TASKPAGE_TASK: TASKPAGE = TASKPAGE(0i32);
pub const TASKPAGE_SCHEDULE: TASKPAGE = TASKPAGE(1i32);
pub const TASKPAGE_SETTINGS: TASKPAGE = TASKPAGE(2i32);
impl ::core::marker::Copy for TASKPAGE {}
impl ::core::clone::Clone for TASKPAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TASKPAGE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TASKPAGE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TASKPAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASKPAGE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TASK_ACTION_TYPE(pub i32);
pub const TASK_ACTION_EXEC: TASK_ACTION_TYPE = TASK_ACTION_TYPE(0i32);
pub const TASK_ACTION_COM_HANDLER: TASK_ACTION_TYPE = TASK_ACTION_TYPE(5i32);
pub const TASK_ACTION_SEND_EMAIL: TASK_ACTION_TYPE = TASK_ACTION_TYPE(6i32);
pub const TASK_ACTION_SHOW_MESSAGE: TASK_ACTION_TYPE = TASK_ACTION_TYPE(7i32);
impl ::core::marker::Copy for TASK_ACTION_TYPE {}
impl ::core::clone::Clone for TASK_ACTION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TASK_ACTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TASK_ACTION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TASK_ACTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASK_ACTION_TYPE").field(&self.0).finish()
    }
}
pub const TASK_APRIL: u32 = 8u32;
pub const TASK_AUGUST: u32 = 128u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TASK_COMPATIBILITY(pub i32);
pub const TASK_COMPATIBILITY_AT: TASK_COMPATIBILITY = TASK_COMPATIBILITY(0i32);
pub const TASK_COMPATIBILITY_V1: TASK_COMPATIBILITY = TASK_COMPATIBILITY(1i32);
pub const TASK_COMPATIBILITY_V2: TASK_COMPATIBILITY = TASK_COMPATIBILITY(2i32);
pub const TASK_COMPATIBILITY_V2_1: TASK_COMPATIBILITY = TASK_COMPATIBILITY(3i32);
pub const TASK_COMPATIBILITY_V2_2: TASK_COMPATIBILITY = TASK_COMPATIBILITY(4i32);
pub const TASK_COMPATIBILITY_V2_3: TASK_COMPATIBILITY = TASK_COMPATIBILITY(5i32);
pub const TASK_COMPATIBILITY_V2_4: TASK_COMPATIBILITY = TASK_COMPATIBILITY(6i32);
impl ::core::marker::Copy for TASK_COMPATIBILITY {}
impl ::core::clone::Clone for TASK_COMPATIBILITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TASK_COMPATIBILITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TASK_COMPATIBILITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for TASK_COMPATIBILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASK_COMPATIBILITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TASK_CREATION(pub i32);
pub const TASK_VALIDATE_ONLY: TASK_CREATION = TASK_CREATION(1i32);
pub const TASK_CREATE: TASK_CREATION = TASK_CREATION(2i32);
pub const TASK_UPDATE: TASK_CREATION = TASK_CREATION(4i32);
pub const TASK_CREATE_OR_UPDATE: TASK_CREATION = TASK_CREATION(6i32);
pub const TASK_DISABLE: TASK_CREATION = TASK_CREATION(8i32);
pub const TASK_DONT_ADD_PRINCIPAL_ACE: TASK_CREATION = TASK_CREATION(16i32);
pub const TASK_IGNORE_REGISTRATION_TRIGGERS: TASK_CREATION = TASK_CREATION(32i32);
impl ::core::marker::Copy for TASK_CREATION {}
impl ::core::clone::Clone for TASK_CREATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TASK_CREATION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TASK_CREATION {
    type Abi = Self;
}
impl ::core::fmt::Debug for TASK_CREATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASK_CREATION").field(&self.0).finish()
    }
}
pub const TASK_DECEMBER: u32 = 2048u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TASK_ENUM_FLAGS(pub i32);
pub const TASK_ENUM_HIDDEN: TASK_ENUM_FLAGS = TASK_ENUM_FLAGS(1i32);
impl ::core::marker::Copy for TASK_ENUM_FLAGS {}
impl ::core::clone::Clone for TASK_ENUM_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TASK_ENUM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TASK_ENUM_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for TASK_ENUM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASK_ENUM_FLAGS").field(&self.0).finish()
    }
}
pub const TASK_FEBRUARY: u32 = 2u32;
pub const TASK_FIRST_WEEK: u32 = 1u32;
pub const TASK_FLAG_DELETE_WHEN_DONE: u32 = 2u32;
pub const TASK_FLAG_DISABLED: u32 = 4u32;
pub const TASK_FLAG_DONT_START_IF_ON_BATTERIES: u32 = 64u32;
pub const TASK_FLAG_HIDDEN: u32 = 512u32;
pub const TASK_FLAG_INTERACTIVE: u32 = 1u32;
pub const TASK_FLAG_KILL_IF_GOING_ON_BATTERIES: u32 = 128u32;
pub const TASK_FLAG_KILL_ON_IDLE_END: u32 = 32u32;
pub const TASK_FLAG_RESTART_ON_IDLE_RESUME: u32 = 2048u32;
pub const TASK_FLAG_RUN_IF_CONNECTED_TO_INTERNET: u32 = 1024u32;
pub const TASK_FLAG_RUN_ONLY_IF_DOCKED: u32 = 256u32;
pub const TASK_FLAG_RUN_ONLY_IF_LOGGED_ON: u32 = 8192u32;
pub const TASK_FLAG_START_ONLY_IF_IDLE: u32 = 16u32;
pub const TASK_FLAG_SYSTEM_REQUIRED: u32 = 4096u32;
pub const TASK_FOURTH_WEEK: u32 = 4u32;
pub const TASK_FRIDAY: u32 = 32u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TASK_INSTANCES_POLICY(pub i32);
pub const TASK_INSTANCES_PARALLEL: TASK_INSTANCES_POLICY = TASK_INSTANCES_POLICY(0i32);
pub const TASK_INSTANCES_QUEUE: TASK_INSTANCES_POLICY = TASK_INSTANCES_POLICY(1i32);
pub const TASK_INSTANCES_IGNORE_NEW: TASK_INSTANCES_POLICY = TASK_INSTANCES_POLICY(2i32);
pub const TASK_INSTANCES_STOP_EXISTING: TASK_INSTANCES_POLICY = TASK_INSTANCES_POLICY(3i32);
impl ::core::marker::Copy for TASK_INSTANCES_POLICY {}
impl ::core::clone::Clone for TASK_INSTANCES_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TASK_INSTANCES_POLICY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TASK_INSTANCES_POLICY {
    type Abi = Self;
}
impl ::core::fmt::Debug for TASK_INSTANCES_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASK_INSTANCES_POLICY").field(&self.0).finish()
    }
}
pub const TASK_JANUARY: u32 = 1u32;
pub const TASK_JULY: u32 = 64u32;
pub const TASK_JUNE: u32 = 32u32;
pub const TASK_LAST_WEEK: u32 = 5u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TASK_LOGON_TYPE(pub i32);
pub const TASK_LOGON_NONE: TASK_LOGON_TYPE = TASK_LOGON_TYPE(0i32);
pub const TASK_LOGON_PASSWORD: TASK_LOGON_TYPE = TASK_LOGON_TYPE(1i32);
pub const TASK_LOGON_S4U: TASK_LOGON_TYPE = TASK_LOGON_TYPE(2i32);
pub const TASK_LOGON_INTERACTIVE_TOKEN: TASK_LOGON_TYPE = TASK_LOGON_TYPE(3i32);
pub const TASK_LOGON_GROUP: TASK_LOGON_TYPE = TASK_LOGON_TYPE(4i32);
pub const TASK_LOGON_SERVICE_ACCOUNT: TASK_LOGON_TYPE = TASK_LOGON_TYPE(5i32);
pub const TASK_LOGON_INTERACTIVE_TOKEN_OR_PASSWORD: TASK_LOGON_TYPE = TASK_LOGON_TYPE(6i32);
impl ::core::marker::Copy for TASK_LOGON_TYPE {}
impl ::core::clone::Clone for TASK_LOGON_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TASK_LOGON_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TASK_LOGON_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TASK_LOGON_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASK_LOGON_TYPE").field(&self.0).finish()
    }
}
pub const TASK_MARCH: u32 = 4u32;
pub const TASK_MAX_RUN_TIMES: u32 = 1440u32;
pub const TASK_MAY: u32 = 16u32;
pub const TASK_MONDAY: u32 = 2u32;
pub const TASK_NOVEMBER: u32 = 1024u32;
pub const TASK_OCTOBER: u32 = 512u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TASK_PROCESSTOKENSID_TYPE(pub i32);
pub const TASK_PROCESSTOKENSID_NONE: TASK_PROCESSTOKENSID_TYPE = TASK_PROCESSTOKENSID_TYPE(0i32);
pub const TASK_PROCESSTOKENSID_UNRESTRICTED: TASK_PROCESSTOKENSID_TYPE = TASK_PROCESSTOKENSID_TYPE(1i32);
pub const TASK_PROCESSTOKENSID_DEFAULT: TASK_PROCESSTOKENSID_TYPE = TASK_PROCESSTOKENSID_TYPE(2i32);
impl ::core::marker::Copy for TASK_PROCESSTOKENSID_TYPE {}
impl ::core::clone::Clone for TASK_PROCESSTOKENSID_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TASK_PROCESSTOKENSID_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TASK_PROCESSTOKENSID_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TASK_PROCESSTOKENSID_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASK_PROCESSTOKENSID_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TASK_RUNLEVEL_TYPE(pub i32);
pub const TASK_RUNLEVEL_LUA: TASK_RUNLEVEL_TYPE = TASK_RUNLEVEL_TYPE(0i32);
pub const TASK_RUNLEVEL_HIGHEST: TASK_RUNLEVEL_TYPE = TASK_RUNLEVEL_TYPE(1i32);
impl ::core::marker::Copy for TASK_RUNLEVEL_TYPE {}
impl ::core::clone::Clone for TASK_RUNLEVEL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TASK_RUNLEVEL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TASK_RUNLEVEL_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TASK_RUNLEVEL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASK_RUNLEVEL_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TASK_RUN_FLAGS(pub i32);
pub const TASK_RUN_NO_FLAGS: TASK_RUN_FLAGS = TASK_RUN_FLAGS(0i32);
pub const TASK_RUN_AS_SELF: TASK_RUN_FLAGS = TASK_RUN_FLAGS(1i32);
pub const TASK_RUN_IGNORE_CONSTRAINTS: TASK_RUN_FLAGS = TASK_RUN_FLAGS(2i32);
pub const TASK_RUN_USE_SESSION_ID: TASK_RUN_FLAGS = TASK_RUN_FLAGS(4i32);
pub const TASK_RUN_USER_SID: TASK_RUN_FLAGS = TASK_RUN_FLAGS(8i32);
impl ::core::marker::Copy for TASK_RUN_FLAGS {}
impl ::core::clone::Clone for TASK_RUN_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TASK_RUN_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TASK_RUN_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for TASK_RUN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASK_RUN_FLAGS").field(&self.0).finish()
    }
}
pub const TASK_SATURDAY: u32 = 64u32;
pub const TASK_SECOND_WEEK: u32 = 2u32;
pub const TASK_SEPTEMBER: u32 = 256u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TASK_SESSION_STATE_CHANGE_TYPE(pub i32);
pub const TASK_CONSOLE_CONNECT: TASK_SESSION_STATE_CHANGE_TYPE = TASK_SESSION_STATE_CHANGE_TYPE(1i32);
pub const TASK_CONSOLE_DISCONNECT: TASK_SESSION_STATE_CHANGE_TYPE = TASK_SESSION_STATE_CHANGE_TYPE(2i32);
pub const TASK_REMOTE_CONNECT: TASK_SESSION_STATE_CHANGE_TYPE = TASK_SESSION_STATE_CHANGE_TYPE(3i32);
pub const TASK_REMOTE_DISCONNECT: TASK_SESSION_STATE_CHANGE_TYPE = TASK_SESSION_STATE_CHANGE_TYPE(4i32);
pub const TASK_SESSION_LOCK: TASK_SESSION_STATE_CHANGE_TYPE = TASK_SESSION_STATE_CHANGE_TYPE(7i32);
pub const TASK_SESSION_UNLOCK: TASK_SESSION_STATE_CHANGE_TYPE = TASK_SESSION_STATE_CHANGE_TYPE(8i32);
impl ::core::marker::Copy for TASK_SESSION_STATE_CHANGE_TYPE {}
impl ::core::clone::Clone for TASK_SESSION_STATE_CHANGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TASK_SESSION_STATE_CHANGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TASK_SESSION_STATE_CHANGE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TASK_SESSION_STATE_CHANGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASK_SESSION_STATE_CHANGE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TASK_STATE(pub i32);
pub const TASK_STATE_UNKNOWN: TASK_STATE = TASK_STATE(0i32);
pub const TASK_STATE_DISABLED: TASK_STATE = TASK_STATE(1i32);
pub const TASK_STATE_QUEUED: TASK_STATE = TASK_STATE(2i32);
pub const TASK_STATE_READY: TASK_STATE = TASK_STATE(3i32);
pub const TASK_STATE_RUNNING: TASK_STATE = TASK_STATE(4i32);
impl ::core::marker::Copy for TASK_STATE {}
impl ::core::clone::Clone for TASK_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TASK_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TASK_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TASK_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASK_STATE").field(&self.0).finish()
    }
}
pub const TASK_SUNDAY: u32 = 1u32;
pub const TASK_THIRD_WEEK: u32 = 3u32;
pub const TASK_THURSDAY: u32 = 16u32;
#[repr(C)]
pub struct TASK_TRIGGER {
    pub cbTriggerSize: u16,
    pub Reserved1: u16,
    pub wBeginYear: u16,
    pub wBeginMonth: u16,
    pub wBeginDay: u16,
    pub wEndYear: u16,
    pub wEndMonth: u16,
    pub wEndDay: u16,
    pub wStartHour: u16,
    pub wStartMinute: u16,
    pub MinutesDuration: u32,
    pub MinutesInterval: u32,
    pub rgFlags: u32,
    pub TriggerType: TASK_TRIGGER_TYPE,
    pub Type: TRIGGER_TYPE_UNION,
    pub Reserved2: u16,
    pub wRandomMinutesInterval: u16,
}
impl ::core::marker::Copy for TASK_TRIGGER {}
impl ::core::clone::Clone for TASK_TRIGGER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for TASK_TRIGGER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TASK_TRIGGER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TASK_TRIGGER>()) == 0 }
    }
}
impl ::core::cmp::Eq for TASK_TRIGGER {}
impl ::core::default::Default for TASK_TRIGGER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const TASK_TRIGGER_FLAG_DISABLED: u32 = 4u32;
pub const TASK_TRIGGER_FLAG_HAS_END_DATE: u32 = 1u32;
pub const TASK_TRIGGER_FLAG_KILL_AT_DURATION_END: u32 = 2u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TASK_TRIGGER_TYPE(pub i32);
pub const TASK_TIME_TRIGGER_ONCE: TASK_TRIGGER_TYPE = TASK_TRIGGER_TYPE(0i32);
pub const TASK_TIME_TRIGGER_DAILY: TASK_TRIGGER_TYPE = TASK_TRIGGER_TYPE(1i32);
pub const TASK_TIME_TRIGGER_WEEKLY: TASK_TRIGGER_TYPE = TASK_TRIGGER_TYPE(2i32);
pub const TASK_TIME_TRIGGER_MONTHLYDATE: TASK_TRIGGER_TYPE = TASK_TRIGGER_TYPE(3i32);
pub const TASK_TIME_TRIGGER_MONTHLYDOW: TASK_TRIGGER_TYPE = TASK_TRIGGER_TYPE(4i32);
pub const TASK_EVENT_TRIGGER_ON_IDLE: TASK_TRIGGER_TYPE = TASK_TRIGGER_TYPE(5i32);
pub const TASK_EVENT_TRIGGER_AT_SYSTEMSTART: TASK_TRIGGER_TYPE = TASK_TRIGGER_TYPE(6i32);
pub const TASK_EVENT_TRIGGER_AT_LOGON: TASK_TRIGGER_TYPE = TASK_TRIGGER_TYPE(7i32);
impl ::core::marker::Copy for TASK_TRIGGER_TYPE {}
impl ::core::clone::Clone for TASK_TRIGGER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TASK_TRIGGER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TASK_TRIGGER_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TASK_TRIGGER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASK_TRIGGER_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TASK_TRIGGER_TYPE2(pub i32);
pub const TASK_TRIGGER_EVENT: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(0i32);
pub const TASK_TRIGGER_TIME: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(1i32);
pub const TASK_TRIGGER_DAILY: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(2i32);
pub const TASK_TRIGGER_WEEKLY: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(3i32);
pub const TASK_TRIGGER_MONTHLY: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(4i32);
pub const TASK_TRIGGER_MONTHLYDOW: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(5i32);
pub const TASK_TRIGGER_IDLE: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(6i32);
pub const TASK_TRIGGER_REGISTRATION: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(7i32);
pub const TASK_TRIGGER_BOOT: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(8i32);
pub const TASK_TRIGGER_LOGON: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(9i32);
pub const TASK_TRIGGER_SESSION_STATE_CHANGE: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(11i32);
pub const TASK_TRIGGER_CUSTOM_TRIGGER_01: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(12i32);
impl ::core::marker::Copy for TASK_TRIGGER_TYPE2 {}
impl ::core::clone::Clone for TASK_TRIGGER_TYPE2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TASK_TRIGGER_TYPE2 {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TASK_TRIGGER_TYPE2 {
    type Abi = Self;
}
impl ::core::fmt::Debug for TASK_TRIGGER_TYPE2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASK_TRIGGER_TYPE2").field(&self.0).finish()
    }
}
pub const TASK_TUESDAY: u32 = 4u32;
pub const TASK_WEDNESDAY: u32 = 8u32;
#[repr(C)]
pub union TRIGGER_TYPE_UNION {
    pub Daily: DAILY,
    pub Weekly: WEEKLY,
    pub MonthlyDate: MONTHLYDATE,
    pub MonthlyDOW: MONTHLYDOW,
}
impl ::core::marker::Copy for TRIGGER_TYPE_UNION {}
impl ::core::clone::Clone for TRIGGER_TYPE_UNION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for TRIGGER_TYPE_UNION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TRIGGER_TYPE_UNION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRIGGER_TYPE_UNION>()) == 0 }
    }
}
impl ::core::cmp::Eq for TRIGGER_TYPE_UNION {}
impl ::core::default::Default for TRIGGER_TYPE_UNION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const TaskHandlerPS: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf2a69db7_da2c_4352_9066_86fee6dacac9);
pub const TaskHandlerStatusPS: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f15266d_d7ba_48f0_93c1_e6895f6fe5ac);
pub const TaskScheduler: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0f87369f_a4e5_4cfc_bd3e_73e6154572dd);
#[repr(C)]
pub struct WEEKLY {
    pub WeeksInterval: u16,
    pub rgfDaysOfTheWeek: u16,
}
impl ::core::marker::Copy for WEEKLY {}
impl ::core::clone::Clone for WEEKLY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEEKLY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEEKLY").field("WeeksInterval", &self.WeeksInterval).field("rgfDaysOfTheWeek", &self.rgfDaysOfTheWeek).finish()
    }
}
unsafe impl ::windows_core::Abi for WEEKLY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WEEKLY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WEEKLY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WEEKLY {}
impl ::core::default::Default for WEEKLY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
