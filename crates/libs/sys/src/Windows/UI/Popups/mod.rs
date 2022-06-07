#[repr(C)]
pub struct IMessageDialog {
    pub base__: ::windows_sys::core::IInspectable,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Commands: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Commands: usize,
    pub DefaultCommandIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetDefaultCommandIndex: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub CancelCommandIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetCancelCommandIndex: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub Content: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetContent: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ShowAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAsync: usize,
    pub Options: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MessageDialogOptions) -> ::windows_sys::core::HRESULT,
    pub SetOptions: unsafe extern "system" fn(this: *mut *mut Self, value: MessageDialogOptions) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMessageDialog {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 871734017, data2: 21285, data3: 17323, data4: [154, 179, 189, 174, 68, 14, 65, 33] };
}
#[repr(C)]
pub struct IMessageDialogFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, content: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithTitle: unsafe extern "system" fn(this: *mut *mut Self, content: ::windows_sys::core::HSTRING, title: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMessageDialogFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 756422519, data2: 42607, data3: 20133, data4: [187, 135, 121, 63, 250, 73, 65, 242] };
}
#[repr(C)]
pub struct IPopupMenu {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Commands: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Commands: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAsync: unsafe extern "system" fn(this: *mut *mut Self, invocationpoint: super::super::Foundation::Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAsyncWithRect: unsafe extern "system" fn(this: *mut *mut Self, selection: super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAsyncWithRect: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAsyncWithRectAndPlacement: unsafe extern "system" fn(this: *mut *mut Self, selection: super::super::Foundation::Rect, preferredplacement: Placement, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAsyncWithRectAndPlacement: usize,
}
impl ::windows_sys::core::Interface for IPopupMenu {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1318831836, data2: 34829, data3: 18428, data4: [160, 161, 114, 182, 57, 230, 37, 89] };
}
#[repr(C)]
pub struct IUICommand {
    pub base__: ::windows_sys::core::IInspectable,
    pub Label: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Invoked: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetInvoked: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUICommand {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1341733493, data2: 16709, data3: 18431, data4: [172, 127, 223, 241, 193, 250, 91, 15] };
}
#[repr(C)]
pub struct IUICommandFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, label: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithHandler: unsafe extern "system" fn(this: *mut *mut Self, label: ::windows_sys::core::HSTRING, action: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithHandlerAndId: unsafe extern "system" fn(this: *mut *mut Self, label: ::windows_sys::core::HSTRING, action: *mut ::core::ffi::c_void, commandid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUICommandFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2719646089, data2: 9904, data3: 18038, data4: [174, 148, 84, 4, 27, 193, 37, 232] };
}
pub type MessageDialog = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Popups\"`*"]
#[repr(transparent)]
pub struct MessageDialogOptions(pub u32);
impl MessageDialogOptions {
    pub const None: Self = Self(0u32);
    pub const AcceptUserInputAfterDelay: Self = Self(1u32);
}
impl ::core::marker::Copy for MessageDialogOptions {}
impl ::core::clone::Clone for MessageDialogOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Popups\"`*"]
#[repr(transparent)]
pub struct Placement(pub i32);
impl Placement {
    pub const Default: Self = Self(0i32);
    pub const Above: Self = Self(1i32);
    pub const Below: Self = Self(2i32);
    pub const Left: Self = Self(3i32);
    pub const Right: Self = Self(4i32);
}
impl ::core::marker::Copy for Placement {}
impl ::core::clone::Clone for Placement {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PopupMenu = *mut ::core::ffi::c_void;
pub type UICommand = *mut ::core::ffi::c_void;
pub type UICommandInvokedHandler = *mut ::core::ffi::c_void;
pub type UICommandSeparator = *mut ::core::ffi::c_void;
