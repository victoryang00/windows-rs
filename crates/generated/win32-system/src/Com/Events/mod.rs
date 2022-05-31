pub const CEventClass: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcdbec9c0_7a68_11d1_88f9_0080c7d771bf);
pub const CEventPublisher: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xab944620_79c6_11d1_88f9_0080c7d771bf);
pub const CEventSubscription: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7542e960_79c7_11d1_88f9_0080c7d771bf);
pub const CEventSystem: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4e14fba2_2e22_11d1_9964_00c04fbbb345);
#[repr(C)]
pub struct COMEVENTSYSCHANGEINFO {
    pub cbSize: u32,
    pub changeType: EOC_ChangeType,
    pub objectId: ::win32_foundation::BSTR,
    pub partitionId: ::win32_foundation::BSTR,
    pub applicationId: ::win32_foundation::BSTR,
    pub reserved: [::windows_core::GUID; 10],
}
impl ::core::clone::Clone for COMEVENTSYSCHANGEINFO {
    fn clone(&self) -> Self {
        Self {
            cbSize: self.cbSize,
            changeType: self.changeType,
            objectId: self.objectId.clone(),
            partitionId: self.partitionId.clone(),
            applicationId: self.applicationId.clone(),
            reserved: self.reserved,
        }
    }
}
impl ::core::fmt::Debug for COMEVENTSYSCHANGEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMEVENTSYSCHANGEINFO").field("cbSize", &self.cbSize).field("changeType", &self.changeType).field("objectId", &self.objectId).field("partitionId", &self.partitionId).field("applicationId", &self.applicationId).field("reserved", &self.reserved).finish()
    }
}
unsafe impl ::windows_core::Abi for COMEVENTSYSCHANGEINFO {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for COMEVENTSYSCHANGEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.changeType == other.changeType && self.objectId == other.objectId && self.partitionId == other.partitionId && self.applicationId == other.applicationId && self.reserved == other.reserved
    }
}
impl ::core::cmp::Eq for COMEVENTSYSCHANGEINFO {}
impl ::core::default::Default for COMEVENTSYSCHANGEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EOC_ChangeType(pub i32);
pub const EOC_NewObject: EOC_ChangeType = EOC_ChangeType(0i32);
pub const EOC_ModifiedObject: EOC_ChangeType = EOC_ChangeType(1i32);
pub const EOC_DeletedObject: EOC_ChangeType = EOC_ChangeType(2i32);
impl ::core::marker::Copy for EOC_ChangeType {}
impl ::core::clone::Clone for EOC_ChangeType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EOC_ChangeType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EOC_ChangeType {
    type Abi = Self;
}
impl ::core::fmt::Debug for EOC_ChangeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EOC_ChangeType").field(&self.0).finish()
    }
}
pub const EventObjectChange: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd0565000_9df4_11d1_a281_00c04fca0aa7);
pub const EventObjectChange2: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbb07bacd_cd56_4e63_a8ff_cbf0355fb9f4);
#[repr(transparent)]
pub struct IDontSupportEventSubscription(::windows_core::IUnknown);
impl IDontSupportEventSubscription {}
impl ::core::convert::From<IDontSupportEventSubscription> for ::windows_core::IUnknown {
    fn from(value: IDontSupportEventSubscription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDontSupportEventSubscription> for ::windows_core::IUnknown {
    fn from(value: &IDontSupportEventSubscription) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDontSupportEventSubscription {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDontSupportEventSubscription {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDontSupportEventSubscription {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDontSupportEventSubscription {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDontSupportEventSubscription {}
impl ::core::fmt::Debug for IDontSupportEventSubscription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDontSupportEventSubscription").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDontSupportEventSubscription {
    type Vtable = IDontSupportEventSubscription_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x784121f1_62a6_4b89_855f_d65f296de83a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDontSupportEventSubscription_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
}
#[repr(transparent)]
pub struct IEnumEventObject(::windows_core::IUnknown);
impl IEnumEventObject {
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumEventObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumEventObject>(result__)
    }
    pub unsafe fn Next(&self, ppinterface: &mut [::core::option::Option<::windows_core::IUnknown>], cretelem: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ppinterface.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(ppinterface)), ::core::mem::transmute(cretelem)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, cskipelem: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cskipelem)).ok()
    }
}
impl ::core::convert::From<IEnumEventObject> for ::windows_core::IUnknown {
    fn from(value: IEnumEventObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumEventObject> for ::windows_core::IUnknown {
    fn from(value: &IEnumEventObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumEventObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumEventObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumEventObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumEventObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumEventObject {}
impl ::core::fmt::Debug for IEnumEventObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumEventObject").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumEventObject {
    type Vtable = IEnumEventObject_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf4a07d63_2e25_11d1_9964_00c04fbbb345);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumEventObject_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppinterface: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, creqelem: u32, ppinterface: *mut *mut ::core::ffi::c_void, cretelem: *mut u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cskipelem: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEventClass(::windows_core::IUnknown);
impl IEventClass {
    pub unsafe fn EventClassID(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).EventClassID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetEventClassID<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstreventclassid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEventClassID)(::windows_core::Interface::as_raw(self), bstreventclassid.into_param().abi()).ok()
    }
    pub unsafe fn EventClassName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).EventClassName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetEventClassName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstreventclassname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEventClassName)(::windows_core::Interface::as_raw(self), bstreventclassname.into_param().abi()).ok()
    }
    pub unsafe fn OwnerSID(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).OwnerSID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetOwnerSID<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrownersid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOwnerSID)(::windows_core::Interface::as_raw(self), bstrownersid.into_param().abi()).ok()
    }
    pub unsafe fn FiringInterfaceID(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).FiringInterfaceID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetFiringInterfaceID<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrfiringinterfaceid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFiringInterfaceID)(::windows_core::Interface::as_raw(self), bstrfiringinterfaceid.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdescription: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn CustomConfigCLSID(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).CustomConfigCLSID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetCustomConfigCLSID<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrcustomconfigclsid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCustomConfigCLSID)(::windows_core::Interface::as_raw(self), bstrcustomconfigclsid.into_param().abi()).ok()
    }
    pub unsafe fn TypeLib(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).TypeLib)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetTypeLib<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrtypelib: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTypeLib)(::windows_core::Interface::as_raw(self), bstrtypelib.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IEventClass> for ::windows_core::IUnknown {
    fn from(value: IEventClass) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventClass> for ::windows_core::IUnknown {
    fn from(value: &IEventClass) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEventClass {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEventClass {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IEventClass> for super::IDispatch {
    fn from(value: IEventClass) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventClass> for super::IDispatch {
    fn from(value: &IEventClass) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IDispatch> for IEventClass {
    fn into_param(self) -> ::windows_core::Param<'a, super::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IDispatch> for &'a IEventClass {
    fn into_param(self) -> ::windows_core::Param<'a, super::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEventClass {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEventClass {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventClass {}
impl ::core::fmt::Debug for IEventClass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventClass").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEventClass {
    type Vtable = IEventClass_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfb2b72a0_7a68_11d1_88f9_0080c7d771bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventClass_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub EventClassID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstreventclassid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetEventClassID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstreventclassid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub EventClassName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstreventclassname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetEventClassName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstreventclassname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub OwnerSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrownersid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetOwnerSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrownersid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub FiringInterfaceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfiringinterfaceid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetFiringInterfaceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfiringinterfaceid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub CustomConfigCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcustomconfigclsid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetCustomConfigCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcustomconfigclsid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub TypeLib: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtypelib: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetTypeLib: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtypelib: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEventClass2(::windows_core::IUnknown);
impl IEventClass2 {
    pub unsafe fn EventClassID(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.EventClassID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetEventClassID<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstreventclassid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEventClassID)(::windows_core::Interface::as_raw(self), bstreventclassid.into_param().abi()).ok()
    }
    pub unsafe fn EventClassName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.EventClassName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetEventClassName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstreventclassname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEventClassName)(::windows_core::Interface::as_raw(self), bstreventclassname.into_param().abi()).ok()
    }
    pub unsafe fn OwnerSID(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.OwnerSID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetOwnerSID<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrownersid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetOwnerSID)(::windows_core::Interface::as_raw(self), bstrownersid.into_param().abi()).ok()
    }
    pub unsafe fn FiringInterfaceID(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.FiringInterfaceID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetFiringInterfaceID<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrfiringinterfaceid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetFiringInterfaceID)(::windows_core::Interface::as_raw(self), bstrfiringinterfaceid.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdescription: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn CustomConfigCLSID(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CustomConfigCLSID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetCustomConfigCLSID<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrcustomconfigclsid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetCustomConfigCLSID)(::windows_core::Interface::as_raw(self), bstrcustomconfigclsid.into_param().abi()).ok()
    }
    pub unsafe fn TypeLib(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.TypeLib)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetTypeLib<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrtypelib: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetTypeLib)(::windows_core::Interface::as_raw(self), bstrtypelib.into_param().abi()).ok()
    }
    pub unsafe fn PublisherID(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).PublisherID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetPublisherID<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpublisherid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPublisherID)(::windows_core::Interface::as_raw(self), bstrpublisherid.into_param().abi()).ok()
    }
    pub unsafe fn MultiInterfacePublisherFilterCLSID(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).MultiInterfacePublisherFilterCLSID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetMultiInterfacePublisherFilterCLSID<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpubfilclsid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMultiInterfacePublisherFilterCLSID)(::windows_core::Interface::as_raw(self), bstrpubfilclsid.into_param().abi()).ok()
    }
    pub unsafe fn AllowInprocActivation(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).AllowInprocActivation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetAllowInprocActivation<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fallowinprocactivation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAllowInprocActivation)(::windows_core::Interface::as_raw(self), fallowinprocactivation.into_param().abi()).ok()
    }
    pub unsafe fn FireInParallel(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).FireInParallel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetFireInParallel<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, ffireinparallel: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFireInParallel)(::windows_core::Interface::as_raw(self), ffireinparallel.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IEventClass2> for ::windows_core::IUnknown {
    fn from(value: IEventClass2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventClass2> for ::windows_core::IUnknown {
    fn from(value: &IEventClass2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEventClass2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEventClass2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IEventClass2> for super::IDispatch {
    fn from(value: IEventClass2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventClass2> for super::IDispatch {
    fn from(value: &IEventClass2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IDispatch> for IEventClass2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IDispatch> for &'a IEventClass2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IEventClass2> for IEventClass {
    fn from(value: IEventClass2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventClass2> for IEventClass {
    fn from(value: &IEventClass2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IEventClass> for IEventClass2 {
    fn into_param(self) -> ::windows_core::Param<'a, IEventClass> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IEventClass> for &'a IEventClass2 {
    fn into_param(self) -> ::windows_core::Param<'a, IEventClass> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEventClass2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEventClass2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventClass2 {}
impl ::core::fmt::Debug for IEventClass2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventClass2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEventClass2 {
    type Vtable = IEventClass2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfb2b72a1_7a68_11d1_88f9_0080c7d771bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventClass2_Vtbl {
    pub base__: IEventClass_Vtbl,
    pub PublisherID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpublisherid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetPublisherID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpublisherid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub MultiInterfacePublisherFilterCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpubfilclsid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetMultiInterfacePublisherFilterCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpubfilclsid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub AllowInprocActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfallowinprocactivation: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetAllowInprocActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fallowinprocactivation: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub FireInParallel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pffireinparallel: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetFireInParallel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ffireinparallel: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEventControl(::windows_core::IUnknown);
impl IEventControl {
    pub unsafe fn SetPublisherFilter<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, IPublisherFilter>>(&self, methodname: Param0, ppublisherfilter: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPublisherFilter)(::windows_core::Interface::as_raw(self), methodname.into_param().abi(), ppublisherfilter.into_param().abi()).ok()
    }
    pub unsafe fn AllowInprocActivation(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).AllowInprocActivation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetAllowInprocActivation<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fallowinprocactivation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAllowInprocActivation)(::windows_core::Interface::as_raw(self), fallowinprocactivation.into_param().abi()).ok()
    }
    pub unsafe fn GetSubscriptions<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, methodname: Param0, optionalcriteria: Param1, optionalerrorindex: *const i32) -> ::windows_core::Result<IEventObjectCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSubscriptions)(::windows_core::Interface::as_raw(self), methodname.into_param().abi(), optionalcriteria.into_param().abi(), ::core::mem::transmute(optionalerrorindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEventObjectCollection>(result__)
    }
    pub unsafe fn SetDefaultQuery<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, methodname: Param0, criteria: Param1) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).SetDefaultQuery)(::windows_core::Interface::as_raw(self), methodname.into_param().abi(), criteria.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
impl ::core::convert::From<IEventControl> for ::windows_core::IUnknown {
    fn from(value: IEventControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventControl> for ::windows_core::IUnknown {
    fn from(value: &IEventControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEventControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEventControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IEventControl> for super::IDispatch {
    fn from(value: IEventControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventControl> for super::IDispatch {
    fn from(value: &IEventControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IDispatch> for IEventControl {
    fn into_param(self) -> ::windows_core::Param<'a, super::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IDispatch> for &'a IEventControl {
    fn into_param(self) -> ::windows_core::Param<'a, super::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEventControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEventControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventControl {}
impl ::core::fmt::Debug for IEventControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventControl").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEventControl {
    type Vtable = IEventControl_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0343e2f4_86f6_11d1_b760_00c04fb926af);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventControl_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub SetPublisherFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, methodname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppublisherfilter: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AllowInprocActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfallowinprocactivation: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetAllowInprocActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fallowinprocactivation: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetSubscriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, methodname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, optionalcriteria: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, optionalerrorindex: *const i32, ppcollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetDefaultQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, methodname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, criteria: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, errorindex: *mut i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEventObjectChange(::windows_core::IUnknown);
impl IEventObjectChange {
    pub unsafe fn ChangedSubscription<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, changetype: EOC_ChangeType, bstrsubscriptionid: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ChangedSubscription)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(changetype), bstrsubscriptionid.into_param().abi()).ok()
    }
    pub unsafe fn ChangedEventClass<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, changetype: EOC_ChangeType, bstreventclassid: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ChangedEventClass)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(changetype), bstreventclassid.into_param().abi()).ok()
    }
    pub unsafe fn ChangedPublisher<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, changetype: EOC_ChangeType, bstrpublisherid: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ChangedPublisher)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(changetype), bstrpublisherid.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IEventObjectChange> for ::windows_core::IUnknown {
    fn from(value: IEventObjectChange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventObjectChange> for ::windows_core::IUnknown {
    fn from(value: &IEventObjectChange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEventObjectChange {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEventObjectChange {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEventObjectChange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEventObjectChange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventObjectChange {}
impl ::core::fmt::Debug for IEventObjectChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventObjectChange").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEventObjectChange {
    type Vtable = IEventObjectChange_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf4a07d70_2e25_11d1_9964_00c04fbbb345);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventObjectChange_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub ChangedSubscription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changetype: EOC_ChangeType, bstrsubscriptionid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub ChangedEventClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changetype: EOC_ChangeType, bstreventclassid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub ChangedPublisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changetype: EOC_ChangeType, bstrpublisherid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEventObjectChange2(::windows_core::IUnknown);
impl IEventObjectChange2 {
    pub unsafe fn ChangedSubscription(&self, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ChangedSubscription)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo)).ok()
    }
    pub unsafe fn ChangedEventClass(&self, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ChangedEventClass)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo)).ok()
    }
}
impl ::core::convert::From<IEventObjectChange2> for ::windows_core::IUnknown {
    fn from(value: IEventObjectChange2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventObjectChange2> for ::windows_core::IUnknown {
    fn from(value: &IEventObjectChange2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEventObjectChange2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEventObjectChange2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEventObjectChange2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEventObjectChange2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventObjectChange2 {}
impl ::core::fmt::Debug for IEventObjectChange2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventObjectChange2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEventObjectChange2 {
    type Vtable = IEventObjectChange2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7701a9c3_bd68_438f_83e0_67bf4f53a422);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventObjectChange2_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub ChangedSubscription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows_core::HRESULT,
    pub ChangedEventClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEventObjectCollection(::windows_core::IUnknown);
impl IEventObjectCollection {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn get_Item<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, objectid: Param0) -> ::windows_core::Result<super::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), objectid.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::VARIANT>(result__)
    }
    pub unsafe fn NewEnum(&self) -> ::windows_core::Result<IEnumEventObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumEventObject>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn Add<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, item: *const super::VARIANT, objectid: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(item), objectid.into_param().abi()).ok()
    }
    pub unsafe fn Remove<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, objectid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), objectid.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IEventObjectCollection> for ::windows_core::IUnknown {
    fn from(value: IEventObjectCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventObjectCollection> for ::windows_core::IUnknown {
    fn from(value: &IEventObjectCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEventObjectCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEventObjectCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IEventObjectCollection> for super::IDispatch {
    fn from(value: IEventObjectCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventObjectCollection> for super::IDispatch {
    fn from(value: &IEventObjectCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IDispatch> for IEventObjectCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IDispatch> for &'a IEventObjectCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEventObjectCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEventObjectCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventObjectCollection {}
impl ::core::fmt::Debug for IEventObjectCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventObjectCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEventObjectCollection {
    type Vtable = IEventObjectCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf89ac270_d4eb_11d1_b682_00805fc79216);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventObjectCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunkenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Ole")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objectid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pitem: *mut super::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    get_Item: usize,
    pub NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Ole")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *const super::VARIANT, objectid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objectid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEventProperty(::windows_core::IUnknown);
impl IEventProperty {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, propertyname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), propertyname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn Value(&self) -> ::windows_core::Result<super::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Value)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn SetValue(&self, propertyvalue: *const super::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(propertyvalue)).ok()
    }
}
impl ::core::convert::From<IEventProperty> for ::windows_core::IUnknown {
    fn from(value: IEventProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventProperty> for ::windows_core::IUnknown {
    fn from(value: &IEventProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEventProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEventProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IEventProperty> for super::IDispatch {
    fn from(value: IEventProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventProperty> for super::IDispatch {
    fn from(value: &IEventProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IDispatch> for IEventProperty {
    fn into_param(self) -> ::windows_core::Param<'a, super::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IDispatch> for &'a IEventProperty {
    fn into_param(self) -> ::windows_core::Param<'a, super::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEventProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEventProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventProperty {}
impl ::core::fmt::Debug for IEventProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventProperty").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEventProperty {
    type Vtable = IEventProperty_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xda538ee2_f4de_11d1_b6bb_00805fc79216);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventProperty_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Ole")]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyvalue: *mut super::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    Value: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyvalue: *const super::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    SetValue: usize,
}
#[repr(transparent)]
pub struct IEventPublisher(::windows_core::IUnknown);
impl IEventPublisher {
    pub unsafe fn PublisherID(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).PublisherID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetPublisherID<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpublisherid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPublisherID)(::windows_core::Interface::as_raw(self), bstrpublisherid.into_param().abi()).ok()
    }
    pub unsafe fn PublisherName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).PublisherName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetPublisherName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpublishername: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPublisherName)(::windows_core::Interface::as_raw(self), bstrpublishername.into_param().abi()).ok()
    }
    pub unsafe fn PublisherType(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).PublisherType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetPublisherType<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpublishertype: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPublisherType)(::windows_core::Interface::as_raw(self), bstrpublishertype.into_param().abi()).ok()
    }
    pub unsafe fn OwnerSID(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).OwnerSID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetOwnerSID<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrownersid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOwnerSID)(::windows_core::Interface::as_raw(self), bstrownersid.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdescription: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetDefaultProperty<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpropertyname: Param0) -> ::windows_core::Result<super::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetDefaultProperty)(::windows_core::Interface::as_raw(self), bstrpropertyname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn PutDefaultProperty<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpropertyname: Param0, propertyvalue: *const super::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PutDefaultProperty)(::windows_core::Interface::as_raw(self), bstrpropertyname.into_param().abi(), ::core::mem::transmute(propertyvalue)).ok()
    }
    pub unsafe fn RemoveDefaultProperty<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpropertyname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveDefaultProperty)(::windows_core::Interface::as_raw(self), bstrpropertyname.into_param().abi()).ok()
    }
    pub unsafe fn GetDefaultPropertyCollection(&self) -> ::windows_core::Result<IEventObjectCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetDefaultPropertyCollection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEventObjectCollection>(result__)
    }
}
impl ::core::convert::From<IEventPublisher> for ::windows_core::IUnknown {
    fn from(value: IEventPublisher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventPublisher> for ::windows_core::IUnknown {
    fn from(value: &IEventPublisher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEventPublisher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEventPublisher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IEventPublisher> for super::IDispatch {
    fn from(value: IEventPublisher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventPublisher> for super::IDispatch {
    fn from(value: &IEventPublisher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IDispatch> for IEventPublisher {
    fn into_param(self) -> ::windows_core::Param<'a, super::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IDispatch> for &'a IEventPublisher {
    fn into_param(self) -> ::windows_core::Param<'a, super::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEventPublisher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEventPublisher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventPublisher {}
impl ::core::fmt::Debug for IEventPublisher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventPublisher").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEventPublisher {
    type Vtable = IEventPublisher_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe341516b_2e32_11d1_9964_00c04fbbb345);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventPublisher_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub PublisherID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpublisherid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetPublisherID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpublisherid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub PublisherName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpublishername: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetPublisherName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpublishername: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub PublisherType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpublishertype: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetPublisherType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpublishertype: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub OwnerSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrownersid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetOwnerSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrownersid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetDefaultProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, propertyvalue: *mut super::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetDefaultProperty: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub PutDefaultProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, propertyvalue: *const super::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    PutDefaultProperty: usize,
    pub RemoveDefaultProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub GetDefaultPropertyCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEventSubscription(::windows_core::IUnknown);
impl IEventSubscription {
    pub unsafe fn SubscriptionID(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).SubscriptionID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetSubscriptionID<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrsubscriptionid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSubscriptionID)(::windows_core::Interface::as_raw(self), bstrsubscriptionid.into_param().abi()).ok()
    }
    pub unsafe fn SubscriptionName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).SubscriptionName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetSubscriptionName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrsubscriptionname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSubscriptionName)(::windows_core::Interface::as_raw(self), bstrsubscriptionname.into_param().abi()).ok()
    }
    pub unsafe fn PublisherID(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).PublisherID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetPublisherID<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpublisherid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPublisherID)(::windows_core::Interface::as_raw(self), bstrpublisherid.into_param().abi()).ok()
    }
    pub unsafe fn EventClassID(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).EventClassID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetEventClassID<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstreventclassid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEventClassID)(::windows_core::Interface::as_raw(self), bstreventclassid.into_param().abi()).ok()
    }
    pub unsafe fn MethodName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).MethodName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetMethodName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrmethodname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMethodName)(::windows_core::Interface::as_raw(self), bstrmethodname.into_param().abi()).ok()
    }
    pub unsafe fn SubscriberCLSID(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).SubscriberCLSID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetSubscriberCLSID<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrsubscriberclsid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSubscriberCLSID)(::windows_core::Interface::as_raw(self), bstrsubscriberclsid.into_param().abi()).ok()
    }
    pub unsafe fn SubscriberInterface(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).SubscriberInterface)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn SetSubscriberInterface<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, psubscriberinterface: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSubscriberInterface)(::windows_core::Interface::as_raw(self), psubscriberinterface.into_param().abi()).ok()
    }
    pub unsafe fn PerUser(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).PerUser)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetPerUser<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fperuser: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPerUser)(::windows_core::Interface::as_raw(self), fperuser.into_param().abi()).ok()
    }
    pub unsafe fn OwnerSID(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).OwnerSID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetOwnerSID<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrownersid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOwnerSID)(::windows_core::Interface::as_raw(self), bstrownersid.into_param().abi()).ok()
    }
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).Enabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetEnabled<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fenabled: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEnabled)(::windows_core::Interface::as_raw(self), fenabled.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdescription: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn MachineName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).MachineName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetMachineName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrmachinename: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMachineName)(::windows_core::Interface::as_raw(self), bstrmachinename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetPublisherProperty<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpropertyname: Param0) -> ::windows_core::Result<super::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetPublisherProperty)(::windows_core::Interface::as_raw(self), bstrpropertyname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn PutPublisherProperty<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpropertyname: Param0, propertyvalue: *const super::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PutPublisherProperty)(::windows_core::Interface::as_raw(self), bstrpropertyname.into_param().abi(), ::core::mem::transmute(propertyvalue)).ok()
    }
    pub unsafe fn RemovePublisherProperty<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpropertyname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemovePublisherProperty)(::windows_core::Interface::as_raw(self), bstrpropertyname.into_param().abi()).ok()
    }
    pub unsafe fn GetPublisherPropertyCollection(&self) -> ::windows_core::Result<IEventObjectCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetPublisherPropertyCollection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEventObjectCollection>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetSubscriberProperty<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpropertyname: Param0) -> ::windows_core::Result<super::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetSubscriberProperty)(::windows_core::Interface::as_raw(self), bstrpropertyname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn PutSubscriberProperty<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpropertyname: Param0, propertyvalue: *const super::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PutSubscriberProperty)(::windows_core::Interface::as_raw(self), bstrpropertyname.into_param().abi(), ::core::mem::transmute(propertyvalue)).ok()
    }
    pub unsafe fn RemoveSubscriberProperty<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpropertyname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveSubscriberProperty)(::windows_core::Interface::as_raw(self), bstrpropertyname.into_param().abi()).ok()
    }
    pub unsafe fn GetSubscriberPropertyCollection(&self) -> ::windows_core::Result<IEventObjectCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSubscriberPropertyCollection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEventObjectCollection>(result__)
    }
    pub unsafe fn InterfaceID(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).InterfaceID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetInterfaceID<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrinterfaceid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInterfaceID)(::windows_core::Interface::as_raw(self), bstrinterfaceid.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IEventSubscription> for ::windows_core::IUnknown {
    fn from(value: IEventSubscription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventSubscription> for ::windows_core::IUnknown {
    fn from(value: &IEventSubscription) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEventSubscription {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEventSubscription {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IEventSubscription> for super::IDispatch {
    fn from(value: IEventSubscription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventSubscription> for super::IDispatch {
    fn from(value: &IEventSubscription) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IDispatch> for IEventSubscription {
    fn into_param(self) -> ::windows_core::Param<'a, super::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IDispatch> for &'a IEventSubscription {
    fn into_param(self) -> ::windows_core::Param<'a, super::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEventSubscription {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEventSubscription {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventSubscription {}
impl ::core::fmt::Debug for IEventSubscription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventSubscription").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEventSubscription {
    type Vtable = IEventSubscription_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4a6b0e15_2e38_11d1_9965_00c04fbbb345);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventSubscription_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub SubscriptionID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsubscriptionid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetSubscriptionID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsubscriptionid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub SubscriptionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsubscriptionname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetSubscriptionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsubscriptionname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub PublisherID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpublisherid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetPublisherID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpublisherid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub EventClassID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstreventclassid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetEventClassID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstreventclassid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub MethodName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrmethodname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetMethodName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmethodname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub SubscriberCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsubscriberclsid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetSubscriberCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsubscriberclsid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub SubscriberInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsubscriberinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetSubscriberInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psubscriberinterface: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PerUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfperuser: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetPerUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fperuser: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub OwnerSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrownersid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetOwnerSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrownersid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenabled: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenabled: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub MachineName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrmachinename: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetMachineName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmachinename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetPublisherProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, propertyvalue: *mut super::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetPublisherProperty: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub PutPublisherProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, propertyvalue: *const super::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    PutPublisherProperty: usize,
    pub RemovePublisherProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub GetPublisherPropertyCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetSubscriberProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, propertyvalue: *mut super::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetSubscriberProperty: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub PutSubscriberProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, propertyvalue: *const super::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    PutSubscriberProperty: usize,
    pub RemoveSubscriberProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub GetSubscriberPropertyCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub InterfaceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrinterfaceid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetInterfaceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrinterfaceid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEventSystem(::windows_core::IUnknown);
impl IEventSystem {
    pub unsafe fn Query<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, progid: Param0, querycriteria: Param1, errorindex: *mut i32, ppinterface: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Query)(::windows_core::Interface::as_raw(self), progid.into_param().abi(), querycriteria.into_param().abi(), ::core::mem::transmute(errorindex), ::core::mem::transmute(ppinterface)).ok()
    }
    pub unsafe fn Store<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, progid: Param0, pinterface: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Store)(::windows_core::Interface::as_raw(self), progid.into_param().abi(), pinterface.into_param().abi()).ok()
    }
    pub unsafe fn Remove<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, progid: Param0, querycriteria: Param1) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), progid.into_param().abi(), querycriteria.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn EventObjectChangeEventClassID(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).EventObjectChangeEventClassID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn QueryS<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, progid: Param0, querycriteria: Param1) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).QueryS)(::windows_core::Interface::as_raw(self), progid.into_param().abi(), querycriteria.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn RemoveS<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, progid: Param0, querycriteria: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveS)(::windows_core::Interface::as_raw(self), progid.into_param().abi(), querycriteria.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IEventSystem> for ::windows_core::IUnknown {
    fn from(value: IEventSystem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventSystem> for ::windows_core::IUnknown {
    fn from(value: &IEventSystem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEventSystem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEventSystem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IEventSystem> for super::IDispatch {
    fn from(value: IEventSystem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventSystem> for super::IDispatch {
    fn from(value: &IEventSystem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IDispatch> for IEventSystem {
    fn into_param(self) -> ::windows_core::Param<'a, super::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IDispatch> for &'a IEventSystem {
    fn into_param(self) -> ::windows_core::Param<'a, super::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEventSystem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEventSystem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventSystem {}
impl ::core::fmt::Debug for IEventSystem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventSystem").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEventSystem {
    type Vtable = IEventSystem_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4e14fb9f_2e22_11d1_9964_00c04fbbb345);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventSystem_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Query: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, progid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, querycriteria: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, errorindex: *mut i32, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Store: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, progid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pinterface: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, progid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, querycriteria: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, errorindex: *mut i32) -> ::windows_core::HRESULT,
    pub EventObjectChangeEventClassID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstreventclassid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub QueryS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, progid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, querycriteria: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, progid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, querycriteria: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IFiringControl(::windows_core::IUnknown);
impl IFiringControl {
    pub unsafe fn FireSubscription<'a, Param0: ::windows_core::IntoParam<'a, IEventSubscription>>(&self, subscription: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FireSubscription)(::windows_core::Interface::as_raw(self), subscription.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IFiringControl> for ::windows_core::IUnknown {
    fn from(value: IFiringControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFiringControl> for ::windows_core::IUnknown {
    fn from(value: &IFiringControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFiringControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFiringControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFiringControl> for super::IDispatch {
    fn from(value: IFiringControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFiringControl> for super::IDispatch {
    fn from(value: &IFiringControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IDispatch> for IFiringControl {
    fn into_param(self) -> ::windows_core::Param<'a, super::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IDispatch> for &'a IFiringControl {
    fn into_param(self) -> ::windows_core::Param<'a, super::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFiringControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFiringControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFiringControl {}
impl ::core::fmt::Debug for IFiringControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFiringControl").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IFiringControl {
    type Vtable = IFiringControl_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe0498c93_4efe_11d1_9971_00c04fbbb345);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFiringControl_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub FireSubscription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subscription: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IMultiInterfaceEventControl(::windows_core::IUnknown);
impl IMultiInterfaceEventControl {
    pub unsafe fn SetMultiInterfacePublisherFilter<'a, Param0: ::windows_core::IntoParam<'a, IMultiInterfacePublisherFilter>>(&self, classfilter: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMultiInterfacePublisherFilter)(::windows_core::Interface::as_raw(self), classfilter.into_param().abi()).ok()
    }
    pub unsafe fn GetSubscriptions<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, eventiid: *const ::windows_core::GUID, bstrmethodname: Param1, optionalcriteria: Param2, optionalerrorindex: *const i32) -> ::windows_core::Result<IEventObjectCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSubscriptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(eventiid), bstrmethodname.into_param().abi(), optionalcriteria.into_param().abi(), ::core::mem::transmute(optionalerrorindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEventObjectCollection>(result__)
    }
    pub unsafe fn SetDefaultQuery<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, eventiid: *const ::windows_core::GUID, bstrmethodname: Param1, bstrcriteria: Param2) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).SetDefaultQuery)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(eventiid), bstrmethodname.into_param().abi(), bstrcriteria.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn AllowInprocActivation(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).AllowInprocActivation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetAllowInprocActivation<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fallowinprocactivation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAllowInprocActivation)(::windows_core::Interface::as_raw(self), fallowinprocactivation.into_param().abi()).ok()
    }
    pub unsafe fn FireInParallel(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).FireInParallel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetFireInParallel<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, ffireinparallel: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFireInParallel)(::windows_core::Interface::as_raw(self), ffireinparallel.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IMultiInterfaceEventControl> for ::windows_core::IUnknown {
    fn from(value: IMultiInterfaceEventControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMultiInterfaceEventControl> for ::windows_core::IUnknown {
    fn from(value: &IMultiInterfaceEventControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMultiInterfaceEventControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMultiInterfaceEventControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMultiInterfaceEventControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMultiInterfaceEventControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMultiInterfaceEventControl {}
impl ::core::fmt::Debug for IMultiInterfaceEventControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMultiInterfaceEventControl").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IMultiInterfaceEventControl {
    type Vtable = IMultiInterfaceEventControl_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0343e2f5_86f6_11d1_b760_00c04fb926af);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultiInterfaceEventControl_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetMultiInterfacePublisherFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, classfilter: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetSubscriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventiid: *const ::windows_core::GUID, bstrmethodname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, optionalcriteria: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, optionalerrorindex: *const i32, ppcollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetDefaultQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventiid: *const ::windows_core::GUID, bstrmethodname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrcriteria: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, errorindex: *mut i32) -> ::windows_core::HRESULT,
    pub AllowInprocActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfallowinprocactivation: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetAllowInprocActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fallowinprocactivation: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub FireInParallel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pffireinparallel: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetFireInParallel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ffireinparallel: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IMultiInterfacePublisherFilter(::windows_core::IUnknown);
impl IMultiInterfacePublisherFilter {
    pub unsafe fn Initialize<'a, Param0: ::windows_core::IntoParam<'a, IMultiInterfaceEventControl>>(&self, peic: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), peic.into_param().abi()).ok()
    }
    pub unsafe fn PrepareToFire<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, IFiringControl>>(&self, iid: *const ::windows_core::GUID, methodname: Param1, firingcontrol: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PrepareToFire)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iid), methodname.into_param().abi(), firingcontrol.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IMultiInterfacePublisherFilter> for ::windows_core::IUnknown {
    fn from(value: IMultiInterfacePublisherFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMultiInterfacePublisherFilter> for ::windows_core::IUnknown {
    fn from(value: &IMultiInterfacePublisherFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMultiInterfacePublisherFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMultiInterfacePublisherFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMultiInterfacePublisherFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMultiInterfacePublisherFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMultiInterfacePublisherFilter {}
impl ::core::fmt::Debug for IMultiInterfacePublisherFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMultiInterfacePublisherFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IMultiInterfacePublisherFilter {
    type Vtable = IMultiInterfacePublisherFilter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x465e5cc1_7b26_11d1_88fb_0080c7d771bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultiInterfacePublisherFilter_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peic: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PrepareToFire: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: *const ::windows_core::GUID, methodname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, firingcontrol: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPublisherFilter(::windows_core::IUnknown);
impl IPublisherFilter {
    pub unsafe fn Initialize<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, super::IDispatch>>(&self, methodname: Param0, dispuserdefined: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), methodname.into_param().abi(), dispuserdefined.into_param().abi()).ok()
    }
    pub unsafe fn PrepareToFire<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, IFiringControl>>(&self, methodname: Param0, firingcontrol: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PrepareToFire)(::windows_core::Interface::as_raw(self), methodname.into_param().abi(), firingcontrol.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IPublisherFilter> for ::windows_core::IUnknown {
    fn from(value: IPublisherFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPublisherFilter> for ::windows_core::IUnknown {
    fn from(value: &IPublisherFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPublisherFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPublisherFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPublisherFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPublisherFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPublisherFilter {}
impl ::core::fmt::Debug for IPublisherFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPublisherFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPublisherFilter {
    type Vtable = IPublisherFilter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x465e5cc0_7b26_11d1_88fb_0080c7d771bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPublisherFilter_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, methodname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, dispuserdefined: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PrepareToFire: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, methodname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, firingcontrol: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
