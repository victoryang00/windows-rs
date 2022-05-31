#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CONFLICT_RESOLUTION_POLICY(pub i32);
pub const CRP_NONE: CONFLICT_RESOLUTION_POLICY = CONFLICT_RESOLUTION_POLICY(0i32);
pub const CRP_DESTINATION_PROVIDER_WINS: CONFLICT_RESOLUTION_POLICY = CONFLICT_RESOLUTION_POLICY(1i32);
pub const CRP_SOURCE_PROVIDER_WINS: CONFLICT_RESOLUTION_POLICY = CONFLICT_RESOLUTION_POLICY(2i32);
pub const CRP_LAST: CONFLICT_RESOLUTION_POLICY = CONFLICT_RESOLUTION_POLICY(3i32);
impl ::core::marker::Copy for CONFLICT_RESOLUTION_POLICY {}
impl ::core::clone::Clone for CONFLICT_RESOLUTION_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CONFLICT_RESOLUTION_POLICY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CONFLICT_RESOLUTION_POLICY {
    type Abi = Self;
}
impl ::core::fmt::Debug for CONFLICT_RESOLUTION_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONFLICT_RESOLUTION_POLICY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CONSTRAINT_CONFLICT_REASON(pub i32);
pub const CCR_OTHER: CONSTRAINT_CONFLICT_REASON = CONSTRAINT_CONFLICT_REASON(0i32);
pub const CCR_COLLISION: CONSTRAINT_CONFLICT_REASON = CONSTRAINT_CONFLICT_REASON(1i32);
pub const CCR_NOPARENT: CONSTRAINT_CONFLICT_REASON = CONSTRAINT_CONFLICT_REASON(2i32);
pub const CCR_IDENTITY: CONSTRAINT_CONFLICT_REASON = CONSTRAINT_CONFLICT_REASON(3i32);
impl ::core::marker::Copy for CONSTRAINT_CONFLICT_REASON {}
impl ::core::clone::Clone for CONSTRAINT_CONFLICT_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CONSTRAINT_CONFLICT_REASON {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CONSTRAINT_CONFLICT_REASON {
    type Abi = Self;
}
impl ::core::fmt::Debug for CONSTRAINT_CONFLICT_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONSTRAINT_CONFLICT_REASON").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FILTERING_TYPE(pub i32);
pub const FT_CURRENT_ITEMS_ONLY: FILTERING_TYPE = FILTERING_TYPE(0i32);
pub const FT_CURRENT_ITEMS_AND_VERSIONS_FOR_MOVED_OUT_ITEMS: FILTERING_TYPE = FILTERING_TYPE(1i32);
impl ::core::marker::Copy for FILTERING_TYPE {}
impl ::core::clone::Clone for FILTERING_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILTERING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FILTERING_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FILTERING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILTERING_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FILTER_COMBINATION_TYPE(pub i32);
pub const FCT_INTERSECTION: FILTER_COMBINATION_TYPE = FILTER_COMBINATION_TYPE(0i32);
impl ::core::marker::Copy for FILTER_COMBINATION_TYPE {}
impl ::core::clone::Clone for FILTER_COMBINATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILTER_COMBINATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FILTER_COMBINATION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FILTER_COMBINATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILTER_COMBINATION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
pub struct IAsynchronousDataRetriever(::windows_core::IUnknown);
impl IAsynchronousDataRetriever {
    pub unsafe fn GetIdParameters(&self, pidparameters: *mut ID_PARAMETERS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetIdParameters)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pidparameters)).ok()
    }
    pub unsafe fn RegisterCallback<'a, Param0: ::windows_core::IntoParam<'a, IDataRetrieverCallback>>(&self, pdataretrievercallback: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterCallback)(::windows_core::Interface::as_raw(self), pdataretrievercallback.into_param().abi()).ok()
    }
    pub unsafe fn RevokeCallback<'a, Param0: ::windows_core::IntoParam<'a, IDataRetrieverCallback>>(&self, pdataretrievercallback: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RevokeCallback)(::windows_core::Interface::as_raw(self), pdataretrievercallback.into_param().abi()).ok()
    }
    pub unsafe fn LoadChangeData<'a, Param0: ::windows_core::IntoParam<'a, ILoadChangeContext>>(&self, ploadchangecontext: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LoadChangeData)(::windows_core::Interface::as_raw(self), ploadchangecontext.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IAsynchronousDataRetriever> for ::windows_core::IUnknown {
    fn from(value: IAsynchronousDataRetriever) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAsynchronousDataRetriever> for ::windows_core::IUnknown {
    fn from(value: &IAsynchronousDataRetriever) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAsynchronousDataRetriever {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAsynchronousDataRetriever {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAsynchronousDataRetriever {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAsynchronousDataRetriever {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAsynchronousDataRetriever {}
impl ::core::fmt::Debug for IAsynchronousDataRetriever {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAsynchronousDataRetriever").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAsynchronousDataRetriever {
    type Vtable = IAsynchronousDataRetriever_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9fc7e470_61ea_4a88_9be4_df56a27cfef2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsynchronousDataRetriever_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetIdParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> ::windows_core::HRESULT,
    pub RegisterCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataretrievercallback: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RevokeCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataretrievercallback: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LoadChangeData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ploadchangecontext: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IChangeConflict(::windows_core::IUnknown);
impl IChangeConflict {
    pub unsafe fn GetDestinationProviderConflictingChange(&self) -> ::windows_core::Result<ISyncChange> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetDestinationProviderConflictingChange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChange>(result__)
    }
    pub unsafe fn GetSourceProviderConflictingChange(&self) -> ::windows_core::Result<ISyncChange> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSourceProviderConflictingChange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChange>(result__)
    }
    pub unsafe fn GetDestinationProviderConflictingData(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).GetDestinationProviderConflictingData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn GetSourceProviderConflictingData(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).GetSourceProviderConflictingData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn GetResolveActionForChange(&self, presolveaction: *mut SYNC_RESOLVE_ACTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetResolveActionForChange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(presolveaction)).ok()
    }
    pub unsafe fn SetResolveActionForChange(&self, resolveaction: SYNC_RESOLVE_ACTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetResolveActionForChange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(resolveaction)).ok()
    }
    pub unsafe fn GetResolveActionForChangeUnit<'a, Param0: ::windows_core::IntoParam<'a, ISyncChangeUnit>>(&self, pchangeunit: Param0, presolveaction: *mut SYNC_RESOLVE_ACTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetResolveActionForChangeUnit)(::windows_core::Interface::as_raw(self), pchangeunit.into_param().abi(), ::core::mem::transmute(presolveaction)).ok()
    }
    pub unsafe fn SetResolveActionForChangeUnit<'a, Param0: ::windows_core::IntoParam<'a, ISyncChangeUnit>>(&self, pchangeunit: Param0, resolveaction: SYNC_RESOLVE_ACTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetResolveActionForChangeUnit)(::windows_core::Interface::as_raw(self), pchangeunit.into_param().abi(), ::core::mem::transmute(resolveaction)).ok()
    }
}
impl ::core::convert::From<IChangeConflict> for ::windows_core::IUnknown {
    fn from(value: IChangeConflict) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IChangeConflict> for ::windows_core::IUnknown {
    fn from(value: &IChangeConflict) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IChangeConflict {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IChangeConflict {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IChangeConflict {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IChangeConflict {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IChangeConflict {}
impl ::core::fmt::Debug for IChangeConflict {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IChangeConflict").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IChangeConflict {
    type Vtable = IChangeConflict_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x014ebf97_9f20_4f7a_bdd4_25979c77c002);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChangeConflict_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetDestinationProviderConflictingChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconflictingchange: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetSourceProviderConflictingChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconflictingchange: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDestinationProviderConflictingData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetSourceProviderConflictingData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetResolveActionForChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presolveaction: *mut SYNC_RESOLVE_ACTION) -> ::windows_core::HRESULT,
    pub SetResolveActionForChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resolveaction: SYNC_RESOLVE_ACTION) -> ::windows_core::HRESULT,
    pub GetResolveActionForChangeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchangeunit: ::windows_core::RawPtr, presolveaction: *mut SYNC_RESOLVE_ACTION) -> ::windows_core::HRESULT,
    pub SetResolveActionForChangeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchangeunit: ::windows_core::RawPtr, resolveaction: SYNC_RESOLVE_ACTION) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IChangeUnitException(::windows_core::IUnknown);
impl IChangeUnitException {
    pub unsafe fn GetItemId(&self, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetItemId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pcbidsize)).ok()
    }
    pub unsafe fn GetChangeUnitId(&self, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetChangeUnitId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbchangeunitid), ::core::mem::transmute(pcbidsize)).ok()
    }
    pub unsafe fn GetClockVector(&self, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetClockVector)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
}
impl ::core::convert::From<IChangeUnitException> for ::windows_core::IUnknown {
    fn from(value: IChangeUnitException) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IChangeUnitException> for ::windows_core::IUnknown {
    fn from(value: &IChangeUnitException) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IChangeUnitException {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IChangeUnitException {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IChangeUnitException {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IChangeUnitException {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IChangeUnitException {}
impl ::core::fmt::Debug for IChangeUnitException {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IChangeUnitException").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IChangeUnitException {
    type Vtable = IChangeUnitException_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0cd7ee7c_fec0_4021_99ee_f0e5348f2a5f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChangeUnitException_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetItemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT,
    pub GetChangeUnitId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT,
    pub GetClockVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IChangeUnitListFilterInfo(::windows_core::IUnknown);
impl IChangeUnitListFilterInfo {
    pub unsafe fn Serialize(&self, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Serialize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbbuffer), ::core::mem::transmute(pcbbuffer)).ok()
    }
    pub unsafe fn Initialize(&self, ppbchangeunitids: *const *const u8, dwchangeunitcount: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppbchangeunitids), ::core::mem::transmute(dwchangeunitcount)).ok()
    }
    pub unsafe fn GetChangeUnitIdCount(&self, pdwchangeunitidcount: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetChangeUnitIdCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwchangeunitidcount)).ok()
    }
    pub unsafe fn GetChangeUnitId(&self, dwchangeunitidindex: u32, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetChangeUnitId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwchangeunitidindex), ::core::mem::transmute(pbchangeunitid), ::core::mem::transmute(pcbidsize)).ok()
    }
}
impl ::core::convert::From<IChangeUnitListFilterInfo> for ::windows_core::IUnknown {
    fn from(value: IChangeUnitListFilterInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IChangeUnitListFilterInfo> for ::windows_core::IUnknown {
    fn from(value: &IChangeUnitListFilterInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IChangeUnitListFilterInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IChangeUnitListFilterInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IChangeUnitListFilterInfo> for ISyncFilterInfo {
    fn from(value: IChangeUnitListFilterInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IChangeUnitListFilterInfo> for ISyncFilterInfo {
    fn from(value: &IChangeUnitListFilterInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyncFilterInfo> for IChangeUnitListFilterInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ISyncFilterInfo> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyncFilterInfo> for &'a IChangeUnitListFilterInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ISyncFilterInfo> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IChangeUnitListFilterInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IChangeUnitListFilterInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IChangeUnitListFilterInfo {}
impl ::core::fmt::Debug for IChangeUnitListFilterInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IChangeUnitListFilterInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IChangeUnitListFilterInfo {
    type Vtable = IChangeUnitListFilterInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf2837671_0bdf_43fa_b502_232375fb50c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChangeUnitListFilterInfo_Vtbl {
    pub base__: ISyncFilterInfo_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbchangeunitids: *const *const u8, dwchangeunitcount: u32) -> ::windows_core::HRESULT,
    pub GetChangeUnitIdCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwchangeunitidcount: *mut u32) -> ::windows_core::HRESULT,
    pub GetChangeUnitId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwchangeunitidindex: u32, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IClockVector(::windows_core::IUnknown);
impl IClockVector {
    pub unsafe fn GetClockVectorElements(&self, riid: *const ::windows_core::GUID, ppienumclockvector: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetClockVectorElements)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppienumclockvector)).ok()
    }
    pub unsafe fn GetClockVectorElementCount(&self, pdwcount: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetClockVectorElementCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwcount)).ok()
    }
}
impl ::core::convert::From<IClockVector> for ::windows_core::IUnknown {
    fn from(value: IClockVector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IClockVector> for ::windows_core::IUnknown {
    fn from(value: &IClockVector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IClockVector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IClockVector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IClockVector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IClockVector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IClockVector {}
impl ::core::fmt::Debug for IClockVector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IClockVector").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IClockVector {
    type Vtable = IClockVector_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x14b2274a_8698_4cc6_9333_f89bd1d47bc4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClockVector_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetClockVectorElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppienumclockvector: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetClockVectorElementCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IClockVectorElement(::windows_core::IUnknown);
impl IClockVectorElement {
    pub unsafe fn GetReplicaKey(&self, pdwreplicakey: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetReplicaKey)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwreplicakey)).ok()
    }
    pub unsafe fn GetTickCount(&self, pulltickcount: *mut u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetTickCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pulltickcount)).ok()
    }
}
impl ::core::convert::From<IClockVectorElement> for ::windows_core::IUnknown {
    fn from(value: IClockVectorElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IClockVectorElement> for ::windows_core::IUnknown {
    fn from(value: &IClockVectorElement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IClockVectorElement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IClockVectorElement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IClockVectorElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IClockVectorElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IClockVectorElement {}
impl ::core::fmt::Debug for IClockVectorElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IClockVectorElement").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IClockVectorElement {
    type Vtable = IClockVectorElement_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe71c4250_adf8_4a07_8fae_5669596909c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClockVectorElement_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetReplicaKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwreplicakey: *mut u32) -> ::windows_core::HRESULT,
    pub GetTickCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulltickcount: *mut u64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICombinedFilterInfo(::windows_core::IUnknown);
impl ICombinedFilterInfo {
    pub unsafe fn Serialize(&self, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Serialize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbbuffer), ::core::mem::transmute(pcbbuffer)).ok()
    }
    pub unsafe fn GetFilterCount(&self, pdwfiltercount: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFilterCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwfiltercount)).ok()
    }
    pub unsafe fn GetFilterInfo(&self, dwfilterindex: u32) -> ::windows_core::Result<ISyncFilterInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetFilterInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwfilterindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncFilterInfo>(result__)
    }
    pub unsafe fn GetFilterCombinationType(&self, pfiltercombinationtype: *mut FILTER_COMBINATION_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFilterCombinationType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pfiltercombinationtype)).ok()
    }
}
impl ::core::convert::From<ICombinedFilterInfo> for ::windows_core::IUnknown {
    fn from(value: ICombinedFilterInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICombinedFilterInfo> for ::windows_core::IUnknown {
    fn from(value: &ICombinedFilterInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICombinedFilterInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICombinedFilterInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICombinedFilterInfo> for ISyncFilterInfo {
    fn from(value: ICombinedFilterInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICombinedFilterInfo> for ISyncFilterInfo {
    fn from(value: &ICombinedFilterInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyncFilterInfo> for ICombinedFilterInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ISyncFilterInfo> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyncFilterInfo> for &'a ICombinedFilterInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ISyncFilterInfo> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICombinedFilterInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICombinedFilterInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICombinedFilterInfo {}
impl ::core::fmt::Debug for ICombinedFilterInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICombinedFilterInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICombinedFilterInfo {
    type Vtable = ICombinedFilterInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x11f9de71_2818_4779_b2ac_42d450565f45);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICombinedFilterInfo_Vtbl {
    pub base__: ISyncFilterInfo_Vtbl,
    pub GetFilterCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwfiltercount: *mut u32) -> ::windows_core::HRESULT,
    pub GetFilterInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwfilterindex: u32, ppifilterinfo: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetFilterCombinationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfiltercombinationtype: *mut FILTER_COMBINATION_TYPE) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IConstraintConflict(::windows_core::IUnknown);
impl IConstraintConflict {
    pub unsafe fn GetDestinationProviderConflictingChange(&self) -> ::windows_core::Result<ISyncChange> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetDestinationProviderConflictingChange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChange>(result__)
    }
    pub unsafe fn GetSourceProviderConflictingChange(&self) -> ::windows_core::Result<ISyncChange> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSourceProviderConflictingChange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChange>(result__)
    }
    pub unsafe fn GetDestinationProviderOriginalChange(&self) -> ::windows_core::Result<ISyncChange> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetDestinationProviderOriginalChange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChange>(result__)
    }
    pub unsafe fn GetDestinationProviderConflictingData(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).GetDestinationProviderConflictingData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn GetSourceProviderConflictingData(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).GetSourceProviderConflictingData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn GetDestinationProviderOriginalData(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).GetDestinationProviderOriginalData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn GetConstraintResolveActionForChange(&self, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetConstraintResolveActionForChange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pconstraintresolveaction)).ok()
    }
    pub unsafe fn SetConstraintResolveActionForChange(&self, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetConstraintResolveActionForChange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(constraintresolveaction)).ok()
    }
    pub unsafe fn GetConstraintResolveActionForChangeUnit<'a, Param0: ::windows_core::IntoParam<'a, ISyncChangeUnit>>(&self, pchangeunit: Param0, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetConstraintResolveActionForChangeUnit)(::windows_core::Interface::as_raw(self), pchangeunit.into_param().abi(), ::core::mem::transmute(pconstraintresolveaction)).ok()
    }
    pub unsafe fn SetConstraintResolveActionForChangeUnit<'a, Param0: ::windows_core::IntoParam<'a, ISyncChangeUnit>>(&self, pchangeunit: Param0, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetConstraintResolveActionForChangeUnit)(::windows_core::Interface::as_raw(self), pchangeunit.into_param().abi(), ::core::mem::transmute(constraintresolveaction)).ok()
    }
    pub unsafe fn GetConstraintConflictReason(&self, pconstraintconflictreason: *mut CONSTRAINT_CONFLICT_REASON) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetConstraintConflictReason)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pconstraintconflictreason)).ok()
    }
    pub unsafe fn IsTemporary(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsTemporary)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IConstraintConflict> for ::windows_core::IUnknown {
    fn from(value: IConstraintConflict) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IConstraintConflict> for ::windows_core::IUnknown {
    fn from(value: &IConstraintConflict) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IConstraintConflict {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IConstraintConflict {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IConstraintConflict {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IConstraintConflict {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConstraintConflict {}
impl ::core::fmt::Debug for IConstraintConflict {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConstraintConflict").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IConstraintConflict {
    type Vtable = IConstraintConflict_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00d2302e_1cf8_4835_b85f_b7ca4f799e0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConstraintConflict_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetDestinationProviderConflictingChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconflictingchange: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetSourceProviderConflictingChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconflictingchange: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDestinationProviderOriginalChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pporiginalchange: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDestinationProviderConflictingData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetSourceProviderConflictingData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDestinationProviderOriginalData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pporiginaldata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetConstraintResolveActionForChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows_core::HRESULT,
    pub SetConstraintResolveActionForChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows_core::HRESULT,
    pub GetConstraintResolveActionForChangeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchangeunit: ::windows_core::RawPtr, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows_core::HRESULT,
    pub SetConstraintResolveActionForChangeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchangeunit: ::windows_core::RawPtr, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows_core::HRESULT,
    pub GetConstraintConflictReason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconstraintconflictreason: *mut CONSTRAINT_CONFLICT_REASON) -> ::windows_core::HRESULT,
    pub IsTemporary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IConstructReplicaKeyMap(::windows_core::IUnknown);
impl IConstructReplicaKeyMap {
    pub unsafe fn FindOrAddReplica(&self, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FindOrAddReplica)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbreplicaid), ::core::mem::transmute(pdwreplicakey)).ok()
    }
}
impl ::core::convert::From<IConstructReplicaKeyMap> for ::windows_core::IUnknown {
    fn from(value: IConstructReplicaKeyMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IConstructReplicaKeyMap> for ::windows_core::IUnknown {
    fn from(value: &IConstructReplicaKeyMap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IConstructReplicaKeyMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IConstructReplicaKeyMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IConstructReplicaKeyMap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IConstructReplicaKeyMap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConstructReplicaKeyMap {}
impl ::core::fmt::Debug for IConstructReplicaKeyMap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConstructReplicaKeyMap").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IConstructReplicaKeyMap {
    type Vtable = IConstructReplicaKeyMap_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xded10970_ec85_4115_b52c_4405845642a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConstructReplicaKeyMap_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub FindOrAddReplica: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICoreFragment(::windows_core::IUnknown);
impl ICoreFragment {
    pub unsafe fn NextColumn(&self, pchangeunitid: *mut u8, pchangeunitidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NextColumn)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pchangeunitid), ::core::mem::transmute(pchangeunitidsize)).ok()
    }
    pub unsafe fn NextRange(&self, pitemid: *mut u8, pitemidsize: *mut u32, piclockvector: *mut ::core::option::Option<IClockVector>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NextRange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pitemid), ::core::mem::transmute(pitemidsize), ::core::mem::transmute(piclockvector)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetColumnCount(&self, pcolumncount: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetColumnCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcolumncount)).ok()
    }
    pub unsafe fn GetRangeCount(&self, prangecount: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRangeCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(prangecount)).ok()
    }
}
impl ::core::convert::From<ICoreFragment> for ::windows_core::IUnknown {
    fn from(value: ICoreFragment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICoreFragment> for ::windows_core::IUnknown {
    fn from(value: &ICoreFragment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICoreFragment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICoreFragment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICoreFragment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICoreFragment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoreFragment {}
impl ::core::fmt::Debug for ICoreFragment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoreFragment").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICoreFragment {
    type Vtable = ICoreFragment_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x613b2ab5_b304_47d9_9c31_ce6c54401a15);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreFragment_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub NextColumn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchangeunitid: *mut u8, pchangeunitidsize: *mut u32) -> ::windows_core::HRESULT,
    pub NextRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitemid: *mut u8, pitemidsize: *mut u32, piclockvector: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetColumnCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolumncount: *mut u32) -> ::windows_core::HRESULT,
    pub GetRangeCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prangecount: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICoreFragmentInspector(::windows_core::IUnknown);
impl ICoreFragmentInspector {
    pub unsafe fn NextCoreFragments(&self, requestedcount: u32, ppicorefragments: *mut ::core::option::Option<ICoreFragment>, pfetchedcount: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NextCoreFragments)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(requestedcount), ::core::mem::transmute(ppicorefragments), ::core::mem::transmute(pfetchedcount)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<ICoreFragmentInspector> for ::windows_core::IUnknown {
    fn from(value: ICoreFragmentInspector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICoreFragmentInspector> for ::windows_core::IUnknown {
    fn from(value: &ICoreFragmentInspector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICoreFragmentInspector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICoreFragmentInspector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICoreFragmentInspector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICoreFragmentInspector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoreFragmentInspector {}
impl ::core::fmt::Debug for ICoreFragmentInspector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoreFragmentInspector").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICoreFragmentInspector {
    type Vtable = ICoreFragmentInspector_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf7fcc5fd_ae26_4679_ba16_96aac583c134);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreFragmentInspector_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub NextCoreFragments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedcount: u32, ppicorefragments: *mut ::windows_core::RawPtr, pfetchedcount: *mut u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICustomFilterInfo(::windows_core::IUnknown);
impl ICustomFilterInfo {
    pub unsafe fn Serialize(&self, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Serialize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbbuffer), ::core::mem::transmute(pcbbuffer)).ok()
    }
    pub unsafe fn GetSyncFilter(&self) -> ::windows_core::Result<ISyncFilter> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSyncFilter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncFilter>(result__)
    }
}
impl ::core::convert::From<ICustomFilterInfo> for ::windows_core::IUnknown {
    fn from(value: ICustomFilterInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICustomFilterInfo> for ::windows_core::IUnknown {
    fn from(value: &ICustomFilterInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICustomFilterInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICustomFilterInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICustomFilterInfo> for ISyncFilterInfo {
    fn from(value: ICustomFilterInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICustomFilterInfo> for ISyncFilterInfo {
    fn from(value: &ICustomFilterInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyncFilterInfo> for ICustomFilterInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ISyncFilterInfo> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyncFilterInfo> for &'a ICustomFilterInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ISyncFilterInfo> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICustomFilterInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICustomFilterInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICustomFilterInfo {}
impl ::core::fmt::Debug for ICustomFilterInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICustomFilterInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICustomFilterInfo {
    type Vtable = ICustomFilterInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d335dff_6f88_4e4d_91a8_a3f351cfd473);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomFilterInfo_Vtbl {
    pub base__: ISyncFilterInfo_Vtbl,
    pub GetSyncFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisyncfilter: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(C)]
pub struct ID_PARAMETERS {
    pub dwSize: u32,
    pub replicaId: ID_PARAMETER_PAIR,
    pub itemId: ID_PARAMETER_PAIR,
    pub changeUnitId: ID_PARAMETER_PAIR,
}
impl ::core::marker::Copy for ID_PARAMETERS {}
impl ::core::clone::Clone for ID_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ID_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ID_PARAMETERS").field("dwSize", &self.dwSize).field("replicaId", &self.replicaId).field("itemId", &self.itemId).field("changeUnitId", &self.changeUnitId).finish()
    }
}
unsafe impl ::windows_core::Abi for ID_PARAMETERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ID_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ID_PARAMETERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for ID_PARAMETERS {}
impl ::core::default::Default for ID_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ID_PARAMETER_PAIR {
    pub fIsVariable: ::win32_foundation::BOOL,
    pub cbIdSize: u16,
}
impl ::core::marker::Copy for ID_PARAMETER_PAIR {}
impl ::core::clone::Clone for ID_PARAMETER_PAIR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ID_PARAMETER_PAIR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ID_PARAMETER_PAIR").field("fIsVariable", &self.fIsVariable).field("cbIdSize", &self.cbIdSize).finish()
    }
}
unsafe impl ::windows_core::Abi for ID_PARAMETER_PAIR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ID_PARAMETER_PAIR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ID_PARAMETER_PAIR>()) == 0 }
    }
}
impl ::core::cmp::Eq for ID_PARAMETER_PAIR {}
impl ::core::default::Default for ID_PARAMETER_PAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct IDataRetrieverCallback(::windows_core::IUnknown);
impl IDataRetrieverCallback {
    pub unsafe fn LoadChangeDataComplete<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, punkdata: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LoadChangeDataComplete)(::windows_core::Interface::as_raw(self), punkdata.into_param().abi()).ok()
    }
    pub unsafe fn LoadChangeDataError(&self, hrerror: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LoadChangeDataError)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hrerror)).ok()
    }
}
impl ::core::convert::From<IDataRetrieverCallback> for ::windows_core::IUnknown {
    fn from(value: IDataRetrieverCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDataRetrieverCallback> for ::windows_core::IUnknown {
    fn from(value: &IDataRetrieverCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDataRetrieverCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDataRetrieverCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDataRetrieverCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDataRetrieverCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataRetrieverCallback {}
impl ::core::fmt::Debug for IDataRetrieverCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataRetrieverCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDataRetrieverCallback {
    type Vtable = IDataRetrieverCallback_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x71b4863b_f969_4676_bbc3_3d9fdc3fb2c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataRetrieverCallback_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub LoadChangeDataComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub LoadChangeDataError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrerror: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEnumChangeUnitExceptions(::windows_core::IUnknown);
impl IEnumChangeUnitExceptions {
    pub unsafe fn Next(&self, cexceptions: u32, ppchangeunitexception: *mut ::core::option::Option<IChangeUnitException>, pcfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cexceptions), ::core::mem::transmute(ppchangeunitexception), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Skip(&self, cexceptions: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cexceptions)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumChangeUnitExceptions> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumChangeUnitExceptions>(result__)
    }
}
impl ::core::convert::From<IEnumChangeUnitExceptions> for ::windows_core::IUnknown {
    fn from(value: IEnumChangeUnitExceptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumChangeUnitExceptions> for ::windows_core::IUnknown {
    fn from(value: &IEnumChangeUnitExceptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumChangeUnitExceptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumChangeUnitExceptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumChangeUnitExceptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumChangeUnitExceptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumChangeUnitExceptions {}
impl ::core::fmt::Debug for IEnumChangeUnitExceptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumChangeUnitExceptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumChangeUnitExceptions {
    type Vtable = IEnumChangeUnitExceptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3074e802_9319_4420_be21_1022e2e21da8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumChangeUnitExceptions_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cexceptions: u32, ppchangeunitexception: *mut ::windows_core::RawPtr, pcfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cexceptions: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEnumClockVector(::windows_core::IUnknown);
impl IEnumClockVector {
    pub unsafe fn Next(&self, cclockvectorelements: u32, ppiclockvectorelements: *mut ::core::option::Option<IClockVectorElement>, pcfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cclockvectorelements), ::core::mem::transmute(ppiclockvectorelements), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Skip(&self, csyncversions: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(csyncversions)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumClockVector> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumClockVector>(result__)
    }
}
impl ::core::convert::From<IEnumClockVector> for ::windows_core::IUnknown {
    fn from(value: IEnumClockVector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumClockVector> for ::windows_core::IUnknown {
    fn from(value: &IEnumClockVector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumClockVector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumClockVector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumClockVector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumClockVector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumClockVector {}
impl ::core::fmt::Debug for IEnumClockVector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumClockVector").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumClockVector {
    type Vtable = IEnumClockVector_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x525844db_2837_4799_9e80_81a66e02220c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumClockVector_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cclockvectorelements: u32, ppiclockvectorelements: *mut ::windows_core::RawPtr, pcfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, csyncversions: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppienum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEnumFeedClockVector(::windows_core::IUnknown);
impl IEnumFeedClockVector {
    pub unsafe fn Next(&self, cclockvectorelements: u32, ppiclockvectorelements: *mut ::core::option::Option<IFeedClockVectorElement>, pcfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cclockvectorelements), ::core::mem::transmute(ppiclockvectorelements), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Skip(&self, csyncversions: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(csyncversions)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumFeedClockVector> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumFeedClockVector>(result__)
    }
}
impl ::core::convert::From<IEnumFeedClockVector> for ::windows_core::IUnknown {
    fn from(value: IEnumFeedClockVector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumFeedClockVector> for ::windows_core::IUnknown {
    fn from(value: &IEnumFeedClockVector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumFeedClockVector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumFeedClockVector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumFeedClockVector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumFeedClockVector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumFeedClockVector {}
impl ::core::fmt::Debug for IEnumFeedClockVector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumFeedClockVector").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumFeedClockVector {
    type Vtable = IEnumFeedClockVector_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x550f763d_146a_48f6_abeb_6c88c7f70514);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumFeedClockVector_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cclockvectorelements: u32, ppiclockvectorelements: *mut ::windows_core::RawPtr, pcfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, csyncversions: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppienum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEnumItemIds(::windows_core::IUnknown);
impl IEnumItemIds {
    pub unsafe fn Next(&self, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pcbitemidsize)).ok()
    }
}
impl ::core::convert::From<IEnumItemIds> for ::windows_core::IUnknown {
    fn from(value: IEnumItemIds) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumItemIds> for ::windows_core::IUnknown {
    fn from(value: &IEnumItemIds) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumItemIds {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumItemIds {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumItemIds {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumItemIds {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumItemIds {}
impl ::core::fmt::Debug for IEnumItemIds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumItemIds").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumItemIds {
    type Vtable = IEnumItemIds_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x43aa3f61_4b2e_4b60_83df_b110d3e148f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumItemIds_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEnumRangeExceptions(::windows_core::IUnknown);
impl IEnumRangeExceptions {
    pub unsafe fn Next(&self, cexceptions: u32, pprangeexception: *mut ::core::option::Option<IRangeException>, pcfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cexceptions), ::core::mem::transmute(pprangeexception), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Skip(&self, cexceptions: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cexceptions)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumRangeExceptions> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumRangeExceptions>(result__)
    }
}
impl ::core::convert::From<IEnumRangeExceptions> for ::windows_core::IUnknown {
    fn from(value: IEnumRangeExceptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumRangeExceptions> for ::windows_core::IUnknown {
    fn from(value: &IEnumRangeExceptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumRangeExceptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumRangeExceptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumRangeExceptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumRangeExceptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumRangeExceptions {}
impl ::core::fmt::Debug for IEnumRangeExceptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumRangeExceptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumRangeExceptions {
    type Vtable = IEnumRangeExceptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0944439f_ddb1_4176_b703_046ff22a2386);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumRangeExceptions_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cexceptions: u32, pprangeexception: *mut ::windows_core::RawPtr, pcfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cexceptions: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEnumSingleItemExceptions(::windows_core::IUnknown);
impl IEnumSingleItemExceptions {
    pub unsafe fn Next(&self, cexceptions: u32, ppsingleitemexception: *mut ::core::option::Option<ISingleItemException>, pcfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cexceptions), ::core::mem::transmute(ppsingleitemexception), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Skip(&self, cexceptions: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cexceptions)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumSingleItemExceptions> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumSingleItemExceptions>(result__)
    }
}
impl ::core::convert::From<IEnumSingleItemExceptions> for ::windows_core::IUnknown {
    fn from(value: IEnumSingleItemExceptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumSingleItemExceptions> for ::windows_core::IUnknown {
    fn from(value: &IEnumSingleItemExceptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumSingleItemExceptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumSingleItemExceptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumSingleItemExceptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumSingleItemExceptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumSingleItemExceptions {}
impl ::core::fmt::Debug for IEnumSingleItemExceptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumSingleItemExceptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumSingleItemExceptions {
    type Vtable = IEnumSingleItemExceptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe563381c_1b4d_4c66_9796_c86faccdcd40);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSingleItemExceptions_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cexceptions: u32, ppsingleitemexception: *mut ::windows_core::RawPtr, pcfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cexceptions: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEnumSyncChangeUnits(::windows_core::IUnknown);
impl IEnumSyncChangeUnits {
    pub unsafe fn Next(&self, cchanges: u32, ppchangeunit: *mut ::core::option::Option<ISyncChangeUnit>, pcfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cchanges), ::core::mem::transmute(ppchangeunit), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Skip(&self, cchanges: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cchanges)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumSyncChangeUnits> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumSyncChangeUnits>(result__)
    }
}
impl ::core::convert::From<IEnumSyncChangeUnits> for ::windows_core::IUnknown {
    fn from(value: IEnumSyncChangeUnits) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumSyncChangeUnits> for ::windows_core::IUnknown {
    fn from(value: &IEnumSyncChangeUnits) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumSyncChangeUnits {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumSyncChangeUnits {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumSyncChangeUnits {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumSyncChangeUnits {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumSyncChangeUnits {}
impl ::core::fmt::Debug for IEnumSyncChangeUnits {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumSyncChangeUnits").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumSyncChangeUnits {
    type Vtable = IEnumSyncChangeUnits_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x346b35f1_8703_4c6d_ab1a_4dbca2cff97f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSyncChangeUnits_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchanges: u32, ppchangeunit: *mut ::windows_core::RawPtr, pcfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchanges: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEnumSyncChanges(::windows_core::IUnknown);
impl IEnumSyncChanges {
    pub unsafe fn Next(&self, cchanges: u32, ppchange: *mut ::core::option::Option<ISyncChange>, pcfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cchanges), ::core::mem::transmute(ppchange), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Skip(&self, cchanges: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cchanges)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumSyncChanges> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumSyncChanges>(result__)
    }
}
impl ::core::convert::From<IEnumSyncChanges> for ::windows_core::IUnknown {
    fn from(value: IEnumSyncChanges) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumSyncChanges> for ::windows_core::IUnknown {
    fn from(value: &IEnumSyncChanges) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumSyncChanges {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumSyncChanges {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumSyncChanges {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumSyncChanges {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumSyncChanges {}
impl ::core::fmt::Debug for IEnumSyncChanges {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumSyncChanges").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumSyncChanges {
    type Vtable = IEnumSyncChanges_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5f86be4a_5e78_4e32_ac1c_c24fd223ef85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSyncChanges_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchanges: u32, ppchange: *mut ::windows_core::RawPtr, pcfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchanges: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEnumSyncProviderConfigUIInfos(::windows_core::IUnknown);
impl IEnumSyncProviderConfigUIInfos {
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Next(&self, cfactories: u32, ppsyncproviderconfiguiinfo: *mut ::core::option::Option<ISyncProviderConfigUIInfo>, pcfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cfactories), ::core::mem::transmute(ppsyncproviderconfiguiinfo), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Skip(&self, cfactories: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cfactories)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumSyncProviderConfigUIInfos> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumSyncProviderConfigUIInfos>(result__)
    }
}
impl ::core::convert::From<IEnumSyncProviderConfigUIInfos> for ::windows_core::IUnknown {
    fn from(value: IEnumSyncProviderConfigUIInfos) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumSyncProviderConfigUIInfos> for ::windows_core::IUnknown {
    fn from(value: &IEnumSyncProviderConfigUIInfos) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumSyncProviderConfigUIInfos {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumSyncProviderConfigUIInfos {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumSyncProviderConfigUIInfos {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumSyncProviderConfigUIInfos {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumSyncProviderConfigUIInfos {}
impl ::core::fmt::Debug for IEnumSyncProviderConfigUIInfos {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumSyncProviderConfigUIInfos").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumSyncProviderConfigUIInfos {
    type Vtable = IEnumSyncProviderConfigUIInfos_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf6be2602_17c6_4658_a2d7_68ed3330f641);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSyncProviderConfigUIInfos_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cfactories: u32, ppsyncproviderconfiguiinfo: *mut ::windows_core::RawPtr, pcfetched: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cfactories: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEnumSyncProviderInfos(::windows_core::IUnknown);
impl IEnumSyncProviderInfos {
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Next(&self, cinstances: u32, ppsyncproviderinfo: *mut ::core::option::Option<ISyncProviderInfo>, pcfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cinstances), ::core::mem::transmute(ppsyncproviderinfo), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Skip(&self, cinstances: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cinstances)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumSyncProviderInfos> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumSyncProviderInfos>(result__)
    }
}
impl ::core::convert::From<IEnumSyncProviderInfos> for ::windows_core::IUnknown {
    fn from(value: IEnumSyncProviderInfos) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumSyncProviderInfos> for ::windows_core::IUnknown {
    fn from(value: &IEnumSyncProviderInfos) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumSyncProviderInfos {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumSyncProviderInfos {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumSyncProviderInfos {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumSyncProviderInfos {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumSyncProviderInfos {}
impl ::core::fmt::Debug for IEnumSyncProviderInfos {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumSyncProviderInfos").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumSyncProviderInfos {
    type Vtable = IEnumSyncProviderInfos_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa04ba850_5eb1_460d_a973_393fcb608a11);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSyncProviderInfos_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cinstances: u32, ppsyncproviderinfo: *mut ::windows_core::RawPtr, pcfetched: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cinstances: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IFeedClockVector(::windows_core::IUnknown);
impl IFeedClockVector {
    pub unsafe fn GetClockVectorElements(&self, riid: *const ::windows_core::GUID, ppienumclockvector: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetClockVectorElements)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppienumclockvector)).ok()
    }
    pub unsafe fn GetClockVectorElementCount(&self, pdwcount: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetClockVectorElementCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwcount)).ok()
    }
    pub unsafe fn GetUpdateCount(&self, pdwupdatecount: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetUpdateCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwupdatecount)).ok()
    }
    pub unsafe fn IsNoConflictsSpecified(&self, pfisnoconflictsspecified: *mut ::win32_foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsNoConflictsSpecified)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pfisnoconflictsspecified)).ok()
    }
}
impl ::core::convert::From<IFeedClockVector> for ::windows_core::IUnknown {
    fn from(value: IFeedClockVector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFeedClockVector> for ::windows_core::IUnknown {
    fn from(value: &IFeedClockVector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFeedClockVector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFeedClockVector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFeedClockVector> for IClockVector {
    fn from(value: IFeedClockVector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFeedClockVector> for IClockVector {
    fn from(value: &IFeedClockVector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IClockVector> for IFeedClockVector {
    fn into_param(self) -> ::windows_core::Param<'a, IClockVector> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IClockVector> for &'a IFeedClockVector {
    fn into_param(self) -> ::windows_core::Param<'a, IClockVector> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFeedClockVector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFeedClockVector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFeedClockVector {}
impl ::core::fmt::Debug for IFeedClockVector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFeedClockVector").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IFeedClockVector {
    type Vtable = IFeedClockVector_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8d1d98d1_9fb8_4ec9_a553_54dd924e0f67);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFeedClockVector_Vtbl {
    pub base__: IClockVector_Vtbl,
    pub GetUpdateCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwupdatecount: *mut u32) -> ::windows_core::HRESULT,
    pub IsNoConflictsSpecified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfisnoconflictsspecified: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IFeedClockVectorElement(::windows_core::IUnknown);
impl IFeedClockVectorElement {
    pub unsafe fn GetReplicaKey(&self, pdwreplicakey: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetReplicaKey)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwreplicakey)).ok()
    }
    pub unsafe fn GetTickCount(&self, pulltickcount: *mut u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetTickCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pulltickcount)).ok()
    }
    pub unsafe fn GetSyncTime(&self, psynctime: *mut SYNC_TIME) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSyncTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(psynctime)).ok()
    }
    pub unsafe fn GetFlags(&self, pbflags: *mut u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbflags)).ok()
    }
}
impl ::core::convert::From<IFeedClockVectorElement> for ::windows_core::IUnknown {
    fn from(value: IFeedClockVectorElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFeedClockVectorElement> for ::windows_core::IUnknown {
    fn from(value: &IFeedClockVectorElement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFeedClockVectorElement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFeedClockVectorElement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFeedClockVectorElement> for IClockVectorElement {
    fn from(value: IFeedClockVectorElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFeedClockVectorElement> for IClockVectorElement {
    fn from(value: &IFeedClockVectorElement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IClockVectorElement> for IFeedClockVectorElement {
    fn into_param(self) -> ::windows_core::Param<'a, IClockVectorElement> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IClockVectorElement> for &'a IFeedClockVectorElement {
    fn into_param(self) -> ::windows_core::Param<'a, IClockVectorElement> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFeedClockVectorElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFeedClockVectorElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFeedClockVectorElement {}
impl ::core::fmt::Debug for IFeedClockVectorElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFeedClockVectorElement").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IFeedClockVectorElement {
    type Vtable = IFeedClockVectorElement_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa40b46d2_e97b_4156_b6da_991f501b0f05);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFeedClockVectorElement_Vtbl {
    pub base__: IClockVectorElement_Vtbl,
    pub GetSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psynctime: *mut SYNC_TIME) -> ::windows_core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbflags: *mut u8) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IFilterKeyMap(::windows_core::IUnknown);
impl IFilterKeyMap {
    pub unsafe fn GetCount(&self, pdwcount: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwcount)).ok()
    }
    pub unsafe fn AddFilter<'a, Param0: ::windows_core::IntoParam<'a, ISyncFilter>>(&self, pisyncfilter: Param0, pdwfilterkey: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddFilter)(::windows_core::Interface::as_raw(self), pisyncfilter.into_param().abi(), ::core::mem::transmute(pdwfilterkey)).ok()
    }
    pub unsafe fn GetFilter(&self, dwfilterkey: u32) -> ::windows_core::Result<ISyncFilter> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetFilter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwfilterkey), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncFilter>(result__)
    }
    pub unsafe fn Serialize(&self, pbfilterkeymap: *mut u8, pcbfilterkeymap: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Serialize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbfilterkeymap), ::core::mem::transmute(pcbfilterkeymap)).ok()
    }
}
impl ::core::convert::From<IFilterKeyMap> for ::windows_core::IUnknown {
    fn from(value: IFilterKeyMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFilterKeyMap> for ::windows_core::IUnknown {
    fn from(value: &IFilterKeyMap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFilterKeyMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFilterKeyMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFilterKeyMap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFilterKeyMap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFilterKeyMap {}
impl ::core::fmt::Debug for IFilterKeyMap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFilterKeyMap").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IFilterKeyMap {
    type Vtable = IFilterKeyMap_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xca169652_07c6_4708_a3da_6e4eba8d2297);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFilterKeyMap_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows_core::HRESULT,
    pub AddFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisyncfilter: ::windows_core::RawPtr, pdwfilterkey: *mut u32) -> ::windows_core::HRESULT,
    pub GetFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwfilterkey: u32, ppisyncfilter: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Serialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbfilterkeymap: *mut u8, pcbfilterkeymap: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IFilterRequestCallback(::windows_core::IUnknown);
impl IFilterRequestCallback {
    pub unsafe fn RequestFilter<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, pfilter: Param0, filteringtype: FILTERING_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RequestFilter)(::windows_core::Interface::as_raw(self), pfilter.into_param().abi(), ::core::mem::transmute(filteringtype)).ok()
    }
}
impl ::core::convert::From<IFilterRequestCallback> for ::windows_core::IUnknown {
    fn from(value: IFilterRequestCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFilterRequestCallback> for ::windows_core::IUnknown {
    fn from(value: &IFilterRequestCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFilterRequestCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFilterRequestCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFilterRequestCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFilterRequestCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFilterRequestCallback {}
impl ::core::fmt::Debug for IFilterRequestCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFilterRequestCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IFilterRequestCallback {
    type Vtable = IFilterRequestCallback_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x82df8873_6360_463a_a8a1_ede5e1a1594d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFilterRequestCallback_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub RequestFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilter: *mut ::core::ffi::c_void, filteringtype: FILTERING_TYPE) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IFilterTrackingProvider(::windows_core::IUnknown);
impl IFilterTrackingProvider {
    pub unsafe fn SpecifyTrackedFilters<'a, Param0: ::windows_core::IntoParam<'a, IFilterTrackingRequestCallback>>(&self, pcallback: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SpecifyTrackedFilters)(::windows_core::Interface::as_raw(self), pcallback.into_param().abi()).ok()
    }
    pub unsafe fn AddTrackedFilter<'a, Param0: ::windows_core::IntoParam<'a, ISyncFilter>>(&self, pfilter: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddTrackedFilter)(::windows_core::Interface::as_raw(self), pfilter.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IFilterTrackingProvider> for ::windows_core::IUnknown {
    fn from(value: IFilterTrackingProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFilterTrackingProvider> for ::windows_core::IUnknown {
    fn from(value: &IFilterTrackingProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFilterTrackingProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFilterTrackingProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFilterTrackingProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFilterTrackingProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFilterTrackingProvider {}
impl ::core::fmt::Debug for IFilterTrackingProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFilterTrackingProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IFilterTrackingProvider {
    type Vtable = IFilterTrackingProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x743383c0_fc4e_45ba_ad81_d9d84c7a24f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFilterTrackingProvider_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SpecifyTrackedFilters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AddTrackedFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilter: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IFilterTrackingRequestCallback(::windows_core::IUnknown);
impl IFilterTrackingRequestCallback {
    pub unsafe fn RequestTrackedFilter<'a, Param0: ::windows_core::IntoParam<'a, ISyncFilter>>(&self, pfilter: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RequestTrackedFilter)(::windows_core::Interface::as_raw(self), pfilter.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IFilterTrackingRequestCallback> for ::windows_core::IUnknown {
    fn from(value: IFilterTrackingRequestCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFilterTrackingRequestCallback> for ::windows_core::IUnknown {
    fn from(value: &IFilterTrackingRequestCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFilterTrackingRequestCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFilterTrackingRequestCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFilterTrackingRequestCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFilterTrackingRequestCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFilterTrackingRequestCallback {}
impl ::core::fmt::Debug for IFilterTrackingRequestCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFilterTrackingRequestCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IFilterTrackingRequestCallback {
    type Vtable = IFilterTrackingRequestCallback_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x713ca7bb_c858_4674_b4b6_1122436587a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFilterTrackingRequestCallback_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub RequestTrackedFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilter: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IFilterTrackingSyncChangeBuilder(::windows_core::IUnknown);
impl IFilterTrackingSyncChangeBuilder {
    pub unsafe fn AddFilterChange(&self, dwfilterkey: u32, pfilterchange: *const SYNC_FILTER_CHANGE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddFilterChange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwfilterkey), ::core::mem::transmute(pfilterchange)).ok()
    }
    pub unsafe fn SetAllChangeUnitsPresentFlag(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAllChangeUnitsPresentFlag)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IFilterTrackingSyncChangeBuilder> for ::windows_core::IUnknown {
    fn from(value: IFilterTrackingSyncChangeBuilder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFilterTrackingSyncChangeBuilder> for ::windows_core::IUnknown {
    fn from(value: &IFilterTrackingSyncChangeBuilder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFilterTrackingSyncChangeBuilder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFilterTrackingSyncChangeBuilder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFilterTrackingSyncChangeBuilder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFilterTrackingSyncChangeBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFilterTrackingSyncChangeBuilder {}
impl ::core::fmt::Debug for IFilterTrackingSyncChangeBuilder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFilterTrackingSyncChangeBuilder").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IFilterTrackingSyncChangeBuilder {
    type Vtable = IFilterTrackingSyncChangeBuilder_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x295024a0_70da_4c58_883c_ce2afb308d0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFilterTrackingSyncChangeBuilder_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub AddFilterChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwfilterkey: u32, pfilterchange: *const SYNC_FILTER_CHANGE) -> ::windows_core::HRESULT,
    pub SetAllChangeUnitsPresentFlag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IForgottenKnowledge(::windows_core::IUnknown);
impl IForgottenKnowledge {
    pub unsafe fn GetOwnerReplicaId(&self, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetOwnerReplicaId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbreplicaid), ::core::mem::transmute(pcbidsize)).ok()
    }
    pub unsafe fn Serialize<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fserializereplicakeymap: Param0, pbknowledge: *mut u8, pcbknowledge: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Serialize)(::windows_core::Interface::as_raw(self), fserializereplicakeymap.into_param().abi(), ::core::mem::transmute(pbknowledge), ::core::mem::transmute(pcbknowledge)).ok()
    }
    pub unsafe fn SetLocalTickCount(&self, ulltickcount: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetLocalTickCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ulltickcount)).ok()
    }
    pub unsafe fn ContainsChange(&self, pbversionownerreplicaid: *const u8, pgiditemid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ContainsChange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbversionownerreplicaid), ::core::mem::transmute(pgiditemid), ::core::mem::transmute(psyncversion)).ok()
    }
    pub unsafe fn ContainsChangeUnit(&self, pbversionownerreplicaid: *const u8, pbitemid: *const u8, pbchangeunitid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ContainsChangeUnit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbversionownerreplicaid), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pbchangeunitid), ::core::mem::transmute(psyncversion)).ok()
    }
    pub unsafe fn GetScopeVector(&self, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetScopeVector)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn GetReplicaKeyMap(&self) -> ::windows_core::Result<IReplicaKeyMap> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetReplicaKeyMap)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IReplicaKeyMap>(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn ConvertVersion<'a, Param0: ::windows_core::IntoParam<'a, ISyncKnowledge>>(&self, pknowledgein: Param0, pbcurrentownerid: *const u8, pversionin: *const SYNC_VERSION, pbnewownerid: *mut u8, pcbidsize: *mut u32, pversionout: *mut SYNC_VERSION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ConvertVersion)(::windows_core::Interface::as_raw(self), pknowledgein.into_param().abi(), ::core::mem::transmute(pbcurrentownerid), ::core::mem::transmute(pversionin), ::core::mem::transmute(pbnewownerid), ::core::mem::transmute(pcbidsize), ::core::mem::transmute(pversionout)).ok()
    }
    pub unsafe fn MapRemoteToLocal<'a, Param0: ::windows_core::IntoParam<'a, ISyncKnowledge>>(&self, premoteknowledge: Param0) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.MapRemoteToLocal)(::windows_core::Interface::as_raw(self), premoteknowledge.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn Union<'a, Param0: ::windows_core::IntoParam<'a, ISyncKnowledge>>(&self, pknowledge: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Union)(::windows_core::Interface::as_raw(self), pknowledge.into_param().abi()).ok()
    }
    pub unsafe fn ProjectOntoItem(&self, pbitemid: *const u8) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ProjectOntoItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn ProjectOntoChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ProjectOntoChangeUnit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pbchangeunitid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn ProjectOntoRange(&self, psrngsyncrange: *const SYNC_RANGE) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ProjectOntoRange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(psrngsyncrange), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn ExcludeItem(&self, pbitemid: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ExcludeItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbitemid)).ok()
    }
    pub unsafe fn ExcludeChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ExcludeChangeUnit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pbchangeunitid)).ok()
    }
    pub unsafe fn ContainsKnowledge<'a, Param0: ::windows_core::IntoParam<'a, ISyncKnowledge>>(&self, pknowledge: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ContainsKnowledge)(::windows_core::Interface::as_raw(self), pknowledge.into_param().abi()).ok()
    }
    pub unsafe fn FindMinTickCountForReplica(&self, pbreplicaid: *const u8, pullreplicatickcount: *mut u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.FindMinTickCountForReplica)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbreplicaid), ::core::mem::transmute(pullreplicatickcount)).ok()
    }
    pub unsafe fn GetRangeExceptions(&self, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetRangeExceptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn GetSingleItemExceptions(&self, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetSingleItemExceptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn GetChangeUnitExceptions(&self, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetChangeUnitExceptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn FindClockVectorForItem(&self, pbitemid: *const u8, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.FindClockVectorForItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn FindClockVectorForChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.FindClockVectorForChangeUnit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pbchangeunitid), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn GetVersion(&self, pdwversion: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwversion)).ok()
    }
    pub unsafe fn ForgetToVersion<'a, Param0: ::windows_core::IntoParam<'a, ISyncKnowledge>>(&self, pknowledge: Param0, pversion: *const SYNC_VERSION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ForgetToVersion)(::windows_core::Interface::as_raw(self), pknowledge.into_param().abi(), ::core::mem::transmute(pversion)).ok()
    }
}
impl ::core::convert::From<IForgottenKnowledge> for ::windows_core::IUnknown {
    fn from(value: IForgottenKnowledge) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IForgottenKnowledge> for ::windows_core::IUnknown {
    fn from(value: &IForgottenKnowledge) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IForgottenKnowledge {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IForgottenKnowledge {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IForgottenKnowledge> for ISyncKnowledge {
    fn from(value: IForgottenKnowledge) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IForgottenKnowledge> for ISyncKnowledge {
    fn from(value: &IForgottenKnowledge) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyncKnowledge> for IForgottenKnowledge {
    fn into_param(self) -> ::windows_core::Param<'a, ISyncKnowledge> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyncKnowledge> for &'a IForgottenKnowledge {
    fn into_param(self) -> ::windows_core::Param<'a, ISyncKnowledge> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IForgottenKnowledge {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IForgottenKnowledge {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IForgottenKnowledge {}
impl ::core::fmt::Debug for IForgottenKnowledge {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IForgottenKnowledge").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IForgottenKnowledge {
    type Vtable = IForgottenKnowledge_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x456e0f96_6036_452b_9f9d_bcc4b4a85db2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IForgottenKnowledge_Vtbl {
    pub base__: ISyncKnowledge_Vtbl,
    pub ForgetToVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pknowledge: ::windows_core::RawPtr, pversion: *const SYNC_VERSION) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IKnowledgeSyncProvider(::windows_core::IUnknown);
impl IKnowledgeSyncProvider {
    pub unsafe fn GetIdParameters(&self, pidparameters: *mut ID_PARAMETERS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetIdParameters)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pidparameters)).ok()
    }
    pub unsafe fn BeginSession<'a, Param1: ::windows_core::IntoParam<'a, ISyncSessionState>>(&self, role: SYNC_PROVIDER_ROLE, psessionstate: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginSession)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(role), psessionstate.into_param().abi()).ok()
    }
    pub unsafe fn GetSyncBatchParameters(&self, ppsyncknowledge: *mut ::core::option::Option<ISyncKnowledge>, pdwrequestedbatchsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSyncBatchParameters)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppsyncknowledge), ::core::mem::transmute(pdwrequestedbatchsize)).ok()
    }
    pub unsafe fn GetChangeBatch<'a, Param1: ::windows_core::IntoParam<'a, ISyncKnowledge>>(&self, dwbatchsize: u32, psyncknowledge: Param1, ppsyncchangebatch: *mut ::core::option::Option<ISyncChangeBatch>, ppunkdataretriever: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetChangeBatch)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwbatchsize), psyncknowledge.into_param().abi(), ::core::mem::transmute(ppsyncchangebatch), ::core::mem::transmute(ppunkdataretriever)).ok()
    }
    pub unsafe fn GetFullEnumerationChangeBatch<'a, Param2: ::windows_core::IntoParam<'a, ISyncKnowledge>>(&self, dwbatchsize: u32, pblowerenumerationbound: *const u8, psyncknowledge: Param2, ppsyncchangebatch: *mut ::core::option::Option<ISyncFullEnumerationChangeBatch>, ppunkdataretriever: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFullEnumerationChangeBatch)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwbatchsize), ::core::mem::transmute(pblowerenumerationbound), psyncknowledge.into_param().abi(), ::core::mem::transmute(ppsyncchangebatch), ::core::mem::transmute(ppunkdataretriever)).ok()
    }
    pub unsafe fn ProcessChangeBatch<'a, Param1: ::windows_core::IntoParam<'a, ISyncChangeBatch>, Param2: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param3: ::windows_core::IntoParam<'a, ISyncCallback>>(&self, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: Param1, punkdataretriever: Param2, pcallback: Param3, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ProcessChangeBatch)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(resolutionpolicy), psourcechangebatch.into_param().abi(), punkdataretriever.into_param().abi(), pcallback.into_param().abi(), ::core::mem::transmute(psyncsessionstatistics)).ok()
    }
    pub unsafe fn ProcessFullEnumerationChangeBatch<'a, Param1: ::windows_core::IntoParam<'a, ISyncFullEnumerationChangeBatch>, Param2: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param3: ::windows_core::IntoParam<'a, ISyncCallback>>(&self, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: Param1, punkdataretriever: Param2, pcallback: Param3, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ProcessFullEnumerationChangeBatch)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(resolutionpolicy), psourcechangebatch.into_param().abi(), punkdataretriever.into_param().abi(), pcallback.into_param().abi(), ::core::mem::transmute(psyncsessionstatistics)).ok()
    }
    pub unsafe fn EndSession<'a, Param0: ::windows_core::IntoParam<'a, ISyncSessionState>>(&self, psessionstate: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndSession)(::windows_core::Interface::as_raw(self), psessionstate.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IKnowledgeSyncProvider> for ::windows_core::IUnknown {
    fn from(value: IKnowledgeSyncProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IKnowledgeSyncProvider> for ::windows_core::IUnknown {
    fn from(value: &IKnowledgeSyncProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IKnowledgeSyncProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IKnowledgeSyncProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IKnowledgeSyncProvider> for ISyncProvider {
    fn from(value: IKnowledgeSyncProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IKnowledgeSyncProvider> for ISyncProvider {
    fn from(value: &IKnowledgeSyncProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyncProvider> for IKnowledgeSyncProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ISyncProvider> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyncProvider> for &'a IKnowledgeSyncProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ISyncProvider> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IKnowledgeSyncProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IKnowledgeSyncProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IKnowledgeSyncProvider {}
impl ::core::fmt::Debug for IKnowledgeSyncProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKnowledgeSyncProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IKnowledgeSyncProvider {
    type Vtable = IKnowledgeSyncProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x43434a49_8da4_47f2_8172_ad7b8b024978);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnowledgeSyncProvider_Vtbl {
    pub base__: ISyncProvider_Vtbl,
    pub BeginSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, role: SYNC_PROVIDER_ROLE, psessionstate: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetSyncBatchParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsyncknowledge: *mut ::windows_core::RawPtr, pdwrequestedbatchsize: *mut u32) -> ::windows_core::HRESULT,
    pub GetChangeBatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwbatchsize: u32, psyncknowledge: ::windows_core::RawPtr, ppsyncchangebatch: *mut ::windows_core::RawPtr, ppunkdataretriever: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFullEnumerationChangeBatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwbatchsize: u32, pblowerenumerationbound: *const u8, psyncknowledge: ::windows_core::RawPtr, ppsyncchangebatch: *mut ::windows_core::RawPtr, ppunkdataretriever: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ProcessChangeBatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: ::windows_core::RawPtr, punkdataretriever: *mut ::core::ffi::c_void, pcallback: ::windows_core::RawPtr, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> ::windows_core::HRESULT,
    pub ProcessFullEnumerationChangeBatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: ::windows_core::RawPtr, punkdataretriever: *mut ::core::ffi::c_void, pcallback: ::windows_core::RawPtr, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> ::windows_core::HRESULT,
    pub EndSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psessionstate: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ILoadChangeContext(::windows_core::IUnknown);
impl ILoadChangeContext {
    pub unsafe fn GetSyncChange(&self) -> ::windows_core::Result<ISyncChange> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSyncChange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChange>(result__)
    }
    pub unsafe fn SetRecoverableErrorOnChange<'a, Param1: ::windows_core::IntoParam<'a, IRecoverableErrorData>>(&self, hrerror: ::windows_core::HRESULT, perrordata: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRecoverableErrorOnChange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hrerror), perrordata.into_param().abi()).ok()
    }
    pub unsafe fn SetRecoverableErrorOnChangeUnit<'a, Param1: ::windows_core::IntoParam<'a, ISyncChangeUnit>, Param2: ::windows_core::IntoParam<'a, IRecoverableErrorData>>(&self, hrerror: ::windows_core::HRESULT, pchangeunit: Param1, perrordata: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRecoverableErrorOnChangeUnit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hrerror), pchangeunit.into_param().abi(), perrordata.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ILoadChangeContext> for ::windows_core::IUnknown {
    fn from(value: ILoadChangeContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILoadChangeContext> for ::windows_core::IUnknown {
    fn from(value: &ILoadChangeContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ILoadChangeContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ILoadChangeContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ILoadChangeContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILoadChangeContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILoadChangeContext {}
impl ::core::fmt::Debug for ILoadChangeContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILoadChangeContext").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ILoadChangeContext {
    type Vtable = ILoadChangeContext_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x44a4aaca_ec39_46d5_b5c9_d633c0ee67e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoadChangeContext_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetSyncChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsyncchange: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetRecoverableErrorOnChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrerror: ::windows_core::HRESULT, perrordata: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetRecoverableErrorOnChangeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrerror: ::windows_core::HRESULT, pchangeunit: ::windows_core::RawPtr, perrordata: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IProviderConverter(::windows_core::IUnknown);
impl IProviderConverter {
    pub unsafe fn Initialize<'a, Param0: ::windows_core::IntoParam<'a, ISyncProvider>>(&self, pisyncprovider: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), pisyncprovider.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IProviderConverter> for ::windows_core::IUnknown {
    fn from(value: IProviderConverter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProviderConverter> for ::windows_core::IUnknown {
    fn from(value: &IProviderConverter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IProviderConverter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IProviderConverter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IProviderConverter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProviderConverter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProviderConverter {}
impl ::core::fmt::Debug for IProviderConverter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProviderConverter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IProviderConverter {
    type Vtable = IProviderConverter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x809b7276_98cf_4957_93a5_0ebdd3dddffd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProviderConverter_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisyncprovider: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRangeException(::windows_core::IUnknown);
impl IRangeException {
    pub unsafe fn GetClosedRangeStart(&self, pbclosedrangestart: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetClosedRangeStart)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbclosedrangestart), ::core::mem::transmute(pcbidsize)).ok()
    }
    pub unsafe fn GetClosedRangeEnd(&self, pbclosedrangeend: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetClosedRangeEnd)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbclosedrangeend), ::core::mem::transmute(pcbidsize)).ok()
    }
    pub unsafe fn GetClockVector(&self, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetClockVector)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
}
impl ::core::convert::From<IRangeException> for ::windows_core::IUnknown {
    fn from(value: IRangeException) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRangeException> for ::windows_core::IUnknown {
    fn from(value: &IRangeException) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRangeException {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRangeException {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRangeException {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRangeException {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRangeException {}
impl ::core::fmt::Debug for IRangeException {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRangeException").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRangeException {
    type Vtable = IRangeException_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x75ae8777_6848_49f7_956c_a3a92f5096e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeException_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetClosedRangeStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbclosedrangestart: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT,
    pub GetClosedRangeEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbclosedrangeend: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT,
    pub GetClockVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRecoverableError(::windows_core::IUnknown);
impl IRecoverableError {
    pub unsafe fn GetStage(&self, pstage: *mut SYNC_PROGRESS_STAGE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetStage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pstage)).ok()
    }
    pub unsafe fn GetProvider(&self, pproviderrole: *mut SYNC_PROVIDER_ROLE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetProvider)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pproviderrole)).ok()
    }
    pub unsafe fn GetChangeWithRecoverableError(&self) -> ::windows_core::Result<ISyncChange> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetChangeWithRecoverableError)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChange>(result__)
    }
    pub unsafe fn GetRecoverableErrorDataForChange(&self, phrerror: *mut ::windows_core::HRESULT, pperrordata: *mut ::core::option::Option<IRecoverableErrorData>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRecoverableErrorDataForChange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(phrerror), ::core::mem::transmute(pperrordata)).ok()
    }
    pub unsafe fn GetRecoverableErrorDataForChangeUnit<'a, Param0: ::windows_core::IntoParam<'a, ISyncChangeUnit>>(&self, pchangeunit: Param0, phrerror: *mut ::windows_core::HRESULT, pperrordata: *mut ::core::option::Option<IRecoverableErrorData>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRecoverableErrorDataForChangeUnit)(::windows_core::Interface::as_raw(self), pchangeunit.into_param().abi(), ::core::mem::transmute(phrerror), ::core::mem::transmute(pperrordata)).ok()
    }
}
impl ::core::convert::From<IRecoverableError> for ::windows_core::IUnknown {
    fn from(value: IRecoverableError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRecoverableError> for ::windows_core::IUnknown {
    fn from(value: &IRecoverableError) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRecoverableError {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRecoverableError {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRecoverableError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRecoverableError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRecoverableError {}
impl ::core::fmt::Debug for IRecoverableError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRecoverableError").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRecoverableError {
    type Vtable = IRecoverableError_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0f5625e8_0a7b_45ee_9637_1ce13645909e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRecoverableError_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetStage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstage: *mut SYNC_PROGRESS_STAGE) -> ::windows_core::HRESULT,
    pub GetProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproviderrole: *mut SYNC_PROVIDER_ROLE) -> ::windows_core::HRESULT,
    pub GetChangeWithRecoverableError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppchangewithrecoverableerror: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetRecoverableErrorDataForChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phrerror: *mut ::windows_core::HRESULT, pperrordata: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetRecoverableErrorDataForChangeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchangeunit: ::windows_core::RawPtr, phrerror: *mut ::windows_core::HRESULT, pperrordata: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRecoverableErrorData(::windows_core::IUnknown);
impl IRecoverableErrorData {
    pub unsafe fn Initialize<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pcszitemdisplayname: Param0, pcszerrordescription: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), pcszitemdisplayname.into_param().abi(), pcszerrordescription.into_param().abi()).ok()
    }
    pub unsafe fn GetItemDisplayName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszitemdisplayname: Param0, pcchitemdisplayname: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetItemDisplayName)(::windows_core::Interface::as_raw(self), pszitemdisplayname.into_param().abi(), ::core::mem::transmute(pcchitemdisplayname)).ok()
    }
    pub unsafe fn GetErrorDescription<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszerrordescription: Param0, pccherrordescription: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetErrorDescription)(::windows_core::Interface::as_raw(self), pszerrordescription.into_param().abi(), ::core::mem::transmute(pccherrordescription)).ok()
    }
}
impl ::core::convert::From<IRecoverableErrorData> for ::windows_core::IUnknown {
    fn from(value: IRecoverableErrorData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRecoverableErrorData> for ::windows_core::IUnknown {
    fn from(value: &IRecoverableErrorData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRecoverableErrorData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRecoverableErrorData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRecoverableErrorData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRecoverableErrorData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRecoverableErrorData {}
impl ::core::fmt::Debug for IRecoverableErrorData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRecoverableErrorData").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRecoverableErrorData {
    type Vtable = IRecoverableErrorData_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb37c4a0a_4b7d_4c2d_9711_3b00d119b1c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRecoverableErrorData_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcszitemdisplayname: ::windows_core::PCWSTR, pcszerrordescription: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetItemDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszitemdisplayname: ::windows_core::PCWSTR, pcchitemdisplayname: *mut u32) -> ::windows_core::HRESULT,
    pub GetErrorDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszerrordescription: ::windows_core::PCWSTR, pccherrordescription: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRegisteredSyncProvider(::windows_core::IUnknown);
impl IRegisteredSyncProvider {
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Init<'a, Param2: ::windows_core::IntoParam<'a, ::win32_ui::Shell::PropertiesSystem::IPropertyStore>>(&self, pguidinstanceid: *const ::windows_core::GUID, pguidcontenttype: *const ::windows_core::GUID, pcontextpropertystore: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Init)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pguidinstanceid), ::core::mem::transmute(pguidcontenttype), pcontextpropertystore.into_param().abi()).ok()
    }
    pub unsafe fn GetInstanceId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).GetInstanceId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IRegisteredSyncProvider> for ::windows_core::IUnknown {
    fn from(value: IRegisteredSyncProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRegisteredSyncProvider> for ::windows_core::IUnknown {
    fn from(value: &IRegisteredSyncProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRegisteredSyncProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRegisteredSyncProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRegisteredSyncProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRegisteredSyncProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRegisteredSyncProvider {}
impl ::core::fmt::Debug for IRegisteredSyncProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRegisteredSyncProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRegisteredSyncProvider {
    type Vtable = IRegisteredSyncProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x913bcf76_47c1_40b5_a896_5e8a9c414c14);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRegisteredSyncProvider_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Init: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows_core::GUID, pguidcontenttype: *const ::windows_core::GUID, pcontextpropertystore: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Init: usize,
    pub GetInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidinstanceid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IReplicaKeyMap(::windows_core::IUnknown);
impl IReplicaKeyMap {
    pub unsafe fn LookupReplicaKey(&self, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LookupReplicaKey)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbreplicaid), ::core::mem::transmute(pdwreplicakey)).ok()
    }
    pub unsafe fn LookupReplicaId(&self, dwreplicakey: u32, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LookupReplicaId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwreplicakey), ::core::mem::transmute(pbreplicaid), ::core::mem::transmute(pcbidsize)).ok()
    }
    pub unsafe fn Serialize(&self, pbreplicakeymap: *mut u8, pcbreplicakeymap: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Serialize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbreplicakeymap), ::core::mem::transmute(pcbreplicakeymap)).ok()
    }
}
impl ::core::convert::From<IReplicaKeyMap> for ::windows_core::IUnknown {
    fn from(value: IReplicaKeyMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IReplicaKeyMap> for ::windows_core::IUnknown {
    fn from(value: &IReplicaKeyMap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IReplicaKeyMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IReplicaKeyMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IReplicaKeyMap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IReplicaKeyMap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReplicaKeyMap {}
impl ::core::fmt::Debug for IReplicaKeyMap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReplicaKeyMap").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IReplicaKeyMap {
    type Vtable = IReplicaKeyMap_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2209f4fc_fd10_4ff0_84a8_f0a1982e440e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReplicaKeyMap_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub LookupReplicaKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> ::windows_core::HRESULT,
    pub LookupReplicaId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwreplicakey: u32, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT,
    pub Serialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbreplicakeymap: *mut u8, pcbreplicakeymap: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRequestFilteredSync(::windows_core::IUnknown);
impl IRequestFilteredSync {
    pub unsafe fn SpecifyFilter<'a, Param0: ::windows_core::IntoParam<'a, IFilterRequestCallback>>(&self, pcallback: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SpecifyFilter)(::windows_core::Interface::as_raw(self), pcallback.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IRequestFilteredSync> for ::windows_core::IUnknown {
    fn from(value: IRequestFilteredSync) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRequestFilteredSync> for ::windows_core::IUnknown {
    fn from(value: &IRequestFilteredSync) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRequestFilteredSync {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRequestFilteredSync {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRequestFilteredSync {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRequestFilteredSync {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRequestFilteredSync {}
impl ::core::fmt::Debug for IRequestFilteredSync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRequestFilteredSync").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRequestFilteredSync {
    type Vtable = IRequestFilteredSync_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2e020184_6d18_46a7_a32a_da4aeb06696c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRequestFilteredSync_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SpecifyFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISingleItemException(::windows_core::IUnknown);
impl ISingleItemException {
    pub unsafe fn GetItemId(&self, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetItemId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pcbidsize)).ok()
    }
    pub unsafe fn GetClockVector(&self, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetClockVector)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
}
impl ::core::convert::From<ISingleItemException> for ::windows_core::IUnknown {
    fn from(value: ISingleItemException) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISingleItemException> for ::windows_core::IUnknown {
    fn from(value: &ISingleItemException) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISingleItemException {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISingleItemException {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISingleItemException {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISingleItemException {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISingleItemException {}
impl ::core::fmt::Debug for ISingleItemException {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISingleItemException").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISingleItemException {
    type Vtable = ISingleItemException_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x892fb9b0_7c55_4a18_9316_fdf449569b64);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISingleItemException_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetItemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT,
    pub GetClockVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISupportFilteredSync(::windows_core::IUnknown);
impl ISupportFilteredSync {
    pub unsafe fn AddFilter<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, pfilter: Param0, filteringtype: FILTERING_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddFilter)(::windows_core::Interface::as_raw(self), pfilter.into_param().abi(), ::core::mem::transmute(filteringtype)).ok()
    }
}
impl ::core::convert::From<ISupportFilteredSync> for ::windows_core::IUnknown {
    fn from(value: ISupportFilteredSync) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISupportFilteredSync> for ::windows_core::IUnknown {
    fn from(value: &ISupportFilteredSync) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISupportFilteredSync {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISupportFilteredSync {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISupportFilteredSync {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISupportFilteredSync {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISupportFilteredSync {}
impl ::core::fmt::Debug for ISupportFilteredSync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISupportFilteredSync").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISupportFilteredSync {
    type Vtable = ISupportFilteredSync_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3d128ded_d555_4e0d_bf4b_fb213a8a9302);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISupportFilteredSync_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub AddFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilter: *mut ::core::ffi::c_void, filteringtype: FILTERING_TYPE) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISupportLastWriteTime(::windows_core::IUnknown);
impl ISupportLastWriteTime {
    pub unsafe fn GetItemChangeTime(&self, pbitemid: *const u8, pulltimestamp: *mut u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetItemChangeTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pulltimestamp)).ok()
    }
    pub unsafe fn GetChangeUnitChangeTime(&self, pbitemid: *const u8, pbchangeunitid: *const u8, pulltimestamp: *mut u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetChangeUnitChangeTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pbchangeunitid), ::core::mem::transmute(pulltimestamp)).ok()
    }
}
impl ::core::convert::From<ISupportLastWriteTime> for ::windows_core::IUnknown {
    fn from(value: ISupportLastWriteTime) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISupportLastWriteTime> for ::windows_core::IUnknown {
    fn from(value: &ISupportLastWriteTime) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISupportLastWriteTime {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISupportLastWriteTime {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISupportLastWriteTime {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISupportLastWriteTime {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISupportLastWriteTime {}
impl ::core::fmt::Debug for ISupportLastWriteTime {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISupportLastWriteTime").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISupportLastWriteTime {
    type Vtable = ISupportLastWriteTime_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeadf816f_d0bd_43ca_8f40_5acdc6c06f7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISupportLastWriteTime_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetItemChangeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pulltimestamp: *mut u64) -> ::windows_core::HRESULT,
    pub GetChangeUnitChangeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8, pulltimestamp: *mut u64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISyncCallback(::windows_core::IUnknown);
impl ISyncCallback {
    pub unsafe fn OnProgress(&self, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnProgress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(provider), ::core::mem::transmute(syncstage), ::core::mem::transmute(dwcompletedwork), ::core::mem::transmute(dwtotalwork)).ok()
    }
    pub unsafe fn OnChange<'a, Param0: ::windows_core::IntoParam<'a, ISyncChange>>(&self, psyncchange: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnChange)(::windows_core::Interface::as_raw(self), psyncchange.into_param().abi()).ok()
    }
    pub unsafe fn OnConflict<'a, Param0: ::windows_core::IntoParam<'a, IChangeConflict>>(&self, pconflict: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnConflict)(::windows_core::Interface::as_raw(self), pconflict.into_param().abi()).ok()
    }
    pub unsafe fn OnFullEnumerationNeeded(&self, pfullenumerationaction: *mut SYNC_FULL_ENUMERATION_ACTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnFullEnumerationNeeded)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pfullenumerationaction)).ok()
    }
    pub unsafe fn OnRecoverableError<'a, Param0: ::windows_core::IntoParam<'a, IRecoverableError>>(&self, precoverableerror: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnRecoverableError)(::windows_core::Interface::as_raw(self), precoverableerror.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ISyncCallback> for ::windows_core::IUnknown {
    fn from(value: ISyncCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncCallback> for ::windows_core::IUnknown {
    fn from(value: &ISyncCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISyncCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISyncCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISyncCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncCallback {}
impl ::core::fmt::Debug for ISyncCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISyncCallback {
    type Vtable = ISyncCallback_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0599797f_5ed9_485c_ae36_0c5d1bf2e7a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncCallback_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows_core::HRESULT,
    pub OnChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psyncchange: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OnConflict: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconflict: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OnFullEnumerationNeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfullenumerationaction: *mut SYNC_FULL_ENUMERATION_ACTION) -> ::windows_core::HRESULT,
    pub OnRecoverableError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, precoverableerror: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISyncCallback2(::windows_core::IUnknown);
impl ISyncCallback2 {
    pub unsafe fn OnProgress(&self, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnProgress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(provider), ::core::mem::transmute(syncstage), ::core::mem::transmute(dwcompletedwork), ::core::mem::transmute(dwtotalwork)).ok()
    }
    pub unsafe fn OnChange<'a, Param0: ::windows_core::IntoParam<'a, ISyncChange>>(&self, psyncchange: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnChange)(::windows_core::Interface::as_raw(self), psyncchange.into_param().abi()).ok()
    }
    pub unsafe fn OnConflict<'a, Param0: ::windows_core::IntoParam<'a, IChangeConflict>>(&self, pconflict: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnConflict)(::windows_core::Interface::as_raw(self), pconflict.into_param().abi()).ok()
    }
    pub unsafe fn OnFullEnumerationNeeded(&self, pfullenumerationaction: *mut SYNC_FULL_ENUMERATION_ACTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnFullEnumerationNeeded)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pfullenumerationaction)).ok()
    }
    pub unsafe fn OnRecoverableError<'a, Param0: ::windows_core::IntoParam<'a, IRecoverableError>>(&self, precoverableerror: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnRecoverableError)(::windows_core::Interface::as_raw(self), precoverableerror.into_param().abi()).ok()
    }
    pub unsafe fn OnChangeApplied(&self, dwchangesapplied: u32, dwchangesfailed: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnChangeApplied)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwchangesapplied), ::core::mem::transmute(dwchangesfailed)).ok()
    }
    pub unsafe fn OnChangeFailed(&self, dwchangesapplied: u32, dwchangesfailed: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnChangeFailed)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwchangesapplied), ::core::mem::transmute(dwchangesfailed)).ok()
    }
}
impl ::core::convert::From<ISyncCallback2> for ::windows_core::IUnknown {
    fn from(value: ISyncCallback2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncCallback2> for ::windows_core::IUnknown {
    fn from(value: &ISyncCallback2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISyncCallback2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISyncCallback2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISyncCallback2> for ISyncCallback {
    fn from(value: ISyncCallback2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncCallback2> for ISyncCallback {
    fn from(value: &ISyncCallback2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyncCallback> for ISyncCallback2 {
    fn into_param(self) -> ::windows_core::Param<'a, ISyncCallback> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyncCallback> for &'a ISyncCallback2 {
    fn into_param(self) -> ::windows_core::Param<'a, ISyncCallback> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISyncCallback2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncCallback2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncCallback2 {}
impl ::core::fmt::Debug for ISyncCallback2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncCallback2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISyncCallback2 {
    type Vtable = ISyncCallback2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x47ce84af_7442_4ead_8630_12015e030ad7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncCallback2_Vtbl {
    pub base__: ISyncCallback_Vtbl,
    pub OnChangeApplied: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwchangesapplied: u32, dwchangesfailed: u32) -> ::windows_core::HRESULT,
    pub OnChangeFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwchangesapplied: u32, dwchangesfailed: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISyncChange(::windows_core::IUnknown);
impl ISyncChange {
    pub unsafe fn GetOwnerReplicaId(&self, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetOwnerReplicaId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbreplicaid), ::core::mem::transmute(pcbidsize)).ok()
    }
    pub unsafe fn GetRootItemId(&self, pbrootitemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRootItemId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbrootitemid), ::core::mem::transmute(pcbidsize)).ok()
    }
    pub unsafe fn GetChangeVersion(&self, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetChangeVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbcurrentreplicaid), ::core::mem::transmute(pversion)).ok()
    }
    pub unsafe fn GetCreationVersion(&self, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCreationVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbcurrentreplicaid), ::core::mem::transmute(pversion)).ok()
    }
    pub unsafe fn GetFlags(&self, pdwflags: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwflags)).ok()
    }
    pub unsafe fn GetWorkEstimate(&self, pdwwork: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetWorkEstimate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwwork)).ok()
    }
    pub unsafe fn GetChangeUnits(&self) -> ::windows_core::Result<IEnumSyncChangeUnits> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetChangeUnits)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumSyncChangeUnits>(result__)
    }
    pub unsafe fn GetMadeWithKnowledge(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetMadeWithKnowledge)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetLearnedKnowledge(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetLearnedKnowledge)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn SetWorkEstimate(&self, dwwork: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetWorkEstimate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwwork)).ok()
    }
}
impl ::core::convert::From<ISyncChange> for ::windows_core::IUnknown {
    fn from(value: ISyncChange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncChange> for ::windows_core::IUnknown {
    fn from(value: &ISyncChange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISyncChange {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISyncChange {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISyncChange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncChange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncChange {}
impl ::core::fmt::Debug for ISyncChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncChange").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISyncChange {
    type Vtable = ISyncChange_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa1952beb_0f6b_4711_b136_01da85b968a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChange_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetOwnerReplicaId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT,
    pub GetRootItemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbrootitemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT,
    pub GetChangeVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows_core::HRESULT,
    pub GetCreationVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows_core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT,
    pub GetWorkEstimate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwwork: *mut u32) -> ::windows_core::HRESULT,
    pub GetChangeUnits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetMadeWithKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmadewithknowledge: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetLearnedKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplearnedknowledge: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetWorkEstimate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwwork: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISyncChangeBatch(::windows_core::IUnknown);
impl ISyncChangeBatch {
    pub unsafe fn GetChangeEnumerator(&self) -> ::windows_core::Result<IEnumSyncChanges> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetChangeEnumerator)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumSyncChanges>(result__)
    }
    pub unsafe fn GetIsLastBatch(&self, pflastbatch: *mut ::win32_foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetIsLastBatch)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pflastbatch)).ok()
    }
    pub unsafe fn GetWorkEstimateForBatch(&self, pdwworkforbatch: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetWorkEstimateForBatch)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwworkforbatch)).ok()
    }
    pub unsafe fn GetRemainingWorkEstimateForSession(&self, pdwremainingworkforsession: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetRemainingWorkEstimateForSession)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwremainingworkforsession)).ok()
    }
    pub unsafe fn BeginOrderedGroup(&self, pblowerbound: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.BeginOrderedGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pblowerbound)).ok()
    }
    pub unsafe fn EndOrderedGroup<'a, Param1: ::windows_core::IntoParam<'a, ISyncKnowledge>>(&self, pbupperbound: *const u8, pmadewithknowledge: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EndOrderedGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbupperbound), pmadewithknowledge.into_param().abi()).ok()
    }
    pub unsafe fn AddItemMetadataToGroup(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32) -> ::windows_core::Result<ISyncChangeBuilder> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.AddItemMetadataToGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbownerreplicaid), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pchangeversion), ::core::mem::transmute(pcreationversion), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwworkforchange), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChangeBuilder>(result__)
    }
    pub unsafe fn GetLearnedKnowledge(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetLearnedKnowledge)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetPrerequisiteKnowledge(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPrerequisiteKnowledge)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetSourceForgottenKnowledge(&self) -> ::windows_core::Result<IForgottenKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSourceForgottenKnowledge)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IForgottenKnowledge>(result__)
    }
    pub unsafe fn SetLastBatch(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetLastBatch)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetWorkEstimateForBatch(&self, dwworkforbatch: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetWorkEstimateForBatch)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwworkforbatch)).ok()
    }
    pub unsafe fn SetRemainingWorkEstimateForSession(&self, dwremainingworkforsession: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRemainingWorkEstimateForSession)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwremainingworkforsession)).ok()
    }
    pub unsafe fn Serialize(&self, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Serialize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbchangebatch), ::core::mem::transmute(pcbchangebatch)).ok()
    }
    pub unsafe fn BeginUnorderedGroup(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginUnorderedGroup)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EndUnorderedGroup<'a, Param0: ::windows_core::IntoParam<'a, ISyncKnowledge>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, pmadewithknowledge: Param0, fallchangesforknowledge: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndUnorderedGroup)(::windows_core::Interface::as_raw(self), pmadewithknowledge.into_param().abi(), fallchangesforknowledge.into_param().abi()).ok()
    }
    pub unsafe fn AddLoggedConflict<'a, Param6: ::windows_core::IntoParam<'a, ISyncKnowledge>>(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, pconflictknowledge: Param6) -> ::windows_core::Result<ISyncChangeBuilder> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).AddLoggedConflict)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbownerreplicaid), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pchangeversion), ::core::mem::transmute(pcreationversion), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwworkforchange), pconflictknowledge.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChangeBuilder>(result__)
    }
}
impl ::core::convert::From<ISyncChangeBatch> for ::windows_core::IUnknown {
    fn from(value: ISyncChangeBatch) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncChangeBatch> for ::windows_core::IUnknown {
    fn from(value: &ISyncChangeBatch) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISyncChangeBatch {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISyncChangeBatch {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISyncChangeBatch> for ISyncChangeBatchBase {
    fn from(value: ISyncChangeBatch) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncChangeBatch> for ISyncChangeBatchBase {
    fn from(value: &ISyncChangeBatch) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyncChangeBatchBase> for ISyncChangeBatch {
    fn into_param(self) -> ::windows_core::Param<'a, ISyncChangeBatchBase> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyncChangeBatchBase> for &'a ISyncChangeBatch {
    fn into_param(self) -> ::windows_core::Param<'a, ISyncChangeBatchBase> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISyncChangeBatch {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncChangeBatch {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncChangeBatch {}
impl ::core::fmt::Debug for ISyncChangeBatch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncChangeBatch").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISyncChangeBatch {
    type Vtable = ISyncChangeBatch_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x70c64dee_380f_4c2e_8f70_31c55bd5f9b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeBatch_Vtbl {
    pub base__: ISyncChangeBatchBase_Vtbl,
    pub BeginUnorderedGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EndUnorderedGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmadewithknowledge: ::windows_core::RawPtr, fallchangesforknowledge: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub AddLoggedConflict: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, pconflictknowledge: ::windows_core::RawPtr, ppchangebuilder: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISyncChangeBatch2(::windows_core::IUnknown);
impl ISyncChangeBatch2 {
    pub unsafe fn GetChangeEnumerator(&self) -> ::windows_core::Result<IEnumSyncChanges> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetChangeEnumerator)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumSyncChanges>(result__)
    }
    pub unsafe fn GetIsLastBatch(&self, pflastbatch: *mut ::win32_foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetIsLastBatch)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pflastbatch)).ok()
    }
    pub unsafe fn GetWorkEstimateForBatch(&self, pdwworkforbatch: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetWorkEstimateForBatch)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwworkforbatch)).ok()
    }
    pub unsafe fn GetRemainingWorkEstimateForSession(&self, pdwremainingworkforsession: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetRemainingWorkEstimateForSession)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwremainingworkforsession)).ok()
    }
    pub unsafe fn BeginOrderedGroup(&self, pblowerbound: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.BeginOrderedGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pblowerbound)).ok()
    }
    pub unsafe fn EndOrderedGroup<'a, Param1: ::windows_core::IntoParam<'a, ISyncKnowledge>>(&self, pbupperbound: *const u8, pmadewithknowledge: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.EndOrderedGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbupperbound), pmadewithknowledge.into_param().abi()).ok()
    }
    pub unsafe fn AddItemMetadataToGroup(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32) -> ::windows_core::Result<ISyncChangeBuilder> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.AddItemMetadataToGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbownerreplicaid), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pchangeversion), ::core::mem::transmute(pcreationversion), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwworkforchange), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChangeBuilder>(result__)
    }
    pub unsafe fn GetLearnedKnowledge(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetLearnedKnowledge)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetPrerequisiteKnowledge(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetPrerequisiteKnowledge)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetSourceForgottenKnowledge(&self) -> ::windows_core::Result<IForgottenKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetSourceForgottenKnowledge)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IForgottenKnowledge>(result__)
    }
    pub unsafe fn SetLastBatch(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetLastBatch)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetWorkEstimateForBatch(&self, dwworkforbatch: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetWorkEstimateForBatch)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwworkforbatch)).ok()
    }
    pub unsafe fn SetRemainingWorkEstimateForSession(&self, dwremainingworkforsession: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetRemainingWorkEstimateForSession)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwremainingworkforsession)).ok()
    }
    pub unsafe fn Serialize(&self, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Serialize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbchangebatch), ::core::mem::transmute(pcbchangebatch)).ok()
    }
    pub unsafe fn BeginUnorderedGroup(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.BeginUnorderedGroup)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EndUnorderedGroup<'a, Param0: ::windows_core::IntoParam<'a, ISyncKnowledge>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, pmadewithknowledge: Param0, fallchangesforknowledge: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EndUnorderedGroup)(::windows_core::Interface::as_raw(self), pmadewithknowledge.into_param().abi(), fallchangesforknowledge.into_param().abi()).ok()
    }
    pub unsafe fn AddLoggedConflict<'a, Param6: ::windows_core::IntoParam<'a, ISyncKnowledge>>(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, pconflictknowledge: Param6) -> ::windows_core::Result<ISyncChangeBuilder> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.AddLoggedConflict)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbownerreplicaid), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pchangeversion), ::core::mem::transmute(pcreationversion), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwworkforchange), pconflictknowledge.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChangeBuilder>(result__)
    }
    pub unsafe fn AddMergeTombstoneMetadataToGroup(&self, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32) -> ::windows_core::Result<ISyncChangeBuilder> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).AddMergeTombstoneMetadataToGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbownerreplicaid), ::core::mem::transmute(pbwinneritemid), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pchangeversion), ::core::mem::transmute(pcreationversion), ::core::mem::transmute(dwworkforchange), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChangeBuilder>(result__)
    }
    pub unsafe fn AddMergeTombstoneLoggedConflict<'a, Param6: ::windows_core::IntoParam<'a, ISyncKnowledge>>(&self, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, pconflictknowledge: Param6) -> ::windows_core::Result<ISyncChangeBuilder> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).AddMergeTombstoneLoggedConflict)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbownerreplicaid), ::core::mem::transmute(pbwinneritemid), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pchangeversion), ::core::mem::transmute(pcreationversion), ::core::mem::transmute(dwworkforchange), pconflictknowledge.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChangeBuilder>(result__)
    }
}
impl ::core::convert::From<ISyncChangeBatch2> for ::windows_core::IUnknown {
    fn from(value: ISyncChangeBatch2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncChangeBatch2> for ::windows_core::IUnknown {
    fn from(value: &ISyncChangeBatch2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISyncChangeBatch2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISyncChangeBatch2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISyncChangeBatch2> for ISyncChangeBatchBase {
    fn from(value: ISyncChangeBatch2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncChangeBatch2> for ISyncChangeBatchBase {
    fn from(value: &ISyncChangeBatch2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyncChangeBatchBase> for ISyncChangeBatch2 {
    fn into_param(self) -> ::windows_core::Param<'a, ISyncChangeBatchBase> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyncChangeBatchBase> for &'a ISyncChangeBatch2 {
    fn into_param(self) -> ::windows_core::Param<'a, ISyncChangeBatchBase> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISyncChangeBatch2> for ISyncChangeBatch {
    fn from(value: ISyncChangeBatch2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncChangeBatch2> for ISyncChangeBatch {
    fn from(value: &ISyncChangeBatch2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyncChangeBatch> for ISyncChangeBatch2 {
    fn into_param(self) -> ::windows_core::Param<'a, ISyncChangeBatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyncChangeBatch> for &'a ISyncChangeBatch2 {
    fn into_param(self) -> ::windows_core::Param<'a, ISyncChangeBatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISyncChangeBatch2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncChangeBatch2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncChangeBatch2 {}
impl ::core::fmt::Debug for ISyncChangeBatch2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncChangeBatch2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISyncChangeBatch2 {
    type Vtable = ISyncChangeBatch2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x225f4a33_f5ee_4cc7_b039_67a262b4b2ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeBatch2_Vtbl {
    pub base__: ISyncChangeBatch_Vtbl,
    pub AddMergeTombstoneMetadataToGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, ppchangebuilder: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AddMergeTombstoneLoggedConflict: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, pconflictknowledge: ::windows_core::RawPtr, ppchangebuilder: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISyncChangeBatchAdvanced(::windows_core::IUnknown);
impl ISyncChangeBatchAdvanced {
    pub unsafe fn GetFilterInfo(&self) -> ::windows_core::Result<ISyncFilterInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetFilterInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncFilterInfo>(result__)
    }
    pub unsafe fn ConvertFullEnumerationChangeBatchToRegularChangeBatch(&self) -> ::windows_core::Result<ISyncChangeBatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ConvertFullEnumerationChangeBatchToRegularChangeBatch)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChangeBatch>(result__)
    }
    pub unsafe fn GetUpperBoundItemId(&self, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetUpperBoundItemId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pcbidsize)).ok()
    }
    pub unsafe fn GetBatchLevelKnowledgeShouldBeApplied(&self, pfbatchknowledgeshouldbeapplied: *mut ::win32_foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetBatchLevelKnowledgeShouldBeApplied)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pfbatchknowledgeshouldbeapplied)).ok()
    }
}
impl ::core::convert::From<ISyncChangeBatchAdvanced> for ::windows_core::IUnknown {
    fn from(value: ISyncChangeBatchAdvanced) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncChangeBatchAdvanced> for ::windows_core::IUnknown {
    fn from(value: &ISyncChangeBatchAdvanced) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISyncChangeBatchAdvanced {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISyncChangeBatchAdvanced {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISyncChangeBatchAdvanced {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncChangeBatchAdvanced {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncChangeBatchAdvanced {}
impl ::core::fmt::Debug for ISyncChangeBatchAdvanced {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncChangeBatchAdvanced").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISyncChangeBatchAdvanced {
    type Vtable = ISyncChangeBatchAdvanced_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0f1a4995_cbc8_421d_b550_5d0bebf3e9a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeBatchAdvanced_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetFilterInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfilterinfo: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ConvertFullEnumerationChangeBatchToRegularChangeBatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppchangebatch: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetUpperBoundItemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT,
    pub GetBatchLevelKnowledgeShouldBeApplied: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfbatchknowledgeshouldbeapplied: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISyncChangeBatchBase(::windows_core::IUnknown);
impl ISyncChangeBatchBase {
    pub unsafe fn GetChangeEnumerator(&self) -> ::windows_core::Result<IEnumSyncChanges> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetChangeEnumerator)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumSyncChanges>(result__)
    }
    pub unsafe fn GetIsLastBatch(&self, pflastbatch: *mut ::win32_foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetIsLastBatch)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pflastbatch)).ok()
    }
    pub unsafe fn GetWorkEstimateForBatch(&self, pdwworkforbatch: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetWorkEstimateForBatch)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwworkforbatch)).ok()
    }
    pub unsafe fn GetRemainingWorkEstimateForSession(&self, pdwremainingworkforsession: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRemainingWorkEstimateForSession)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwremainingworkforsession)).ok()
    }
    pub unsafe fn BeginOrderedGroup(&self, pblowerbound: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginOrderedGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pblowerbound)).ok()
    }
    pub unsafe fn EndOrderedGroup<'a, Param1: ::windows_core::IntoParam<'a, ISyncKnowledge>>(&self, pbupperbound: *const u8, pmadewithknowledge: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndOrderedGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbupperbound), pmadewithknowledge.into_param().abi()).ok()
    }
    pub unsafe fn AddItemMetadataToGroup(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32) -> ::windows_core::Result<ISyncChangeBuilder> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).AddItemMetadataToGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbownerreplicaid), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pchangeversion), ::core::mem::transmute(pcreationversion), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwworkforchange), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChangeBuilder>(result__)
    }
    pub unsafe fn GetLearnedKnowledge(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetLearnedKnowledge)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetPrerequisiteKnowledge(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetPrerequisiteKnowledge)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetSourceForgottenKnowledge(&self) -> ::windows_core::Result<IForgottenKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSourceForgottenKnowledge)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IForgottenKnowledge>(result__)
    }
    pub unsafe fn SetLastBatch(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLastBatch)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetWorkEstimateForBatch(&self, dwworkforbatch: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetWorkEstimateForBatch)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwworkforbatch)).ok()
    }
    pub unsafe fn SetRemainingWorkEstimateForSession(&self, dwremainingworkforsession: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRemainingWorkEstimateForSession)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwremainingworkforsession)).ok()
    }
    pub unsafe fn Serialize(&self, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Serialize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbchangebatch), ::core::mem::transmute(pcbchangebatch)).ok()
    }
}
impl ::core::convert::From<ISyncChangeBatchBase> for ::windows_core::IUnknown {
    fn from(value: ISyncChangeBatchBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncChangeBatchBase> for ::windows_core::IUnknown {
    fn from(value: &ISyncChangeBatchBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISyncChangeBatchBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISyncChangeBatchBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISyncChangeBatchBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncChangeBatchBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncChangeBatchBase {}
impl ::core::fmt::Debug for ISyncChangeBatchBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncChangeBatchBase").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISyncChangeBatchBase {
    type Vtable = ISyncChangeBatchBase_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x52f6e694_6a71_4494_a184_a8311bf5d227);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeBatchBase_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetChangeEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetIsLastBatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflastbatch: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetWorkEstimateForBatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwworkforbatch: *mut u32) -> ::windows_core::HRESULT,
    pub GetRemainingWorkEstimateForSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwremainingworkforsession: *mut u32) -> ::windows_core::HRESULT,
    pub BeginOrderedGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblowerbound: *const u8) -> ::windows_core::HRESULT,
    pub EndOrderedGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbupperbound: *const u8, pmadewithknowledge: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AddItemMetadataToGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, ppchangebuilder: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetLearnedKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplearnedknowledge: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetPrerequisiteKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprerequisteknowledge: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetSourceForgottenKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsourceforgottenknowledge: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetLastBatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetWorkEstimateForBatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwworkforbatch: u32) -> ::windows_core::HRESULT,
    pub SetRemainingWorkEstimateForSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwremainingworkforsession: u32) -> ::windows_core::HRESULT,
    pub Serialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISyncChangeBatchBase2(::windows_core::IUnknown);
impl ISyncChangeBatchBase2 {
    pub unsafe fn GetChangeEnumerator(&self) -> ::windows_core::Result<IEnumSyncChanges> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetChangeEnumerator)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumSyncChanges>(result__)
    }
    pub unsafe fn GetIsLastBatch(&self, pflastbatch: *mut ::win32_foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetIsLastBatch)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pflastbatch)).ok()
    }
    pub unsafe fn GetWorkEstimateForBatch(&self, pdwworkforbatch: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetWorkEstimateForBatch)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwworkforbatch)).ok()
    }
    pub unsafe fn GetRemainingWorkEstimateForSession(&self, pdwremainingworkforsession: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetRemainingWorkEstimateForSession)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwremainingworkforsession)).ok()
    }
    pub unsafe fn BeginOrderedGroup(&self, pblowerbound: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.BeginOrderedGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pblowerbound)).ok()
    }
    pub unsafe fn EndOrderedGroup<'a, Param1: ::windows_core::IntoParam<'a, ISyncKnowledge>>(&self, pbupperbound: *const u8, pmadewithknowledge: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EndOrderedGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbupperbound), pmadewithknowledge.into_param().abi()).ok()
    }
    pub unsafe fn AddItemMetadataToGroup(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32) -> ::windows_core::Result<ISyncChangeBuilder> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.AddItemMetadataToGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbownerreplicaid), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pchangeversion), ::core::mem::transmute(pcreationversion), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwworkforchange), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChangeBuilder>(result__)
    }
    pub unsafe fn GetLearnedKnowledge(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetLearnedKnowledge)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetPrerequisiteKnowledge(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPrerequisiteKnowledge)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetSourceForgottenKnowledge(&self) -> ::windows_core::Result<IForgottenKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSourceForgottenKnowledge)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IForgottenKnowledge>(result__)
    }
    pub unsafe fn SetLastBatch(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetLastBatch)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetWorkEstimateForBatch(&self, dwworkforbatch: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetWorkEstimateForBatch)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwworkforbatch)).ok()
    }
    pub unsafe fn SetRemainingWorkEstimateForSession(&self, dwremainingworkforsession: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRemainingWorkEstimateForSession)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwremainingworkforsession)).ok()
    }
    pub unsafe fn Serialize(&self, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Serialize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbchangebatch), ::core::mem::transmute(pcbchangebatch)).ok()
    }
    pub unsafe fn SerializeWithOptions(&self, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SerializeWithOptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(targetformatversion), ::core::mem::transmute(dwflags), ::core::mem::transmute(pbbuffer), ::core::mem::transmute(pdwserializedsize)).ok()
    }
}
impl ::core::convert::From<ISyncChangeBatchBase2> for ::windows_core::IUnknown {
    fn from(value: ISyncChangeBatchBase2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncChangeBatchBase2> for ::windows_core::IUnknown {
    fn from(value: &ISyncChangeBatchBase2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISyncChangeBatchBase2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISyncChangeBatchBase2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISyncChangeBatchBase2> for ISyncChangeBatchBase {
    fn from(value: ISyncChangeBatchBase2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncChangeBatchBase2> for ISyncChangeBatchBase {
    fn from(value: &ISyncChangeBatchBase2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyncChangeBatchBase> for ISyncChangeBatchBase2 {
    fn into_param(self) -> ::windows_core::Param<'a, ISyncChangeBatchBase> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyncChangeBatchBase> for &'a ISyncChangeBatchBase2 {
    fn into_param(self) -> ::windows_core::Param<'a, ISyncChangeBatchBase> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISyncChangeBatchBase2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncChangeBatchBase2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncChangeBatchBase2 {}
impl ::core::fmt::Debug for ISyncChangeBatchBase2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncChangeBatchBase2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISyncChangeBatchBase2 {
    type Vtable = ISyncChangeBatchBase2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6fdb596a_d755_4584_bd0c_c0c23a548fbf);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeBatchBase2_Vtbl {
    pub base__: ISyncChangeBatchBase_Vtbl,
    pub SerializeWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISyncChangeBatchWithFilterKeyMap(::windows_core::IUnknown);
impl ISyncChangeBatchWithFilterKeyMap {
    pub unsafe fn GetFilterKeyMap(&self) -> ::windows_core::Result<IFilterKeyMap> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetFilterKeyMap)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFilterKeyMap>(result__)
    }
    pub unsafe fn SetFilterKeyMap<'a, Param0: ::windows_core::IntoParam<'a, IFilterKeyMap>>(&self, pifilterkeymap: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFilterKeyMap)(::windows_core::Interface::as_raw(self), pifilterkeymap.into_param().abi()).ok()
    }
    pub unsafe fn SetFilterForgottenKnowledge<'a, Param1: ::windows_core::IntoParam<'a, ISyncKnowledge>>(&self, dwfilterkey: u32, pfilterforgottenknowledge: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFilterForgottenKnowledge)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwfilterkey), pfilterforgottenknowledge.into_param().abi()).ok()
    }
    pub unsafe fn GetFilteredReplicaLearnedKnowledge<'a, Param0: ::windows_core::IntoParam<'a, ISyncKnowledge>, Param1: ::windows_core::IntoParam<'a, IEnumItemIds>>(&self, pdestinationknowledge: Param0, pnewmoveins: Param1) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetFilteredReplicaLearnedKnowledge)(::windows_core::Interface::as_raw(self), pdestinationknowledge.into_param().abi(), pnewmoveins.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetLearnedFilterForgottenKnowledge<'a, Param0: ::windows_core::IntoParam<'a, ISyncKnowledge>, Param1: ::windows_core::IntoParam<'a, IEnumItemIds>>(&self, pdestinationknowledge: Param0, pnewmoveins: Param1, dwfilterkey: u32) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetLearnedFilterForgottenKnowledge)(::windows_core::Interface::as_raw(self), pdestinationknowledge.into_param().abi(), pnewmoveins.into_param().abi(), ::core::mem::transmute(dwfilterkey), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetFilteredReplicaLearnedForgottenKnowledge<'a, Param0: ::windows_core::IntoParam<'a, ISyncKnowledge>, Param1: ::windows_core::IntoParam<'a, IEnumItemIds>>(&self, pdestinationknowledge: Param0, pnewmoveins: Param1) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetFilteredReplicaLearnedForgottenKnowledge)(::windows_core::Interface::as_raw(self), pdestinationknowledge.into_param().abi(), pnewmoveins.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete<'a, Param0: ::windows_core::IntoParam<'a, ISyncKnowledge>, Param1: ::windows_core::IntoParam<'a, IEnumItemIds>>(&self, pdestinationknowledge: Param0, pnewmoveins: Param1) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete)(::windows_core::Interface::as_raw(self), pdestinationknowledge.into_param().abi(), pnewmoveins.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete<'a, Param0: ::windows_core::IntoParam<'a, ISyncKnowledge>, Param1: ::windows_core::IntoParam<'a, IEnumItemIds>>(&self, pdestinationknowledge: Param0, pnewmoveins: Param1, dwfilterkey: u32) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete)(::windows_core::Interface::as_raw(self), pdestinationknowledge.into_param().abi(), pnewmoveins.into_param().abi(), ::core::mem::transmute(dwfilterkey), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
}
impl ::core::convert::From<ISyncChangeBatchWithFilterKeyMap> for ::windows_core::IUnknown {
    fn from(value: ISyncChangeBatchWithFilterKeyMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncChangeBatchWithFilterKeyMap> for ::windows_core::IUnknown {
    fn from(value: &ISyncChangeBatchWithFilterKeyMap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISyncChangeBatchWithFilterKeyMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISyncChangeBatchWithFilterKeyMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISyncChangeBatchWithFilterKeyMap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncChangeBatchWithFilterKeyMap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncChangeBatchWithFilterKeyMap {}
impl ::core::fmt::Debug for ISyncChangeBatchWithFilterKeyMap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncChangeBatchWithFilterKeyMap").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISyncChangeBatchWithFilterKeyMap {
    type Vtable = ISyncChangeBatchWithFilterKeyMap_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xde247002_566d_459a_a6ed_a5aab3459fb7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeBatchWithFilterKeyMap_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetFilterKeyMap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppifilterkeymap: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetFilterKeyMap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pifilterkeymap: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetFilterForgottenKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwfilterkey: u32, pfilterforgottenknowledge: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetFilteredReplicaLearnedKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows_core::RawPtr, pnewmoveins: ::windows_core::RawPtr, pplearnedforgottenknowledge: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetLearnedFilterForgottenKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows_core::RawPtr, pnewmoveins: ::windows_core::RawPtr, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetFilteredReplicaLearnedForgottenKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows_core::RawPtr, pnewmoveins: ::windows_core::RawPtr, pplearnedforgottenknowledge: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows_core::RawPtr, pnewmoveins: ::windows_core::RawPtr, pplearnedforgottenknowledge: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows_core::RawPtr, pnewmoveins: ::windows_core::RawPtr, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISyncChangeBatchWithPrerequisite(::windows_core::IUnknown);
impl ISyncChangeBatchWithPrerequisite {
    pub unsafe fn GetChangeEnumerator(&self) -> ::windows_core::Result<IEnumSyncChanges> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetChangeEnumerator)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumSyncChanges>(result__)
    }
    pub unsafe fn GetIsLastBatch(&self, pflastbatch: *mut ::win32_foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetIsLastBatch)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pflastbatch)).ok()
    }
    pub unsafe fn GetWorkEstimateForBatch(&self, pdwworkforbatch: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetWorkEstimateForBatch)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwworkforbatch)).ok()
    }
    pub unsafe fn GetRemainingWorkEstimateForSession(&self, pdwremainingworkforsession: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetRemainingWorkEstimateForSession)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwremainingworkforsession)).ok()
    }
    pub unsafe fn BeginOrderedGroup(&self, pblowerbound: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.BeginOrderedGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pblowerbound)).ok()
    }
    pub unsafe fn EndOrderedGroup<'a, Param1: ::windows_core::IntoParam<'a, ISyncKnowledge>>(&self, pbupperbound: *const u8, pmadewithknowledge: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EndOrderedGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbupperbound), pmadewithknowledge.into_param().abi()).ok()
    }
    pub unsafe fn AddItemMetadataToGroup(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32) -> ::windows_core::Result<ISyncChangeBuilder> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.AddItemMetadataToGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbownerreplicaid), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pchangeversion), ::core::mem::transmute(pcreationversion), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwworkforchange), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChangeBuilder>(result__)
    }
    pub unsafe fn GetLearnedKnowledge(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetLearnedKnowledge)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetPrerequisiteKnowledge(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPrerequisiteKnowledge)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetSourceForgottenKnowledge(&self) -> ::windows_core::Result<IForgottenKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSourceForgottenKnowledge)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IForgottenKnowledge>(result__)
    }
    pub unsafe fn SetLastBatch(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetLastBatch)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetWorkEstimateForBatch(&self, dwworkforbatch: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetWorkEstimateForBatch)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwworkforbatch)).ok()
    }
    pub unsafe fn SetRemainingWorkEstimateForSession(&self, dwremainingworkforsession: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRemainingWorkEstimateForSession)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwremainingworkforsession)).ok()
    }
    pub unsafe fn Serialize(&self, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Serialize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbchangebatch), ::core::mem::transmute(pcbchangebatch)).ok()
    }
    pub unsafe fn SetPrerequisiteKnowledge<'a, Param0: ::windows_core::IntoParam<'a, ISyncKnowledge>>(&self, pprerequisiteknowledge: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPrerequisiteKnowledge)(::windows_core::Interface::as_raw(self), pprerequisiteknowledge.into_param().abi()).ok()
    }
    pub unsafe fn GetLearnedKnowledgeWithPrerequisite<'a, Param0: ::windows_core::IntoParam<'a, ISyncKnowledge>>(&self, pdestinationknowledge: Param0) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetLearnedKnowledgeWithPrerequisite)(::windows_core::Interface::as_raw(self), pdestinationknowledge.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetLearnedForgottenKnowledge(&self) -> ::windows_core::Result<IForgottenKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetLearnedForgottenKnowledge)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IForgottenKnowledge>(result__)
    }
}
impl ::core::convert::From<ISyncChangeBatchWithPrerequisite> for ::windows_core::IUnknown {
    fn from(value: ISyncChangeBatchWithPrerequisite) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncChangeBatchWithPrerequisite> for ::windows_core::IUnknown {
    fn from(value: &ISyncChangeBatchWithPrerequisite) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISyncChangeBatchWithPrerequisite {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISyncChangeBatchWithPrerequisite {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISyncChangeBatchWithPrerequisite> for ISyncChangeBatchBase {
    fn from(value: ISyncChangeBatchWithPrerequisite) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncChangeBatchWithPrerequisite> for ISyncChangeBatchBase {
    fn from(value: &ISyncChangeBatchWithPrerequisite) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyncChangeBatchBase> for ISyncChangeBatchWithPrerequisite {
    fn into_param(self) -> ::windows_core::Param<'a, ISyncChangeBatchBase> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyncChangeBatchBase> for &'a ISyncChangeBatchWithPrerequisite {
    fn into_param(self) -> ::windows_core::Param<'a, ISyncChangeBatchBase> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISyncChangeBatchWithPrerequisite {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncChangeBatchWithPrerequisite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncChangeBatchWithPrerequisite {}
impl ::core::fmt::Debug for ISyncChangeBatchWithPrerequisite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncChangeBatchWithPrerequisite").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISyncChangeBatchWithPrerequisite {
    type Vtable = ISyncChangeBatchWithPrerequisite_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x097f13be_5b92_4048_b3f2_7b42a2515e07);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeBatchWithPrerequisite_Vtbl {
    pub base__: ISyncChangeBatchBase_Vtbl,
    pub SetPrerequisiteKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprerequisiteknowledge: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetLearnedKnowledgeWithPrerequisite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows_core::RawPtr, pplearnedwithprerequisiteknowledge: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetLearnedForgottenKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplearnedforgottenknowledge: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISyncChangeBuilder(::windows_core::IUnknown);
impl ISyncChangeBuilder {
    pub unsafe fn AddChangeUnitMetadata(&self, pbchangeunitid: *const u8, pchangeunitversion: *const SYNC_VERSION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddChangeUnitMetadata)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbchangeunitid), ::core::mem::transmute(pchangeunitversion)).ok()
    }
}
impl ::core::convert::From<ISyncChangeBuilder> for ::windows_core::IUnknown {
    fn from(value: ISyncChangeBuilder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncChangeBuilder> for ::windows_core::IUnknown {
    fn from(value: &ISyncChangeBuilder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISyncChangeBuilder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISyncChangeBuilder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISyncChangeBuilder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncChangeBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncChangeBuilder {}
impl ::core::fmt::Debug for ISyncChangeBuilder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncChangeBuilder").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISyncChangeBuilder {
    type Vtable = ISyncChangeBuilder_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x56f14771_8677_484f_a170_e386e418a676);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeBuilder_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub AddChangeUnitMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbchangeunitid: *const u8, pchangeunitversion: *const SYNC_VERSION) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISyncChangeUnit(::windows_core::IUnknown);
impl ISyncChangeUnit {
    pub unsafe fn GetItemChange(&self) -> ::windows_core::Result<ISyncChange> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetItemChange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChange>(result__)
    }
    pub unsafe fn GetChangeUnitId(&self, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetChangeUnitId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbchangeunitid), ::core::mem::transmute(pcbidsize)).ok()
    }
    pub unsafe fn GetChangeUnitVersion(&self, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetChangeUnitVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbcurrentreplicaid), ::core::mem::transmute(pversion)).ok()
    }
}
impl ::core::convert::From<ISyncChangeUnit> for ::windows_core::IUnknown {
    fn from(value: ISyncChangeUnit) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncChangeUnit> for ::windows_core::IUnknown {
    fn from(value: &ISyncChangeUnit) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISyncChangeUnit {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISyncChangeUnit {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISyncChangeUnit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncChangeUnit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncChangeUnit {}
impl ::core::fmt::Debug for ISyncChangeUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncChangeUnit").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISyncChangeUnit {
    type Vtable = ISyncChangeUnit_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x60edd8ca_7341_4bb7_95ce_fab6394b51cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeUnit_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetItemChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsyncchange: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetChangeUnitId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT,
    pub GetChangeUnitVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISyncChangeWithFilterKeyMap(::windows_core::IUnknown);
impl ISyncChangeWithFilterKeyMap {
    pub unsafe fn GetFilterCount(&self, pdwfiltercount: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFilterCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwfiltercount)).ok()
    }
    pub unsafe fn GetFilterChange(&self, dwfilterkey: u32, pfilterchange: *mut SYNC_FILTER_CHANGE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFilterChange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwfilterkey), ::core::mem::transmute(pfilterchange)).ok()
    }
    pub unsafe fn GetAllChangeUnitsPresentFlag(&self, pfallchangeunitspresent: *mut ::win32_foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetAllChangeUnitsPresentFlag)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pfallchangeunitspresent)).ok()
    }
    pub unsafe fn GetFilterForgottenKnowledge(&self, dwfilterkey: u32) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetFilterForgottenKnowledge)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwfilterkey), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetFilteredReplicaLearnedKnowledge<'a, Param0: ::windows_core::IntoParam<'a, ISyncKnowledge>, Param1: ::windows_core::IntoParam<'a, IEnumItemIds>>(&self, pdestinationknowledge: Param0, pnewmoveins: Param1) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetFilteredReplicaLearnedKnowledge)(::windows_core::Interface::as_raw(self), pdestinationknowledge.into_param().abi(), pnewmoveins.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetLearnedFilterForgottenKnowledge<'a, Param0: ::windows_core::IntoParam<'a, ISyncKnowledge>, Param1: ::windows_core::IntoParam<'a, IEnumItemIds>>(&self, pdestinationknowledge: Param0, pnewmoveins: Param1, dwfilterkey: u32) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetLearnedFilterForgottenKnowledge)(::windows_core::Interface::as_raw(self), pdestinationknowledge.into_param().abi(), pnewmoveins.into_param().abi(), ::core::mem::transmute(dwfilterkey), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetFilteredReplicaLearnedForgottenKnowledge<'a, Param0: ::windows_core::IntoParam<'a, ISyncKnowledge>, Param1: ::windows_core::IntoParam<'a, IEnumItemIds>>(&self, pdestinationknowledge: Param0, pnewmoveins: Param1) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetFilteredReplicaLearnedForgottenKnowledge)(::windows_core::Interface::as_raw(self), pdestinationknowledge.into_param().abi(), pnewmoveins.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete<'a, Param0: ::windows_core::IntoParam<'a, ISyncKnowledge>, Param1: ::windows_core::IntoParam<'a, IEnumItemIds>>(&self, pdestinationknowledge: Param0, pnewmoveins: Param1) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete)(::windows_core::Interface::as_raw(self), pdestinationknowledge.into_param().abi(), pnewmoveins.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete<'a, Param0: ::windows_core::IntoParam<'a, ISyncKnowledge>, Param1: ::windows_core::IntoParam<'a, IEnumItemIds>>(&self, pdestinationknowledge: Param0, pnewmoveins: Param1, dwfilterkey: u32) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete)(::windows_core::Interface::as_raw(self), pdestinationknowledge.into_param().abi(), pnewmoveins.into_param().abi(), ::core::mem::transmute(dwfilterkey), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
}
impl ::core::convert::From<ISyncChangeWithFilterKeyMap> for ::windows_core::IUnknown {
    fn from(value: ISyncChangeWithFilterKeyMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncChangeWithFilterKeyMap> for ::windows_core::IUnknown {
    fn from(value: &ISyncChangeWithFilterKeyMap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISyncChangeWithFilterKeyMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISyncChangeWithFilterKeyMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISyncChangeWithFilterKeyMap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncChangeWithFilterKeyMap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncChangeWithFilterKeyMap {}
impl ::core::fmt::Debug for ISyncChangeWithFilterKeyMap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncChangeWithFilterKeyMap").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISyncChangeWithFilterKeyMap {
    type Vtable = ISyncChangeWithFilterKeyMap_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbfe1ef00_e87d_42fd_a4e9_242d70414aef);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeWithFilterKeyMap_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetFilterCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwfiltercount: *mut u32) -> ::windows_core::HRESULT,
    pub GetFilterChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwfilterkey: u32, pfilterchange: *mut SYNC_FILTER_CHANGE) -> ::windows_core::HRESULT,
    pub GetAllChangeUnitsPresentFlag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfallchangeunitspresent: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetFilterForgottenKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwfilterkey: u32, ppifilterforgottenknowledge: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetFilteredReplicaLearnedKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows_core::RawPtr, pnewmoveins: ::windows_core::RawPtr, pplearnedknowledge: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetLearnedFilterForgottenKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows_core::RawPtr, pnewmoveins: ::windows_core::RawPtr, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetFilteredReplicaLearnedForgottenKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows_core::RawPtr, pnewmoveins: ::windows_core::RawPtr, pplearnedforgottenknowledge: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows_core::RawPtr, pnewmoveins: ::windows_core::RawPtr, pplearnedforgottenknowledge: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows_core::RawPtr, pnewmoveins: ::windows_core::RawPtr, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISyncChangeWithPrerequisite(::windows_core::IUnknown);
impl ISyncChangeWithPrerequisite {
    pub unsafe fn GetPrerequisiteKnowledge(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetPrerequisiteKnowledge)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetLearnedKnowledgeWithPrerequisite<'a, Param0: ::windows_core::IntoParam<'a, ISyncKnowledge>>(&self, pdestinationknowledge: Param0) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetLearnedKnowledgeWithPrerequisite)(::windows_core::Interface::as_raw(self), pdestinationknowledge.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
}
impl ::core::convert::From<ISyncChangeWithPrerequisite> for ::windows_core::IUnknown {
    fn from(value: ISyncChangeWithPrerequisite) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncChangeWithPrerequisite> for ::windows_core::IUnknown {
    fn from(value: &ISyncChangeWithPrerequisite) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISyncChangeWithPrerequisite {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISyncChangeWithPrerequisite {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISyncChangeWithPrerequisite {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncChangeWithPrerequisite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncChangeWithPrerequisite {}
impl ::core::fmt::Debug for ISyncChangeWithPrerequisite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncChangeWithPrerequisite").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISyncChangeWithPrerequisite {
    type Vtable = ISyncChangeWithPrerequisite_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9e38382f_1589_48c3_92e4_05ecdcb4f3f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeWithPrerequisite_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetPrerequisiteKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprerequisiteknowledge: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetLearnedKnowledgeWithPrerequisite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows_core::RawPtr, pplearnedknowledgewithprerequisite: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISyncConstraintCallback(::windows_core::IUnknown);
impl ISyncConstraintCallback {
    pub unsafe fn OnConstraintConflict<'a, Param0: ::windows_core::IntoParam<'a, IConstraintConflict>>(&self, pconflict: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnConstraintConflict)(::windows_core::Interface::as_raw(self), pconflict.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ISyncConstraintCallback> for ::windows_core::IUnknown {
    fn from(value: ISyncConstraintCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncConstraintCallback> for ::windows_core::IUnknown {
    fn from(value: &ISyncConstraintCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISyncConstraintCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISyncConstraintCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISyncConstraintCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncConstraintCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncConstraintCallback {}
impl ::core::fmt::Debug for ISyncConstraintCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncConstraintCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISyncConstraintCallback {
    type Vtable = ISyncConstraintCallback_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8af3843e_75b3_438c_bb51_6f020d70d3cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncConstraintCallback_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnConstraintConflict: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconflict: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISyncDataConverter(::windows_core::IUnknown);
impl ISyncDataConverter {
    pub unsafe fn ConvertDataRetrieverFromProviderFormat<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param1: ::windows_core::IntoParam<'a, IEnumSyncChanges>>(&self, punkdataretrieverin: Param0, penumsyncchanges: Param1) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).ConvertDataRetrieverFromProviderFormat)(::windows_core::Interface::as_raw(self), punkdataretrieverin.into_param().abi(), penumsyncchanges.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn ConvertDataRetrieverToProviderFormat<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param1: ::windows_core::IntoParam<'a, IEnumSyncChanges>>(&self, punkdataretrieverin: Param0, penumsyncchanges: Param1) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).ConvertDataRetrieverToProviderFormat)(::windows_core::Interface::as_raw(self), punkdataretrieverin.into_param().abi(), penumsyncchanges.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn ConvertDataFromProviderFormat<'a, Param0: ::windows_core::IntoParam<'a, ILoadChangeContext>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, pdatacontext: Param0, punkdatain: Param1) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).ConvertDataFromProviderFormat)(::windows_core::Interface::as_raw(self), pdatacontext.into_param().abi(), punkdatain.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn ConvertDataToProviderFormat<'a, Param0: ::windows_core::IntoParam<'a, ILoadChangeContext>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, pdatacontext: Param0, punkdataout: Param1) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).ConvertDataToProviderFormat)(::windows_core::Interface::as_raw(self), pdatacontext.into_param().abi(), punkdataout.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
}
impl ::core::convert::From<ISyncDataConverter> for ::windows_core::IUnknown {
    fn from(value: ISyncDataConverter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncDataConverter> for ::windows_core::IUnknown {
    fn from(value: &ISyncDataConverter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISyncDataConverter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISyncDataConverter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISyncDataConverter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncDataConverter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncDataConverter {}
impl ::core::fmt::Debug for ISyncDataConverter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncDataConverter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISyncDataConverter {
    type Vtable = ISyncDataConverter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x435d4861_68d5_44aa_a0f9_72a0b00ef9cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncDataConverter_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub ConvertDataRetrieverFromProviderFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkdataretrieverin: *mut ::core::ffi::c_void, penumsyncchanges: ::windows_core::RawPtr, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ConvertDataRetrieverToProviderFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkdataretrieverin: *mut ::core::ffi::c_void, penumsyncchanges: ::windows_core::RawPtr, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ConvertDataFromProviderFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatacontext: ::windows_core::RawPtr, punkdatain: *mut ::core::ffi::c_void, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ConvertDataToProviderFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatacontext: ::windows_core::RawPtr, punkdataout: *mut ::core::ffi::c_void, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISyncFilter(::windows_core::IUnknown);
impl ISyncFilter {
    pub unsafe fn IsIdentical<'a, Param0: ::windows_core::IntoParam<'a, ISyncFilter>>(&self, psyncfilter: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsIdentical)(::windows_core::Interface::as_raw(self), psyncfilter.into_param().abi()).ok()
    }
    pub unsafe fn Serialize(&self, pbsyncfilter: *mut u8, pcbsyncfilter: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Serialize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbsyncfilter), ::core::mem::transmute(pcbsyncfilter)).ok()
    }
}
impl ::core::convert::From<ISyncFilter> for ::windows_core::IUnknown {
    fn from(value: ISyncFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncFilter> for ::windows_core::IUnknown {
    fn from(value: &ISyncFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISyncFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISyncFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISyncFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncFilter {}
impl ::core::fmt::Debug for ISyncFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISyncFilter {
    type Vtable = ISyncFilter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x087a3f15_0fcb_44c1_9639_53c14e2b5506);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncFilter_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub IsIdentical: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psyncfilter: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Serialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsyncfilter: *mut u8, pcbsyncfilter: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISyncFilterDeserializer(::windows_core::IUnknown);
impl ISyncFilterDeserializer {
    pub unsafe fn DeserializeSyncFilter(&self, pbsyncfilter: *const u8, dwcbsyncfilter: u32) -> ::windows_core::Result<ISyncFilter> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).DeserializeSyncFilter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbsyncfilter), ::core::mem::transmute(dwcbsyncfilter), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncFilter>(result__)
    }
}
impl ::core::convert::From<ISyncFilterDeserializer> for ::windows_core::IUnknown {
    fn from(value: ISyncFilterDeserializer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncFilterDeserializer> for ::windows_core::IUnknown {
    fn from(value: &ISyncFilterDeserializer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISyncFilterDeserializer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISyncFilterDeserializer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISyncFilterDeserializer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncFilterDeserializer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncFilterDeserializer {}
impl ::core::fmt::Debug for ISyncFilterDeserializer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncFilterDeserializer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISyncFilterDeserializer {
    type Vtable = ISyncFilterDeserializer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb45b7a72_e5c7_46be_9c82_77b8b15dab8a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncFilterDeserializer_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub DeserializeSyncFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsyncfilter: *const u8, dwcbsyncfilter: u32, ppisyncfilter: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISyncFilterInfo(::windows_core::IUnknown);
impl ISyncFilterInfo {
    pub unsafe fn Serialize(&self, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Serialize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbbuffer), ::core::mem::transmute(pcbbuffer)).ok()
    }
}
impl ::core::convert::From<ISyncFilterInfo> for ::windows_core::IUnknown {
    fn from(value: ISyncFilterInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncFilterInfo> for ::windows_core::IUnknown {
    fn from(value: &ISyncFilterInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISyncFilterInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISyncFilterInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISyncFilterInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncFilterInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncFilterInfo {}
impl ::core::fmt::Debug for ISyncFilterInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncFilterInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISyncFilterInfo {
    type Vtable = ISyncFilterInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x794eaaf8_3f2e_47e6_9728_17e6fcf94cb7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncFilterInfo_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Serialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISyncFilterInfo2(::windows_core::IUnknown);
impl ISyncFilterInfo2 {
    pub unsafe fn Serialize(&self, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Serialize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbbuffer), ::core::mem::transmute(pcbbuffer)).ok()
    }
    pub unsafe fn GetFlags(&self, pdwflags: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwflags)).ok()
    }
}
impl ::core::convert::From<ISyncFilterInfo2> for ::windows_core::IUnknown {
    fn from(value: ISyncFilterInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncFilterInfo2> for ::windows_core::IUnknown {
    fn from(value: &ISyncFilterInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISyncFilterInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISyncFilterInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISyncFilterInfo2> for ISyncFilterInfo {
    fn from(value: ISyncFilterInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncFilterInfo2> for ISyncFilterInfo {
    fn from(value: &ISyncFilterInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyncFilterInfo> for ISyncFilterInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, ISyncFilterInfo> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyncFilterInfo> for &'a ISyncFilterInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, ISyncFilterInfo> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISyncFilterInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncFilterInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncFilterInfo2 {}
impl ::core::fmt::Debug for ISyncFilterInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncFilterInfo2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISyncFilterInfo2 {
    type Vtable = ISyncFilterInfo2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x19b394ba_e3d0_468c_934d_321968b2ab34);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncFilterInfo2_Vtbl {
    pub base__: ISyncFilterInfo_Vtbl,
    pub GetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISyncFullEnumerationChange(::windows_core::IUnknown);
impl ISyncFullEnumerationChange {
    pub unsafe fn GetLearnedKnowledgeAfterRecoveryComplete(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetLearnedKnowledgeAfterRecoveryComplete)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetLearnedForgottenKnowledge(&self) -> ::windows_core::Result<IForgottenKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetLearnedForgottenKnowledge)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IForgottenKnowledge>(result__)
    }
}
impl ::core::convert::From<ISyncFullEnumerationChange> for ::windows_core::IUnknown {
    fn from(value: ISyncFullEnumerationChange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncFullEnumerationChange> for ::windows_core::IUnknown {
    fn from(value: &ISyncFullEnumerationChange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISyncFullEnumerationChange {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISyncFullEnumerationChange {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISyncFullEnumerationChange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncFullEnumerationChange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncFullEnumerationChange {}
impl ::core::fmt::Debug for ISyncFullEnumerationChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncFullEnumerationChange").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISyncFullEnumerationChange {
    type Vtable = ISyncFullEnumerationChange_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9785e0bd_bdff_40c4_98c5_b34b2f1991b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncFullEnumerationChange_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetLearnedKnowledgeAfterRecoveryComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplearnedknowledge: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetLearnedForgottenKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplearnedforgottenknowledge: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISyncFullEnumerationChangeBatch(::windows_core::IUnknown);
impl ISyncFullEnumerationChangeBatch {
    pub unsafe fn GetChangeEnumerator(&self) -> ::windows_core::Result<IEnumSyncChanges> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetChangeEnumerator)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumSyncChanges>(result__)
    }
    pub unsafe fn GetIsLastBatch(&self, pflastbatch: *mut ::win32_foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetIsLastBatch)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pflastbatch)).ok()
    }
    pub unsafe fn GetWorkEstimateForBatch(&self, pdwworkforbatch: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetWorkEstimateForBatch)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwworkforbatch)).ok()
    }
    pub unsafe fn GetRemainingWorkEstimateForSession(&self, pdwremainingworkforsession: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetRemainingWorkEstimateForSession)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwremainingworkforsession)).ok()
    }
    pub unsafe fn BeginOrderedGroup(&self, pblowerbound: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.BeginOrderedGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pblowerbound)).ok()
    }
    pub unsafe fn EndOrderedGroup<'a, Param1: ::windows_core::IntoParam<'a, ISyncKnowledge>>(&self, pbupperbound: *const u8, pmadewithknowledge: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EndOrderedGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbupperbound), pmadewithknowledge.into_param().abi()).ok()
    }
    pub unsafe fn AddItemMetadataToGroup(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32) -> ::windows_core::Result<ISyncChangeBuilder> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.AddItemMetadataToGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbownerreplicaid), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pchangeversion), ::core::mem::transmute(pcreationversion), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwworkforchange), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChangeBuilder>(result__)
    }
    pub unsafe fn GetLearnedKnowledge(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetLearnedKnowledge)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetPrerequisiteKnowledge(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPrerequisiteKnowledge)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetSourceForgottenKnowledge(&self) -> ::windows_core::Result<IForgottenKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSourceForgottenKnowledge)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IForgottenKnowledge>(result__)
    }
    pub unsafe fn SetLastBatch(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetLastBatch)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetWorkEstimateForBatch(&self, dwworkforbatch: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetWorkEstimateForBatch)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwworkforbatch)).ok()
    }
    pub unsafe fn SetRemainingWorkEstimateForSession(&self, dwremainingworkforsession: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRemainingWorkEstimateForSession)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwremainingworkforsession)).ok()
    }
    pub unsafe fn Serialize(&self, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Serialize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbchangebatch), ::core::mem::transmute(pcbchangebatch)).ok()
    }
    pub unsafe fn GetLearnedKnowledgeAfterRecoveryComplete(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetLearnedKnowledgeAfterRecoveryComplete)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetClosedLowerBoundItemId(&self, pbclosedlowerbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetClosedLowerBoundItemId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbclosedlowerbounditemid), ::core::mem::transmute(pcbidsize)).ok()
    }
    pub unsafe fn GetClosedUpperBoundItemId(&self, pbclosedupperbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetClosedUpperBoundItemId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbclosedupperbounditemid), ::core::mem::transmute(pcbidsize)).ok()
    }
}
impl ::core::convert::From<ISyncFullEnumerationChangeBatch> for ::windows_core::IUnknown {
    fn from(value: ISyncFullEnumerationChangeBatch) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncFullEnumerationChangeBatch> for ::windows_core::IUnknown {
    fn from(value: &ISyncFullEnumerationChangeBatch) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISyncFullEnumerationChangeBatch {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISyncFullEnumerationChangeBatch {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISyncFullEnumerationChangeBatch> for ISyncChangeBatchBase {
    fn from(value: ISyncFullEnumerationChangeBatch) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncFullEnumerationChangeBatch> for ISyncChangeBatchBase {
    fn from(value: &ISyncFullEnumerationChangeBatch) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyncChangeBatchBase> for ISyncFullEnumerationChangeBatch {
    fn into_param(self) -> ::windows_core::Param<'a, ISyncChangeBatchBase> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyncChangeBatchBase> for &'a ISyncFullEnumerationChangeBatch {
    fn into_param(self) -> ::windows_core::Param<'a, ISyncChangeBatchBase> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISyncFullEnumerationChangeBatch {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncFullEnumerationChangeBatch {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncFullEnumerationChangeBatch {}
impl ::core::fmt::Debug for ISyncFullEnumerationChangeBatch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncFullEnumerationChangeBatch").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISyncFullEnumerationChangeBatch {
    type Vtable = ISyncFullEnumerationChangeBatch_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef64197d_4f44_4ea2_b355_4524713e3bed);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncFullEnumerationChangeBatch_Vtbl {
    pub base__: ISyncChangeBatchBase_Vtbl,
    pub GetLearnedKnowledgeAfterRecoveryComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplearnedknowledgeafterrecoverycomplete: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetClosedLowerBoundItemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbclosedlowerbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT,
    pub GetClosedUpperBoundItemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbclosedupperbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISyncFullEnumerationChangeBatch2(::windows_core::IUnknown);
impl ISyncFullEnumerationChangeBatch2 {
    pub unsafe fn GetChangeEnumerator(&self) -> ::windows_core::Result<IEnumSyncChanges> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetChangeEnumerator)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumSyncChanges>(result__)
    }
    pub unsafe fn GetIsLastBatch(&self, pflastbatch: *mut ::win32_foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetIsLastBatch)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pflastbatch)).ok()
    }
    pub unsafe fn GetWorkEstimateForBatch(&self, pdwworkforbatch: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetWorkEstimateForBatch)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwworkforbatch)).ok()
    }
    pub unsafe fn GetRemainingWorkEstimateForSession(&self, pdwremainingworkforsession: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetRemainingWorkEstimateForSession)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwremainingworkforsession)).ok()
    }
    pub unsafe fn BeginOrderedGroup(&self, pblowerbound: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.BeginOrderedGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pblowerbound)).ok()
    }
    pub unsafe fn EndOrderedGroup<'a, Param1: ::windows_core::IntoParam<'a, ISyncKnowledge>>(&self, pbupperbound: *const u8, pmadewithknowledge: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.EndOrderedGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbupperbound), pmadewithknowledge.into_param().abi()).ok()
    }
    pub unsafe fn AddItemMetadataToGroup(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32) -> ::windows_core::Result<ISyncChangeBuilder> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.AddItemMetadataToGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbownerreplicaid), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pchangeversion), ::core::mem::transmute(pcreationversion), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwworkforchange), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChangeBuilder>(result__)
    }
    pub unsafe fn GetLearnedKnowledge(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetLearnedKnowledge)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetPrerequisiteKnowledge(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetPrerequisiteKnowledge)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetSourceForgottenKnowledge(&self) -> ::windows_core::Result<IForgottenKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetSourceForgottenKnowledge)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IForgottenKnowledge>(result__)
    }
    pub unsafe fn SetLastBatch(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetLastBatch)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetWorkEstimateForBatch(&self, dwworkforbatch: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetWorkEstimateForBatch)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwworkforbatch)).ok()
    }
    pub unsafe fn SetRemainingWorkEstimateForSession(&self, dwremainingworkforsession: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetRemainingWorkEstimateForSession)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwremainingworkforsession)).ok()
    }
    pub unsafe fn Serialize(&self, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Serialize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbchangebatch), ::core::mem::transmute(pcbchangebatch)).ok()
    }
    pub unsafe fn GetLearnedKnowledgeAfterRecoveryComplete(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetLearnedKnowledgeAfterRecoveryComplete)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetClosedLowerBoundItemId(&self, pbclosedlowerbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetClosedLowerBoundItemId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbclosedlowerbounditemid), ::core::mem::transmute(pcbidsize)).ok()
    }
    pub unsafe fn GetClosedUpperBoundItemId(&self, pbclosedupperbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetClosedUpperBoundItemId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbclosedupperbounditemid), ::core::mem::transmute(pcbidsize)).ok()
    }
    pub unsafe fn AddMergeTombstoneMetadataToGroup(&self, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32) -> ::windows_core::Result<ISyncChangeBuilder> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).AddMergeTombstoneMetadataToGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbownerreplicaid), ::core::mem::transmute(pbwinneritemid), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pchangeversion), ::core::mem::transmute(pcreationversion), ::core::mem::transmute(dwworkforchange), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChangeBuilder>(result__)
    }
}
impl ::core::convert::From<ISyncFullEnumerationChangeBatch2> for ::windows_core::IUnknown {
    fn from(value: ISyncFullEnumerationChangeBatch2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncFullEnumerationChangeBatch2> for ::windows_core::IUnknown {
    fn from(value: &ISyncFullEnumerationChangeBatch2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISyncFullEnumerationChangeBatch2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISyncFullEnumerationChangeBatch2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISyncFullEnumerationChangeBatch2> for ISyncChangeBatchBase {
    fn from(value: ISyncFullEnumerationChangeBatch2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncFullEnumerationChangeBatch2> for ISyncChangeBatchBase {
    fn from(value: &ISyncFullEnumerationChangeBatch2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyncChangeBatchBase> for ISyncFullEnumerationChangeBatch2 {
    fn into_param(self) -> ::windows_core::Param<'a, ISyncChangeBatchBase> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyncChangeBatchBase> for &'a ISyncFullEnumerationChangeBatch2 {
    fn into_param(self) -> ::windows_core::Param<'a, ISyncChangeBatchBase> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISyncFullEnumerationChangeBatch2> for ISyncFullEnumerationChangeBatch {
    fn from(value: ISyncFullEnumerationChangeBatch2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncFullEnumerationChangeBatch2> for ISyncFullEnumerationChangeBatch {
    fn from(value: &ISyncFullEnumerationChangeBatch2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyncFullEnumerationChangeBatch> for ISyncFullEnumerationChangeBatch2 {
    fn into_param(self) -> ::windows_core::Param<'a, ISyncFullEnumerationChangeBatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyncFullEnumerationChangeBatch> for &'a ISyncFullEnumerationChangeBatch2 {
    fn into_param(self) -> ::windows_core::Param<'a, ISyncFullEnumerationChangeBatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISyncFullEnumerationChangeBatch2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncFullEnumerationChangeBatch2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncFullEnumerationChangeBatch2 {}
impl ::core::fmt::Debug for ISyncFullEnumerationChangeBatch2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncFullEnumerationChangeBatch2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISyncFullEnumerationChangeBatch2 {
    type Vtable = ISyncFullEnumerationChangeBatch2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe06449f4_a205_4b65_9724_01b22101eec1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncFullEnumerationChangeBatch2_Vtbl {
    pub base__: ISyncFullEnumerationChangeBatch_Vtbl,
    pub AddMergeTombstoneMetadataToGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, ppchangebuilder: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISyncKnowledge(::windows_core::IUnknown);
impl ISyncKnowledge {
    pub unsafe fn GetOwnerReplicaId(&self, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetOwnerReplicaId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbreplicaid), ::core::mem::transmute(pcbidsize)).ok()
    }
    pub unsafe fn Serialize<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fserializereplicakeymap: Param0, pbknowledge: *mut u8, pcbknowledge: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Serialize)(::windows_core::Interface::as_raw(self), fserializereplicakeymap.into_param().abi(), ::core::mem::transmute(pbknowledge), ::core::mem::transmute(pcbknowledge)).ok()
    }
    pub unsafe fn SetLocalTickCount(&self, ulltickcount: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLocalTickCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ulltickcount)).ok()
    }
    pub unsafe fn ContainsChange(&self, pbversionownerreplicaid: *const u8, pgiditemid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ContainsChange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbversionownerreplicaid), ::core::mem::transmute(pgiditemid), ::core::mem::transmute(psyncversion)).ok()
    }
    pub unsafe fn ContainsChangeUnit(&self, pbversionownerreplicaid: *const u8, pbitemid: *const u8, pbchangeunitid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ContainsChangeUnit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbversionownerreplicaid), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pbchangeunitid), ::core::mem::transmute(psyncversion)).ok()
    }
    pub unsafe fn GetScopeVector(&self, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetScopeVector)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn GetReplicaKeyMap(&self) -> ::windows_core::Result<IReplicaKeyMap> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetReplicaKeyMap)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IReplicaKeyMap>(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn ConvertVersion<'a, Param0: ::windows_core::IntoParam<'a, ISyncKnowledge>>(&self, pknowledgein: Param0, pbcurrentownerid: *const u8, pversionin: *const SYNC_VERSION, pbnewownerid: *mut u8, pcbidsize: *mut u32, pversionout: *mut SYNC_VERSION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ConvertVersion)(::windows_core::Interface::as_raw(self), pknowledgein.into_param().abi(), ::core::mem::transmute(pbcurrentownerid), ::core::mem::transmute(pversionin), ::core::mem::transmute(pbnewownerid), ::core::mem::transmute(pcbidsize), ::core::mem::transmute(pversionout)).ok()
    }
    pub unsafe fn MapRemoteToLocal<'a, Param0: ::windows_core::IntoParam<'a, ISyncKnowledge>>(&self, premoteknowledge: Param0) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).MapRemoteToLocal)(::windows_core::Interface::as_raw(self), premoteknowledge.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn Union<'a, Param0: ::windows_core::IntoParam<'a, ISyncKnowledge>>(&self, pknowledge: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Union)(::windows_core::Interface::as_raw(self), pknowledge.into_param().abi()).ok()
    }
    pub unsafe fn ProjectOntoItem(&self, pbitemid: *const u8) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ProjectOntoItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn ProjectOntoChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ProjectOntoChangeUnit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pbchangeunitid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn ProjectOntoRange(&self, psrngsyncrange: *const SYNC_RANGE) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ProjectOntoRange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(psrngsyncrange), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn ExcludeItem(&self, pbitemid: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ExcludeItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbitemid)).ok()
    }
    pub unsafe fn ExcludeChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ExcludeChangeUnit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pbchangeunitid)).ok()
    }
    pub unsafe fn ContainsKnowledge<'a, Param0: ::windows_core::IntoParam<'a, ISyncKnowledge>>(&self, pknowledge: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ContainsKnowledge)(::windows_core::Interface::as_raw(self), pknowledge.into_param().abi()).ok()
    }
    pub unsafe fn FindMinTickCountForReplica(&self, pbreplicaid: *const u8, pullreplicatickcount: *mut u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FindMinTickCountForReplica)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbreplicaid), ::core::mem::transmute(pullreplicatickcount)).ok()
    }
    pub unsafe fn GetRangeExceptions(&self, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRangeExceptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn GetSingleItemExceptions(&self, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSingleItemExceptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn GetChangeUnitExceptions(&self, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetChangeUnitExceptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn FindClockVectorForItem(&self, pbitemid: *const u8, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FindClockVectorForItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn FindClockVectorForChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FindClockVectorForChangeUnit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pbchangeunitid), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn GetVersion(&self, pdwversion: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwversion)).ok()
    }
}
impl ::core::convert::From<ISyncKnowledge> for ::windows_core::IUnknown {
    fn from(value: ISyncKnowledge) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncKnowledge> for ::windows_core::IUnknown {
    fn from(value: &ISyncKnowledge) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISyncKnowledge {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISyncKnowledge {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISyncKnowledge {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncKnowledge {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncKnowledge {}
impl ::core::fmt::Debug for ISyncKnowledge {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncKnowledge").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISyncKnowledge {
    type Vtable = ISyncKnowledge_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x615bbb53_c945_4203_bf4b_2cb65919a0aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncKnowledge_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetOwnerReplicaId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT,
    pub Serialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fserializereplicakeymap: ::win32_foundation::BOOL, pbknowledge: *mut u8, pcbknowledge: *mut u32) -> ::windows_core::HRESULT,
    pub SetLocalTickCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulltickcount: u64) -> ::windows_core::HRESULT,
    pub ContainsChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbversionownerreplicaid: *const u8, pgiditemid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows_core::HRESULT,
    pub ContainsChangeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbversionownerreplicaid: *const u8, pbitemid: *const u8, pbchangeunitid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows_core::HRESULT,
    pub GetScopeVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetReplicaKeyMap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppreplicakeymap: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppclonedknowledge: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ConvertVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pknowledgein: ::windows_core::RawPtr, pbcurrentownerid: *const u8, pversionin: *const SYNC_VERSION, pbnewownerid: *mut u8, pcbidsize: *mut u32, pversionout: *mut SYNC_VERSION) -> ::windows_core::HRESULT,
    pub MapRemoteToLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, premoteknowledge: ::windows_core::RawPtr, ppmappedknowledge: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Union: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pknowledge: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ProjectOntoItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitemid: *const u8, ppknowledgeout: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ProjectOntoChangeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8, ppknowledgeout: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ProjectOntoRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psrngsyncrange: *const SYNC_RANGE, ppknowledgeout: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ExcludeItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitemid: *const u8) -> ::windows_core::HRESULT,
    pub ExcludeChangeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows_core::HRESULT,
    pub ContainsKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pknowledge: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FindMinTickCountForReplica: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbreplicaid: *const u8, pullreplicatickcount: *mut u64) -> ::windows_core::HRESULT,
    pub GetRangeExceptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetSingleItemExceptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetChangeUnitExceptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FindClockVectorForItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitemid: *const u8, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FindClockVectorForChangeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISyncKnowledge2(::windows_core::IUnknown);
impl ISyncKnowledge2 {
    pub unsafe fn GetOwnerReplicaId(&self, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetOwnerReplicaId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbreplicaid), ::core::mem::transmute(pcbidsize)).ok()
    }
    pub unsafe fn Serialize<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fserializereplicakeymap: Param0, pbknowledge: *mut u8, pcbknowledge: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Serialize)(::windows_core::Interface::as_raw(self), fserializereplicakeymap.into_param().abi(), ::core::mem::transmute(pbknowledge), ::core::mem::transmute(pcbknowledge)).ok()
    }
    pub unsafe fn SetLocalTickCount(&self, ulltickcount: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetLocalTickCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ulltickcount)).ok()
    }
    pub unsafe fn ContainsChange(&self, pbversionownerreplicaid: *const u8, pgiditemid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ContainsChange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbversionownerreplicaid), ::core::mem::transmute(pgiditemid), ::core::mem::transmute(psyncversion)).ok()
    }
    pub unsafe fn ContainsChangeUnit(&self, pbversionownerreplicaid: *const u8, pbitemid: *const u8, pbchangeunitid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ContainsChangeUnit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbversionownerreplicaid), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pbchangeunitid), ::core::mem::transmute(psyncversion)).ok()
    }
    pub unsafe fn GetScopeVector(&self, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetScopeVector)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn GetReplicaKeyMap(&self) -> ::windows_core::Result<IReplicaKeyMap> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetReplicaKeyMap)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IReplicaKeyMap>(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn ConvertVersion<'a, Param0: ::windows_core::IntoParam<'a, ISyncKnowledge>>(&self, pknowledgein: Param0, pbcurrentownerid: *const u8, pversionin: *const SYNC_VERSION, pbnewownerid: *mut u8, pcbidsize: *mut u32, pversionout: *mut SYNC_VERSION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ConvertVersion)(::windows_core::Interface::as_raw(self), pknowledgein.into_param().abi(), ::core::mem::transmute(pbcurrentownerid), ::core::mem::transmute(pversionin), ::core::mem::transmute(pbnewownerid), ::core::mem::transmute(pcbidsize), ::core::mem::transmute(pversionout)).ok()
    }
    pub unsafe fn MapRemoteToLocal<'a, Param0: ::windows_core::IntoParam<'a, ISyncKnowledge>>(&self, premoteknowledge: Param0) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.MapRemoteToLocal)(::windows_core::Interface::as_raw(self), premoteknowledge.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn Union<'a, Param0: ::windows_core::IntoParam<'a, ISyncKnowledge>>(&self, pknowledge: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Union)(::windows_core::Interface::as_raw(self), pknowledge.into_param().abi()).ok()
    }
    pub unsafe fn ProjectOntoItem(&self, pbitemid: *const u8) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ProjectOntoItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn ProjectOntoChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ProjectOntoChangeUnit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pbchangeunitid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn ProjectOntoRange(&self, psrngsyncrange: *const SYNC_RANGE) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ProjectOntoRange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(psrngsyncrange), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn ExcludeItem(&self, pbitemid: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ExcludeItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbitemid)).ok()
    }
    pub unsafe fn ExcludeChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ExcludeChangeUnit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pbchangeunitid)).ok()
    }
    pub unsafe fn ContainsKnowledge<'a, Param0: ::windows_core::IntoParam<'a, ISyncKnowledge>>(&self, pknowledge: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ContainsKnowledge)(::windows_core::Interface::as_raw(self), pknowledge.into_param().abi()).ok()
    }
    pub unsafe fn FindMinTickCountForReplica(&self, pbreplicaid: *const u8, pullreplicatickcount: *mut u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.FindMinTickCountForReplica)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbreplicaid), ::core::mem::transmute(pullreplicatickcount)).ok()
    }
    pub unsafe fn GetRangeExceptions(&self, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetRangeExceptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn GetSingleItemExceptions(&self, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetSingleItemExceptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn GetChangeUnitExceptions(&self, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetChangeUnitExceptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn FindClockVectorForItem(&self, pbitemid: *const u8, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.FindClockVectorForItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn FindClockVectorForChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.FindClockVectorForChangeUnit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pbchangeunitid), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn GetVersion(&self, pdwversion: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwversion)).ok()
    }
    pub unsafe fn GetIdParameters(&self, pidparameters: *mut ID_PARAMETERS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetIdParameters)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pidparameters)).ok()
    }
    pub unsafe fn ProjectOntoColumnSet(&self, ppcolumns: *const *const u8, count: u32) -> ::windows_core::Result<ISyncKnowledge2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ProjectOntoColumnSet)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppcolumns), ::core::mem::transmute(count), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge2>(result__)
    }
    pub unsafe fn SerializeWithOptions(&self, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SerializeWithOptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(targetformatversion), ::core::mem::transmute(dwflags), ::core::mem::transmute(pbbuffer), ::core::mem::transmute(pdwserializedsize)).ok()
    }
    pub unsafe fn GetLowestUncontainedId<'a, Param0: ::windows_core::IntoParam<'a, ISyncKnowledge2>>(&self, pisyncknowledge: Param0, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetLowestUncontainedId)(::windows_core::Interface::as_raw(self), pisyncknowledge.into_param().abi(), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pcbitemidsize)).ok()
    }
    pub unsafe fn GetInspector(&self, riid: *const ::windows_core::GUID, ppiinspector: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetInspector)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppiinspector)).ok()
    }
    pub unsafe fn GetMinimumSupportedVersion(&self, pversion: *mut SYNC_SERIALIZATION_VERSION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetMinimumSupportedVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pversion)).ok()
    }
    pub unsafe fn GetStatistics(&self, which: SYNC_STATISTICS, pvalue: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetStatistics)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(which), ::core::mem::transmute(pvalue)).ok()
    }
    pub unsafe fn ContainsKnowledgeForItem<'a, Param0: ::windows_core::IntoParam<'a, ISyncKnowledge>>(&self, pknowledge: Param0, pbitemid: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ContainsKnowledgeForItem)(::windows_core::Interface::as_raw(self), pknowledge.into_param().abi(), ::core::mem::transmute(pbitemid)).ok()
    }
    pub unsafe fn ContainsKnowledgeForChangeUnit<'a, Param0: ::windows_core::IntoParam<'a, ISyncKnowledge>>(&self, pknowledge: Param0, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ContainsKnowledgeForChangeUnit)(::windows_core::Interface::as_raw(self), pknowledge.into_param().abi(), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pbchangeunitid)).ok()
    }
    pub unsafe fn ProjectOntoKnowledgeWithPrerequisite<'a, Param0: ::windows_core::IntoParam<'a, ISyncKnowledge>, Param1: ::windows_core::IntoParam<'a, ISyncKnowledge>>(&self, pprerequisiteknowledge: Param0, ptemplateknowledge: Param1) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ProjectOntoKnowledgeWithPrerequisite)(::windows_core::Interface::as_raw(self), pprerequisiteknowledge.into_param().abi(), ptemplateknowledge.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn Complement<'a, Param0: ::windows_core::IntoParam<'a, ISyncKnowledge>>(&self, psyncknowledge: Param0) -> ::windows_core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Complement)(::windows_core::Interface::as_raw(self), psyncknowledge.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn IntersectsWithKnowledge<'a, Param0: ::windows_core::IntoParam<'a, ISyncKnowledge>>(&self, psyncknowledge: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IntersectsWithKnowledge)(::windows_core::Interface::as_raw(self), psyncknowledge.into_param().abi()).ok()
    }
    pub unsafe fn GetKnowledgeCookie(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).GetKnowledgeCookie)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn CompareToKnowledgeCookie<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, pknowledgecookie: Param0, presult: *mut KNOWLEDGE_COOKIE_COMPARISON_RESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CompareToKnowledgeCookie)(::windows_core::Interface::as_raw(self), pknowledgecookie.into_param().abi(), ::core::mem::transmute(presult)).ok()
    }
}
impl ::core::convert::From<ISyncKnowledge2> for ::windows_core::IUnknown {
    fn from(value: ISyncKnowledge2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncKnowledge2> for ::windows_core::IUnknown {
    fn from(value: &ISyncKnowledge2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISyncKnowledge2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISyncKnowledge2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISyncKnowledge2> for ISyncKnowledge {
    fn from(value: ISyncKnowledge2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncKnowledge2> for ISyncKnowledge {
    fn from(value: &ISyncKnowledge2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyncKnowledge> for ISyncKnowledge2 {
    fn into_param(self) -> ::windows_core::Param<'a, ISyncKnowledge> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyncKnowledge> for &'a ISyncKnowledge2 {
    fn into_param(self) -> ::windows_core::Param<'a, ISyncKnowledge> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISyncKnowledge2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncKnowledge2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncKnowledge2 {}
impl ::core::fmt::Debug for ISyncKnowledge2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncKnowledge2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISyncKnowledge2 {
    type Vtable = ISyncKnowledge2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xed0addc0_3b4b_46a1_9a45_45661d2114c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncKnowledge2_Vtbl {
    pub base__: ISyncKnowledge_Vtbl,
    pub GetIdParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> ::windows_core::HRESULT,
    pub ProjectOntoColumnSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolumns: *const *const u8, count: u32, ppiknowledgeout: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SerializeWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> ::windows_core::HRESULT,
    pub GetLowestUncontainedId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisyncknowledge: ::windows_core::RawPtr, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> ::windows_core::HRESULT,
    pub GetInspector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppiinspector: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetMinimumSupportedVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pversion: *mut SYNC_SERIALIZATION_VERSION) -> ::windows_core::HRESULT,
    pub GetStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, which: SYNC_STATISTICS, pvalue: *mut u32) -> ::windows_core::HRESULT,
    pub ContainsKnowledgeForItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pknowledge: ::windows_core::RawPtr, pbitemid: *const u8) -> ::windows_core::HRESULT,
    pub ContainsKnowledgeForChangeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pknowledge: ::windows_core::RawPtr, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows_core::HRESULT,
    pub ProjectOntoKnowledgeWithPrerequisite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprerequisiteknowledge: ::windows_core::RawPtr, ptemplateknowledge: ::windows_core::RawPtr, ppprojectedknowledge: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Complement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psyncknowledge: ::windows_core::RawPtr, ppcomplementedknowledge: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IntersectsWithKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psyncknowledge: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetKnowledgeCookie: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppknowledgecookie: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CompareToKnowledgeCookie: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pknowledgecookie: *mut ::core::ffi::c_void, presult: *mut KNOWLEDGE_COOKIE_COMPARISON_RESULT) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISyncMergeTombstoneChange(::windows_core::IUnknown);
impl ISyncMergeTombstoneChange {
    pub unsafe fn GetWinnerItemId(&self, pbwinneritemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetWinnerItemId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbwinneritemid), ::core::mem::transmute(pcbidsize)).ok()
    }
}
impl ::core::convert::From<ISyncMergeTombstoneChange> for ::windows_core::IUnknown {
    fn from(value: ISyncMergeTombstoneChange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncMergeTombstoneChange> for ::windows_core::IUnknown {
    fn from(value: &ISyncMergeTombstoneChange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISyncMergeTombstoneChange {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISyncMergeTombstoneChange {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISyncMergeTombstoneChange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncMergeTombstoneChange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncMergeTombstoneChange {}
impl ::core::fmt::Debug for ISyncMergeTombstoneChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncMergeTombstoneChange").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISyncMergeTombstoneChange {
    type Vtable = ISyncMergeTombstoneChange_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6ec62597_0903_484c_ad61_36d6e938f47b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncMergeTombstoneChange_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetWinnerItemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbwinneritemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISyncProvider(::windows_core::IUnknown);
impl ISyncProvider {
    pub unsafe fn GetIdParameters(&self, pidparameters: *mut ID_PARAMETERS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetIdParameters)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pidparameters)).ok()
    }
}
impl ::core::convert::From<ISyncProvider> for ::windows_core::IUnknown {
    fn from(value: ISyncProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncProvider> for ::windows_core::IUnknown {
    fn from(value: &ISyncProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISyncProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISyncProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISyncProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncProvider {}
impl ::core::fmt::Debug for ISyncProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISyncProvider {
    type Vtable = ISyncProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8f657056_2bce_4a17_8c68_c7bb7898b56f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncProvider_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetIdParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISyncProviderConfigUI(::windows_core::IUnknown);
impl ISyncProviderConfigUI {
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Init<'a, Param2: ::windows_core::IntoParam<'a, ::win32_ui::Shell::PropertiesSystem::IPropertyStore>>(&self, pguidinstanceid: *const ::windows_core::GUID, pguidcontenttype: *const ::windows_core::GUID, pconfigurationproperties: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Init)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pguidinstanceid), ::core::mem::transmute(pguidcontenttype), pconfigurationproperties.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetRegisteredProperties(&self) -> ::windows_core::Result<::win32_ui::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetRegisteredProperties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_ui::Shell::PropertiesSystem::IPropertyStore>(result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn CreateAndRegisterNewSyncProvider<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, hwndparent: Param0, punkcontext: Param1) -> ::windows_core::Result<ISyncProviderInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateAndRegisterNewSyncProvider)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), punkcontext.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncProviderInfo>(result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn ModifySyncProvider<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param2: ::windows_core::IntoParam<'a, ISyncProviderInfo>>(&self, hwndparent: Param0, punkcontext: Param1, pproviderinfo: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ModifySyncProvider)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), punkcontext.into_param().abi(), pproviderinfo.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ISyncProviderConfigUI> for ::windows_core::IUnknown {
    fn from(value: ISyncProviderConfigUI) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncProviderConfigUI> for ::windows_core::IUnknown {
    fn from(value: &ISyncProviderConfigUI) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISyncProviderConfigUI {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISyncProviderConfigUI {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISyncProviderConfigUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncProviderConfigUI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncProviderConfigUI {}
impl ::core::fmt::Debug for ISyncProviderConfigUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncProviderConfigUI").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISyncProviderConfigUI {
    type Vtable = ISyncProviderConfigUI_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7b0705f6_cbcd_4071_ab05_3bdc364d4a0c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncProviderConfigUI_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Init: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows_core::GUID, pguidcontenttype: *const ::windows_core::GUID, pconfigurationproperties: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Init: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetRegisteredProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconfiguiproperties: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetRegisteredProperties: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub CreateAndRegisterNewSyncProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: ::win32_foundation::HWND, punkcontext: *mut ::core::ffi::c_void, ppproviderinfo: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    CreateAndRegisterNewSyncProvider: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub ModifySyncProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: ::win32_foundation::HWND, punkcontext: *mut ::core::ffi::c_void, pproviderinfo: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    ModifySyncProvider: usize,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[repr(transparent)]
pub struct ISyncProviderConfigUIInfo(::windows_core::IUnknown);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ISyncProviderConfigUIInfo {
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetAt(&self, iprop: u32) -> ::windows_core::Result<::win32_ui::Shell::PropertiesSystem::PROPERTYKEY> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_ui::Shell::PropertiesSystem::PROPERTYKEY>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetAt)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iprop), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_ui::Shell::PropertiesSystem::PROPERTYKEY>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetValue(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<super::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn SetValue(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, propvar: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(propvar)).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetSyncProviderConfigUI(&self, dwclscontext: u32) -> ::windows_core::Result<ISyncProviderConfigUI> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSyncProviderConfigUI)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwclscontext), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncProviderConfigUI>(result__)
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::convert::From<ISyncProviderConfigUIInfo> for ::windows_core::IUnknown {
    fn from(value: ISyncProviderConfigUIInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::convert::From<&ISyncProviderConfigUIInfo> for ::windows_core::IUnknown {
    fn from(value: &ISyncProviderConfigUIInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISyncProviderConfigUIInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISyncProviderConfigUIInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::convert::From<ISyncProviderConfigUIInfo> for ::win32_ui::Shell::PropertiesSystem::IPropertyStore {
    fn from(value: ISyncProviderConfigUIInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::convert::From<&ISyncProviderConfigUIInfo> for ::win32_ui::Shell::PropertiesSystem::IPropertyStore {
    fn from(value: &ISyncProviderConfigUIInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_ui::Shell::PropertiesSystem::IPropertyStore> for ISyncProviderConfigUIInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_ui::Shell::PropertiesSystem::IPropertyStore> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_ui::Shell::PropertiesSystem::IPropertyStore> for &'a ISyncProviderConfigUIInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_ui::Shell::PropertiesSystem::IPropertyStore> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for ISyncProviderConfigUIInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::PartialEq for ISyncProviderConfigUIInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::Eq for ISyncProviderConfigUIInfo {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::fmt::Debug for ISyncProviderConfigUIInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncProviderConfigUIInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
unsafe impl ::windows_core::Interface for ISyncProviderConfigUIInfo {
    type Vtable = ISyncProviderConfigUIInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x214141ae_33d7_4d8d_8e37_f227e880ce50);
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[repr(C)]
#[doc(hidden)]
pub struct ISyncProviderConfigUIInfo_Vtbl {
    pub base__: ::win32_ui::Shell::PropertiesSystem::IPropertyStore_Vtbl,
    pub GetSyncProviderConfigUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwclscontext: u32, ppsyncproviderconfigui: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[repr(transparent)]
pub struct ISyncProviderInfo(::windows_core::IUnknown);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ISyncProviderInfo {
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetAt(&self, iprop: u32) -> ::windows_core::Result<::win32_ui::Shell::PropertiesSystem::PROPERTYKEY> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_ui::Shell::PropertiesSystem::PROPERTYKEY>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetAt)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iprop), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_ui::Shell::PropertiesSystem::PROPERTYKEY>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetValue(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<super::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn SetValue(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, propvar: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(propvar)).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetSyncProvider(&self, dwclscontext: u32) -> ::windows_core::Result<IRegisteredSyncProvider> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSyncProvider)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwclscontext), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRegisteredSyncProvider>(result__)
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::convert::From<ISyncProviderInfo> for ::windows_core::IUnknown {
    fn from(value: ISyncProviderInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::convert::From<&ISyncProviderInfo> for ::windows_core::IUnknown {
    fn from(value: &ISyncProviderInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISyncProviderInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISyncProviderInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::convert::From<ISyncProviderInfo> for ::win32_ui::Shell::PropertiesSystem::IPropertyStore {
    fn from(value: ISyncProviderInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::convert::From<&ISyncProviderInfo> for ::win32_ui::Shell::PropertiesSystem::IPropertyStore {
    fn from(value: &ISyncProviderInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_ui::Shell::PropertiesSystem::IPropertyStore> for ISyncProviderInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_ui::Shell::PropertiesSystem::IPropertyStore> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_ui::Shell::PropertiesSystem::IPropertyStore> for &'a ISyncProviderInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_ui::Shell::PropertiesSystem::IPropertyStore> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for ISyncProviderInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::PartialEq for ISyncProviderInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::Eq for ISyncProviderInfo {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::fmt::Debug for ISyncProviderInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncProviderInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
unsafe impl ::windows_core::Interface for ISyncProviderInfo {
    type Vtable = ISyncProviderInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1ee135de_88a4_4504_b0d0_f7920d7e5ba6);
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[repr(C)]
#[doc(hidden)]
pub struct ISyncProviderInfo_Vtbl {
    pub base__: ::win32_ui::Shell::PropertiesSystem::IPropertyStore_Vtbl,
    pub GetSyncProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwclscontext: u32, ppsyncprovider: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISyncProviderRegistration(::windows_core::IUnknown);
impl ISyncProviderRegistration {
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn CreateSyncProviderConfigUIRegistrationInstance(&self, pconfiguiconfig: *const SyncProviderConfigUIConfiguration) -> ::windows_core::Result<ISyncProviderConfigUIInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateSyncProviderConfigUIRegistrationInstance)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pconfiguiconfig), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncProviderConfigUIInfo>(result__)
    }
    pub unsafe fn UnregisterSyncProviderConfigUI(&self, pguidinstanceid: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnregisterSyncProviderConfigUI)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pguidinstanceid)).ok()
    }
    pub unsafe fn EnumerateSyncProviderConfigUIs(&self, pguidcontenttype: *const ::windows_core::GUID, dwsupportedarchitecture: u32) -> ::windows_core::Result<IEnumSyncProviderConfigUIInfos> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumerateSyncProviderConfigUIs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pguidcontenttype), ::core::mem::transmute(dwsupportedarchitecture), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumSyncProviderConfigUIInfos>(result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn CreateSyncProviderRegistrationInstance(&self, pproviderconfiguration: *const SyncProviderConfiguration) -> ::windows_core::Result<ISyncProviderInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateSyncProviderRegistrationInstance)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pproviderconfiguration), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncProviderInfo>(result__)
    }
    pub unsafe fn UnregisterSyncProvider(&self, pguidinstanceid: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnregisterSyncProvider)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pguidinstanceid)).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetSyncProviderConfigUIInfoforProvider(&self, pguidproviderinstanceid: *const ::windows_core::GUID) -> ::windows_core::Result<ISyncProviderConfigUIInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSyncProviderConfigUIInfoforProvider)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pguidproviderinstanceid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncProviderConfigUIInfo>(result__)
    }
    pub unsafe fn EnumerateSyncProviders(&self, pguidcontenttype: *const ::windows_core::GUID, dwstateflagstofiltermask: u32, dwstateflagstofilter: u32, refproviderclsid: *const ::windows_core::GUID, dwsupportedarchitecture: u32) -> ::windows_core::Result<IEnumSyncProviderInfos> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumerateSyncProviders)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pguidcontenttype), ::core::mem::transmute(dwstateflagstofiltermask), ::core::mem::transmute(dwstateflagstofilter), ::core::mem::transmute(refproviderclsid), ::core::mem::transmute(dwsupportedarchitecture), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumSyncProviderInfos>(result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetSyncProviderInfo(&self, pguidinstanceid: *const ::windows_core::GUID) -> ::windows_core::Result<ISyncProviderInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSyncProviderInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pguidinstanceid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncProviderInfo>(result__)
    }
    pub unsafe fn GetSyncProviderFromInstanceId(&self, pguidinstanceid: *const ::windows_core::GUID, dwclscontext: u32) -> ::windows_core::Result<IRegisteredSyncProvider> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSyncProviderFromInstanceId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pguidinstanceid), ::core::mem::transmute(dwclscontext), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRegisteredSyncProvider>(result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetSyncProviderConfigUIInfo(&self, pguidinstanceid: *const ::windows_core::GUID) -> ::windows_core::Result<ISyncProviderConfigUIInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSyncProviderConfigUIInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pguidinstanceid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncProviderConfigUIInfo>(result__)
    }
    pub unsafe fn GetSyncProviderConfigUIFromInstanceId(&self, pguidinstanceid: *const ::windows_core::GUID, dwclscontext: u32) -> ::windows_core::Result<ISyncProviderConfigUI> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSyncProviderConfigUIFromInstanceId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pguidinstanceid), ::core::mem::transmute(dwclscontext), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncProviderConfigUI>(result__)
    }
    pub unsafe fn GetSyncProviderState(&self, pguidinstanceid: *const ::windows_core::GUID) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetSyncProviderState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pguidinstanceid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetSyncProviderState(&self, pguidinstanceid: *const ::windows_core::GUID, dwstateflagsmask: u32, dwstateflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSyncProviderState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pguidinstanceid), ::core::mem::transmute(dwstateflagsmask), ::core::mem::transmute(dwstateflags)).ok()
    }
    pub unsafe fn RegisterForEvent(&self, phevent: *mut ::win32_foundation::HANDLE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterForEvent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(phevent)).ok()
    }
    pub unsafe fn RevokeEvent<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(&self, hevent: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RevokeEvent)(::windows_core::Interface::as_raw(self), hevent.into_param().abi()).ok()
    }
    pub unsafe fn GetChange<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(&self, hevent: Param0) -> ::windows_core::Result<ISyncRegistrationChange> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetChange)(::windows_core::Interface::as_raw(self), hevent.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncRegistrationChange>(result__)
    }
}
impl ::core::convert::From<ISyncProviderRegistration> for ::windows_core::IUnknown {
    fn from(value: ISyncProviderRegistration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncProviderRegistration> for ::windows_core::IUnknown {
    fn from(value: &ISyncProviderRegistration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISyncProviderRegistration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISyncProviderRegistration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISyncProviderRegistration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncProviderRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncProviderRegistration {}
impl ::core::fmt::Debug for ISyncProviderRegistration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncProviderRegistration").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISyncProviderRegistration {
    type Vtable = ISyncProviderRegistration_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcb45953b_7624_47bc_a472_eb8cac6b222e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncProviderRegistration_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub CreateSyncProviderConfigUIRegistrationInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconfiguiconfig: *const SyncProviderConfigUIConfiguration, ppconfiguiinfo: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    CreateSyncProviderConfigUIRegistrationInstance: usize,
    pub UnregisterSyncProviderConfigUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub EnumerateSyncProviderConfigUIs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidcontenttype: *const ::windows_core::GUID, dwsupportedarchitecture: u32, ppenumsyncproviderconfiguiinfos: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub CreateSyncProviderRegistrationInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproviderconfiguration: *const SyncProviderConfiguration, ppproviderinfo: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    CreateSyncProviderRegistrationInstance: usize,
    pub UnregisterSyncProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetSyncProviderConfigUIInfoforProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidproviderinstanceid: *const ::windows_core::GUID, ppproviderconfiguiinfo: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetSyncProviderConfigUIInfoforProvider: usize,
    pub EnumerateSyncProviders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidcontenttype: *const ::windows_core::GUID, dwstateflagstofiltermask: u32, dwstateflagstofilter: u32, refproviderclsid: *const ::windows_core::GUID, dwsupportedarchitecture: u32, ppenumsyncproviderinfos: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetSyncProviderInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows_core::GUID, ppproviderinfo: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetSyncProviderInfo: usize,
    pub GetSyncProviderFromInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows_core::GUID, dwclscontext: u32, ppsyncprovider: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetSyncProviderConfigUIInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows_core::GUID, ppconfiguiinfo: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetSyncProviderConfigUIInfo: usize,
    pub GetSyncProviderConfigUIFromInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows_core::GUID, dwclscontext: u32, ppconfigui: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetSyncProviderState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows_core::GUID, pdwstateflags: *mut u32) -> ::windows_core::HRESULT,
    pub SetSyncProviderState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows_core::GUID, dwstateflagsmask: u32, dwstateflags: u32) -> ::windows_core::HRESULT,
    pub RegisterForEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phevent: *mut ::win32_foundation::HANDLE) -> ::windows_core::HRESULT,
    pub RevokeEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hevent: ::win32_foundation::HANDLE) -> ::windows_core::HRESULT,
    pub GetChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hevent: ::win32_foundation::HANDLE, ppchange: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISyncRegistrationChange(::windows_core::IUnknown);
impl ISyncRegistrationChange {
    pub unsafe fn GetEvent(&self) -> ::windows_core::Result<SYNC_REGISTRATION_EVENT> {
        let mut result__ = ::core::mem::MaybeUninit::<SYNC_REGISTRATION_EVENT>::zeroed();
        (::windows_core::Interface::vtable(self).GetEvent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<SYNC_REGISTRATION_EVENT>(result__)
    }
    pub unsafe fn GetInstanceId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).GetInstanceId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
}
impl ::core::convert::From<ISyncRegistrationChange> for ::windows_core::IUnknown {
    fn from(value: ISyncRegistrationChange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncRegistrationChange> for ::windows_core::IUnknown {
    fn from(value: &ISyncRegistrationChange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISyncRegistrationChange {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISyncRegistrationChange {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISyncRegistrationChange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncRegistrationChange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncRegistrationChange {}
impl ::core::fmt::Debug for ISyncRegistrationChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncRegistrationChange").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISyncRegistrationChange {
    type Vtable = ISyncRegistrationChange_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeea0d9ae_6b29_43b4_9e70_e3ae33bb2c3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncRegistrationChange_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psreevent: *mut SYNC_REGISTRATION_EVENT) -> ::windows_core::HRESULT,
    pub GetInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidinstanceid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISyncSessionExtendedErrorInfo(::windows_core::IUnknown);
impl ISyncSessionExtendedErrorInfo {
    pub unsafe fn GetSyncProviderWithError(&self) -> ::windows_core::Result<ISyncProvider> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSyncProviderWithError)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncProvider>(result__)
    }
}
impl ::core::convert::From<ISyncSessionExtendedErrorInfo> for ::windows_core::IUnknown {
    fn from(value: ISyncSessionExtendedErrorInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncSessionExtendedErrorInfo> for ::windows_core::IUnknown {
    fn from(value: &ISyncSessionExtendedErrorInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISyncSessionExtendedErrorInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISyncSessionExtendedErrorInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISyncSessionExtendedErrorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncSessionExtendedErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncSessionExtendedErrorInfo {}
impl ::core::fmt::Debug for ISyncSessionExtendedErrorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncSessionExtendedErrorInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISyncSessionExtendedErrorInfo {
    type Vtable = ISyncSessionExtendedErrorInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x326c6810_790a_409b_b741_6999388761eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncSessionExtendedErrorInfo_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetSyncProviderWithError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproviderwitherror: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISyncSessionState(::windows_core::IUnknown);
impl ISyncSessionState {
    pub unsafe fn IsCanceled(&self, pfiscanceled: *mut ::win32_foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsCanceled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pfiscanceled)).ok()
    }
    pub unsafe fn GetInfoForChangeApplication(&self, pbchangeapplierinfo: *mut u8, pcbchangeapplierinfo: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetInfoForChangeApplication)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbchangeapplierinfo), ::core::mem::transmute(pcbchangeapplierinfo)).ok()
    }
    pub unsafe fn LoadInfoFromChangeApplication(&self, pbchangeapplierinfo: *const u8, cbchangeapplierinfo: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LoadInfoFromChangeApplication)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbchangeapplierinfo), ::core::mem::transmute(cbchangeapplierinfo)).ok()
    }
    pub unsafe fn GetForgottenKnowledgeRecoveryRangeStart(&self, pbrangestart: *mut u8, pcbrangestart: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetForgottenKnowledgeRecoveryRangeStart)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbrangestart), ::core::mem::transmute(pcbrangestart)).ok()
    }
    pub unsafe fn GetForgottenKnowledgeRecoveryRangeEnd(&self, pbrangeend: *mut u8, pcbrangeend: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetForgottenKnowledgeRecoveryRangeEnd)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbrangeend), ::core::mem::transmute(pcbrangeend)).ok()
    }
    pub unsafe fn SetForgottenKnowledgeRecoveryRange(&self, prange: *const SYNC_RANGE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetForgottenKnowledgeRecoveryRange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(prange)).ok()
    }
    pub unsafe fn OnProgress(&self, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnProgress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(provider), ::core::mem::transmute(syncstage), ::core::mem::transmute(dwcompletedwork), ::core::mem::transmute(dwtotalwork)).ok()
    }
}
impl ::core::convert::From<ISyncSessionState> for ::windows_core::IUnknown {
    fn from(value: ISyncSessionState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncSessionState> for ::windows_core::IUnknown {
    fn from(value: &ISyncSessionState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISyncSessionState {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISyncSessionState {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISyncSessionState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncSessionState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncSessionState {}
impl ::core::fmt::Debug for ISyncSessionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncSessionState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISyncSessionState {
    type Vtable = ISyncSessionState_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb8a940fe_9f01_483b_9434_c37d361225d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncSessionState_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub IsCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfiscanceled: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetInfoForChangeApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbchangeapplierinfo: *mut u8, pcbchangeapplierinfo: *mut u32) -> ::windows_core::HRESULT,
    pub LoadInfoFromChangeApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbchangeapplierinfo: *const u8, cbchangeapplierinfo: u32) -> ::windows_core::HRESULT,
    pub GetForgottenKnowledgeRecoveryRangeStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbrangestart: *mut u8, pcbrangestart: *mut u32) -> ::windows_core::HRESULT,
    pub GetForgottenKnowledgeRecoveryRangeEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbrangeend: *mut u8, pcbrangeend: *mut u32) -> ::windows_core::HRESULT,
    pub SetForgottenKnowledgeRecoveryRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: *const SYNC_RANGE) -> ::windows_core::HRESULT,
    pub OnProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISyncSessionState2(::windows_core::IUnknown);
impl ISyncSessionState2 {
    pub unsafe fn IsCanceled(&self, pfiscanceled: *mut ::win32_foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.IsCanceled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pfiscanceled)).ok()
    }
    pub unsafe fn GetInfoForChangeApplication(&self, pbchangeapplierinfo: *mut u8, pcbchangeapplierinfo: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfoForChangeApplication)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbchangeapplierinfo), ::core::mem::transmute(pcbchangeapplierinfo)).ok()
    }
    pub unsafe fn LoadInfoFromChangeApplication(&self, pbchangeapplierinfo: *const u8, cbchangeapplierinfo: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.LoadInfoFromChangeApplication)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbchangeapplierinfo), ::core::mem::transmute(cbchangeapplierinfo)).ok()
    }
    pub unsafe fn GetForgottenKnowledgeRecoveryRangeStart(&self, pbrangestart: *mut u8, pcbrangestart: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetForgottenKnowledgeRecoveryRangeStart)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbrangestart), ::core::mem::transmute(pcbrangestart)).ok()
    }
    pub unsafe fn GetForgottenKnowledgeRecoveryRangeEnd(&self, pbrangeend: *mut u8, pcbrangeend: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetForgottenKnowledgeRecoveryRangeEnd)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbrangeend), ::core::mem::transmute(pcbrangeend)).ok()
    }
    pub unsafe fn SetForgottenKnowledgeRecoveryRange(&self, prange: *const SYNC_RANGE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetForgottenKnowledgeRecoveryRange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(prange)).ok()
    }
    pub unsafe fn OnProgress(&self, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnProgress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(provider), ::core::mem::transmute(syncstage), ::core::mem::transmute(dwcompletedwork), ::core::mem::transmute(dwtotalwork)).ok()
    }
    pub unsafe fn SetProviderWithError<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fself: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProviderWithError)(::windows_core::Interface::as_raw(self), fself.into_param().abi()).ok()
    }
    pub unsafe fn GetSessionErrorStatus(&self, phrsessionerror: *mut ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSessionErrorStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(phrsessionerror)).ok()
    }
}
impl ::core::convert::From<ISyncSessionState2> for ::windows_core::IUnknown {
    fn from(value: ISyncSessionState2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncSessionState2> for ::windows_core::IUnknown {
    fn from(value: &ISyncSessionState2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISyncSessionState2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISyncSessionState2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISyncSessionState2> for ISyncSessionState {
    fn from(value: ISyncSessionState2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncSessionState2> for ISyncSessionState {
    fn from(value: &ISyncSessionState2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyncSessionState> for ISyncSessionState2 {
    fn into_param(self) -> ::windows_core::Param<'a, ISyncSessionState> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyncSessionState> for &'a ISyncSessionState2 {
    fn into_param(self) -> ::windows_core::Param<'a, ISyncSessionState> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISyncSessionState2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncSessionState2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncSessionState2 {}
impl ::core::fmt::Debug for ISyncSessionState2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncSessionState2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISyncSessionState2 {
    type Vtable = ISyncSessionState2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9e37cfa3_9e38_4c61_9ca3_ffe810b45ca2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncSessionState2_Vtbl {
    pub base__: ISyncSessionState_Vtbl,
    pub SetProviderWithError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fself: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetSessionErrorStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phrsessionerror: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISynchronousDataRetriever(::windows_core::IUnknown);
impl ISynchronousDataRetriever {
    pub unsafe fn GetIdParameters(&self, pidparameters: *mut ID_PARAMETERS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetIdParameters)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pidparameters)).ok()
    }
    pub unsafe fn LoadChangeData<'a, Param0: ::windows_core::IntoParam<'a, ILoadChangeContext>>(&self, ploadchangecontext: Param0) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).LoadChangeData)(::windows_core::Interface::as_raw(self), ploadchangecontext.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
}
impl ::core::convert::From<ISynchronousDataRetriever> for ::windows_core::IUnknown {
    fn from(value: ISynchronousDataRetriever) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISynchronousDataRetriever> for ::windows_core::IUnknown {
    fn from(value: &ISynchronousDataRetriever) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISynchronousDataRetriever {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISynchronousDataRetriever {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISynchronousDataRetriever {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISynchronousDataRetriever {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISynchronousDataRetriever {}
impl ::core::fmt::Debug for ISynchronousDataRetriever {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISynchronousDataRetriever").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISynchronousDataRetriever {
    type Vtable = ISynchronousDataRetriever_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9b22f2a9_a4cd_4648_9d8e_3a510d4da04b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronousDataRetriever_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetIdParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> ::windows_core::HRESULT,
    pub LoadChangeData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ploadchangecontext: ::windows_core::RawPtr, ppunkdata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct KNOWLEDGE_COOKIE_COMPARISON_RESULT(pub i32);
pub const KCCR_COOKIE_KNOWLEDGE_EQUAL: KNOWLEDGE_COOKIE_COMPARISON_RESULT = KNOWLEDGE_COOKIE_COMPARISON_RESULT(0i32);
pub const KCCR_COOKIE_KNOWLEDGE_CONTAINED: KNOWLEDGE_COOKIE_COMPARISON_RESULT = KNOWLEDGE_COOKIE_COMPARISON_RESULT(1i32);
pub const KCCR_COOKIE_KNOWLEDGE_CONTAINS: KNOWLEDGE_COOKIE_COMPARISON_RESULT = KNOWLEDGE_COOKIE_COMPARISON_RESULT(2i32);
pub const KCCR_COOKIE_KNOWLEDGE_NOT_COMPARABLE: KNOWLEDGE_COOKIE_COMPARISON_RESULT = KNOWLEDGE_COOKIE_COMPARISON_RESULT(3i32);
impl ::core::marker::Copy for KNOWLEDGE_COOKIE_COMPARISON_RESULT {}
impl ::core::clone::Clone for KNOWLEDGE_COOKIE_COMPARISON_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KNOWLEDGE_COOKIE_COMPARISON_RESULT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for KNOWLEDGE_COOKIE_COMPARISON_RESULT {
    type Abi = Self;
}
impl ::core::fmt::Debug for KNOWLEDGE_COOKIE_COMPARISON_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KNOWLEDGE_COOKIE_COMPARISON_RESULT").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_CAPABILITIES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_CLSID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_CONTENTTYPE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_DESCRIPTION: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_ICON: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 11u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_INSTANCEID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_IS_GLOBAL: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_MENUITEM: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 13u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_MENUITEM_NOUI: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 12u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_NAME: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 8u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_SUPPORTED_ARCHITECTURE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_TOOLTIPS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 10u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_CAPABILITIES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_CLSID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_CONFIGUI: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_CONTENTTYPE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_DESCRIPTION: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_ICON: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 11u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_INSTANCEID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_NAME: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 8u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_SUPPORTED_ARCHITECTURE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_TOOLTIPS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 10u32 };
pub const SYNC_CHANGE_FLAG_DELETED: u32 = 1u32;
pub const SYNC_CHANGE_FLAG_DOES_NOT_EXIST: u32 = 2u32;
pub const SYNC_CHANGE_FLAG_GHOST: u32 = 4u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SYNC_CONSTRAINT_RESOLVE_ACTION(pub i32);
pub const SCRA_DEFER: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(0i32);
pub const SCRA_ACCEPT_DESTINATION_PROVIDER: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(1i32);
pub const SCRA_ACCEPT_SOURCE_PROVIDER: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(2i32);
pub const SCRA_TRANSFER_AND_DEFER: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(3i32);
pub const SCRA_MERGE: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(4i32);
pub const SCRA_RENAME_SOURCE: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(5i32);
pub const SCRA_RENAME_DESTINATION: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(6i32);
impl ::core::marker::Copy for SYNC_CONSTRAINT_RESOLVE_ACTION {}
impl ::core::clone::Clone for SYNC_CONSTRAINT_RESOLVE_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYNC_CONSTRAINT_RESOLVE_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SYNC_CONSTRAINT_RESOLVE_ACTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for SYNC_CONSTRAINT_RESOLVE_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNC_CONSTRAINT_RESOLVE_ACTION").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct SYNC_FILTER_CHANGE {
    pub fMoveIn: ::win32_foundation::BOOL,
    pub moveVersion: SYNC_VERSION,
}
impl ::core::marker::Copy for SYNC_FILTER_CHANGE {}
impl ::core::clone::Clone for SYNC_FILTER_CHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYNC_FILTER_CHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYNC_FILTER_CHANGE").field("fMoveIn", &self.fMoveIn).field("moveVersion", &self.moveVersion).finish()
    }
}
unsafe impl ::windows_core::Abi for SYNC_FILTER_CHANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SYNC_FILTER_CHANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SYNC_FILTER_CHANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for SYNC_FILTER_CHANGE {}
impl ::core::default::Default for SYNC_FILTER_CHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const SYNC_FILTER_INFO_COMBINED: u32 = 8u32;
pub const SYNC_FILTER_INFO_FLAG_CHANGE_UNIT_LIST: u32 = 2u32;
pub const SYNC_FILTER_INFO_FLAG_CUSTOM: u32 = 4u32;
pub const SYNC_FILTER_INFO_FLAG_ITEM_LIST: u32 = 1u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SYNC_FULL_ENUMERATION_ACTION(pub i32);
pub const SFEA_FULL_ENUMERATION: SYNC_FULL_ENUMERATION_ACTION = SYNC_FULL_ENUMERATION_ACTION(0i32);
pub const SFEA_PARTIAL_SYNC: SYNC_FULL_ENUMERATION_ACTION = SYNC_FULL_ENUMERATION_ACTION(1i32);
pub const SFEA_ABORT: SYNC_FULL_ENUMERATION_ACTION = SYNC_FULL_ENUMERATION_ACTION(2i32);
impl ::core::marker::Copy for SYNC_FULL_ENUMERATION_ACTION {}
impl ::core::clone::Clone for SYNC_FULL_ENUMERATION_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYNC_FULL_ENUMERATION_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SYNC_FULL_ENUMERATION_ACTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for SYNC_FULL_ENUMERATION_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNC_FULL_ENUMERATION_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SYNC_PROGRESS_STAGE(pub i32);
pub const SPS_CHANGE_DETECTION: SYNC_PROGRESS_STAGE = SYNC_PROGRESS_STAGE(0i32);
pub const SPS_CHANGE_ENUMERATION: SYNC_PROGRESS_STAGE = SYNC_PROGRESS_STAGE(1i32);
pub const SPS_CHANGE_APPLICATION: SYNC_PROGRESS_STAGE = SYNC_PROGRESS_STAGE(2i32);
impl ::core::marker::Copy for SYNC_PROGRESS_STAGE {}
impl ::core::clone::Clone for SYNC_PROGRESS_STAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYNC_PROGRESS_STAGE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SYNC_PROGRESS_STAGE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SYNC_PROGRESS_STAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNC_PROGRESS_STAGE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SYNC_PROVIDER_ROLE(pub i32);
pub const SPR_SOURCE: SYNC_PROVIDER_ROLE = SYNC_PROVIDER_ROLE(0i32);
pub const SPR_DESTINATION: SYNC_PROVIDER_ROLE = SYNC_PROVIDER_ROLE(1i32);
impl ::core::marker::Copy for SYNC_PROVIDER_ROLE {}
impl ::core::clone::Clone for SYNC_PROVIDER_ROLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYNC_PROVIDER_ROLE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SYNC_PROVIDER_ROLE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SYNC_PROVIDER_ROLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNC_PROVIDER_ROLE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct SYNC_RANGE {
    pub pbClosedLowerBound: *mut u8,
    pub pbClosedUpperBound: *mut u8,
}
impl ::core::marker::Copy for SYNC_RANGE {}
impl ::core::clone::Clone for SYNC_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYNC_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYNC_RANGE").field("pbClosedLowerBound", &self.pbClosedLowerBound).field("pbClosedUpperBound", &self.pbClosedUpperBound).finish()
    }
}
unsafe impl ::windows_core::Abi for SYNC_RANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SYNC_RANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SYNC_RANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for SYNC_RANGE {}
impl ::core::default::Default for SYNC_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SYNC_REGISTRATION_EVENT(pub i32);
pub const SRE_PROVIDER_ADDED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(0i32);
pub const SRE_PROVIDER_REMOVED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(1i32);
pub const SRE_PROVIDER_UPDATED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(2i32);
pub const SRE_PROVIDER_STATE_CHANGED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(3i32);
pub const SRE_CONFIGUI_ADDED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(4i32);
pub const SRE_CONFIGUI_REMOVED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(5i32);
pub const SRE_CONFIGUI_UPDATED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(6i32);
impl ::core::marker::Copy for SYNC_REGISTRATION_EVENT {}
impl ::core::clone::Clone for SYNC_REGISTRATION_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYNC_REGISTRATION_EVENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SYNC_REGISTRATION_EVENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for SYNC_REGISTRATION_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNC_REGISTRATION_EVENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SYNC_RESOLVE_ACTION(pub i32);
pub const SRA_DEFER: SYNC_RESOLVE_ACTION = SYNC_RESOLVE_ACTION(0i32);
pub const SRA_ACCEPT_DESTINATION_PROVIDER: SYNC_RESOLVE_ACTION = SYNC_RESOLVE_ACTION(1i32);
pub const SRA_ACCEPT_SOURCE_PROVIDER: SYNC_RESOLVE_ACTION = SYNC_RESOLVE_ACTION(2i32);
pub const SRA_MERGE: SYNC_RESOLVE_ACTION = SYNC_RESOLVE_ACTION(3i32);
pub const SRA_TRANSFER_AND_DEFER: SYNC_RESOLVE_ACTION = SYNC_RESOLVE_ACTION(4i32);
pub const SRA_LAST: SYNC_RESOLVE_ACTION = SYNC_RESOLVE_ACTION(5i32);
impl ::core::marker::Copy for SYNC_RESOLVE_ACTION {}
impl ::core::clone::Clone for SYNC_RESOLVE_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYNC_RESOLVE_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SYNC_RESOLVE_ACTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for SYNC_RESOLVE_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNC_RESOLVE_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SYNC_SERIALIZATION_VERSION(pub i32);
pub const SYNC_SERIALIZATION_VERSION_V1: SYNC_SERIALIZATION_VERSION = SYNC_SERIALIZATION_VERSION(1i32);
pub const SYNC_SERIALIZATION_VERSION_V2: SYNC_SERIALIZATION_VERSION = SYNC_SERIALIZATION_VERSION(4i32);
pub const SYNC_SERIALIZATION_VERSION_V3: SYNC_SERIALIZATION_VERSION = SYNC_SERIALIZATION_VERSION(5i32);
impl ::core::marker::Copy for SYNC_SERIALIZATION_VERSION {}
impl ::core::clone::Clone for SYNC_SERIALIZATION_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYNC_SERIALIZATION_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SYNC_SERIALIZATION_VERSION {
    type Abi = Self;
}
impl ::core::fmt::Debug for SYNC_SERIALIZATION_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNC_SERIALIZATION_VERSION").field(&self.0).finish()
    }
}
pub const SYNC_SERIALIZE_REPLICA_KEY_MAP: u32 = 1u32;
#[repr(C)]
pub struct SYNC_SESSION_STATISTICS {
    pub dwChangesApplied: u32,
    pub dwChangesFailed: u32,
}
impl ::core::marker::Copy for SYNC_SESSION_STATISTICS {}
impl ::core::clone::Clone for SYNC_SESSION_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYNC_SESSION_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYNC_SESSION_STATISTICS").field("dwChangesApplied", &self.dwChangesApplied).field("dwChangesFailed", &self.dwChangesFailed).finish()
    }
}
unsafe impl ::windows_core::Abi for SYNC_SESSION_STATISTICS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SYNC_SESSION_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SYNC_SESSION_STATISTICS>()) == 0 }
    }
}
impl ::core::cmp::Eq for SYNC_SESSION_STATISTICS {}
impl ::core::default::Default for SYNC_SESSION_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SYNC_STATISTICS(pub i32);
pub const SYNC_STATISTICS_RANGE_COUNT: SYNC_STATISTICS = SYNC_STATISTICS(0i32);
impl ::core::marker::Copy for SYNC_STATISTICS {}
impl ::core::clone::Clone for SYNC_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYNC_STATISTICS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SYNC_STATISTICS {
    type Abi = Self;
}
impl ::core::fmt::Debug for SYNC_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNC_STATISTICS").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct SYNC_TIME {
    pub dwDate: u32,
    pub dwTime: u32,
}
impl ::core::marker::Copy for SYNC_TIME {}
impl ::core::clone::Clone for SYNC_TIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYNC_TIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYNC_TIME").field("dwDate", &self.dwDate).field("dwTime", &self.dwTime).finish()
    }
}
unsafe impl ::windows_core::Abi for SYNC_TIME {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SYNC_TIME {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SYNC_TIME>()) == 0 }
    }
}
impl ::core::cmp::Eq for SYNC_TIME {}
impl ::core::default::Default for SYNC_TIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SYNC_VERSION {
    pub dwLastUpdatingReplicaKey: u32,
    pub ullTickCount: u64,
}
impl ::core::marker::Copy for SYNC_VERSION {}
impl ::core::clone::Clone for SYNC_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYNC_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYNC_VERSION").field("dwLastUpdatingReplicaKey", &self.dwLastUpdatingReplicaKey).field("ullTickCount", &self.ullTickCount).finish()
    }
}
unsafe impl ::windows_core::Abi for SYNC_VERSION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SYNC_VERSION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SYNC_VERSION>()) == 0 }
    }
}
impl ::core::cmp::Eq for SYNC_VERSION {}
impl ::core::default::Default for SYNC_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const SYNC_VERSION_FLAG_FROM_FEED: u32 = 1u32;
pub const SYNC_VERSION_FLAG_HAS_BY: u32 = 2u32;
#[repr(C)]
pub struct SyncProviderConfigUIConfiguration {
    pub dwVersion: u32,
    pub guidInstanceId: ::windows_core::GUID,
    pub clsidConfigUI: ::windows_core::GUID,
    pub guidContentType: ::windows_core::GUID,
    pub dwCapabilities: u32,
    pub dwSupportedArchitecture: u32,
    pub fIsGlobal: ::win32_foundation::BOOL,
}
impl ::core::marker::Copy for SyncProviderConfigUIConfiguration {}
impl ::core::clone::Clone for SyncProviderConfigUIConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SyncProviderConfigUIConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SyncProviderConfigUIConfiguration").field("dwVersion", &self.dwVersion).field("guidInstanceId", &self.guidInstanceId).field("clsidConfigUI", &self.clsidConfigUI).field("guidContentType", &self.guidContentType).field("dwCapabilities", &self.dwCapabilities).field("dwSupportedArchitecture", &self.dwSupportedArchitecture).field("fIsGlobal", &self.fIsGlobal).finish()
    }
}
unsafe impl ::windows_core::Abi for SyncProviderConfigUIConfiguration {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SyncProviderConfigUIConfiguration {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SyncProviderConfigUIConfiguration>()) == 0 }
    }
}
impl ::core::cmp::Eq for SyncProviderConfigUIConfiguration {}
impl ::core::default::Default for SyncProviderConfigUIConfiguration {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SyncProviderConfiguration {
    pub dwVersion: u32,
    pub guidInstanceId: ::windows_core::GUID,
    pub clsidProvider: ::windows_core::GUID,
    pub guidConfigUIInstanceId: ::windows_core::GUID,
    pub guidContentType: ::windows_core::GUID,
    pub dwCapabilities: u32,
    pub dwSupportedArchitecture: u32,
}
impl ::core::marker::Copy for SyncProviderConfiguration {}
impl ::core::clone::Clone for SyncProviderConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SyncProviderConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SyncProviderConfiguration").field("dwVersion", &self.dwVersion).field("guidInstanceId", &self.guidInstanceId).field("clsidProvider", &self.clsidProvider).field("guidConfigUIInstanceId", &self.guidConfigUIInstanceId).field("guidContentType", &self.guidContentType).field("dwCapabilities", &self.dwCapabilities).field("dwSupportedArchitecture", &self.dwSupportedArchitecture).finish()
    }
}
unsafe impl ::windows_core::Abi for SyncProviderConfiguration {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SyncProviderConfiguration {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SyncProviderConfiguration>()) == 0 }
    }
}
impl ::core::cmp::Eq for SyncProviderConfiguration {}
impl ::core::default::Default for SyncProviderConfiguration {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const SyncProviderRegistration: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf82b4ef1_93a9_4dde_8015_f7950a1a6e31);
