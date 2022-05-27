#[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
#[repr(transparent)]
pub struct CoreDragDropManager(::windows_core::IUnknown);
impl CoreDragDropManager {
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TargetRequested<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CoreDragDropManager, CoreDropOperationTargetRequestedEventArgs>>>(&self, value: Param0) -> ::windows_core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).TargetRequested)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTargetRequested<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveTargetRequested)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
    pub fn AreConcurrentOperationsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AreConcurrentOperationsEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
    pub fn SetAreConcurrentOperationsEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAreConcurrentOperationsEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
    pub fn GetForCurrentView() -> ::windows_core::Result<CoreDragDropManager> {
        Self::ICoreDragDropManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CoreDragDropManager>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICoreDragDropManagerStatics<R, F: FnOnce(&ICoreDragDropManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CoreDragDropManager, ICoreDragDropManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CoreDragDropManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreDragDropManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreDragDropManager {}
impl ::core::fmt::Debug for CoreDragDropManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreDragDropManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CoreDragDropManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragDropManager;{7d56d344-8464-4faf-aa49-37ea6e2d7bd1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CoreDragDropManager {
    type Vtable = ICoreDragDropManager_Vtbl;
    const IID: ::windows_core::GUID = <ICoreDragDropManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CoreDragDropManager {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragDropManager";
}
impl ::core::convert::From<CoreDragDropManager> for ::windows_core::IUnknown {
    fn from(value: CoreDragDropManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreDragDropManager> for ::windows_core::IUnknown {
    fn from(value: &CoreDragDropManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CoreDragDropManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CoreDragDropManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreDragDropManager> for ::windows_core::IInspectable {
    fn from(value: CoreDragDropManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreDragDropManager> for ::windows_core::IInspectable {
    fn from(value: &CoreDragDropManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CoreDragDropManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CoreDragDropManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreDragDropManager {}
unsafe impl ::core::marker::Sync for CoreDragDropManager {}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
#[repr(transparent)]
pub struct CoreDragInfo(::windows_core::IUnknown);
impl CoreDragInfo {
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
    pub fn Data(&self) -> ::windows_core::Result<super::super::DataPackageView> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DataPackageView>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
    pub fn Modifiers(&self) -> ::windows_core::Result<super::DragDropModifiers> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::DragDropModifiers>::zeroed();
            (::windows_core::Interface::vtable(this).Modifiers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DragDropModifiers>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows_core::Result<super::super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::super::Foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
    pub fn AllowedOperations(&self) -> ::windows_core::Result<super::super::DataPackageOperation> {
        let this = &::windows_core::Interface::cast::<ICoreDragInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::DataPackageOperation>::zeroed();
            (::windows_core::Interface::vtable(this).AllowedOperations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DataPackageOperation>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreDragInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreDragInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreDragInfo {}
impl ::core::fmt::Debug for CoreDragInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreDragInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CoreDragInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragInfo;{48353a8b-cb50-464e-9575-cd4e3a7ab028})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CoreDragInfo {
    type Vtable = ICoreDragInfo_Vtbl;
    const IID: ::windows_core::GUID = <ICoreDragInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CoreDragInfo {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragInfo";
}
impl ::core::convert::From<CoreDragInfo> for ::windows_core::IUnknown {
    fn from(value: CoreDragInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreDragInfo> for ::windows_core::IUnknown {
    fn from(value: &CoreDragInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CoreDragInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CoreDragInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreDragInfo> for ::windows_core::IInspectable {
    fn from(value: CoreDragInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreDragInfo> for ::windows_core::IInspectable {
    fn from(value: &CoreDragInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CoreDragInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CoreDragInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreDragInfo {}
unsafe impl ::core::marker::Sync for CoreDragInfo {}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
#[repr(transparent)]
pub struct CoreDragOperation(::windows_core::IUnknown);
impl CoreDragOperation {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CoreDragOperation, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
    pub fn Data(&self) -> ::windows_core::Result<super::super::DataPackage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DataPackage>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
    pub fn SetPointerId(&self, pointerid: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPointerId)(::windows_core::Interface::as_raw(this), pointerid).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetDragUIContentFromSoftwareBitmap<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::super::Graphics::Imaging::SoftwareBitmap>>(&self, softwarebitmap: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDragUIContentFromSoftwareBitmap)(::windows_core::Interface::as_raw(this), softwarebitmap.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`, `\"Foundation\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub fn SetDragUIContentFromSoftwareBitmapWithAnchorPoint<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::super::Graphics::Imaging::SoftwareBitmap>, Param1: ::windows_core::IntoParam<'a, super::super::super::super::Foundation::Point>>(&self, softwarebitmap: Param0, anchorpoint: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDragUIContentFromSoftwareBitmapWithAnchorPoint)(::windows_core::Interface::as_raw(this), softwarebitmap.into_param().abi(), anchorpoint.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
    pub fn DragUIContentMode(&self) -> ::windows_core::Result<CoreDragUIContentMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<CoreDragUIContentMode>::zeroed();
            (::windows_core::Interface::vtable(this).DragUIContentMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CoreDragUIContentMode>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
    pub fn SetDragUIContentMode(&self, value: CoreDragUIContentMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDragUIContentMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartAsync(&self) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
    pub fn AllowedOperations(&self) -> ::windows_core::Result<super::super::DataPackageOperation> {
        let this = &::windows_core::Interface::cast::<ICoreDragOperation2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::DataPackageOperation>::zeroed();
            (::windows_core::Interface::vtable(this).AllowedOperations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DataPackageOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
    pub fn SetAllowedOperations(&self, value: super::super::DataPackageOperation) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ICoreDragOperation2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowedOperations)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for CoreDragOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreDragOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreDragOperation {}
impl ::core::fmt::Debug for CoreDragOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreDragOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CoreDragOperation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragOperation;{cc06de4f-6db0-4e62-ab1b-a74a02dc6d85})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CoreDragOperation {
    type Vtable = ICoreDragOperation_Vtbl;
    const IID: ::windows_core::GUID = <ICoreDragOperation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CoreDragOperation {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragOperation";
}
impl ::core::convert::From<CoreDragOperation> for ::windows_core::IUnknown {
    fn from(value: CoreDragOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreDragOperation> for ::windows_core::IUnknown {
    fn from(value: &CoreDragOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CoreDragOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CoreDragOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreDragOperation> for ::windows_core::IInspectable {
    fn from(value: CoreDragOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreDragOperation> for ::windows_core::IInspectable {
    fn from(value: &CoreDragOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CoreDragOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CoreDragOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreDragOperation {}
unsafe impl ::core::marker::Sync for CoreDragOperation {}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CoreDragUIContentMode(pub u32);
impl CoreDragUIContentMode {
    pub const Auto: Self = Self(0u32);
    pub const Deferred: Self = Self(1u32);
}
impl ::core::marker::Copy for CoreDragUIContentMode {}
impl ::core::clone::Clone for CoreDragUIContentMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreDragUIContentMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CoreDragUIContentMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreDragUIContentMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreDragUIContentMode").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CoreDragUIContentMode {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CoreDragUIContentMode {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CoreDragUIContentMode {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CoreDragUIContentMode {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CoreDragUIContentMode {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for CoreDragUIContentMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragUIContentMode;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
#[repr(transparent)]
pub struct CoreDragUIOverride(::windows_core::IUnknown);
impl CoreDragUIOverride {
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetContentFromSoftwareBitmap<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::super::Graphics::Imaging::SoftwareBitmap>>(&self, softwarebitmap: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContentFromSoftwareBitmap)(::windows_core::Interface::as_raw(this), softwarebitmap.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`, `\"Foundation\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub fn SetContentFromSoftwareBitmapWithAnchorPoint<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::super::Graphics::Imaging::SoftwareBitmap>, Param1: ::windows_core::IntoParam<'a, super::super::super::super::Foundation::Point>>(&self, softwarebitmap: Param0, anchorpoint: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContentFromSoftwareBitmapWithAnchorPoint)(::windows_core::Interface::as_raw(this), softwarebitmap.into_param().abi(), anchorpoint.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
    pub fn IsContentVisible(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsContentVisible)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
    pub fn SetIsContentVisible(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsContentVisible)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
    pub fn Caption(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Caption)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
    pub fn SetCaption<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCaption)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
    pub fn IsCaptionVisible(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCaptionVisible)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
    pub fn SetIsCaptionVisible(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsCaptionVisible)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
    pub fn IsGlyphVisible(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsGlyphVisible)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
    pub fn SetIsGlyphVisible(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsGlyphVisible)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for CoreDragUIOverride {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreDragUIOverride {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreDragUIOverride {}
impl ::core::fmt::Debug for CoreDragUIOverride {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreDragUIOverride").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CoreDragUIOverride {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragUIOverride;{89a85064-3389-4f4f-8897-7e8a3ffb3c93})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CoreDragUIOverride {
    type Vtable = ICoreDragUIOverride_Vtbl;
    const IID: ::windows_core::GUID = <ICoreDragUIOverride as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CoreDragUIOverride {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragUIOverride";
}
impl ::core::convert::From<CoreDragUIOverride> for ::windows_core::IUnknown {
    fn from(value: CoreDragUIOverride) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreDragUIOverride> for ::windows_core::IUnknown {
    fn from(value: &CoreDragUIOverride) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CoreDragUIOverride {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CoreDragUIOverride {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreDragUIOverride> for ::windows_core::IInspectable {
    fn from(value: CoreDragUIOverride) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreDragUIOverride> for ::windows_core::IInspectable {
    fn from(value: &CoreDragUIOverride) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CoreDragUIOverride {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CoreDragUIOverride {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreDragUIOverride {}
unsafe impl ::core::marker::Sync for CoreDragUIOverride {}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
#[repr(transparent)]
pub struct CoreDropOperationTargetRequestedEventArgs(::windows_core::IUnknown);
impl CoreDropOperationTargetRequestedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
    pub fn SetTarget<'a, Param0: ::windows_core::IntoParam<'a, ICoreDropOperationTarget>>(&self, target: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTarget)(::windows_core::Interface::as_raw(this), target.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for CoreDropOperationTargetRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreDropOperationTargetRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreDropOperationTargetRequestedEventArgs {}
impl ::core::fmt::Debug for CoreDropOperationTargetRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreDropOperationTargetRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CoreDropOperationTargetRequestedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDropOperationTargetRequestedEventArgs;{2aca929a-5e28-4ea6-829e-29134e665d6d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CoreDropOperationTargetRequestedEventArgs {
    type Vtable = ICoreDropOperationTargetRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ICoreDropOperationTargetRequestedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CoreDropOperationTargetRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDropOperationTargetRequestedEventArgs";
}
impl ::core::convert::From<CoreDropOperationTargetRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: CoreDropOperationTargetRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreDropOperationTargetRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &CoreDropOperationTargetRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CoreDropOperationTargetRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CoreDropOperationTargetRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreDropOperationTargetRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: CoreDropOperationTargetRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreDropOperationTargetRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &CoreDropOperationTargetRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CoreDropOperationTargetRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CoreDropOperationTargetRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreDropOperationTargetRequestedEventArgs {}
unsafe impl ::core::marker::Sync for CoreDropOperationTargetRequestedEventArgs {}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreDragDropManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreDragDropManager {
    type Vtable = ICoreDragDropManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d56d344_8464_4faf_aa49_37ea6e2d7bd1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDragDropManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub TargetRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TargetRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTargetRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTargetRequested: usize,
    pub AreConcurrentOperationsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAreConcurrentOperationsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreDragDropManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreDragDropManagerStatics {
    type Vtable = ICoreDragDropManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9542fdca_da12_4c1c_8d06_041db29733c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDragDropManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreDragInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreDragInfo {
    type Vtable = ICoreDragInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x48353a8b_cb50_464e_9575_cd4e3a7ab028);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDragInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Modifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::DragDropModifiers) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreDragInfo2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreDragInfo2 {
    type Vtable = ICoreDragInfo2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc54691e5_e6fb_4d74_b4b1_8a3c17f25e9e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDragInfo2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AllowedOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::DataPackageOperation) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreDragOperation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreDragOperation {
    type Vtable = ICoreDragOperation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcc06de4f_6db0_4e62_ab1b_a74a02dc6d85);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDragOperation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetPointerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetDragUIContentFromSoftwareBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, softwarebitmap: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetDragUIContentFromSoftwareBitmap: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub SetDragUIContentFromSoftwareBitmapWithAnchorPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, softwarebitmap: ::windows_core::RawPtr, anchorpoint: super::super::super::super::Foundation::Point) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))]
    SetDragUIContentFromSoftwareBitmapWithAnchorPoint: usize,
    pub DragUIContentMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreDragUIContentMode) -> ::windows_core::HRESULT,
    pub SetDragUIContentMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CoreDragUIContentMode) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreDragOperation2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreDragOperation2 {
    type Vtable = ICoreDragOperation2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x824b1e2c_d99a_4fc3_8507_6c182f33b46a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDragOperation2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AllowedOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::DataPackageOperation) -> ::windows_core::HRESULT,
    pub SetAllowedOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::DataPackageOperation) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreDragUIOverride(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreDragUIOverride {
    type Vtable = ICoreDragUIOverride_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x89a85064_3389_4f4f_8897_7e8a3ffb3c93);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDragUIOverride_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetContentFromSoftwareBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, softwarebitmap: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetContentFromSoftwareBitmap: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub SetContentFromSoftwareBitmapWithAnchorPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, softwarebitmap: ::windows_core::RawPtr, anchorpoint: super::super::super::super::Foundation::Point) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))]
    SetContentFromSoftwareBitmapWithAnchorPoint: usize,
    pub IsContentVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsContentVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Caption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCaption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsCaptionVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsCaptionVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsGlyphVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsGlyphVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
#[repr(transparent)]
pub struct ICoreDropOperationTarget(::windows_core::IUnknown);
impl ICoreDropOperationTarget {
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnterAsync<'a, Param0: ::windows_core::IntoParam<'a, CoreDragInfo>, Param1: ::windows_core::IntoParam<'a, CoreDragUIOverride>>(&self, draginfo: Param0, draguioverride: Param1) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EnterAsync)(::windows_core::Interface::as_raw(this), draginfo.into_param().abi(), draguioverride.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OverAsync<'a, Param0: ::windows_core::IntoParam<'a, CoreDragInfo>, Param1: ::windows_core::IntoParam<'a, CoreDragUIOverride>>(&self, draginfo: Param0, draguioverride: Param1) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OverAsync)(::windows_core::Interface::as_raw(this), draginfo.into_param().abi(), draguioverride.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LeaveAsync<'a, Param0: ::windows_core::IntoParam<'a, CoreDragInfo>>(&self, draginfo: Param0) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LeaveAsync)(::windows_core::Interface::as_raw(this), draginfo.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DropAsync<'a, Param0: ::windows_core::IntoParam<'a, CoreDragInfo>>(&self, draginfo: Param0) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DropAsync)(::windows_core::Interface::as_raw(this), draginfo.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>>(result__)
        }
    }
}
impl ::core::convert::From<ICoreDropOperationTarget> for ::windows_core::IUnknown {
    fn from(value: ICoreDropOperationTarget) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICoreDropOperationTarget> for ::windows_core::IUnknown {
    fn from(value: &ICoreDropOperationTarget) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICoreDropOperationTarget {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICoreDropOperationTarget {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICoreDropOperationTarget> for ::windows_core::IInspectable {
    fn from(value: ICoreDropOperationTarget) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICoreDropOperationTarget> for ::windows_core::IInspectable {
    fn from(value: &ICoreDropOperationTarget) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ICoreDropOperationTarget {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ICoreDropOperationTarget {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICoreDropOperationTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICoreDropOperationTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoreDropOperationTarget {}
impl ::core::fmt::Debug for ICoreDropOperationTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoreDropOperationTarget").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ICoreDropOperationTarget {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{d9126196-4c5b-417d-bb37-76381def8db4}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ICoreDropOperationTarget {
    type Vtable = ICoreDropOperationTarget_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd9126196_4c5b_417d_bb37_76381def8db4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDropOperationTarget_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub EnterAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, draginfo: ::windows_core::RawPtr, draguioverride: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnterAsync: usize,
    #[cfg(feature = "Foundation")]
    pub OverAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, draginfo: ::windows_core::RawPtr, draguioverride: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OverAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LeaveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, draginfo: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LeaveAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DropAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, draginfo: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DropAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreDropOperationTargetRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreDropOperationTargetRequestedEventArgs {
    type Vtable = ICoreDropOperationTargetRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2aca929a_5e28_4ea6_829e_29134e665d6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDropOperationTargetRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
