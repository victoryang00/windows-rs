pub const CLSID_XMLGraphBuilder: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1bb05961_5fbf_11d2_a521_44df07c10000);
#[repr(transparent)]
pub struct IXMLGraphBuilder(::windows_core::IUnknown);
impl IXMLGraphBuilder {
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub unsafe fn BuildFromXML<'a, Param0: ::windows_core::IntoParam<'a, super::IGraphBuilder>, Param1: ::windows_core::IntoParam<'a, ::win32_data::Xml::MsXml::IXMLElement>>(&self, pgraph: Param0, pxml: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BuildFromXML)(::windows_core::Interface::as_raw(self), pgraph.into_param().abi(), pxml.into_param().abi()).ok()
    }
    pub unsafe fn SaveToXML<'a, Param0: ::windows_core::IntoParam<'a, super::IGraphBuilder>>(&self, pgraph: Param0, pbstrxml: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SaveToXML)(::windows_core::Interface::as_raw(self), pgraph.into_param().abi(), ::core::mem::transmute(pbstrxml)).ok()
    }
    pub unsafe fn BuildFromXMLFile<'a, Param0: ::windows_core::IntoParam<'a, super::IGraphBuilder>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pgraph: Param0, wszfilename: Param1, wszbaseurl: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BuildFromXMLFile)(::windows_core::Interface::as_raw(self), pgraph.into_param().abi(), wszfilename.into_param().abi(), wszbaseurl.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IXMLGraphBuilder> for ::windows_core::IUnknown {
    fn from(value: IXMLGraphBuilder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXMLGraphBuilder> for ::windows_core::IUnknown {
    fn from(value: &IXMLGraphBuilder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IXMLGraphBuilder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IXMLGraphBuilder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXMLGraphBuilder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXMLGraphBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXMLGraphBuilder {}
impl ::core::fmt::Debug for IXMLGraphBuilder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXMLGraphBuilder").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IXMLGraphBuilder {
    type Vtable = IXMLGraphBuilder_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1bb05960_5fbf_11d2_a521_44df07c10000);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXMLGraphBuilder_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub BuildFromXML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgraph: ::windows_core::RawPtr, pxml: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com")))]
    BuildFromXML: usize,
    pub SaveToXML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgraph: ::windows_core::RawPtr, pbstrxml: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub BuildFromXMLFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgraph: ::windows_core::RawPtr, wszfilename: ::windows_core::PCWSTR, wszbaseurl: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
