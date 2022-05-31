#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CellularClass(pub i32);
impl CellularClass {
    pub const None: Self = Self(0i32);
    pub const Gsm: Self = Self(1i32);
    pub const Cdma: Self = Self(2i32);
}
impl ::core::marker::Copy for CellularClass {}
impl ::core::clone::Clone for CellularClass {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CellularClass {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CellularClass {
    type Abi = Self;
}
impl ::core::fmt::Debug for CellularClass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CellularClass").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CellularClass {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.CellularClass;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct DeleteSmsMessageOperation(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl DeleteSmsMessageOperation {
    pub fn SetCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::AsyncActionCompletedHandler>>(&self, handler: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi()).ok() }
    }
    pub fn Completed(&self) -> ::windows_core::Result<::winrt_foundation::AsyncActionCompletedHandler> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Completed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::AsyncActionCompletedHandler>(result__)
        }
    }
    pub fn GetResults(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).GetResults)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<::winrt_foundation::AsyncStatus> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::AsyncStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::AsyncStatus>(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
    pub fn Cancel(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Cancel)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for DeleteSmsMessageOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for DeleteSmsMessageOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for DeleteSmsMessageOperation {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for DeleteSmsMessageOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeleteSmsMessageOperation").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for DeleteSmsMessageOperation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.DeleteSmsMessageOperation;{5a648006-843a-4da9-865b-9d26e5dfad7b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for DeleteSmsMessageOperation {
    type Vtable = ::winrt_foundation::IAsyncAction_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_foundation::IAsyncAction as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for DeleteSmsMessageOperation {
    const NAME: &'static str = "Windows.Devices.Sms.DeleteSmsMessageOperation";
}
#[cfg(feature = "winrt-")]
impl DeleteSmsMessageOperation {
    pub fn get(&self) -> ::windows_core::Result<()> {
        if self.Status()? == ::winrt_foundation::AsyncStatus::Started {
            let (_waiter, signaler) = ::windows_core::Waiter::new()?;
            self.SetCompleted(::winrt_foundation::AsyncActionCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
#[cfg(feature = "winrt-")]
impl ::std::future::Future for DeleteSmsMessageOperation {
    type Output = ::windows_core::Result<()>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == ::winrt_foundation::AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(::winrt_foundation::AsyncActionCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<DeleteSmsMessageOperation> for ::windows_core::IUnknown {
    fn from(value: DeleteSmsMessageOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&DeleteSmsMessageOperation> for ::windows_core::IUnknown {
    fn from(value: &DeleteSmsMessageOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DeleteSmsMessageOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DeleteSmsMessageOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<DeleteSmsMessageOperation> for ::windows_core::IInspectable {
    fn from(value: DeleteSmsMessageOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&DeleteSmsMessageOperation> for ::windows_core::IInspectable {
    fn from(value: &DeleteSmsMessageOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DeleteSmsMessageOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DeleteSmsMessageOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<DeleteSmsMessageOperation> for ::winrt_foundation::IAsyncAction {
    type Error = ::windows_core::Error;
    fn try_from(value: DeleteSmsMessageOperation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<&DeleteSmsMessageOperation> for ::winrt_foundation::IAsyncAction {
    type Error = ::windows_core::Error;
    fn try_from(value: &DeleteSmsMessageOperation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IAsyncAction> for DeleteSmsMessageOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IAsyncAction> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IAsyncAction> for &DeleteSmsMessageOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IAsyncAction> {
        ::core::convert::TryInto::<::winrt_foundation::IAsyncAction>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<DeleteSmsMessageOperation> for ::winrt_foundation::IAsyncInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: DeleteSmsMessageOperation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<&DeleteSmsMessageOperation> for ::winrt_foundation::IAsyncInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: &DeleteSmsMessageOperation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IAsyncInfo> for DeleteSmsMessageOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IAsyncInfo> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IAsyncInfo> for &DeleteSmsMessageOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IAsyncInfo> {
        ::core::convert::TryInto::<::winrt_foundation::IAsyncInfo>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct DeleteSmsMessagesOperation(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl DeleteSmsMessagesOperation {
    pub fn SetCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::AsyncActionCompletedHandler>>(&self, handler: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi()).ok() }
    }
    pub fn Completed(&self) -> ::windows_core::Result<::winrt_foundation::AsyncActionCompletedHandler> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Completed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::AsyncActionCompletedHandler>(result__)
        }
    }
    pub fn GetResults(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).GetResults)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<::winrt_foundation::AsyncStatus> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::AsyncStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::AsyncStatus>(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
    pub fn Cancel(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Cancel)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for DeleteSmsMessagesOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for DeleteSmsMessagesOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for DeleteSmsMessagesOperation {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for DeleteSmsMessagesOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeleteSmsMessagesOperation").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for DeleteSmsMessagesOperation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.DeleteSmsMessagesOperation;{5a648006-843a-4da9-865b-9d26e5dfad7b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for DeleteSmsMessagesOperation {
    type Vtable = ::winrt_foundation::IAsyncAction_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_foundation::IAsyncAction as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for DeleteSmsMessagesOperation {
    const NAME: &'static str = "Windows.Devices.Sms.DeleteSmsMessagesOperation";
}
#[cfg(feature = "winrt-")]
impl DeleteSmsMessagesOperation {
    pub fn get(&self) -> ::windows_core::Result<()> {
        if self.Status()? == ::winrt_foundation::AsyncStatus::Started {
            let (_waiter, signaler) = ::windows_core::Waiter::new()?;
            self.SetCompleted(::winrt_foundation::AsyncActionCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
#[cfg(feature = "winrt-")]
impl ::std::future::Future for DeleteSmsMessagesOperation {
    type Output = ::windows_core::Result<()>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == ::winrt_foundation::AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(::winrt_foundation::AsyncActionCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<DeleteSmsMessagesOperation> for ::windows_core::IUnknown {
    fn from(value: DeleteSmsMessagesOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&DeleteSmsMessagesOperation> for ::windows_core::IUnknown {
    fn from(value: &DeleteSmsMessagesOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DeleteSmsMessagesOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DeleteSmsMessagesOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<DeleteSmsMessagesOperation> for ::windows_core::IInspectable {
    fn from(value: DeleteSmsMessagesOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&DeleteSmsMessagesOperation> for ::windows_core::IInspectable {
    fn from(value: &DeleteSmsMessagesOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DeleteSmsMessagesOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DeleteSmsMessagesOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<DeleteSmsMessagesOperation> for ::winrt_foundation::IAsyncAction {
    type Error = ::windows_core::Error;
    fn try_from(value: DeleteSmsMessagesOperation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<&DeleteSmsMessagesOperation> for ::winrt_foundation::IAsyncAction {
    type Error = ::windows_core::Error;
    fn try_from(value: &DeleteSmsMessagesOperation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IAsyncAction> for DeleteSmsMessagesOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IAsyncAction> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IAsyncAction> for &DeleteSmsMessagesOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IAsyncAction> {
        ::core::convert::TryInto::<::winrt_foundation::IAsyncAction>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<DeleteSmsMessagesOperation> for ::winrt_foundation::IAsyncInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: DeleteSmsMessagesOperation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<&DeleteSmsMessagesOperation> for ::winrt_foundation::IAsyncInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: &DeleteSmsMessagesOperation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IAsyncInfo> for DeleteSmsMessagesOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IAsyncInfo> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IAsyncInfo> for &DeleteSmsMessagesOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IAsyncInfo> {
        ::core::convert::TryInto::<::winrt_foundation::IAsyncInfo>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct GetSmsDeviceOperation(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl GetSmsDeviceOperation {
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<::winrt_foundation::AsyncStatus> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::AsyncStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::AsyncStatus>(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
    pub fn Cancel(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Cancel)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::AsyncOperationCompletedHandler<SmsDevice>>>(&self, handler: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi()).ok() }
    }
    pub fn Completed(&self) -> ::windows_core::Result<::winrt_foundation::AsyncOperationCompletedHandler<SmsDevice>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Completed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::AsyncOperationCompletedHandler<SmsDevice>>(result__)
        }
    }
    pub fn GetResults(&self) -> ::windows_core::Result<SmsDevice> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetResults)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsDevice>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for GetSmsDeviceOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for GetSmsDeviceOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for GetSmsDeviceOperation {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for GetSmsDeviceOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GetSmsDeviceOperation").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for GetSmsDeviceOperation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.GetSmsDeviceOperation;pinterface({9fc2b0bb-e446-44e2-aa61-9cab8f636af2};rc(Windows.Devices.Sms.SmsDevice;{091791ed-872b-4eec-9c72-ab11627b34ec})))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for GetSmsDeviceOperation {
    type Vtable = ::winrt_foundation::IAsyncOperation_Vtbl<SmsDevice>;
    const IID: ::windows_core::GUID = <::winrt_foundation::IAsyncOperation<SmsDevice> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for GetSmsDeviceOperation {
    const NAME: &'static str = "Windows.Devices.Sms.GetSmsDeviceOperation";
}
#[cfg(feature = "winrt-")]
impl GetSmsDeviceOperation {
    pub fn get(&self) -> ::windows_core::Result<SmsDevice> {
        if self.Status()? == ::winrt_foundation::AsyncStatus::Started {
            let (_waiter, signaler) = ::windows_core::Waiter::new()?;
            self.SetCompleted(::winrt_foundation::AsyncOperationCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
#[cfg(feature = "winrt-")]
impl ::std::future::Future for GetSmsDeviceOperation {
    type Output = ::windows_core::Result<SmsDevice>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == ::winrt_foundation::AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(::winrt_foundation::AsyncOperationCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<GetSmsDeviceOperation> for ::windows_core::IUnknown {
    fn from(value: GetSmsDeviceOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&GetSmsDeviceOperation> for ::windows_core::IUnknown {
    fn from(value: &GetSmsDeviceOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GetSmsDeviceOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GetSmsDeviceOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<GetSmsDeviceOperation> for ::windows_core::IInspectable {
    fn from(value: GetSmsDeviceOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&GetSmsDeviceOperation> for ::windows_core::IInspectable {
    fn from(value: &GetSmsDeviceOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GetSmsDeviceOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GetSmsDeviceOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<GetSmsDeviceOperation> for ::winrt_foundation::IAsyncInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: GetSmsDeviceOperation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<&GetSmsDeviceOperation> for ::winrt_foundation::IAsyncInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: &GetSmsDeviceOperation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IAsyncInfo> for GetSmsDeviceOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IAsyncInfo> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IAsyncInfo> for &GetSmsDeviceOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IAsyncInfo> {
        ::core::convert::TryInto::<::winrt_foundation::IAsyncInfo>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<GetSmsDeviceOperation> for ::winrt_foundation::IAsyncOperation<SmsDevice> {
    type Error = ::windows_core::Error;
    fn try_from(value: GetSmsDeviceOperation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<&GetSmsDeviceOperation> for ::winrt_foundation::IAsyncOperation<SmsDevice> {
    type Error = ::windows_core::Error;
    fn try_from(value: &GetSmsDeviceOperation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IAsyncOperation<SmsDevice>> for GetSmsDeviceOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IAsyncOperation<SmsDevice>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IAsyncOperation<SmsDevice>> for &GetSmsDeviceOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IAsyncOperation<SmsDevice>> {
        ::core::convert::TryInto::<::winrt_foundation::IAsyncOperation<SmsDevice>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct GetSmsMessageOperation(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl GetSmsMessageOperation {
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<::winrt_foundation::AsyncStatus> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::AsyncStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::AsyncStatus>(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
    pub fn Cancel(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Cancel)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::AsyncOperationCompletedHandler<ISmsMessage>>>(&self, handler: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi()).ok() }
    }
    pub fn Completed(&self) -> ::windows_core::Result<::winrt_foundation::AsyncOperationCompletedHandler<ISmsMessage>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Completed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::AsyncOperationCompletedHandler<ISmsMessage>>(result__)
        }
    }
    pub fn GetResults(&self) -> ::windows_core::Result<ISmsMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetResults)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ISmsMessage>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for GetSmsMessageOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for GetSmsMessageOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for GetSmsMessageOperation {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for GetSmsMessageOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GetSmsMessageOperation").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for GetSmsMessageOperation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.GetSmsMessageOperation;pinterface({9fc2b0bb-e446-44e2-aa61-9cab8f636af2};{ed3c5e28-6984-4b07-811d-8d5906ed3cea}))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for GetSmsMessageOperation {
    type Vtable = ::winrt_foundation::IAsyncOperation_Vtbl<ISmsMessage>;
    const IID: ::windows_core::GUID = <::winrt_foundation::IAsyncOperation<ISmsMessage> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for GetSmsMessageOperation {
    const NAME: &'static str = "Windows.Devices.Sms.GetSmsMessageOperation";
}
#[cfg(feature = "winrt-")]
impl GetSmsMessageOperation {
    pub fn get(&self) -> ::windows_core::Result<ISmsMessage> {
        if self.Status()? == ::winrt_foundation::AsyncStatus::Started {
            let (_waiter, signaler) = ::windows_core::Waiter::new()?;
            self.SetCompleted(::winrt_foundation::AsyncOperationCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
#[cfg(feature = "winrt-")]
impl ::std::future::Future for GetSmsMessageOperation {
    type Output = ::windows_core::Result<ISmsMessage>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == ::winrt_foundation::AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(::winrt_foundation::AsyncOperationCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<GetSmsMessageOperation> for ::windows_core::IUnknown {
    fn from(value: GetSmsMessageOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&GetSmsMessageOperation> for ::windows_core::IUnknown {
    fn from(value: &GetSmsMessageOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GetSmsMessageOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GetSmsMessageOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<GetSmsMessageOperation> for ::windows_core::IInspectable {
    fn from(value: GetSmsMessageOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&GetSmsMessageOperation> for ::windows_core::IInspectable {
    fn from(value: &GetSmsMessageOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GetSmsMessageOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GetSmsMessageOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<GetSmsMessageOperation> for ::winrt_foundation::IAsyncInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: GetSmsMessageOperation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<&GetSmsMessageOperation> for ::winrt_foundation::IAsyncInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: &GetSmsMessageOperation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IAsyncInfo> for GetSmsMessageOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IAsyncInfo> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IAsyncInfo> for &GetSmsMessageOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IAsyncInfo> {
        ::core::convert::TryInto::<::winrt_foundation::IAsyncInfo>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<GetSmsMessageOperation> for ::winrt_foundation::IAsyncOperation<ISmsMessage> {
    type Error = ::windows_core::Error;
    fn try_from(value: GetSmsMessageOperation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<&GetSmsMessageOperation> for ::winrt_foundation::IAsyncOperation<ISmsMessage> {
    type Error = ::windows_core::Error;
    fn try_from(value: &GetSmsMessageOperation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IAsyncOperation<ISmsMessage>> for GetSmsMessageOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IAsyncOperation<ISmsMessage>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IAsyncOperation<ISmsMessage>> for &GetSmsMessageOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IAsyncOperation<ISmsMessage>> {
        ::core::convert::TryInto::<::winrt_foundation::IAsyncOperation<ISmsMessage>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct GetSmsMessagesOperation(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl GetSmsMessagesOperation {
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<::winrt_foundation::AsyncStatus> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::AsyncStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::AsyncStatus>(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
    pub fn Cancel(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Cancel)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetProgress<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::AsyncOperationProgressHandler<::winrt_foundation::Collections::IVectorView<ISmsMessage>, i32>>>(&self, handler: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetProgress)(::windows_core::Interface::as_raw(this), handler.into_param().abi()).ok() }
    }
    pub fn Progress(&self) -> ::windows_core::Result<::winrt_foundation::AsyncOperationProgressHandler<::winrt_foundation::Collections::IVectorView<ISmsMessage>, i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Progress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::AsyncOperationProgressHandler<::winrt_foundation::Collections::IVectorView<ISmsMessage>, i32>>(result__)
        }
    }
    pub fn SetCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::AsyncOperationWithProgressCompletedHandler<::winrt_foundation::Collections::IVectorView<ISmsMessage>, i32>>>(&self, handler: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi()).ok() }
    }
    pub fn Completed(&self) -> ::windows_core::Result<::winrt_foundation::AsyncOperationWithProgressCompletedHandler<::winrt_foundation::Collections::IVectorView<ISmsMessage>, i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Completed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::AsyncOperationWithProgressCompletedHandler<::winrt_foundation::Collections::IVectorView<ISmsMessage>, i32>>(result__)
        }
    }
    pub fn GetResults(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<ISmsMessage>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetResults)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<ISmsMessage>>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for GetSmsMessagesOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for GetSmsMessagesOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for GetSmsMessagesOperation {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for GetSmsMessagesOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GetSmsMessagesOperation").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for GetSmsMessagesOperation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.GetSmsMessagesOperation;pinterface({b5d036d7-e297-498f-ba60-0289e76e23dd};pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};{ed3c5e28-6984-4b07-811d-8d5906ed3cea});i4))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for GetSmsMessagesOperation {
    type Vtable = ::winrt_foundation::IAsyncOperationWithProgress_Vtbl<::winrt_foundation::Collections::IVectorView<ISmsMessage>, i32>;
    const IID: ::windows_core::GUID = <::winrt_foundation::IAsyncOperationWithProgress<::winrt_foundation::Collections::IVectorView<ISmsMessage>, i32> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for GetSmsMessagesOperation {
    const NAME: &'static str = "Windows.Devices.Sms.GetSmsMessagesOperation";
}
#[cfg(feature = "winrt-")]
impl GetSmsMessagesOperation {
    pub fn get(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<ISmsMessage>> {
        if self.Status()? == ::winrt_foundation::AsyncStatus::Started {
            let (_waiter, signaler) = ::windows_core::Waiter::new()?;
            self.SetCompleted(::winrt_foundation::AsyncOperationWithProgressCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
#[cfg(feature = "winrt-")]
impl ::std::future::Future for GetSmsMessagesOperation {
    type Output = ::windows_core::Result<::winrt_foundation::Collections::IVectorView<ISmsMessage>>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == ::winrt_foundation::AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(::winrt_foundation::AsyncOperationWithProgressCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<GetSmsMessagesOperation> for ::windows_core::IUnknown {
    fn from(value: GetSmsMessagesOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&GetSmsMessagesOperation> for ::windows_core::IUnknown {
    fn from(value: &GetSmsMessagesOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GetSmsMessagesOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GetSmsMessagesOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<GetSmsMessagesOperation> for ::windows_core::IInspectable {
    fn from(value: GetSmsMessagesOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&GetSmsMessagesOperation> for ::windows_core::IInspectable {
    fn from(value: &GetSmsMessagesOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GetSmsMessagesOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GetSmsMessagesOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<GetSmsMessagesOperation> for ::winrt_foundation::IAsyncInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: GetSmsMessagesOperation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<&GetSmsMessagesOperation> for ::winrt_foundation::IAsyncInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: &GetSmsMessagesOperation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IAsyncInfo> for GetSmsMessagesOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IAsyncInfo> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IAsyncInfo> for &GetSmsMessagesOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IAsyncInfo> {
        ::core::convert::TryInto::<::winrt_foundation::IAsyncInfo>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
impl ::core::convert::TryFrom<GetSmsMessagesOperation> for ::winrt_foundation::IAsyncOperationWithProgress<::winrt_foundation::Collections::IVectorView<ISmsMessage>, i32> {
    type Error = ::windows_core::Error;
    fn try_from(value: GetSmsMessagesOperation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
impl ::core::convert::TryFrom<&GetSmsMessagesOperation> for ::winrt_foundation::IAsyncOperationWithProgress<::winrt_foundation::Collections::IVectorView<ISmsMessage>, i32> {
    type Error = ::windows_core::Error;
    fn try_from(value: &GetSmsMessagesOperation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IAsyncOperationWithProgress<::winrt_foundation::Collections::IVectorView<ISmsMessage>, i32>> for GetSmsMessagesOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IAsyncOperationWithProgress<::winrt_foundation::Collections::IVectorView<ISmsMessage>, i32>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IAsyncOperationWithProgress<::winrt_foundation::Collections::IVectorView<ISmsMessage>, i32>> for &GetSmsMessagesOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IAsyncOperationWithProgress<::winrt_foundation::Collections::IVectorView<ISmsMessage>, i32>> {
        ::core::convert::TryInto::<::winrt_foundation::IAsyncOperationWithProgress<::winrt_foundation::Collections::IVectorView<ISmsMessage>, i32>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmsAppMessage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmsAppMessage {
    type Vtable = ISmsAppMessage_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe8bb8494_d3a0_4a0a_86d7_291033a8cf54);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsAppMessage_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub To: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub From: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Body: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CallbackNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCallbackNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsDeliveryNotificationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsDeliveryNotificationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub RetryAttemptCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetRetryAttemptCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub Encoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmsEncoding) -> ::windows_core::HRESULT,
    pub SetEncoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SmsEncoding) -> ::windows_core::HRESULT,
    pub PortNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetPortNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub TeleserviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetTeleserviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub ProtocolId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetProtocolId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub BinaryBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    BinaryBody: usize,
    #[cfg(feature = "winrt-storage")]
    pub SetBinaryBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    SetBinaryBody: usize,
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ISmsBinaryMessage(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl ISmsBinaryMessage {
    #[cfg(feature = "winrt-")]
    pub fn Format(&self) -> ::windows_core::Result<SmsDataFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmsDataFormat>::zeroed();
            (::windows_core::Interface::vtable(this).Format)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsDataFormat>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetFormat(&self, value: SmsDataFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFormat)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn GetData(&self) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::Array<u8>>::zeroed();
            (::windows_core::Interface::vtable(this).GetData)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetData(&self, value: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetData)(::windows_core::Interface::as_raw(this), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<ISmsMessage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn MessageClass(&self) -> ::windows_core::Result<SmsMessageClass> {
        let this = &::windows_core::Interface::cast::<ISmsMessage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmsMessageClass>::zeroed();
            (::windows_core::Interface::vtable(this).MessageClass)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsMessageClass>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<ISmsBinaryMessage> for ::windows_core::IUnknown {
    fn from(value: ISmsBinaryMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&ISmsBinaryMessage> for ::windows_core::IUnknown {
    fn from(value: &ISmsBinaryMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISmsBinaryMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISmsBinaryMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<ISmsBinaryMessage> for ::windows_core::IInspectable {
    fn from(value: ISmsBinaryMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&ISmsBinaryMessage> for ::windows_core::IInspectable {
    fn from(value: &ISmsBinaryMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ISmsBinaryMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ISmsBinaryMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<ISmsBinaryMessage> for ISmsMessage {
    type Error = ::windows_core::Error;
    fn try_from(value: ISmsBinaryMessage) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<&ISmsBinaryMessage> for ISmsMessage {
    type Error = ::windows_core::Error;
    fn try_from(value: &ISmsBinaryMessage) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ISmsMessage> for ISmsBinaryMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ISmsMessage> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ISmsMessage> for &ISmsBinaryMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ISmsMessage> {
        ::core::convert::TryInto::<ISmsMessage>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for ISmsBinaryMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for ISmsBinaryMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for ISmsBinaryMessage {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for ISmsBinaryMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISmsBinaryMessage").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for ISmsBinaryMessage {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{5bf4e813-3b53-4c6e-b61a-d86a63755650}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ISmsBinaryMessage {
    type Vtable = ISmsBinaryMessage_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5bf4e813_3b53_4c6e_b61a_d86a63755650);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ISmsBinaryMessage_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmsDataFormat) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Format: usize,
    #[cfg(feature = "winrt-")]
    pub SetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SmsDataFormat) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetFormat: usize,
    #[cfg(feature = "winrt-")]
    pub GetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    GetData: usize,
    #[cfg(feature = "winrt-")]
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmsBroadcastMessage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmsBroadcastMessage {
    type Vtable = ISmsBroadcastMessage_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x75aebbf1_e4b7_4874_a09c_2956e592f957);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsBroadcastMessage_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub To: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Body: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Channel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub GeographicalScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmsGeographicalScope) -> ::windows_core::HRESULT,
    pub MessageCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub UpdateNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub BroadcastType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmsBroadcastType) -> ::windows_core::HRESULT,
    pub IsEmergencyAlert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsUserPopupRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ISmsDevice(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl ISmsDevice {
    #[cfg(feature = "winrt-")]
    pub fn SendMessageAsync<'a, Param0: ::windows_core::IntoParam<'a, ISmsMessage>>(&self, message: Param0) -> ::windows_core::Result<SendSmsMessageOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SendMessageAsync)(::windows_core::Interface::as_raw(this), message.into_param().abi(), result__.as_mut_ptr()).from_abi::<SendSmsMessageOperation>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn CalculateLength<'a, Param0: ::windows_core::IntoParam<'a, SmsTextMessage>>(&self, message: Param0) -> ::windows_core::Result<SmsEncodedLength> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmsEncodedLength>::zeroed();
            (::windows_core::Interface::vtable(this).CalculateLength)(::windows_core::Interface::as_raw(this), message.into_param().abi(), result__.as_mut_ptr()).from_abi::<SmsEncodedLength>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn AccountPhoneNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AccountPhoneNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn CellularClass(&self) -> ::windows_core::Result<CellularClass> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<CellularClass>::zeroed();
            (::windows_core::Interface::vtable(this).CellularClass)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CellularClass>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn MessageStore(&self) -> ::windows_core::Result<SmsDeviceMessageStore> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MessageStore)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsDeviceMessageStore>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn DeviceStatus(&self) -> ::windows_core::Result<SmsDeviceStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmsDeviceStatus>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsDeviceStatus>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SmsMessageReceived<'a, Param0: ::windows_core::IntoParam<'a, SmsMessageReceivedEventHandler>>(&self, eventhandler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SmsMessageReceived)(::windows_core::Interface::as_raw(this), eventhandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveSmsMessageReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSmsMessageReceived)(::windows_core::Interface::as_raw(this), eventcookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn SmsDeviceStatusChanged<'a, Param0: ::windows_core::IntoParam<'a, SmsDeviceStatusChangedEventHandler>>(&self, eventhandler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SmsDeviceStatusChanged)(::windows_core::Interface::as_raw(this), eventhandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveSmsDeviceStatusChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSmsDeviceStatusChanged)(::windows_core::Interface::as_raw(this), eventcookie.into_param().abi()).ok() }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<ISmsDevice> for ::windows_core::IUnknown {
    fn from(value: ISmsDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&ISmsDevice> for ::windows_core::IUnknown {
    fn from(value: &ISmsDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISmsDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISmsDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<ISmsDevice> for ::windows_core::IInspectable {
    fn from(value: ISmsDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&ISmsDevice> for ::windows_core::IInspectable {
    fn from(value: &ISmsDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ISmsDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ISmsDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for ISmsDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for ISmsDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for ISmsDevice {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for ISmsDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISmsDevice").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for ISmsDevice {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{091791ed-872b-4eec-9c72-ab11627b34ec}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ISmsDevice {
    type Vtable = ISmsDevice_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x091791ed_872b_4eec_9c72_ab11627b34ec);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ISmsDevice_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub SendMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SendMessageAsync: usize,
    #[cfg(feature = "winrt-")]
    pub CalculateLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: ::windows_core::RawPtr, result__: *mut SmsEncodedLength) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    CalculateLength: usize,
    #[cfg(feature = "winrt-")]
    pub AccountPhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    AccountPhoneNumber: usize,
    #[cfg(feature = "winrt-")]
    pub CellularClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CellularClass) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    CellularClass: usize,
    #[cfg(feature = "winrt-")]
    pub MessageStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    MessageStore: usize,
    #[cfg(feature = "winrt-")]
    pub DeviceStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmsDeviceStatus) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    DeviceStatus: usize,
    #[cfg(feature = "winrt-")]
    pub SmsMessageReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SmsMessageReceived: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveSmsMessageReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveSmsMessageReceived: usize,
    #[cfg(feature = "winrt-")]
    pub SmsDeviceStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SmsDeviceStatusChanged: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveSmsDeviceStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveSmsDeviceStatusChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmsDevice2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmsDevice2 {
    type Vtable = ISmsDevice2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbd8a5c13_e522_46cb_b8d5_9ead30fb6c47);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsDevice2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SmscAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetSmscAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ParentDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AccountPhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CellularClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CellularClass) -> ::windows_core::HRESULT,
    pub DeviceStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmsDeviceStatus) -> ::windows_core::HRESULT,
    pub CalculateLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: ::windows_core::RawPtr, result__: *mut SmsEncodedLength) -> ::windows_core::HRESULT,
    pub SendMessageAndGetResultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DeviceStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveDeviceStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmsDevice2Statics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmsDevice2Statics {
    type Vtable = ISmsDevice2Statics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x65c78325_1031_491e_8fb6_ef9991afe363);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsDevice2Statics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FromId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FromParentId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parentdeviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ISmsDeviceMessageStore(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ISmsDeviceMessageStore {
    type Vtable = ISmsDeviceMessageStore_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9889f253_f188_4427_8d54_ce0c2423c5c1);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ISmsDeviceMessageStore_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub DeleteMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messageid: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    DeleteMessageAsync: usize,
    #[cfg(feature = "winrt-")]
    pub DeleteMessagesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagefilter: SmsMessageFilter, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    DeleteMessagesAsync: usize,
    #[cfg(feature = "winrt-")]
    pub GetMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messageid: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    GetMessageAsync: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub GetMessagesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagefilter: SmsMessageFilter, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    GetMessagesAsync: usize,
    #[cfg(feature = "winrt-")]
    pub MaxMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    MaxMessages: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ISmsDeviceStatics(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ISmsDeviceStatics {
    type Vtable = ISmsDeviceStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf88d07ea_d815_4dd1_a234_4520ce4604a4);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ISmsDeviceStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    GetDeviceSelector: usize,
    #[cfg(feature = "winrt-")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    FromIdAsync: usize,
    #[cfg(feature = "winrt-")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    GetDefaultAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ISmsDeviceStatics2(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ISmsDeviceStatics2 {
    type Vtable = ISmsDeviceStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2ca11c87_0873_4caf_8a7d_bd471e8586d1);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ISmsDeviceStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub FromNetworkAccountIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, networkaccountid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    FromNetworkAccountIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmsFilterRule(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmsFilterRule {
    type Vtable = ISmsFilterRule_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x40e32fae_b049_4fbc_afe9_e2a610eff55c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsFilterRule_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MessageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmsMessageType) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub ImsiPrefixes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ImsiPrefixes: usize,
    #[cfg(feature = "winrt-foundation")]
    pub DeviceIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    DeviceIds: usize,
    #[cfg(feature = "winrt-foundation")]
    pub SenderNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SenderNumbers: usize,
    #[cfg(feature = "winrt-foundation")]
    pub TextMessagePrefixes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    TextMessagePrefixes: usize,
    #[cfg(feature = "winrt-foundation")]
    pub PortNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    PortNumbers: usize,
    pub CellularClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CellularClass) -> ::windows_core::HRESULT,
    pub SetCellularClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CellularClass) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub ProtocolIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ProtocolIds: usize,
    #[cfg(feature = "winrt-foundation")]
    pub TeleserviceIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    TeleserviceIds: usize,
    #[cfg(feature = "winrt-foundation")]
    pub WapApplicationIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    WapApplicationIds: usize,
    #[cfg(feature = "winrt-foundation")]
    pub WapContentTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    WapContentTypes: usize,
    #[cfg(feature = "winrt-foundation")]
    pub BroadcastTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    BroadcastTypes: usize,
    #[cfg(feature = "winrt-foundation")]
    pub BroadcastChannels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    BroadcastChannels: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmsFilterRuleFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmsFilterRuleFactory {
    type Vtable = ISmsFilterRuleFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00c36508_6296_4f29_9aad_8920ceba3ce8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsFilterRuleFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateFilterRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagetype: SmsMessageType, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmsFilterRules(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmsFilterRules {
    type Vtable = ISmsFilterRules_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4e47eafb_79cd_4881_9894_55a4135b23fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsFilterRules_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ActionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmsFilterActionType) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Rules: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Rules: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmsFilterRulesFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmsFilterRulesFactory {
    type Vtable = ISmsFilterRulesFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa09924ed_6e2e_4530_9fde_465d02eed00e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsFilterRulesFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateFilterRules: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actiontype: SmsFilterActionType, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISmsMessage(::windows_core::IUnknown);
impl ISmsMessage {
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn MessageClass(&self) -> ::windows_core::Result<SmsMessageClass> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmsMessageClass>::zeroed();
            (::windows_core::Interface::vtable(this).MessageClass)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsMessageClass>(result__)
        }
    }
}
impl ::core::convert::From<ISmsMessage> for ::windows_core::IUnknown {
    fn from(value: ISmsMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISmsMessage> for ::windows_core::IUnknown {
    fn from(value: &ISmsMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISmsMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISmsMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISmsMessage> for ::windows_core::IInspectable {
    fn from(value: ISmsMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISmsMessage> for ::windows_core::IInspectable {
    fn from(value: &ISmsMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ISmsMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ISmsMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISmsMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISmsMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISmsMessage {}
impl ::core::fmt::Debug for ISmsMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISmsMessage").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ISmsMessage {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{ed3c5e28-6984-4b07-811d-8d5906ed3cea}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ISmsMessage {
    type Vtable = ISmsMessage_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xed3c5e28_6984_4b07_811d_8d5906ed3cea);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsMessage_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub MessageClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmsMessageClass) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISmsMessageBase(::windows_core::IUnknown);
impl ISmsMessageBase {
    pub fn MessageType(&self) -> ::windows_core::Result<SmsMessageType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmsMessageType>::zeroed();
            (::windows_core::Interface::vtable(this).MessageType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsMessageType>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn CellularClass(&self) -> ::windows_core::Result<CellularClass> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<CellularClass>::zeroed();
            (::windows_core::Interface::vtable(this).CellularClass)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CellularClass>(result__)
        }
    }
    pub fn MessageClass(&self) -> ::windows_core::Result<SmsMessageClass> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmsMessageClass>::zeroed();
            (::windows_core::Interface::vtable(this).MessageClass)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsMessageClass>(result__)
        }
    }
    pub fn SimIccId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SimIccId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<ISmsMessageBase> for ::windows_core::IUnknown {
    fn from(value: ISmsMessageBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISmsMessageBase> for ::windows_core::IUnknown {
    fn from(value: &ISmsMessageBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISmsMessageBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISmsMessageBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISmsMessageBase> for ::windows_core::IInspectable {
    fn from(value: ISmsMessageBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISmsMessageBase> for ::windows_core::IInspectable {
    fn from(value: &ISmsMessageBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ISmsMessageBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ISmsMessageBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISmsMessageBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISmsMessageBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISmsMessageBase {}
impl ::core::fmt::Debug for ISmsMessageBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISmsMessageBase").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ISmsMessageBase {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{2cf0fe30-fe50-4fc6-aa88-4ccfe27a29ea}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ISmsMessageBase {
    type Vtable = ISmsMessageBase_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2cf0fe30_fe50_4fc6_aa88_4ccfe27a29ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsMessageBase_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MessageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmsMessageType) -> ::windows_core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CellularClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CellularClass) -> ::windows_core::HRESULT,
    pub MessageClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmsMessageClass) -> ::windows_core::HRESULT,
    pub SimIccId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ISmsMessageReceivedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ISmsMessageReceivedEventArgs {
    type Vtable = ISmsMessageReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x08e80a98_b8e5_41c1_a3d8_d3abfae22675);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ISmsMessageReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub TextMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    TextMessage: usize,
    #[cfg(feature = "winrt-")]
    pub BinaryMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    BinaryMessage: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmsMessageReceivedTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmsMessageReceivedTriggerDetails {
    type Vtable = ISmsMessageReceivedTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2bcfcbd4_2657_4128_ad5f_e3877132bdb1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsMessageReceivedTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MessageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmsMessageType) -> ::windows_core::HRESULT,
    pub TextMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub WapMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AppMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub BroadcastMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub VoicemailMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StatusMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Drop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Accept: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmsMessageRegistration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmsMessageRegistration {
    type Vtable = ISmsMessageRegistration_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1720503e_f34f_446b_83b3_0ff19923b409);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsMessageRegistration_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Unregister: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub MessageReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveMessageReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmsMessageRegistrationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmsMessageRegistrationStatics {
    type Vtable = ISmsMessageRegistrationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x63a05464_2898_4778_a03c_6f994907d63a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsMessageRegistrationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub AllRegistrations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    AllRegistrations: usize,
    pub Register: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, filterrules: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ISmsReceivedEventDetails(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ISmsReceivedEventDetails {
    type Vtable = ISmsReceivedEventDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5bb50f15_e46d_4c82_847d_5a0304c1d53d);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ISmsReceivedEventDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    DeviceId: usize,
    #[cfg(feature = "winrt-")]
    pub MessageIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    MessageIndex: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ISmsReceivedEventDetails2(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ISmsReceivedEventDetails2 {
    type Vtable = ISmsReceivedEventDetails2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x40e05c86_a7b4_4771_9ae7_0b5ffb12c03a);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ISmsReceivedEventDetails2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub MessageClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmsMessageClass) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    MessageClass: usize,
    #[cfg(feature = "winrt-")]
    pub BinaryMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    BinaryMessage: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmsSendMessageResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmsSendMessageResult {
    type Vtable = ISmsSendMessageResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdb139af2_78c9_4feb_9622_452328088d62);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsSendMessageResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsSuccessful: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub MessageReferenceNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    MessageReferenceNumbers: usize,
    pub CellularClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CellularClass) -> ::windows_core::HRESULT,
    pub ModemErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmsModemErrorCode) -> ::windows_core::HRESULT,
    pub IsErrorTransient: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub NetworkCauseCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub TransportFailureCause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmsStatusMessage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmsStatusMessage {
    type Vtable = ISmsStatusMessage_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe6d28342_b70b_4677_9379_c9783fdff8f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsStatusMessage_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub To: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub From: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Body: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MessageReferenceNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub ServiceCenterTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub DischargeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ISmsTextMessage(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl ISmsTextMessage {
    #[cfg(feature = "winrt-")]
    pub fn Timestamp(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn PartReferenceId(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).PartReferenceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn PartNumber(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).PartNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn PartCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).PartCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn To(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).To)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetTo<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTo)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn From(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).From)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetFrom<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFrom)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn Body(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Body)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetBody<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBody)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn Encoding(&self) -> ::windows_core::Result<SmsEncoding> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmsEncoding>::zeroed();
            (::windows_core::Interface::vtable(this).Encoding)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsEncoding>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetEncoding(&self, value: SmsEncoding) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEncoding)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn ToBinaryMessages(&self, format: SmsDataFormat) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<ISmsBinaryMessage>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ToBinaryMessages)(::windows_core::Interface::as_raw(this), format, result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<ISmsBinaryMessage>>(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<ISmsMessage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn MessageClass(&self) -> ::windows_core::Result<SmsMessageClass> {
        let this = &::windows_core::Interface::cast::<ISmsMessage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmsMessageClass>::zeroed();
            (::windows_core::Interface::vtable(this).MessageClass)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsMessageClass>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<ISmsTextMessage> for ::windows_core::IUnknown {
    fn from(value: ISmsTextMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&ISmsTextMessage> for ::windows_core::IUnknown {
    fn from(value: &ISmsTextMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISmsTextMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISmsTextMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<ISmsTextMessage> for ::windows_core::IInspectable {
    fn from(value: ISmsTextMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&ISmsTextMessage> for ::windows_core::IInspectable {
    fn from(value: &ISmsTextMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ISmsTextMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ISmsTextMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<ISmsTextMessage> for ISmsMessage {
    type Error = ::windows_core::Error;
    fn try_from(value: ISmsTextMessage) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<&ISmsTextMessage> for ISmsMessage {
    type Error = ::windows_core::Error;
    fn try_from(value: &ISmsTextMessage) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ISmsMessage> for ISmsTextMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ISmsMessage> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ISmsMessage> for &ISmsTextMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ISmsMessage> {
        ::core::convert::TryInto::<ISmsMessage>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for ISmsTextMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for ISmsTextMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for ISmsTextMessage {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for ISmsTextMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISmsTextMessage").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for ISmsTextMessage {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{d61c904c-a495-487f-9a6f-971548c5bc9f}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ISmsTextMessage {
    type Vtable = ISmsTextMessage_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd61c904c_a495_487f_9a6f_971548c5bc9f);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ISmsTextMessage_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Timestamp: usize,
    #[cfg(feature = "winrt-")]
    pub PartReferenceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    PartReferenceId: usize,
    #[cfg(feature = "winrt-")]
    pub PartNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    PartNumber: usize,
    #[cfg(feature = "winrt-")]
    pub PartCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    PartCount: usize,
    #[cfg(feature = "winrt-")]
    pub To: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    To: usize,
    #[cfg(feature = "winrt-")]
    pub SetTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetTo: usize,
    #[cfg(feature = "winrt-")]
    pub From: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    From: usize,
    #[cfg(feature = "winrt-")]
    pub SetFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetFrom: usize,
    #[cfg(feature = "winrt-")]
    pub Body: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Body: usize,
    #[cfg(feature = "winrt-")]
    pub SetBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetBody: usize,
    #[cfg(feature = "winrt-")]
    pub Encoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmsEncoding) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Encoding: usize,
    #[cfg(feature = "winrt-")]
    pub SetEncoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SmsEncoding) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetEncoding: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub ToBinaryMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: SmsDataFormat, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    ToBinaryMessages: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmsTextMessage2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmsTextMessage2 {
    type Vtable = ISmsTextMessage2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x22a0d893_4555_4755_b5a1_e7fd84955f8d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsTextMessage2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub To: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub From: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Body: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Encoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmsEncoding) -> ::windows_core::HRESULT,
    pub SetEncoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SmsEncoding) -> ::windows_core::HRESULT,
    pub CallbackNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCallbackNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsDeliveryNotificationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsDeliveryNotificationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub RetryAttemptCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetRetryAttemptCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub TeleserviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub ProtocolId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ISmsTextMessageStatics(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ISmsTextMessageStatics {
    type Vtable = ISmsTextMessageStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7f68c5ed_3ccc_47a3_8c55_380d3b010892);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ISmsTextMessageStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub FromBinaryMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, binarymessage: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    FromBinaryMessage: usize,
    #[cfg(feature = "winrt-")]
    pub FromBinaryData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: SmsDataFormat, value_array_size: u32, value: *const u8, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    FromBinaryData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmsVoicemailMessage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmsVoicemailMessage {
    type Vtable = ISmsVoicemailMessage_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x271aa0a6_95b1_44ff_bcb8_b8fdd7e08bc3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsVoicemailMessage_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub To: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Body: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MessageCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmsWapMessage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmsWapMessage {
    type Vtable = ISmsWapMessage_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcd937743_7a55_4d3b_9021_f22e022d09c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsWapMessage_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub To: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub From: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ApplicationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub BinaryBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    BinaryBody: usize,
    #[cfg(feature = "winrt-foundation")]
    pub Headers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Headers: usize,
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct SendSmsMessageOperation(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl SendSmsMessageOperation {
    pub fn SetCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::AsyncActionCompletedHandler>>(&self, handler: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi()).ok() }
    }
    pub fn Completed(&self) -> ::windows_core::Result<::winrt_foundation::AsyncActionCompletedHandler> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Completed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::AsyncActionCompletedHandler>(result__)
        }
    }
    pub fn GetResults(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).GetResults)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<::winrt_foundation::AsyncStatus> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::AsyncStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::AsyncStatus>(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
    pub fn Cancel(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Cancel)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for SendSmsMessageOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for SendSmsMessageOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for SendSmsMessageOperation {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for SendSmsMessageOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SendSmsMessageOperation").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for SendSmsMessageOperation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SendSmsMessageOperation;{5a648006-843a-4da9-865b-9d26e5dfad7b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for SendSmsMessageOperation {
    type Vtable = ::winrt_foundation::IAsyncAction_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_foundation::IAsyncAction as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for SendSmsMessageOperation {
    const NAME: &'static str = "Windows.Devices.Sms.SendSmsMessageOperation";
}
#[cfg(feature = "winrt-")]
impl SendSmsMessageOperation {
    pub fn get(&self) -> ::windows_core::Result<()> {
        if self.Status()? == ::winrt_foundation::AsyncStatus::Started {
            let (_waiter, signaler) = ::windows_core::Waiter::new()?;
            self.SetCompleted(::winrt_foundation::AsyncActionCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
#[cfg(feature = "winrt-")]
impl ::std::future::Future for SendSmsMessageOperation {
    type Output = ::windows_core::Result<()>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == ::winrt_foundation::AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(::winrt_foundation::AsyncActionCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SendSmsMessageOperation> for ::windows_core::IUnknown {
    fn from(value: SendSmsMessageOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SendSmsMessageOperation> for ::windows_core::IUnknown {
    fn from(value: &SendSmsMessageOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SendSmsMessageOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SendSmsMessageOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SendSmsMessageOperation> for ::windows_core::IInspectable {
    fn from(value: SendSmsMessageOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SendSmsMessageOperation> for ::windows_core::IInspectable {
    fn from(value: &SendSmsMessageOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SendSmsMessageOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SendSmsMessageOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<SendSmsMessageOperation> for ::winrt_foundation::IAsyncAction {
    type Error = ::windows_core::Error;
    fn try_from(value: SendSmsMessageOperation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<&SendSmsMessageOperation> for ::winrt_foundation::IAsyncAction {
    type Error = ::windows_core::Error;
    fn try_from(value: &SendSmsMessageOperation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IAsyncAction> for SendSmsMessageOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IAsyncAction> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IAsyncAction> for &SendSmsMessageOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IAsyncAction> {
        ::core::convert::TryInto::<::winrt_foundation::IAsyncAction>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<SendSmsMessageOperation> for ::winrt_foundation::IAsyncInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: SendSmsMessageOperation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<&SendSmsMessageOperation> for ::winrt_foundation::IAsyncInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: &SendSmsMessageOperation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IAsyncInfo> for SendSmsMessageOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IAsyncInfo> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IAsyncInfo> for &SendSmsMessageOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IAsyncInfo> {
        ::core::convert::TryInto::<::winrt_foundation::IAsyncInfo>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[repr(transparent)]
pub struct SmsAppMessage(::windows_core::IUnknown);
impl SmsAppMessage {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SmsAppMessage, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Timestamp(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn To(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).To)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetTo<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTo)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn From(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).From)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Body(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Body)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetBody<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBody)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CallbackNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CallbackNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetCallbackNumber<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCallbackNumber)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IsDeliveryNotificationEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDeliveryNotificationEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsDeliveryNotificationEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsDeliveryNotificationEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RetryAttemptCount(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).RetryAttemptCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetRetryAttemptCount(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRetryAttemptCount)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Encoding(&self) -> ::windows_core::Result<SmsEncoding> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmsEncoding>::zeroed();
            (::windows_core::Interface::vtable(this).Encoding)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsEncoding>(result__)
        }
    }
    pub fn SetEncoding(&self, value: SmsEncoding) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEncoding)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PortNumber(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).PortNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetPortNumber(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPortNumber)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TeleserviceId(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).TeleserviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetTeleserviceId(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTeleserviceId)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ProtocolId(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).ProtocolId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetProtocolId(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetProtocolId)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn BinaryBody(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BinaryBody)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn SetBinaryBody<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBinaryBody)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn MessageType(&self) -> ::windows_core::Result<SmsMessageType> {
        let this = &::windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmsMessageType>::zeroed();
            (::windows_core::Interface::vtable(this).MessageType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsMessageType>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn CellularClass(&self) -> ::windows_core::Result<CellularClass> {
        let this = &::windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<CellularClass>::zeroed();
            (::windows_core::Interface::vtable(this).CellularClass)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CellularClass>(result__)
        }
    }
    pub fn MessageClass(&self) -> ::windows_core::Result<SmsMessageClass> {
        let this = &::windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmsMessageClass>::zeroed();
            (::windows_core::Interface::vtable(this).MessageClass)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsMessageClass>(result__)
        }
    }
    pub fn SimIccId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SimIccId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for SmsAppMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmsAppMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmsAppMessage {}
impl ::core::fmt::Debug for SmsAppMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsAppMessage").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmsAppMessage {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsAppMessage;{e8bb8494-d3a0-4a0a-86d7-291033a8cf54})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SmsAppMessage {
    type Vtable = ISmsAppMessage_Vtbl;
    const IID: ::windows_core::GUID = <ISmsAppMessage as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SmsAppMessage {
    const NAME: &'static str = "Windows.Devices.Sms.SmsAppMessage";
}
impl ::core::convert::From<SmsAppMessage> for ::windows_core::IUnknown {
    fn from(value: SmsAppMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmsAppMessage> for ::windows_core::IUnknown {
    fn from(value: &SmsAppMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmsAppMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmsAppMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmsAppMessage> for ::windows_core::IInspectable {
    fn from(value: SmsAppMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmsAppMessage> for ::windows_core::IInspectable {
    fn from(value: &SmsAppMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmsAppMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmsAppMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SmsAppMessage> for ISmsMessageBase {
    type Error = ::windows_core::Error;
    fn try_from(value: SmsAppMessage) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SmsAppMessage> for ISmsMessageBase {
    type Error = ::windows_core::Error;
    fn try_from(value: &SmsAppMessage) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISmsMessageBase> for SmsAppMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ISmsMessageBase> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISmsMessageBase> for &SmsAppMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ISmsMessageBase> {
        ::core::convert::TryInto::<ISmsMessageBase>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SmsAppMessage {}
unsafe impl ::core::marker::Sync for SmsAppMessage {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct SmsBinaryMessage(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl SmsBinaryMessage {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SmsBinaryMessage, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-")]
    pub fn Format(&self) -> ::windows_core::Result<SmsDataFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmsDataFormat>::zeroed();
            (::windows_core::Interface::vtable(this).Format)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsDataFormat>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetFormat(&self, value: SmsDataFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFormat)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn GetData(&self) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::Array<u8>>::zeroed();
            (::windows_core::Interface::vtable(this).GetData)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetData(&self, value: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetData)(::windows_core::Interface::as_raw(this), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<ISmsMessage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn MessageClass(&self) -> ::windows_core::Result<SmsMessageClass> {
        let this = &::windows_core::Interface::cast::<ISmsMessage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmsMessageClass>::zeroed();
            (::windows_core::Interface::vtable(this).MessageClass)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsMessageClass>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for SmsBinaryMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for SmsBinaryMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for SmsBinaryMessage {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for SmsBinaryMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsBinaryMessage").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for SmsBinaryMessage {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsBinaryMessage;{5bf4e813-3b53-4c6e-b61a-d86a63755650})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for SmsBinaryMessage {
    type Vtable = ISmsBinaryMessage_Vtbl;
    const IID: ::windows_core::GUID = <ISmsBinaryMessage as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for SmsBinaryMessage {
    const NAME: &'static str = "Windows.Devices.Sms.SmsBinaryMessage";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SmsBinaryMessage> for ::windows_core::IUnknown {
    fn from(value: SmsBinaryMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SmsBinaryMessage> for ::windows_core::IUnknown {
    fn from(value: &SmsBinaryMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmsBinaryMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmsBinaryMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SmsBinaryMessage> for ::windows_core::IInspectable {
    fn from(value: SmsBinaryMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SmsBinaryMessage> for ::windows_core::IInspectable {
    fn from(value: &SmsBinaryMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmsBinaryMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmsBinaryMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<SmsBinaryMessage> for ISmsBinaryMessage {
    type Error = ::windows_core::Error;
    fn try_from(value: SmsBinaryMessage) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<&SmsBinaryMessage> for ISmsBinaryMessage {
    type Error = ::windows_core::Error;
    fn try_from(value: &SmsBinaryMessage) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ISmsBinaryMessage> for SmsBinaryMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ISmsBinaryMessage> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ISmsBinaryMessage> for &SmsBinaryMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ISmsBinaryMessage> {
        ::core::convert::TryInto::<ISmsBinaryMessage>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<SmsBinaryMessage> for ISmsMessage {
    type Error = ::windows_core::Error;
    fn try_from(value: SmsBinaryMessage) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<&SmsBinaryMessage> for ISmsMessage {
    type Error = ::windows_core::Error;
    fn try_from(value: &SmsBinaryMessage) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ISmsMessage> for SmsBinaryMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ISmsMessage> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ISmsMessage> for &SmsBinaryMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ISmsMessage> {
        ::core::convert::TryInto::<ISmsMessage>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for SmsBinaryMessage {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for SmsBinaryMessage {}
#[repr(transparent)]
pub struct SmsBroadcastMessage(::windows_core::IUnknown);
impl SmsBroadcastMessage {
    pub fn Timestamp(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn To(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).To)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Body(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Body)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Channel(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Channel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn GeographicalScope(&self) -> ::windows_core::Result<SmsGeographicalScope> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmsGeographicalScope>::zeroed();
            (::windows_core::Interface::vtable(this).GeographicalScope)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsGeographicalScope>(result__)
        }
    }
    pub fn MessageCode(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).MessageCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn UpdateNumber(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).UpdateNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn BroadcastType(&self) -> ::windows_core::Result<SmsBroadcastType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmsBroadcastType>::zeroed();
            (::windows_core::Interface::vtable(this).BroadcastType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsBroadcastType>(result__)
        }
    }
    pub fn IsEmergencyAlert(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsEmergencyAlert)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsUserPopupRequested(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsUserPopupRequested)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn MessageType(&self) -> ::windows_core::Result<SmsMessageType> {
        let this = &::windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmsMessageType>::zeroed();
            (::windows_core::Interface::vtable(this).MessageType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsMessageType>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn CellularClass(&self) -> ::windows_core::Result<CellularClass> {
        let this = &::windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<CellularClass>::zeroed();
            (::windows_core::Interface::vtable(this).CellularClass)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CellularClass>(result__)
        }
    }
    pub fn MessageClass(&self) -> ::windows_core::Result<SmsMessageClass> {
        let this = &::windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmsMessageClass>::zeroed();
            (::windows_core::Interface::vtable(this).MessageClass)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsMessageClass>(result__)
        }
    }
    pub fn SimIccId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SimIccId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for SmsBroadcastMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmsBroadcastMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmsBroadcastMessage {}
impl ::core::fmt::Debug for SmsBroadcastMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsBroadcastMessage").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmsBroadcastMessage {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsBroadcastMessage;{75aebbf1-e4b7-4874-a09c-2956e592f957})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SmsBroadcastMessage {
    type Vtable = ISmsBroadcastMessage_Vtbl;
    const IID: ::windows_core::GUID = <ISmsBroadcastMessage as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SmsBroadcastMessage {
    const NAME: &'static str = "Windows.Devices.Sms.SmsBroadcastMessage";
}
impl ::core::convert::From<SmsBroadcastMessage> for ::windows_core::IUnknown {
    fn from(value: SmsBroadcastMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmsBroadcastMessage> for ::windows_core::IUnknown {
    fn from(value: &SmsBroadcastMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmsBroadcastMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmsBroadcastMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmsBroadcastMessage> for ::windows_core::IInspectable {
    fn from(value: SmsBroadcastMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmsBroadcastMessage> for ::windows_core::IInspectable {
    fn from(value: &SmsBroadcastMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmsBroadcastMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmsBroadcastMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SmsBroadcastMessage> for ISmsMessageBase {
    type Error = ::windows_core::Error;
    fn try_from(value: SmsBroadcastMessage) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SmsBroadcastMessage> for ISmsMessageBase {
    type Error = ::windows_core::Error;
    fn try_from(value: &SmsBroadcastMessage) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISmsMessageBase> for SmsBroadcastMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ISmsMessageBase> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISmsMessageBase> for &SmsBroadcastMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ISmsMessageBase> {
        ::core::convert::TryInto::<ISmsMessageBase>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SmsBroadcastMessage {}
unsafe impl ::core::marker::Sync for SmsBroadcastMessage {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SmsBroadcastType(pub i32);
impl SmsBroadcastType {
    pub const Other: Self = Self(0i32);
    pub const CmasPresidential: Self = Self(1i32);
    pub const CmasExtreme: Self = Self(2i32);
    pub const CmasSevere: Self = Self(3i32);
    pub const CmasAmber: Self = Self(4i32);
    pub const CmasTest: Self = Self(5i32);
    pub const EUAlert1: Self = Self(6i32);
    pub const EUAlert2: Self = Self(7i32);
    pub const EUAlert3: Self = Self(8i32);
    pub const EUAlertAmber: Self = Self(9i32);
    pub const EUAlertInfo: Self = Self(10i32);
    pub const EtwsEarthquake: Self = Self(11i32);
    pub const EtwsTsunami: Self = Self(12i32);
    pub const EtwsTsunamiAndEarthquake: Self = Self(13i32);
    pub const LatAlertLocal: Self = Self(14i32);
}
impl ::core::marker::Copy for SmsBroadcastType {}
impl ::core::clone::Clone for SmsBroadcastType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmsBroadcastType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SmsBroadcastType {
    type Abi = Self;
}
impl ::core::fmt::Debug for SmsBroadcastType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsBroadcastType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmsBroadcastType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsBroadcastType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SmsDataFormat(pub i32);
impl SmsDataFormat {
    pub const Unknown: Self = Self(0i32);
    pub const CdmaSubmit: Self = Self(1i32);
    pub const GsmSubmit: Self = Self(2i32);
    pub const CdmaDeliver: Self = Self(3i32);
    pub const GsmDeliver: Self = Self(4i32);
}
impl ::core::marker::Copy for SmsDataFormat {}
impl ::core::clone::Clone for SmsDataFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmsDataFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SmsDataFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for SmsDataFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsDataFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmsDataFormat {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsDataFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct SmsDevice(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl SmsDevice {
    #[cfg(feature = "winrt-")]
    pub fn SendMessageAsync<'a, Param0: ::windows_core::IntoParam<'a, ISmsMessage>>(&self, message: Param0) -> ::windows_core::Result<SendSmsMessageOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SendMessageAsync)(::windows_core::Interface::as_raw(this), message.into_param().abi(), result__.as_mut_ptr()).from_abi::<SendSmsMessageOperation>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn CalculateLength<'a, Param0: ::windows_core::IntoParam<'a, SmsTextMessage>>(&self, message: Param0) -> ::windows_core::Result<SmsEncodedLength> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmsEncodedLength>::zeroed();
            (::windows_core::Interface::vtable(this).CalculateLength)(::windows_core::Interface::as_raw(this), message.into_param().abi(), result__.as_mut_ptr()).from_abi::<SmsEncodedLength>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn AccountPhoneNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AccountPhoneNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn CellularClass(&self) -> ::windows_core::Result<CellularClass> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<CellularClass>::zeroed();
            (::windows_core::Interface::vtable(this).CellularClass)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CellularClass>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn MessageStore(&self) -> ::windows_core::Result<SmsDeviceMessageStore> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MessageStore)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsDeviceMessageStore>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn DeviceStatus(&self) -> ::windows_core::Result<SmsDeviceStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmsDeviceStatus>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsDeviceStatus>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SmsMessageReceived<'a, Param0: ::windows_core::IntoParam<'a, SmsMessageReceivedEventHandler>>(&self, eventhandler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SmsMessageReceived)(::windows_core::Interface::as_raw(this), eventhandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveSmsMessageReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSmsMessageReceived)(::windows_core::Interface::as_raw(this), eventcookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn SmsDeviceStatusChanged<'a, Param0: ::windows_core::IntoParam<'a, SmsDeviceStatusChangedEventHandler>>(&self, eventhandler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SmsDeviceStatusChanged)(::windows_core::Interface::as_raw(this), eventhandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveSmsDeviceStatusChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSmsDeviceStatusChanged)(::windows_core::Interface::as_raw(this), eventcookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISmsDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SmsDevice>> {
        Self::ISmsDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SmsDevice>>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn GetDefaultAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SmsDevice>> {
        Self::ISmsDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefaultAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SmsDevice>>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn FromNetworkAccountIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(networkaccountid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SmsDevice>> {
        Self::ISmsDeviceStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromNetworkAccountIdAsync)(::windows_core::Interface::as_raw(this), networkaccountid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SmsDevice>>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn ISmsDeviceStatics<R, F: FnOnce(&ISmsDeviceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SmsDevice, ISmsDeviceStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-")]
    pub fn ISmsDeviceStatics2<R, F: FnOnce(&ISmsDeviceStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SmsDevice, ISmsDeviceStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for SmsDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for SmsDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for SmsDevice {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for SmsDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsDevice").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for SmsDevice {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsDevice;{091791ed-872b-4eec-9c72-ab11627b34ec})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for SmsDevice {
    type Vtable = ISmsDevice_Vtbl;
    const IID: ::windows_core::GUID = <ISmsDevice as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for SmsDevice {
    const NAME: &'static str = "Windows.Devices.Sms.SmsDevice";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SmsDevice> for ::windows_core::IUnknown {
    fn from(value: SmsDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SmsDevice> for ::windows_core::IUnknown {
    fn from(value: &SmsDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmsDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmsDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SmsDevice> for ::windows_core::IInspectable {
    fn from(value: SmsDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SmsDevice> for ::windows_core::IInspectable {
    fn from(value: &SmsDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmsDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmsDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<SmsDevice> for ISmsDevice {
    type Error = ::windows_core::Error;
    fn try_from(value: SmsDevice) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<&SmsDevice> for ISmsDevice {
    type Error = ::windows_core::Error;
    fn try_from(value: &SmsDevice) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ISmsDevice> for SmsDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ISmsDevice> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ISmsDevice> for &SmsDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ISmsDevice> {
        ::core::convert::TryInto::<ISmsDevice>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[repr(transparent)]
pub struct SmsDevice2(::windows_core::IUnknown);
impl SmsDevice2 {
    pub fn SmscAddress(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SmscAddress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetSmscAddress<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSmscAddress)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ParentDeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ParentDeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AccountPhoneNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AccountPhoneNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn CellularClass(&self) -> ::windows_core::Result<CellularClass> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<CellularClass>::zeroed();
            (::windows_core::Interface::vtable(this).CellularClass)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CellularClass>(result__)
        }
    }
    pub fn DeviceStatus(&self) -> ::windows_core::Result<SmsDeviceStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmsDeviceStatus>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsDeviceStatus>(result__)
        }
    }
    pub fn CalculateLength<'a, Param0: ::windows_core::IntoParam<'a, ISmsMessageBase>>(&self, message: Param0) -> ::windows_core::Result<SmsEncodedLength> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmsEncodedLength>::zeroed();
            (::windows_core::Interface::vtable(this).CalculateLength)(::windows_core::Interface::as_raw(this), message.into_param().abi(), result__.as_mut_ptr()).from_abi::<SmsEncodedLength>(result__)
        }
    }
    pub fn SendMessageAndGetResultAsync<'a, Param0: ::windows_core::IntoParam<'a, ISmsMessageBase>>(&self, message: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SmsSendMessageResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SendMessageAndGetResultAsync)(::windows_core::Interface::as_raw(this), message.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SmsSendMessageResult>>(result__)
        }
    }
    pub fn DeviceStatusChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<SmsDevice2, ::windows_core::IInspectable>>>(&self, eventhandler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceStatusChanged)(::windows_core::Interface::as_raw(this), eventhandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDeviceStatusChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDeviceStatusChanged)(::windows_core::Interface::as_raw(this), eventcookie.into_param().abi()).ok() }
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISmsDevice2Statics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn FromId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0) -> ::windows_core::Result<SmsDevice2> {
        Self::ISmsDevice2Statics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromId)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<SmsDevice2>(result__)
        })
    }
    pub fn GetDefault() -> ::windows_core::Result<SmsDevice2> {
        Self::ISmsDevice2Statics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsDevice2>(result__)
        })
    }
    pub fn FromParentId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(parentdeviceid: Param0) -> ::windows_core::Result<SmsDevice2> {
        Self::ISmsDevice2Statics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromParentId)(::windows_core::Interface::as_raw(this), parentdeviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<SmsDevice2>(result__)
        })
    }
    pub fn ISmsDevice2Statics<R, F: FnOnce(&ISmsDevice2Statics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SmsDevice2, ISmsDevice2Statics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SmsDevice2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmsDevice2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmsDevice2 {}
impl ::core::fmt::Debug for SmsDevice2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsDevice2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmsDevice2 {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsDevice2;{bd8a5c13-e522-46cb-b8d5-9ead30fb6c47})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SmsDevice2 {
    type Vtable = ISmsDevice2_Vtbl;
    const IID: ::windows_core::GUID = <ISmsDevice2 as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SmsDevice2 {
    const NAME: &'static str = "Windows.Devices.Sms.SmsDevice2";
}
impl ::core::convert::From<SmsDevice2> for ::windows_core::IUnknown {
    fn from(value: SmsDevice2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmsDevice2> for ::windows_core::IUnknown {
    fn from(value: &SmsDevice2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmsDevice2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmsDevice2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmsDevice2> for ::windows_core::IInspectable {
    fn from(value: SmsDevice2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmsDevice2> for ::windows_core::IInspectable {
    fn from(value: &SmsDevice2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmsDevice2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmsDevice2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct SmsDeviceMessageStore(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl SmsDeviceMessageStore {
    #[cfg(feature = "winrt-")]
    pub fn DeleteMessageAsync(&self, messageid: u32) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeleteMessageAsync)(::windows_core::Interface::as_raw(this), messageid, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn DeleteMessagesAsync(&self, messagefilter: SmsMessageFilter) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeleteMessagesAsync)(::windows_core::Interface::as_raw(this), messagefilter, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn GetMessageAsync(&self, messageid: u32) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ISmsMessage>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetMessageAsync)(::windows_core::Interface::as_raw(this), messageid, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ISmsMessage>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn GetMessagesAsync(&self, messagefilter: SmsMessageFilter) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<::winrt_foundation::Collections::IVectorView<ISmsMessage>, i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetMessagesAsync)(::windows_core::Interface::as_raw(this), messagefilter, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<::winrt_foundation::Collections::IVectorView<ISmsMessage>, i32>>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn MaxMessages(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxMessages)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for SmsDeviceMessageStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for SmsDeviceMessageStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for SmsDeviceMessageStore {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for SmsDeviceMessageStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsDeviceMessageStore").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for SmsDeviceMessageStore {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsDeviceMessageStore;{9889f253-f188-4427-8d54-ce0c2423c5c1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for SmsDeviceMessageStore {
    type Vtable = ISmsDeviceMessageStore_Vtbl;
    const IID: ::windows_core::GUID = <ISmsDeviceMessageStore as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for SmsDeviceMessageStore {
    const NAME: &'static str = "Windows.Devices.Sms.SmsDeviceMessageStore";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SmsDeviceMessageStore> for ::windows_core::IUnknown {
    fn from(value: SmsDeviceMessageStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SmsDeviceMessageStore> for ::windows_core::IUnknown {
    fn from(value: &SmsDeviceMessageStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmsDeviceMessageStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmsDeviceMessageStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SmsDeviceMessageStore> for ::windows_core::IInspectable {
    fn from(value: SmsDeviceMessageStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SmsDeviceMessageStore> for ::windows_core::IInspectable {
    fn from(value: &SmsDeviceMessageStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmsDeviceMessageStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmsDeviceMessageStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SmsDeviceStatus(pub i32);
impl SmsDeviceStatus {
    pub const Off: Self = Self(0i32);
    pub const Ready: Self = Self(1i32);
    pub const SimNotInserted: Self = Self(2i32);
    pub const BadSim: Self = Self(3i32);
    pub const DeviceFailure: Self = Self(4i32);
    pub const SubscriptionNotActivated: Self = Self(5i32);
    pub const DeviceLocked: Self = Self(6i32);
    pub const DeviceBlocked: Self = Self(7i32);
}
impl ::core::marker::Copy for SmsDeviceStatus {}
impl ::core::clone::Clone for SmsDeviceStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmsDeviceStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SmsDeviceStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for SmsDeviceStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsDeviceStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmsDeviceStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsDeviceStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct SmsDeviceStatusChangedEventHandler(pub ::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl SmsDeviceStatusChangedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<SmsDevice>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = SmsDeviceStatusChangedEventHandlerBox::<F> { vtable: &SmsDeviceStatusChangedEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    #[cfg(feature = "winrt-")]
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, SmsDevice>>(&self, sender: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi()).ok() }
    }
}
#[cfg(feature = "winrt-")]
#[repr(C)]
struct SmsDeviceStatusChangedEventHandlerBox<F: FnMut(&::core::option::Option<SmsDevice>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const SmsDeviceStatusChangedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
#[cfg(feature = "winrt-")]
impl<F: FnMut(&::core::option::Option<SmsDevice>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> SmsDeviceStatusChangedEventHandlerBox<F> {
    const VTABLE: SmsDeviceStatusChangedEventHandler_Vtbl = SmsDeviceStatusChangedEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<SmsDeviceStatusChangedEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender)).into()
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for SmsDeviceStatusChangedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for SmsDeviceStatusChangedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for SmsDeviceStatusChangedEventHandler {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for SmsDeviceStatusChangedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsDeviceStatusChangedEventHandler").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for SmsDeviceStatusChangedEventHandler {
    type Vtable = SmsDeviceStatusChangedEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x982b1162_3dd7_4618_af89_0c272d5d06d8);
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for SmsDeviceStatusChangedEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{982b1162-3dd7-4618-af89-0c272d5d06d8}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct SmsDeviceStatusChangedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "winrt-")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Invoke: usize,
}
#[repr(C)]
pub struct SmsEncodedLength {
    pub SegmentCount: u32,
    pub CharacterCountLastSegment: u32,
    pub CharactersPerSegment: u32,
    pub ByteCountLastSegment: u32,
    pub BytesPerSegment: u32,
}
impl ::core::marker::Copy for SmsEncodedLength {}
impl ::core::clone::Clone for SmsEncodedLength {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SmsEncodedLength {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SmsEncodedLength").field("SegmentCount", &self.SegmentCount).field("CharacterCountLastSegment", &self.CharacterCountLastSegment).field("CharactersPerSegment", &self.CharactersPerSegment).field("ByteCountLastSegment", &self.ByteCountLastSegment).field("BytesPerSegment", &self.BytesPerSegment).finish()
    }
}
unsafe impl ::windows_core::Abi for SmsEncodedLength {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for SmsEncodedLength {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Devices.Sms.SmsEncodedLength;u4;u4;u4;u4;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for SmsEncodedLength {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SmsEncodedLength>()) == 0 }
    }
}
impl ::core::cmp::Eq for SmsEncodedLength {}
impl ::core::default::Default for SmsEncodedLength {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SmsEncoding(pub i32);
impl SmsEncoding {
    pub const Unknown: Self = Self(0i32);
    pub const Optimal: Self = Self(1i32);
    pub const SevenBitAscii: Self = Self(2i32);
    pub const Unicode: Self = Self(3i32);
    pub const GsmSevenBit: Self = Self(4i32);
    pub const EightBit: Self = Self(5i32);
    pub const Latin: Self = Self(6i32);
    pub const Korean: Self = Self(7i32);
    pub const IA5: Self = Self(8i32);
    pub const ShiftJis: Self = Self(9i32);
    pub const LatinHebrew: Self = Self(10i32);
}
impl ::core::marker::Copy for SmsEncoding {}
impl ::core::clone::Clone for SmsEncoding {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmsEncoding {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SmsEncoding {
    type Abi = Self;
}
impl ::core::fmt::Debug for SmsEncoding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsEncoding").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmsEncoding {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsEncoding;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SmsFilterActionType(pub i32);
impl SmsFilterActionType {
    pub const AcceptImmediately: Self = Self(0i32);
    pub const Drop: Self = Self(1i32);
    pub const Peek: Self = Self(2i32);
    pub const Accept: Self = Self(3i32);
}
impl ::core::marker::Copy for SmsFilterActionType {}
impl ::core::clone::Clone for SmsFilterActionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmsFilterActionType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SmsFilterActionType {
    type Abi = Self;
}
impl ::core::fmt::Debug for SmsFilterActionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsFilterActionType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmsFilterActionType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsFilterActionType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SmsFilterRule(::windows_core::IUnknown);
impl SmsFilterRule {
    pub fn MessageType(&self) -> ::windows_core::Result<SmsMessageType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmsMessageType>::zeroed();
            (::windows_core::Interface::vtable(this).MessageType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsMessageType>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ImsiPrefixes(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImsiPrefixes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn DeviceIds(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceIds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SenderNumbers(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SenderNumbers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn TextMessagePrefixes(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TextMessagePrefixes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn PortNumbers(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PortNumbers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<i32>>(result__)
        }
    }
    pub fn CellularClass(&self) -> ::windows_core::Result<CellularClass> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<CellularClass>::zeroed();
            (::windows_core::Interface::vtable(this).CellularClass)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CellularClass>(result__)
        }
    }
    pub fn SetCellularClass(&self, value: CellularClass) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCellularClass)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ProtocolIds(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProtocolIds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<i32>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn TeleserviceIds(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TeleserviceIds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<i32>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn WapApplicationIds(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).WapApplicationIds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn WapContentTypes(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).WapContentTypes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn BroadcastTypes(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<SmsBroadcastType>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BroadcastTypes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<SmsBroadcastType>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn BroadcastChannels(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BroadcastChannels)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<i32>>(result__)
        }
    }
    pub fn CreateFilterRule(messagetype: SmsMessageType) -> ::windows_core::Result<SmsFilterRule> {
        Self::ISmsFilterRuleFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFilterRule)(::windows_core::Interface::as_raw(this), messagetype, result__.as_mut_ptr()).from_abi::<SmsFilterRule>(result__)
        })
    }
    pub fn ISmsFilterRuleFactory<R, F: FnOnce(&ISmsFilterRuleFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SmsFilterRule, ISmsFilterRuleFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SmsFilterRule {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmsFilterRule {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmsFilterRule {}
impl ::core::fmt::Debug for SmsFilterRule {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsFilterRule").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmsFilterRule {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsFilterRule;{40e32fae-b049-4fbc-afe9-e2a610eff55c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SmsFilterRule {
    type Vtable = ISmsFilterRule_Vtbl;
    const IID: ::windows_core::GUID = <ISmsFilterRule as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SmsFilterRule {
    const NAME: &'static str = "Windows.Devices.Sms.SmsFilterRule";
}
impl ::core::convert::From<SmsFilterRule> for ::windows_core::IUnknown {
    fn from(value: SmsFilterRule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmsFilterRule> for ::windows_core::IUnknown {
    fn from(value: &SmsFilterRule) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmsFilterRule {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmsFilterRule {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmsFilterRule> for ::windows_core::IInspectable {
    fn from(value: SmsFilterRule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmsFilterRule> for ::windows_core::IInspectable {
    fn from(value: &SmsFilterRule) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmsFilterRule {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmsFilterRule {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SmsFilterRule {}
unsafe impl ::core::marker::Sync for SmsFilterRule {}
#[repr(transparent)]
pub struct SmsFilterRules(::windows_core::IUnknown);
impl SmsFilterRules {
    pub fn ActionType(&self) -> ::windows_core::Result<SmsFilterActionType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmsFilterActionType>::zeroed();
            (::windows_core::Interface::vtable(this).ActionType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsFilterActionType>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Rules(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<SmsFilterRule>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Rules)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<SmsFilterRule>>(result__)
        }
    }
    pub fn CreateFilterRules(actiontype: SmsFilterActionType) -> ::windows_core::Result<SmsFilterRules> {
        Self::ISmsFilterRulesFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFilterRules)(::windows_core::Interface::as_raw(this), actiontype, result__.as_mut_ptr()).from_abi::<SmsFilterRules>(result__)
        })
    }
    pub fn ISmsFilterRulesFactory<R, F: FnOnce(&ISmsFilterRulesFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SmsFilterRules, ISmsFilterRulesFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SmsFilterRules {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmsFilterRules {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmsFilterRules {}
impl ::core::fmt::Debug for SmsFilterRules {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsFilterRules").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmsFilterRules {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsFilterRules;{4e47eafb-79cd-4881-9894-55a4135b23fa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SmsFilterRules {
    type Vtable = ISmsFilterRules_Vtbl;
    const IID: ::windows_core::GUID = <ISmsFilterRules as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SmsFilterRules {
    const NAME: &'static str = "Windows.Devices.Sms.SmsFilterRules";
}
impl ::core::convert::From<SmsFilterRules> for ::windows_core::IUnknown {
    fn from(value: SmsFilterRules) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmsFilterRules> for ::windows_core::IUnknown {
    fn from(value: &SmsFilterRules) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmsFilterRules {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmsFilterRules {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmsFilterRules> for ::windows_core::IInspectable {
    fn from(value: SmsFilterRules) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmsFilterRules> for ::windows_core::IInspectable {
    fn from(value: &SmsFilterRules) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmsFilterRules {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmsFilterRules {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SmsFilterRules {}
unsafe impl ::core::marker::Sync for SmsFilterRules {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SmsGeographicalScope(pub i32);
impl SmsGeographicalScope {
    pub const None: Self = Self(0i32);
    pub const CellWithImmediateDisplay: Self = Self(1i32);
    pub const LocationArea: Self = Self(2i32);
    pub const Plmn: Self = Self(3i32);
    pub const Cell: Self = Self(4i32);
}
impl ::core::marker::Copy for SmsGeographicalScope {}
impl ::core::clone::Clone for SmsGeographicalScope {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmsGeographicalScope {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SmsGeographicalScope {
    type Abi = Self;
}
impl ::core::fmt::Debug for SmsGeographicalScope {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsGeographicalScope").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmsGeographicalScope {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsGeographicalScope;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SmsMessageClass(pub i32);
impl SmsMessageClass {
    pub const None: Self = Self(0i32);
    pub const Class0: Self = Self(1i32);
    pub const Class1: Self = Self(2i32);
    pub const Class2: Self = Self(3i32);
    pub const Class3: Self = Self(4i32);
}
impl ::core::marker::Copy for SmsMessageClass {}
impl ::core::clone::Clone for SmsMessageClass {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmsMessageClass {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SmsMessageClass {
    type Abi = Self;
}
impl ::core::fmt::Debug for SmsMessageClass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsMessageClass").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmsMessageClass {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsMessageClass;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SmsMessageFilter(pub i32);
#[cfg(feature = "winrt-")]
impl SmsMessageFilter {
    pub const All: Self = Self(0i32);
    pub const Unread: Self = Self(1i32);
    pub const Read: Self = Self(2i32);
    pub const Sent: Self = Self(3i32);
    pub const Draft: Self = Self(4i32);
}
#[cfg(feature = "winrt-")]
impl ::core::marker::Copy for SmsMessageFilter {}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for SmsMessageFilter {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "winrt-")]
impl ::core::default::Default for SmsMessageFilter {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Abi for SmsMessageFilter {
    type Abi = Self;
}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for SmsMessageFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsMessageFilter").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for SmsMessageFilter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsMessageFilter;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct SmsMessageReceivedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl SmsMessageReceivedEventArgs {
    #[cfg(feature = "winrt-")]
    pub fn TextMessage(&self) -> ::windows_core::Result<SmsTextMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TextMessage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsTextMessage>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn BinaryMessage(&self) -> ::windows_core::Result<SmsBinaryMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BinaryMessage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsBinaryMessage>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for SmsMessageReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for SmsMessageReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for SmsMessageReceivedEventArgs {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for SmsMessageReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsMessageReceivedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for SmsMessageReceivedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsMessageReceivedEventArgs;{08e80a98-b8e5-41c1-a3d8-d3abfae22675})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for SmsMessageReceivedEventArgs {
    type Vtable = ISmsMessageReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISmsMessageReceivedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for SmsMessageReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sms.SmsMessageReceivedEventArgs";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SmsMessageReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: SmsMessageReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SmsMessageReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SmsMessageReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmsMessageReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmsMessageReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SmsMessageReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: SmsMessageReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SmsMessageReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SmsMessageReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmsMessageReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmsMessageReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct SmsMessageReceivedEventHandler(pub ::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl SmsMessageReceivedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<SmsDevice>, &::core::option::Option<SmsMessageReceivedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = SmsMessageReceivedEventHandlerBox::<F> { vtable: &SmsMessageReceivedEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    #[cfg(feature = "winrt-")]
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, SmsDevice>, Param1: ::windows_core::IntoParam<'a, SmsMessageReceivedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[cfg(feature = "winrt-")]
#[repr(C)]
struct SmsMessageReceivedEventHandlerBox<F: FnMut(&::core::option::Option<SmsDevice>, &::core::option::Option<SmsMessageReceivedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const SmsMessageReceivedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
#[cfg(feature = "winrt-")]
impl<F: FnMut(&::core::option::Option<SmsDevice>, &::core::option::Option<SmsMessageReceivedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> SmsMessageReceivedEventHandlerBox<F> {
    const VTABLE: SmsMessageReceivedEventHandler_Vtbl = SmsMessageReceivedEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<SmsMessageReceivedEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: ::windows_core::RawPtr, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for SmsMessageReceivedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for SmsMessageReceivedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for SmsMessageReceivedEventHandler {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for SmsMessageReceivedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsMessageReceivedEventHandler").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for SmsMessageReceivedEventHandler {
    type Vtable = SmsMessageReceivedEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0b7ad409_ec2d_47ce_a253_732beeebcacd);
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for SmsMessageReceivedEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{0b7ad409-ec2d-47ce-a253-732beeebcacd}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct SmsMessageReceivedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "winrt-")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: ::windows_core::RawPtr, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Invoke: usize,
}
#[repr(transparent)]
pub struct SmsMessageReceivedTriggerDetails(::windows_core::IUnknown);
impl SmsMessageReceivedTriggerDetails {
    pub fn MessageType(&self) -> ::windows_core::Result<SmsMessageType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmsMessageType>::zeroed();
            (::windows_core::Interface::vtable(this).MessageType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsMessageType>(result__)
        }
    }
    pub fn TextMessage(&self) -> ::windows_core::Result<SmsTextMessage2> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TextMessage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsTextMessage2>(result__)
        }
    }
    pub fn WapMessage(&self) -> ::windows_core::Result<SmsWapMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).WapMessage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsWapMessage>(result__)
        }
    }
    pub fn AppMessage(&self) -> ::windows_core::Result<SmsAppMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppMessage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsAppMessage>(result__)
        }
    }
    pub fn BroadcastMessage(&self) -> ::windows_core::Result<SmsBroadcastMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BroadcastMessage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsBroadcastMessage>(result__)
        }
    }
    pub fn VoicemailMessage(&self) -> ::windows_core::Result<SmsVoicemailMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VoicemailMessage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsVoicemailMessage>(result__)
        }
    }
    pub fn StatusMessage(&self) -> ::windows_core::Result<SmsStatusMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StatusMessage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsStatusMessage>(result__)
        }
    }
    pub fn Drop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Drop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Accept(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Accept)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for SmsMessageReceivedTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmsMessageReceivedTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmsMessageReceivedTriggerDetails {}
impl ::core::fmt::Debug for SmsMessageReceivedTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsMessageReceivedTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmsMessageReceivedTriggerDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsMessageReceivedTriggerDetails;{2bcfcbd4-2657-4128-ad5f-e3877132bdb1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SmsMessageReceivedTriggerDetails {
    type Vtable = ISmsMessageReceivedTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = <ISmsMessageReceivedTriggerDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SmsMessageReceivedTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Sms.SmsMessageReceivedTriggerDetails";
}
impl ::core::convert::From<SmsMessageReceivedTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: SmsMessageReceivedTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmsMessageReceivedTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: &SmsMessageReceivedTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmsMessageReceivedTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmsMessageReceivedTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmsMessageReceivedTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: SmsMessageReceivedTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmsMessageReceivedTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: &SmsMessageReceivedTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmsMessageReceivedTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmsMessageReceivedTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SmsMessageReceivedTriggerDetails {}
unsafe impl ::core::marker::Sync for SmsMessageReceivedTriggerDetails {}
#[repr(transparent)]
pub struct SmsMessageRegistration(::windows_core::IUnknown);
impl SmsMessageRegistration {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Unregister(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Unregister)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn MessageReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<SmsMessageRegistration, SmsMessageReceivedTriggerDetails>>>(&self, eventhandler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).MessageReceived)(::windows_core::Interface::as_raw(this), eventhandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveMessageReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveMessageReceived)(::windows_core::Interface::as_raw(this), eventcookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn AllRegistrations() -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<SmsMessageRegistration>> {
        Self::ISmsMessageRegistrationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AllRegistrations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<SmsMessageRegistration>>(result__)
        })
    }
    pub fn Register<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, SmsFilterRules>>(id: Param0, filterrules: Param1) -> ::windows_core::Result<SmsMessageRegistration> {
        Self::ISmsMessageRegistrationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Register)(::windows_core::Interface::as_raw(this), id.into_param().abi(), filterrules.into_param().abi(), result__.as_mut_ptr()).from_abi::<SmsMessageRegistration>(result__)
        })
    }
    pub fn ISmsMessageRegistrationStatics<R, F: FnOnce(&ISmsMessageRegistrationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SmsMessageRegistration, ISmsMessageRegistrationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SmsMessageRegistration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmsMessageRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmsMessageRegistration {}
impl ::core::fmt::Debug for SmsMessageRegistration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsMessageRegistration").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmsMessageRegistration {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsMessageRegistration;{1720503e-f34f-446b-83b3-0ff19923b409})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SmsMessageRegistration {
    type Vtable = ISmsMessageRegistration_Vtbl;
    const IID: ::windows_core::GUID = <ISmsMessageRegistration as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SmsMessageRegistration {
    const NAME: &'static str = "Windows.Devices.Sms.SmsMessageRegistration";
}
impl ::core::convert::From<SmsMessageRegistration> for ::windows_core::IUnknown {
    fn from(value: SmsMessageRegistration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmsMessageRegistration> for ::windows_core::IUnknown {
    fn from(value: &SmsMessageRegistration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmsMessageRegistration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmsMessageRegistration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmsMessageRegistration> for ::windows_core::IInspectable {
    fn from(value: SmsMessageRegistration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmsMessageRegistration> for ::windows_core::IInspectable {
    fn from(value: &SmsMessageRegistration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmsMessageRegistration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmsMessageRegistration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SmsMessageType(pub i32);
impl SmsMessageType {
    pub const Binary: Self = Self(0i32);
    pub const Text: Self = Self(1i32);
    pub const Wap: Self = Self(2i32);
    pub const App: Self = Self(3i32);
    pub const Broadcast: Self = Self(4i32);
    pub const Voicemail: Self = Self(5i32);
    pub const Status: Self = Self(6i32);
}
impl ::core::marker::Copy for SmsMessageType {}
impl ::core::clone::Clone for SmsMessageType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmsMessageType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SmsMessageType {
    type Abi = Self;
}
impl ::core::fmt::Debug for SmsMessageType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsMessageType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmsMessageType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsMessageType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SmsModemErrorCode(pub i32);
impl SmsModemErrorCode {
    pub const Other: Self = Self(0i32);
    pub const MessagingNetworkError: Self = Self(1i32);
    pub const SmsOperationNotSupportedByDevice: Self = Self(2i32);
    pub const SmsServiceNotSupportedByNetwork: Self = Self(3i32);
    pub const DeviceFailure: Self = Self(4i32);
    pub const MessageNotEncodedProperly: Self = Self(5i32);
    pub const MessageTooLarge: Self = Self(6i32);
    pub const DeviceNotReady: Self = Self(7i32);
    pub const NetworkNotReady: Self = Self(8i32);
    pub const InvalidSmscAddress: Self = Self(9i32);
    pub const NetworkFailure: Self = Self(10i32);
    pub const FixedDialingNumberRestricted: Self = Self(11i32);
}
impl ::core::marker::Copy for SmsModemErrorCode {}
impl ::core::clone::Clone for SmsModemErrorCode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmsModemErrorCode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SmsModemErrorCode {
    type Abi = Self;
}
impl ::core::fmt::Debug for SmsModemErrorCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsModemErrorCode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmsModemErrorCode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsModemErrorCode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct SmsReceivedEventDetails(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl SmsReceivedEventDetails {
    #[cfg(feature = "winrt-")]
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn MessageIndex(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MessageIndex)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn MessageClass(&self) -> ::windows_core::Result<SmsMessageClass> {
        let this = &::windows_core::Interface::cast::<ISmsReceivedEventDetails2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmsMessageClass>::zeroed();
            (::windows_core::Interface::vtable(this).MessageClass)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsMessageClass>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn BinaryMessage(&self) -> ::windows_core::Result<SmsBinaryMessage> {
        let this = &::windows_core::Interface::cast::<ISmsReceivedEventDetails2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BinaryMessage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsBinaryMessage>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for SmsReceivedEventDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for SmsReceivedEventDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for SmsReceivedEventDetails {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for SmsReceivedEventDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsReceivedEventDetails").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for SmsReceivedEventDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsReceivedEventDetails;{5bb50f15-e46d-4c82-847d-5a0304c1d53d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for SmsReceivedEventDetails {
    type Vtable = ISmsReceivedEventDetails_Vtbl;
    const IID: ::windows_core::GUID = <ISmsReceivedEventDetails as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for SmsReceivedEventDetails {
    const NAME: &'static str = "Windows.Devices.Sms.SmsReceivedEventDetails";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SmsReceivedEventDetails> for ::windows_core::IUnknown {
    fn from(value: SmsReceivedEventDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SmsReceivedEventDetails> for ::windows_core::IUnknown {
    fn from(value: &SmsReceivedEventDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmsReceivedEventDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmsReceivedEventDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SmsReceivedEventDetails> for ::windows_core::IInspectable {
    fn from(value: SmsReceivedEventDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SmsReceivedEventDetails> for ::windows_core::IInspectable {
    fn from(value: &SmsReceivedEventDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmsReceivedEventDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmsReceivedEventDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for SmsReceivedEventDetails {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for SmsReceivedEventDetails {}
#[repr(transparent)]
pub struct SmsSendMessageResult(::windows_core::IUnknown);
impl SmsSendMessageResult {
    pub fn IsSuccessful(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSuccessful)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn MessageReferenceNumbers(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MessageReferenceNumbers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<i32>>(result__)
        }
    }
    pub fn CellularClass(&self) -> ::windows_core::Result<CellularClass> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<CellularClass>::zeroed();
            (::windows_core::Interface::vtable(this).CellularClass)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CellularClass>(result__)
        }
    }
    pub fn ModemErrorCode(&self) -> ::windows_core::Result<SmsModemErrorCode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmsModemErrorCode>::zeroed();
            (::windows_core::Interface::vtable(this).ModemErrorCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsModemErrorCode>(result__)
        }
    }
    pub fn IsErrorTransient(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsErrorTransient)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn NetworkCauseCode(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).NetworkCauseCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn TransportFailureCause(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).TransportFailureCause)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
}
impl ::core::clone::Clone for SmsSendMessageResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmsSendMessageResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmsSendMessageResult {}
impl ::core::fmt::Debug for SmsSendMessageResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsSendMessageResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmsSendMessageResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsSendMessageResult;{db139af2-78c9-4feb-9622-452328088d62})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SmsSendMessageResult {
    type Vtable = ISmsSendMessageResult_Vtbl;
    const IID: ::windows_core::GUID = <ISmsSendMessageResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SmsSendMessageResult {
    const NAME: &'static str = "Windows.Devices.Sms.SmsSendMessageResult";
}
impl ::core::convert::From<SmsSendMessageResult> for ::windows_core::IUnknown {
    fn from(value: SmsSendMessageResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmsSendMessageResult> for ::windows_core::IUnknown {
    fn from(value: &SmsSendMessageResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmsSendMessageResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmsSendMessageResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmsSendMessageResult> for ::windows_core::IInspectable {
    fn from(value: SmsSendMessageResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmsSendMessageResult> for ::windows_core::IInspectable {
    fn from(value: &SmsSendMessageResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmsSendMessageResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmsSendMessageResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SmsSendMessageResult {}
unsafe impl ::core::marker::Sync for SmsSendMessageResult {}
#[repr(transparent)]
pub struct SmsStatusMessage(::windows_core::IUnknown);
impl SmsStatusMessage {
    pub fn MessageType(&self) -> ::windows_core::Result<SmsMessageType> {
        let this = &::windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmsMessageType>::zeroed();
            (::windows_core::Interface::vtable(this).MessageType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsMessageType>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn CellularClass(&self) -> ::windows_core::Result<CellularClass> {
        let this = &::windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<CellularClass>::zeroed();
            (::windows_core::Interface::vtable(this).CellularClass)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CellularClass>(result__)
        }
    }
    pub fn MessageClass(&self) -> ::windows_core::Result<SmsMessageClass> {
        let this = &::windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmsMessageClass>::zeroed();
            (::windows_core::Interface::vtable(this).MessageClass)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsMessageClass>(result__)
        }
    }
    pub fn SimIccId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SimIccId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn To(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).To)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn From(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).From)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Body(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Body)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn MessageReferenceNumber(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).MessageReferenceNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn ServiceCenterTimestamp(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceCenterTimestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn DischargeTime(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).DischargeTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
}
impl ::core::clone::Clone for SmsStatusMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmsStatusMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmsStatusMessage {}
impl ::core::fmt::Debug for SmsStatusMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsStatusMessage").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmsStatusMessage {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsStatusMessage;{e6d28342-b70b-4677-9379-c9783fdff8f4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SmsStatusMessage {
    type Vtable = ISmsStatusMessage_Vtbl;
    const IID: ::windows_core::GUID = <ISmsStatusMessage as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SmsStatusMessage {
    const NAME: &'static str = "Windows.Devices.Sms.SmsStatusMessage";
}
impl ::core::convert::From<SmsStatusMessage> for ::windows_core::IUnknown {
    fn from(value: SmsStatusMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmsStatusMessage> for ::windows_core::IUnknown {
    fn from(value: &SmsStatusMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmsStatusMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmsStatusMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmsStatusMessage> for ::windows_core::IInspectable {
    fn from(value: SmsStatusMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmsStatusMessage> for ::windows_core::IInspectable {
    fn from(value: &SmsStatusMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmsStatusMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmsStatusMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SmsStatusMessage> for ISmsMessageBase {
    type Error = ::windows_core::Error;
    fn try_from(value: SmsStatusMessage) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SmsStatusMessage> for ISmsMessageBase {
    type Error = ::windows_core::Error;
    fn try_from(value: &SmsStatusMessage) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISmsMessageBase> for SmsStatusMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ISmsMessageBase> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISmsMessageBase> for &SmsStatusMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ISmsMessageBase> {
        ::core::convert::TryInto::<ISmsMessageBase>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SmsStatusMessage {}
unsafe impl ::core::marker::Sync for SmsStatusMessage {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct SmsTextMessage(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl SmsTextMessage {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SmsTextMessage, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<ISmsMessage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn MessageClass(&self) -> ::windows_core::Result<SmsMessageClass> {
        let this = &::windows_core::Interface::cast::<ISmsMessage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmsMessageClass>::zeroed();
            (::windows_core::Interface::vtable(this).MessageClass)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsMessageClass>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Timestamp(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn PartReferenceId(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).PartReferenceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn PartNumber(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).PartNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn PartCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).PartCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn To(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).To)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetTo<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTo)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn From(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).From)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetFrom<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFrom)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn Body(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Body)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetBody<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBody)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn Encoding(&self) -> ::windows_core::Result<SmsEncoding> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmsEncoding>::zeroed();
            (::windows_core::Interface::vtable(this).Encoding)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsEncoding>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetEncoding(&self, value: SmsEncoding) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEncoding)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn ToBinaryMessages(&self, format: SmsDataFormat) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<ISmsBinaryMessage>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ToBinaryMessages)(::windows_core::Interface::as_raw(this), format, result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<ISmsBinaryMessage>>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn FromBinaryMessage<'a, Param0: ::windows_core::IntoParam<'a, SmsBinaryMessage>>(binarymessage: Param0) -> ::windows_core::Result<SmsTextMessage> {
        Self::ISmsTextMessageStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromBinaryMessage)(::windows_core::Interface::as_raw(this), binarymessage.into_param().abi(), result__.as_mut_ptr()).from_abi::<SmsTextMessage>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn FromBinaryData(format: SmsDataFormat, value: &[u8]) -> ::windows_core::Result<SmsTextMessage> {
        Self::ISmsTextMessageStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromBinaryData)(::windows_core::Interface::as_raw(this), format, value.len() as u32, ::core::mem::transmute(value.as_ptr()), result__.as_mut_ptr()).from_abi::<SmsTextMessage>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn ISmsTextMessageStatics<R, F: FnOnce(&ISmsTextMessageStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SmsTextMessage, ISmsTextMessageStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for SmsTextMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for SmsTextMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for SmsTextMessage {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for SmsTextMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsTextMessage").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for SmsTextMessage {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsTextMessage;{d61c904c-a495-487f-9a6f-971548c5bc9f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for SmsTextMessage {
    type Vtable = ISmsTextMessage_Vtbl;
    const IID: ::windows_core::GUID = <ISmsTextMessage as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for SmsTextMessage {
    const NAME: &'static str = "Windows.Devices.Sms.SmsTextMessage";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SmsTextMessage> for ::windows_core::IUnknown {
    fn from(value: SmsTextMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SmsTextMessage> for ::windows_core::IUnknown {
    fn from(value: &SmsTextMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmsTextMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmsTextMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SmsTextMessage> for ::windows_core::IInspectable {
    fn from(value: SmsTextMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SmsTextMessage> for ::windows_core::IInspectable {
    fn from(value: &SmsTextMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmsTextMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmsTextMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<SmsTextMessage> for ISmsMessage {
    type Error = ::windows_core::Error;
    fn try_from(value: SmsTextMessage) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<&SmsTextMessage> for ISmsMessage {
    type Error = ::windows_core::Error;
    fn try_from(value: &SmsTextMessage) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ISmsMessage> for SmsTextMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ISmsMessage> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ISmsMessage> for &SmsTextMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ISmsMessage> {
        ::core::convert::TryInto::<ISmsMessage>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<SmsTextMessage> for ISmsTextMessage {
    type Error = ::windows_core::Error;
    fn try_from(value: SmsTextMessage) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<&SmsTextMessage> for ISmsTextMessage {
    type Error = ::windows_core::Error;
    fn try_from(value: &SmsTextMessage) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ISmsTextMessage> for SmsTextMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ISmsTextMessage> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ISmsTextMessage> for &SmsTextMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ISmsTextMessage> {
        ::core::convert::TryInto::<ISmsTextMessage>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for SmsTextMessage {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for SmsTextMessage {}
#[repr(transparent)]
pub struct SmsTextMessage2(::windows_core::IUnknown);
impl SmsTextMessage2 {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SmsTextMessage2, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn MessageType(&self) -> ::windows_core::Result<SmsMessageType> {
        let this = &::windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmsMessageType>::zeroed();
            (::windows_core::Interface::vtable(this).MessageType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsMessageType>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn CellularClass(&self) -> ::windows_core::Result<CellularClass> {
        let this = &::windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<CellularClass>::zeroed();
            (::windows_core::Interface::vtable(this).CellularClass)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CellularClass>(result__)
        }
    }
    pub fn MessageClass(&self) -> ::windows_core::Result<SmsMessageClass> {
        let this = &::windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmsMessageClass>::zeroed();
            (::windows_core::Interface::vtable(this).MessageClass)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsMessageClass>(result__)
        }
    }
    pub fn SimIccId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SimIccId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Timestamp(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn To(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).To)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetTo<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTo)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn From(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).From)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Body(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Body)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetBody<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBody)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Encoding(&self) -> ::windows_core::Result<SmsEncoding> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmsEncoding>::zeroed();
            (::windows_core::Interface::vtable(this).Encoding)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsEncoding>(result__)
        }
    }
    pub fn SetEncoding(&self, value: SmsEncoding) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEncoding)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CallbackNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CallbackNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetCallbackNumber<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCallbackNumber)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IsDeliveryNotificationEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDeliveryNotificationEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsDeliveryNotificationEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsDeliveryNotificationEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RetryAttemptCount(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).RetryAttemptCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetRetryAttemptCount(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRetryAttemptCount)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TeleserviceId(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).TeleserviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn ProtocolId(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).ProtocolId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
}
impl ::core::clone::Clone for SmsTextMessage2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmsTextMessage2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmsTextMessage2 {}
impl ::core::fmt::Debug for SmsTextMessage2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsTextMessage2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmsTextMessage2 {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsTextMessage2;{22a0d893-4555-4755-b5a1-e7fd84955f8d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SmsTextMessage2 {
    type Vtable = ISmsTextMessage2_Vtbl;
    const IID: ::windows_core::GUID = <ISmsTextMessage2 as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SmsTextMessage2 {
    const NAME: &'static str = "Windows.Devices.Sms.SmsTextMessage2";
}
impl ::core::convert::From<SmsTextMessage2> for ::windows_core::IUnknown {
    fn from(value: SmsTextMessage2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmsTextMessage2> for ::windows_core::IUnknown {
    fn from(value: &SmsTextMessage2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmsTextMessage2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmsTextMessage2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmsTextMessage2> for ::windows_core::IInspectable {
    fn from(value: SmsTextMessage2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmsTextMessage2> for ::windows_core::IInspectable {
    fn from(value: &SmsTextMessage2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmsTextMessage2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmsTextMessage2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SmsTextMessage2> for ISmsMessageBase {
    type Error = ::windows_core::Error;
    fn try_from(value: SmsTextMessage2) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SmsTextMessage2> for ISmsMessageBase {
    type Error = ::windows_core::Error;
    fn try_from(value: &SmsTextMessage2) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISmsMessageBase> for SmsTextMessage2 {
    fn into_param(self) -> ::windows_core::Param<'a, ISmsMessageBase> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISmsMessageBase> for &SmsTextMessage2 {
    fn into_param(self) -> ::windows_core::Param<'a, ISmsMessageBase> {
        ::core::convert::TryInto::<ISmsMessageBase>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SmsTextMessage2 {}
unsafe impl ::core::marker::Sync for SmsTextMessage2 {}
#[repr(transparent)]
pub struct SmsVoicemailMessage(::windows_core::IUnknown);
impl SmsVoicemailMessage {
    pub fn MessageType(&self) -> ::windows_core::Result<SmsMessageType> {
        let this = &::windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmsMessageType>::zeroed();
            (::windows_core::Interface::vtable(this).MessageType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsMessageType>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn CellularClass(&self) -> ::windows_core::Result<CellularClass> {
        let this = &::windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<CellularClass>::zeroed();
            (::windows_core::Interface::vtable(this).CellularClass)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CellularClass>(result__)
        }
    }
    pub fn MessageClass(&self) -> ::windows_core::Result<SmsMessageClass> {
        let this = &::windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmsMessageClass>::zeroed();
            (::windows_core::Interface::vtable(this).MessageClass)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsMessageClass>(result__)
        }
    }
    pub fn SimIccId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SimIccId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Timestamp(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn To(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).To)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Body(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Body)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn MessageCount(&self) -> ::windows_core::Result<::winrt_foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MessageCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<i32>>(result__)
        }
    }
}
impl ::core::clone::Clone for SmsVoicemailMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmsVoicemailMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmsVoicemailMessage {}
impl ::core::fmt::Debug for SmsVoicemailMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsVoicemailMessage").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmsVoicemailMessage {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsVoicemailMessage;{271aa0a6-95b1-44ff-bcb8-b8fdd7e08bc3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SmsVoicemailMessage {
    type Vtable = ISmsVoicemailMessage_Vtbl;
    const IID: ::windows_core::GUID = <ISmsVoicemailMessage as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SmsVoicemailMessage {
    const NAME: &'static str = "Windows.Devices.Sms.SmsVoicemailMessage";
}
impl ::core::convert::From<SmsVoicemailMessage> for ::windows_core::IUnknown {
    fn from(value: SmsVoicemailMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmsVoicemailMessage> for ::windows_core::IUnknown {
    fn from(value: &SmsVoicemailMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmsVoicemailMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmsVoicemailMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmsVoicemailMessage> for ::windows_core::IInspectable {
    fn from(value: SmsVoicemailMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmsVoicemailMessage> for ::windows_core::IInspectable {
    fn from(value: &SmsVoicemailMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmsVoicemailMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmsVoicemailMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SmsVoicemailMessage> for ISmsMessageBase {
    type Error = ::windows_core::Error;
    fn try_from(value: SmsVoicemailMessage) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SmsVoicemailMessage> for ISmsMessageBase {
    type Error = ::windows_core::Error;
    fn try_from(value: &SmsVoicemailMessage) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISmsMessageBase> for SmsVoicemailMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ISmsMessageBase> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISmsMessageBase> for &SmsVoicemailMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ISmsMessageBase> {
        ::core::convert::TryInto::<ISmsMessageBase>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SmsVoicemailMessage {}
unsafe impl ::core::marker::Sync for SmsVoicemailMessage {}
#[repr(transparent)]
pub struct SmsWapMessage(::windows_core::IUnknown);
impl SmsWapMessage {
    pub fn MessageType(&self) -> ::windows_core::Result<SmsMessageType> {
        let this = &::windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmsMessageType>::zeroed();
            (::windows_core::Interface::vtable(this).MessageType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsMessageType>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn CellularClass(&self) -> ::windows_core::Result<CellularClass> {
        let this = &::windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<CellularClass>::zeroed();
            (::windows_core::Interface::vtable(this).CellularClass)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CellularClass>(result__)
        }
    }
    pub fn MessageClass(&self) -> ::windows_core::Result<SmsMessageClass> {
        let this = &::windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmsMessageClass>::zeroed();
            (::windows_core::Interface::vtable(this).MessageClass)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmsMessageClass>(result__)
        }
    }
    pub fn SimIccId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SimIccId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Timestamp(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn To(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).To)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn From(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).From)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ApplicationId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ApplicationId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ContentType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ContentType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn BinaryBody(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BinaryBody)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Headers(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Headers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(result__)
        }
    }
}
impl ::core::clone::Clone for SmsWapMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmsWapMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmsWapMessage {}
impl ::core::fmt::Debug for SmsWapMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsWapMessage").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmsWapMessage {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsWapMessage;{cd937743-7a55-4d3b-9021-f22e022d09c5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SmsWapMessage {
    type Vtable = ISmsWapMessage_Vtbl;
    const IID: ::windows_core::GUID = <ISmsWapMessage as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SmsWapMessage {
    const NAME: &'static str = "Windows.Devices.Sms.SmsWapMessage";
}
impl ::core::convert::From<SmsWapMessage> for ::windows_core::IUnknown {
    fn from(value: SmsWapMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmsWapMessage> for ::windows_core::IUnknown {
    fn from(value: &SmsWapMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmsWapMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmsWapMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmsWapMessage> for ::windows_core::IInspectable {
    fn from(value: SmsWapMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmsWapMessage> for ::windows_core::IInspectable {
    fn from(value: &SmsWapMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmsWapMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmsWapMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SmsWapMessage> for ISmsMessageBase {
    type Error = ::windows_core::Error;
    fn try_from(value: SmsWapMessage) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SmsWapMessage> for ISmsMessageBase {
    type Error = ::windows_core::Error;
    fn try_from(value: &SmsWapMessage) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISmsMessageBase> for SmsWapMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ISmsMessageBase> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISmsMessageBase> for &SmsWapMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ISmsMessageBase> {
        ::core::convert::TryInto::<ISmsMessageBase>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SmsWapMessage {}
unsafe impl ::core::marker::Sync for SmsWapMessage {}
