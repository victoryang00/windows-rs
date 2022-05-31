#[cfg(feature = "Core")]
pub mod Core;
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivity(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserActivity {
    type Vtable = IUserActivity_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfc103e9e_2cab_4d36_aea2_b4bb556cef0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivity_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserActivityState) -> ::windows_core::HRESULT,
    pub ActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VisualElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ContentUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetContentUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FallbackUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetFallbackUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ActivationUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetActivationUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ContentInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetContentInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SaveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivity2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserActivity2 {
    type Vtable = IUserActivity2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9dc40c62_08c4_47ac_aa9c_2bb2221c55fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivity2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ToJson: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivity3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserActivity3 {
    type Vtable = IUserActivity3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe7697744_e1a2_5147_8e06_55f1eeef271c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivity3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsRoamable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsRoamable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityAttribution(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserActivityAttribution {
    type Vtable = IUserActivityAttribution_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x34a5c8b5_86dd_4aec_a491_6a4faea5d22e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityAttribution_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IconUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetIconUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AlternateText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetAlternateText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AddImageQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAddImageQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityAttributionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserActivityAttributionFactory {
    type Vtable = IUserActivityAttributionFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe62bd252_c566_4f42_9974_916c4d76377e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityAttributionFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateWithUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iconuri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityChannel(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserActivityChannel {
    type Vtable = IUserActivityChannel_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbac0f8b8_a0e4_483b_b948_9cbabd06070c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityChannel_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetOrCreateUserActivityAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activityid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DeleteActivityAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activityid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DeleteAllActivitiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityChannel2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserActivityChannel2 {
    type Vtable = IUserActivityChannel2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1698e35b_eb7e_4ea0_bf17_a459e8be706c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityChannel2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub GetRecentUserActivitiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxuniqueactivities: i32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetRecentUserActivitiesAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub GetSessionHistoryItemsForUserActivityAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activityid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, starttime: ::winrt_foundation::DateTime, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetSessionHistoryItemsForUserActivityAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityChannelStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserActivityChannelStatics {
    type Vtable = IUserActivityChannelStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc8c005ab_198d_4d80_abb2_c9775ec4a729);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityChannelStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityChannelStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserActivityChannelStatics2 {
    type Vtable = IUserActivityChannelStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8e87de30_aa4f_4624_9ad0_d40f3ba0317c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityChannelStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DisableAutoSessionCreation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-security")]
    pub TryGetForWebAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, account: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-security"))]
    TryGetForWebAccount: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityChannelStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserActivityChannelStatics3 {
    type Vtable = IUserActivityChannelStatics3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x53bc4ddb_bbdf_5984_802a_5305874e205c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityChannelStatics3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-system")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    GetForUser: usize,
}
#[repr(transparent)]
pub struct IUserActivityContentInfo(::windows_core::IUnknown);
impl IUserActivityContentInfo {
    pub fn ToJson(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToJson)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IUserActivityContentInfo> for ::windows_core::IUnknown {
    fn from(value: IUserActivityContentInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUserActivityContentInfo> for ::windows_core::IUnknown {
    fn from(value: &IUserActivityContentInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUserActivityContentInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUserActivityContentInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IUserActivityContentInfo> for ::windows_core::IInspectable {
    fn from(value: IUserActivityContentInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUserActivityContentInfo> for ::windows_core::IInspectable {
    fn from(value: &IUserActivityContentInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IUserActivityContentInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IUserActivityContentInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUserActivityContentInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUserActivityContentInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUserActivityContentInfo {}
impl ::core::fmt::Debug for IUserActivityContentInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUserActivityContentInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IUserActivityContentInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{b399e5ad-137f-409d-822d-e1af27ce08dc}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IUserActivityContentInfo {
    type Vtable = IUserActivityContentInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb399e5ad_137f_409d_822d_e1af27ce08dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityContentInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ToJson: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityContentInfoStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserActivityContentInfoStatics {
    type Vtable = IUserActivityContentInfoStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9988c34b_0386_4bc9_968a_8200b004144f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityContentInfoStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FromJson: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserActivityFactory {
    type Vtable = IUserActivityFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7c385758_361d_4a67_8a3b_34ca2978f9a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateWithActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activityid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserActivityRequest {
    type Vtable = IUserActivityRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa0ef6355_cf35_4ff0_8833_50cb4b72e06d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetUserActivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activity: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityRequestManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserActivityRequestManager {
    type Vtable = IUserActivityRequestManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0c30be4e_903d_48d6_82d4_4043ed57791b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityRequestManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub UserActivityRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveUserActivityRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityRequestManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserActivityRequestManagerStatics {
    type Vtable = IUserActivityRequestManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc0392df1_224a_432c_81e5_0c76b4c4cefa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityRequestManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserActivityRequestedEventArgs {
    type Vtable = IUserActivityRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa4cc7a4c_8229_4cfd_a3bc_c61d318575a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivitySession(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserActivitySession {
    type Vtable = IUserActivitySession_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xae434d78_24fa_44a3_ad48_6eda61aa1924);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivitySession_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivitySessionHistoryItem(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserActivitySessionHistoryItem {
    type Vtable = IUserActivitySessionHistoryItem_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe8d59bd3_3e5d_49fd_98d7_6da97521e255);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivitySessionHistoryItem_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub UserActivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub EndTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserActivityStatics {
    type Vtable = IUserActivityStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8c8fd333_0e09_47f6_9ac7_95cf5c39367b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TryParseFromJson: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, json: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub TryParseFromJsonArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, json: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    TryParseFromJsonArray: usize,
    #[cfg(feature = "winrt-foundation")]
    pub ToJsonArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activities: ::windows_core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ToJsonArray: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityVisualElements(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserActivityVisualElements {
    type Vtable = IUserActivityVisualElements_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x94757513_262f_49ef_bbbf_9b75d2e85250);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityVisualElements_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DisplayText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDisplayText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub BackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    BackgroundColor: usize,
    #[cfg(feature = "winrt-ui")]
    pub SetBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    SetBackgroundColor: usize,
    pub Attribution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetAttribution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    SetContent: usize,
    #[cfg(feature = "winrt-ui")]
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    Content: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserActivityVisualElements2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserActivityVisualElements2 {
    type Vtable = IUserActivityVisualElements2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcaae7fc7_3eef_4359_825c_9d51b9220de3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityVisualElements2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AttributionDisplayText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetAttributionDisplayText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct UserActivity(::windows_core::IUnknown);
impl UserActivity {
    pub fn State(&self) -> ::windows_core::Result<UserActivityState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<UserActivityState>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserActivityState>(result__)
        }
    }
    pub fn ActivityId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ActivityId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn VisualElements(&self) -> ::windows_core::Result<UserActivityVisualElements> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VisualElements)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserActivityVisualElements>(result__)
        }
    }
    pub fn ContentUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContentUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetContentUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContentUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ContentType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ContentType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetContentType<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContentType)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn FallbackUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FallbackUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetFallbackUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFallbackUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ActivationUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivationUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetActivationUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetActivationUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ContentInfo(&self) -> ::windows_core::Result<IUserActivityContentInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContentInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IUserActivityContentInfo>(result__)
        }
    }
    pub fn SetContentInfo<'a, Param0: ::windows_core::IntoParam<'a, IUserActivityContentInfo>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContentInfo)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SaveAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SaveAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn CreateSession(&self) -> ::windows_core::Result<UserActivitySession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateSession)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserActivitySession>(result__)
        }
    }
    pub fn ToJson(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IUserActivity2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToJson)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IsRoamable(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IUserActivity3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsRoamable)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsRoamable(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IUserActivity3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsRoamable)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateWithActivityId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(activityid: Param0) -> ::windows_core::Result<UserActivity> {
        Self::IUserActivityFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithActivityId)(::windows_core::Interface::as_raw(this), activityid.into_param().abi(), result__.as_mut_ptr()).from_abi::<UserActivity>(result__)
        })
    }
    pub fn TryParseFromJson<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(json: Param0) -> ::windows_core::Result<UserActivity> {
        Self::IUserActivityStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryParseFromJson)(::windows_core::Interface::as_raw(this), json.into_param().abi(), result__.as_mut_ptr()).from_abi::<UserActivity>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn TryParseFromJsonArray<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(json: Param0) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<UserActivity>> {
        Self::IUserActivityStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryParseFromJsonArray)(::windows_core::Interface::as_raw(this), json.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<UserActivity>>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ToJsonArray<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<UserActivity>>>(activities: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IUserActivityStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToJsonArray)(::windows_core::Interface::as_raw(this), activities.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn IUserActivityFactory<R, F: FnOnce(&IUserActivityFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<UserActivity, IUserActivityFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IUserActivityStatics<R, F: FnOnce(&IUserActivityStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<UserActivity, IUserActivityStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for UserActivity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserActivity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserActivity {}
impl ::core::fmt::Debug for UserActivity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserActivity").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserActivity {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivity;{fc103e9e-2cab-4d36-aea2-b4bb556cef0f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserActivity {
    type Vtable = IUserActivity_Vtbl;
    const IID: ::windows_core::GUID = <IUserActivity as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserActivity {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivity";
}
impl ::core::convert::From<UserActivity> for ::windows_core::IUnknown {
    fn from(value: UserActivity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivity> for ::windows_core::IUnknown {
    fn from(value: &UserActivity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserActivity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserActivity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserActivity> for ::windows_core::IInspectable {
    fn from(value: UserActivity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivity> for ::windows_core::IInspectable {
    fn from(value: &UserActivity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserActivity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserActivity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserActivity {}
unsafe impl ::core::marker::Sync for UserActivity {}
#[repr(transparent)]
pub struct UserActivityAttribution(::windows_core::IUnknown);
impl UserActivityAttribution {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<UserActivityAttribution, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IconUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IconUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetIconUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIconUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn AlternateText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AlternateText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetAlternateText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAlternateText)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn AddImageQuery(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AddImageQuery)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAddImageQuery(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAddImageQuery)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateWithUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(iconuri: Param0) -> ::windows_core::Result<UserActivityAttribution> {
        Self::IUserActivityAttributionFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithUri)(::windows_core::Interface::as_raw(this), iconuri.into_param().abi(), result__.as_mut_ptr()).from_abi::<UserActivityAttribution>(result__)
        })
    }
    pub fn IUserActivityAttributionFactory<R, F: FnOnce(&IUserActivityAttributionFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<UserActivityAttribution, IUserActivityAttributionFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for UserActivityAttribution {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserActivityAttribution {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserActivityAttribution {}
impl ::core::fmt::Debug for UserActivityAttribution {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserActivityAttribution").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserActivityAttribution {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivityAttribution;{34a5c8b5-86dd-4aec-a491-6a4faea5d22e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserActivityAttribution {
    type Vtable = IUserActivityAttribution_Vtbl;
    const IID: ::windows_core::GUID = <IUserActivityAttribution as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserActivityAttribution {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivityAttribution";
}
impl ::core::convert::From<UserActivityAttribution> for ::windows_core::IUnknown {
    fn from(value: UserActivityAttribution) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivityAttribution> for ::windows_core::IUnknown {
    fn from(value: &UserActivityAttribution) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserActivityAttribution {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserActivityAttribution {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserActivityAttribution> for ::windows_core::IInspectable {
    fn from(value: UserActivityAttribution) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivityAttribution> for ::windows_core::IInspectable {
    fn from(value: &UserActivityAttribution) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserActivityAttribution {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserActivityAttribution {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserActivityAttribution {}
unsafe impl ::core::marker::Sync for UserActivityAttribution {}
#[repr(transparent)]
pub struct UserActivityChannel(::windows_core::IUnknown);
impl UserActivityChannel {
    pub fn GetOrCreateUserActivityAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, activityid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<UserActivity>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetOrCreateUserActivityAsync)(::windows_core::Interface::as_raw(this), activityid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<UserActivity>>(result__)
        }
    }
    pub fn DeleteActivityAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, activityid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeleteActivityAsync)(::windows_core::Interface::as_raw(this), activityid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn DeleteAllActivitiesAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeleteAllActivitiesAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetRecentUserActivitiesAsync(&self, maxuniqueactivities: i32) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVector<UserActivitySessionHistoryItem>>> {
        let this = &::windows_core::Interface::cast::<IUserActivityChannel2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetRecentUserActivitiesAsync)(::windows_core::Interface::as_raw(this), maxuniqueactivities, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVector<UserActivitySessionHistoryItem>>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetSessionHistoryItemsForUserActivityAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>>(&self, activityid: Param0, starttime: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVector<UserActivitySessionHistoryItem>>> {
        let this = &::windows_core::Interface::cast::<IUserActivityChannel2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSessionHistoryItemsForUserActivityAsync)(::windows_core::Interface::as_raw(this), activityid.into_param().abi(), starttime.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVector<UserActivitySessionHistoryItem>>>(result__)
        }
    }
    pub fn GetDefault() -> ::windows_core::Result<UserActivityChannel> {
        Self::IUserActivityChannelStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserActivityChannel>(result__)
        })
    }
    pub fn DisableAutoSessionCreation() -> ::windows_core::Result<()> {
        Self::IUserActivityChannelStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).DisableAutoSessionCreation)(::windows_core::Interface::as_raw(this)).ok() })
    }
    #[cfg(feature = "winrt-security")]
    pub fn TryGetForWebAccount<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_security::Credentials::WebAccount>>(account: Param0) -> ::windows_core::Result<UserActivityChannel> {
        Self::IUserActivityChannelStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetForWebAccount)(::windows_core::Interface::as_raw(this), account.into_param().abi(), result__.as_mut_ptr()).from_abi::<UserActivityChannel>(result__)
        })
    }
    #[cfg(feature = "winrt-system")]
    pub fn GetForUser<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_system::User>>(user: Param0) -> ::windows_core::Result<UserActivityChannel> {
        Self::IUserActivityChannelStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), result__.as_mut_ptr()).from_abi::<UserActivityChannel>(result__)
        })
    }
    pub fn IUserActivityChannelStatics<R, F: FnOnce(&IUserActivityChannelStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<UserActivityChannel, IUserActivityChannelStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IUserActivityChannelStatics2<R, F: FnOnce(&IUserActivityChannelStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<UserActivityChannel, IUserActivityChannelStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IUserActivityChannelStatics3<R, F: FnOnce(&IUserActivityChannelStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<UserActivityChannel, IUserActivityChannelStatics3> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for UserActivityChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserActivityChannel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserActivityChannel {}
impl ::core::fmt::Debug for UserActivityChannel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserActivityChannel").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserActivityChannel {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivityChannel;{bac0f8b8-a0e4-483b-b948-9cbabd06070c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserActivityChannel {
    type Vtable = IUserActivityChannel_Vtbl;
    const IID: ::windows_core::GUID = <IUserActivityChannel as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserActivityChannel {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivityChannel";
}
impl ::core::convert::From<UserActivityChannel> for ::windows_core::IUnknown {
    fn from(value: UserActivityChannel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivityChannel> for ::windows_core::IUnknown {
    fn from(value: &UserActivityChannel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserActivityChannel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserActivityChannel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserActivityChannel> for ::windows_core::IInspectable {
    fn from(value: UserActivityChannel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivityChannel> for ::windows_core::IInspectable {
    fn from(value: &UserActivityChannel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserActivityChannel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserActivityChannel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserActivityChannel {}
unsafe impl ::core::marker::Sync for UserActivityChannel {}
#[repr(transparent)]
pub struct UserActivityContentInfo(::windows_core::IUnknown);
impl UserActivityContentInfo {
    pub fn ToJson(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToJson)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FromJson<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(value: Param0) -> ::windows_core::Result<UserActivityContentInfo> {
        Self::IUserActivityContentInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromJson)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<UserActivityContentInfo>(result__)
        })
    }
    pub fn IUserActivityContentInfoStatics<R, F: FnOnce(&IUserActivityContentInfoStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<UserActivityContentInfo, IUserActivityContentInfoStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for UserActivityContentInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserActivityContentInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserActivityContentInfo {}
impl ::core::fmt::Debug for UserActivityContentInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserActivityContentInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserActivityContentInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivityContentInfo;{b399e5ad-137f-409d-822d-e1af27ce08dc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserActivityContentInfo {
    type Vtable = IUserActivityContentInfo_Vtbl;
    const IID: ::windows_core::GUID = <IUserActivityContentInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserActivityContentInfo {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivityContentInfo";
}
impl ::core::convert::From<UserActivityContentInfo> for ::windows_core::IUnknown {
    fn from(value: UserActivityContentInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivityContentInfo> for ::windows_core::IUnknown {
    fn from(value: &UserActivityContentInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserActivityContentInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserActivityContentInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserActivityContentInfo> for ::windows_core::IInspectable {
    fn from(value: UserActivityContentInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivityContentInfo> for ::windows_core::IInspectable {
    fn from(value: &UserActivityContentInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserActivityContentInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserActivityContentInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<UserActivityContentInfo> for IUserActivityContentInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: UserActivityContentInfo) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&UserActivityContentInfo> for IUserActivityContentInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: &UserActivityContentInfo) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IUserActivityContentInfo> for UserActivityContentInfo {
    fn into_param(self) -> ::windows_core::Param<'a, IUserActivityContentInfo> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IUserActivityContentInfo> for &UserActivityContentInfo {
    fn into_param(self) -> ::windows_core::Param<'a, IUserActivityContentInfo> {
        ::core::convert::TryInto::<IUserActivityContentInfo>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for UserActivityContentInfo {}
unsafe impl ::core::marker::Sync for UserActivityContentInfo {}
#[repr(transparent)]
pub struct UserActivityRequest(::windows_core::IUnknown);
impl UserActivityRequest {
    pub fn SetUserActivity<'a, Param0: ::windows_core::IntoParam<'a, UserActivity>>(&self, activity: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUserActivity)(::windows_core::Interface::as_raw(this), activity.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for UserActivityRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserActivityRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserActivityRequest {}
impl ::core::fmt::Debug for UserActivityRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserActivityRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserActivityRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivityRequest;{a0ef6355-cf35-4ff0-8833-50cb4b72e06d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserActivityRequest {
    type Vtable = IUserActivityRequest_Vtbl;
    const IID: ::windows_core::GUID = <IUserActivityRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserActivityRequest {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivityRequest";
}
impl ::core::convert::From<UserActivityRequest> for ::windows_core::IUnknown {
    fn from(value: UserActivityRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivityRequest> for ::windows_core::IUnknown {
    fn from(value: &UserActivityRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserActivityRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserActivityRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserActivityRequest> for ::windows_core::IInspectable {
    fn from(value: UserActivityRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivityRequest> for ::windows_core::IInspectable {
    fn from(value: &UserActivityRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserActivityRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserActivityRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserActivityRequest {}
unsafe impl ::core::marker::Sync for UserActivityRequest {}
#[repr(transparent)]
pub struct UserActivityRequestManager(::windows_core::IUnknown);
impl UserActivityRequestManager {
    pub fn UserActivityRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<UserActivityRequestManager, UserActivityRequestedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).UserActivityRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveUserActivityRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUserActivityRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn GetForCurrentView() -> ::windows_core::Result<UserActivityRequestManager> {
        Self::IUserActivityRequestManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserActivityRequestManager>(result__)
        })
    }
    pub fn IUserActivityRequestManagerStatics<R, F: FnOnce(&IUserActivityRequestManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<UserActivityRequestManager, IUserActivityRequestManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for UserActivityRequestManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserActivityRequestManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserActivityRequestManager {}
impl ::core::fmt::Debug for UserActivityRequestManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserActivityRequestManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserActivityRequestManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivityRequestManager;{0c30be4e-903d-48d6-82d4-4043ed57791b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserActivityRequestManager {
    type Vtable = IUserActivityRequestManager_Vtbl;
    const IID: ::windows_core::GUID = <IUserActivityRequestManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserActivityRequestManager {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivityRequestManager";
}
impl ::core::convert::From<UserActivityRequestManager> for ::windows_core::IUnknown {
    fn from(value: UserActivityRequestManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivityRequestManager> for ::windows_core::IUnknown {
    fn from(value: &UserActivityRequestManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserActivityRequestManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserActivityRequestManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserActivityRequestManager> for ::windows_core::IInspectable {
    fn from(value: UserActivityRequestManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivityRequestManager> for ::windows_core::IInspectable {
    fn from(value: &UserActivityRequestManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserActivityRequestManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserActivityRequestManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct UserActivityRequestedEventArgs(::windows_core::IUnknown);
impl UserActivityRequestedEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<UserActivityRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserActivityRequest>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for UserActivityRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserActivityRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserActivityRequestedEventArgs {}
impl ::core::fmt::Debug for UserActivityRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserActivityRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserActivityRequestedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivityRequestedEventArgs;{a4cc7a4c-8229-4cfd-a3bc-c61d318575a4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserActivityRequestedEventArgs {
    type Vtable = IUserActivityRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IUserActivityRequestedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserActivityRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivityRequestedEventArgs";
}
impl ::core::convert::From<UserActivityRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: UserActivityRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivityRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &UserActivityRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserActivityRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserActivityRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserActivityRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: UserActivityRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivityRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &UserActivityRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserActivityRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserActivityRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserActivityRequestedEventArgs {}
unsafe impl ::core::marker::Sync for UserActivityRequestedEventArgs {}
#[repr(transparent)]
pub struct UserActivitySession(::windows_core::IUnknown);
impl UserActivitySession {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ActivityId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ActivityId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for UserActivitySession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserActivitySession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserActivitySession {}
impl ::core::fmt::Debug for UserActivitySession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserActivitySession").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserActivitySession {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivitySession;{ae434d78-24fa-44a3-ad48-6eda61aa1924})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserActivitySession {
    type Vtable = IUserActivitySession_Vtbl;
    const IID: ::windows_core::GUID = <IUserActivitySession as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserActivitySession {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivitySession";
}
impl ::core::convert::From<UserActivitySession> for ::windows_core::IUnknown {
    fn from(value: UserActivitySession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivitySession> for ::windows_core::IUnknown {
    fn from(value: &UserActivitySession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserActivitySession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserActivitySession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserActivitySession> for ::windows_core::IInspectable {
    fn from(value: UserActivitySession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivitySession> for ::windows_core::IInspectable {
    fn from(value: &UserActivitySession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserActivitySession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserActivitySession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<UserActivitySession> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: UserActivitySession) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&UserActivitySession> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &UserActivitySession) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for UserActivitySession {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &UserActivitySession {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for UserActivitySession {}
unsafe impl ::core::marker::Sync for UserActivitySession {}
#[repr(transparent)]
pub struct UserActivitySessionHistoryItem(::windows_core::IUnknown);
impl UserActivitySessionHistoryItem {
    pub fn UserActivity(&self) -> ::windows_core::Result<UserActivity> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UserActivity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserActivity>(result__)
        }
    }
    pub fn StartTime(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn EndTime(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EndTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
}
impl ::core::clone::Clone for UserActivitySessionHistoryItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserActivitySessionHistoryItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserActivitySessionHistoryItem {}
impl ::core::fmt::Debug for UserActivitySessionHistoryItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserActivitySessionHistoryItem").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserActivitySessionHistoryItem {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivitySessionHistoryItem;{e8d59bd3-3e5d-49fd-98d7-6da97521e255})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserActivitySessionHistoryItem {
    type Vtable = IUserActivitySessionHistoryItem_Vtbl;
    const IID: ::windows_core::GUID = <IUserActivitySessionHistoryItem as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserActivitySessionHistoryItem {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivitySessionHistoryItem";
}
impl ::core::convert::From<UserActivitySessionHistoryItem> for ::windows_core::IUnknown {
    fn from(value: UserActivitySessionHistoryItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivitySessionHistoryItem> for ::windows_core::IUnknown {
    fn from(value: &UserActivitySessionHistoryItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserActivitySessionHistoryItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserActivitySessionHistoryItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserActivitySessionHistoryItem> for ::windows_core::IInspectable {
    fn from(value: UserActivitySessionHistoryItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivitySessionHistoryItem> for ::windows_core::IInspectable {
    fn from(value: &UserActivitySessionHistoryItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserActivitySessionHistoryItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserActivitySessionHistoryItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserActivitySessionHistoryItem {}
unsafe impl ::core::marker::Sync for UserActivitySessionHistoryItem {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserActivityState(pub i32);
impl UserActivityState {
    pub const New: Self = Self(0i32);
    pub const Published: Self = Self(1i32);
}
impl ::core::marker::Copy for UserActivityState {}
impl ::core::clone::Clone for UserActivityState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserActivityState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UserActivityState {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserActivityState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserActivityState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserActivityState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserActivities.UserActivityState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct UserActivityVisualElements(::windows_core::IUnknown);
impl UserActivityVisualElements {
    pub fn DisplayText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetDisplayText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayText)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDescription)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn BackgroundColor(&self) -> ::windows_core::Result<::winrt_ui::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_ui::Color>::zeroed();
            (::windows_core::Interface::vtable(this).BackgroundColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_ui::Color>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn SetBackgroundColor<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_ui::Color>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBackgroundColor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Attribution(&self) -> ::windows_core::Result<UserActivityAttribution> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Attribution)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserActivityAttribution>(result__)
        }
    }
    pub fn SetAttribution<'a, Param0: ::windows_core::IntoParam<'a, UserActivityAttribution>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAttribution)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn SetContent<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_ui::Shell::IAdaptiveCard>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContent)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Content(&self) -> ::windows_core::Result<::winrt_ui::Shell::IAdaptiveCard> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Content)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_ui::Shell::IAdaptiveCard>(result__)
        }
    }
    pub fn AttributionDisplayText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IUserActivityVisualElements2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AttributionDisplayText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetAttributionDisplayText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IUserActivityVisualElements2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAttributionDisplayText)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for UserActivityVisualElements {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserActivityVisualElements {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserActivityVisualElements {}
impl ::core::fmt::Debug for UserActivityVisualElements {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserActivityVisualElements").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserActivityVisualElements {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserActivities.UserActivityVisualElements;{94757513-262f-49ef-bbbf-9b75d2e85250})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserActivityVisualElements {
    type Vtable = IUserActivityVisualElements_Vtbl;
    const IID: ::windows_core::GUID = <IUserActivityVisualElements as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserActivityVisualElements {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivityVisualElements";
}
impl ::core::convert::From<UserActivityVisualElements> for ::windows_core::IUnknown {
    fn from(value: UserActivityVisualElements) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivityVisualElements> for ::windows_core::IUnknown {
    fn from(value: &UserActivityVisualElements) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserActivityVisualElements {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserActivityVisualElements {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserActivityVisualElements> for ::windows_core::IInspectable {
    fn from(value: UserActivityVisualElements) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserActivityVisualElements> for ::windows_core::IInspectable {
    fn from(value: &UserActivityVisualElements) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserActivityVisualElements {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserActivityVisualElements {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserActivityVisualElements {}
unsafe impl ::core::marker::Sync for UserActivityVisualElements {}
