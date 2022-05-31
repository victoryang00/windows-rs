#[doc(hidden)]
#[repr(transparent)]
pub struct IXsltProcessor(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXsltProcessor {
    type Vtable = IXsltProcessor_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7b64703f_550c_48c6_a90f_93a5b964518f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXsltProcessor_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-data")]
    pub TransformToString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputnode: ::windows_core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-data"))]
    TransformToString: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXsltProcessor2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXsltProcessor2 {
    type Vtable = IXsltProcessor2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8da45c56_97a5_44cb_a8be_27d86280c70a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXsltProcessor2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-data")]
    pub TransformToDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputnode: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-data"))]
    TransformToDocument: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXsltProcessorFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXsltProcessorFactory {
    type Vtable = IXsltProcessorFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x274146c0_9a51_4663_bf30_0ef742146f20);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXsltProcessorFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-data")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, document: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-data"))]
    CreateInstance: usize,
}
#[repr(transparent)]
pub struct XsltProcessor(::windows_core::IUnknown);
impl XsltProcessor {
    #[cfg(feature = "winrt-data")]
    pub fn TransformToString<'a, Param0: ::windows_core::IntoParam<'a, super::Dom::IXmlNode>>(&self, inputnode: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TransformToString)(::windows_core::Interface::as_raw(this), inputnode.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-data")]
    pub fn TransformToDocument<'a, Param0: ::windows_core::IntoParam<'a, super::Dom::IXmlNode>>(&self, inputnode: Param0) -> ::windows_core::Result<super::Dom::XmlDocument> {
        let this = &::windows_core::Interface::cast::<IXsltProcessor2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TransformToDocument)(::windows_core::Interface::as_raw(this), inputnode.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Dom::XmlDocument>(result__)
        }
    }
    #[cfg(feature = "winrt-data")]
    pub fn CreateInstance<'a, Param0: ::windows_core::IntoParam<'a, super::Dom::XmlDocument>>(document: Param0) -> ::windows_core::Result<XsltProcessor> {
        Self::IXsltProcessorFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), document.into_param().abi(), result__.as_mut_ptr()).from_abi::<XsltProcessor>(result__)
        })
    }
    pub fn IXsltProcessorFactory<R, F: FnOnce(&IXsltProcessorFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<XsltProcessor, IXsltProcessorFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for XsltProcessor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XsltProcessor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XsltProcessor {}
impl ::core::fmt::Debug for XsltProcessor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XsltProcessor").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XsltProcessor {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Xsl.XsltProcessor;{7b64703f-550c-48c6-a90f-93a5b964518f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for XsltProcessor {
    type Vtable = IXsltProcessor_Vtbl;
    const IID: ::windows_core::GUID = <IXsltProcessor as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for XsltProcessor {
    const NAME: &'static str = "Windows.Data.Xml.Xsl.XsltProcessor";
}
impl ::core::convert::From<XsltProcessor> for ::windows_core::IUnknown {
    fn from(value: XsltProcessor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XsltProcessor> for ::windows_core::IUnknown {
    fn from(value: &XsltProcessor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for XsltProcessor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a XsltProcessor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XsltProcessor> for ::windows_core::IInspectable {
    fn from(value: XsltProcessor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XsltProcessor> for ::windows_core::IInspectable {
    fn from(value: &XsltProcessor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for XsltProcessor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a XsltProcessor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for XsltProcessor {}
unsafe impl ::core::marker::Sync for XsltProcessor {}
