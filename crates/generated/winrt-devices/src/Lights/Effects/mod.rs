#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArrayBitmapEffect(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILampArrayBitmapEffect {
    type Vtable = ILampArrayBitmapEffect_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3238e065_d877_4627_89e5_2a88f7052fa6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayBitmapEffect_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub StartDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetStartDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub UpdateInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetUpdateInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SuggestedBitmapSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Size) -> ::windows_core::HRESULT,
    pub BitmapRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveBitmapRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArrayBitmapEffectFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILampArrayBitmapEffectFactory {
    type Vtable = ILampArrayBitmapEffectFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x13608090_e336_4c8f_9053_a92407ca7b1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayBitmapEffectFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lamparray: ::windows_core::RawPtr, lampIndexes_array_size: u32, lampindexes: *const i32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArrayBitmapRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILampArrayBitmapRequestedEventArgs {
    type Vtable = ILampArrayBitmapRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc8b4af9e_fe63_4d51_babd_619defb454ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayBitmapRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SinceStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-graphics")]
    pub UpdateBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    UpdateBitmap: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArrayBlinkEffect(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILampArrayBlinkEffect {
    type Vtable = ILampArrayBlinkEffect_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xebbf35f6_2fc5_4bb3_b3c3_6221a7680d13);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayBlinkEffect_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-ui")]
    pub Color: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    Color: usize,
    #[cfg(feature = "winrt-ui")]
    pub SetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    SetColor: usize,
    pub AttackDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetAttackDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SustainDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetSustainDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub DecayDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetDecayDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub RepetitionDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetRepetitionDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub StartDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetStartDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub Occurrences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetOccurrences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub RepetitionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LampArrayRepetitionMode) -> ::windows_core::HRESULT,
    pub SetRepetitionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: LampArrayRepetitionMode) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArrayBlinkEffectFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILampArrayBlinkEffectFactory {
    type Vtable = ILampArrayBlinkEffectFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x879f1d97_9f50_49b2_a56f_013aa08d55e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayBlinkEffectFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lamparray: ::windows_core::RawPtr, lampIndexes_array_size: u32, lampindexes: *const i32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArrayColorRampEffect(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILampArrayColorRampEffect {
    type Vtable = ILampArrayColorRampEffect_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2b004437_40a7_432e_a0b9_0d570c2153ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayColorRampEffect_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-ui")]
    pub Color: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    Color: usize,
    #[cfg(feature = "winrt-ui")]
    pub SetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    SetColor: usize,
    pub RampDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetRampDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub StartDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetStartDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub CompletionBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LampArrayEffectCompletionBehavior) -> ::windows_core::HRESULT,
    pub SetCompletionBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: LampArrayEffectCompletionBehavior) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArrayColorRampEffectFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILampArrayColorRampEffectFactory {
    type Vtable = ILampArrayColorRampEffectFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x520bd133_0c74_4df5_bea7_4899e0266b0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayColorRampEffectFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lamparray: ::windows_core::RawPtr, lampIndexes_array_size: u32, lampindexes: *const i32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArrayCustomEffect(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILampArrayCustomEffect {
    type Vtable = ILampArrayCustomEffect_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xec579170_3c34_4876_818b_5765f78b0ee4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayCustomEffect_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub UpdateInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetUpdateInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub UpdateRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveUpdateRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArrayCustomEffectFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILampArrayCustomEffectFactory {
    type Vtable = ILampArrayCustomEffectFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x68b4774d_63e5_4af0_a58b_3e535b94e8c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayCustomEffectFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lamparray: ::windows_core::RawPtr, lampIndexes_array_size: u32, lampindexes: *const i32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ILampArrayEffect(::windows_core::IUnknown);
impl ILampArrayEffect {
    pub fn ZIndex(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).ZIndex)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetZIndex(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetZIndex)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::convert::From<ILampArrayEffect> for ::windows_core::IUnknown {
    fn from(value: ILampArrayEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILampArrayEffect> for ::windows_core::IUnknown {
    fn from(value: &ILampArrayEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ILampArrayEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ILampArrayEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ILampArrayEffect> for ::windows_core::IInspectable {
    fn from(value: ILampArrayEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILampArrayEffect> for ::windows_core::IInspectable {
    fn from(value: &ILampArrayEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ILampArrayEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ILampArrayEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ILampArrayEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILampArrayEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILampArrayEffect {}
impl ::core::fmt::Debug for ILampArrayEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILampArrayEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ILampArrayEffect {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{11d45590-57fb-4546-b1ce-863107f740df}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ILampArrayEffect {
    type Vtable = ILampArrayEffect_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x11d45590_57fb_4546_b1ce_863107f740df);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayEffect_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ZIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetZIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArrayEffectPlaylist(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILampArrayEffectPlaylist {
    type Vtable = ILampArrayEffectPlaylist_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7de58bfe_6f61_4103_98c7_d6632f7b9169);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayEffectPlaylist_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effect: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OverrideZIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, zindex: i32) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EffectStartMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LampArrayEffectStartMode) -> ::windows_core::HRESULT,
    pub SetEffectStartMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: LampArrayEffectStartMode) -> ::windows_core::HRESULT,
    pub Occurrences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetOccurrences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub RepetitionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LampArrayRepetitionMode) -> ::windows_core::HRESULT,
    pub SetRepetitionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: LampArrayRepetitionMode) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArrayEffectPlaylistStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILampArrayEffectPlaylistStatics {
    type Vtable = ILampArrayEffectPlaylistStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfb15235c_ea35_4c7f_a016_f3bfc6a6c47d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayEffectPlaylistStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub StartAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    StartAll: usize,
    #[cfg(feature = "winrt-foundation")]
    pub StopAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    StopAll: usize,
    #[cfg(feature = "winrt-foundation")]
    pub PauseAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    PauseAll: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArraySolidEffect(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILampArraySolidEffect {
    type Vtable = ILampArraySolidEffect_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x441f8213_43cc_4b33_80eb_c6ddde7dc8ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArraySolidEffect_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-ui")]
    pub Color: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    Color: usize,
    #[cfg(feature = "winrt-ui")]
    pub SetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    SetColor: usize,
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub StartDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetStartDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub CompletionBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LampArrayEffectCompletionBehavior) -> ::windows_core::HRESULT,
    pub SetCompletionBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: LampArrayEffectCompletionBehavior) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArraySolidEffectFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILampArraySolidEffectFactory {
    type Vtable = ILampArraySolidEffectFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf862a32c_5576_4341_961b_aee1f13cf9dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArraySolidEffectFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lamparray: ::windows_core::RawPtr, lampIndexes_array_size: u32, lampindexes: *const i32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArrayUpdateRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILampArrayUpdateRequestedEventArgs {
    type Vtable = ILampArrayUpdateRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x73560d6a_576a_48af_8539_67ffa0ab3516);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayUpdateRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SinceStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub SetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredcolor: ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    SetColor: usize,
    #[cfg(feature = "winrt-ui")]
    pub SetColorForIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lampindex: i32, desiredcolor: ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    SetColorForIndex: usize,
    #[cfg(feature = "winrt-ui")]
    pub SetSingleColorForIndices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredcolor: ::winrt_ui::Color, lampIndexes_array_size: u32, lampindexes: *const i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    SetSingleColorForIndices: usize,
    #[cfg(feature = "winrt-ui")]
    pub SetColorsForIndices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredColors_array_size: u32, desiredcolors: *const ::winrt_ui::Color, lampIndexes_array_size: u32, lampindexes: *const i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    SetColorsForIndices: usize,
}
#[repr(transparent)]
pub struct LampArrayBitmapEffect(::windows_core::IUnknown);
impl LampArrayBitmapEffect {
    pub fn Duration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetDuration<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartDelay(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).StartDelay)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetStartDelay<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStartDelay)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn UpdateInterval(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).UpdateInterval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetUpdateInterval<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUpdateInterval)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SuggestedBitmapSize(&self) -> ::windows_core::Result<::winrt_foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Size>::zeroed();
            (::windows_core::Interface::vtable(this).SuggestedBitmapSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Size>(result__)
        }
    }
    pub fn BitmapRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<LampArrayBitmapEffect, LampArrayBitmapRequestedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).BitmapRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveBitmapRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveBitmapRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn CreateInstance<'a, Param0: ::windows_core::IntoParam<'a, super::LampArray>>(lamparray: Param0, lampindexes: &[i32]) -> ::windows_core::Result<LampArrayBitmapEffect> {
        Self::ILampArrayBitmapEffectFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), lamparray.into_param().abi(), lampindexes.len() as u32, ::core::mem::transmute(lampindexes.as_ptr()), result__.as_mut_ptr()).from_abi::<LampArrayBitmapEffect>(result__)
        })
    }
    pub fn ZIndex(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).ZIndex)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetZIndex(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetZIndex)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ILampArrayBitmapEffectFactory<R, F: FnOnce(&ILampArrayBitmapEffectFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LampArrayBitmapEffect, ILampArrayBitmapEffectFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LampArrayBitmapEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LampArrayBitmapEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LampArrayBitmapEffect {}
impl ::core::fmt::Debug for LampArrayBitmapEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampArrayBitmapEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LampArrayBitmapEffect {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.Effects.LampArrayBitmapEffect;{3238e065-d877-4627-89e5-2a88f7052fa6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LampArrayBitmapEffect {
    type Vtable = ILampArrayBitmapEffect_Vtbl;
    const IID: ::windows_core::GUID = <ILampArrayBitmapEffect as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LampArrayBitmapEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArrayBitmapEffect";
}
impl ::core::convert::From<LampArrayBitmapEffect> for ::windows_core::IUnknown {
    fn from(value: LampArrayBitmapEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampArrayBitmapEffect> for ::windows_core::IUnknown {
    fn from(value: &LampArrayBitmapEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LampArrayBitmapEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LampArrayBitmapEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LampArrayBitmapEffect> for ::windows_core::IInspectable {
    fn from(value: LampArrayBitmapEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampArrayBitmapEffect> for ::windows_core::IInspectable {
    fn from(value: &LampArrayBitmapEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LampArrayBitmapEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LampArrayBitmapEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LampArrayBitmapEffect> for ILampArrayEffect {
    type Error = ::windows_core::Error;
    fn try_from(value: LampArrayBitmapEffect) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LampArrayBitmapEffect> for ILampArrayEffect {
    type Error = ::windows_core::Error;
    fn try_from(value: &LampArrayBitmapEffect) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ILampArrayEffect> for LampArrayBitmapEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ILampArrayEffect> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ILampArrayEffect> for &LampArrayBitmapEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ILampArrayEffect> {
        ::core::convert::TryInto::<ILampArrayEffect>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LampArrayBitmapEffect {}
unsafe impl ::core::marker::Sync for LampArrayBitmapEffect {}
#[repr(transparent)]
pub struct LampArrayBitmapRequestedEventArgs(::windows_core::IUnknown);
impl LampArrayBitmapRequestedEventArgs {
    pub fn SinceStarted(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).SinceStarted)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn UpdateBitmap<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::SoftwareBitmap>>(&self, bitmap: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).UpdateBitmap)(::windows_core::Interface::as_raw(this), bitmap.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for LampArrayBitmapRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LampArrayBitmapRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LampArrayBitmapRequestedEventArgs {}
impl ::core::fmt::Debug for LampArrayBitmapRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampArrayBitmapRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LampArrayBitmapRequestedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.Effects.LampArrayBitmapRequestedEventArgs;{c8b4af9e-fe63-4d51-babd-619defb454ba})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LampArrayBitmapRequestedEventArgs {
    type Vtable = ILampArrayBitmapRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ILampArrayBitmapRequestedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LampArrayBitmapRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArrayBitmapRequestedEventArgs";
}
impl ::core::convert::From<LampArrayBitmapRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: LampArrayBitmapRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampArrayBitmapRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &LampArrayBitmapRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LampArrayBitmapRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LampArrayBitmapRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LampArrayBitmapRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: LampArrayBitmapRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampArrayBitmapRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &LampArrayBitmapRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LampArrayBitmapRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LampArrayBitmapRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LampArrayBitmapRequestedEventArgs {}
unsafe impl ::core::marker::Sync for LampArrayBitmapRequestedEventArgs {}
#[repr(transparent)]
pub struct LampArrayBlinkEffect(::windows_core::IUnknown);
impl LampArrayBlinkEffect {
    #[cfg(feature = "winrt-ui")]
    pub fn Color(&self) -> ::windows_core::Result<::winrt_ui::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_ui::Color>::zeroed();
            (::windows_core::Interface::vtable(this).Color)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_ui::Color>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn SetColor<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_ui::Color>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetColor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn AttackDuration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).AttackDuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetAttackDuration<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAttackDuration)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SustainDuration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).SustainDuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetSustainDuration<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSustainDuration)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn DecayDuration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).DecayDuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetDecayDuration<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDecayDuration)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn RepetitionDelay(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).RepetitionDelay)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetRepetitionDelay<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRepetitionDelay)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartDelay(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).StartDelay)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetStartDelay<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStartDelay)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Occurrences(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Occurrences)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetOccurrences(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOccurrences)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RepetitionMode(&self) -> ::windows_core::Result<LampArrayRepetitionMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<LampArrayRepetitionMode>::zeroed();
            (::windows_core::Interface::vtable(this).RepetitionMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LampArrayRepetitionMode>(result__)
        }
    }
    pub fn SetRepetitionMode(&self, value: LampArrayRepetitionMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRepetitionMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateInstance<'a, Param0: ::windows_core::IntoParam<'a, super::LampArray>>(lamparray: Param0, lampindexes: &[i32]) -> ::windows_core::Result<LampArrayBlinkEffect> {
        Self::ILampArrayBlinkEffectFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), lamparray.into_param().abi(), lampindexes.len() as u32, ::core::mem::transmute(lampindexes.as_ptr()), result__.as_mut_ptr()).from_abi::<LampArrayBlinkEffect>(result__)
        })
    }
    pub fn ZIndex(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).ZIndex)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetZIndex(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetZIndex)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ILampArrayBlinkEffectFactory<R, F: FnOnce(&ILampArrayBlinkEffectFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LampArrayBlinkEffect, ILampArrayBlinkEffectFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LampArrayBlinkEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LampArrayBlinkEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LampArrayBlinkEffect {}
impl ::core::fmt::Debug for LampArrayBlinkEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampArrayBlinkEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LampArrayBlinkEffect {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.Effects.LampArrayBlinkEffect;{ebbf35f6-2fc5-4bb3-b3c3-6221a7680d13})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LampArrayBlinkEffect {
    type Vtable = ILampArrayBlinkEffect_Vtbl;
    const IID: ::windows_core::GUID = <ILampArrayBlinkEffect as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LampArrayBlinkEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArrayBlinkEffect";
}
impl ::core::convert::From<LampArrayBlinkEffect> for ::windows_core::IUnknown {
    fn from(value: LampArrayBlinkEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampArrayBlinkEffect> for ::windows_core::IUnknown {
    fn from(value: &LampArrayBlinkEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LampArrayBlinkEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LampArrayBlinkEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LampArrayBlinkEffect> for ::windows_core::IInspectable {
    fn from(value: LampArrayBlinkEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampArrayBlinkEffect> for ::windows_core::IInspectable {
    fn from(value: &LampArrayBlinkEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LampArrayBlinkEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LampArrayBlinkEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LampArrayBlinkEffect> for ILampArrayEffect {
    type Error = ::windows_core::Error;
    fn try_from(value: LampArrayBlinkEffect) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LampArrayBlinkEffect> for ILampArrayEffect {
    type Error = ::windows_core::Error;
    fn try_from(value: &LampArrayBlinkEffect) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ILampArrayEffect> for LampArrayBlinkEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ILampArrayEffect> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ILampArrayEffect> for &LampArrayBlinkEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ILampArrayEffect> {
        ::core::convert::TryInto::<ILampArrayEffect>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LampArrayBlinkEffect {}
unsafe impl ::core::marker::Sync for LampArrayBlinkEffect {}
#[repr(transparent)]
pub struct LampArrayColorRampEffect(::windows_core::IUnknown);
impl LampArrayColorRampEffect {
    #[cfg(feature = "winrt-ui")]
    pub fn Color(&self) -> ::windows_core::Result<::winrt_ui::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_ui::Color>::zeroed();
            (::windows_core::Interface::vtable(this).Color)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_ui::Color>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn SetColor<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_ui::Color>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetColor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn RampDuration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).RampDuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetRampDuration<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRampDuration)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartDelay(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).StartDelay)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetStartDelay<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStartDelay)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CompletionBehavior(&self) -> ::windows_core::Result<LampArrayEffectCompletionBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<LampArrayEffectCompletionBehavior>::zeroed();
            (::windows_core::Interface::vtable(this).CompletionBehavior)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LampArrayEffectCompletionBehavior>(result__)
        }
    }
    pub fn SetCompletionBehavior(&self, value: LampArrayEffectCompletionBehavior) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCompletionBehavior)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateInstance<'a, Param0: ::windows_core::IntoParam<'a, super::LampArray>>(lamparray: Param0, lampindexes: &[i32]) -> ::windows_core::Result<LampArrayColorRampEffect> {
        Self::ILampArrayColorRampEffectFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), lamparray.into_param().abi(), lampindexes.len() as u32, ::core::mem::transmute(lampindexes.as_ptr()), result__.as_mut_ptr()).from_abi::<LampArrayColorRampEffect>(result__)
        })
    }
    pub fn ZIndex(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).ZIndex)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetZIndex(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetZIndex)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ILampArrayColorRampEffectFactory<R, F: FnOnce(&ILampArrayColorRampEffectFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LampArrayColorRampEffect, ILampArrayColorRampEffectFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LampArrayColorRampEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LampArrayColorRampEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LampArrayColorRampEffect {}
impl ::core::fmt::Debug for LampArrayColorRampEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampArrayColorRampEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LampArrayColorRampEffect {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.Effects.LampArrayColorRampEffect;{2b004437-40a7-432e-a0b9-0d570c2153ff})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LampArrayColorRampEffect {
    type Vtable = ILampArrayColorRampEffect_Vtbl;
    const IID: ::windows_core::GUID = <ILampArrayColorRampEffect as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LampArrayColorRampEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArrayColorRampEffect";
}
impl ::core::convert::From<LampArrayColorRampEffect> for ::windows_core::IUnknown {
    fn from(value: LampArrayColorRampEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampArrayColorRampEffect> for ::windows_core::IUnknown {
    fn from(value: &LampArrayColorRampEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LampArrayColorRampEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LampArrayColorRampEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LampArrayColorRampEffect> for ::windows_core::IInspectable {
    fn from(value: LampArrayColorRampEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampArrayColorRampEffect> for ::windows_core::IInspectable {
    fn from(value: &LampArrayColorRampEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LampArrayColorRampEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LampArrayColorRampEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LampArrayColorRampEffect> for ILampArrayEffect {
    type Error = ::windows_core::Error;
    fn try_from(value: LampArrayColorRampEffect) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LampArrayColorRampEffect> for ILampArrayEffect {
    type Error = ::windows_core::Error;
    fn try_from(value: &LampArrayColorRampEffect) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ILampArrayEffect> for LampArrayColorRampEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ILampArrayEffect> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ILampArrayEffect> for &LampArrayColorRampEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ILampArrayEffect> {
        ::core::convert::TryInto::<ILampArrayEffect>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LampArrayColorRampEffect {}
unsafe impl ::core::marker::Sync for LampArrayColorRampEffect {}
#[repr(transparent)]
pub struct LampArrayCustomEffect(::windows_core::IUnknown);
impl LampArrayCustomEffect {
    pub fn Duration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetDuration<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn UpdateInterval(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).UpdateInterval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetUpdateInterval<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUpdateInterval)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn UpdateRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<LampArrayCustomEffect, LampArrayUpdateRequestedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).UpdateRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveUpdateRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUpdateRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn CreateInstance<'a, Param0: ::windows_core::IntoParam<'a, super::LampArray>>(lamparray: Param0, lampindexes: &[i32]) -> ::windows_core::Result<LampArrayCustomEffect> {
        Self::ILampArrayCustomEffectFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), lamparray.into_param().abi(), lampindexes.len() as u32, ::core::mem::transmute(lampindexes.as_ptr()), result__.as_mut_ptr()).from_abi::<LampArrayCustomEffect>(result__)
        })
    }
    pub fn ZIndex(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).ZIndex)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetZIndex(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetZIndex)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ILampArrayCustomEffectFactory<R, F: FnOnce(&ILampArrayCustomEffectFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LampArrayCustomEffect, ILampArrayCustomEffectFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LampArrayCustomEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LampArrayCustomEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LampArrayCustomEffect {}
impl ::core::fmt::Debug for LampArrayCustomEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampArrayCustomEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LampArrayCustomEffect {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.Effects.LampArrayCustomEffect;{ec579170-3c34-4876-818b-5765f78b0ee4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LampArrayCustomEffect {
    type Vtable = ILampArrayCustomEffect_Vtbl;
    const IID: ::windows_core::GUID = <ILampArrayCustomEffect as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LampArrayCustomEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArrayCustomEffect";
}
impl ::core::convert::From<LampArrayCustomEffect> for ::windows_core::IUnknown {
    fn from(value: LampArrayCustomEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampArrayCustomEffect> for ::windows_core::IUnknown {
    fn from(value: &LampArrayCustomEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LampArrayCustomEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LampArrayCustomEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LampArrayCustomEffect> for ::windows_core::IInspectable {
    fn from(value: LampArrayCustomEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampArrayCustomEffect> for ::windows_core::IInspectable {
    fn from(value: &LampArrayCustomEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LampArrayCustomEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LampArrayCustomEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LampArrayCustomEffect> for ILampArrayEffect {
    type Error = ::windows_core::Error;
    fn try_from(value: LampArrayCustomEffect) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LampArrayCustomEffect> for ILampArrayEffect {
    type Error = ::windows_core::Error;
    fn try_from(value: &LampArrayCustomEffect) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ILampArrayEffect> for LampArrayCustomEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ILampArrayEffect> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ILampArrayEffect> for &LampArrayCustomEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ILampArrayEffect> {
        ::core::convert::TryInto::<ILampArrayEffect>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LampArrayCustomEffect {}
unsafe impl ::core::marker::Sync for LampArrayCustomEffect {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LampArrayEffectCompletionBehavior(pub i32);
impl LampArrayEffectCompletionBehavior {
    pub const ClearState: Self = Self(0i32);
    pub const KeepState: Self = Self(1i32);
}
impl ::core::marker::Copy for LampArrayEffectCompletionBehavior {}
impl ::core::clone::Clone for LampArrayEffectCompletionBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LampArrayEffectCompletionBehavior {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for LampArrayEffectCompletionBehavior {
    type Abi = Self;
}
impl ::core::fmt::Debug for LampArrayEffectCompletionBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampArrayEffectCompletionBehavior").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LampArrayEffectCompletionBehavior {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Lights.Effects.LampArrayEffectCompletionBehavior;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct LampArrayEffectPlaylist(::windows_core::IUnknown);
impl LampArrayEffectPlaylist {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LampArrayEffectPlaylist, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<ILampArrayEffect>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<ILampArrayEffect>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<ILampArrayEffect>>(result__)
        }
    }
    pub fn Append<'a, Param0: ::windows_core::IntoParam<'a, ILampArrayEffect>>(&self, effect: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), effect.into_param().abi()).ok() }
    }
    pub fn OverrideZIndex(&self, zindex: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OverrideZIndex)(::windows_core::Interface::as_raw(this), zindex).ok() }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Pause(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Pause)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn EffectStartMode(&self) -> ::windows_core::Result<LampArrayEffectStartMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<LampArrayEffectStartMode>::zeroed();
            (::windows_core::Interface::vtable(this).EffectStartMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LampArrayEffectStartMode>(result__)
        }
    }
    pub fn SetEffectStartMode(&self, value: LampArrayEffectStartMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEffectStartMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Occurrences(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Occurrences)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetOccurrences(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOccurrences)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RepetitionMode(&self) -> ::windows_core::Result<LampArrayRepetitionMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<LampArrayRepetitionMode>::zeroed();
            (::windows_core::Interface::vtable(this).RepetitionMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LampArrayRepetitionMode>(result__)
        }
    }
    pub fn SetRepetitionMode(&self, value: LampArrayRepetitionMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRepetitionMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn StartAll<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<LampArrayEffectPlaylist>>>(value: Param0) -> ::windows_core::Result<()> {
        Self::ILampArrayEffectPlaylistStatics(|this| unsafe { (::windows_core::Interface::vtable(this).StartAll)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn StopAll<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<LampArrayEffectPlaylist>>>(value: Param0) -> ::windows_core::Result<()> {
        Self::ILampArrayEffectPlaylistStatics(|this| unsafe { (::windows_core::Interface::vtable(this).StopAll)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn PauseAll<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<LampArrayEffectPlaylist>>>(value: Param0) -> ::windows_core::Result<()> {
        Self::ILampArrayEffectPlaylistStatics(|this| unsafe { (::windows_core::Interface::vtable(this).PauseAll)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<ILampArrayEffect> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVectorView<ILampArrayEffect>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<ILampArrayEffect>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVectorView<ILampArrayEffect>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, ILampArrayEffect>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVectorView<ILampArrayEffect>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<ILampArrayEffect>]) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVectorView<ILampArrayEffect>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn ILampArrayEffectPlaylistStatics<R, F: FnOnce(&ILampArrayEffectPlaylistStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LampArrayEffectPlaylist, ILampArrayEffectPlaylistStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LampArrayEffectPlaylist {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LampArrayEffectPlaylist {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LampArrayEffectPlaylist {}
impl ::core::fmt::Debug for LampArrayEffectPlaylist {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampArrayEffectPlaylist").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LampArrayEffectPlaylist {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.Effects.LampArrayEffectPlaylist;{7de58bfe-6f61-4103-98c7-d6632f7b9169})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LampArrayEffectPlaylist {
    type Vtable = ILampArrayEffectPlaylist_Vtbl;
    const IID: ::windows_core::GUID = <ILampArrayEffectPlaylist as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LampArrayEffectPlaylist {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArrayEffectPlaylist";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for LampArrayEffectPlaylist {
    type Item = ILampArrayEffect;
    type IntoIter = ::winrt_foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &LampArrayEffectPlaylist {
    type Item = ILampArrayEffect;
    type IntoIter = ::winrt_foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
impl ::core::convert::From<LampArrayEffectPlaylist> for ::windows_core::IUnknown {
    fn from(value: LampArrayEffectPlaylist) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampArrayEffectPlaylist> for ::windows_core::IUnknown {
    fn from(value: &LampArrayEffectPlaylist) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LampArrayEffectPlaylist {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LampArrayEffectPlaylist {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LampArrayEffectPlaylist> for ::windows_core::IInspectable {
    fn from(value: LampArrayEffectPlaylist) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampArrayEffectPlaylist> for ::windows_core::IInspectable {
    fn from(value: &LampArrayEffectPlaylist) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LampArrayEffectPlaylist {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LampArrayEffectPlaylist {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<LampArrayEffectPlaylist> for ::winrt_foundation::Collections::IIterable<ILampArrayEffect> {
    type Error = ::windows_core::Error;
    fn try_from(value: LampArrayEffectPlaylist) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&LampArrayEffectPlaylist> for ::winrt_foundation::Collections::IIterable<ILampArrayEffect> {
    type Error = ::windows_core::Error;
    fn try_from(value: &LampArrayEffectPlaylist) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<ILampArrayEffect>> for LampArrayEffectPlaylist {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<ILampArrayEffect>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<ILampArrayEffect>> for &LampArrayEffectPlaylist {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<ILampArrayEffect>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<ILampArrayEffect>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<LampArrayEffectPlaylist> for ::winrt_foundation::Collections::IVectorView<ILampArrayEffect> {
    type Error = ::windows_core::Error;
    fn try_from(value: LampArrayEffectPlaylist) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&LampArrayEffectPlaylist> for ::winrt_foundation::Collections::IVectorView<ILampArrayEffect> {
    type Error = ::windows_core::Error;
    fn try_from(value: &LampArrayEffectPlaylist) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVectorView<ILampArrayEffect>> for LampArrayEffectPlaylist {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVectorView<ILampArrayEffect>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVectorView<ILampArrayEffect>> for &LampArrayEffectPlaylist {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVectorView<ILampArrayEffect>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVectorView<ILampArrayEffect>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LampArrayEffectPlaylist {}
unsafe impl ::core::marker::Sync for LampArrayEffectPlaylist {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LampArrayEffectStartMode(pub i32);
impl LampArrayEffectStartMode {
    pub const Sequential: Self = Self(0i32);
    pub const Simultaneous: Self = Self(1i32);
}
impl ::core::marker::Copy for LampArrayEffectStartMode {}
impl ::core::clone::Clone for LampArrayEffectStartMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LampArrayEffectStartMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for LampArrayEffectStartMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for LampArrayEffectStartMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampArrayEffectStartMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LampArrayEffectStartMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Lights.Effects.LampArrayEffectStartMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LampArrayRepetitionMode(pub i32);
impl LampArrayRepetitionMode {
    pub const Occurrences: Self = Self(0i32);
    pub const Forever: Self = Self(1i32);
}
impl ::core::marker::Copy for LampArrayRepetitionMode {}
impl ::core::clone::Clone for LampArrayRepetitionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LampArrayRepetitionMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for LampArrayRepetitionMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for LampArrayRepetitionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampArrayRepetitionMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LampArrayRepetitionMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Lights.Effects.LampArrayRepetitionMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct LampArraySolidEffect(::windows_core::IUnknown);
impl LampArraySolidEffect {
    pub fn ZIndex(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).ZIndex)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetZIndex(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetZIndex)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Color(&self) -> ::windows_core::Result<::winrt_ui::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_ui::Color>::zeroed();
            (::windows_core::Interface::vtable(this).Color)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_ui::Color>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn SetColor<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_ui::Color>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetColor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Duration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetDuration<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartDelay(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).StartDelay)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetStartDelay<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStartDelay)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CompletionBehavior(&self) -> ::windows_core::Result<LampArrayEffectCompletionBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<LampArrayEffectCompletionBehavior>::zeroed();
            (::windows_core::Interface::vtable(this).CompletionBehavior)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LampArrayEffectCompletionBehavior>(result__)
        }
    }
    pub fn SetCompletionBehavior(&self, value: LampArrayEffectCompletionBehavior) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCompletionBehavior)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateInstance<'a, Param0: ::windows_core::IntoParam<'a, super::LampArray>>(lamparray: Param0, lampindexes: &[i32]) -> ::windows_core::Result<LampArraySolidEffect> {
        Self::ILampArraySolidEffectFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), lamparray.into_param().abi(), lampindexes.len() as u32, ::core::mem::transmute(lampindexes.as_ptr()), result__.as_mut_ptr()).from_abi::<LampArraySolidEffect>(result__)
        })
    }
    pub fn ILampArraySolidEffectFactory<R, F: FnOnce(&ILampArraySolidEffectFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LampArraySolidEffect, ILampArraySolidEffectFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LampArraySolidEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LampArraySolidEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LampArraySolidEffect {}
impl ::core::fmt::Debug for LampArraySolidEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampArraySolidEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LampArraySolidEffect {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.Effects.LampArraySolidEffect;{441f8213-43cc-4b33-80eb-c6ddde7dc8ed})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LampArraySolidEffect {
    type Vtable = ILampArraySolidEffect_Vtbl;
    const IID: ::windows_core::GUID = <ILampArraySolidEffect as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LampArraySolidEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArraySolidEffect";
}
impl ::core::convert::From<LampArraySolidEffect> for ::windows_core::IUnknown {
    fn from(value: LampArraySolidEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampArraySolidEffect> for ::windows_core::IUnknown {
    fn from(value: &LampArraySolidEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LampArraySolidEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LampArraySolidEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LampArraySolidEffect> for ::windows_core::IInspectable {
    fn from(value: LampArraySolidEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampArraySolidEffect> for ::windows_core::IInspectable {
    fn from(value: &LampArraySolidEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LampArraySolidEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LampArraySolidEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LampArraySolidEffect> for ILampArrayEffect {
    type Error = ::windows_core::Error;
    fn try_from(value: LampArraySolidEffect) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LampArraySolidEffect> for ILampArrayEffect {
    type Error = ::windows_core::Error;
    fn try_from(value: &LampArraySolidEffect) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ILampArrayEffect> for LampArraySolidEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ILampArrayEffect> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ILampArrayEffect> for &LampArraySolidEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ILampArrayEffect> {
        ::core::convert::TryInto::<ILampArrayEffect>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LampArraySolidEffect {}
unsafe impl ::core::marker::Sync for LampArraySolidEffect {}
#[repr(transparent)]
pub struct LampArrayUpdateRequestedEventArgs(::windows_core::IUnknown);
impl LampArrayUpdateRequestedEventArgs {
    pub fn SinceStarted(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).SinceStarted)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn SetColor<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_ui::Color>>(&self, desiredcolor: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetColor)(::windows_core::Interface::as_raw(this), desiredcolor.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn SetColorForIndex<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_ui::Color>>(&self, lampindex: i32, desiredcolor: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetColorForIndex)(::windows_core::Interface::as_raw(this), lampindex, desiredcolor.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn SetSingleColorForIndices<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_ui::Color>>(&self, desiredcolor: Param0, lampindexes: &[i32]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSingleColorForIndices)(::windows_core::Interface::as_raw(this), desiredcolor.into_param().abi(), lampindexes.len() as u32, ::core::mem::transmute(lampindexes.as_ptr())).ok() }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn SetColorsForIndices(&self, desiredcolors: &[::winrt_ui::Color], lampindexes: &[i32]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetColorsForIndices)(::windows_core::Interface::as_raw(this), desiredcolors.len() as u32, ::core::mem::transmute(desiredcolors.as_ptr()), lampindexes.len() as u32, ::core::mem::transmute(lampindexes.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for LampArrayUpdateRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LampArrayUpdateRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LampArrayUpdateRequestedEventArgs {}
impl ::core::fmt::Debug for LampArrayUpdateRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampArrayUpdateRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LampArrayUpdateRequestedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.Effects.LampArrayUpdateRequestedEventArgs;{73560d6a-576a-48af-8539-67ffa0ab3516})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LampArrayUpdateRequestedEventArgs {
    type Vtable = ILampArrayUpdateRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ILampArrayUpdateRequestedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LampArrayUpdateRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArrayUpdateRequestedEventArgs";
}
impl ::core::convert::From<LampArrayUpdateRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: LampArrayUpdateRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampArrayUpdateRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &LampArrayUpdateRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LampArrayUpdateRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LampArrayUpdateRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LampArrayUpdateRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: LampArrayUpdateRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampArrayUpdateRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &LampArrayUpdateRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LampArrayUpdateRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LampArrayUpdateRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LampArrayUpdateRequestedEventArgs {}
unsafe impl ::core::marker::Sync for LampArrayUpdateRequestedEventArgs {}
