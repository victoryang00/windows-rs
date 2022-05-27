#[doc = "*Required features: `\"Data_Html\"`*"]
pub struct HtmlUtilities;
impl HtmlUtilities {
    #[doc = "*Required features: `\"Data_Html\"`*"]
    pub fn ConvertToText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(html: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IHtmlUtilities(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ConvertToText)(::windows_core::Interface::as_raw(this), html.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IHtmlUtilities<R, F: FnOnce(&IHtmlUtilities) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HtmlUtilities, IHtmlUtilities> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for HtmlUtilities {
    const NAME: &'static str = "Windows.Data.Html.HtmlUtilities";
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHtmlUtilities(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHtmlUtilities {
    type Vtable = IHtmlUtilities_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfec00add_2399_4fac_b5a7_05e9acd7181d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHtmlUtilities_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ConvertToText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, html: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
