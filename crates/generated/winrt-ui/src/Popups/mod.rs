#[doc(hidden)]
#[repr(transparent)]
pub struct IMessageDialog(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMessageDialog {
    type Vtable = IMessageDialog_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x33f59b01_5325_43ab_9ab3_bdae440e4121);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageDialog_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Commands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Commands: usize,
    pub DefaultCommandIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetDefaultCommandIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub CancelCommandIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetCancelCommandIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ShowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Options: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MessageDialogOptions) -> ::windows_core::HRESULT,
    pub SetOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MessageDialogOptions) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMessageDialogFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMessageDialogFactory {
    type Vtable = IMessageDialogFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2d161777_a66f_4ea5_bb87_793ffa4941f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageDialogFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateWithTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, title: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPopupMenu(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPopupMenu {
    type Vtable = IPopupMenu_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4e9bc6dc_880d_47fc_a0a1_72b639e62559);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopupMenu_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Commands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Commands: usize,
    pub ShowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, invocationpoint: ::winrt_foundation::Point, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ShowAsyncWithRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: ::winrt_foundation::Rect, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ShowAsyncWithRectAndPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: ::winrt_foundation::Rect, preferredplacement: Placement, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IUICommand(::windows_core::IUnknown);
impl IUICommand {
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetLabel<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLabel)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Invoked(&self) -> ::windows_core::Result<UICommandInvokedHandler> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Invoked)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UICommandInvokedHandler>(result__)
        }
    }
    pub fn SetInvoked<'a, Param0: ::windows_core::IntoParam<'a, UICommandInvokedHandler>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInvoked)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn SetId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<IUICommand> for ::windows_core::IUnknown {
    fn from(value: IUICommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUICommand> for ::windows_core::IUnknown {
    fn from(value: &IUICommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUICommand {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUICommand {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IUICommand> for ::windows_core::IInspectable {
    fn from(value: IUICommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUICommand> for ::windows_core::IInspectable {
    fn from(value: &IUICommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IUICommand {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IUICommand {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUICommand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUICommand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUICommand {}
impl ::core::fmt::Debug for IUICommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUICommand").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IUICommand {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{4ff93a75-4145-47ff-ac7f-dff1c1fa5b0f}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IUICommand {
    type Vtable = IUICommand_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4ff93a75_4145_47ff_ac7f_dff1c1fa5b0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUICommand_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Invoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetInvoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUICommandFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUICommandFactory {
    type Vtable = IUICommandFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa21a8189_26b0_4676_ae94_54041bc125e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUICommandFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, label: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateWithHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, label: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, action: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateWithHandlerAndId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, label: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, action: ::windows_core::RawPtr, commandid: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct MessageDialog(::windows_core::IUnknown);
impl MessageDialog {
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetTitle<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTitle)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Commands(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<IUICommand>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Commands)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<IUICommand>>(result__)
        }
    }
    pub fn DefaultCommandIndex(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).DefaultCommandIndex)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetDefaultCommandIndex(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDefaultCommandIndex)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CancelCommandIndex(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).CancelCommandIndex)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetCancelCommandIndex(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCancelCommandIndex)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Content(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Content)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetContent<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContent)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ShowAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<IUICommand>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<IUICommand>>(result__)
        }
    }
    pub fn Options(&self) -> ::windows_core::Result<MessageDialogOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MessageDialogOptions>::zeroed();
            (::windows_core::Interface::vtable(this).Options)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MessageDialogOptions>(result__)
        }
    }
    pub fn SetOptions(&self, value: MessageDialogOptions) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOptions)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(content: Param0) -> ::windows_core::Result<MessageDialog> {
        Self::IMessageDialogFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), content.into_param().abi(), result__.as_mut_ptr()).from_abi::<MessageDialog>(result__)
        })
    }
    pub fn CreateWithTitle<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(content: Param0, title: Param1) -> ::windows_core::Result<MessageDialog> {
        Self::IMessageDialogFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithTitle)(::windows_core::Interface::as_raw(this), content.into_param().abi(), title.into_param().abi(), result__.as_mut_ptr()).from_abi::<MessageDialog>(result__)
        })
    }
    pub fn IMessageDialogFactory<R, F: FnOnce(&IMessageDialogFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MessageDialog, IMessageDialogFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MessageDialog {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MessageDialog {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MessageDialog {}
impl ::core::fmt::Debug for MessageDialog {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MessageDialog").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MessageDialog {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Popups.MessageDialog;{33f59b01-5325-43ab-9ab3-bdae440e4121})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MessageDialog {
    type Vtable = IMessageDialog_Vtbl;
    const IID: ::windows_core::GUID = <IMessageDialog as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MessageDialog {
    const NAME: &'static str = "Windows.UI.Popups.MessageDialog";
}
impl ::core::convert::From<MessageDialog> for ::windows_core::IUnknown {
    fn from(value: MessageDialog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MessageDialog> for ::windows_core::IUnknown {
    fn from(value: &MessageDialog) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MessageDialog {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MessageDialog {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MessageDialog> for ::windows_core::IInspectable {
    fn from(value: MessageDialog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MessageDialog> for ::windows_core::IInspectable {
    fn from(value: &MessageDialog) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MessageDialog {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MessageDialog {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for MessageDialogOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MessageDialogOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for MessageDialogOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MessageDialogOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MessageDialogOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MessageDialogOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MessageDialogOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MessageDialogOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MessageDialogOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for MessageDialogOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Popups.MessageDialogOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for Placement {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for Placement {
    type Abi = Self;
}
impl ::core::fmt::Debug for Placement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Placement").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Placement {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Popups.Placement;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PopupMenu(::windows_core::IUnknown);
impl PopupMenu {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PopupMenu, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Commands(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<IUICommand>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Commands)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<IUICommand>>(result__)
        }
    }
    pub fn ShowAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>>(&self, invocationpoint: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<IUICommand>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowAsync)(::windows_core::Interface::as_raw(this), invocationpoint.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<IUICommand>>(result__)
        }
    }
    pub fn ShowAsyncWithRect<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(&self, selection: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<IUICommand>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowAsyncWithRect)(::windows_core::Interface::as_raw(this), selection.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<IUICommand>>(result__)
        }
    }
    pub fn ShowAsyncWithRectAndPlacement<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(&self, selection: Param0, preferredplacement: Placement) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<IUICommand>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowAsyncWithRectAndPlacement)(::windows_core::Interface::as_raw(this), selection.into_param().abi(), preferredplacement, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<IUICommand>>(result__)
        }
    }
}
impl ::core::clone::Clone for PopupMenu {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PopupMenu {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PopupMenu {}
impl ::core::fmt::Debug for PopupMenu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PopupMenu").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PopupMenu {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Popups.PopupMenu;{4e9bc6dc-880d-47fc-a0a1-72b639e62559})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PopupMenu {
    type Vtable = IPopupMenu_Vtbl;
    const IID: ::windows_core::GUID = <IPopupMenu as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PopupMenu {
    const NAME: &'static str = "Windows.UI.Popups.PopupMenu";
}
impl ::core::convert::From<PopupMenu> for ::windows_core::IUnknown {
    fn from(value: PopupMenu) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PopupMenu> for ::windows_core::IUnknown {
    fn from(value: &PopupMenu) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PopupMenu {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PopupMenu {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PopupMenu> for ::windows_core::IInspectable {
    fn from(value: PopupMenu) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PopupMenu> for ::windows_core::IInspectable {
    fn from(value: &PopupMenu) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PopupMenu {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PopupMenu {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct UICommand(::windows_core::IUnknown);
impl UICommand {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<UICommand, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetLabel<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLabel)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Invoked(&self) -> ::windows_core::Result<UICommandInvokedHandler> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Invoked)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UICommandInvokedHandler>(result__)
        }
    }
    pub fn SetInvoked<'a, Param0: ::windows_core::IntoParam<'a, UICommandInvokedHandler>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInvoked)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn SetId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(label: Param0) -> ::windows_core::Result<UICommand> {
        Self::IUICommandFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), label.into_param().abi(), result__.as_mut_ptr()).from_abi::<UICommand>(result__)
        })
    }
    pub fn CreateWithHandler<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, UICommandInvokedHandler>>(label: Param0, action: Param1) -> ::windows_core::Result<UICommand> {
        Self::IUICommandFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithHandler)(::windows_core::Interface::as_raw(this), label.into_param().abi(), action.into_param().abi(), result__.as_mut_ptr()).from_abi::<UICommand>(result__)
        })
    }
    pub fn CreateWithHandlerAndId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, UICommandInvokedHandler>, Param2: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(label: Param0, action: Param1, commandid: Param2) -> ::windows_core::Result<UICommand> {
        Self::IUICommandFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithHandlerAndId)(::windows_core::Interface::as_raw(this), label.into_param().abi(), action.into_param().abi(), commandid.into_param().abi(), result__.as_mut_ptr()).from_abi::<UICommand>(result__)
        })
    }
    pub fn IUICommandFactory<R, F: FnOnce(&IUICommandFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<UICommand, IUICommandFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for UICommand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UICommand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UICommand {}
impl ::core::fmt::Debug for UICommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UICommand").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UICommand {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Popups.UICommand;{4ff93a75-4145-47ff-ac7f-dff1c1fa5b0f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UICommand {
    type Vtable = IUICommand_Vtbl;
    const IID: ::windows_core::GUID = <IUICommand as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UICommand {
    const NAME: &'static str = "Windows.UI.Popups.UICommand";
}
impl ::core::convert::From<UICommand> for ::windows_core::IUnknown {
    fn from(value: UICommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UICommand> for ::windows_core::IUnknown {
    fn from(value: &UICommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UICommand {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UICommand {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UICommand> for ::windows_core::IInspectable {
    fn from(value: UICommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UICommand> for ::windows_core::IInspectable {
    fn from(value: &UICommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UICommand {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UICommand {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<UICommand> for IUICommand {
    type Error = ::windows_core::Error;
    fn try_from(value: UICommand) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&UICommand> for IUICommand {
    type Error = ::windows_core::Error;
    fn try_from(value: &UICommand) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IUICommand> for UICommand {
    fn into_param(self) -> ::windows_core::Param<'a, IUICommand> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IUICommand> for &UICommand {
    fn into_param(self) -> ::windows_core::Param<'a, IUICommand> {
        ::core::convert::TryInto::<IUICommand>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for UICommand {}
unsafe impl ::core::marker::Sync for UICommand {}
#[repr(transparent)]
pub struct UICommandInvokedHandler(pub ::windows_core::IUnknown);
impl UICommandInvokedHandler {
    pub fn new<F: FnMut(&::core::option::Option<IUICommand>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = UICommandInvokedHandlerBox::<F> { vtable: &UICommandInvokedHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, IUICommand>>(&self, command: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), command.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct UICommandInvokedHandlerBox<F: FnMut(&::core::option::Option<IUICommand>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const UICommandInvokedHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
impl<F: FnMut(&::core::option::Option<IUICommand>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> UICommandInvokedHandlerBox<F> {
    const VTABLE: UICommandInvokedHandler_Vtbl = UICommandInvokedHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<UICommandInvokedHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows_core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, command: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&command)).into()
    }
}
impl ::core::clone::Clone for UICommandInvokedHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UICommandInvokedHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UICommandInvokedHandler {}
impl ::core::fmt::Debug for UICommandInvokedHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UICommandInvokedHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for UICommandInvokedHandler {
    type Vtable = UICommandInvokedHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdaf77a4f_c27a_4298_9ac6_2922c45e7da6);
}
unsafe impl ::windows_core::RuntimeType for UICommandInvokedHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{daf77a4f-c27a-4298-9ac6-2922c45e7da6}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct UICommandInvokedHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, command: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct UICommandSeparator(::windows_core::IUnknown);
impl UICommandSeparator {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<UICommandSeparator, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetLabel<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLabel)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Invoked(&self) -> ::windows_core::Result<UICommandInvokedHandler> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Invoked)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UICommandInvokedHandler>(result__)
        }
    }
    pub fn SetInvoked<'a, Param0: ::windows_core::IntoParam<'a, UICommandInvokedHandler>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInvoked)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn SetId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for UICommandSeparator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UICommandSeparator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UICommandSeparator {}
impl ::core::fmt::Debug for UICommandSeparator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UICommandSeparator").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UICommandSeparator {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Popups.UICommandSeparator;{4ff93a75-4145-47ff-ac7f-dff1c1fa5b0f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UICommandSeparator {
    type Vtable = IUICommand_Vtbl;
    const IID: ::windows_core::GUID = <IUICommand as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UICommandSeparator {
    const NAME: &'static str = "Windows.UI.Popups.UICommandSeparator";
}
impl ::core::convert::From<UICommandSeparator> for ::windows_core::IUnknown {
    fn from(value: UICommandSeparator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UICommandSeparator> for ::windows_core::IUnknown {
    fn from(value: &UICommandSeparator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UICommandSeparator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UICommandSeparator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UICommandSeparator> for ::windows_core::IInspectable {
    fn from(value: UICommandSeparator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UICommandSeparator> for ::windows_core::IInspectable {
    fn from(value: &UICommandSeparator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UICommandSeparator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UICommandSeparator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<UICommandSeparator> for IUICommand {
    type Error = ::windows_core::Error;
    fn try_from(value: UICommandSeparator) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&UICommandSeparator> for IUICommand {
    type Error = ::windows_core::Error;
    fn try_from(value: &UICommandSeparator) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IUICommand> for UICommandSeparator {
    fn into_param(self) -> ::windows_core::Param<'a, IUICommand> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IUICommand> for &UICommandSeparator {
    fn into_param(self) -> ::windows_core::Param<'a, IUICommand> {
        ::core::convert::TryInto::<IUICommand>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for UICommandSeparator {}
unsafe impl ::core::marker::Sync for UICommandSeparator {}
