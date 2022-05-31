#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ICS_TARGETTYPE(pub i32);
pub const ICSTT_NAME: ICS_TARGETTYPE = ICS_TARGETTYPE(0i32);
pub const ICSTT_IPADDRESS: ICS_TARGETTYPE = ICS_TARGETTYPE(1i32);
impl ::core::marker::Copy for ICS_TARGETTYPE {}
impl ::core::clone::Clone for ICS_TARGETTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ICS_TARGETTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ICS_TARGETTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for ICS_TARGETTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICS_TARGETTYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IDynamicPortMapping(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IDynamicPortMapping {
    pub unsafe fn ExternalIPAddress(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ExternalIPAddress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn RemoteHost(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).RemoteHost)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ExternalPort(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).ExternalPort)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn Protocol(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Protocol)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn InternalPort(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).InternalPort)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn InternalClient(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).InternalClient)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).Enabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn LeaseDuration(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).LeaseDuration)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn RenewLease(&self, lleasedurationdesired: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).RenewLease)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lleasedurationdesired), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn EditInternalClient<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrinternalclient: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EditInternalClient)(::windows_core::Interface::as_raw(self), bstrinternalclient.into_param().abi()).ok()
    }
    pub unsafe fn Enable(&self, vb: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Enable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vb)).ok()
    }
    pub unsafe fn EditDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdescription: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EditDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn EditInternalPort(&self, linternalport: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EditInternalPort)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(linternalport)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IDynamicPortMapping> for ::windows_core::IUnknown {
    fn from(value: IDynamicPortMapping) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IDynamicPortMapping> for ::windows_core::IUnknown {
    fn from(value: &IDynamicPortMapping) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDynamicPortMapping {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDynamicPortMapping {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IDynamicPortMapping> for ::win32_system::Com::IDispatch {
    fn from(value: IDynamicPortMapping) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IDynamicPortMapping> for ::win32_system::Com::IDispatch {
    fn from(value: &IDynamicPortMapping) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IDynamicPortMapping {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IDynamicPortMapping {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IDynamicPortMapping {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IDynamicPortMapping {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IDynamicPortMapping {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IDynamicPortMapping {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDynamicPortMapping").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IDynamicPortMapping {
    type Vtable = IDynamicPortMapping_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4fc80282_23b6_4378_9a27_cd8f17c9400c);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IDynamicPortMapping_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub ExternalIPAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub RemoteHost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub ExternalPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub Protocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub InternalPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub InternalClient: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub LeaseDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub RenewLease: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lleasedurationdesired: i32, pleasedurationreturned: *mut i32) -> ::windows_core::HRESULT,
    pub EditInternalClient: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrinternalclient: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Enable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vb: i16) -> ::windows_core::HRESULT,
    pub EditDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub EditInternalPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linternalport: i32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IDynamicPortMappingCollection(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IDynamicPortMappingCollection {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn get_Item<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrremotehost: Param0, lexternalport: i32, bstrprotocol: Param2) -> ::windows_core::Result<IDynamicPortMapping> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), bstrremotehost.into_param().abi(), ::core::mem::transmute(lexternalport), bstrprotocol.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDynamicPortMapping>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn Remove<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrremotehost: Param0, lexternalport: i32, bstrprotocol: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), bstrremotehost.into_param().abi(), ::core::mem::transmute(lexternalport), bstrprotocol.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Add<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param6: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrremotehost: Param0, lexternalport: i32, bstrprotocol: Param2, linternalport: i32, bstrinternalclient: Param4, benabled: i16, bstrdescription: Param6, lleaseduration: i32) -> ::windows_core::Result<IDynamicPortMapping> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), bstrremotehost.into_param().abi(), ::core::mem::transmute(lexternalport), bstrprotocol.into_param().abi(), ::core::mem::transmute(linternalport), bstrinternalclient.into_param().abi(), ::core::mem::transmute(benabled), bstrdescription.into_param().abi(), ::core::mem::transmute(lleaseduration), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDynamicPortMapping>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IDynamicPortMappingCollection> for ::windows_core::IUnknown {
    fn from(value: IDynamicPortMappingCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IDynamicPortMappingCollection> for ::windows_core::IUnknown {
    fn from(value: &IDynamicPortMappingCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDynamicPortMappingCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDynamicPortMappingCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IDynamicPortMappingCollection> for ::win32_system::Com::IDispatch {
    fn from(value: IDynamicPortMappingCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IDynamicPortMappingCollection> for ::win32_system::Com::IDispatch {
    fn from(value: &IDynamicPortMappingCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IDynamicPortMappingCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IDynamicPortMappingCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IDynamicPortMappingCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IDynamicPortMappingCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IDynamicPortMappingCollection {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IDynamicPortMappingCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDynamicPortMappingCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IDynamicPortMappingCollection {
    type Vtable = IDynamicPortMappingCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb60de00f_156e_4e8d_9ec1_3a2342c10899);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IDynamicPortMappingCollection_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrremotehost: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppdpm: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrremotehost: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrremotehost: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, linternalport: i32, bstrinternalclient: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, benabled: i16, bstrdescription: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, lleaseduration: i32, ppdpm: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Add: usize,
}
#[repr(transparent)]
pub struct IEnumNetConnection(::windows_core::IUnknown);
impl IEnumNetConnection {
    pub unsafe fn Next(&self, rgelt: &mut [::core::option::Option<INetConnection>], pceltfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), rgelt.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(rgelt)), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumNetConnection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumNetConnection>(result__)
    }
}
impl ::core::convert::From<IEnumNetConnection> for ::windows_core::IUnknown {
    fn from(value: IEnumNetConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumNetConnection> for ::windows_core::IUnknown {
    fn from(value: &IEnumNetConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumNetConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumNetConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumNetConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumNetConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumNetConnection {}
impl ::core::fmt::Debug for IEnumNetConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumNetConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumNetConnection {
    type Vtable = IEnumNetConnection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc08956a0_1cd3_11d1_b1c5_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetConnection_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows_core::RawPtr, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEnumNetSharingEveryConnection(::windows_core::IUnknown);
impl IEnumNetSharingEveryConnection {
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Next(&self, rgvar: &mut [::win32_system::Com::VARIANT], pceltfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), rgvar.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(rgvar)), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumNetSharingEveryConnection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumNetSharingEveryConnection>(result__)
    }
}
impl ::core::convert::From<IEnumNetSharingEveryConnection> for ::windows_core::IUnknown {
    fn from(value: IEnumNetSharingEveryConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumNetSharingEveryConnection> for ::windows_core::IUnknown {
    fn from(value: &IEnumNetSharingEveryConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumNetSharingEveryConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumNetSharingEveryConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumNetSharingEveryConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumNetSharingEveryConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumNetSharingEveryConnection {}
impl ::core::fmt::Debug for IEnumNetSharingEveryConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumNetSharingEveryConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumNetSharingEveryConnection {
    type Vtable = IEnumNetSharingEveryConnection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc08956b8_1cd3_11d1_b1c5_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetSharingEveryConnection_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut ::win32_system::Com::VARIANT, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEnumNetSharingPortMapping(::windows_core::IUnknown);
impl IEnumNetSharingPortMapping {
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Next(&self, rgvar: &mut [::win32_system::Com::VARIANT], pceltfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), rgvar.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(rgvar)), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumNetSharingPortMapping> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumNetSharingPortMapping>(result__)
    }
}
impl ::core::convert::From<IEnumNetSharingPortMapping> for ::windows_core::IUnknown {
    fn from(value: IEnumNetSharingPortMapping) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumNetSharingPortMapping> for ::windows_core::IUnknown {
    fn from(value: &IEnumNetSharingPortMapping) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumNetSharingPortMapping {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumNetSharingPortMapping {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumNetSharingPortMapping {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumNetSharingPortMapping {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumNetSharingPortMapping {}
impl ::core::fmt::Debug for IEnumNetSharingPortMapping {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumNetSharingPortMapping").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumNetSharingPortMapping {
    type Vtable = IEnumNetSharingPortMapping_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc08956b0_1cd3_11d1_b1c5_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetSharingPortMapping_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut ::win32_system::Com::VARIANT, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEnumNetSharingPrivateConnection(::windows_core::IUnknown);
impl IEnumNetSharingPrivateConnection {
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Next(&self, rgvar: &mut [::win32_system::Com::VARIANT], pceltfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), rgvar.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(rgvar)), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumNetSharingPrivateConnection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumNetSharingPrivateConnection>(result__)
    }
}
impl ::core::convert::From<IEnumNetSharingPrivateConnection> for ::windows_core::IUnknown {
    fn from(value: IEnumNetSharingPrivateConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumNetSharingPrivateConnection> for ::windows_core::IUnknown {
    fn from(value: &IEnumNetSharingPrivateConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumNetSharingPrivateConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumNetSharingPrivateConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumNetSharingPrivateConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumNetSharingPrivateConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumNetSharingPrivateConnection {}
impl ::core::fmt::Debug for IEnumNetSharingPrivateConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumNetSharingPrivateConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumNetSharingPrivateConnection {
    type Vtable = IEnumNetSharingPrivateConnection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc08956b5_1cd3_11d1_b1c5_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetSharingPrivateConnection_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut ::win32_system::Com::VARIANT, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEnumNetSharingPublicConnection(::windows_core::IUnknown);
impl IEnumNetSharingPublicConnection {
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Next(&self, rgvar: &mut [::win32_system::Com::VARIANT], pceltfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), rgvar.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(rgvar)), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumNetSharingPublicConnection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumNetSharingPublicConnection>(result__)
    }
}
impl ::core::convert::From<IEnumNetSharingPublicConnection> for ::windows_core::IUnknown {
    fn from(value: IEnumNetSharingPublicConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumNetSharingPublicConnection> for ::windows_core::IUnknown {
    fn from(value: &IEnumNetSharingPublicConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumNetSharingPublicConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumNetSharingPublicConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumNetSharingPublicConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumNetSharingPublicConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumNetSharingPublicConnection {}
impl ::core::fmt::Debug for IEnumNetSharingPublicConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumNetSharingPublicConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumNetSharingPublicConnection {
    type Vtable = IEnumNetSharingPublicConnection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc08956b4_1cd3_11d1_b1c5_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetSharingPublicConnection_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut ::win32_system::Com::VARIANT, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct INATEventManager(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl INATEventManager {
    pub unsafe fn SetExternalIPAddressCallback<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, punk: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetExternalIPAddressCallback)(::windows_core::Interface::as_raw(self), punk.into_param().abi()).ok()
    }
    pub unsafe fn SetNumberOfEntriesCallback<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, punk: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetNumberOfEntriesCallback)(::windows_core::Interface::as_raw(self), punk.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INATEventManager> for ::windows_core::IUnknown {
    fn from(value: INATEventManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INATEventManager> for ::windows_core::IUnknown {
    fn from(value: &INATEventManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INATEventManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INATEventManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INATEventManager> for ::win32_system::Com::IDispatch {
    fn from(value: INATEventManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INATEventManager> for ::win32_system::Com::IDispatch {
    fn from(value: &INATEventManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for INATEventManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a INATEventManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for INATEventManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for INATEventManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for INATEventManager {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for INATEventManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INATEventManager").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for INATEventManager {
    type Vtable = INATEventManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x624bd588_9060_4109_b0b0_1adbbcac32df);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct INATEventManager_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub SetExternalIPAddressCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetNumberOfEntriesCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct INATExternalIPAddressCallback(::windows_core::IUnknown);
impl INATExternalIPAddressCallback {
    pub unsafe fn NewExternalIPAddress<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrnewexternalipaddress: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NewExternalIPAddress)(::windows_core::Interface::as_raw(self), bstrnewexternalipaddress.into_param().abi()).ok()
    }
}
impl ::core::convert::From<INATExternalIPAddressCallback> for ::windows_core::IUnknown {
    fn from(value: INATExternalIPAddressCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INATExternalIPAddressCallback> for ::windows_core::IUnknown {
    fn from(value: &INATExternalIPAddressCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INATExternalIPAddressCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INATExternalIPAddressCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INATExternalIPAddressCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INATExternalIPAddressCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INATExternalIPAddressCallback {}
impl ::core::fmt::Debug for INATExternalIPAddressCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INATExternalIPAddressCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for INATExternalIPAddressCallback {
    type Vtable = INATExternalIPAddressCallback_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9c416740_a34e_446f_ba06_abd04c3149ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct INATExternalIPAddressCallback_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub NewExternalIPAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrnewexternalipaddress: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct INATNumberOfEntriesCallback(::windows_core::IUnknown);
impl INATNumberOfEntriesCallback {
    pub unsafe fn NewNumberOfEntries(&self, lnewnumberofentries: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NewNumberOfEntries)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lnewnumberofentries)).ok()
    }
}
impl ::core::convert::From<INATNumberOfEntriesCallback> for ::windows_core::IUnknown {
    fn from(value: INATNumberOfEntriesCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INATNumberOfEntriesCallback> for ::windows_core::IUnknown {
    fn from(value: &INATNumberOfEntriesCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INATNumberOfEntriesCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INATNumberOfEntriesCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INATNumberOfEntriesCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INATNumberOfEntriesCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INATNumberOfEntriesCallback {}
impl ::core::fmt::Debug for INATNumberOfEntriesCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INATNumberOfEntriesCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for INATNumberOfEntriesCallback {
    type Vtable = INATNumberOfEntriesCallback_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc83a0a74_91ee_41b6_b67a_67e0f00bbd78);
}
#[repr(C)]
#[doc(hidden)]
pub struct INATNumberOfEntriesCallback_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub NewNumberOfEntries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnewnumberofentries: i32) -> ::windows_core::HRESULT,
}
#[repr(C)]
pub struct INET_FIREWALL_AC_BINARIES {
    pub count: u32,
    pub binaries: *mut ::windows_core::PWSTR,
}
impl ::core::marker::Copy for INET_FIREWALL_AC_BINARIES {}
impl ::core::clone::Clone for INET_FIREWALL_AC_BINARIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INET_FIREWALL_AC_BINARIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INET_FIREWALL_AC_BINARIES").field("count", &self.count).field("binaries", &self.binaries).finish()
    }
}
unsafe impl ::windows_core::Abi for INET_FIREWALL_AC_BINARIES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for INET_FIREWALL_AC_BINARIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INET_FIREWALL_AC_BINARIES>()) == 0 }
    }
}
impl ::core::cmp::Eq for INET_FIREWALL_AC_BINARIES {}
impl ::core::default::Default for INET_FIREWALL_AC_BINARIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "win32-security")]
pub struct INET_FIREWALL_AC_CAPABILITIES {
    pub count: u32,
    pub capabilities: *mut ::win32_security::SID_AND_ATTRIBUTES,
}
#[cfg(feature = "win32-security")]
impl ::core::marker::Copy for INET_FIREWALL_AC_CAPABILITIES {}
#[cfg(feature = "win32-security")]
impl ::core::clone::Clone for INET_FIREWALL_AC_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "win32-security")]
impl ::core::fmt::Debug for INET_FIREWALL_AC_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INET_FIREWALL_AC_CAPABILITIES").field("count", &self.count).field("capabilities", &self.capabilities).finish()
    }
}
#[cfg(feature = "win32-security")]
unsafe impl ::windows_core::Abi for INET_FIREWALL_AC_CAPABILITIES {
    type Abi = Self;
}
#[cfg(feature = "win32-security")]
impl ::core::cmp::PartialEq for INET_FIREWALL_AC_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INET_FIREWALL_AC_CAPABILITIES>()) == 0 }
    }
}
#[cfg(feature = "win32-security")]
impl ::core::cmp::Eq for INET_FIREWALL_AC_CAPABILITIES {}
#[cfg(feature = "win32-security")]
impl ::core::default::Default for INET_FIREWALL_AC_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "win32-security")]
pub struct INET_FIREWALL_AC_CHANGE {
    pub changeType: INET_FIREWALL_AC_CHANGE_TYPE,
    pub createType: INET_FIREWALL_AC_CREATION_TYPE,
    pub appContainerSid: *mut ::win32_security::SID,
    pub userSid: *mut ::win32_security::SID,
    pub displayName: ::windows_core::PWSTR,
    pub Anonymous: INET_FIREWALL_AC_CHANGE_0,
}
#[cfg(feature = "win32-security")]
impl ::core::marker::Copy for INET_FIREWALL_AC_CHANGE {}
#[cfg(feature = "win32-security")]
impl ::core::clone::Clone for INET_FIREWALL_AC_CHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "win32-security")]
unsafe impl ::windows_core::Abi for INET_FIREWALL_AC_CHANGE {
    type Abi = Self;
}
#[cfg(feature = "win32-security")]
impl ::core::cmp::PartialEq for INET_FIREWALL_AC_CHANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INET_FIREWALL_AC_CHANGE>()) == 0 }
    }
}
#[cfg(feature = "win32-security")]
impl ::core::cmp::Eq for INET_FIREWALL_AC_CHANGE {}
#[cfg(feature = "win32-security")]
impl ::core::default::Default for INET_FIREWALL_AC_CHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "win32-security")]
pub union INET_FIREWALL_AC_CHANGE_0 {
    pub capabilities: INET_FIREWALL_AC_CAPABILITIES,
    pub binaries: INET_FIREWALL_AC_BINARIES,
}
#[cfg(feature = "win32-security")]
impl ::core::marker::Copy for INET_FIREWALL_AC_CHANGE_0 {}
#[cfg(feature = "win32-security")]
impl ::core::clone::Clone for INET_FIREWALL_AC_CHANGE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "win32-security")]
unsafe impl ::windows_core::Abi for INET_FIREWALL_AC_CHANGE_0 {
    type Abi = Self;
}
#[cfg(feature = "win32-security")]
impl ::core::cmp::PartialEq for INET_FIREWALL_AC_CHANGE_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INET_FIREWALL_AC_CHANGE_0>()) == 0 }
    }
}
#[cfg(feature = "win32-security")]
impl ::core::cmp::Eq for INET_FIREWALL_AC_CHANGE_0 {}
#[cfg(feature = "win32-security")]
impl ::core::default::Default for INET_FIREWALL_AC_CHANGE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct INET_FIREWALL_AC_CHANGE_TYPE(pub i32);
pub const INET_FIREWALL_AC_CHANGE_INVALID: INET_FIREWALL_AC_CHANGE_TYPE = INET_FIREWALL_AC_CHANGE_TYPE(0i32);
pub const INET_FIREWALL_AC_CHANGE_CREATE: INET_FIREWALL_AC_CHANGE_TYPE = INET_FIREWALL_AC_CHANGE_TYPE(1i32);
pub const INET_FIREWALL_AC_CHANGE_DELETE: INET_FIREWALL_AC_CHANGE_TYPE = INET_FIREWALL_AC_CHANGE_TYPE(2i32);
pub const INET_FIREWALL_AC_CHANGE_MAX: INET_FIREWALL_AC_CHANGE_TYPE = INET_FIREWALL_AC_CHANGE_TYPE(3i32);
impl ::core::marker::Copy for INET_FIREWALL_AC_CHANGE_TYPE {}
impl ::core::clone::Clone for INET_FIREWALL_AC_CHANGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INET_FIREWALL_AC_CHANGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for INET_FIREWALL_AC_CHANGE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for INET_FIREWALL_AC_CHANGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INET_FIREWALL_AC_CHANGE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct INET_FIREWALL_AC_CREATION_TYPE(pub i32);
pub const INET_FIREWALL_AC_NONE: INET_FIREWALL_AC_CREATION_TYPE = INET_FIREWALL_AC_CREATION_TYPE(0i32);
pub const INET_FIREWALL_AC_PACKAGE_ID_ONLY: INET_FIREWALL_AC_CREATION_TYPE = INET_FIREWALL_AC_CREATION_TYPE(1i32);
pub const INET_FIREWALL_AC_BINARY: INET_FIREWALL_AC_CREATION_TYPE = INET_FIREWALL_AC_CREATION_TYPE(2i32);
pub const INET_FIREWALL_AC_MAX: INET_FIREWALL_AC_CREATION_TYPE = INET_FIREWALL_AC_CREATION_TYPE(4i32);
impl ::core::marker::Copy for INET_FIREWALL_AC_CREATION_TYPE {}
impl ::core::clone::Clone for INET_FIREWALL_AC_CREATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INET_FIREWALL_AC_CREATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for INET_FIREWALL_AC_CREATION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for INET_FIREWALL_AC_CREATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INET_FIREWALL_AC_CREATION_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[cfg(feature = "win32-security")]
pub struct INET_FIREWALL_APP_CONTAINER {
    pub appContainerSid: *mut ::win32_security::SID,
    pub userSid: *mut ::win32_security::SID,
    pub appContainerName: ::windows_core::PWSTR,
    pub displayName: ::windows_core::PWSTR,
    pub description: ::windows_core::PWSTR,
    pub capabilities: INET_FIREWALL_AC_CAPABILITIES,
    pub binaries: INET_FIREWALL_AC_BINARIES,
    pub workingDirectory: ::windows_core::PWSTR,
    pub packageFullName: ::windows_core::PWSTR,
}
#[cfg(feature = "win32-security")]
impl ::core::marker::Copy for INET_FIREWALL_APP_CONTAINER {}
#[cfg(feature = "win32-security")]
impl ::core::clone::Clone for INET_FIREWALL_APP_CONTAINER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "win32-security")]
impl ::core::fmt::Debug for INET_FIREWALL_APP_CONTAINER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INET_FIREWALL_APP_CONTAINER").field("appContainerSid", &self.appContainerSid).field("userSid", &self.userSid).field("appContainerName", &self.appContainerName).field("displayName", &self.displayName).field("description", &self.description).field("capabilities", &self.capabilities).field("binaries", &self.binaries).field("workingDirectory", &self.workingDirectory).field("packageFullName", &self.packageFullName).finish()
    }
}
#[cfg(feature = "win32-security")]
unsafe impl ::windows_core::Abi for INET_FIREWALL_APP_CONTAINER {
    type Abi = Self;
}
#[cfg(feature = "win32-security")]
impl ::core::cmp::PartialEq for INET_FIREWALL_APP_CONTAINER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INET_FIREWALL_APP_CONTAINER>()) == 0 }
    }
}
#[cfg(feature = "win32-security")]
impl ::core::cmp::Eq for INET_FIREWALL_APP_CONTAINER {}
#[cfg(feature = "win32-security")]
impl ::core::default::Default for INET_FIREWALL_APP_CONTAINER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct INetConnection(::windows_core::IUnknown);
impl INetConnection {
    pub unsafe fn Connect(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Connect)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Disconnect(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Disconnect)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Duplicate<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszwduplicatename: Param0) -> ::windows_core::Result<INetConnection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Duplicate)(::windows_core::Interface::as_raw(self), pszwduplicatename.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INetConnection>(result__)
    }
    pub unsafe fn GetProperties(&self) -> ::windows_core::Result<*mut NETCON_PROPERTIES> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut NETCON_PROPERTIES>::zeroed();
        (::windows_core::Interface::vtable(self).GetProperties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut NETCON_PROPERTIES>(result__)
    }
    pub unsafe fn GetUiObjectClassId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).GetUiObjectClassId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn Rename<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszwnewname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Rename)(::windows_core::Interface::as_raw(self), pszwnewname.into_param().abi()).ok()
    }
}
impl ::core::convert::From<INetConnection> for ::windows_core::IUnknown {
    fn from(value: INetConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetConnection> for ::windows_core::IUnknown {
    fn from(value: &INetConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INetConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INetConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetConnection {}
impl ::core::fmt::Debug for INetConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for INetConnection {
    type Vtable = INetConnection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc08956a1_1cd3_11d1_b1c5_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetConnection_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Duplicate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszwduplicatename: ::windows_core::PCWSTR, ppcon: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprops: *mut *mut NETCON_PROPERTIES) -> ::windows_core::HRESULT,
    pub GetUiObjectClassId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Rename: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszwnewname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct INetConnectionConnectUi(::windows_core::IUnknown);
impl INetConnectionConnectUi {
    pub unsafe fn SetConnection<'a, Param0: ::windows_core::IntoParam<'a, INetConnection>>(&self, pcon: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetConnection)(::windows_core::Interface::as_raw(self), pcon.into_param().abi()).ok()
    }
    pub unsafe fn Connect<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(&self, hwndparent: Param0, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Connect)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn Disconnect<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(&self, hwndparent: Param0, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Disconnect)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
}
impl ::core::convert::From<INetConnectionConnectUi> for ::windows_core::IUnknown {
    fn from(value: INetConnectionConnectUi) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetConnectionConnectUi> for ::windows_core::IUnknown {
    fn from(value: &INetConnectionConnectUi) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INetConnectionConnectUi {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INetConnectionConnectUi {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetConnectionConnectUi {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetConnectionConnectUi {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetConnectionConnectUi {}
impl ::core::fmt::Debug for INetConnectionConnectUi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetConnectionConnectUi").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for INetConnectionConnectUi {
    type Vtable = INetConnectionConnectUi_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc08956a3_1cd3_11d1_b1c5_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetConnectionConnectUi_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcon: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: ::win32_foundation::HWND, dwflags: u32) -> ::windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: ::win32_foundation::HWND, dwflags: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct INetConnectionManager(::windows_core::IUnknown);
impl INetConnectionManager {
    pub unsafe fn EnumConnections(&self, flags: NETCONMGR_ENUM_FLAGS) -> ::windows_core::Result<IEnumNetConnection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumConnections)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(flags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumNetConnection>(result__)
    }
}
impl ::core::convert::From<INetConnectionManager> for ::windows_core::IUnknown {
    fn from(value: INetConnectionManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetConnectionManager> for ::windows_core::IUnknown {
    fn from(value: &INetConnectionManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INetConnectionManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INetConnectionManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetConnectionManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetConnectionManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetConnectionManager {}
impl ::core::fmt::Debug for INetConnectionManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetConnectionManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for INetConnectionManager {
    type Vtable = INetConnectionManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc08956a2_1cd3_11d1_b1c5_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetConnectionManager_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub EnumConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: NETCONMGR_ENUM_FLAGS, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct INetConnectionProps(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl INetConnectionProps {
    pub unsafe fn Guid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Guid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn DeviceName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).DeviceName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Status(&self) -> ::windows_core::Result<NETCON_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<NETCON_STATUS>::zeroed();
        (::windows_core::Interface::vtable(self).Status)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<NETCON_STATUS>(result__)
    }
    pub unsafe fn MediaType(&self) -> ::windows_core::Result<NETCON_MEDIATYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<NETCON_MEDIATYPE>::zeroed();
        (::windows_core::Interface::vtable(self).MediaType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<NETCON_MEDIATYPE>(result__)
    }
    pub unsafe fn Characteristics(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).Characteristics)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetConnectionProps> for ::windows_core::IUnknown {
    fn from(value: INetConnectionProps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetConnectionProps> for ::windows_core::IUnknown {
    fn from(value: &INetConnectionProps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INetConnectionProps {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INetConnectionProps {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetConnectionProps> for ::win32_system::Com::IDispatch {
    fn from(value: INetConnectionProps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetConnectionProps> for ::win32_system::Com::IDispatch {
    fn from(value: &INetConnectionProps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for INetConnectionProps {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a INetConnectionProps {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for INetConnectionProps {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for INetConnectionProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for INetConnectionProps {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for INetConnectionProps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetConnectionProps").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for INetConnectionProps {
    type Vtable = INetConnectionProps_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf4277c95_ce5b_463d_8167_5662d9bcaa72);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct INetConnectionProps_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Guid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrguid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub DeviceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut NETCON_STATUS) -> ::windows_core::HRESULT,
    pub MediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmediatype: *mut NETCON_MEDIATYPE) -> ::windows_core::HRESULT,
    pub Characteristics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct INetFwAuthorizedApplication(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl INetFwAuthorizedApplication {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn ProcessImageFileName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ProcessImageFileName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetProcessImageFileName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, imagefilename: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProcessImageFileName)(::windows_core::Interface::as_raw(self), imagefilename.into_param().abi()).ok()
    }
    pub unsafe fn IpVersion(&self) -> ::windows_core::Result<NET_FW_IP_VERSION> {
        let mut result__ = ::core::mem::MaybeUninit::<NET_FW_IP_VERSION>::zeroed();
        (::windows_core::Interface::vtable(self).IpVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<NET_FW_IP_VERSION>(result__)
    }
    pub unsafe fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIpVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ipversion)).ok()
    }
    pub unsafe fn Scope(&self) -> ::windows_core::Result<NET_FW_SCOPE> {
        let mut result__ = ::core::mem::MaybeUninit::<NET_FW_SCOPE>::zeroed();
        (::windows_core::Interface::vtable(self).Scope)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<NET_FW_SCOPE>(result__)
    }
    pub unsafe fn SetScope(&self, scope: NET_FW_SCOPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetScope)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(scope)).ok()
    }
    pub unsafe fn RemoteAddresses(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).RemoteAddresses)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetRemoteAddresses<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, remoteaddrs: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRemoteAddresses)(::windows_core::Interface::as_raw(self), remoteaddrs.into_param().abi()).ok()
    }
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).Enabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enabled)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwAuthorizedApplication> for ::windows_core::IUnknown {
    fn from(value: INetFwAuthorizedApplication) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwAuthorizedApplication> for ::windows_core::IUnknown {
    fn from(value: &INetFwAuthorizedApplication) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INetFwAuthorizedApplication {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INetFwAuthorizedApplication {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwAuthorizedApplication> for ::win32_system::Com::IDispatch {
    fn from(value: INetFwAuthorizedApplication) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwAuthorizedApplication> for ::win32_system::Com::IDispatch {
    fn from(value: &INetFwAuthorizedApplication) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for INetFwAuthorizedApplication {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a INetFwAuthorizedApplication {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for INetFwAuthorizedApplication {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for INetFwAuthorizedApplication {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for INetFwAuthorizedApplication {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for INetFwAuthorizedApplication {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetFwAuthorizedApplication").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for INetFwAuthorizedApplication {
    type Vtable = INetFwAuthorizedApplication_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb5e64ffa_c2c5_444e_a301_fb5e00018050);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwAuthorizedApplication_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub ProcessImageFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagefilename: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetProcessImageFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagefilename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub IpVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> ::windows_core::HRESULT,
    pub SetIpVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> ::windows_core::HRESULT,
    pub Scope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> ::windows_core::HRESULT,
    pub SetScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: NET_FW_SCOPE) -> ::windows_core::HRESULT,
    pub RemoteAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteaddrs: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetRemoteAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteaddrs: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct INetFwAuthorizedApplications(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl INetFwAuthorizedApplications {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Add<'a, Param0: ::windows_core::IntoParam<'a, INetFwAuthorizedApplication>>(&self, app: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), app.into_param().abi()).ok()
    }
    pub unsafe fn Remove<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, imagefilename: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), imagefilename.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Item<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, imagefilename: Param0) -> ::windows_core::Result<INetFwAuthorizedApplication> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), imagefilename.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INetFwAuthorizedApplication>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwAuthorizedApplications> for ::windows_core::IUnknown {
    fn from(value: INetFwAuthorizedApplications) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwAuthorizedApplications> for ::windows_core::IUnknown {
    fn from(value: &INetFwAuthorizedApplications) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INetFwAuthorizedApplications {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INetFwAuthorizedApplications {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwAuthorizedApplications> for ::win32_system::Com::IDispatch {
    fn from(value: INetFwAuthorizedApplications) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwAuthorizedApplications> for ::win32_system::Com::IDispatch {
    fn from(value: &INetFwAuthorizedApplications) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for INetFwAuthorizedApplications {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a INetFwAuthorizedApplications {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for INetFwAuthorizedApplications {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for INetFwAuthorizedApplications {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for INetFwAuthorizedApplications {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for INetFwAuthorizedApplications {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetFwAuthorizedApplications").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for INetFwAuthorizedApplications {
    type Vtable = INetFwAuthorizedApplications_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x644efd52_ccf9_486c_97a2_39f352570b30);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwAuthorizedApplications_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, app: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagefilename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagefilename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, app: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct INetFwIcmpSettings(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl INetFwIcmpSettings {
    pub unsafe fn AllowOutboundDestinationUnreachable(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).AllowOutboundDestinationUnreachable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetAllowOutboundDestinationUnreachable(&self, allow: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAllowOutboundDestinationUnreachable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(allow)).ok()
    }
    pub unsafe fn AllowRedirect(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).AllowRedirect)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetAllowRedirect(&self, allow: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAllowRedirect)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(allow)).ok()
    }
    pub unsafe fn AllowInboundEchoRequest(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).AllowInboundEchoRequest)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetAllowInboundEchoRequest(&self, allow: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAllowInboundEchoRequest)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(allow)).ok()
    }
    pub unsafe fn AllowOutboundTimeExceeded(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).AllowOutboundTimeExceeded)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetAllowOutboundTimeExceeded(&self, allow: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAllowOutboundTimeExceeded)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(allow)).ok()
    }
    pub unsafe fn AllowOutboundParameterProblem(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).AllowOutboundParameterProblem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetAllowOutboundParameterProblem(&self, allow: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAllowOutboundParameterProblem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(allow)).ok()
    }
    pub unsafe fn AllowOutboundSourceQuench(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).AllowOutboundSourceQuench)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetAllowOutboundSourceQuench(&self, allow: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAllowOutboundSourceQuench)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(allow)).ok()
    }
    pub unsafe fn AllowInboundRouterRequest(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).AllowInboundRouterRequest)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetAllowInboundRouterRequest(&self, allow: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAllowInboundRouterRequest)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(allow)).ok()
    }
    pub unsafe fn AllowInboundTimestampRequest(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).AllowInboundTimestampRequest)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetAllowInboundTimestampRequest(&self, allow: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAllowInboundTimestampRequest)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(allow)).ok()
    }
    pub unsafe fn AllowInboundMaskRequest(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).AllowInboundMaskRequest)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetAllowInboundMaskRequest(&self, allow: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAllowInboundMaskRequest)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(allow)).ok()
    }
    pub unsafe fn AllowOutboundPacketTooBig(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).AllowOutboundPacketTooBig)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetAllowOutboundPacketTooBig(&self, allow: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAllowOutboundPacketTooBig)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(allow)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwIcmpSettings> for ::windows_core::IUnknown {
    fn from(value: INetFwIcmpSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwIcmpSettings> for ::windows_core::IUnknown {
    fn from(value: &INetFwIcmpSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INetFwIcmpSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INetFwIcmpSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwIcmpSettings> for ::win32_system::Com::IDispatch {
    fn from(value: INetFwIcmpSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwIcmpSettings> for ::win32_system::Com::IDispatch {
    fn from(value: &INetFwIcmpSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for INetFwIcmpSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a INetFwIcmpSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for INetFwIcmpSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for INetFwIcmpSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for INetFwIcmpSettings {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for INetFwIcmpSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetFwIcmpSettings").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for INetFwIcmpSettings {
    type Vtable = INetFwIcmpSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa6207b2e_7cdd_426a_951e_5e1cbc5afead);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwIcmpSettings_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub AllowOutboundDestinationUnreachable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows_core::HRESULT,
    pub SetAllowOutboundDestinationUnreachable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows_core::HRESULT,
    pub AllowRedirect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows_core::HRESULT,
    pub SetAllowRedirect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows_core::HRESULT,
    pub AllowInboundEchoRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows_core::HRESULT,
    pub SetAllowInboundEchoRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows_core::HRESULT,
    pub AllowOutboundTimeExceeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows_core::HRESULT,
    pub SetAllowOutboundTimeExceeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows_core::HRESULT,
    pub AllowOutboundParameterProblem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows_core::HRESULT,
    pub SetAllowOutboundParameterProblem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows_core::HRESULT,
    pub AllowOutboundSourceQuench: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows_core::HRESULT,
    pub SetAllowOutboundSourceQuench: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows_core::HRESULT,
    pub AllowInboundRouterRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows_core::HRESULT,
    pub SetAllowInboundRouterRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows_core::HRESULT,
    pub AllowInboundTimestampRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows_core::HRESULT,
    pub SetAllowInboundTimestampRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows_core::HRESULT,
    pub AllowInboundMaskRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows_core::HRESULT,
    pub SetAllowInboundMaskRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows_core::HRESULT,
    pub AllowOutboundPacketTooBig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows_core::HRESULT,
    pub SetAllowOutboundPacketTooBig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct INetFwMgr(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl INetFwMgr {
    #[cfg(feature = "win32-system")]
    pub unsafe fn LocalPolicy(&self) -> ::windows_core::Result<INetFwPolicy> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).LocalPolicy)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INetFwPolicy>(result__)
    }
    pub unsafe fn CurrentProfileType(&self) -> ::windows_core::Result<NET_FW_PROFILE_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<NET_FW_PROFILE_TYPE>::zeroed();
        (::windows_core::Interface::vtable(self).CurrentProfileType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<NET_FW_PROFILE_TYPE>(result__)
    }
    pub unsafe fn RestoreDefaults(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RestoreDefaults)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn IsPortAllowed<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, imagefilename: Param0, ipversion: NET_FW_IP_VERSION, portnumber: i32, localaddress: Param3, ipprotocol: NET_FW_IP_PROTOCOL, allowed: *mut ::win32_system::Com::VARIANT, restricted: *mut ::win32_system::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsPortAllowed)(::windows_core::Interface::as_raw(self), imagefilename.into_param().abi(), ::core::mem::transmute(ipversion), ::core::mem::transmute(portnumber), localaddress.into_param().abi(), ::core::mem::transmute(ipprotocol), ::core::mem::transmute(allowed), ::core::mem::transmute(restricted)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn IsIcmpTypeAllowed<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, ipversion: NET_FW_IP_VERSION, localaddress: Param1, r#type: u8, allowed: *mut ::win32_system::Com::VARIANT, restricted: *mut ::win32_system::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsIcmpTypeAllowed)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ipversion), localaddress.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(allowed), ::core::mem::transmute(restricted)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwMgr> for ::windows_core::IUnknown {
    fn from(value: INetFwMgr) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwMgr> for ::windows_core::IUnknown {
    fn from(value: &INetFwMgr) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INetFwMgr {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INetFwMgr {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwMgr> for ::win32_system::Com::IDispatch {
    fn from(value: INetFwMgr) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwMgr> for ::win32_system::Com::IDispatch {
    fn from(value: &INetFwMgr) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for INetFwMgr {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a INetFwMgr {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for INetFwMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for INetFwMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for INetFwMgr {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for INetFwMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetFwMgr").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for INetFwMgr {
    type Vtable = INetFwMgr_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf7898af5_cac4_4632_a2ec_da06e5111af2);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwMgr_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub LocalPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localpolicy: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    LocalPolicy: usize,
    pub CurrentProfileType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: *mut NET_FW_PROFILE_TYPE) -> ::windows_core::HRESULT,
    pub RestoreDefaults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub IsPortAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagefilename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ipversion: NET_FW_IP_VERSION, portnumber: i32, localaddress: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ipprotocol: NET_FW_IP_PROTOCOL, allowed: *mut ::win32_system::Com::VARIANT, restricted: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    IsPortAllowed: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub IsIcmpTypeAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION, localaddress: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, r#type: u8, allowed: *mut ::win32_system::Com::VARIANT, restricted: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    IsIcmpTypeAllowed: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct INetFwOpenPort(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl INetFwOpenPort {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn IpVersion(&self) -> ::windows_core::Result<NET_FW_IP_VERSION> {
        let mut result__ = ::core::mem::MaybeUninit::<NET_FW_IP_VERSION>::zeroed();
        (::windows_core::Interface::vtable(self).IpVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<NET_FW_IP_VERSION>(result__)
    }
    pub unsafe fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIpVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ipversion)).ok()
    }
    pub unsafe fn Protocol(&self) -> ::windows_core::Result<NET_FW_IP_PROTOCOL> {
        let mut result__ = ::core::mem::MaybeUninit::<NET_FW_IP_PROTOCOL>::zeroed();
        (::windows_core::Interface::vtable(self).Protocol)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<NET_FW_IP_PROTOCOL>(result__)
    }
    pub unsafe fn SetProtocol(&self, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProtocol)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ipprotocol)).ok()
    }
    pub unsafe fn Port(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Port)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPort(&self, portnumber: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPort)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(portnumber)).ok()
    }
    pub unsafe fn Scope(&self) -> ::windows_core::Result<NET_FW_SCOPE> {
        let mut result__ = ::core::mem::MaybeUninit::<NET_FW_SCOPE>::zeroed();
        (::windows_core::Interface::vtable(self).Scope)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<NET_FW_SCOPE>(result__)
    }
    pub unsafe fn SetScope(&self, scope: NET_FW_SCOPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetScope)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(scope)).ok()
    }
    pub unsafe fn RemoteAddresses(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).RemoteAddresses)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetRemoteAddresses<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, remoteaddrs: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRemoteAddresses)(::windows_core::Interface::as_raw(self), remoteaddrs.into_param().abi()).ok()
    }
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).Enabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enabled)).ok()
    }
    pub unsafe fn BuiltIn(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).BuiltIn)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwOpenPort> for ::windows_core::IUnknown {
    fn from(value: INetFwOpenPort) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwOpenPort> for ::windows_core::IUnknown {
    fn from(value: &INetFwOpenPort) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INetFwOpenPort {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INetFwOpenPort {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwOpenPort> for ::win32_system::Com::IDispatch {
    fn from(value: INetFwOpenPort) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwOpenPort> for ::win32_system::Com::IDispatch {
    fn from(value: &INetFwOpenPort) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for INetFwOpenPort {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a INetFwOpenPort {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for INetFwOpenPort {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for INetFwOpenPort {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for INetFwOpenPort {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for INetFwOpenPort {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetFwOpenPort").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for INetFwOpenPort {
    type Vtable = INetFwOpenPort_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe0483ba0_47ff_4d9c_a6d6_7741d0b195f7);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwOpenPort_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub IpVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> ::windows_core::HRESULT,
    pub SetIpVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> ::windows_core::HRESULT,
    pub Protocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ipprotocol: *mut NET_FW_IP_PROTOCOL) -> ::windows_core::HRESULT,
    pub SetProtocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows_core::HRESULT,
    pub Port: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, portnumber: *mut i32) -> ::windows_core::HRESULT,
    pub SetPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, portnumber: i32) -> ::windows_core::HRESULT,
    pub Scope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> ::windows_core::HRESULT,
    pub SetScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: NET_FW_SCOPE) -> ::windows_core::HRESULT,
    pub RemoteAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteaddrs: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetRemoteAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteaddrs: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows_core::HRESULT,
    pub BuiltIn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, builtin: *mut i16) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct INetFwOpenPorts(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl INetFwOpenPorts {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Add<'a, Param0: ::windows_core::IntoParam<'a, INetFwOpenPort>>(&self, port: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), port.into_param().abi()).ok()
    }
    pub unsafe fn Remove(&self, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(portnumber), ::core::mem::transmute(ipprotocol)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Item(&self, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows_core::Result<INetFwOpenPort> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(portnumber), ::core::mem::transmute(ipprotocol), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INetFwOpenPort>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwOpenPorts> for ::windows_core::IUnknown {
    fn from(value: INetFwOpenPorts) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwOpenPorts> for ::windows_core::IUnknown {
    fn from(value: &INetFwOpenPorts) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INetFwOpenPorts {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INetFwOpenPorts {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwOpenPorts> for ::win32_system::Com::IDispatch {
    fn from(value: INetFwOpenPorts) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwOpenPorts> for ::win32_system::Com::IDispatch {
    fn from(value: &INetFwOpenPorts) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for INetFwOpenPorts {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a INetFwOpenPorts {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for INetFwOpenPorts {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for INetFwOpenPorts {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for INetFwOpenPorts {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for INetFwOpenPorts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetFwOpenPorts").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for INetFwOpenPorts {
    type Vtable = INetFwOpenPorts_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc0e9d7fa_e07e_430a_b19a_090ce82d92e2);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwOpenPorts_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, port: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL, openport: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct INetFwPolicy(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl INetFwPolicy {
    #[cfg(feature = "win32-system")]
    pub unsafe fn CurrentProfile(&self) -> ::windows_core::Result<INetFwProfile> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CurrentProfile)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INetFwProfile>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetProfileByType(&self, profiletype: NET_FW_PROFILE_TYPE) -> ::windows_core::Result<INetFwProfile> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetProfileByType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(profiletype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INetFwProfile>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwPolicy> for ::windows_core::IUnknown {
    fn from(value: INetFwPolicy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwPolicy> for ::windows_core::IUnknown {
    fn from(value: &INetFwPolicy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INetFwPolicy {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INetFwPolicy {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwPolicy> for ::win32_system::Com::IDispatch {
    fn from(value: INetFwPolicy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwPolicy> for ::win32_system::Com::IDispatch {
    fn from(value: &INetFwPolicy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for INetFwPolicy {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a INetFwPolicy {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for INetFwPolicy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for INetFwPolicy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for INetFwPolicy {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for INetFwPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetFwPolicy").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for INetFwPolicy {
    type Vtable = INetFwPolicy_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd46d2478_9ac9_4008_9dc7_5563ce5536cc);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwPolicy_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub CurrentProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profile: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CurrentProfile: usize,
    #[cfg(feature = "win32-system")]
    pub GetProfileByType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE, profile: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetProfileByType: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct INetFwPolicy2(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl INetFwPolicy2 {
    pub unsafe fn CurrentProfileTypes(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).CurrentProfileTypes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn get_FirewallEnabled(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).get_FirewallEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(profiletype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn put_FirewallEnabled(&self, profiletype: NET_FW_PROFILE_TYPE2, enabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).put_FirewallEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(profiletype), ::core::mem::transmute(enabled)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn get_ExcludedInterfaces(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).get_ExcludedInterfaces)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(profiletype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn put_ExcludedInterfaces<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, profiletype: NET_FW_PROFILE_TYPE2, interfaces: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).put_ExcludedInterfaces)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(profiletype), interfaces.into_param().abi()).ok()
    }
    pub unsafe fn get_BlockAllInboundTraffic(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).get_BlockAllInboundTraffic)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(profiletype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn put_BlockAllInboundTraffic(&self, profiletype: NET_FW_PROFILE_TYPE2, block: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).put_BlockAllInboundTraffic)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(profiletype), ::core::mem::transmute(block)).ok()
    }
    pub unsafe fn get_NotificationsDisabled(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).get_NotificationsDisabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(profiletype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn put_NotificationsDisabled(&self, profiletype: NET_FW_PROFILE_TYPE2, disabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).put_NotificationsDisabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(profiletype), ::core::mem::transmute(disabled)).ok()
    }
    pub unsafe fn get_UnicastResponsesToMulticastBroadcastDisabled(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).get_UnicastResponsesToMulticastBroadcastDisabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(profiletype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn put_UnicastResponsesToMulticastBroadcastDisabled(&self, profiletype: NET_FW_PROFILE_TYPE2, disabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).put_UnicastResponsesToMulticastBroadcastDisabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(profiletype), ::core::mem::transmute(disabled)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Rules(&self) -> ::windows_core::Result<INetFwRules> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Rules)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INetFwRules>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ServiceRestriction(&self) -> ::windows_core::Result<INetFwServiceRestriction> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ServiceRestriction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INetFwServiceRestriction>(result__)
    }
    pub unsafe fn EnableRuleGroup<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, profiletypesbitmask: i32, group: Param1, enable: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnableRuleGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(profiletypesbitmask), group.into_param().abi(), ::core::mem::transmute(enable)).ok()
    }
    pub unsafe fn IsRuleGroupEnabled<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, profiletypesbitmask: i32, group: Param1) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsRuleGroupEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(profiletypesbitmask), group.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn RestoreLocalFirewallDefaults(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RestoreLocalFirewallDefaults)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn get_DefaultInboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows_core::Result<NET_FW_ACTION> {
        let mut result__ = ::core::mem::MaybeUninit::<NET_FW_ACTION>::zeroed();
        (::windows_core::Interface::vtable(self).get_DefaultInboundAction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(profiletype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<NET_FW_ACTION>(result__)
    }
    pub unsafe fn put_DefaultInboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).put_DefaultInboundAction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(profiletype), ::core::mem::transmute(action)).ok()
    }
    pub unsafe fn get_DefaultOutboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows_core::Result<NET_FW_ACTION> {
        let mut result__ = ::core::mem::MaybeUninit::<NET_FW_ACTION>::zeroed();
        (::windows_core::Interface::vtable(self).get_DefaultOutboundAction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(profiletype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<NET_FW_ACTION>(result__)
    }
    pub unsafe fn put_DefaultOutboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).put_DefaultOutboundAction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(profiletype), ::core::mem::transmute(action)).ok()
    }
    pub unsafe fn get_IsRuleGroupCurrentlyEnabled<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, group: Param0) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).get_IsRuleGroupCurrentlyEnabled)(::windows_core::Interface::as_raw(self), group.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn LocalPolicyModifyState(&self) -> ::windows_core::Result<NET_FW_MODIFY_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::<NET_FW_MODIFY_STATE>::zeroed();
        (::windows_core::Interface::vtable(self).LocalPolicyModifyState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<NET_FW_MODIFY_STATE>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwPolicy2> for ::windows_core::IUnknown {
    fn from(value: INetFwPolicy2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwPolicy2> for ::windows_core::IUnknown {
    fn from(value: &INetFwPolicy2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INetFwPolicy2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INetFwPolicy2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwPolicy2> for ::win32_system::Com::IDispatch {
    fn from(value: INetFwPolicy2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwPolicy2> for ::win32_system::Com::IDispatch {
    fn from(value: &INetFwPolicy2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for INetFwPolicy2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a INetFwPolicy2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for INetFwPolicy2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for INetFwPolicy2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for INetFwPolicy2 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for INetFwPolicy2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetFwPolicy2").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for INetFwPolicy2 {
    type Vtable = INetFwPolicy2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x98325047_c671_4174_8d81_defcd3f03186);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwPolicy2_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub CurrentProfileTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletypesbitmask: *mut i32) -> ::windows_core::HRESULT,
    pub get_FirewallEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, enabled: *mut i16) -> ::windows_core::HRESULT,
    pub put_FirewallEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, enabled: i16) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub get_ExcludedInterfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, interfaces: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    get_ExcludedInterfaces: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub put_ExcludedInterfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, interfaces: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    put_ExcludedInterfaces: usize,
    pub get_BlockAllInboundTraffic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, block: *mut i16) -> ::windows_core::HRESULT,
    pub put_BlockAllInboundTraffic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, block: i16) -> ::windows_core::HRESULT,
    pub get_NotificationsDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: *mut i16) -> ::windows_core::HRESULT,
    pub put_NotificationsDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: i16) -> ::windows_core::HRESULT,
    pub get_UnicastResponsesToMulticastBroadcastDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: *mut i16) -> ::windows_core::HRESULT,
    pub put_UnicastResponsesToMulticastBroadcastDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Rules: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rules: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Rules: usize,
    #[cfg(feature = "win32-system")]
    pub ServiceRestriction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servicerestriction: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ServiceRestriction: usize,
    pub EnableRuleGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletypesbitmask: i32, group: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, enable: i16) -> ::windows_core::HRESULT,
    pub IsRuleGroupEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletypesbitmask: i32, group: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, enabled: *mut i16) -> ::windows_core::HRESULT,
    pub RestoreLocalFirewallDefaults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub get_DefaultInboundAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: *mut NET_FW_ACTION) -> ::windows_core::HRESULT,
    pub put_DefaultInboundAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> ::windows_core::HRESULT,
    pub get_DefaultOutboundAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: *mut NET_FW_ACTION) -> ::windows_core::HRESULT,
    pub put_DefaultOutboundAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> ::windows_core::HRESULT,
    pub get_IsRuleGroupCurrentlyEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, group: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, enabled: *mut i16) -> ::windows_core::HRESULT,
    pub LocalPolicyModifyState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modifystate: *mut NET_FW_MODIFY_STATE) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct INetFwProduct(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl INetFwProduct {
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn RuleCategories(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).RuleCategories)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetRuleCategories<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, rulecategories: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRuleCategories)(::windows_core::Interface::as_raw(self), rulecategories.into_param().abi()).ok()
    }
    pub unsafe fn DisplayName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).DisplayName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDisplayName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, displayname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDisplayName)(::windows_core::Interface::as_raw(self), displayname.into_param().abi()).ok()
    }
    pub unsafe fn PathToSignedProductExe(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).PathToSignedProductExe)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwProduct> for ::windows_core::IUnknown {
    fn from(value: INetFwProduct) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwProduct> for ::windows_core::IUnknown {
    fn from(value: &INetFwProduct) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INetFwProduct {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INetFwProduct {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwProduct> for ::win32_system::Com::IDispatch {
    fn from(value: INetFwProduct) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwProduct> for ::win32_system::Com::IDispatch {
    fn from(value: &INetFwProduct) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for INetFwProduct {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a INetFwProduct {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for INetFwProduct {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for INetFwProduct {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for INetFwProduct {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for INetFwProduct {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetFwProduct").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for INetFwProduct {
    type Vtable = INetFwProduct_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x71881699_18f4_458b_b892_3ffce5e07f75);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwProduct_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub RuleCategories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rulecategories: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    RuleCategories: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetRuleCategories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rulecategories: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetRuleCategories: usize,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub PathToSignedProductExe: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct INetFwProducts(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl INetFwProducts {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Register<'a, Param0: ::windows_core::IntoParam<'a, INetFwProduct>>(&self, product: Param0) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).Register)(::windows_core::Interface::as_raw(self), product.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Item(&self, index: i32) -> ::windows_core::Result<INetFwProduct> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INetFwProduct>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwProducts> for ::windows_core::IUnknown {
    fn from(value: INetFwProducts) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwProducts> for ::windows_core::IUnknown {
    fn from(value: &INetFwProducts) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INetFwProducts {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INetFwProducts {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwProducts> for ::win32_system::Com::IDispatch {
    fn from(value: INetFwProducts) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwProducts> for ::win32_system::Com::IDispatch {
    fn from(value: &INetFwProducts) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for INetFwProducts {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a INetFwProducts {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for INetFwProducts {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for INetFwProducts {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for INetFwProducts {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for INetFwProducts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetFwProducts").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for INetFwProducts {
    type Vtable = INetFwProducts_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x39eb36e0_2097_40bd_8af2_63a13b525362);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwProducts_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Register: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, product: ::windows_core::RawPtr, registration: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Register: usize,
    #[cfg(feature = "win32-system")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, product: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct INetFwProfile(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl INetFwProfile {
    pub unsafe fn Type(&self) -> ::windows_core::Result<NET_FW_PROFILE_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<NET_FW_PROFILE_TYPE>::zeroed();
        (::windows_core::Interface::vtable(self).Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<NET_FW_PROFILE_TYPE>(result__)
    }
    pub unsafe fn FirewallEnabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).FirewallEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetFirewallEnabled(&self, enabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFirewallEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enabled)).ok()
    }
    pub unsafe fn ExceptionsNotAllowed(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).ExceptionsNotAllowed)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetExceptionsNotAllowed(&self, notallowed: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetExceptionsNotAllowed)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(notallowed)).ok()
    }
    pub unsafe fn NotificationsDisabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).NotificationsDisabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetNotificationsDisabled(&self, disabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetNotificationsDisabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(disabled)).ok()
    }
    pub unsafe fn UnicastResponsesToMulticastBroadcastDisabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).UnicastResponsesToMulticastBroadcastDisabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetUnicastResponsesToMulticastBroadcastDisabled(&self, disabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetUnicastResponsesToMulticastBroadcastDisabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(disabled)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn RemoteAdminSettings(&self) -> ::windows_core::Result<INetFwRemoteAdminSettings> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).RemoteAdminSettings)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INetFwRemoteAdminSettings>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn IcmpSettings(&self) -> ::windows_core::Result<INetFwIcmpSettings> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).IcmpSettings)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INetFwIcmpSettings>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GloballyOpenPorts(&self) -> ::windows_core::Result<INetFwOpenPorts> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GloballyOpenPorts)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INetFwOpenPorts>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Services(&self) -> ::windows_core::Result<INetFwServices> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Services)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INetFwServices>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn AuthorizedApplications(&self) -> ::windows_core::Result<INetFwAuthorizedApplications> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).AuthorizedApplications)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INetFwAuthorizedApplications>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwProfile> for ::windows_core::IUnknown {
    fn from(value: INetFwProfile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwProfile> for ::windows_core::IUnknown {
    fn from(value: &INetFwProfile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INetFwProfile {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INetFwProfile {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwProfile> for ::win32_system::Com::IDispatch {
    fn from(value: INetFwProfile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwProfile> for ::win32_system::Com::IDispatch {
    fn from(value: &INetFwProfile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for INetFwProfile {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a INetFwProfile {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for INetFwProfile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for INetFwProfile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for INetFwProfile {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for INetFwProfile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetFwProfile").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for INetFwProfile {
    type Vtable = INetFwProfile_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x174a0dda_e9f9_449d_993b_21ab667ca456);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwProfile_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut NET_FW_PROFILE_TYPE) -> ::windows_core::HRESULT,
    pub FirewallEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows_core::HRESULT,
    pub SetFirewallEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows_core::HRESULT,
    pub ExceptionsNotAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notallowed: *mut i16) -> ::windows_core::HRESULT,
    pub SetExceptionsNotAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notallowed: i16) -> ::windows_core::HRESULT,
    pub NotificationsDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disabled: *mut i16) -> ::windows_core::HRESULT,
    pub SetNotificationsDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disabled: i16) -> ::windows_core::HRESULT,
    pub UnicastResponsesToMulticastBroadcastDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disabled: *mut i16) -> ::windows_core::HRESULT,
    pub SetUnicastResponsesToMulticastBroadcastDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disabled: i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub RemoteAdminSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteadminsettings: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    RemoteAdminSettings: usize,
    #[cfg(feature = "win32-system")]
    pub IcmpSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icmpsettings: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    IcmpSettings: usize,
    #[cfg(feature = "win32-system")]
    pub GloballyOpenPorts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, openports: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GloballyOpenPorts: usize,
    #[cfg(feature = "win32-system")]
    pub Services: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, services: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Services: usize,
    #[cfg(feature = "win32-system")]
    pub AuthorizedApplications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, apps: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    AuthorizedApplications: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct INetFwRemoteAdminSettings(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl INetFwRemoteAdminSettings {
    pub unsafe fn IpVersion(&self) -> ::windows_core::Result<NET_FW_IP_VERSION> {
        let mut result__ = ::core::mem::MaybeUninit::<NET_FW_IP_VERSION>::zeroed();
        (::windows_core::Interface::vtable(self).IpVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<NET_FW_IP_VERSION>(result__)
    }
    pub unsafe fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIpVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ipversion)).ok()
    }
    pub unsafe fn Scope(&self) -> ::windows_core::Result<NET_FW_SCOPE> {
        let mut result__ = ::core::mem::MaybeUninit::<NET_FW_SCOPE>::zeroed();
        (::windows_core::Interface::vtable(self).Scope)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<NET_FW_SCOPE>(result__)
    }
    pub unsafe fn SetScope(&self, scope: NET_FW_SCOPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetScope)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(scope)).ok()
    }
    pub unsafe fn RemoteAddresses(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).RemoteAddresses)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetRemoteAddresses<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, remoteaddrs: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRemoteAddresses)(::windows_core::Interface::as_raw(self), remoteaddrs.into_param().abi()).ok()
    }
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).Enabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enabled)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwRemoteAdminSettings> for ::windows_core::IUnknown {
    fn from(value: INetFwRemoteAdminSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwRemoteAdminSettings> for ::windows_core::IUnknown {
    fn from(value: &INetFwRemoteAdminSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INetFwRemoteAdminSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INetFwRemoteAdminSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwRemoteAdminSettings> for ::win32_system::Com::IDispatch {
    fn from(value: INetFwRemoteAdminSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwRemoteAdminSettings> for ::win32_system::Com::IDispatch {
    fn from(value: &INetFwRemoteAdminSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for INetFwRemoteAdminSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a INetFwRemoteAdminSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for INetFwRemoteAdminSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for INetFwRemoteAdminSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for INetFwRemoteAdminSettings {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for INetFwRemoteAdminSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetFwRemoteAdminSettings").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for INetFwRemoteAdminSettings {
    type Vtable = INetFwRemoteAdminSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd4becddf_6f73_4a83_b832_9c66874cd20e);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRemoteAdminSettings_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub IpVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> ::windows_core::HRESULT,
    pub SetIpVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> ::windows_core::HRESULT,
    pub Scope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> ::windows_core::HRESULT,
    pub SetScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: NET_FW_SCOPE) -> ::windows_core::HRESULT,
    pub RemoteAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteaddrs: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetRemoteAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteaddrs: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct INetFwRule(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl INetFwRule {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, desc: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), desc.into_param().abi()).ok()
    }
    pub unsafe fn ApplicationName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ApplicationName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetApplicationName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, imagefilename: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetApplicationName)(::windows_core::Interface::as_raw(self), imagefilename.into_param().abi()).ok()
    }
    pub unsafe fn ServiceName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ServiceName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetServiceName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, servicename: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetServiceName)(::windows_core::Interface::as_raw(self), servicename.into_param().abi()).ok()
    }
    pub unsafe fn Protocol(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Protocol)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetProtocol(&self, protocol: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProtocol)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(protocol)).ok()
    }
    pub unsafe fn LocalPorts(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).LocalPorts)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetLocalPorts<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, portnumbers: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLocalPorts)(::windows_core::Interface::as_raw(self), portnumbers.into_param().abi()).ok()
    }
    pub unsafe fn RemotePorts(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).RemotePorts)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetRemotePorts<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, portnumbers: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRemotePorts)(::windows_core::Interface::as_raw(self), portnumbers.into_param().abi()).ok()
    }
    pub unsafe fn LocalAddresses(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).LocalAddresses)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetLocalAddresses<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, localaddrs: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLocalAddresses)(::windows_core::Interface::as_raw(self), localaddrs.into_param().abi()).ok()
    }
    pub unsafe fn RemoteAddresses(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).RemoteAddresses)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetRemoteAddresses<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, remoteaddrs: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRemoteAddresses)(::windows_core::Interface::as_raw(self), remoteaddrs.into_param().abi()).ok()
    }
    pub unsafe fn IcmpTypesAndCodes(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).IcmpTypesAndCodes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetIcmpTypesAndCodes<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, icmptypesandcodes: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIcmpTypesAndCodes)(::windows_core::Interface::as_raw(self), icmptypesandcodes.into_param().abi()).ok()
    }
    pub unsafe fn Direction(&self) -> ::windows_core::Result<NET_FW_RULE_DIRECTION> {
        let mut result__ = ::core::mem::MaybeUninit::<NET_FW_RULE_DIRECTION>::zeroed();
        (::windows_core::Interface::vtable(self).Direction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<NET_FW_RULE_DIRECTION>(result__)
    }
    pub unsafe fn SetDirection(&self, dir: NET_FW_RULE_DIRECTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDirection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dir)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Interfaces(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Interfaces)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetInterfaces<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, interfaces: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInterfaces)(::windows_core::Interface::as_raw(self), interfaces.into_param().abi()).ok()
    }
    pub unsafe fn InterfaceTypes(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).InterfaceTypes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetInterfaceTypes<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, interfacetypes: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInterfaceTypes)(::windows_core::Interface::as_raw(self), interfacetypes.into_param().abi()).ok()
    }
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).Enabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enabled)).ok()
    }
    pub unsafe fn Grouping(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Grouping)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetGrouping<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, context: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetGrouping)(::windows_core::Interface::as_raw(self), context.into_param().abi()).ok()
    }
    pub unsafe fn Profiles(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Profiles)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetProfiles(&self, profiletypesbitmask: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProfiles)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(profiletypesbitmask)).ok()
    }
    pub unsafe fn EdgeTraversal(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).EdgeTraversal)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEdgeTraversal(&self, enabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEdgeTraversal)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enabled)).ok()
    }
    pub unsafe fn Action(&self) -> ::windows_core::Result<NET_FW_ACTION> {
        let mut result__ = ::core::mem::MaybeUninit::<NET_FW_ACTION>::zeroed();
        (::windows_core::Interface::vtable(self).Action)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<NET_FW_ACTION>(result__)
    }
    pub unsafe fn SetAction(&self, action: NET_FW_ACTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(action)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwRule> for ::windows_core::IUnknown {
    fn from(value: INetFwRule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwRule> for ::windows_core::IUnknown {
    fn from(value: &INetFwRule) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INetFwRule {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INetFwRule {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwRule> for ::win32_system::Com::IDispatch {
    fn from(value: INetFwRule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwRule> for ::win32_system::Com::IDispatch {
    fn from(value: &INetFwRule) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for INetFwRule {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a INetFwRule {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for INetFwRule {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for INetFwRule {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for INetFwRule {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for INetFwRule {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetFwRule").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for INetFwRule {
    type Vtable = INetFwRule_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaf230d27_baba_4e42_aced_f524f22cfce2);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRule_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desc: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desc: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub ApplicationName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagefilename: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetApplicationName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagefilename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub ServiceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servicename: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetServiceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servicename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Protocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protocol: *mut i32) -> ::windows_core::HRESULT,
    pub SetProtocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protocol: i32) -> ::windows_core::HRESULT,
    pub LocalPorts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, portnumbers: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetLocalPorts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, portnumbers: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub RemotePorts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, portnumbers: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetRemotePorts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, portnumbers: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub LocalAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localaddrs: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetLocalAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localaddrs: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub RemoteAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteaddrs: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetRemoteAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteaddrs: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub IcmpTypesAndCodes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icmptypesandcodes: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetIcmpTypesAndCodes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icmptypesandcodes: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Direction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dir: *mut NET_FW_RULE_DIRECTION) -> ::windows_core::HRESULT,
    pub SetDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dir: NET_FW_RULE_DIRECTION) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Interfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfaces: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Interfaces: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetInterfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfaces: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetInterfaces: usize,
    pub InterfaceTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfacetypes: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetInterfaceTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfacetypes: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows_core::HRESULT,
    pub Grouping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetGrouping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Profiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletypesbitmask: *mut i32) -> ::windows_core::HRESULT,
    pub SetProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletypesbitmask: i32) -> ::windows_core::HRESULT,
    pub EdgeTraversal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows_core::HRESULT,
    pub SetEdgeTraversal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows_core::HRESULT,
    pub Action: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, action: *mut NET_FW_ACTION) -> ::windows_core::HRESULT,
    pub SetAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, action: NET_FW_ACTION) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct INetFwRule2(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl INetFwRule2 {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, desc: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDescription)(::windows_core::Interface::as_raw(self), desc.into_param().abi()).ok()
    }
    pub unsafe fn ApplicationName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ApplicationName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetApplicationName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, imagefilename: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetApplicationName)(::windows_core::Interface::as_raw(self), imagefilename.into_param().abi()).ok()
    }
    pub unsafe fn ServiceName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ServiceName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetServiceName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, servicename: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetServiceName)(::windows_core::Interface::as_raw(self), servicename.into_param().abi()).ok()
    }
    pub unsafe fn Protocol(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Protocol)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetProtocol(&self, protocol: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetProtocol)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(protocol)).ok()
    }
    pub unsafe fn LocalPorts(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.LocalPorts)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetLocalPorts<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, portnumbers: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetLocalPorts)(::windows_core::Interface::as_raw(self), portnumbers.into_param().abi()).ok()
    }
    pub unsafe fn RemotePorts(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.RemotePorts)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetRemotePorts<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, portnumbers: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRemotePorts)(::windows_core::Interface::as_raw(self), portnumbers.into_param().abi()).ok()
    }
    pub unsafe fn LocalAddresses(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.LocalAddresses)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetLocalAddresses<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, localaddrs: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetLocalAddresses)(::windows_core::Interface::as_raw(self), localaddrs.into_param().abi()).ok()
    }
    pub unsafe fn RemoteAddresses(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.RemoteAddresses)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetRemoteAddresses<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, remoteaddrs: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRemoteAddresses)(::windows_core::Interface::as_raw(self), remoteaddrs.into_param().abi()).ok()
    }
    pub unsafe fn IcmpTypesAndCodes(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IcmpTypesAndCodes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetIcmpTypesAndCodes<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, icmptypesandcodes: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetIcmpTypesAndCodes)(::windows_core::Interface::as_raw(self), icmptypesandcodes.into_param().abi()).ok()
    }
    pub unsafe fn Direction(&self) -> ::windows_core::Result<NET_FW_RULE_DIRECTION> {
        let mut result__ = ::core::mem::MaybeUninit::<NET_FW_RULE_DIRECTION>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Direction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<NET_FW_RULE_DIRECTION>(result__)
    }
    pub unsafe fn SetDirection(&self, dir: NET_FW_RULE_DIRECTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDirection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dir)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Interfaces(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Interfaces)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetInterfaces<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, interfaces: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInterfaces)(::windows_core::Interface::as_raw(self), interfaces.into_param().abi()).ok()
    }
    pub unsafe fn InterfaceTypes(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.InterfaceTypes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetInterfaceTypes<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, interfacetypes: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInterfaceTypes)(::windows_core::Interface::as_raw(self), interfacetypes.into_param().abi()).ok()
    }
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Enabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enabled)).ok()
    }
    pub unsafe fn Grouping(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Grouping)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetGrouping<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, context: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetGrouping)(::windows_core::Interface::as_raw(self), context.into_param().abi()).ok()
    }
    pub unsafe fn Profiles(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Profiles)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetProfiles(&self, profiletypesbitmask: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetProfiles)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(profiletypesbitmask)).ok()
    }
    pub unsafe fn EdgeTraversal(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.EdgeTraversal)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEdgeTraversal(&self, enabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEdgeTraversal)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enabled)).ok()
    }
    pub unsafe fn Action(&self) -> ::windows_core::Result<NET_FW_ACTION> {
        let mut result__ = ::core::mem::MaybeUninit::<NET_FW_ACTION>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Action)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<NET_FW_ACTION>(result__)
    }
    pub unsafe fn SetAction(&self, action: NET_FW_ACTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetAction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(action)).ok()
    }
    pub unsafe fn EdgeTraversalOptions(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).EdgeTraversalOptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetEdgeTraversalOptions(&self, loptions: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEdgeTraversalOptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(loptions)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwRule2> for ::windows_core::IUnknown {
    fn from(value: INetFwRule2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwRule2> for ::windows_core::IUnknown {
    fn from(value: &INetFwRule2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INetFwRule2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INetFwRule2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwRule2> for ::win32_system::Com::IDispatch {
    fn from(value: INetFwRule2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwRule2> for ::win32_system::Com::IDispatch {
    fn from(value: &INetFwRule2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for INetFwRule2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a INetFwRule2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwRule2> for INetFwRule {
    fn from(value: INetFwRule2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwRule2> for INetFwRule {
    fn from(value: &INetFwRule2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, INetFwRule> for INetFwRule2 {
    fn into_param(self) -> ::windows_core::Param<'a, INetFwRule> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, INetFwRule> for &'a INetFwRule2 {
    fn into_param(self) -> ::windows_core::Param<'a, INetFwRule> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for INetFwRule2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for INetFwRule2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for INetFwRule2 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for INetFwRule2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetFwRule2").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for INetFwRule2 {
    type Vtable = INetFwRule2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9c27c8da_189b_4dde_89f7_8b39a316782c);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRule2_Vtbl {
    pub base__: INetFwRule_Vtbl,
    pub EdgeTraversalOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loptions: *mut i32) -> ::windows_core::HRESULT,
    pub SetEdgeTraversalOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loptions: i32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct INetFwRule3(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl INetFwRule3 {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, desc: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetDescription)(::windows_core::Interface::as_raw(self), desc.into_param().abi()).ok()
    }
    pub unsafe fn ApplicationName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ApplicationName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetApplicationName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, imagefilename: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetApplicationName)(::windows_core::Interface::as_raw(self), imagefilename.into_param().abi()).ok()
    }
    pub unsafe fn ServiceName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ServiceName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetServiceName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, servicename: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetServiceName)(::windows_core::Interface::as_raw(self), servicename.into_param().abi()).ok()
    }
    pub unsafe fn Protocol(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Protocol)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetProtocol(&self, protocol: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetProtocol)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(protocol)).ok()
    }
    pub unsafe fn LocalPorts(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.LocalPorts)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetLocalPorts<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, portnumbers: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetLocalPorts)(::windows_core::Interface::as_raw(self), portnumbers.into_param().abi()).ok()
    }
    pub unsafe fn RemotePorts(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.RemotePorts)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetRemotePorts<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, portnumbers: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetRemotePorts)(::windows_core::Interface::as_raw(self), portnumbers.into_param().abi()).ok()
    }
    pub unsafe fn LocalAddresses(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.LocalAddresses)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetLocalAddresses<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, localaddrs: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetLocalAddresses)(::windows_core::Interface::as_raw(self), localaddrs.into_param().abi()).ok()
    }
    pub unsafe fn RemoteAddresses(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.RemoteAddresses)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetRemoteAddresses<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, remoteaddrs: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetRemoteAddresses)(::windows_core::Interface::as_raw(self), remoteaddrs.into_param().abi()).ok()
    }
    pub unsafe fn IcmpTypesAndCodes(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.IcmpTypesAndCodes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetIcmpTypesAndCodes<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, icmptypesandcodes: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetIcmpTypesAndCodes)(::windows_core::Interface::as_raw(self), icmptypesandcodes.into_param().abi()).ok()
    }
    pub unsafe fn Direction(&self) -> ::windows_core::Result<NET_FW_RULE_DIRECTION> {
        let mut result__ = ::core::mem::MaybeUninit::<NET_FW_RULE_DIRECTION>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Direction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<NET_FW_RULE_DIRECTION>(result__)
    }
    pub unsafe fn SetDirection(&self, dir: NET_FW_RULE_DIRECTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetDirection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dir)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Interfaces(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Interfaces)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetInterfaces<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, interfaces: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetInterfaces)(::windows_core::Interface::as_raw(self), interfaces.into_param().abi()).ok()
    }
    pub unsafe fn InterfaceTypes(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.InterfaceTypes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetInterfaceTypes<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, interfacetypes: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetInterfaceTypes)(::windows_core::Interface::as_raw(self), interfacetypes.into_param().abi()).ok()
    }
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Enabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enabled)).ok()
    }
    pub unsafe fn Grouping(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Grouping)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetGrouping<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, context: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetGrouping)(::windows_core::Interface::as_raw(self), context.into_param().abi()).ok()
    }
    pub unsafe fn Profiles(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Profiles)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetProfiles(&self, profiletypesbitmask: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetProfiles)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(profiletypesbitmask)).ok()
    }
    pub unsafe fn EdgeTraversal(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.EdgeTraversal)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEdgeTraversal(&self, enabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetEdgeTraversal)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enabled)).ok()
    }
    pub unsafe fn Action(&self) -> ::windows_core::Result<NET_FW_ACTION> {
        let mut result__ = ::core::mem::MaybeUninit::<NET_FW_ACTION>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Action)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<NET_FW_ACTION>(result__)
    }
    pub unsafe fn SetAction(&self, action: NET_FW_ACTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetAction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(action)).ok()
    }
    pub unsafe fn EdgeTraversalOptions(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.EdgeTraversalOptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetEdgeTraversalOptions(&self, loptions: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEdgeTraversalOptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(loptions)).ok()
    }
    pub unsafe fn LocalAppPackageId(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).LocalAppPackageId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetLocalAppPackageId<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, wszpackageid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLocalAppPackageId)(::windows_core::Interface::as_raw(self), wszpackageid.into_param().abi()).ok()
    }
    pub unsafe fn LocalUserOwner(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).LocalUserOwner)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetLocalUserOwner<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, wszuserowner: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLocalUserOwner)(::windows_core::Interface::as_raw(self), wszuserowner.into_param().abi()).ok()
    }
    pub unsafe fn LocalUserAuthorizedList(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).LocalUserAuthorizedList)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetLocalUserAuthorizedList<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, wszuserauthlist: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLocalUserAuthorizedList)(::windows_core::Interface::as_raw(self), wszuserauthlist.into_param().abi()).ok()
    }
    pub unsafe fn RemoteUserAuthorizedList(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).RemoteUserAuthorizedList)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetRemoteUserAuthorizedList<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, wszuserauthlist: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRemoteUserAuthorizedList)(::windows_core::Interface::as_raw(self), wszuserauthlist.into_param().abi()).ok()
    }
    pub unsafe fn RemoteMachineAuthorizedList(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).RemoteMachineAuthorizedList)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetRemoteMachineAuthorizedList<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, wszuserauthlist: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRemoteMachineAuthorizedList)(::windows_core::Interface::as_raw(self), wszuserauthlist.into_param().abi()).ok()
    }
    pub unsafe fn SecureFlags(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).SecureFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetSecureFlags(&self, loptions: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSecureFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(loptions)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwRule3> for ::windows_core::IUnknown {
    fn from(value: INetFwRule3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwRule3> for ::windows_core::IUnknown {
    fn from(value: &INetFwRule3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INetFwRule3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INetFwRule3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwRule3> for ::win32_system::Com::IDispatch {
    fn from(value: INetFwRule3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwRule3> for ::win32_system::Com::IDispatch {
    fn from(value: &INetFwRule3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for INetFwRule3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a INetFwRule3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwRule3> for INetFwRule {
    fn from(value: INetFwRule3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwRule3> for INetFwRule {
    fn from(value: &INetFwRule3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, INetFwRule> for INetFwRule3 {
    fn into_param(self) -> ::windows_core::Param<'a, INetFwRule> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, INetFwRule> for &'a INetFwRule3 {
    fn into_param(self) -> ::windows_core::Param<'a, INetFwRule> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwRule3> for INetFwRule2 {
    fn from(value: INetFwRule3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwRule3> for INetFwRule2 {
    fn from(value: &INetFwRule3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, INetFwRule2> for INetFwRule3 {
    fn into_param(self) -> ::windows_core::Param<'a, INetFwRule2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, INetFwRule2> for &'a INetFwRule3 {
    fn into_param(self) -> ::windows_core::Param<'a, INetFwRule2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for INetFwRule3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for INetFwRule3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for INetFwRule3 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for INetFwRule3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetFwRule3").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for INetFwRule3 {
    type Vtable = INetFwRule3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb21563ff_d696_4222_ab46_4e89b73ab34a);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRule3_Vtbl {
    pub base__: INetFwRule2_Vtbl,
    pub LocalAppPackageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszpackageid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetLocalAppPackageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszpackageid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub LocalUserOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszuserowner: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetLocalUserOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszuserowner: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub LocalUserAuthorizedList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszuserauthlist: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetLocalUserAuthorizedList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszuserauthlist: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub RemoteUserAuthorizedList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszuserauthlist: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetRemoteUserAuthorizedList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszuserauthlist: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub RemoteMachineAuthorizedList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszuserauthlist: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetRemoteMachineAuthorizedList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszuserauthlist: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub SecureFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loptions: *mut i32) -> ::windows_core::HRESULT,
    pub SetSecureFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loptions: i32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct INetFwRules(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl INetFwRules {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Add<'a, Param0: ::windows_core::IntoParam<'a, INetFwRule>>(&self, rule: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), rule.into_param().abi()).ok()
    }
    pub unsafe fn Remove<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Item<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<INetFwRule> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), name.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INetFwRule>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwRules> for ::windows_core::IUnknown {
    fn from(value: INetFwRules) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwRules> for ::windows_core::IUnknown {
    fn from(value: &INetFwRules) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INetFwRules {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INetFwRules {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwRules> for ::win32_system::Com::IDispatch {
    fn from(value: INetFwRules) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwRules> for ::win32_system::Com::IDispatch {
    fn from(value: &INetFwRules) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for INetFwRules {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a INetFwRules {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for INetFwRules {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for INetFwRules {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for INetFwRules {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for INetFwRules {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetFwRules").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for INetFwRules {
    type Vtable = INetFwRules_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9c4c6277_5027_441e_afae_ca1f542da009);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRules_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rule: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, rule: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct INetFwService(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl INetFwService {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Type(&self) -> ::windows_core::Result<NET_FW_SERVICE_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<NET_FW_SERVICE_TYPE>::zeroed();
        (::windows_core::Interface::vtable(self).Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<NET_FW_SERVICE_TYPE>(result__)
    }
    pub unsafe fn Customized(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).Customized)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn IpVersion(&self) -> ::windows_core::Result<NET_FW_IP_VERSION> {
        let mut result__ = ::core::mem::MaybeUninit::<NET_FW_IP_VERSION>::zeroed();
        (::windows_core::Interface::vtable(self).IpVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<NET_FW_IP_VERSION>(result__)
    }
    pub unsafe fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIpVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ipversion)).ok()
    }
    pub unsafe fn Scope(&self) -> ::windows_core::Result<NET_FW_SCOPE> {
        let mut result__ = ::core::mem::MaybeUninit::<NET_FW_SCOPE>::zeroed();
        (::windows_core::Interface::vtable(self).Scope)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<NET_FW_SCOPE>(result__)
    }
    pub unsafe fn SetScope(&self, scope: NET_FW_SCOPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetScope)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(scope)).ok()
    }
    pub unsafe fn RemoteAddresses(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).RemoteAddresses)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetRemoteAddresses<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, remoteaddrs: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRemoteAddresses)(::windows_core::Interface::as_raw(self), remoteaddrs.into_param().abi()).ok()
    }
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).Enabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enabled)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GloballyOpenPorts(&self) -> ::windows_core::Result<INetFwOpenPorts> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GloballyOpenPorts)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INetFwOpenPorts>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwService> for ::windows_core::IUnknown {
    fn from(value: INetFwService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwService> for ::windows_core::IUnknown {
    fn from(value: &INetFwService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INetFwService {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INetFwService {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwService> for ::win32_system::Com::IDispatch {
    fn from(value: INetFwService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwService> for ::win32_system::Com::IDispatch {
    fn from(value: &INetFwService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for INetFwService {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a INetFwService {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for INetFwService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for INetFwService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for INetFwService {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for INetFwService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetFwService").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for INetFwService {
    type Vtable = INetFwService_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79fd57c8_908e_4a36_9888_d5b3f0a444cf);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwService_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut NET_FW_SERVICE_TYPE) -> ::windows_core::HRESULT,
    pub Customized: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customized: *mut i16) -> ::windows_core::HRESULT,
    pub IpVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> ::windows_core::HRESULT,
    pub SetIpVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> ::windows_core::HRESULT,
    pub Scope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> ::windows_core::HRESULT,
    pub SetScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: NET_FW_SCOPE) -> ::windows_core::HRESULT,
    pub RemoteAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteaddrs: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetRemoteAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteaddrs: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub GloballyOpenPorts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, openports: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GloballyOpenPorts: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct INetFwServiceRestriction(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl INetFwServiceRestriction {
    pub unsafe fn RestrictService<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, servicename: Param0, appname: Param1, restrictservice: i16, servicesidrestricted: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RestrictService)(::windows_core::Interface::as_raw(self), servicename.into_param().abi(), appname.into_param().abi(), ::core::mem::transmute(restrictservice), ::core::mem::transmute(servicesidrestricted)).ok()
    }
    pub unsafe fn ServiceRestricted<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, servicename: Param0, appname: Param1) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).ServiceRestricted)(::windows_core::Interface::as_raw(self), servicename.into_param().abi(), appname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Rules(&self) -> ::windows_core::Result<INetFwRules> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Rules)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INetFwRules>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwServiceRestriction> for ::windows_core::IUnknown {
    fn from(value: INetFwServiceRestriction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwServiceRestriction> for ::windows_core::IUnknown {
    fn from(value: &INetFwServiceRestriction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INetFwServiceRestriction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INetFwServiceRestriction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwServiceRestriction> for ::win32_system::Com::IDispatch {
    fn from(value: INetFwServiceRestriction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwServiceRestriction> for ::win32_system::Com::IDispatch {
    fn from(value: &INetFwServiceRestriction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for INetFwServiceRestriction {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a INetFwServiceRestriction {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for INetFwServiceRestriction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for INetFwServiceRestriction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for INetFwServiceRestriction {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for INetFwServiceRestriction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetFwServiceRestriction").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for INetFwServiceRestriction {
    type Vtable = INetFwServiceRestriction_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8267bbe3_f890_491c_b7b6_2db1ef0e5d2b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwServiceRestriction_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub RestrictService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servicename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, appname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, restrictservice: i16, servicesidrestricted: i16) -> ::windows_core::HRESULT,
    pub ServiceRestricted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servicename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, appname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, servicerestricted: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Rules: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rules: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Rules: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct INetFwServices(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl INetFwServices {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Item(&self, svctype: NET_FW_SERVICE_TYPE) -> ::windows_core::Result<INetFwService> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(svctype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INetFwService>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwServices> for ::windows_core::IUnknown {
    fn from(value: INetFwServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwServices> for ::windows_core::IUnknown {
    fn from(value: &INetFwServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INetFwServices {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INetFwServices {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetFwServices> for ::win32_system::Com::IDispatch {
    fn from(value: INetFwServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetFwServices> for ::win32_system::Com::IDispatch {
    fn from(value: &INetFwServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for INetFwServices {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a INetFwServices {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for INetFwServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for INetFwServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for INetFwServices {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for INetFwServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetFwServices").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for INetFwServices {
    type Vtable = INetFwServices_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79649bb4_903e_421b_94c9_79848e79f6ee);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwServices_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, svctype: NET_FW_SERVICE_TYPE, service: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct INetSharingConfiguration(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl INetSharingConfiguration {
    pub unsafe fn SharingEnabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).SharingEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SharingConnectionType(&self) -> ::windows_core::Result<SHARINGCONNECTIONTYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<SHARINGCONNECTIONTYPE>::zeroed();
        (::windows_core::Interface::vtable(self).SharingConnectionType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<SHARINGCONNECTIONTYPE>(result__)
    }
    pub unsafe fn DisableSharing(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DisableSharing)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnableSharing(&self, r#type: SHARINGCONNECTIONTYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnableSharing)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(r#type)).ok()
    }
    pub unsafe fn InternetFirewallEnabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).InternetFirewallEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn DisableInternetFirewall(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DisableInternetFirewall)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnableInternetFirewall(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnableInternetFirewall)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn get_EnumPortMappings(&self, flags: SHARINGCONNECTION_ENUM_FLAGS) -> ::windows_core::Result<INetSharingPortMappingCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).get_EnumPortMappings)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(flags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INetSharingPortMappingCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn AddPortMapping<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param5: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrname: Param0, ucipprotocol: u8, usexternalport: u16, usinternalport: u16, dwoptions: u32, bstrtargetnameoripaddress: Param5, etargettype: ICS_TARGETTYPE) -> ::windows_core::Result<INetSharingPortMapping> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).AddPortMapping)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), ::core::mem::transmute(ucipprotocol), ::core::mem::transmute(usexternalport), ::core::mem::transmute(usinternalport), ::core::mem::transmute(dwoptions), bstrtargetnameoripaddress.into_param().abi(), ::core::mem::transmute(etargettype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INetSharingPortMapping>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn RemovePortMapping<'a, Param0: ::windows_core::IntoParam<'a, INetSharingPortMapping>>(&self, pmapping: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemovePortMapping)(::windows_core::Interface::as_raw(self), pmapping.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetSharingConfiguration> for ::windows_core::IUnknown {
    fn from(value: INetSharingConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetSharingConfiguration> for ::windows_core::IUnknown {
    fn from(value: &INetSharingConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INetSharingConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INetSharingConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetSharingConfiguration> for ::win32_system::Com::IDispatch {
    fn from(value: INetSharingConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetSharingConfiguration> for ::win32_system::Com::IDispatch {
    fn from(value: &INetSharingConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for INetSharingConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a INetSharingConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for INetSharingConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for INetSharingConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for INetSharingConfiguration {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for INetSharingConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetSharingConfiguration").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for INetSharingConfiguration {
    type Vtable = INetSharingConfiguration_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc08956b6_1cd3_11d1_b1c5_00805fc1270e);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingConfiguration_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub SharingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows_core::HRESULT,
    pub SharingConnectionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut SHARINGCONNECTIONTYPE) -> ::windows_core::HRESULT,
    pub DisableSharing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EnableSharing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: SHARINGCONNECTIONTYPE) -> ::windows_core::HRESULT,
    pub InternetFirewallEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows_core::HRESULT,
    pub DisableInternetFirewall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EnableInternetFirewall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub get_EnumPortMappings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    get_EnumPortMappings: usize,
    #[cfg(feature = "win32-system")]
    pub AddPortMapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ucipprotocol: u8, usexternalport: u16, usinternalport: u16, dwoptions: u32, bstrtargetnameoripaddress: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, etargettype: ICS_TARGETTYPE, ppmapping: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    AddPortMapping: usize,
    #[cfg(feature = "win32-system")]
    pub RemovePortMapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmapping: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    RemovePortMapping: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct INetSharingEveryConnectionCollection(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl INetSharingEveryConnectionCollection {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetSharingEveryConnectionCollection> for ::windows_core::IUnknown {
    fn from(value: INetSharingEveryConnectionCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetSharingEveryConnectionCollection> for ::windows_core::IUnknown {
    fn from(value: &INetSharingEveryConnectionCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INetSharingEveryConnectionCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INetSharingEveryConnectionCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetSharingEveryConnectionCollection> for ::win32_system::Com::IDispatch {
    fn from(value: INetSharingEveryConnectionCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetSharingEveryConnectionCollection> for ::win32_system::Com::IDispatch {
    fn from(value: &INetSharingEveryConnectionCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for INetSharingEveryConnectionCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a INetSharingEveryConnectionCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for INetSharingEveryConnectionCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for INetSharingEveryConnectionCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for INetSharingEveryConnectionCollection {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for INetSharingEveryConnectionCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetSharingEveryConnectionCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for INetSharingEveryConnectionCollection {
    type Vtable = INetSharingEveryConnectionCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x33c4643c_7811_46fa_a89a_768597bd7223);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingEveryConnectionCollection_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct INetSharingManager(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl INetSharingManager {
    pub unsafe fn SharingInstalled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).SharingInstalled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn get_EnumPublicConnections(&self, flags: SHARINGCONNECTION_ENUM_FLAGS) -> ::windows_core::Result<INetSharingPublicConnectionCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).get_EnumPublicConnections)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(flags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INetSharingPublicConnectionCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn get_EnumPrivateConnections(&self, flags: SHARINGCONNECTION_ENUM_FLAGS) -> ::windows_core::Result<INetSharingPrivateConnectionCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).get_EnumPrivateConnections)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(flags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INetSharingPrivateConnectionCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn get_INetSharingConfigurationForINetConnection<'a, Param0: ::windows_core::IntoParam<'a, INetConnection>>(&self, pnetconnection: Param0) -> ::windows_core::Result<INetSharingConfiguration> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).get_INetSharingConfigurationForINetConnection)(::windows_core::Interface::as_raw(self), pnetconnection.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INetSharingConfiguration>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn EnumEveryConnection(&self) -> ::windows_core::Result<INetSharingEveryConnectionCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumEveryConnection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INetSharingEveryConnectionCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn get_NetConnectionProps<'a, Param0: ::windows_core::IntoParam<'a, INetConnection>>(&self, pnetconnection: Param0) -> ::windows_core::Result<INetConnectionProps> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).get_NetConnectionProps)(::windows_core::Interface::as_raw(self), pnetconnection.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INetConnectionProps>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetSharingManager> for ::windows_core::IUnknown {
    fn from(value: INetSharingManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetSharingManager> for ::windows_core::IUnknown {
    fn from(value: &INetSharingManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INetSharingManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INetSharingManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetSharingManager> for ::win32_system::Com::IDispatch {
    fn from(value: INetSharingManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetSharingManager> for ::win32_system::Com::IDispatch {
    fn from(value: &INetSharingManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for INetSharingManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a INetSharingManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for INetSharingManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for INetSharingManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for INetSharingManager {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for INetSharingManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetSharingManager").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for INetSharingManager {
    type Vtable = INetSharingManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc08956b7_1cd3_11d1_b1c5_00805fc1270e);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingManager_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub SharingInstalled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbinstalled: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub get_EnumPublicConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    get_EnumPublicConnections: usize,
    #[cfg(feature = "win32-system")]
    pub get_EnumPrivateConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    get_EnumPrivateConnections: usize,
    #[cfg(feature = "win32-system")]
    pub get_INetSharingConfigurationForINetConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnetconnection: ::windows_core::RawPtr, ppnetsharingconfiguration: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    get_INetSharingConfigurationForINetConnection: usize,
    #[cfg(feature = "win32-system")]
    pub EnumEveryConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcoll: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    EnumEveryConnection: usize,
    #[cfg(feature = "win32-system")]
    pub get_NetConnectionProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnetconnection: ::windows_core::RawPtr, ppprops: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    get_NetConnectionProps: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct INetSharingPortMapping(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl INetSharingPortMapping {
    pub unsafe fn Disable(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Disable)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Enable(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Enable)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Properties(&self) -> ::windows_core::Result<INetSharingPortMappingProps> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Properties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INetSharingPortMappingProps>(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetSharingPortMapping> for ::windows_core::IUnknown {
    fn from(value: INetSharingPortMapping) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetSharingPortMapping> for ::windows_core::IUnknown {
    fn from(value: &INetSharingPortMapping) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INetSharingPortMapping {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INetSharingPortMapping {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetSharingPortMapping> for ::win32_system::Com::IDispatch {
    fn from(value: INetSharingPortMapping) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetSharingPortMapping> for ::win32_system::Com::IDispatch {
    fn from(value: &INetSharingPortMapping) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for INetSharingPortMapping {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a INetSharingPortMapping {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for INetSharingPortMapping {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for INetSharingPortMapping {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for INetSharingPortMapping {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for INetSharingPortMapping {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetSharingPortMapping").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for INetSharingPortMapping {
    type Vtable = INetSharingPortMapping_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc08956b1_1cd3_11d1_b1c5_00805fc1270e);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingPortMapping_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Disable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Enable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnspmp: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Properties: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct INetSharingPortMappingCollection(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl INetSharingPortMappingCollection {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetSharingPortMappingCollection> for ::windows_core::IUnknown {
    fn from(value: INetSharingPortMappingCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetSharingPortMappingCollection> for ::windows_core::IUnknown {
    fn from(value: &INetSharingPortMappingCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INetSharingPortMappingCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INetSharingPortMappingCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetSharingPortMappingCollection> for ::win32_system::Com::IDispatch {
    fn from(value: INetSharingPortMappingCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetSharingPortMappingCollection> for ::win32_system::Com::IDispatch {
    fn from(value: &INetSharingPortMappingCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for INetSharingPortMappingCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a INetSharingPortMappingCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for INetSharingPortMappingCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for INetSharingPortMappingCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for INetSharingPortMappingCollection {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for INetSharingPortMappingCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetSharingPortMappingCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for INetSharingPortMappingCollection {
    type Vtable = INetSharingPortMappingCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x02e4a2de_da20_4e34_89c8_ac22275a010b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingPortMappingCollection_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct INetSharingPortMappingProps(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl INetSharingPortMappingProps {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn IPProtocol(&self) -> ::windows_core::Result<u8> {
        let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
        (::windows_core::Interface::vtable(self).IPProtocol)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u8>(result__)
    }
    pub unsafe fn ExternalPort(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).ExternalPort)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn InternalPort(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).InternalPort)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn Options(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Options)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn TargetName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).TargetName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn TargetIPAddress(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).TargetIPAddress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).Enabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetSharingPortMappingProps> for ::windows_core::IUnknown {
    fn from(value: INetSharingPortMappingProps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetSharingPortMappingProps> for ::windows_core::IUnknown {
    fn from(value: &INetSharingPortMappingProps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INetSharingPortMappingProps {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INetSharingPortMappingProps {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetSharingPortMappingProps> for ::win32_system::Com::IDispatch {
    fn from(value: INetSharingPortMappingProps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetSharingPortMappingProps> for ::win32_system::Com::IDispatch {
    fn from(value: &INetSharingPortMappingProps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for INetSharingPortMappingProps {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a INetSharingPortMappingProps {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for INetSharingPortMappingProps {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for INetSharingPortMappingProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for INetSharingPortMappingProps {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for INetSharingPortMappingProps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetSharingPortMappingProps").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for INetSharingPortMappingProps {
    type Vtable = INetSharingPortMappingProps_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x24b7e9b5_e38f_4685_851b_00892cf5f940);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingPortMappingProps_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub IPProtocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pucipprot: *mut u8) -> ::windows_core::HRESULT,
    pub ExternalPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pusport: *mut i32) -> ::windows_core::HRESULT,
    pub InternalPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pusport: *mut i32) -> ::windows_core::HRESULT,
    pub Options: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwoptions: *mut i32) -> ::windows_core::HRESULT,
    pub TargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtargetname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub TargetIPAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtargetipaddress: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbool: *mut i16) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct INetSharingPrivateConnectionCollection(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl INetSharingPrivateConnectionCollection {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetSharingPrivateConnectionCollection> for ::windows_core::IUnknown {
    fn from(value: INetSharingPrivateConnectionCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetSharingPrivateConnectionCollection> for ::windows_core::IUnknown {
    fn from(value: &INetSharingPrivateConnectionCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INetSharingPrivateConnectionCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INetSharingPrivateConnectionCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetSharingPrivateConnectionCollection> for ::win32_system::Com::IDispatch {
    fn from(value: INetSharingPrivateConnectionCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetSharingPrivateConnectionCollection> for ::win32_system::Com::IDispatch {
    fn from(value: &INetSharingPrivateConnectionCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for INetSharingPrivateConnectionCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a INetSharingPrivateConnectionCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for INetSharingPrivateConnectionCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for INetSharingPrivateConnectionCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for INetSharingPrivateConnectionCollection {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for INetSharingPrivateConnectionCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetSharingPrivateConnectionCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for INetSharingPrivateConnectionCollection {
    type Vtable = INetSharingPrivateConnectionCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x38ae69e0_4409_402a_a2cb_e965c727f840);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingPrivateConnectionCollection_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct INetSharingPublicConnectionCollection(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl INetSharingPublicConnectionCollection {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetSharingPublicConnectionCollection> for ::windows_core::IUnknown {
    fn from(value: INetSharingPublicConnectionCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetSharingPublicConnectionCollection> for ::windows_core::IUnknown {
    fn from(value: &INetSharingPublicConnectionCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INetSharingPublicConnectionCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INetSharingPublicConnectionCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<INetSharingPublicConnectionCollection> for ::win32_system::Com::IDispatch {
    fn from(value: INetSharingPublicConnectionCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&INetSharingPublicConnectionCollection> for ::win32_system::Com::IDispatch {
    fn from(value: &INetSharingPublicConnectionCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for INetSharingPublicConnectionCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a INetSharingPublicConnectionCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for INetSharingPublicConnectionCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for INetSharingPublicConnectionCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for INetSharingPublicConnectionCollection {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for INetSharingPublicConnectionCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetSharingPublicConnectionCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for INetSharingPublicConnectionCollection {
    type Vtable = INetSharingPublicConnectionCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d7a6355_f372_4971_a149_bfc927be762a);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingPublicConnectionCollection_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IStaticPortMapping(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IStaticPortMapping {
    pub unsafe fn ExternalIPAddress(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ExternalIPAddress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ExternalPort(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).ExternalPort)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn InternalPort(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).InternalPort)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn Protocol(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Protocol)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn InternalClient(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).InternalClient)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).Enabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn EditInternalClient<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrinternalclient: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EditInternalClient)(::windows_core::Interface::as_raw(self), bstrinternalclient.into_param().abi()).ok()
    }
    pub unsafe fn Enable(&self, vb: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Enable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vb)).ok()
    }
    pub unsafe fn EditDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdescription: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EditDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn EditInternalPort(&self, linternalport: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EditInternalPort)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(linternalport)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IStaticPortMapping> for ::windows_core::IUnknown {
    fn from(value: IStaticPortMapping) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IStaticPortMapping> for ::windows_core::IUnknown {
    fn from(value: &IStaticPortMapping) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IStaticPortMapping {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IStaticPortMapping {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IStaticPortMapping> for ::win32_system::Com::IDispatch {
    fn from(value: IStaticPortMapping) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IStaticPortMapping> for ::win32_system::Com::IDispatch {
    fn from(value: &IStaticPortMapping) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IStaticPortMapping {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IStaticPortMapping {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IStaticPortMapping {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IStaticPortMapping {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IStaticPortMapping {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IStaticPortMapping {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStaticPortMapping").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IStaticPortMapping {
    type Vtable = IStaticPortMapping_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6f10711f_729b_41e5_93b8_f21d0f818df1);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IStaticPortMapping_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub ExternalIPAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub ExternalPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub InternalPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub Protocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub InternalClient: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub EditInternalClient: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrinternalclient: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Enable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vb: i16) -> ::windows_core::HRESULT,
    pub EditDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub EditInternalPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linternalport: i32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IStaticPortMappingCollection(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IStaticPortMappingCollection {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn get_Item<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, lexternalport: i32, bstrprotocol: Param1) -> ::windows_core::Result<IStaticPortMapping> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lexternalport), bstrprotocol.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStaticPortMapping>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn Remove<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, lexternalport: i32, bstrprotocol: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lexternalport), bstrprotocol.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Add<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param5: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, lexternalport: i32, bstrprotocol: Param1, linternalport: i32, bstrinternalclient: Param3, benabled: i16, bstrdescription: Param5) -> ::windows_core::Result<IStaticPortMapping> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lexternalport), bstrprotocol.into_param().abi(), ::core::mem::transmute(linternalport), bstrinternalclient.into_param().abi(), ::core::mem::transmute(benabled), bstrdescription.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStaticPortMapping>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IStaticPortMappingCollection> for ::windows_core::IUnknown {
    fn from(value: IStaticPortMappingCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IStaticPortMappingCollection> for ::windows_core::IUnknown {
    fn from(value: &IStaticPortMappingCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IStaticPortMappingCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IStaticPortMappingCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IStaticPortMappingCollection> for ::win32_system::Com::IDispatch {
    fn from(value: IStaticPortMappingCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IStaticPortMappingCollection> for ::win32_system::Com::IDispatch {
    fn from(value: &IStaticPortMappingCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IStaticPortMappingCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IStaticPortMappingCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IStaticPortMappingCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IStaticPortMappingCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IStaticPortMappingCollection {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IStaticPortMappingCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStaticPortMappingCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IStaticPortMappingCollection {
    type Vtable = IStaticPortMappingCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcd1f3e77_66d6_4664_82c7_36dbb641d0f1);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IStaticPortMappingCollection_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppspm: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, linternalport: i32, bstrinternalclient: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, benabled: i16, bstrdescription: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppspm: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Add: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IUPnPNAT(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IUPnPNAT {
    #[cfg(feature = "win32-system")]
    pub unsafe fn StaticPortMappingCollection(&self) -> ::windows_core::Result<IStaticPortMappingCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).StaticPortMappingCollection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStaticPortMappingCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn DynamicPortMappingCollection(&self) -> ::windows_core::Result<IDynamicPortMappingCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).DynamicPortMappingCollection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDynamicPortMappingCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn NATEventManager(&self) -> ::windows_core::Result<INATEventManager> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).NATEventManager)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INATEventManager>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IUPnPNAT> for ::windows_core::IUnknown {
    fn from(value: IUPnPNAT) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IUPnPNAT> for ::windows_core::IUnknown {
    fn from(value: &IUPnPNAT) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUPnPNAT {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUPnPNAT {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IUPnPNAT> for ::win32_system::Com::IDispatch {
    fn from(value: IUPnPNAT) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IUPnPNAT> for ::win32_system::Com::IDispatch {
    fn from(value: &IUPnPNAT) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IUPnPNAT {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IUPnPNAT {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IUPnPNAT {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IUPnPNAT {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IUPnPNAT {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IUPnPNAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPNAT").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IUPnPNAT {
    type Vtable = IUPnPNAT_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb171c812_cc76_485a_94d8_b6b3a2794e99);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPNAT_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub StaticPortMappingCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppspms: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    StaticPortMappingCollection: usize,
    #[cfg(feature = "win32-system")]
    pub DynamicPortMappingCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdpms: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    DynamicPortMappingCollection: usize,
    #[cfg(feature = "win32-system")]
    pub NATEventManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnem: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    NATEventManager: usize,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NETCONMGR_ENUM_FLAGS(pub i32);
pub const NCME_DEFAULT: NETCONMGR_ENUM_FLAGS = NETCONMGR_ENUM_FLAGS(0i32);
pub const NCME_HIDDEN: NETCONMGR_ENUM_FLAGS = NETCONMGR_ENUM_FLAGS(1i32);
impl ::core::marker::Copy for NETCONMGR_ENUM_FLAGS {}
impl ::core::clone::Clone for NETCONMGR_ENUM_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NETCONMGR_ENUM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NETCONMGR_ENUM_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for NETCONMGR_ENUM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NETCONMGR_ENUM_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NETCONUI_CONNECT_FLAGS(pub i32);
pub const NCUC_DEFAULT: NETCONUI_CONNECT_FLAGS = NETCONUI_CONNECT_FLAGS(0i32);
pub const NCUC_NO_UI: NETCONUI_CONNECT_FLAGS = NETCONUI_CONNECT_FLAGS(1i32);
pub const NCUC_ENABLE_DISABLE: NETCONUI_CONNECT_FLAGS = NETCONUI_CONNECT_FLAGS(2i32);
impl ::core::marker::Copy for NETCONUI_CONNECT_FLAGS {}
impl ::core::clone::Clone for NETCONUI_CONNECT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NETCONUI_CONNECT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NETCONUI_CONNECT_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for NETCONUI_CONNECT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NETCONUI_CONNECT_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NETCON_CHARACTERISTIC_FLAGS(pub i32);
pub const NCCF_NONE: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(0i32);
pub const NCCF_ALL_USERS: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(1i32);
pub const NCCF_ALLOW_DUPLICATION: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(2i32);
pub const NCCF_ALLOW_REMOVAL: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(4i32);
pub const NCCF_ALLOW_RENAME: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(8i32);
pub const NCCF_INCOMING_ONLY: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(32i32);
pub const NCCF_OUTGOING_ONLY: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(64i32);
pub const NCCF_BRANDED: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(128i32);
pub const NCCF_SHARED: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(256i32);
pub const NCCF_BRIDGED: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(512i32);
pub const NCCF_FIREWALLED: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(1024i32);
pub const NCCF_DEFAULT: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(2048i32);
pub const NCCF_HOMENET_CAPABLE: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(4096i32);
pub const NCCF_SHARED_PRIVATE: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(8192i32);
pub const NCCF_QUARANTINED: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(16384i32);
pub const NCCF_RESERVED: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(32768i32);
pub const NCCF_HOSTED_NETWORK: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(65536i32);
pub const NCCF_VIRTUAL_STATION: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(131072i32);
pub const NCCF_WIFI_DIRECT: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(262144i32);
pub const NCCF_BLUETOOTH_MASK: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(983040i32);
pub const NCCF_LAN_MASK: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(15728640i32);
impl ::core::marker::Copy for NETCON_CHARACTERISTIC_FLAGS {}
impl ::core::clone::Clone for NETCON_CHARACTERISTIC_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NETCON_CHARACTERISTIC_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NETCON_CHARACTERISTIC_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for NETCON_CHARACTERISTIC_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NETCON_CHARACTERISTIC_FLAGS").field(&self.0).finish()
    }
}
pub const NETCON_MAX_NAME_LEN: u32 = 256u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NETCON_MEDIATYPE(pub i32);
pub const NCM_NONE: NETCON_MEDIATYPE = NETCON_MEDIATYPE(0i32);
pub const NCM_DIRECT: NETCON_MEDIATYPE = NETCON_MEDIATYPE(1i32);
pub const NCM_ISDN: NETCON_MEDIATYPE = NETCON_MEDIATYPE(2i32);
pub const NCM_LAN: NETCON_MEDIATYPE = NETCON_MEDIATYPE(3i32);
pub const NCM_PHONE: NETCON_MEDIATYPE = NETCON_MEDIATYPE(4i32);
pub const NCM_TUNNEL: NETCON_MEDIATYPE = NETCON_MEDIATYPE(5i32);
pub const NCM_PPPOE: NETCON_MEDIATYPE = NETCON_MEDIATYPE(6i32);
pub const NCM_BRIDGE: NETCON_MEDIATYPE = NETCON_MEDIATYPE(7i32);
pub const NCM_SHAREDACCESSHOST_LAN: NETCON_MEDIATYPE = NETCON_MEDIATYPE(8i32);
pub const NCM_SHAREDACCESSHOST_RAS: NETCON_MEDIATYPE = NETCON_MEDIATYPE(9i32);
impl ::core::marker::Copy for NETCON_MEDIATYPE {}
impl ::core::clone::Clone for NETCON_MEDIATYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NETCON_MEDIATYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NETCON_MEDIATYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for NETCON_MEDIATYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NETCON_MEDIATYPE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct NETCON_PROPERTIES {
    pub guidId: ::windows_core::GUID,
    pub pszwName: ::windows_core::PWSTR,
    pub pszwDeviceName: ::windows_core::PWSTR,
    pub Status: NETCON_STATUS,
    pub MediaType: NETCON_MEDIATYPE,
    pub dwCharacter: u32,
    pub clsidThisObject: ::windows_core::GUID,
    pub clsidUiObject: ::windows_core::GUID,
}
impl ::core::marker::Copy for NETCON_PROPERTIES {}
impl ::core::clone::Clone for NETCON_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NETCON_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETCON_PROPERTIES").field("guidId", &self.guidId).field("pszwName", &self.pszwName).field("pszwDeviceName", &self.pszwDeviceName).field("Status", &self.Status).field("MediaType", &self.MediaType).field("dwCharacter", &self.dwCharacter).field("clsidThisObject", &self.clsidThisObject).field("clsidUiObject", &self.clsidUiObject).finish()
    }
}
unsafe impl ::windows_core::Abi for NETCON_PROPERTIES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NETCON_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NETCON_PROPERTIES>()) == 0 }
    }
}
impl ::core::cmp::Eq for NETCON_PROPERTIES {}
impl ::core::default::Default for NETCON_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NETCON_STATUS(pub i32);
pub const NCS_DISCONNECTED: NETCON_STATUS = NETCON_STATUS(0i32);
pub const NCS_CONNECTING: NETCON_STATUS = NETCON_STATUS(1i32);
pub const NCS_CONNECTED: NETCON_STATUS = NETCON_STATUS(2i32);
pub const NCS_DISCONNECTING: NETCON_STATUS = NETCON_STATUS(3i32);
pub const NCS_HARDWARE_NOT_PRESENT: NETCON_STATUS = NETCON_STATUS(4i32);
pub const NCS_HARDWARE_DISABLED: NETCON_STATUS = NETCON_STATUS(5i32);
pub const NCS_HARDWARE_MALFUNCTION: NETCON_STATUS = NETCON_STATUS(6i32);
pub const NCS_MEDIA_DISCONNECTED: NETCON_STATUS = NETCON_STATUS(7i32);
pub const NCS_AUTHENTICATING: NETCON_STATUS = NETCON_STATUS(8i32);
pub const NCS_AUTHENTICATION_SUCCEEDED: NETCON_STATUS = NETCON_STATUS(9i32);
pub const NCS_AUTHENTICATION_FAILED: NETCON_STATUS = NETCON_STATUS(10i32);
pub const NCS_INVALID_ADDRESS: NETCON_STATUS = NETCON_STATUS(11i32);
pub const NCS_CREDENTIALS_REQUIRED: NETCON_STATUS = NETCON_STATUS(12i32);
pub const NCS_ACTION_REQUIRED: NETCON_STATUS = NETCON_STATUS(13i32);
pub const NCS_ACTION_REQUIRED_RETRY: NETCON_STATUS = NETCON_STATUS(14i32);
pub const NCS_CONNECT_FAILED: NETCON_STATUS = NETCON_STATUS(15i32);
impl ::core::marker::Copy for NETCON_STATUS {}
impl ::core::clone::Clone for NETCON_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NETCON_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NETCON_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for NETCON_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NETCON_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NETCON_TYPE(pub i32);
pub const NCT_DIRECT_CONNECT: NETCON_TYPE = NETCON_TYPE(0i32);
pub const NCT_INBOUND: NETCON_TYPE = NETCON_TYPE(1i32);
pub const NCT_INTERNET: NETCON_TYPE = NETCON_TYPE(2i32);
pub const NCT_LAN: NETCON_TYPE = NETCON_TYPE(3i32);
pub const NCT_PHONE: NETCON_TYPE = NETCON_TYPE(4i32);
pub const NCT_TUNNEL: NETCON_TYPE = NETCON_TYPE(5i32);
pub const NCT_BRIDGE: NETCON_TYPE = NETCON_TYPE(6i32);
impl ::core::marker::Copy for NETCON_TYPE {}
impl ::core::clone::Clone for NETCON_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NETCON_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NETCON_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for NETCON_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NETCON_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NETISO_ERROR_TYPE(pub i32);
pub const NETISO_ERROR_TYPE_NONE: NETISO_ERROR_TYPE = NETISO_ERROR_TYPE(0i32);
pub const NETISO_ERROR_TYPE_PRIVATE_NETWORK: NETISO_ERROR_TYPE = NETISO_ERROR_TYPE(1i32);
pub const NETISO_ERROR_TYPE_INTERNET_CLIENT: NETISO_ERROR_TYPE = NETISO_ERROR_TYPE(2i32);
pub const NETISO_ERROR_TYPE_INTERNET_CLIENT_SERVER: NETISO_ERROR_TYPE = NETISO_ERROR_TYPE(3i32);
pub const NETISO_ERROR_TYPE_MAX: NETISO_ERROR_TYPE = NETISO_ERROR_TYPE(4i32);
impl ::core::marker::Copy for NETISO_ERROR_TYPE {}
impl ::core::clone::Clone for NETISO_ERROR_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NETISO_ERROR_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NETISO_ERROR_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for NETISO_ERROR_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NETISO_ERROR_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NETISO_FLAG(pub i32);
pub const NETISO_FLAG_FORCE_COMPUTE_BINARIES: NETISO_FLAG = NETISO_FLAG(1i32);
pub const NETISO_FLAG_MAX: NETISO_FLAG = NETISO_FLAG(2i32);
impl ::core::marker::Copy for NETISO_FLAG {}
impl ::core::clone::Clone for NETISO_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NETISO_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NETISO_FLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for NETISO_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NETISO_FLAG").field(&self.0).finish()
    }
}
pub const NETISO_GEID_FOR_NEUTRAL_AWARE: u32 = 2u32;
pub const NETISO_GEID_FOR_WDAG: u32 = 1u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NET_FW_ACTION(pub i32);
pub const NET_FW_ACTION_BLOCK: NET_FW_ACTION = NET_FW_ACTION(0i32);
pub const NET_FW_ACTION_ALLOW: NET_FW_ACTION = NET_FW_ACTION(1i32);
pub const NET_FW_ACTION_MAX: NET_FW_ACTION = NET_FW_ACTION(2i32);
impl ::core::marker::Copy for NET_FW_ACTION {}
impl ::core::clone::Clone for NET_FW_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_FW_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NET_FW_ACTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for NET_FW_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_FW_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NET_FW_AUTHENTICATE_TYPE(pub i32);
pub const NET_FW_AUTHENTICATE_NONE: NET_FW_AUTHENTICATE_TYPE = NET_FW_AUTHENTICATE_TYPE(0i32);
pub const NET_FW_AUTHENTICATE_NO_ENCAPSULATION: NET_FW_AUTHENTICATE_TYPE = NET_FW_AUTHENTICATE_TYPE(1i32);
pub const NET_FW_AUTHENTICATE_WITH_INTEGRITY: NET_FW_AUTHENTICATE_TYPE = NET_FW_AUTHENTICATE_TYPE(2i32);
pub const NET_FW_AUTHENTICATE_AND_NEGOTIATE_ENCRYPTION: NET_FW_AUTHENTICATE_TYPE = NET_FW_AUTHENTICATE_TYPE(3i32);
pub const NET_FW_AUTHENTICATE_AND_ENCRYPT: NET_FW_AUTHENTICATE_TYPE = NET_FW_AUTHENTICATE_TYPE(4i32);
impl ::core::marker::Copy for NET_FW_AUTHENTICATE_TYPE {}
impl ::core::clone::Clone for NET_FW_AUTHENTICATE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_FW_AUTHENTICATE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NET_FW_AUTHENTICATE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for NET_FW_AUTHENTICATE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_FW_AUTHENTICATE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NET_FW_EDGE_TRAVERSAL_TYPE(pub i32);
pub const NET_FW_EDGE_TRAVERSAL_TYPE_DENY: NET_FW_EDGE_TRAVERSAL_TYPE = NET_FW_EDGE_TRAVERSAL_TYPE(0i32);
pub const NET_FW_EDGE_TRAVERSAL_TYPE_ALLOW: NET_FW_EDGE_TRAVERSAL_TYPE = NET_FW_EDGE_TRAVERSAL_TYPE(1i32);
pub const NET_FW_EDGE_TRAVERSAL_TYPE_DEFER_TO_APP: NET_FW_EDGE_TRAVERSAL_TYPE = NET_FW_EDGE_TRAVERSAL_TYPE(2i32);
pub const NET_FW_EDGE_TRAVERSAL_TYPE_DEFER_TO_USER: NET_FW_EDGE_TRAVERSAL_TYPE = NET_FW_EDGE_TRAVERSAL_TYPE(3i32);
impl ::core::marker::Copy for NET_FW_EDGE_TRAVERSAL_TYPE {}
impl ::core::clone::Clone for NET_FW_EDGE_TRAVERSAL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_FW_EDGE_TRAVERSAL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NET_FW_EDGE_TRAVERSAL_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for NET_FW_EDGE_TRAVERSAL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_FW_EDGE_TRAVERSAL_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NET_FW_IP_PROTOCOL(pub i32);
pub const NET_FW_IP_PROTOCOL_TCP: NET_FW_IP_PROTOCOL = NET_FW_IP_PROTOCOL(6i32);
pub const NET_FW_IP_PROTOCOL_UDP: NET_FW_IP_PROTOCOL = NET_FW_IP_PROTOCOL(17i32);
pub const NET_FW_IP_PROTOCOL_ANY: NET_FW_IP_PROTOCOL = NET_FW_IP_PROTOCOL(256i32);
impl ::core::marker::Copy for NET_FW_IP_PROTOCOL {}
impl ::core::clone::Clone for NET_FW_IP_PROTOCOL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_FW_IP_PROTOCOL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NET_FW_IP_PROTOCOL {
    type Abi = Self;
}
impl ::core::fmt::Debug for NET_FW_IP_PROTOCOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_FW_IP_PROTOCOL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NET_FW_IP_VERSION(pub i32);
pub const NET_FW_IP_VERSION_V4: NET_FW_IP_VERSION = NET_FW_IP_VERSION(0i32);
pub const NET_FW_IP_VERSION_V6: NET_FW_IP_VERSION = NET_FW_IP_VERSION(1i32);
pub const NET_FW_IP_VERSION_ANY: NET_FW_IP_VERSION = NET_FW_IP_VERSION(2i32);
pub const NET_FW_IP_VERSION_MAX: NET_FW_IP_VERSION = NET_FW_IP_VERSION(3i32);
impl ::core::marker::Copy for NET_FW_IP_VERSION {}
impl ::core::clone::Clone for NET_FW_IP_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_FW_IP_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NET_FW_IP_VERSION {
    type Abi = Self;
}
impl ::core::fmt::Debug for NET_FW_IP_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_FW_IP_VERSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NET_FW_MODIFY_STATE(pub i32);
pub const NET_FW_MODIFY_STATE_OK: NET_FW_MODIFY_STATE = NET_FW_MODIFY_STATE(0i32);
pub const NET_FW_MODIFY_STATE_GP_OVERRIDE: NET_FW_MODIFY_STATE = NET_FW_MODIFY_STATE(1i32);
pub const NET_FW_MODIFY_STATE_INBOUND_BLOCKED: NET_FW_MODIFY_STATE = NET_FW_MODIFY_STATE(2i32);
impl ::core::marker::Copy for NET_FW_MODIFY_STATE {}
impl ::core::clone::Clone for NET_FW_MODIFY_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_FW_MODIFY_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NET_FW_MODIFY_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for NET_FW_MODIFY_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_FW_MODIFY_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NET_FW_POLICY_TYPE(pub i32);
pub const NET_FW_POLICY_GROUP: NET_FW_POLICY_TYPE = NET_FW_POLICY_TYPE(0i32);
pub const NET_FW_POLICY_LOCAL: NET_FW_POLICY_TYPE = NET_FW_POLICY_TYPE(1i32);
pub const NET_FW_POLICY_EFFECTIVE: NET_FW_POLICY_TYPE = NET_FW_POLICY_TYPE(2i32);
pub const NET_FW_POLICY_TYPE_MAX: NET_FW_POLICY_TYPE = NET_FW_POLICY_TYPE(3i32);
impl ::core::marker::Copy for NET_FW_POLICY_TYPE {}
impl ::core::clone::Clone for NET_FW_POLICY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_FW_POLICY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NET_FW_POLICY_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for NET_FW_POLICY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_FW_POLICY_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NET_FW_PROFILE_TYPE(pub i32);
pub const NET_FW_PROFILE_DOMAIN: NET_FW_PROFILE_TYPE = NET_FW_PROFILE_TYPE(0i32);
pub const NET_FW_PROFILE_STANDARD: NET_FW_PROFILE_TYPE = NET_FW_PROFILE_TYPE(1i32);
pub const NET_FW_PROFILE_CURRENT: NET_FW_PROFILE_TYPE = NET_FW_PROFILE_TYPE(2i32);
pub const NET_FW_PROFILE_TYPE_MAX: NET_FW_PROFILE_TYPE = NET_FW_PROFILE_TYPE(3i32);
impl ::core::marker::Copy for NET_FW_PROFILE_TYPE {}
impl ::core::clone::Clone for NET_FW_PROFILE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_FW_PROFILE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NET_FW_PROFILE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for NET_FW_PROFILE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_FW_PROFILE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NET_FW_PROFILE_TYPE2(pub i32);
pub const NET_FW_PROFILE2_DOMAIN: NET_FW_PROFILE_TYPE2 = NET_FW_PROFILE_TYPE2(1i32);
pub const NET_FW_PROFILE2_PRIVATE: NET_FW_PROFILE_TYPE2 = NET_FW_PROFILE_TYPE2(2i32);
pub const NET_FW_PROFILE2_PUBLIC: NET_FW_PROFILE_TYPE2 = NET_FW_PROFILE_TYPE2(4i32);
pub const NET_FW_PROFILE2_ALL: NET_FW_PROFILE_TYPE2 = NET_FW_PROFILE_TYPE2(2147483647i32);
impl ::core::marker::Copy for NET_FW_PROFILE_TYPE2 {}
impl ::core::clone::Clone for NET_FW_PROFILE_TYPE2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_FW_PROFILE_TYPE2 {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NET_FW_PROFILE_TYPE2 {
    type Abi = Self;
}
impl ::core::fmt::Debug for NET_FW_PROFILE_TYPE2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_FW_PROFILE_TYPE2").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NET_FW_RULE_CATEGORY(pub i32);
pub const NET_FW_RULE_CATEGORY_BOOT: NET_FW_RULE_CATEGORY = NET_FW_RULE_CATEGORY(0i32);
pub const NET_FW_RULE_CATEGORY_STEALTH: NET_FW_RULE_CATEGORY = NET_FW_RULE_CATEGORY(1i32);
pub const NET_FW_RULE_CATEGORY_FIREWALL: NET_FW_RULE_CATEGORY = NET_FW_RULE_CATEGORY(2i32);
pub const NET_FW_RULE_CATEGORY_CONSEC: NET_FW_RULE_CATEGORY = NET_FW_RULE_CATEGORY(3i32);
pub const NET_FW_RULE_CATEGORY_MAX: NET_FW_RULE_CATEGORY = NET_FW_RULE_CATEGORY(4i32);
impl ::core::marker::Copy for NET_FW_RULE_CATEGORY {}
impl ::core::clone::Clone for NET_FW_RULE_CATEGORY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_FW_RULE_CATEGORY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NET_FW_RULE_CATEGORY {
    type Abi = Self;
}
impl ::core::fmt::Debug for NET_FW_RULE_CATEGORY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_FW_RULE_CATEGORY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NET_FW_RULE_DIRECTION(pub i32);
pub const NET_FW_RULE_DIR_IN: NET_FW_RULE_DIRECTION = NET_FW_RULE_DIRECTION(1i32);
pub const NET_FW_RULE_DIR_OUT: NET_FW_RULE_DIRECTION = NET_FW_RULE_DIRECTION(2i32);
pub const NET_FW_RULE_DIR_MAX: NET_FW_RULE_DIRECTION = NET_FW_RULE_DIRECTION(3i32);
impl ::core::marker::Copy for NET_FW_RULE_DIRECTION {}
impl ::core::clone::Clone for NET_FW_RULE_DIRECTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_FW_RULE_DIRECTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NET_FW_RULE_DIRECTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for NET_FW_RULE_DIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_FW_RULE_DIRECTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NET_FW_SCOPE(pub i32);
pub const NET_FW_SCOPE_ALL: NET_FW_SCOPE = NET_FW_SCOPE(0i32);
pub const NET_FW_SCOPE_LOCAL_SUBNET: NET_FW_SCOPE = NET_FW_SCOPE(1i32);
pub const NET_FW_SCOPE_CUSTOM: NET_FW_SCOPE = NET_FW_SCOPE(2i32);
pub const NET_FW_SCOPE_MAX: NET_FW_SCOPE = NET_FW_SCOPE(3i32);
impl ::core::marker::Copy for NET_FW_SCOPE {}
impl ::core::clone::Clone for NET_FW_SCOPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_FW_SCOPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NET_FW_SCOPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for NET_FW_SCOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_FW_SCOPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NET_FW_SERVICE_TYPE(pub i32);
pub const NET_FW_SERVICE_FILE_AND_PRINT: NET_FW_SERVICE_TYPE = NET_FW_SERVICE_TYPE(0i32);
pub const NET_FW_SERVICE_UPNP: NET_FW_SERVICE_TYPE = NET_FW_SERVICE_TYPE(1i32);
pub const NET_FW_SERVICE_REMOTE_DESKTOP: NET_FW_SERVICE_TYPE = NET_FW_SERVICE_TYPE(2i32);
pub const NET_FW_SERVICE_NONE: NET_FW_SERVICE_TYPE = NET_FW_SERVICE_TYPE(3i32);
pub const NET_FW_SERVICE_TYPE_MAX: NET_FW_SERVICE_TYPE = NET_FW_SERVICE_TYPE(4i32);
impl ::core::marker::Copy for NET_FW_SERVICE_TYPE {}
impl ::core::clone::Clone for NET_FW_SERVICE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_FW_SERVICE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NET_FW_SERVICE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for NET_FW_SERVICE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_FW_SERVICE_TYPE").field(&self.0).finish()
    }
}
pub const NetFwAuthorizedApplication: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xec9846b3_2762_4a6b_a214_6acb603462d2);
pub const NetFwMgr: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x304ce942_6e39_40d8_943a_b913c40c9cd4);
pub const NetFwOpenPort: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0ca545c6_37ad_4a6c_bf92_9f7610067ef5);
pub const NetFwPolicy2: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe2b3c97f_6ae1_41ac_817a_f6f92166d7dd);
pub const NetFwProduct: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9d745ed8_c514_4d1d_bf42_751fed2d5ac7);
pub const NetFwProducts: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcc19079b_8272_4d73_bb70_cdb533527b61);
pub const NetFwRule: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c5bc43e_3369_4c33_ab0c_be9469677af4);
pub const NetSharingManager: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5c63c1ad_3956_4ff8_8486_40034758315b);
#[inline]
pub unsafe fn NetworkIsolationDiagnoseConnectFailureAndGetInfo<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(wszservername: Param0, netisoerror: *mut NETISO_ERROR_TYPE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetworkIsolationDiagnoseConnectFailureAndGetInfo(wszservername: ::windows_core::PCWSTR, netisoerror: *mut NETISO_ERROR_TYPE) -> u32;
        }
        ::core::mem::transmute(NetworkIsolationDiagnoseConnectFailureAndGetInfo(wszservername.into_param().abi(), ::core::mem::transmute(netisoerror)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-security")]
#[inline]
pub unsafe fn NetworkIsolationEnumAppContainers(flags: u32, pdwnumpublicappcs: *mut u32, pppublicappcs: *mut *mut INET_FIREWALL_APP_CONTAINER) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetworkIsolationEnumAppContainers(flags: u32, pdwnumpublicappcs: *mut u32, pppublicappcs: *mut *mut INET_FIREWALL_APP_CONTAINER) -> u32;
        }
        ::core::mem::transmute(NetworkIsolationEnumAppContainers(::core::mem::transmute(flags), ::core::mem::transmute(pdwnumpublicappcs), ::core::mem::transmute(pppublicappcs)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-security")]
#[inline]
pub unsafe fn NetworkIsolationFreeAppContainers(ppublicappcs: *const INET_FIREWALL_APP_CONTAINER) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetworkIsolationFreeAppContainers(ppublicappcs: *const INET_FIREWALL_APP_CONTAINER) -> u32;
        }
        ::core::mem::transmute(NetworkIsolationFreeAppContainers(::core::mem::transmute(ppublicappcs)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-security")]
#[inline]
pub unsafe fn NetworkIsolationGetAppContainerConfig(pdwnumpublicappcs: *mut u32, appcontainersids: *mut *mut ::win32_security::SID_AND_ATTRIBUTES) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetworkIsolationGetAppContainerConfig(pdwnumpublicappcs: *mut u32, appcontainersids: *mut *mut ::win32_security::SID_AND_ATTRIBUTES) -> u32;
        }
        ::core::mem::transmute(NetworkIsolationGetAppContainerConfig(::core::mem::transmute(pdwnumpublicappcs), ::core::mem::transmute(appcontainersids)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-security")]
#[inline]
pub unsafe fn NetworkIsolationRegisterForAppContainerChanges(flags: u32, callback: PAC_CHANGES_CALLBACK_FN, context: *const ::core::ffi::c_void, registrationobject: *mut ::win32_foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetworkIsolationRegisterForAppContainerChanges(flags: u32, callback: ::windows_core::RawPtr, context: *const ::core::ffi::c_void, registrationobject: *mut ::win32_foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(NetworkIsolationRegisterForAppContainerChanges(::core::mem::transmute(flags), ::core::mem::transmute(callback), ::core::mem::transmute(context), ::core::mem::transmute(registrationobject)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-security")]
#[inline]
pub unsafe fn NetworkIsolationSetAppContainerConfig(appcontainersids: &[::win32_security::SID_AND_ATTRIBUTES]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetworkIsolationSetAppContainerConfig(dwnumpublicappcs: u32, appcontainersids: *const ::win32_security::SID_AND_ATTRIBUTES) -> u32;
        }
        ::core::mem::transmute(NetworkIsolationSetAppContainerConfig(appcontainersids.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(appcontainersids))))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn NetworkIsolationSetupAppContainerBinaries<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::PSID>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(applicationcontainersid: Param0, packagefullname: Param1, packagefolder: Param2, displayname: Param3, bbinariesfullycomputed: Param4, binaries: &[::windows_core::PWSTR]) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetworkIsolationSetupAppContainerBinaries(applicationcontainersid: ::win32_foundation::PSID, packagefullname: ::windows_core::PCWSTR, packagefolder: ::windows_core::PCWSTR, displayname: ::windows_core::PCWSTR, bbinariesfullycomputed: ::win32_foundation::BOOL, binaries: *const ::windows_core::PWSTR, binariescount: u32) -> ::windows_core::HRESULT;
        }
        NetworkIsolationSetupAppContainerBinaries(applicationcontainersid.into_param().abi(), packagefullname.into_param().abi(), packagefolder.into_param().abi(), displayname.into_param().abi(), bbinariesfullycomputed.into_param().abi(), ::core::mem::transmute(::windows_core::as_ptr_or_null(binaries)), binaries.len() as _).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn NetworkIsolationUnregisterForAppContainerChanges<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(registrationobject: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetworkIsolationUnregisterForAppContainerChanges(registrationobject: ::win32_foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(NetworkIsolationUnregisterForAppContainerChanges(registrationobject.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-security")]
pub type PAC_CHANGES_CALLBACK_FN = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, pchange: *const INET_FIREWALL_AC_CHANGE)>;
pub type PFN_FWADDDYNAMICKEYWORDADDRESS0 = ::core::option::Option<unsafe extern "system" fn(dynamickeywordaddress: *const _tag_FW_DYNAMIC_KEYWORD_ADDRESS0) -> u32>;
pub type PFN_FWDELETEDYNAMICKEYWORDADDRESS0 = ::core::option::Option<unsafe extern "system" fn(dynamickeywordaddressid: ::windows_core::GUID) -> u32>;
pub type PFN_FWENUMDYNAMICKEYWORDADDRESSBYID0 = ::core::option::Option<unsafe extern "system" fn(dynamickeywordaddressid: ::windows_core::GUID, dynamickeywordaddressdata: *mut *mut _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> u32>;
pub type PFN_FWENUMDYNAMICKEYWORDADDRESSESBYTYPE0 = ::core::option::Option<unsafe extern "system" fn(flags: u32, dynamickeywordaddressdata: *mut *mut _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> u32>;
pub type PFN_FWFREEDYNAMICKEYWORDADDRESSDATA0 = ::core::option::Option<unsafe extern "system" fn(dynamickeywordaddressdata: *const _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> u32>;
pub type PFN_FWUPDATEDYNAMICKEYWORDADDRESS0 = ::core::option::Option<unsafe extern "system" fn(dynamickeywordaddressid: ::windows_core::GUID, updatedaddresses: ::windows_core::PCWSTR, append: ::win32_foundation::BOOL) -> u32>;
pub type PNETISO_EDP_ID_CALLBACK_FN = ::core::option::Option<unsafe extern "system" fn(context: *mut ::core::ffi::c_void, wszenterpriseid: ::windows_core::PCWSTR, dwerr: u32)>;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SHARINGCONNECTIONTYPE(pub i32);
pub const ICSSHARINGTYPE_PUBLIC: SHARINGCONNECTIONTYPE = SHARINGCONNECTIONTYPE(0i32);
pub const ICSSHARINGTYPE_PRIVATE: SHARINGCONNECTIONTYPE = SHARINGCONNECTIONTYPE(1i32);
impl ::core::marker::Copy for SHARINGCONNECTIONTYPE {}
impl ::core::clone::Clone for SHARINGCONNECTIONTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SHARINGCONNECTIONTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SHARINGCONNECTIONTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SHARINGCONNECTIONTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHARINGCONNECTIONTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SHARINGCONNECTION_ENUM_FLAGS(pub i32);
pub const ICSSC_DEFAULT: SHARINGCONNECTION_ENUM_FLAGS = SHARINGCONNECTION_ENUM_FLAGS(0i32);
pub const ICSSC_ENABLED: SHARINGCONNECTION_ENUM_FLAGS = SHARINGCONNECTION_ENUM_FLAGS(1i32);
impl ::core::marker::Copy for SHARINGCONNECTION_ENUM_FLAGS {}
impl ::core::clone::Clone for SHARINGCONNECTION_ENUM_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SHARINGCONNECTION_ENUM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SHARINGCONNECTION_ENUM_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for SHARINGCONNECTION_ENUM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHARINGCONNECTION_ENUM_FLAGS").field(&self.0).finish()
    }
}
pub const S_OBJECT_NO_LONGER_VALID: ::windows_core::HRESULT = ::windows_core::HRESULT(2i32);
pub const UPnPNAT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xae1e00aa_3fd5_403c_8a27_2bbdc30cd0e1);
#[repr(C)]
pub struct _tag_FW_DYNAMIC_KEYWORD_ADDRESS0 {
    pub id: ::windows_core::GUID,
    pub keyword: ::windows_core::PCWSTR,
    pub flags: u32,
    pub addresses: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for _tag_FW_DYNAMIC_KEYWORD_ADDRESS0 {}
impl ::core::clone::Clone for _tag_FW_DYNAMIC_KEYWORD_ADDRESS0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for _tag_FW_DYNAMIC_KEYWORD_ADDRESS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_tag_FW_DYNAMIC_KEYWORD_ADDRESS0").field("id", &self.id).field("keyword", &self.keyword).field("flags", &self.flags).field("addresses", &self.addresses).finish()
    }
}
unsafe impl ::windows_core::Abi for _tag_FW_DYNAMIC_KEYWORD_ADDRESS0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for _tag_FW_DYNAMIC_KEYWORD_ADDRESS0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<_tag_FW_DYNAMIC_KEYWORD_ADDRESS0>()) == 0 }
    }
}
impl ::core::cmp::Eq for _tag_FW_DYNAMIC_KEYWORD_ADDRESS0 {}
impl ::core::default::Default for _tag_FW_DYNAMIC_KEYWORD_ADDRESS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    pub dynamicKeywordAddress: _tag_FW_DYNAMIC_KEYWORD_ADDRESS0,
    pub next: *mut _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0,
    pub schemaVersion: u16,
    pub originType: _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE,
}
impl ::core::marker::Copy for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {}
impl ::core::clone::Clone for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0").field("dynamicKeywordAddress", &self.dynamicKeywordAddress).field("next", &self.next).field("schemaVersion", &self.schemaVersion).field("originType", &self.originType).finish()
    }
}
unsafe impl ::windows_core::Abi for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<_tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0>()) == 0 }
    }
}
impl ::core::cmp::Eq for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {}
impl ::core::default::Default for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS(pub i32);
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_AUTO_RESOLVE: _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS(1i32);
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_NON_AUTO_RESOLVE: _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS(2i32);
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_ALL: _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS(3i32);
impl ::core::marker::Copy for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS {}
impl ::core::clone::Clone for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct _tag_FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS(pub i32);
pub const FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS_AUTO_RESOLVE: _tag_FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS = _tag_FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS(1i32);
impl ::core::marker::Copy for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS {}
impl ::core::clone::Clone for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_tag_FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE(pub i32);
pub const FW_DYNAMIC_KEYWORD_ORIGIN_INVALID: _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE(0i32);
pub const FW_DYNAMIC_KEYWORD_ORIGIN_LOCAL: _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE(1i32);
pub const FW_DYNAMIC_KEYWORD_ORIGIN_MDM: _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE(2i32);
impl ::core::marker::Copy for _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE {}
impl ::core::clone::Clone for _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE").field(&self.0).finish()
    }
}
