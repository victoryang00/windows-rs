#[doc = "*Required features: `\"Perception_Automation_Core\"`*"]
pub struct CorePerceptionAutomation;
impl CorePerceptionAutomation {
    #[doc = "*Required features: `\"Perception_Automation_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetActivationFactoryProvider<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::IGetActivationFactory>>(provider: Param0) -> ::windows_core::Result<()> {
        Self::ICorePerceptionAutomationStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetActivationFactoryProvider)(::windows_core::Interface::as_raw(this), provider.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn ICorePerceptionAutomationStatics<R, F: FnOnce(&ICorePerceptionAutomationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CorePerceptionAutomation, ICorePerceptionAutomationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for CorePerceptionAutomation {
    const NAME: &'static str = "Windows.Perception.Automation.Core.CorePerceptionAutomation";
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICorePerceptionAutomationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICorePerceptionAutomationStatics {
    type Vtable = ICorePerceptionAutomationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0bb04541_4ce2_4923_9a76_8187ecc59112);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorePerceptionAutomationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub SetActivationFactoryProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetActivationFactoryProvider: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
