#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentAction(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITargetedContentAction {
    type Vtable = ITargetedContentAction_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd75b691e_6cd6_4ca0_9d8f_4728b0b7e6b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentAction_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InvokeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentAvailabilityChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITargetedContentAvailabilityChangedEventArgs {
    type Vtable = ITargetedContentAvailabilityChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe0f59d26_5927_4450_965c_1ceb7becde65);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentAvailabilityChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITargetedContentChangedEventArgs {
    type Vtable = ITargetedContentChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x99d488c9_587e_4586_8ef7_b54ca9453a16);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub HasPreviousContentExpired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentCollection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITargetedContentCollection {
    type Vtable = ITargetedContentCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2d4b66c5_f163_44ba_9f6e_e1a4c2bb559d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentCollection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ReportInteraction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interaction: TargetedContentInteraction) -> ::windows_core::HRESULT,
    pub ReportCustomInteraction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, custominteractionname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Properties: usize,
    #[cfg(feature = "winrt-foundation")]
    pub Collections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Collections: usize,
    #[cfg(feature = "winrt-foundation")]
    pub Items: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Items: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentContainer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITargetedContentContainer {
    type Vtable = ITargetedContentContainer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbc2494c9_8837_47c2_850f_d79d64595926);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentContainer_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub Availability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TargetedContentAvailability) -> ::windows_core::HRESULT,
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectSingleObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentContainerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITargetedContentContainerStatics {
    type Vtable = ITargetedContentContainerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5b47e7fb_2140_4c1f_a736_c59583f227d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentContainerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentImage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITargetedContentImage {
    type Vtable = ITargetedContentImage_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa7a585d9_779f_4b1e_bbb1_8eaf53fbeab2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentImage_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentItem(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITargetedContentItem {
    type Vtable = ITargetedContentItem_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x38168dc4_276c_4c32_96ba_565c6e406e74);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentItem_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ReportInteraction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interaction: TargetedContentInteraction) -> ::windows_core::HRESULT,
    pub ReportCustomInteraction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, custominteractionname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Properties: usize,
    #[cfg(feature = "winrt-foundation")]
    pub Collections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Collections: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentItemState(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITargetedContentItemState {
    type Vtable = ITargetedContentItemState_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x73935454_4c65_4b47_a441_472de53c79b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentItemState_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ShouldDisplay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub AppInstallationState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TargetedContentAppInstallationState) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentObject(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITargetedContentObject {
    type Vtable = ITargetedContentObject_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x041d7969_2212_42d1_9dfa_88a8e3033aa3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentObject_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ObjectKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TargetedContentObjectKind) -> ::windows_core::HRESULT,
    pub Collection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentStateChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITargetedContentStateChangedEventArgs {
    type Vtable = ITargetedContentStateChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9a1cef3d_8073_4416_8df2_546835a6414f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentStateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentSubscription(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITargetedContentSubscription {
    type Vtable = ITargetedContentSubscription_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x882c2c49_c652_4c7a_acad_1f7fa2986c73);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentSubscription_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetContentContainerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ContentChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveContentChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub AvailabilityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveAvailabilityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub StateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentSubscriptionOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITargetedContentSubscriptionOptions {
    type Vtable = ITargetedContentSubscriptionOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x61ee6ad0_2c83_421b_8467_413eaf1aeb97);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentSubscriptionOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SubscriptionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AllowPartialContentAvailability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllowPartialContentAvailability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub CloudQueryParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    CloudQueryParameters: usize,
    #[cfg(feature = "winrt-foundation")]
    pub LocalFilters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    LocalFilters: usize,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentSubscriptionStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITargetedContentSubscriptionStatics {
    type Vtable = ITargetedContentSubscriptionStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfaddfe80_360d_4916_b53c_7ea27090d02a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentSubscriptionStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subscriptionid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subscriptionid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentValue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITargetedContentValue {
    type Vtable = ITargetedContentValue_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaafde4b3_4215_4bf8_867f_43f04865f9bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentValue_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ValueKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TargetedContentValueKind) -> ::windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub String: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Number: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub Boolean: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    File: usize,
    pub ImageFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Action: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Strings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Strings: usize,
    #[cfg(feature = "winrt-foundation")]
    pub Uris: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Uris: usize,
    #[cfg(feature = "winrt-foundation")]
    pub Numbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Numbers: usize,
    #[cfg(feature = "winrt-foundation")]
    pub Booleans: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Booleans: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage"))]
    pub Files: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-storage")))]
    Files: usize,
    #[cfg(feature = "winrt-foundation")]
    pub ImageFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ImageFiles: usize,
    #[cfg(feature = "winrt-foundation")]
    pub Actions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Actions: usize,
}
#[repr(transparent)]
pub struct TargetedContentAction(::windows_core::IUnknown);
impl TargetedContentAction {
    pub fn InvokeAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InvokeAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for TargetedContentAction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TargetedContentAction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentAction {}
impl ::core::fmt::Debug for TargetedContentAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentAction").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TargetedContentAction {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentAction;{d75b691e-6cd6-4ca0-9d8f-4728b0b7e6b6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TargetedContentAction {
    type Vtable = ITargetedContentAction_Vtbl;
    const IID: ::windows_core::GUID = <ITargetedContentAction as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TargetedContentAction {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentAction";
}
impl ::core::convert::From<TargetedContentAction> for ::windows_core::IUnknown {
    fn from(value: TargetedContentAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentAction> for ::windows_core::IUnknown {
    fn from(value: &TargetedContentAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TargetedContentAction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TargetedContentAction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TargetedContentAction> for ::windows_core::IInspectable {
    fn from(value: TargetedContentAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentAction> for ::windows_core::IInspectable {
    fn from(value: &TargetedContentAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TargetedContentAction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TargetedContentAction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TargetedContentAction {}
unsafe impl ::core::marker::Sync for TargetedContentAction {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TargetedContentAppInstallationState(pub i32);
impl TargetedContentAppInstallationState {
    pub const NotApplicable: Self = Self(0i32);
    pub const NotInstalled: Self = Self(1i32);
    pub const Installed: Self = Self(2i32);
}
impl ::core::marker::Copy for TargetedContentAppInstallationState {}
impl ::core::clone::Clone for TargetedContentAppInstallationState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TargetedContentAppInstallationState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TargetedContentAppInstallationState {
    type Abi = Self;
}
impl ::core::fmt::Debug for TargetedContentAppInstallationState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentAppInstallationState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TargetedContentAppInstallationState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Services.TargetedContent.TargetedContentAppInstallationState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TargetedContentAvailability(pub i32);
impl TargetedContentAvailability {
    pub const None: Self = Self(0i32);
    pub const Partial: Self = Self(1i32);
    pub const All: Self = Self(2i32);
}
impl ::core::marker::Copy for TargetedContentAvailability {}
impl ::core::clone::Clone for TargetedContentAvailability {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TargetedContentAvailability {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TargetedContentAvailability {
    type Abi = Self;
}
impl ::core::fmt::Debug for TargetedContentAvailability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentAvailability").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TargetedContentAvailability {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Services.TargetedContent.TargetedContentAvailability;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct TargetedContentAvailabilityChangedEventArgs(::windows_core::IUnknown);
impl TargetedContentAvailabilityChangedEventArgs {
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for TargetedContentAvailabilityChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TargetedContentAvailabilityChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentAvailabilityChangedEventArgs {}
impl ::core::fmt::Debug for TargetedContentAvailabilityChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentAvailabilityChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TargetedContentAvailabilityChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentAvailabilityChangedEventArgs;{e0f59d26-5927-4450-965c-1ceb7becde65})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TargetedContentAvailabilityChangedEventArgs {
    type Vtable = ITargetedContentAvailabilityChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ITargetedContentAvailabilityChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TargetedContentAvailabilityChangedEventArgs {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentAvailabilityChangedEventArgs";
}
impl ::core::convert::From<TargetedContentAvailabilityChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: TargetedContentAvailabilityChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentAvailabilityChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &TargetedContentAvailabilityChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TargetedContentAvailabilityChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TargetedContentAvailabilityChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TargetedContentAvailabilityChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: TargetedContentAvailabilityChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentAvailabilityChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &TargetedContentAvailabilityChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TargetedContentAvailabilityChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TargetedContentAvailabilityChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TargetedContentAvailabilityChangedEventArgs {}
unsafe impl ::core::marker::Sync for TargetedContentAvailabilityChangedEventArgs {}
#[repr(transparent)]
pub struct TargetedContentChangedEventArgs(::windows_core::IUnknown);
impl TargetedContentChangedEventArgs {
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
    pub fn HasPreviousContentExpired(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasPreviousContentExpired)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for TargetedContentChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TargetedContentChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentChangedEventArgs {}
impl ::core::fmt::Debug for TargetedContentChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TargetedContentChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentChangedEventArgs;{99d488c9-587e-4586-8ef7-b54ca9453a16})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TargetedContentChangedEventArgs {
    type Vtable = ITargetedContentChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ITargetedContentChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TargetedContentChangedEventArgs {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentChangedEventArgs";
}
impl ::core::convert::From<TargetedContentChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: TargetedContentChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &TargetedContentChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TargetedContentChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TargetedContentChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TargetedContentChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: TargetedContentChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &TargetedContentChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TargetedContentChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TargetedContentChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TargetedContentChangedEventArgs {}
unsafe impl ::core::marker::Sync for TargetedContentChangedEventArgs {}
#[repr(transparent)]
pub struct TargetedContentCollection(::windows_core::IUnknown);
impl TargetedContentCollection {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ReportInteraction(&self, interaction: TargetedContentInteraction) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportInteraction)(::windows_core::Interface::as_raw(this), interaction).ok() }
    }
    pub fn ReportCustomInteraction<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, custominteractionname: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportCustomInteraction)(::windows_core::Interface::as_raw(this), custominteractionname.into_param().abi()).ok() }
    }
    pub fn Path(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Path)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, TargetedContentValue>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, TargetedContentValue>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Collections(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<TargetedContentCollection>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Collections)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<TargetedContentCollection>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Items(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<TargetedContentItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Items)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<TargetedContentItem>>(result__)
        }
    }
}
impl ::core::clone::Clone for TargetedContentCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TargetedContentCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentCollection {}
impl ::core::fmt::Debug for TargetedContentCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TargetedContentCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentCollection;{2d4b66c5-f163-44ba-9f6e-e1a4c2bb559d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TargetedContentCollection {
    type Vtable = ITargetedContentCollection_Vtbl;
    const IID: ::windows_core::GUID = <ITargetedContentCollection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TargetedContentCollection {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentCollection";
}
impl ::core::convert::From<TargetedContentCollection> for ::windows_core::IUnknown {
    fn from(value: TargetedContentCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentCollection> for ::windows_core::IUnknown {
    fn from(value: &TargetedContentCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TargetedContentCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TargetedContentCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TargetedContentCollection> for ::windows_core::IInspectable {
    fn from(value: TargetedContentCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentCollection> for ::windows_core::IInspectable {
    fn from(value: &TargetedContentCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TargetedContentCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TargetedContentCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TargetedContentCollection {}
unsafe impl ::core::marker::Sync for TargetedContentCollection {}
#[repr(transparent)]
pub struct TargetedContentContainer(::windows_core::IUnknown);
impl TargetedContentContainer {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Timestamp(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn Availability(&self) -> ::windows_core::Result<TargetedContentAvailability> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<TargetedContentAvailability>::zeroed();
            (::windows_core::Interface::vtable(this).Availability)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TargetedContentAvailability>(result__)
        }
    }
    pub fn Content(&self) -> ::windows_core::Result<TargetedContentCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Content)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TargetedContentCollection>(result__)
        }
    }
    pub fn SelectSingleObject<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, path: Param0) -> ::windows_core::Result<TargetedContentObject> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleObject)(::windows_core::Interface::as_raw(this), path.into_param().abi(), result__.as_mut_ptr()).from_abi::<TargetedContentObject>(result__)
        }
    }
    pub fn GetAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(contentid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<TargetedContentContainer>> {
        Self::ITargetedContentContainerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAsync)(::windows_core::Interface::as_raw(this), contentid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<TargetedContentContainer>>(result__)
        })
    }
    pub fn ITargetedContentContainerStatics<R, F: FnOnce(&ITargetedContentContainerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TargetedContentContainer, ITargetedContentContainerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TargetedContentContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TargetedContentContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentContainer {}
impl ::core::fmt::Debug for TargetedContentContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentContainer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TargetedContentContainer {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentContainer;{bc2494c9-8837-47c2-850f-d79d64595926})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TargetedContentContainer {
    type Vtable = ITargetedContentContainer_Vtbl;
    const IID: ::windows_core::GUID = <ITargetedContentContainer as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TargetedContentContainer {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentContainer";
}
impl ::core::convert::From<TargetedContentContainer> for ::windows_core::IUnknown {
    fn from(value: TargetedContentContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentContainer> for ::windows_core::IUnknown {
    fn from(value: &TargetedContentContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TargetedContentContainer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TargetedContentContainer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TargetedContentContainer> for ::windows_core::IInspectable {
    fn from(value: TargetedContentContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentContainer> for ::windows_core::IInspectable {
    fn from(value: &TargetedContentContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TargetedContentContainer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TargetedContentContainer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TargetedContentContainer {}
unsafe impl ::core::marker::Sync for TargetedContentContainer {}
#[cfg(feature = "winrt-storage")]
#[repr(transparent)]
pub struct TargetedContentFile(::windows_core::IUnknown);
#[cfg(feature = "winrt-storage")]
impl TargetedContentFile {
    #[cfg(feature = "winrt-storage")]
    pub fn OpenReadAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_storage::Streams::IRandomAccessStreamWithContentType>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OpenReadAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_storage::Streams::IRandomAccessStreamWithContentType>>(result__)
        }
    }
}
#[cfg(feature = "winrt-storage")]
impl ::core::clone::Clone for TargetedContentFile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-storage")]
impl ::core::cmp::PartialEq for TargetedContentFile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-storage")]
impl ::core::cmp::Eq for TargetedContentFile {}
#[cfg(feature = "winrt-storage")]
impl ::core::fmt::Debug for TargetedContentFile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentFile").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-storage")]
unsafe impl ::windows_core::RuntimeType for TargetedContentFile {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentFile;{33ee3134-1dd6-4e3a-8067-d1c162e8642b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-storage")]
unsafe impl ::windows_core::Interface for TargetedContentFile {
    type Vtable = ::winrt_storage::Streams::IRandomAccessStreamReference_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_storage::Streams::IRandomAccessStreamReference as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-storage")]
impl ::windows_core::RuntimeName for TargetedContentFile {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentFile";
}
#[cfg(feature = "winrt-storage")]
impl ::core::convert::From<TargetedContentFile> for ::windows_core::IUnknown {
    fn from(value: TargetedContentFile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-storage")]
impl ::core::convert::From<&TargetedContentFile> for ::windows_core::IUnknown {
    fn from(value: &TargetedContentFile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-storage")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TargetedContentFile {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-storage")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TargetedContentFile {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-storage")]
impl ::core::convert::From<TargetedContentFile> for ::windows_core::IInspectable {
    fn from(value: TargetedContentFile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-storage")]
impl ::core::convert::From<&TargetedContentFile> for ::windows_core::IInspectable {
    fn from(value: &TargetedContentFile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-storage")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TargetedContentFile {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-storage")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TargetedContentFile {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-storage")]
impl ::core::convert::TryFrom<TargetedContentFile> for ::winrt_storage::Streams::IRandomAccessStreamReference {
    type Error = ::windows_core::Error;
    fn try_from(value: TargetedContentFile) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-storage")]
impl ::core::convert::TryFrom<&TargetedContentFile> for ::winrt_storage::Streams::IRandomAccessStreamReference {
    type Error = ::windows_core::Error;
    fn try_from(value: &TargetedContentFile) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-storage")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStreamReference> for TargetedContentFile {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IRandomAccessStreamReference> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-storage")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStreamReference> for &TargetedContentFile {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IRandomAccessStreamReference> {
        ::core::convert::TryInto::<::winrt_storage::Streams::IRandomAccessStreamReference>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-storage")]
unsafe impl ::core::marker::Send for TargetedContentFile {}
#[cfg(feature = "winrt-storage")]
unsafe impl ::core::marker::Sync for TargetedContentFile {}
#[repr(transparent)]
pub struct TargetedContentImage(::windows_core::IUnknown);
impl TargetedContentImage {
    #[cfg(feature = "winrt-storage")]
    pub fn OpenReadAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_storage::Streams::IRandomAccessStreamWithContentType>> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IRandomAccessStreamReference>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OpenReadAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_storage::Streams::IRandomAccessStreamWithContentType>>(result__)
        }
    }
    pub fn Height(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Height)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Width(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Width)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for TargetedContentImage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TargetedContentImage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentImage {}
impl ::core::fmt::Debug for TargetedContentImage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentImage").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TargetedContentImage {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentImage;{a7a585d9-779f-4b1e-bbb1-8eaf53fbeab2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TargetedContentImage {
    type Vtable = ITargetedContentImage_Vtbl;
    const IID: ::windows_core::GUID = <ITargetedContentImage as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TargetedContentImage {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentImage";
}
impl ::core::convert::From<TargetedContentImage> for ::windows_core::IUnknown {
    fn from(value: TargetedContentImage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentImage> for ::windows_core::IUnknown {
    fn from(value: &TargetedContentImage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TargetedContentImage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TargetedContentImage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TargetedContentImage> for ::windows_core::IInspectable {
    fn from(value: TargetedContentImage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentImage> for ::windows_core::IInspectable {
    fn from(value: &TargetedContentImage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TargetedContentImage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TargetedContentImage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-storage")]
impl ::core::convert::TryFrom<TargetedContentImage> for ::winrt_storage::Streams::IRandomAccessStreamReference {
    type Error = ::windows_core::Error;
    fn try_from(value: TargetedContentImage) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-storage")]
impl ::core::convert::TryFrom<&TargetedContentImage> for ::winrt_storage::Streams::IRandomAccessStreamReference {
    type Error = ::windows_core::Error;
    fn try_from(value: &TargetedContentImage) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-storage")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStreamReference> for TargetedContentImage {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IRandomAccessStreamReference> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-storage")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStreamReference> for &TargetedContentImage {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IRandomAccessStreamReference> {
        ::core::convert::TryInto::<::winrt_storage::Streams::IRandomAccessStreamReference>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for TargetedContentImage {}
unsafe impl ::core::marker::Sync for TargetedContentImage {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TargetedContentInteraction(pub i32);
impl TargetedContentInteraction {
    pub const Impression: Self = Self(0i32);
    pub const ClickThrough: Self = Self(1i32);
    pub const Hover: Self = Self(2i32);
    pub const Like: Self = Self(3i32);
    pub const Dislike: Self = Self(4i32);
    pub const Dismiss: Self = Self(5i32);
    pub const Ineligible: Self = Self(6i32);
    pub const Accept: Self = Self(7i32);
    pub const Decline: Self = Self(8i32);
    pub const Defer: Self = Self(9i32);
    pub const Canceled: Self = Self(10i32);
    pub const Conversion: Self = Self(11i32);
    pub const Opportunity: Self = Self(12i32);
}
impl ::core::marker::Copy for TargetedContentInteraction {}
impl ::core::clone::Clone for TargetedContentInteraction {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TargetedContentInteraction {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TargetedContentInteraction {
    type Abi = Self;
}
impl ::core::fmt::Debug for TargetedContentInteraction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentInteraction").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TargetedContentInteraction {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Services.TargetedContent.TargetedContentInteraction;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct TargetedContentItem(::windows_core::IUnknown);
impl TargetedContentItem {
    pub fn Path(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Path)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ReportInteraction(&self, interaction: TargetedContentInteraction) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportInteraction)(::windows_core::Interface::as_raw(this), interaction).ok() }
    }
    pub fn ReportCustomInteraction<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, custominteractionname: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportCustomInteraction)(::windows_core::Interface::as_raw(this), custominteractionname.into_param().abi()).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<TargetedContentItemState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TargetedContentItemState>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, TargetedContentValue>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, TargetedContentValue>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Collections(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<TargetedContentCollection>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Collections)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<TargetedContentCollection>>(result__)
        }
    }
}
impl ::core::clone::Clone for TargetedContentItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TargetedContentItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentItem {}
impl ::core::fmt::Debug for TargetedContentItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentItem").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TargetedContentItem {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentItem;{38168dc4-276c-4c32-96ba-565c6e406e74})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TargetedContentItem {
    type Vtable = ITargetedContentItem_Vtbl;
    const IID: ::windows_core::GUID = <ITargetedContentItem as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TargetedContentItem {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentItem";
}
impl ::core::convert::From<TargetedContentItem> for ::windows_core::IUnknown {
    fn from(value: TargetedContentItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentItem> for ::windows_core::IUnknown {
    fn from(value: &TargetedContentItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TargetedContentItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TargetedContentItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TargetedContentItem> for ::windows_core::IInspectable {
    fn from(value: TargetedContentItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentItem> for ::windows_core::IInspectable {
    fn from(value: &TargetedContentItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TargetedContentItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TargetedContentItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TargetedContentItem {}
unsafe impl ::core::marker::Sync for TargetedContentItem {}
#[repr(transparent)]
pub struct TargetedContentItemState(::windows_core::IUnknown);
impl TargetedContentItemState {
    pub fn ShouldDisplay(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ShouldDisplay)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn AppInstallationState(&self) -> ::windows_core::Result<TargetedContentAppInstallationState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<TargetedContentAppInstallationState>::zeroed();
            (::windows_core::Interface::vtable(this).AppInstallationState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TargetedContentAppInstallationState>(result__)
        }
    }
}
impl ::core::clone::Clone for TargetedContentItemState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TargetedContentItemState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentItemState {}
impl ::core::fmt::Debug for TargetedContentItemState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentItemState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TargetedContentItemState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentItemState;{73935454-4c65-4b47-a441-472de53c79b6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TargetedContentItemState {
    type Vtable = ITargetedContentItemState_Vtbl;
    const IID: ::windows_core::GUID = <ITargetedContentItemState as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TargetedContentItemState {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentItemState";
}
impl ::core::convert::From<TargetedContentItemState> for ::windows_core::IUnknown {
    fn from(value: TargetedContentItemState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentItemState> for ::windows_core::IUnknown {
    fn from(value: &TargetedContentItemState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TargetedContentItemState {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TargetedContentItemState {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TargetedContentItemState> for ::windows_core::IInspectable {
    fn from(value: TargetedContentItemState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentItemState> for ::windows_core::IInspectable {
    fn from(value: &TargetedContentItemState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TargetedContentItemState {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TargetedContentItemState {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TargetedContentItemState {}
unsafe impl ::core::marker::Sync for TargetedContentItemState {}
#[repr(transparent)]
pub struct TargetedContentObject(::windows_core::IUnknown);
impl TargetedContentObject {
    pub fn ObjectKind(&self) -> ::windows_core::Result<TargetedContentObjectKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<TargetedContentObjectKind>::zeroed();
            (::windows_core::Interface::vtable(this).ObjectKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TargetedContentObjectKind>(result__)
        }
    }
    pub fn Collection(&self) -> ::windows_core::Result<TargetedContentCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Collection)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TargetedContentCollection>(result__)
        }
    }
    pub fn Item(&self) -> ::windows_core::Result<TargetedContentItem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Item)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TargetedContentItem>(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<TargetedContentValue> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TargetedContentValue>(result__)
        }
    }
}
impl ::core::clone::Clone for TargetedContentObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TargetedContentObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentObject {}
impl ::core::fmt::Debug for TargetedContentObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentObject").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TargetedContentObject {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentObject;{041d7969-2212-42d1-9dfa-88a8e3033aa3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TargetedContentObject {
    type Vtable = ITargetedContentObject_Vtbl;
    const IID: ::windows_core::GUID = <ITargetedContentObject as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TargetedContentObject {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentObject";
}
impl ::core::convert::From<TargetedContentObject> for ::windows_core::IUnknown {
    fn from(value: TargetedContentObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentObject> for ::windows_core::IUnknown {
    fn from(value: &TargetedContentObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TargetedContentObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TargetedContentObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TargetedContentObject> for ::windows_core::IInspectable {
    fn from(value: TargetedContentObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentObject> for ::windows_core::IInspectable {
    fn from(value: &TargetedContentObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TargetedContentObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TargetedContentObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TargetedContentObject {}
unsafe impl ::core::marker::Sync for TargetedContentObject {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TargetedContentObjectKind(pub i32);
impl TargetedContentObjectKind {
    pub const Collection: Self = Self(0i32);
    pub const Item: Self = Self(1i32);
    pub const Value: Self = Self(2i32);
}
impl ::core::marker::Copy for TargetedContentObjectKind {}
impl ::core::clone::Clone for TargetedContentObjectKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TargetedContentObjectKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TargetedContentObjectKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for TargetedContentObjectKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentObjectKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TargetedContentObjectKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Services.TargetedContent.TargetedContentObjectKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct TargetedContentStateChangedEventArgs(::windows_core::IUnknown);
impl TargetedContentStateChangedEventArgs {
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for TargetedContentStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TargetedContentStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentStateChangedEventArgs {}
impl ::core::fmt::Debug for TargetedContentStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentStateChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TargetedContentStateChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentStateChangedEventArgs;{9a1cef3d-8073-4416-8df2-546835a6414f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TargetedContentStateChangedEventArgs {
    type Vtable = ITargetedContentStateChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ITargetedContentStateChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TargetedContentStateChangedEventArgs {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentStateChangedEventArgs";
}
impl ::core::convert::From<TargetedContentStateChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: TargetedContentStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentStateChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &TargetedContentStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TargetedContentStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TargetedContentStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TargetedContentStateChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: TargetedContentStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentStateChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &TargetedContentStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TargetedContentStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TargetedContentStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TargetedContentStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for TargetedContentStateChangedEventArgs {}
#[repr(transparent)]
pub struct TargetedContentSubscription(::windows_core::IUnknown);
impl TargetedContentSubscription {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn GetContentContainerAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<TargetedContentContainer>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetContentContainerAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<TargetedContentContainer>>(result__)
        }
    }
    pub fn ContentChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ContentChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveContentChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveContentChanged)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
    pub fn AvailabilityChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentAvailabilityChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).AvailabilityChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAvailabilityChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAvailabilityChanged)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
    pub fn StateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).StateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveStateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStateChanged)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
    pub fn GetAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(subscriptionid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<TargetedContentSubscription>> {
        Self::ITargetedContentSubscriptionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAsync)(::windows_core::Interface::as_raw(this), subscriptionid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<TargetedContentSubscription>>(result__)
        })
    }
    pub fn GetOptions<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(subscriptionid: Param0) -> ::windows_core::Result<TargetedContentSubscriptionOptions> {
        Self::ITargetedContentSubscriptionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetOptions)(::windows_core::Interface::as_raw(this), subscriptionid.into_param().abi(), result__.as_mut_ptr()).from_abi::<TargetedContentSubscriptionOptions>(result__)
        })
    }
    pub fn ITargetedContentSubscriptionStatics<R, F: FnOnce(&ITargetedContentSubscriptionStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TargetedContentSubscription, ITargetedContentSubscriptionStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TargetedContentSubscription {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TargetedContentSubscription {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentSubscription {}
impl ::core::fmt::Debug for TargetedContentSubscription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentSubscription").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TargetedContentSubscription {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentSubscription;{882c2c49-c652-4c7a-acad-1f7fa2986c73})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TargetedContentSubscription {
    type Vtable = ITargetedContentSubscription_Vtbl;
    const IID: ::windows_core::GUID = <ITargetedContentSubscription as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TargetedContentSubscription {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentSubscription";
}
impl ::core::convert::From<TargetedContentSubscription> for ::windows_core::IUnknown {
    fn from(value: TargetedContentSubscription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentSubscription> for ::windows_core::IUnknown {
    fn from(value: &TargetedContentSubscription) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TargetedContentSubscription {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TargetedContentSubscription {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TargetedContentSubscription> for ::windows_core::IInspectable {
    fn from(value: TargetedContentSubscription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentSubscription> for ::windows_core::IInspectable {
    fn from(value: &TargetedContentSubscription) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TargetedContentSubscription {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TargetedContentSubscription {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TargetedContentSubscription {}
unsafe impl ::core::marker::Sync for TargetedContentSubscription {}
#[repr(transparent)]
pub struct TargetedContentSubscriptionOptions(::windows_core::IUnknown);
impl TargetedContentSubscriptionOptions {
    pub fn SubscriptionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SubscriptionId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AllowPartialContentAvailability(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AllowPartialContentAvailability)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowPartialContentAvailability(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowPartialContentAvailability)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn CloudQueryParameters(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CloudQueryParameters)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn LocalFilters(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LocalFilters)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn Update(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Update)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for TargetedContentSubscriptionOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TargetedContentSubscriptionOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentSubscriptionOptions {}
impl ::core::fmt::Debug for TargetedContentSubscriptionOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentSubscriptionOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TargetedContentSubscriptionOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentSubscriptionOptions;{61ee6ad0-2c83-421b-8467-413eaf1aeb97})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TargetedContentSubscriptionOptions {
    type Vtable = ITargetedContentSubscriptionOptions_Vtbl;
    const IID: ::windows_core::GUID = <ITargetedContentSubscriptionOptions as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TargetedContentSubscriptionOptions {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentSubscriptionOptions";
}
impl ::core::convert::From<TargetedContentSubscriptionOptions> for ::windows_core::IUnknown {
    fn from(value: TargetedContentSubscriptionOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentSubscriptionOptions> for ::windows_core::IUnknown {
    fn from(value: &TargetedContentSubscriptionOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TargetedContentSubscriptionOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TargetedContentSubscriptionOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TargetedContentSubscriptionOptions> for ::windows_core::IInspectable {
    fn from(value: TargetedContentSubscriptionOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentSubscriptionOptions> for ::windows_core::IInspectable {
    fn from(value: &TargetedContentSubscriptionOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TargetedContentSubscriptionOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TargetedContentSubscriptionOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TargetedContentSubscriptionOptions {}
unsafe impl ::core::marker::Sync for TargetedContentSubscriptionOptions {}
#[repr(transparent)]
pub struct TargetedContentValue(::windows_core::IUnknown);
impl TargetedContentValue {
    pub fn ValueKind(&self) -> ::windows_core::Result<TargetedContentValueKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<TargetedContentValueKind>::zeroed();
            (::windows_core::Interface::vtable(this).ValueKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TargetedContentValueKind>(result__)
        }
    }
    pub fn Path(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Path)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn String(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).String)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Uri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn Number(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Number)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn Boolean(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Boolean)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn File(&self) -> ::windows_core::Result<TargetedContentFile> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).File)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TargetedContentFile>(result__)
        }
    }
    pub fn ImageFile(&self) -> ::windows_core::Result<TargetedContentImage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImageFile)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TargetedContentImage>(result__)
        }
    }
    pub fn Action(&self) -> ::windows_core::Result<TargetedContentAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Action)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TargetedContentAction>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Strings(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Strings)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Uris(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uris)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Uri>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Numbers(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Numbers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<f64>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Booleans(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Booleans)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<bool>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage"))]
    pub fn Files(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<TargetedContentFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Files)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<TargetedContentFile>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ImageFiles(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<TargetedContentImage>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImageFiles)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<TargetedContentImage>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Actions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<TargetedContentAction>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Actions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<TargetedContentAction>>(result__)
        }
    }
}
impl ::core::clone::Clone for TargetedContentValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TargetedContentValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentValue {}
impl ::core::fmt::Debug for TargetedContentValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentValue").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TargetedContentValue {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentValue;{aafde4b3-4215-4bf8-867f-43f04865f9bf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TargetedContentValue {
    type Vtable = ITargetedContentValue_Vtbl;
    const IID: ::windows_core::GUID = <ITargetedContentValue as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TargetedContentValue {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentValue";
}
impl ::core::convert::From<TargetedContentValue> for ::windows_core::IUnknown {
    fn from(value: TargetedContentValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentValue> for ::windows_core::IUnknown {
    fn from(value: &TargetedContentValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TargetedContentValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TargetedContentValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TargetedContentValue> for ::windows_core::IInspectable {
    fn from(value: TargetedContentValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentValue> for ::windows_core::IInspectable {
    fn from(value: &TargetedContentValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TargetedContentValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TargetedContentValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TargetedContentValue {}
unsafe impl ::core::marker::Sync for TargetedContentValue {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TargetedContentValueKind(pub i32);
impl TargetedContentValueKind {
    pub const String: Self = Self(0i32);
    pub const Uri: Self = Self(1i32);
    pub const Number: Self = Self(2i32);
    pub const Boolean: Self = Self(3i32);
    pub const File: Self = Self(4i32);
    pub const ImageFile: Self = Self(5i32);
    pub const Action: Self = Self(6i32);
    pub const Strings: Self = Self(7i32);
    pub const Uris: Self = Self(8i32);
    pub const Numbers: Self = Self(9i32);
    pub const Booleans: Self = Self(10i32);
    pub const Files: Self = Self(11i32);
    pub const ImageFiles: Self = Self(12i32);
    pub const Actions: Self = Self(13i32);
}
impl ::core::marker::Copy for TargetedContentValueKind {}
impl ::core::clone::Clone for TargetedContentValueKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TargetedContentValueKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TargetedContentValueKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for TargetedContentValueKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentValueKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TargetedContentValueKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Services.TargetedContent.TargetedContentValueKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
