pub type FrameNavigationOptions = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IFrameNavigationOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsNavigationStackEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsNavigationStackEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    pub TransitionInfoOverride: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Animation"))]
    TransitionInfoOverride: usize,
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    pub SetTransitionInfoOverride: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Animation"))]
    SetTransitionInfoOverride: usize,
}
#[repr(C)]
pub struct IFrameNavigationOptionsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INavigatingCancelEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub NavigationMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut NavigationMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Interop")]
    pub SourcePageType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))]
    SourcePageType: usize,
}
#[repr(C)]
pub struct INavigatingCancelEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parameter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    pub NavigationTransitionInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Animation"))]
    NavigationTransitionInfo: usize,
}
#[repr(C)]
pub struct INavigationEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Content: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Parameter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Interop")]
    pub SourcePageType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))]
    SourcePageType: usize,
    pub NavigationMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut NavigationMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub SetUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUri: usize,
}
#[repr(C)]
pub struct INavigationEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    pub NavigationTransitionInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Animation"))]
    NavigationTransitionInfo: usize,
}
#[repr(C)]
pub struct INavigationFailedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Exception: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Interop")]
    pub SourcePageType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))]
    SourcePageType: usize,
}
#[repr(C)]
pub struct IPageStackEntry {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Interop")]
    pub SourcePageType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))]
    SourcePageType: usize,
    pub Parameter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    pub NavigationTransitionInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Animation"))]
    NavigationTransitionInfo: usize,
}
#[repr(C)]
pub struct IPageStackEntryFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "UI_Xaml_Interop", feature = "UI_Xaml_Media_Animation"))]
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, sourcepagetype: ::core::mem::ManuallyDrop<super::Interop::TypeName>, parameter: *mut ::core::ffi::c_void, navigationtransitioninfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "UI_Xaml_Interop", feature = "UI_Xaml_Media_Animation")))]
    CreateInstance: usize,
}
#[repr(C)]
pub struct IPageStackEntryStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub SourcePageTypeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
pub type LoadCompletedEventHandler = *mut ::core::ffi::c_void;
pub type NavigatedEventHandler = *mut ::core::ffi::c_void;
pub type NavigatingCancelEventArgs = *mut ::core::ffi::c_void;
pub type NavigatingCancelEventHandler = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Navigation\"`*"]
#[repr(transparent)]
pub struct NavigationCacheMode(pub i32);
impl NavigationCacheMode {
    pub const Disabled: Self = Self(0i32);
    pub const Required: Self = Self(1i32);
    pub const Enabled: Self = Self(2i32);
}
impl ::core::marker::Copy for NavigationCacheMode {}
impl ::core::clone::Clone for NavigationCacheMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type NavigationEventArgs = *mut ::core::ffi::c_void;
pub type NavigationFailedEventArgs = *mut ::core::ffi::c_void;
pub type NavigationFailedEventHandler = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Navigation\"`*"]
#[repr(transparent)]
pub struct NavigationMode(pub i32);
impl NavigationMode {
    pub const New: Self = Self(0i32);
    pub const Back: Self = Self(1i32);
    pub const Forward: Self = Self(2i32);
    pub const Refresh: Self = Self(3i32);
}
impl ::core::marker::Copy for NavigationMode {}
impl ::core::clone::Clone for NavigationMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type NavigationStoppedEventHandler = *mut ::core::ffi::c_void;
pub type PageStackEntry = *mut ::core::ffi::c_void;
