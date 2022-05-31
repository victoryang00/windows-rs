#[repr(C)]
pub struct AutomationAnnotationTypeRegistration {
    pub LocalId: i32,
}
impl ::core::marker::Copy for AutomationAnnotationTypeRegistration {}
impl ::core::clone::Clone for AutomationAnnotationTypeRegistration {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AutomationAnnotationTypeRegistration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AutomationAnnotationTypeRegistration").field("LocalId", &self.LocalId).finish()
    }
}
unsafe impl ::windows_core::Abi for AutomationAnnotationTypeRegistration {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for AutomationAnnotationTypeRegistration {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.UI.UIAutomation.Core.AutomationAnnotationTypeRegistration;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for AutomationAnnotationTypeRegistration {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AutomationAnnotationTypeRegistration>()) == 0 }
    }
}
impl ::core::cmp::Eq for AutomationAnnotationTypeRegistration {}
impl ::core::default::Default for AutomationAnnotationTypeRegistration {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct AutomationRemoteOperationOperandId {
    pub Value: i32,
}
impl ::core::marker::Copy for AutomationRemoteOperationOperandId {}
impl ::core::clone::Clone for AutomationRemoteOperationOperandId {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AutomationRemoteOperationOperandId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AutomationRemoteOperationOperandId").field("Value", &self.Value).finish()
    }
}
unsafe impl ::windows_core::Abi for AutomationRemoteOperationOperandId {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for AutomationRemoteOperationOperandId {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.UI.UIAutomation.Core.AutomationRemoteOperationOperandId;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for AutomationRemoteOperationOperandId {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AutomationRemoteOperationOperandId>()) == 0 }
    }
}
impl ::core::cmp::Eq for AutomationRemoteOperationOperandId {}
impl ::core::default::Default for AutomationRemoteOperationOperandId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct AutomationRemoteOperationResult(::windows_core::IUnknown);
impl AutomationRemoteOperationResult {
    pub fn Status(&self) -> ::windows_core::Result<AutomationRemoteOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AutomationRemoteOperationStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationRemoteOperationStatus>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
    pub fn ErrorLocation(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).ErrorLocation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn HasOperand<'a, Param0: ::windows_core::IntoParam<'a, AutomationRemoteOperationOperandId>>(&self, operandid: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasOperand)(::windows_core::Interface::as_raw(this), operandid.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn GetOperand<'a, Param0: ::windows_core::IntoParam<'a, AutomationRemoteOperationOperandId>>(&self, operandid: Param0) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).GetOperand)(::windows_core::Interface::as_raw(this), operandid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
}
impl ::core::clone::Clone for AutomationRemoteOperationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AutomationRemoteOperationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutomationRemoteOperationResult {}
impl ::core::fmt::Debug for AutomationRemoteOperationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationRemoteOperationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AutomationRemoteOperationResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.Core.AutomationRemoteOperationResult;{e0f80c42-4a67-5534-bf5a-09e8a99b36b1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AutomationRemoteOperationResult {
    type Vtable = IAutomationRemoteOperationResult_Vtbl;
    const IID: ::windows_core::GUID = <IAutomationRemoteOperationResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AutomationRemoteOperationResult {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.AutomationRemoteOperationResult";
}
impl ::core::convert::From<AutomationRemoteOperationResult> for ::windows_core::IUnknown {
    fn from(value: AutomationRemoteOperationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutomationRemoteOperationResult> for ::windows_core::IUnknown {
    fn from(value: &AutomationRemoteOperationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AutomationRemoteOperationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AutomationRemoteOperationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AutomationRemoteOperationResult> for ::windows_core::IInspectable {
    fn from(value: AutomationRemoteOperationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutomationRemoteOperationResult> for ::windows_core::IInspectable {
    fn from(value: &AutomationRemoteOperationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AutomationRemoteOperationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AutomationRemoteOperationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AutomationRemoteOperationResult {}
unsafe impl ::core::marker::Sync for AutomationRemoteOperationResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AutomationRemoteOperationStatus(pub i32);
impl AutomationRemoteOperationStatus {
    pub const Success: Self = Self(0i32);
    pub const MalformedBytecode: Self = Self(1i32);
    pub const InstructionLimitExceeded: Self = Self(2i32);
    pub const UnhandledException: Self = Self(3i32);
    pub const ExecutionFailure: Self = Self(4i32);
}
impl ::core::marker::Copy for AutomationRemoteOperationStatus {}
impl ::core::clone::Clone for AutomationRemoteOperationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutomationRemoteOperationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AutomationRemoteOperationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for AutomationRemoteOperationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationRemoteOperationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AutomationRemoteOperationStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.UIAutomation.Core.AutomationRemoteOperationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
pub struct CoreAutomationRegistrar;
impl CoreAutomationRegistrar {
    pub fn RegisterAnnotationType<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(guid: Param0) -> ::windows_core::Result<AutomationAnnotationTypeRegistration> {
        Self::ICoreAutomationRegistrarStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AutomationAnnotationTypeRegistration>::zeroed();
            (::windows_core::Interface::vtable(this).RegisterAnnotationType)(::windows_core::Interface::as_raw(this), guid.into_param().abi(), result__.as_mut_ptr()).from_abi::<AutomationAnnotationTypeRegistration>(result__)
        })
    }
    pub fn UnregisterAnnotationType<'a, Param0: ::windows_core::IntoParam<'a, AutomationAnnotationTypeRegistration>>(registration: Param0) -> ::windows_core::Result<()> {
        Self::ICoreAutomationRegistrarStatics(|this| unsafe { (::windows_core::Interface::vtable(this).UnregisterAnnotationType)(::windows_core::Interface::as_raw(this), registration.into_param().abi()).ok() })
    }
    pub fn ICoreAutomationRegistrarStatics<R, F: FnOnce(&ICoreAutomationRegistrarStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CoreAutomationRegistrar, ICoreAutomationRegistrarStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for CoreAutomationRegistrar {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.CoreAutomationRegistrar";
}
#[repr(transparent)]
pub struct CoreAutomationRemoteOperation(::windows_core::IUnknown);
impl CoreAutomationRemoteOperation {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CoreAutomationRemoteOperation, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IsOpcodeSupported(&self, opcode: u32) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsOpcodeSupported)(::windows_core::Interface::as_raw(this), opcode, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ImportElement<'a, Param0: ::windows_core::IntoParam<'a, AutomationRemoteOperationOperandId>, Param1: ::windows_core::IntoParam<'a, super::AutomationElement>>(&self, operandid: Param0, element: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ImportElement)(::windows_core::Interface::as_raw(this), operandid.into_param().abi(), element.into_param().abi()).ok() }
    }
    pub fn ImportTextRange<'a, Param0: ::windows_core::IntoParam<'a, AutomationRemoteOperationOperandId>, Param1: ::windows_core::IntoParam<'a, super::AutomationTextRange>>(&self, operandid: Param0, textrange: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ImportTextRange)(::windows_core::Interface::as_raw(this), operandid.into_param().abi(), textrange.into_param().abi()).ok() }
    }
    pub fn AddToResults<'a, Param0: ::windows_core::IntoParam<'a, AutomationRemoteOperationOperandId>>(&self, operandid: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddToResults)(::windows_core::Interface::as_raw(this), operandid.into_param().abi()).ok() }
    }
    pub fn Execute(&self, bytecodebuffer: &[u8]) -> ::windows_core::Result<AutomationRemoteOperationResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Execute)(::windows_core::Interface::as_raw(this), bytecodebuffer.len() as u32, ::core::mem::transmute(bytecodebuffer.as_ptr()), result__.as_mut_ptr()).from_abi::<AutomationRemoteOperationResult>(result__)
        }
    }
    pub fn ImportConnectionBoundObject<'a, Param0: ::windows_core::IntoParam<'a, AutomationRemoteOperationOperandId>, Param1: ::windows_core::IntoParam<'a, super::AutomationConnectionBoundObject>>(&self, operandid: Param0, connectionboundobject: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ICoreAutomationRemoteOperation2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ImportConnectionBoundObject)(::windows_core::Interface::as_raw(this), operandid.into_param().abi(), connectionboundobject.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for CoreAutomationRemoteOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreAutomationRemoteOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreAutomationRemoteOperation {}
impl ::core::fmt::Debug for CoreAutomationRemoteOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreAutomationRemoteOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CoreAutomationRemoteOperation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.Core.CoreAutomationRemoteOperation;{3ac656f4-e2bc-5c6e-b8e7-b224fb74b060})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CoreAutomationRemoteOperation {
    type Vtable = ICoreAutomationRemoteOperation_Vtbl;
    const IID: ::windows_core::GUID = <ICoreAutomationRemoteOperation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CoreAutomationRemoteOperation {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.CoreAutomationRemoteOperation";
}
impl ::core::convert::From<CoreAutomationRemoteOperation> for ::windows_core::IUnknown {
    fn from(value: CoreAutomationRemoteOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreAutomationRemoteOperation> for ::windows_core::IUnknown {
    fn from(value: &CoreAutomationRemoteOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CoreAutomationRemoteOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CoreAutomationRemoteOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreAutomationRemoteOperation> for ::windows_core::IInspectable {
    fn from(value: CoreAutomationRemoteOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreAutomationRemoteOperation> for ::windows_core::IInspectable {
    fn from(value: &CoreAutomationRemoteOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CoreAutomationRemoteOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CoreAutomationRemoteOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreAutomationRemoteOperation {}
unsafe impl ::core::marker::Sync for CoreAutomationRemoteOperation {}
#[repr(transparent)]
pub struct CoreAutomationRemoteOperationContext(::windows_core::IUnknown);
impl CoreAutomationRemoteOperationContext {
    pub fn GetOperand<'a, Param0: ::windows_core::IntoParam<'a, AutomationRemoteOperationOperandId>>(&self, id: Param0) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).GetOperand)(::windows_core::Interface::as_raw(this), id.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn SetOperand<'a, Param0: ::windows_core::IntoParam<'a, AutomationRemoteOperationOperandId>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, id: Param0, operand: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOperand)(::windows_core::Interface::as_raw(this), id.into_param().abi(), operand.into_param().abi()).ok() }
    }
    pub fn SetOperand2<'a, Param0: ::windows_core::IntoParam<'a, AutomationRemoteOperationOperandId>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param2: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, id: Param0, operand: Param1, operandinterfaceid: Param2) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOperand2)(::windows_core::Interface::as_raw(this), id.into_param().abi(), operand.into_param().abi(), operandinterfaceid.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for CoreAutomationRemoteOperationContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreAutomationRemoteOperationContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreAutomationRemoteOperationContext {}
impl ::core::fmt::Debug for CoreAutomationRemoteOperationContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreAutomationRemoteOperationContext").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CoreAutomationRemoteOperationContext {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.Core.CoreAutomationRemoteOperationContext;{b9af9cbb-3d3e-5918-a16b-7861626a3aeb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CoreAutomationRemoteOperationContext {
    type Vtable = ICoreAutomationRemoteOperationContext_Vtbl;
    const IID: ::windows_core::GUID = <ICoreAutomationRemoteOperationContext as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CoreAutomationRemoteOperationContext {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.CoreAutomationRemoteOperationContext";
}
impl ::core::convert::From<CoreAutomationRemoteOperationContext> for ::windows_core::IUnknown {
    fn from(value: CoreAutomationRemoteOperationContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreAutomationRemoteOperationContext> for ::windows_core::IUnknown {
    fn from(value: &CoreAutomationRemoteOperationContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CoreAutomationRemoteOperationContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CoreAutomationRemoteOperationContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreAutomationRemoteOperationContext> for ::windows_core::IInspectable {
    fn from(value: CoreAutomationRemoteOperationContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreAutomationRemoteOperationContext> for ::windows_core::IInspectable {
    fn from(value: &CoreAutomationRemoteOperationContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CoreAutomationRemoteOperationContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CoreAutomationRemoteOperationContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreAutomationRemoteOperationContext {}
unsafe impl ::core::marker::Sync for CoreAutomationRemoteOperationContext {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationRemoteOperationResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutomationRemoteOperationResult {
    type Vtable = IAutomationRemoteOperationResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe0f80c42_4a67_5534_bf5a_09e8a99b36b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationRemoteOperationResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AutomationRemoteOperationStatus) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub ErrorLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub HasOperand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operandid: AutomationRemoteOperationOperandId, result__: *mut bool) -> ::windows_core::HRESULT,
    pub GetOperand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operandid: AutomationRemoteOperationOperandId, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICoreAutomationConnectionBoundObjectProvider(::windows_core::IUnknown);
impl ICoreAutomationConnectionBoundObjectProvider {
    pub fn IsComThreadingRequired(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsComThreadingRequired)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::convert::From<ICoreAutomationConnectionBoundObjectProvider> for ::windows_core::IUnknown {
    fn from(value: ICoreAutomationConnectionBoundObjectProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICoreAutomationConnectionBoundObjectProvider> for ::windows_core::IUnknown {
    fn from(value: &ICoreAutomationConnectionBoundObjectProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICoreAutomationConnectionBoundObjectProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICoreAutomationConnectionBoundObjectProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICoreAutomationConnectionBoundObjectProvider> for ::windows_core::IInspectable {
    fn from(value: ICoreAutomationConnectionBoundObjectProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICoreAutomationConnectionBoundObjectProvider> for ::windows_core::IInspectable {
    fn from(value: &ICoreAutomationConnectionBoundObjectProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ICoreAutomationConnectionBoundObjectProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ICoreAutomationConnectionBoundObjectProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICoreAutomationConnectionBoundObjectProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICoreAutomationConnectionBoundObjectProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoreAutomationConnectionBoundObjectProvider {}
impl ::core::fmt::Debug for ICoreAutomationConnectionBoundObjectProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoreAutomationConnectionBoundObjectProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ICoreAutomationConnectionBoundObjectProvider {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{0620bb64-9616-5593-be3a-eb8e6daeb3fa}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ICoreAutomationConnectionBoundObjectProvider {
    type Vtable = ICoreAutomationConnectionBoundObjectProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0620bb64_9616_5593_be3a_eb8e6daeb3fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAutomationConnectionBoundObjectProvider_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsComThreadingRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreAutomationRegistrarStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreAutomationRegistrarStatics {
    type Vtable = ICoreAutomationRegistrarStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3e50129b_d6dc_5680_b580_ffff78300304);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAutomationRegistrarStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RegisterAnnotationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: ::windows_core::GUID, result__: *mut AutomationAnnotationTypeRegistration) -> ::windows_core::HRESULT,
    pub UnregisterAnnotationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, registration: AutomationAnnotationTypeRegistration) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreAutomationRemoteOperation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreAutomationRemoteOperation {
    type Vtable = ICoreAutomationRemoteOperation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3ac656f4_e2bc_5c6e_b8e7_b224fb74b060);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAutomationRemoteOperation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsOpcodeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opcode: u32, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ImportElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operandid: AutomationRemoteOperationOperandId, element: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ImportTextRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operandid: AutomationRemoteOperationOperandId, textrange: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AddToResults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operandid: AutomationRemoteOperationOperandId) -> ::windows_core::HRESULT,
    pub Execute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bytecodeBuffer_array_size: u32, bytecodebuffer: *const u8, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreAutomationRemoteOperation2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreAutomationRemoteOperation2 {
    type Vtable = ICoreAutomationRemoteOperation2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeefaf86f_e953_5099_8ce9_dca813482ba0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAutomationRemoteOperation2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ImportConnectionBoundObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operandid: AutomationRemoteOperationOperandId, connectionboundobject: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreAutomationRemoteOperationContext(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreAutomationRemoteOperationContext {
    type Vtable = ICoreAutomationRemoteOperationContext_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9af9cbb_3d3e_5918_a16b_7861626a3aeb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAutomationRemoteOperationContext_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetOperand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: AutomationRemoteOperationOperandId, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetOperand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: AutomationRemoteOperationOperandId, operand: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetOperand2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: AutomationRemoteOperationOperandId, operand: *mut ::core::ffi::c_void, operandinterfaceid: ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICoreAutomationRemoteOperationExtensionProvider(::windows_core::IUnknown);
impl ICoreAutomationRemoteOperationExtensionProvider {
    pub fn CallExtension<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param1: ::windows_core::IntoParam<'a, CoreAutomationRemoteOperationContext>>(&self, extensionid: Param0, context: Param1, operandids: &[AutomationRemoteOperationOperandId]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).CallExtension)(::windows_core::Interface::as_raw(this), extensionid.into_param().abi(), context.into_param().abi(), operandids.len() as u32, ::core::mem::transmute(operandids.as_ptr())).ok() }
    }
    pub fn IsExtensionSupported<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, extensionid: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsExtensionSupported)(::windows_core::Interface::as_raw(this), extensionid.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::convert::From<ICoreAutomationRemoteOperationExtensionProvider> for ::windows_core::IUnknown {
    fn from(value: ICoreAutomationRemoteOperationExtensionProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICoreAutomationRemoteOperationExtensionProvider> for ::windows_core::IUnknown {
    fn from(value: &ICoreAutomationRemoteOperationExtensionProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICoreAutomationRemoteOperationExtensionProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICoreAutomationRemoteOperationExtensionProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICoreAutomationRemoteOperationExtensionProvider> for ::windows_core::IInspectable {
    fn from(value: ICoreAutomationRemoteOperationExtensionProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICoreAutomationRemoteOperationExtensionProvider> for ::windows_core::IInspectable {
    fn from(value: &ICoreAutomationRemoteOperationExtensionProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ICoreAutomationRemoteOperationExtensionProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ICoreAutomationRemoteOperationExtensionProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICoreAutomationRemoteOperationExtensionProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICoreAutomationRemoteOperationExtensionProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoreAutomationRemoteOperationExtensionProvider {}
impl ::core::fmt::Debug for ICoreAutomationRemoteOperationExtensionProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoreAutomationRemoteOperationExtensionProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ICoreAutomationRemoteOperationExtensionProvider {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{88f53e67-dc69-553b-a0aa-70477e724da8}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ICoreAutomationRemoteOperationExtensionProvider {
    type Vtable = ICoreAutomationRemoteOperationExtensionProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x88f53e67_dc69_553b_a0aa_70477e724da8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAutomationRemoteOperationExtensionProvider_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CallExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extensionid: ::windows_core::GUID, context: ::windows_core::RawPtr, operandIds_array_size: u32, operandids: *const AutomationRemoteOperationOperandId) -> ::windows_core::HRESULT,
    pub IsExtensionSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extensionid: ::windows_core::GUID, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteAutomationClientSession(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteAutomationClientSession {
    type Vtable = IRemoteAutomationClientSession_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5c8a091d_94cc_5b33_afdb_678cded2bd54);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteAutomationClientSession_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotewindowid: u64, remoteprocessid: u32, parentautomationelement: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SessionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub ConnectionRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveConnectionRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Disconnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveDisconnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteAutomationClientSessionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteAutomationClientSessionFactory {
    type Vtable = IRemoteAutomationClientSessionFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf250263d_6057_5373_a5a5_ed7265fe0376);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteAutomationClientSessionFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateInstance2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, sessionid: ::windows_core::GUID, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteAutomationConnectionRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteAutomationConnectionRequestedEventArgs {
    type Vtable = IRemoteAutomationConnectionRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xea3319a8_e3a8_5dc6_adf8_044e46b14af5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteAutomationConnectionRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LocalPipeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RemoteProcessId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteAutomationDisconnectedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteAutomationDisconnectedEventArgs {
    type Vtable = IRemoteAutomationDisconnectedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbbb33a3d_5d90_5c38_9eb2_dd9dcc1b2e3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteAutomationDisconnectedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LocalPipeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteAutomationServerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteAutomationServerStatics {
    type Vtable = IRemoteAutomationServerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe6e8945e_0c11_5028_9ae3_c2771288b6b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteAutomationServerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ReportSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteAutomationWindow(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteAutomationWindow {
    type Vtable = IRemoteAutomationWindow_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7c607689_496d_512a_9bd5_c050cfaf1428);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteAutomationWindow_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AutomationProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub UnregisterAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct RemoteAutomationClientSession(::windows_core::IUnknown);
impl RemoteAutomationClientSession {
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CreateWindowAsync<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, remotewindowid: u64, remoteprocessid: u32, parentautomationelement: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<RemoteAutomationWindow>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWindowAsync)(::windows_core::Interface::as_raw(this), remotewindowid, remoteprocessid, parentautomationelement.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<RemoteAutomationWindow>>(result__)
        }
    }
    pub fn SessionId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).SessionId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn ConnectionRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<RemoteAutomationClientSession, RemoteAutomationConnectionRequestedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveConnectionRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveConnectionRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Disconnected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<RemoteAutomationClientSession, RemoteAutomationDisconnectedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Disconnected)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDisconnected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDisconnected)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn CreateInstance<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(name: Param0) -> ::windows_core::Result<RemoteAutomationClientSession> {
        Self::IRemoteAutomationClientSessionFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<RemoteAutomationClientSession>(result__)
        })
    }
    pub fn CreateInstance2<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(name: Param0, sessionid: Param1) -> ::windows_core::Result<RemoteAutomationClientSession> {
        Self::IRemoteAutomationClientSessionFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance2)(::windows_core::Interface::as_raw(this), name.into_param().abi(), sessionid.into_param().abi(), result__.as_mut_ptr()).from_abi::<RemoteAutomationClientSession>(result__)
        })
    }
    pub fn IRemoteAutomationClientSessionFactory<R, F: FnOnce(&IRemoteAutomationClientSessionFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RemoteAutomationClientSession, IRemoteAutomationClientSessionFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RemoteAutomationClientSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteAutomationClientSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteAutomationClientSession {}
impl ::core::fmt::Debug for RemoteAutomationClientSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteAutomationClientSession").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteAutomationClientSession {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.Core.RemoteAutomationClientSession;{5c8a091d-94cc-5b33-afdb-678cded2bd54})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteAutomationClientSession {
    type Vtable = IRemoteAutomationClientSession_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteAutomationClientSession as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteAutomationClientSession {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.RemoteAutomationClientSession";
}
impl ::core::convert::From<RemoteAutomationClientSession> for ::windows_core::IUnknown {
    fn from(value: RemoteAutomationClientSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteAutomationClientSession> for ::windows_core::IUnknown {
    fn from(value: &RemoteAutomationClientSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteAutomationClientSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteAutomationClientSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteAutomationClientSession> for ::windows_core::IInspectable {
    fn from(value: RemoteAutomationClientSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteAutomationClientSession> for ::windows_core::IInspectable {
    fn from(value: &RemoteAutomationClientSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteAutomationClientSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteAutomationClientSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteAutomationClientSession {}
unsafe impl ::core::marker::Sync for RemoteAutomationClientSession {}
#[repr(transparent)]
pub struct RemoteAutomationConnectionRequestedEventArgs(::windows_core::IUnknown);
impl RemoteAutomationConnectionRequestedEventArgs {
    pub fn LocalPipeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).LocalPipeName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn RemoteProcessId(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).RemoteProcessId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteAutomationConnectionRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteAutomationConnectionRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteAutomationConnectionRequestedEventArgs {}
impl ::core::fmt::Debug for RemoteAutomationConnectionRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteAutomationConnectionRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteAutomationConnectionRequestedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.Core.RemoteAutomationConnectionRequestedEventArgs;{ea3319a8-e3a8-5dc6-adf8-044e46b14af5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteAutomationConnectionRequestedEventArgs {
    type Vtable = IRemoteAutomationConnectionRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteAutomationConnectionRequestedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteAutomationConnectionRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.RemoteAutomationConnectionRequestedEventArgs";
}
impl ::core::convert::From<RemoteAutomationConnectionRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: RemoteAutomationConnectionRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteAutomationConnectionRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &RemoteAutomationConnectionRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteAutomationConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteAutomationConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteAutomationConnectionRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: RemoteAutomationConnectionRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteAutomationConnectionRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &RemoteAutomationConnectionRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteAutomationConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteAutomationConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteAutomationConnectionRequestedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteAutomationConnectionRequestedEventArgs {}
#[repr(transparent)]
pub struct RemoteAutomationDisconnectedEventArgs(::windows_core::IUnknown);
impl RemoteAutomationDisconnectedEventArgs {
    pub fn LocalPipeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).LocalPipeName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteAutomationDisconnectedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteAutomationDisconnectedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteAutomationDisconnectedEventArgs {}
impl ::core::fmt::Debug for RemoteAutomationDisconnectedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteAutomationDisconnectedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteAutomationDisconnectedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.Core.RemoteAutomationDisconnectedEventArgs;{bbb33a3d-5d90-5c38-9eb2-dd9dcc1b2e3f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteAutomationDisconnectedEventArgs {
    type Vtable = IRemoteAutomationDisconnectedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteAutomationDisconnectedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteAutomationDisconnectedEventArgs {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.RemoteAutomationDisconnectedEventArgs";
}
impl ::core::convert::From<RemoteAutomationDisconnectedEventArgs> for ::windows_core::IUnknown {
    fn from(value: RemoteAutomationDisconnectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteAutomationDisconnectedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &RemoteAutomationDisconnectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteAutomationDisconnectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteAutomationDisconnectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteAutomationDisconnectedEventArgs> for ::windows_core::IInspectable {
    fn from(value: RemoteAutomationDisconnectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteAutomationDisconnectedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &RemoteAutomationDisconnectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteAutomationDisconnectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteAutomationDisconnectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteAutomationDisconnectedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteAutomationDisconnectedEventArgs {}
pub struct RemoteAutomationServer;
impl RemoteAutomationServer {
    pub fn ReportSession<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(sessionid: Param0) -> ::windows_core::Result<()> {
        Self::IRemoteAutomationServerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).ReportSession)(::windows_core::Interface::as_raw(this), sessionid.into_param().abi()).ok() })
    }
    pub fn IRemoteAutomationServerStatics<R, F: FnOnce(&IRemoteAutomationServerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RemoteAutomationServer, IRemoteAutomationServerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for RemoteAutomationServer {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.RemoteAutomationServer";
}
#[repr(transparent)]
pub struct RemoteAutomationWindow(::windows_core::IUnknown);
impl RemoteAutomationWindow {
    pub fn AutomationProvider(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).AutomationProvider)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn UnregisterAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UnregisterAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteAutomationWindow {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteAutomationWindow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteAutomationWindow {}
impl ::core::fmt::Debug for RemoteAutomationWindow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteAutomationWindow").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteAutomationWindow {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.Core.RemoteAutomationWindow;{7c607689-496d-512a-9bd5-c050cfaf1428})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteAutomationWindow {
    type Vtable = IRemoteAutomationWindow_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteAutomationWindow as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteAutomationWindow {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.RemoteAutomationWindow";
}
impl ::core::convert::From<RemoteAutomationWindow> for ::windows_core::IUnknown {
    fn from(value: RemoteAutomationWindow) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteAutomationWindow> for ::windows_core::IUnknown {
    fn from(value: &RemoteAutomationWindow) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteAutomationWindow {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteAutomationWindow {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteAutomationWindow> for ::windows_core::IInspectable {
    fn from(value: RemoteAutomationWindow) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteAutomationWindow> for ::windows_core::IInspectable {
    fn from(value: &RemoteAutomationWindow) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteAutomationWindow {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteAutomationWindow {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteAutomationWindow {}
unsafe impl ::core::marker::Sync for RemoteAutomationWindow {}
