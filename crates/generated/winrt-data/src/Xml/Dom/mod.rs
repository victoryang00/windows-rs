#[repr(transparent)]
pub struct DtdEntity(::windows_core::IUnknown);
impl DtdEntity {
    pub fn PublicId(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).PublicId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn SystemId(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).SystemId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn NotationName(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).NotationName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn SetNodeValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows_core::Result<NodeType> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<NodeType>::zeroed();
            (::windows_core::Interface::vtable(this).NodeType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NodeType>(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ParentNode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ChildNodes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FirstChild)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LastChild)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousSibling)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NextSibling)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows_core::Result<XmlNamedNodeMap> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Attributes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasChildNodes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows_core::Result<XmlDocument> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OwnerDocument)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlDocument>(result__)
        }
    }
    pub fn InsertBefore<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>, Param1: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InsertBefore)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), referencechild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ReplaceChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>, Param1: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReplaceChild)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), referencechild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn RemoveChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoveChild)(::windows_core::Interface::as_raw(this), childnode.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn AppendChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppendChild)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CloneNode)(::windows_core::Interface::as_raw(this), deep, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).NamespaceUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).LocalName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).Prefix)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Normalize)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPrefix)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, xpath: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNode)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodes<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, xpath: Param0) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodes)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn SelectSingleNodeNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNodeNS)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), namespaces.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodesNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodesNS)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), namespaces.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetXml)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).InnerText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetInnerText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetInnerText)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for DtdEntity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DtdEntity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DtdEntity {}
impl ::core::fmt::Debug for DtdEntity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DtdEntity").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DtdEntity {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.DtdEntity;{6a0b5ffc-63b4-480f-9e6a-8a92816aade4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DtdEntity {
    type Vtable = IDtdEntity_Vtbl;
    const IID: ::windows_core::GUID = <IDtdEntity as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DtdEntity {
    const NAME: &'static str = "Windows.Data.Xml.Dom.DtdEntity";
}
impl ::core::convert::From<DtdEntity> for ::windows_core::IUnknown {
    fn from(value: DtdEntity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DtdEntity> for ::windows_core::IUnknown {
    fn from(value: &DtdEntity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DtdEntity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DtdEntity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DtdEntity> for ::windows_core::IInspectable {
    fn from(value: DtdEntity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DtdEntity> for ::windows_core::IInspectable {
    fn from(value: &DtdEntity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DtdEntity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DtdEntity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<DtdEntity> for IXmlNode {
    type Error = ::windows_core::Error;
    fn try_from(value: DtdEntity) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DtdEntity> for IXmlNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &DtdEntity) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNode> for DtdEntity {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNode> for &DtdEntity {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNode> {
        ::core::convert::TryInto::<IXmlNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<DtdEntity> for IXmlNodeSelector {
    type Error = ::windows_core::Error;
    fn try_from(value: DtdEntity) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DtdEntity> for IXmlNodeSelector {
    type Error = ::windows_core::Error;
    fn try_from(value: &DtdEntity) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSelector> for DtdEntity {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSelector> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSelector> for &DtdEntity {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSelector> {
        ::core::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<DtdEntity> for IXmlNodeSerializer {
    type Error = ::windows_core::Error;
    fn try_from(value: DtdEntity) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DtdEntity> for IXmlNodeSerializer {
    type Error = ::windows_core::Error;
    fn try_from(value: &DtdEntity) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSerializer> for DtdEntity {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSerializer> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSerializer> for &DtdEntity {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSerializer> {
        ::core::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DtdEntity {}
unsafe impl ::core::marker::Sync for DtdEntity {}
#[repr(transparent)]
pub struct DtdNotation(::windows_core::IUnknown);
impl DtdNotation {
    pub fn PublicId(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).PublicId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn SystemId(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).SystemId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn SetNodeValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows_core::Result<NodeType> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<NodeType>::zeroed();
            (::windows_core::Interface::vtable(this).NodeType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NodeType>(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ParentNode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ChildNodes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FirstChild)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LastChild)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousSibling)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NextSibling)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows_core::Result<XmlNamedNodeMap> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Attributes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasChildNodes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows_core::Result<XmlDocument> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OwnerDocument)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlDocument>(result__)
        }
    }
    pub fn InsertBefore<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>, Param1: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InsertBefore)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), referencechild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ReplaceChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>, Param1: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReplaceChild)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), referencechild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn RemoveChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoveChild)(::windows_core::Interface::as_raw(this), childnode.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn AppendChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppendChild)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CloneNode)(::windows_core::Interface::as_raw(this), deep, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).NamespaceUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).LocalName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).Prefix)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Normalize)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPrefix)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, xpath: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNode)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodes<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, xpath: Param0) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodes)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn SelectSingleNodeNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNodeNS)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), namespaces.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodesNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodesNS)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), namespaces.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetXml)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).InnerText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetInnerText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetInnerText)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for DtdNotation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DtdNotation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DtdNotation {}
impl ::core::fmt::Debug for DtdNotation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DtdNotation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DtdNotation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.DtdNotation;{8cb4e04d-6d46-4edb-ab73-df83c51ad397})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DtdNotation {
    type Vtable = IDtdNotation_Vtbl;
    const IID: ::windows_core::GUID = <IDtdNotation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DtdNotation {
    const NAME: &'static str = "Windows.Data.Xml.Dom.DtdNotation";
}
impl ::core::convert::From<DtdNotation> for ::windows_core::IUnknown {
    fn from(value: DtdNotation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DtdNotation> for ::windows_core::IUnknown {
    fn from(value: &DtdNotation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DtdNotation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DtdNotation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DtdNotation> for ::windows_core::IInspectable {
    fn from(value: DtdNotation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DtdNotation> for ::windows_core::IInspectable {
    fn from(value: &DtdNotation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DtdNotation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DtdNotation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<DtdNotation> for IXmlNode {
    type Error = ::windows_core::Error;
    fn try_from(value: DtdNotation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DtdNotation> for IXmlNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &DtdNotation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNode> for DtdNotation {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNode> for &DtdNotation {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNode> {
        ::core::convert::TryInto::<IXmlNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<DtdNotation> for IXmlNodeSelector {
    type Error = ::windows_core::Error;
    fn try_from(value: DtdNotation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DtdNotation> for IXmlNodeSelector {
    type Error = ::windows_core::Error;
    fn try_from(value: &DtdNotation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSelector> for DtdNotation {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSelector> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSelector> for &DtdNotation {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSelector> {
        ::core::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<DtdNotation> for IXmlNodeSerializer {
    type Error = ::windows_core::Error;
    fn try_from(value: DtdNotation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DtdNotation> for IXmlNodeSerializer {
    type Error = ::windows_core::Error;
    fn try_from(value: &DtdNotation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSerializer> for DtdNotation {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSerializer> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSerializer> for &DtdNotation {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSerializer> {
        ::core::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DtdNotation {}
unsafe impl ::core::marker::Sync for DtdNotation {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDtdEntity(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDtdEntity {
    type Vtable = IDtdEntity_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6a0b5ffc_63b4_480f_9e6a_8a92816aade4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtdEntity_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PublicId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SystemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NotationName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDtdNotation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDtdNotation {
    type Vtable = IDtdNotation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8cb4e04d_6d46_4edb_ab73_df83c51ad397);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtdNotation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PublicId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SystemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlAttribute(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXmlAttribute {
    type Vtable = IXmlAttribute_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xac144aa4_b4f1_4db6_b206_8a22c308db0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlAttribute_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Specified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlCDataSection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXmlCDataSection {
    type Vtable = IXmlCDataSection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4d04b46f_c8bd_45b4_8899_0400d7c2c60f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlCDataSection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[repr(transparent)]
pub struct IXmlCharacterData(::windows_core::IUnknown);
impl IXmlCharacterData {
    pub fn Data(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetData)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Length(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Length)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SubstringData(&self, offset: u32, count: u32) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SubstringData)(::windows_core::Interface::as_raw(this), offset, count, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AppendData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, data: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AppendData)(::windows_core::Interface::as_raw(this), data.into_param().abi()).ok() }
    }
    pub fn InsertData<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, offset: u32, data: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).InsertData)(::windows_core::Interface::as_raw(this), offset, data.into_param().abi()).ok() }
    }
    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).DeleteData)(::windows_core::Interface::as_raw(this), offset, count).ok() }
    }
    pub fn ReplaceData<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, offset: u32, count: u32, data: Param2) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceData)(::windows_core::Interface::as_raw(this), offset, count, data.into_param().abi()).ok() }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn SetNodeValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows_core::Result<NodeType> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<NodeType>::zeroed();
            (::windows_core::Interface::vtable(this).NodeType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NodeType>(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ParentNode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ChildNodes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FirstChild)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LastChild)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousSibling)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NextSibling)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows_core::Result<XmlNamedNodeMap> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Attributes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasChildNodes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows_core::Result<XmlDocument> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OwnerDocument)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlDocument>(result__)
        }
    }
    pub fn InsertBefore<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>, Param1: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InsertBefore)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), referencechild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ReplaceChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>, Param1: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReplaceChild)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), referencechild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn RemoveChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoveChild)(::windows_core::Interface::as_raw(this), childnode.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn AppendChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppendChild)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CloneNode)(::windows_core::Interface::as_raw(this), deep, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).NamespaceUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).LocalName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).Prefix)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Normalize)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPrefix)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, xpath: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNode)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodes<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, xpath: Param0) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodes)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn SelectSingleNodeNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNodeNS)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), namespaces.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodesNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodesNS)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), namespaces.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetXml)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).InnerText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetInnerText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetInnerText)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<IXmlCharacterData> for ::windows_core::IUnknown {
    fn from(value: IXmlCharacterData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXmlCharacterData> for ::windows_core::IUnknown {
    fn from(value: &IXmlCharacterData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IXmlCharacterData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IXmlCharacterData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXmlCharacterData> for ::windows_core::IInspectable {
    fn from(value: IXmlCharacterData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXmlCharacterData> for ::windows_core::IInspectable {
    fn from(value: &IXmlCharacterData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IXmlCharacterData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IXmlCharacterData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IXmlCharacterData> for IXmlNode {
    type Error = ::windows_core::Error;
    fn try_from(value: IXmlCharacterData) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IXmlCharacterData> for IXmlNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &IXmlCharacterData) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNode> for IXmlCharacterData {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNode> for &IXmlCharacterData {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNode> {
        ::core::convert::TryInto::<IXmlNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<IXmlCharacterData> for IXmlNodeSelector {
    type Error = ::windows_core::Error;
    fn try_from(value: IXmlCharacterData) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IXmlCharacterData> for IXmlNodeSelector {
    type Error = ::windows_core::Error;
    fn try_from(value: &IXmlCharacterData) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSelector> for IXmlCharacterData {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSelector> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSelector> for &IXmlCharacterData {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSelector> {
        ::core::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<IXmlCharacterData> for IXmlNodeSerializer {
    type Error = ::windows_core::Error;
    fn try_from(value: IXmlCharacterData) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IXmlCharacterData> for IXmlNodeSerializer {
    type Error = ::windows_core::Error;
    fn try_from(value: &IXmlCharacterData) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSerializer> for IXmlCharacterData {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSerializer> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSerializer> for &IXmlCharacterData {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSerializer> {
        ::core::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IXmlCharacterData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXmlCharacterData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXmlCharacterData {}
impl ::core::fmt::Debug for IXmlCharacterData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXmlCharacterData").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IXmlCharacterData {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{132e42ab-4e36-4df6-b1c8-0ce62fd88b26}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IXmlCharacterData {
    type Vtable = IXmlCharacterData_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x132e42ab_4e36_4df6_b1c8_0ce62fd88b26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlCharacterData_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SubstringData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: u32, count: u32, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AppendData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub InsertData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: u32, data: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DeleteData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: u32, count: u32) -> ::windows_core::HRESULT,
    pub ReplaceData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: u32, count: u32, data: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlComment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXmlComment {
    type Vtable = IXmlComment_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbca474d5_b61f_4611_9cac_2e92e3476d47);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlComment_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlDocument(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXmlDocument {
    type Vtable = IXmlDocument_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf7f3a506_1e87_42d6_bcfb_b8c809fa5494);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocument_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Doctype: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Implementation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DocumentElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tagname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateDocumentFragment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateTextNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateComment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateProcessingInstruction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, data: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateEntityReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetElementsByTagName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tagname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateCDataSection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DocumentUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CreateAttributeNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, qualifiedname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateElementNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, qualifiedname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetElementById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, elementid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ImportNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: ::windows_core::RawPtr, deep: bool, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlDocumentFragment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXmlDocumentFragment {
    type Vtable = IXmlDocumentFragment_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe2ea6a96_0c21_44a5_8bc9_9e4a262708ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocumentFragment_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlDocumentIO(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXmlDocumentIO {
    type Vtable = IXmlDocumentIO_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6cd0e74e_ee65_4489_9ebf_ca43e87ba637);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocumentIO_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LoadXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xml: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LoadXmlWithSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xml: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, loadsettings: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub SaveToFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    SaveToFileAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlDocumentIO2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXmlDocumentIO2 {
    type Vtable = IXmlDocumentIO2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d034661_7bd8_4ad5_9ebf_81e6347263b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocumentIO2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-storage")]
    pub LoadXmlFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    LoadXmlFromBuffer: usize,
    #[cfg(feature = "winrt-storage")]
    pub LoadXmlFromBufferWithSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: ::windows_core::RawPtr, loadsettings: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    LoadXmlFromBufferWithSettings: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlDocumentStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXmlDocumentStatics {
    type Vtable = IXmlDocumentStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5543d254_d757_4b79_9539_232b18f50bf1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocumentStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LoadFromUriAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LoadFromUriWithSettingsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, loadsettings: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub LoadFromFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    LoadFromFileAsync: usize,
    #[cfg(feature = "winrt-storage")]
    pub LoadFromFileWithSettingsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows_core::RawPtr, loadsettings: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    LoadFromFileWithSettingsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlDocumentType(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXmlDocumentType {
    type Vtable = IXmlDocumentType_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf7342425_9781_4964_8e94_9b1c6dfc9bc7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDocumentType_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Entities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Notations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlDomImplementation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXmlDomImplementation {
    type Vtable = IXmlDomImplementation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6de58132_f11d_4fbb_8cc6_583cba93112f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlDomImplementation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub HasFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feature: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, version: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlElement(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXmlElement {
    type Vtable = IXmlElement_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2dfb8a1f_6b10_4ef8_9f83_efcce8faec37);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlElement_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TagName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, attributevalue: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RemoveAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetAttributeNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetAttributeNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newattribute: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveAttributeNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributenode: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetElementsByTagName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tagname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetAttributeNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, qualifiedname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetAttributeNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, localname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RemoveAttributeNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, localname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetAttributeNodeNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newattribute: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetAttributeNodeNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, localname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlEntityReference(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXmlEntityReference {
    type Vtable = IXmlEntityReference_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2e2f47bc_c3d0_4ccf_bb86_0ab8c36a61cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlEntityReference_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlLoadSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXmlLoadSettings {
    type Vtable = IXmlLoadSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x58aa07a8_fed6_46f7_b4c5_fb1ba72108d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlLoadSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MaxElementDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetMaxElementDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub ProhibitDtd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetProhibitDtd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ResolveExternals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetResolveExternals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ValidateOnParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetValidateOnParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ElementContentWhiteSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetElementContentWhiteSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlNamedNodeMap(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXmlNamedNodeMap {
    type Vtable = IXmlNamedNodeMap_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb3a69eb0_aab0_4b82_a6fa_b1453f7c021b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlNamedNodeMap_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetNamedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetNamedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveNamedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetNamedItemNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveNamedItemNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetNamedItemNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IXmlNode(::windows_core::IUnknown);
impl IXmlNode {
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn SetNodeValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows_core::Result<NodeType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<NodeType>::zeroed();
            (::windows_core::Interface::vtable(this).NodeType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NodeType>(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ParentNode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows_core::Result<XmlNodeList> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ChildNodes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FirstChild)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LastChild)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousSibling)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NextSibling)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows_core::Result<XmlNamedNodeMap> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Attributes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasChildNodes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows_core::Result<XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OwnerDocument)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlDocument>(result__)
        }
    }
    pub fn InsertBefore<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>, Param1: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InsertBefore)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), referencechild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ReplaceChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>, Param1: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReplaceChild)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), referencechild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn RemoveChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoveChild)(::windows_core::Interface::as_raw(this), childnode.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn AppendChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppendChild)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CloneNode)(::windows_core::Interface::as_raw(this), deep, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).NamespaceUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).LocalName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).Prefix)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Normalize)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPrefix)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, xpath: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNode)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodes<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, xpath: Param0) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodes)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn SelectSingleNodeNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNodeNS)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), namespaces.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodesNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodesNS)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), namespaces.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetXml)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).InnerText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetInnerText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetInnerText)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<IXmlNode> for ::windows_core::IUnknown {
    fn from(value: IXmlNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXmlNode> for ::windows_core::IUnknown {
    fn from(value: &IXmlNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IXmlNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IXmlNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXmlNode> for ::windows_core::IInspectable {
    fn from(value: IXmlNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXmlNode> for ::windows_core::IInspectable {
    fn from(value: &IXmlNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IXmlNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IXmlNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IXmlNode> for IXmlNodeSelector {
    type Error = ::windows_core::Error;
    fn try_from(value: IXmlNode) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IXmlNode> for IXmlNodeSelector {
    type Error = ::windows_core::Error;
    fn try_from(value: &IXmlNode) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSelector> for IXmlNode {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSelector> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSelector> for &IXmlNode {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSelector> {
        ::core::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<IXmlNode> for IXmlNodeSerializer {
    type Error = ::windows_core::Error;
    fn try_from(value: IXmlNode) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IXmlNode> for IXmlNodeSerializer {
    type Error = ::windows_core::Error;
    fn try_from(value: &IXmlNode) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSerializer> for IXmlNode {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSerializer> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSerializer> for &IXmlNode {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSerializer> {
        ::core::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IXmlNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXmlNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXmlNode {}
impl ::core::fmt::Debug for IXmlNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXmlNode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IXmlNode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{1c741d59-2122-47d5-a856-83f3d4214875}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IXmlNode {
    type Vtable = IXmlNode_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1c741d59_2122_47d5_a856_83f3d4214875);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlNode_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub NodeValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetNodeValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NodeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NodeType) -> ::windows_core::HRESULT,
    pub NodeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ParentNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ChildNodes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FirstChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LastChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PreviousSibling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub NextSibling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Attributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub HasChildNodes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub OwnerDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub InsertBefore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newchild: ::windows_core::RawPtr, referencechild: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ReplaceChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newchild: ::windows_core::RawPtr, referencechild: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, childnode: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AppendChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newchild: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CloneNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deep: bool, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub NamespaceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub LocalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Prefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Normalize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetPrefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlNodeList(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXmlNodeList {
    type Vtable = IXmlNodeList_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8c60ad77_83a4_4ec1_9c54_7ba429e13da6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlNodeList_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IXmlNodeSelector(::windows_core::IUnknown);
impl IXmlNodeSelector {
    pub fn SelectSingleNode<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, xpath: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNode)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodes<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, xpath: Param0) -> ::windows_core::Result<XmlNodeList> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodes)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn SelectSingleNodeNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNodeNS)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), namespaces.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodesNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows_core::Result<XmlNodeList> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodesNS)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), namespaces.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
}
impl ::core::convert::From<IXmlNodeSelector> for ::windows_core::IUnknown {
    fn from(value: IXmlNodeSelector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXmlNodeSelector> for ::windows_core::IUnknown {
    fn from(value: &IXmlNodeSelector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IXmlNodeSelector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IXmlNodeSelector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXmlNodeSelector> for ::windows_core::IInspectable {
    fn from(value: IXmlNodeSelector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXmlNodeSelector> for ::windows_core::IInspectable {
    fn from(value: &IXmlNodeSelector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IXmlNodeSelector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IXmlNodeSelector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXmlNodeSelector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXmlNodeSelector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXmlNodeSelector {}
impl ::core::fmt::Debug for IXmlNodeSelector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXmlNodeSelector").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IXmlNodeSelector {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{63dbba8b-d0db-4fe1-b745-f9433afdc25b}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IXmlNodeSelector {
    type Vtable = IXmlNodeSelector_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x63dbba8b_d0db_4fe1_b745_f9433afdc25b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlNodeSelector_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SelectSingleNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpath: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectNodes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpath: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectSingleNodeNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpath: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, namespaces: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectNodesNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpath: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, namespaces: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IXmlNodeSerializer(::windows_core::IUnknown);
impl IXmlNodeSerializer {
    pub fn GetXml(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetXml)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).InnerText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetInnerText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInnerText)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<IXmlNodeSerializer> for ::windows_core::IUnknown {
    fn from(value: IXmlNodeSerializer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXmlNodeSerializer> for ::windows_core::IUnknown {
    fn from(value: &IXmlNodeSerializer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IXmlNodeSerializer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IXmlNodeSerializer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXmlNodeSerializer> for ::windows_core::IInspectable {
    fn from(value: IXmlNodeSerializer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXmlNodeSerializer> for ::windows_core::IInspectable {
    fn from(value: &IXmlNodeSerializer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IXmlNodeSerializer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IXmlNodeSerializer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXmlNodeSerializer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXmlNodeSerializer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXmlNodeSerializer {}
impl ::core::fmt::Debug for IXmlNodeSerializer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXmlNodeSerializer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IXmlNodeSerializer {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{5cc5b382-e6dd-4991-abef-06d8d2e7bd0c}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IXmlNodeSerializer {
    type Vtable = IXmlNodeSerializer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5cc5b382_e6dd_4991_abef_06d8d2e7bd0c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlNodeSerializer_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub InnerText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetInnerText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXmlProcessingInstruction(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXmlProcessingInstruction {
    type Vtable = IXmlProcessingInstruction_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2707fd1e_1e92_4ece_b6f4_26f069078ddc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlProcessingInstruction_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Target: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IXmlText(::windows_core::IUnknown);
impl IXmlText {
    pub fn SplitText(&self, offset: u32) -> ::windows_core::Result<IXmlText> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplitText)(::windows_core::Interface::as_raw(this), offset, result__.as_mut_ptr()).from_abi::<IXmlText>(result__)
        }
    }
    pub fn Data(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetData)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Length(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Length)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SubstringData(&self, offset: u32, count: u32) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SubstringData)(::windows_core::Interface::as_raw(this), offset, count, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AppendData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, data: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AppendData)(::windows_core::Interface::as_raw(this), data.into_param().abi()).ok() }
    }
    pub fn InsertData<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, offset: u32, data: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).InsertData)(::windows_core::Interface::as_raw(this), offset, data.into_param().abi()).ok() }
    }
    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).DeleteData)(::windows_core::Interface::as_raw(this), offset, count).ok() }
    }
    pub fn ReplaceData<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, offset: u32, count: u32, data: Param2) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceData)(::windows_core::Interface::as_raw(this), offset, count, data.into_param().abi()).ok() }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn SetNodeValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows_core::Result<NodeType> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<NodeType>::zeroed();
            (::windows_core::Interface::vtable(this).NodeType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NodeType>(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ParentNode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ChildNodes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FirstChild)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LastChild)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousSibling)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NextSibling)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows_core::Result<XmlNamedNodeMap> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Attributes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasChildNodes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows_core::Result<XmlDocument> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OwnerDocument)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlDocument>(result__)
        }
    }
    pub fn InsertBefore<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>, Param1: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InsertBefore)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), referencechild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ReplaceChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>, Param1: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReplaceChild)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), referencechild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn RemoveChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoveChild)(::windows_core::Interface::as_raw(this), childnode.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn AppendChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppendChild)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CloneNode)(::windows_core::Interface::as_raw(this), deep, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).NamespaceUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).LocalName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).Prefix)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Normalize)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPrefix)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, xpath: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNode)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodes<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, xpath: Param0) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodes)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn SelectSingleNodeNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNodeNS)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), namespaces.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodesNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodesNS)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), namespaces.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetXml)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).InnerText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetInnerText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetInnerText)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<IXmlText> for ::windows_core::IUnknown {
    fn from(value: IXmlText) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXmlText> for ::windows_core::IUnknown {
    fn from(value: &IXmlText) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IXmlText {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IXmlText {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXmlText> for ::windows_core::IInspectable {
    fn from(value: IXmlText) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXmlText> for ::windows_core::IInspectable {
    fn from(value: &IXmlText) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IXmlText {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IXmlText {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IXmlText> for IXmlCharacterData {
    type Error = ::windows_core::Error;
    fn try_from(value: IXmlText) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IXmlText> for IXmlCharacterData {
    type Error = ::windows_core::Error;
    fn try_from(value: &IXmlText) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlCharacterData> for IXmlText {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlCharacterData> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlCharacterData> for &IXmlText {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlCharacterData> {
        ::core::convert::TryInto::<IXmlCharacterData>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<IXmlText> for IXmlNode {
    type Error = ::windows_core::Error;
    fn try_from(value: IXmlText) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IXmlText> for IXmlNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &IXmlText) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNode> for IXmlText {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNode> for &IXmlText {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNode> {
        ::core::convert::TryInto::<IXmlNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<IXmlText> for IXmlNodeSelector {
    type Error = ::windows_core::Error;
    fn try_from(value: IXmlText) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IXmlText> for IXmlNodeSelector {
    type Error = ::windows_core::Error;
    fn try_from(value: &IXmlText) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSelector> for IXmlText {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSelector> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSelector> for &IXmlText {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSelector> {
        ::core::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<IXmlText> for IXmlNodeSerializer {
    type Error = ::windows_core::Error;
    fn try_from(value: IXmlText) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IXmlText> for IXmlNodeSerializer {
    type Error = ::windows_core::Error;
    fn try_from(value: &IXmlText) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSerializer> for IXmlText {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSerializer> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSerializer> for &IXmlText {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSerializer> {
        ::core::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IXmlText {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXmlText {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXmlText {}
impl ::core::fmt::Debug for IXmlText {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXmlText").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IXmlText {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{f931a4cb-308d-4760-a1d5-43b67450ac7e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IXmlText {
    type Vtable = IXmlText_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf931a4cb_308d_4760_a1d5_43b67450ac7e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlText_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SplitText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NodeType(pub i32);
impl NodeType {
    pub const Invalid: Self = Self(0i32);
    pub const ElementNode: Self = Self(1i32);
    pub const AttributeNode: Self = Self(2i32);
    pub const TextNode: Self = Self(3i32);
    pub const DataSectionNode: Self = Self(4i32);
    pub const EntityReferenceNode: Self = Self(5i32);
    pub const EntityNode: Self = Self(6i32);
    pub const ProcessingInstructionNode: Self = Self(7i32);
    pub const CommentNode: Self = Self(8i32);
    pub const DocumentNode: Self = Self(9i32);
    pub const DocumentTypeNode: Self = Self(10i32);
    pub const DocumentFragmentNode: Self = Self(11i32);
    pub const NotationNode: Self = Self(12i32);
}
impl ::core::marker::Copy for NodeType {}
impl ::core::clone::Clone for NodeType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NodeType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NodeType {
    type Abi = Self;
}
impl ::core::fmt::Debug for NodeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NodeType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for NodeType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Data.Xml.Dom.NodeType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct XmlAttribute(::windows_core::IUnknown);
impl XmlAttribute {
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Specified(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Specified)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn SetNodeValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows_core::Result<NodeType> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<NodeType>::zeroed();
            (::windows_core::Interface::vtable(this).NodeType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NodeType>(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ParentNode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ChildNodes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FirstChild)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LastChild)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousSibling)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NextSibling)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows_core::Result<XmlNamedNodeMap> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Attributes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasChildNodes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows_core::Result<XmlDocument> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OwnerDocument)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlDocument>(result__)
        }
    }
    pub fn InsertBefore<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>, Param1: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InsertBefore)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), referencechild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ReplaceChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>, Param1: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReplaceChild)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), referencechild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn RemoveChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoveChild)(::windows_core::Interface::as_raw(this), childnode.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn AppendChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppendChild)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CloneNode)(::windows_core::Interface::as_raw(this), deep, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).NamespaceUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).LocalName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).Prefix)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Normalize)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPrefix)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, xpath: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNode)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodes<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, xpath: Param0) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodes)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn SelectSingleNodeNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNodeNS)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), namespaces.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodesNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodesNS)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), namespaces.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetXml)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).InnerText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetInnerText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetInnerText)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for XmlAttribute {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XmlAttribute {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XmlAttribute {}
impl ::core::fmt::Debug for XmlAttribute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlAttribute").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XmlAttribute {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlAttribute;{ac144aa4-b4f1-4db6-b206-8a22c308db0a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for XmlAttribute {
    type Vtable = IXmlAttribute_Vtbl;
    const IID: ::windows_core::GUID = <IXmlAttribute as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for XmlAttribute {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlAttribute";
}
impl ::core::convert::From<XmlAttribute> for ::windows_core::IUnknown {
    fn from(value: XmlAttribute) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlAttribute> for ::windows_core::IUnknown {
    fn from(value: &XmlAttribute) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for XmlAttribute {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a XmlAttribute {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XmlAttribute> for ::windows_core::IInspectable {
    fn from(value: XmlAttribute) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlAttribute> for ::windows_core::IInspectable {
    fn from(value: &XmlAttribute) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for XmlAttribute {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a XmlAttribute {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<XmlAttribute> for IXmlNode {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlAttribute) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlAttribute> for IXmlNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlAttribute) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNode> for XmlAttribute {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNode> for &XmlAttribute {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNode> {
        ::core::convert::TryInto::<IXmlNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlAttribute> for IXmlNodeSelector {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlAttribute) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlAttribute> for IXmlNodeSelector {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlAttribute) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSelector> for XmlAttribute {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSelector> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSelector> for &XmlAttribute {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSelector> {
        ::core::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlAttribute> for IXmlNodeSerializer {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlAttribute) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlAttribute> for IXmlNodeSerializer {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlAttribute) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSerializer> for XmlAttribute {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSerializer> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSerializer> for &XmlAttribute {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSerializer> {
        ::core::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for XmlAttribute {}
unsafe impl ::core::marker::Sync for XmlAttribute {}
#[repr(transparent)]
pub struct XmlCDataSection(::windows_core::IUnknown);
impl XmlCDataSection {
    pub fn Data(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetData)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Length(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Length)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SubstringData(&self, offset: u32, count: u32) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SubstringData)(::windows_core::Interface::as_raw(this), offset, count, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AppendData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, data: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AppendData)(::windows_core::Interface::as_raw(this), data.into_param().abi()).ok() }
    }
    pub fn InsertData<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, offset: u32, data: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).InsertData)(::windows_core::Interface::as_raw(this), offset, data.into_param().abi()).ok() }
    }
    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).DeleteData)(::windows_core::Interface::as_raw(this), offset, count).ok() }
    }
    pub fn ReplaceData<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, offset: u32, count: u32, data: Param2) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceData)(::windows_core::Interface::as_raw(this), offset, count, data.into_param().abi()).ok() }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn SetNodeValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows_core::Result<NodeType> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<NodeType>::zeroed();
            (::windows_core::Interface::vtable(this).NodeType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NodeType>(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ParentNode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ChildNodes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FirstChild)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LastChild)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousSibling)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NextSibling)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows_core::Result<XmlNamedNodeMap> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Attributes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasChildNodes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows_core::Result<XmlDocument> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OwnerDocument)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlDocument>(result__)
        }
    }
    pub fn InsertBefore<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>, Param1: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InsertBefore)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), referencechild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ReplaceChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>, Param1: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReplaceChild)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), referencechild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn RemoveChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoveChild)(::windows_core::Interface::as_raw(this), childnode.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn AppendChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppendChild)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CloneNode)(::windows_core::Interface::as_raw(this), deep, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).NamespaceUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).LocalName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).Prefix)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Normalize)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPrefix)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, xpath: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNode)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodes<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, xpath: Param0) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodes)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn SelectSingleNodeNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNodeNS)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), namespaces.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodesNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodesNS)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), namespaces.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetXml)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).InnerText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetInnerText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetInnerText)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SplitText(&self, offset: u32) -> ::windows_core::Result<IXmlText> {
        let this = &::windows_core::Interface::cast::<IXmlText>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplitText)(::windows_core::Interface::as_raw(this), offset, result__.as_mut_ptr()).from_abi::<IXmlText>(result__)
        }
    }
}
impl ::core::clone::Clone for XmlCDataSection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XmlCDataSection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XmlCDataSection {}
impl ::core::fmt::Debug for XmlCDataSection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlCDataSection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XmlCDataSection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlCDataSection;{4d04b46f-c8bd-45b4-8899-0400d7c2c60f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for XmlCDataSection {
    type Vtable = IXmlCDataSection_Vtbl;
    const IID: ::windows_core::GUID = <IXmlCDataSection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for XmlCDataSection {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlCDataSection";
}
impl ::core::convert::From<XmlCDataSection> for ::windows_core::IUnknown {
    fn from(value: XmlCDataSection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlCDataSection> for ::windows_core::IUnknown {
    fn from(value: &XmlCDataSection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for XmlCDataSection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a XmlCDataSection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XmlCDataSection> for ::windows_core::IInspectable {
    fn from(value: XmlCDataSection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlCDataSection> for ::windows_core::IInspectable {
    fn from(value: &XmlCDataSection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for XmlCDataSection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a XmlCDataSection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<XmlCDataSection> for IXmlCharacterData {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlCDataSection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlCDataSection> for IXmlCharacterData {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlCDataSection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlCharacterData> for XmlCDataSection {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlCharacterData> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlCharacterData> for &XmlCDataSection {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlCharacterData> {
        ::core::convert::TryInto::<IXmlCharacterData>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlCDataSection> for IXmlNode {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlCDataSection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlCDataSection> for IXmlNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlCDataSection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNode> for XmlCDataSection {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNode> for &XmlCDataSection {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNode> {
        ::core::convert::TryInto::<IXmlNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlCDataSection> for IXmlNodeSelector {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlCDataSection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlCDataSection> for IXmlNodeSelector {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlCDataSection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSelector> for XmlCDataSection {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSelector> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSelector> for &XmlCDataSection {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSelector> {
        ::core::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlCDataSection> for IXmlNodeSerializer {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlCDataSection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlCDataSection> for IXmlNodeSerializer {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlCDataSection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSerializer> for XmlCDataSection {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSerializer> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSerializer> for &XmlCDataSection {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSerializer> {
        ::core::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlCDataSection> for IXmlText {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlCDataSection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlCDataSection> for IXmlText {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlCDataSection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlText> for XmlCDataSection {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlText> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlText> for &XmlCDataSection {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlText> {
        ::core::convert::TryInto::<IXmlText>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for XmlCDataSection {}
unsafe impl ::core::marker::Sync for XmlCDataSection {}
#[repr(transparent)]
pub struct XmlComment(::windows_core::IUnknown);
impl XmlComment {
    pub fn Data(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetData)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Length(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Length)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SubstringData(&self, offset: u32, count: u32) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SubstringData)(::windows_core::Interface::as_raw(this), offset, count, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AppendData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, data: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AppendData)(::windows_core::Interface::as_raw(this), data.into_param().abi()).ok() }
    }
    pub fn InsertData<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, offset: u32, data: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).InsertData)(::windows_core::Interface::as_raw(this), offset, data.into_param().abi()).ok() }
    }
    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).DeleteData)(::windows_core::Interface::as_raw(this), offset, count).ok() }
    }
    pub fn ReplaceData<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, offset: u32, count: u32, data: Param2) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceData)(::windows_core::Interface::as_raw(this), offset, count, data.into_param().abi()).ok() }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn SetNodeValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows_core::Result<NodeType> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<NodeType>::zeroed();
            (::windows_core::Interface::vtable(this).NodeType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NodeType>(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ParentNode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ChildNodes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FirstChild)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LastChild)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousSibling)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NextSibling)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows_core::Result<XmlNamedNodeMap> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Attributes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasChildNodes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows_core::Result<XmlDocument> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OwnerDocument)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlDocument>(result__)
        }
    }
    pub fn InsertBefore<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>, Param1: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InsertBefore)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), referencechild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ReplaceChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>, Param1: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReplaceChild)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), referencechild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn RemoveChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoveChild)(::windows_core::Interface::as_raw(this), childnode.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn AppendChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppendChild)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CloneNode)(::windows_core::Interface::as_raw(this), deep, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).NamespaceUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).LocalName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).Prefix)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Normalize)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPrefix)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, xpath: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNode)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodes<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, xpath: Param0) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodes)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn SelectSingleNodeNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNodeNS)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), namespaces.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodesNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodesNS)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), namespaces.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetXml)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).InnerText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetInnerText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetInnerText)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for XmlComment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XmlComment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XmlComment {}
impl ::core::fmt::Debug for XmlComment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlComment").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XmlComment {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlComment;{bca474d5-b61f-4611-9cac-2e92e3476d47})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for XmlComment {
    type Vtable = IXmlComment_Vtbl;
    const IID: ::windows_core::GUID = <IXmlComment as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for XmlComment {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlComment";
}
impl ::core::convert::From<XmlComment> for ::windows_core::IUnknown {
    fn from(value: XmlComment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlComment> for ::windows_core::IUnknown {
    fn from(value: &XmlComment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for XmlComment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a XmlComment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XmlComment> for ::windows_core::IInspectable {
    fn from(value: XmlComment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlComment> for ::windows_core::IInspectable {
    fn from(value: &XmlComment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for XmlComment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a XmlComment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<XmlComment> for IXmlCharacterData {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlComment) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlComment> for IXmlCharacterData {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlComment) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlCharacterData> for XmlComment {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlCharacterData> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlCharacterData> for &XmlComment {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlCharacterData> {
        ::core::convert::TryInto::<IXmlCharacterData>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlComment> for IXmlNode {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlComment) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlComment> for IXmlNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlComment) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNode> for XmlComment {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNode> for &XmlComment {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNode> {
        ::core::convert::TryInto::<IXmlNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlComment> for IXmlNodeSelector {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlComment) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlComment> for IXmlNodeSelector {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlComment) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSelector> for XmlComment {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSelector> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSelector> for &XmlComment {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSelector> {
        ::core::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlComment> for IXmlNodeSerializer {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlComment) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlComment> for IXmlNodeSerializer {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlComment) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSerializer> for XmlComment {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSerializer> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSerializer> for &XmlComment {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSerializer> {
        ::core::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for XmlComment {}
unsafe impl ::core::marker::Sync for XmlComment {}
#[repr(transparent)]
pub struct XmlDocument(::windows_core::IUnknown);
impl XmlDocument {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<XmlDocument, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Doctype(&self) -> ::windows_core::Result<XmlDocumentType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Doctype)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlDocumentType>(result__)
        }
    }
    pub fn Implementation(&self) -> ::windows_core::Result<XmlDomImplementation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Implementation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlDomImplementation>(result__)
        }
    }
    pub fn DocumentElement(&self) -> ::windows_core::Result<XmlElement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DocumentElement)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlElement>(result__)
        }
    }
    pub fn CreateElement<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, tagname: Param0) -> ::windows_core::Result<XmlElement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateElement)(::windows_core::Interface::as_raw(this), tagname.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlElement>(result__)
        }
    }
    pub fn CreateDocumentFragment(&self) -> ::windows_core::Result<XmlDocumentFragment> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateDocumentFragment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlDocumentFragment>(result__)
        }
    }
    pub fn CreateTextNode<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, data: Param0) -> ::windows_core::Result<XmlText> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateTextNode)(::windows_core::Interface::as_raw(this), data.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlText>(result__)
        }
    }
    pub fn CreateComment<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, data: Param0) -> ::windows_core::Result<XmlComment> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateComment)(::windows_core::Interface::as_raw(this), data.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlComment>(result__)
        }
    }
    pub fn CreateProcessingInstruction<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, target: Param0, data: Param1) -> ::windows_core::Result<XmlProcessingInstruction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateProcessingInstruction)(::windows_core::Interface::as_raw(this), target.into_param().abi(), data.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlProcessingInstruction>(result__)
        }
    }
    pub fn CreateAttribute<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0) -> ::windows_core::Result<XmlAttribute> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateAttribute)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlAttribute>(result__)
        }
    }
    pub fn CreateEntityReference<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0) -> ::windows_core::Result<XmlEntityReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateEntityReference)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlEntityReference>(result__)
        }
    }
    pub fn GetElementsByTagName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, tagname: Param0) -> ::windows_core::Result<XmlNodeList> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetElementsByTagName)(::windows_core::Interface::as_raw(this), tagname.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn CreateCDataSection<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, data: Param0) -> ::windows_core::Result<XmlCDataSection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateCDataSection)(::windows_core::Interface::as_raw(this), data.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlCDataSection>(result__)
        }
    }
    pub fn DocumentUri(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DocumentUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn CreateAttributeNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, namespaceuri: Param0, qualifiedname: Param1) -> ::windows_core::Result<XmlAttribute> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateAttributeNS)(::windows_core::Interface::as_raw(this), namespaceuri.into_param().abi(), qualifiedname.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlAttribute>(result__)
        }
    }
    pub fn CreateElementNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, namespaceuri: Param0, qualifiedname: Param1) -> ::windows_core::Result<XmlElement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateElementNS)(::windows_core::Interface::as_raw(this), namespaceuri.into_param().abi(), qualifiedname.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlElement>(result__)
        }
    }
    pub fn GetElementById<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, elementid: Param0) -> ::windows_core::Result<XmlElement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetElementById)(::windows_core::Interface::as_raw(this), elementid.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlElement>(result__)
        }
    }
    pub fn ImportNode<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>>(&self, node: Param0, deep: bool) -> ::windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImportNode)(::windows_core::Interface::as_raw(this), node.into_param().abi(), deep, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn LoadXml<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, xml: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlDocumentIO>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).LoadXml)(::windows_core::Interface::as_raw(this), xml.into_param().abi()).ok() }
    }
    pub fn LoadXmlWithSettings<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, XmlLoadSettings>>(&self, xml: Param0, loadsettings: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlDocumentIO>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).LoadXmlWithSettings)(::windows_core::Interface::as_raw(this), xml.into_param().abi(), loadsettings.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn SaveToFileAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageFile>>(&self, file: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IXmlDocumentIO>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SaveToFileAsync)(::windows_core::Interface::as_raw(this), file.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn LoadXmlFromBuffer<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(&self, buffer: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlDocumentIO2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).LoadXmlFromBuffer)(::windows_core::Interface::as_raw(this), buffer.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn LoadXmlFromBufferWithSettings<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>, Param1: ::windows_core::IntoParam<'a, XmlLoadSettings>>(&self, buffer: Param0, loadsettings: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlDocumentIO2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).LoadXmlFromBufferWithSettings)(::windows_core::Interface::as_raw(this), buffer.into_param().abi(), loadsettings.into_param().abi()).ok() }
    }
    pub fn LoadFromUriAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(uri: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<XmlDocument>> {
        Self::IXmlDocumentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LoadFromUriAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<XmlDocument>>(result__)
        })
    }
    pub fn LoadFromUriWithSettingsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param1: ::windows_core::IntoParam<'a, XmlLoadSettings>>(uri: Param0, loadsettings: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<XmlDocument>> {
        Self::IXmlDocumentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LoadFromUriWithSettingsAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), loadsettings.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<XmlDocument>>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn LoadFromFileAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageFile>>(file: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<XmlDocument>> {
        Self::IXmlDocumentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LoadFromFileAsync)(::windows_core::Interface::as_raw(this), file.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<XmlDocument>>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn LoadFromFileWithSettingsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageFile>, Param1: ::windows_core::IntoParam<'a, XmlLoadSettings>>(file: Param0, loadsettings: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<XmlDocument>> {
        Self::IXmlDocumentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LoadFromFileWithSettingsAsync)(::windows_core::Interface::as_raw(this), file.into_param().abi(), loadsettings.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<XmlDocument>>(result__)
        })
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn SetNodeValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows_core::Result<NodeType> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<NodeType>::zeroed();
            (::windows_core::Interface::vtable(this).NodeType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NodeType>(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ParentNode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ChildNodes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FirstChild)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LastChild)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousSibling)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NextSibling)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows_core::Result<XmlNamedNodeMap> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Attributes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasChildNodes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows_core::Result<XmlDocument> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OwnerDocument)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlDocument>(result__)
        }
    }
    pub fn InsertBefore<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>, Param1: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InsertBefore)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), referencechild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ReplaceChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>, Param1: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReplaceChild)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), referencechild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn RemoveChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoveChild)(::windows_core::Interface::as_raw(this), childnode.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn AppendChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppendChild)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CloneNode)(::windows_core::Interface::as_raw(this), deep, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).NamespaceUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).LocalName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).Prefix)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Normalize)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPrefix)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, xpath: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNode)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodes<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, xpath: Param0) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodes)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn SelectSingleNodeNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNodeNS)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), namespaces.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodesNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodesNS)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), namespaces.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetXml)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).InnerText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetInnerText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetInnerText)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IXmlDocumentStatics<R, F: FnOnce(&IXmlDocumentStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<XmlDocument, IXmlDocumentStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for XmlDocument {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XmlDocument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XmlDocument {}
impl ::core::fmt::Debug for XmlDocument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlDocument").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XmlDocument {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlDocument;{f7f3a506-1e87-42d6-bcfb-b8c809fa5494})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for XmlDocument {
    type Vtable = IXmlDocument_Vtbl;
    const IID: ::windows_core::GUID = <IXmlDocument as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for XmlDocument {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlDocument";
}
impl ::core::convert::From<XmlDocument> for ::windows_core::IUnknown {
    fn from(value: XmlDocument) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlDocument> for ::windows_core::IUnknown {
    fn from(value: &XmlDocument) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for XmlDocument {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a XmlDocument {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XmlDocument> for ::windows_core::IInspectable {
    fn from(value: XmlDocument) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlDocument> for ::windows_core::IInspectable {
    fn from(value: &XmlDocument) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for XmlDocument {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a XmlDocument {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<XmlDocument> for IXmlNode {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlDocument) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlDocument> for IXmlNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlDocument) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNode> for XmlDocument {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNode> for &XmlDocument {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNode> {
        ::core::convert::TryInto::<IXmlNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlDocument> for IXmlNodeSelector {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlDocument) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlDocument> for IXmlNodeSelector {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlDocument) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSelector> for XmlDocument {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSelector> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSelector> for &XmlDocument {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSelector> {
        ::core::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlDocument> for IXmlNodeSerializer {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlDocument) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlDocument> for IXmlNodeSerializer {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlDocument) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSerializer> for XmlDocument {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSerializer> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSerializer> for &XmlDocument {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSerializer> {
        ::core::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for XmlDocument {}
unsafe impl ::core::marker::Sync for XmlDocument {}
#[repr(transparent)]
pub struct XmlDocumentFragment(::windows_core::IUnknown);
impl XmlDocumentFragment {
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn SetNodeValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows_core::Result<NodeType> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<NodeType>::zeroed();
            (::windows_core::Interface::vtable(this).NodeType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NodeType>(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ParentNode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ChildNodes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FirstChild)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LastChild)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousSibling)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NextSibling)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows_core::Result<XmlNamedNodeMap> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Attributes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasChildNodes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows_core::Result<XmlDocument> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OwnerDocument)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlDocument>(result__)
        }
    }
    pub fn InsertBefore<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>, Param1: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InsertBefore)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), referencechild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ReplaceChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>, Param1: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReplaceChild)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), referencechild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn RemoveChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoveChild)(::windows_core::Interface::as_raw(this), childnode.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn AppendChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppendChild)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CloneNode)(::windows_core::Interface::as_raw(this), deep, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).NamespaceUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).LocalName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).Prefix)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Normalize)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPrefix)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, xpath: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNode)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodes<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, xpath: Param0) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodes)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn SelectSingleNodeNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNodeNS)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), namespaces.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodesNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodesNS)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), namespaces.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetXml)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).InnerText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetInnerText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetInnerText)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for XmlDocumentFragment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XmlDocumentFragment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XmlDocumentFragment {}
impl ::core::fmt::Debug for XmlDocumentFragment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlDocumentFragment").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XmlDocumentFragment {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlDocumentFragment;{e2ea6a96-0c21-44a5-8bc9-9e4a262708ec})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for XmlDocumentFragment {
    type Vtable = IXmlDocumentFragment_Vtbl;
    const IID: ::windows_core::GUID = <IXmlDocumentFragment as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for XmlDocumentFragment {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlDocumentFragment";
}
impl ::core::convert::From<XmlDocumentFragment> for ::windows_core::IUnknown {
    fn from(value: XmlDocumentFragment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlDocumentFragment> for ::windows_core::IUnknown {
    fn from(value: &XmlDocumentFragment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for XmlDocumentFragment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a XmlDocumentFragment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XmlDocumentFragment> for ::windows_core::IInspectable {
    fn from(value: XmlDocumentFragment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlDocumentFragment> for ::windows_core::IInspectable {
    fn from(value: &XmlDocumentFragment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for XmlDocumentFragment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a XmlDocumentFragment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<XmlDocumentFragment> for IXmlNode {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlDocumentFragment) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlDocumentFragment> for IXmlNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlDocumentFragment) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNode> for XmlDocumentFragment {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNode> for &XmlDocumentFragment {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNode> {
        ::core::convert::TryInto::<IXmlNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlDocumentFragment> for IXmlNodeSelector {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlDocumentFragment) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlDocumentFragment> for IXmlNodeSelector {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlDocumentFragment) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSelector> for XmlDocumentFragment {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSelector> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSelector> for &XmlDocumentFragment {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSelector> {
        ::core::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlDocumentFragment> for IXmlNodeSerializer {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlDocumentFragment) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlDocumentFragment> for IXmlNodeSerializer {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlDocumentFragment) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSerializer> for XmlDocumentFragment {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSerializer> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSerializer> for &XmlDocumentFragment {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSerializer> {
        ::core::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for XmlDocumentFragment {}
unsafe impl ::core::marker::Sync for XmlDocumentFragment {}
#[repr(transparent)]
pub struct XmlDocumentType(::windows_core::IUnknown);
impl XmlDocumentType {
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Entities(&self) -> ::windows_core::Result<XmlNamedNodeMap> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Entities)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    pub fn Notations(&self) -> ::windows_core::Result<XmlNamedNodeMap> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Notations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn SetNodeValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows_core::Result<NodeType> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<NodeType>::zeroed();
            (::windows_core::Interface::vtable(this).NodeType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NodeType>(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ParentNode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ChildNodes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FirstChild)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LastChild)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousSibling)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NextSibling)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows_core::Result<XmlNamedNodeMap> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Attributes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasChildNodes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows_core::Result<XmlDocument> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OwnerDocument)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlDocument>(result__)
        }
    }
    pub fn InsertBefore<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>, Param1: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InsertBefore)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), referencechild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ReplaceChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>, Param1: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReplaceChild)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), referencechild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn RemoveChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoveChild)(::windows_core::Interface::as_raw(this), childnode.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn AppendChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppendChild)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CloneNode)(::windows_core::Interface::as_raw(this), deep, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).NamespaceUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).LocalName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).Prefix)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Normalize)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPrefix)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, xpath: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNode)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodes<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, xpath: Param0) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodes)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn SelectSingleNodeNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNodeNS)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), namespaces.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodesNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodesNS)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), namespaces.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetXml)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).InnerText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetInnerText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetInnerText)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for XmlDocumentType {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XmlDocumentType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XmlDocumentType {}
impl ::core::fmt::Debug for XmlDocumentType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlDocumentType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XmlDocumentType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlDocumentType;{f7342425-9781-4964-8e94-9b1c6dfc9bc7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for XmlDocumentType {
    type Vtable = IXmlDocumentType_Vtbl;
    const IID: ::windows_core::GUID = <IXmlDocumentType as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for XmlDocumentType {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlDocumentType";
}
impl ::core::convert::From<XmlDocumentType> for ::windows_core::IUnknown {
    fn from(value: XmlDocumentType) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlDocumentType> for ::windows_core::IUnknown {
    fn from(value: &XmlDocumentType) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for XmlDocumentType {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a XmlDocumentType {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XmlDocumentType> for ::windows_core::IInspectable {
    fn from(value: XmlDocumentType) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlDocumentType> for ::windows_core::IInspectable {
    fn from(value: &XmlDocumentType) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for XmlDocumentType {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a XmlDocumentType {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<XmlDocumentType> for IXmlNode {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlDocumentType) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlDocumentType> for IXmlNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlDocumentType) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNode> for XmlDocumentType {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNode> for &XmlDocumentType {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNode> {
        ::core::convert::TryInto::<IXmlNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlDocumentType> for IXmlNodeSelector {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlDocumentType) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlDocumentType> for IXmlNodeSelector {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlDocumentType) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSelector> for XmlDocumentType {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSelector> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSelector> for &XmlDocumentType {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSelector> {
        ::core::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlDocumentType> for IXmlNodeSerializer {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlDocumentType) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlDocumentType> for IXmlNodeSerializer {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlDocumentType) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSerializer> for XmlDocumentType {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSerializer> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSerializer> for &XmlDocumentType {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSerializer> {
        ::core::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for XmlDocumentType {}
unsafe impl ::core::marker::Sync for XmlDocumentType {}
#[repr(transparent)]
pub struct XmlDomImplementation(::windows_core::IUnknown);
impl XmlDomImplementation {
    pub fn HasFeature<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, feature: Param0, version: Param1) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasFeature)(::windows_core::Interface::as_raw(this), feature.into_param().abi(), version.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for XmlDomImplementation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XmlDomImplementation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XmlDomImplementation {}
impl ::core::fmt::Debug for XmlDomImplementation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlDomImplementation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XmlDomImplementation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlDomImplementation;{6de58132-f11d-4fbb-8cc6-583cba93112f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for XmlDomImplementation {
    type Vtable = IXmlDomImplementation_Vtbl;
    const IID: ::windows_core::GUID = <IXmlDomImplementation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for XmlDomImplementation {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlDomImplementation";
}
impl ::core::convert::From<XmlDomImplementation> for ::windows_core::IUnknown {
    fn from(value: XmlDomImplementation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlDomImplementation> for ::windows_core::IUnknown {
    fn from(value: &XmlDomImplementation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for XmlDomImplementation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a XmlDomImplementation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XmlDomImplementation> for ::windows_core::IInspectable {
    fn from(value: XmlDomImplementation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlDomImplementation> for ::windows_core::IInspectable {
    fn from(value: &XmlDomImplementation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for XmlDomImplementation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a XmlDomImplementation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for XmlDomImplementation {}
unsafe impl ::core::marker::Sync for XmlDomImplementation {}
#[repr(transparent)]
pub struct XmlElement(::windows_core::IUnknown);
impl XmlElement {
    pub fn TagName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TagName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn GetAttribute<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, attributename: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetAttribute)(::windows_core::Interface::as_raw(this), attributename.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetAttribute<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, attributename: Param0, attributevalue: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAttribute)(::windows_core::Interface::as_raw(this), attributename.into_param().abi(), attributevalue.into_param().abi()).ok() }
    }
    pub fn RemoveAttribute<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, attributename: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAttribute)(::windows_core::Interface::as_raw(this), attributename.into_param().abi()).ok() }
    }
    pub fn GetAttributeNode<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, attributename: Param0) -> ::windows_core::Result<XmlAttribute> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAttributeNode)(::windows_core::Interface::as_raw(this), attributename.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlAttribute>(result__)
        }
    }
    pub fn SetAttributeNode<'a, Param0: ::windows_core::IntoParam<'a, XmlAttribute>>(&self, newattribute: Param0) -> ::windows_core::Result<XmlAttribute> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetAttributeNode)(::windows_core::Interface::as_raw(this), newattribute.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlAttribute>(result__)
        }
    }
    pub fn RemoveAttributeNode<'a, Param0: ::windows_core::IntoParam<'a, XmlAttribute>>(&self, attributenode: Param0) -> ::windows_core::Result<XmlAttribute> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoveAttributeNode)(::windows_core::Interface::as_raw(this), attributenode.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlAttribute>(result__)
        }
    }
    pub fn GetElementsByTagName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, tagname: Param0) -> ::windows_core::Result<XmlNodeList> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetElementsByTagName)(::windows_core::Interface::as_raw(this), tagname.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn SetAttributeNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, namespaceuri: Param0, qualifiedname: Param1, value: Param2) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAttributeNS)(::windows_core::Interface::as_raw(this), namespaceuri.into_param().abi(), qualifiedname.into_param().abi(), value.into_param().abi()).ok() }
    }
    pub fn GetAttributeNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, namespaceuri: Param0, localname: Param1) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetAttributeNS)(::windows_core::Interface::as_raw(this), namespaceuri.into_param().abi(), localname.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn RemoveAttributeNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, namespaceuri: Param0, localname: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAttributeNS)(::windows_core::Interface::as_raw(this), namespaceuri.into_param().abi(), localname.into_param().abi()).ok() }
    }
    pub fn SetAttributeNodeNS<'a, Param0: ::windows_core::IntoParam<'a, XmlAttribute>>(&self, newattribute: Param0) -> ::windows_core::Result<XmlAttribute> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetAttributeNodeNS)(::windows_core::Interface::as_raw(this), newattribute.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlAttribute>(result__)
        }
    }
    pub fn GetAttributeNodeNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, namespaceuri: Param0, localname: Param1) -> ::windows_core::Result<XmlAttribute> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAttributeNodeNS)(::windows_core::Interface::as_raw(this), namespaceuri.into_param().abi(), localname.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlAttribute>(result__)
        }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn SetNodeValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows_core::Result<NodeType> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<NodeType>::zeroed();
            (::windows_core::Interface::vtable(this).NodeType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NodeType>(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ParentNode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ChildNodes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FirstChild)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LastChild)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousSibling)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NextSibling)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows_core::Result<XmlNamedNodeMap> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Attributes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasChildNodes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows_core::Result<XmlDocument> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OwnerDocument)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlDocument>(result__)
        }
    }
    pub fn InsertBefore<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>, Param1: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InsertBefore)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), referencechild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ReplaceChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>, Param1: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReplaceChild)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), referencechild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn RemoveChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoveChild)(::windows_core::Interface::as_raw(this), childnode.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn AppendChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppendChild)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CloneNode)(::windows_core::Interface::as_raw(this), deep, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).NamespaceUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).LocalName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).Prefix)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Normalize)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPrefix)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, xpath: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNode)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodes<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, xpath: Param0) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodes)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn SelectSingleNodeNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNodeNS)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), namespaces.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodesNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodesNS)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), namespaces.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetXml)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).InnerText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetInnerText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetInnerText)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for XmlElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XmlElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XmlElement {}
impl ::core::fmt::Debug for XmlElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlElement").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XmlElement {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlElement;{2dfb8a1f-6b10-4ef8-9f83-efcce8faec37})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for XmlElement {
    type Vtable = IXmlElement_Vtbl;
    const IID: ::windows_core::GUID = <IXmlElement as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for XmlElement {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlElement";
}
impl ::core::convert::From<XmlElement> for ::windows_core::IUnknown {
    fn from(value: XmlElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlElement> for ::windows_core::IUnknown {
    fn from(value: &XmlElement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for XmlElement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a XmlElement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XmlElement> for ::windows_core::IInspectable {
    fn from(value: XmlElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlElement> for ::windows_core::IInspectable {
    fn from(value: &XmlElement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for XmlElement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a XmlElement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<XmlElement> for IXmlNode {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlElement) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlElement> for IXmlNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlElement) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNode> for XmlElement {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNode> for &XmlElement {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNode> {
        ::core::convert::TryInto::<IXmlNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlElement> for IXmlNodeSelector {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlElement) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlElement> for IXmlNodeSelector {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlElement) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSelector> for XmlElement {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSelector> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSelector> for &XmlElement {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSelector> {
        ::core::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlElement> for IXmlNodeSerializer {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlElement) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlElement> for IXmlNodeSerializer {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlElement) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSerializer> for XmlElement {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSerializer> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSerializer> for &XmlElement {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSerializer> {
        ::core::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for XmlElement {}
unsafe impl ::core::marker::Sync for XmlElement {}
#[repr(transparent)]
pub struct XmlEntityReference(::windows_core::IUnknown);
impl XmlEntityReference {
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn SetNodeValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows_core::Result<NodeType> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<NodeType>::zeroed();
            (::windows_core::Interface::vtable(this).NodeType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NodeType>(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ParentNode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ChildNodes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FirstChild)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LastChild)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousSibling)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NextSibling)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows_core::Result<XmlNamedNodeMap> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Attributes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasChildNodes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows_core::Result<XmlDocument> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OwnerDocument)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlDocument>(result__)
        }
    }
    pub fn InsertBefore<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>, Param1: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InsertBefore)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), referencechild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ReplaceChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>, Param1: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReplaceChild)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), referencechild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn RemoveChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoveChild)(::windows_core::Interface::as_raw(this), childnode.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn AppendChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppendChild)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CloneNode)(::windows_core::Interface::as_raw(this), deep, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).NamespaceUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).LocalName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).Prefix)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Normalize)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPrefix)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, xpath: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNode)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodes<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, xpath: Param0) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodes)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn SelectSingleNodeNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNodeNS)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), namespaces.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodesNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodesNS)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), namespaces.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetXml)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).InnerText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetInnerText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetInnerText)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for XmlEntityReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XmlEntityReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XmlEntityReference {}
impl ::core::fmt::Debug for XmlEntityReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlEntityReference").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XmlEntityReference {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlEntityReference;{2e2f47bc-c3d0-4ccf-bb86-0ab8c36a61cf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for XmlEntityReference {
    type Vtable = IXmlEntityReference_Vtbl;
    const IID: ::windows_core::GUID = <IXmlEntityReference as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for XmlEntityReference {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlEntityReference";
}
impl ::core::convert::From<XmlEntityReference> for ::windows_core::IUnknown {
    fn from(value: XmlEntityReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlEntityReference> for ::windows_core::IUnknown {
    fn from(value: &XmlEntityReference) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for XmlEntityReference {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a XmlEntityReference {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XmlEntityReference> for ::windows_core::IInspectable {
    fn from(value: XmlEntityReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlEntityReference> for ::windows_core::IInspectable {
    fn from(value: &XmlEntityReference) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for XmlEntityReference {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a XmlEntityReference {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<XmlEntityReference> for IXmlNode {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlEntityReference) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlEntityReference> for IXmlNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlEntityReference) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNode> for XmlEntityReference {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNode> for &XmlEntityReference {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNode> {
        ::core::convert::TryInto::<IXmlNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlEntityReference> for IXmlNodeSelector {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlEntityReference) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlEntityReference> for IXmlNodeSelector {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlEntityReference) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSelector> for XmlEntityReference {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSelector> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSelector> for &XmlEntityReference {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSelector> {
        ::core::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlEntityReference> for IXmlNodeSerializer {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlEntityReference) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlEntityReference> for IXmlNodeSerializer {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlEntityReference) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSerializer> for XmlEntityReference {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSerializer> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSerializer> for &XmlEntityReference {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSerializer> {
        ::core::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for XmlEntityReference {}
unsafe impl ::core::marker::Sync for XmlEntityReference {}
#[repr(transparent)]
pub struct XmlLoadSettings(::windows_core::IUnknown);
impl XmlLoadSettings {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<XmlLoadSettings, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn MaxElementDepth(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxElementDepth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetMaxElementDepth(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxElementDepth)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ProhibitDtd(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ProhibitDtd)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetProhibitDtd(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetProhibitDtd)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ResolveExternals(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ResolveExternals)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetResolveExternals(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetResolveExternals)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ValidateOnParse(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ValidateOnParse)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetValidateOnParse(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetValidateOnParse)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ElementContentWhiteSpace(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ElementContentWhiteSpace)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetElementContentWhiteSpace(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetElementContentWhiteSpace)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for XmlLoadSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XmlLoadSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XmlLoadSettings {}
impl ::core::fmt::Debug for XmlLoadSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlLoadSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XmlLoadSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlLoadSettings;{58aa07a8-fed6-46f7-b4c5-fb1ba72108d6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for XmlLoadSettings {
    type Vtable = IXmlLoadSettings_Vtbl;
    const IID: ::windows_core::GUID = <IXmlLoadSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for XmlLoadSettings {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlLoadSettings";
}
impl ::core::convert::From<XmlLoadSettings> for ::windows_core::IUnknown {
    fn from(value: XmlLoadSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlLoadSettings> for ::windows_core::IUnknown {
    fn from(value: &XmlLoadSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for XmlLoadSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a XmlLoadSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XmlLoadSettings> for ::windows_core::IInspectable {
    fn from(value: XmlLoadSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlLoadSettings> for ::windows_core::IInspectable {
    fn from(value: &XmlLoadSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for XmlLoadSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a XmlLoadSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for XmlLoadSettings {}
unsafe impl ::core::marker::Sync for XmlLoadSettings {}
#[repr(transparent)]
pub struct XmlNamedNodeMap(::windows_core::IUnknown);
impl XmlNamedNodeMap {
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<IXmlNode>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<IXmlNode>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<IXmlNode>]) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Length(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Length)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Item(&self, index: u32) -> ::windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Item)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn GetNamedItem<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetNamedItem)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SetNamedItem<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>>(&self, node: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetNamedItem)(::windows_core::Interface::as_raw(this), node.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn RemoveNamedItem<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoveNamedItem)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn GetNamedItemNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, namespaceuri: Param0, name: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetNamedItemNS)(::windows_core::Interface::as_raw(this), namespaceuri.into_param().abi(), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn RemoveNamedItemNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, namespaceuri: Param0, name: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoveNamedItemNS)(::windows_core::Interface::as_raw(this), namespaceuri.into_param().abi(), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SetNamedItemNS<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>>(&self, node: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetNamedItemNS)(::windows_core::Interface::as_raw(this), node.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
}
impl ::core::clone::Clone for XmlNamedNodeMap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XmlNamedNodeMap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XmlNamedNodeMap {}
impl ::core::fmt::Debug for XmlNamedNodeMap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlNamedNodeMap").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XmlNamedNodeMap {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlNamedNodeMap;{b3a69eb0-aab0-4b82-a6fa-b1453f7c021b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for XmlNamedNodeMap {
    type Vtable = IXmlNamedNodeMap_Vtbl;
    const IID: ::windows_core::GUID = <IXmlNamedNodeMap as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for XmlNamedNodeMap {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlNamedNodeMap";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for XmlNamedNodeMap {
    type Item = IXmlNode;
    type IntoIter = ::winrt_foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &XmlNamedNodeMap {
    type Item = IXmlNode;
    type IntoIter = ::winrt_foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
impl ::core::convert::From<XmlNamedNodeMap> for ::windows_core::IUnknown {
    fn from(value: XmlNamedNodeMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlNamedNodeMap> for ::windows_core::IUnknown {
    fn from(value: &XmlNamedNodeMap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for XmlNamedNodeMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a XmlNamedNodeMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XmlNamedNodeMap> for ::windows_core::IInspectable {
    fn from(value: XmlNamedNodeMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlNamedNodeMap> for ::windows_core::IInspectable {
    fn from(value: &XmlNamedNodeMap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for XmlNamedNodeMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a XmlNamedNodeMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<XmlNamedNodeMap> for ::winrt_foundation::Collections::IIterable<IXmlNode> {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlNamedNodeMap) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&XmlNamedNodeMap> for ::winrt_foundation::Collections::IIterable<IXmlNode> {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlNamedNodeMap) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<IXmlNode>> for XmlNamedNodeMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<IXmlNode>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<IXmlNode>> for &XmlNamedNodeMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<IXmlNode>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<IXmlNode>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<XmlNamedNodeMap> for ::winrt_foundation::Collections::IVectorView<IXmlNode> {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlNamedNodeMap) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&XmlNamedNodeMap> for ::winrt_foundation::Collections::IVectorView<IXmlNode> {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlNamedNodeMap) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVectorView<IXmlNode>> for XmlNamedNodeMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVectorView<IXmlNode>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVectorView<IXmlNode>> for &XmlNamedNodeMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVectorView<IXmlNode>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVectorView<IXmlNode>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for XmlNamedNodeMap {}
unsafe impl ::core::marker::Sync for XmlNamedNodeMap {}
#[repr(transparent)]
pub struct XmlNodeList(::windows_core::IUnknown);
impl XmlNodeList {
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<IXmlNode>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<IXmlNode>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<IXmlNode>]) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Length(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Length)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Item(&self, index: u32) -> ::windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Item)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
}
impl ::core::clone::Clone for XmlNodeList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XmlNodeList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XmlNodeList {}
impl ::core::fmt::Debug for XmlNodeList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlNodeList").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XmlNodeList {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlNodeList;{8c60ad77-83a4-4ec1-9c54-7ba429e13da6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for XmlNodeList {
    type Vtable = IXmlNodeList_Vtbl;
    const IID: ::windows_core::GUID = <IXmlNodeList as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for XmlNodeList {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlNodeList";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for XmlNodeList {
    type Item = IXmlNode;
    type IntoIter = ::winrt_foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &XmlNodeList {
    type Item = IXmlNode;
    type IntoIter = ::winrt_foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
impl ::core::convert::From<XmlNodeList> for ::windows_core::IUnknown {
    fn from(value: XmlNodeList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlNodeList> for ::windows_core::IUnknown {
    fn from(value: &XmlNodeList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for XmlNodeList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a XmlNodeList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XmlNodeList> for ::windows_core::IInspectable {
    fn from(value: XmlNodeList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlNodeList> for ::windows_core::IInspectable {
    fn from(value: &XmlNodeList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for XmlNodeList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a XmlNodeList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<XmlNodeList> for ::winrt_foundation::Collections::IIterable<IXmlNode> {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlNodeList) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&XmlNodeList> for ::winrt_foundation::Collections::IIterable<IXmlNode> {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlNodeList) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<IXmlNode>> for XmlNodeList {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<IXmlNode>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<IXmlNode>> for &XmlNodeList {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<IXmlNode>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<IXmlNode>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<XmlNodeList> for ::winrt_foundation::Collections::IVectorView<IXmlNode> {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlNodeList) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&XmlNodeList> for ::winrt_foundation::Collections::IVectorView<IXmlNode> {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlNodeList) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVectorView<IXmlNode>> for XmlNodeList {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVectorView<IXmlNode>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVectorView<IXmlNode>> for &XmlNodeList {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVectorView<IXmlNode>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVectorView<IXmlNode>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for XmlNodeList {}
unsafe impl ::core::marker::Sync for XmlNodeList {}
#[repr(transparent)]
pub struct XmlProcessingInstruction(::windows_core::IUnknown);
impl XmlProcessingInstruction {
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn SetNodeValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows_core::Result<NodeType> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<NodeType>::zeroed();
            (::windows_core::Interface::vtable(this).NodeType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NodeType>(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ParentNode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ChildNodes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FirstChild)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LastChild)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousSibling)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NextSibling)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows_core::Result<XmlNamedNodeMap> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Attributes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasChildNodes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows_core::Result<XmlDocument> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OwnerDocument)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlDocument>(result__)
        }
    }
    pub fn InsertBefore<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>, Param1: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InsertBefore)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), referencechild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ReplaceChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>, Param1: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReplaceChild)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), referencechild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn RemoveChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoveChild)(::windows_core::Interface::as_raw(this), childnode.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn AppendChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppendChild)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CloneNode)(::windows_core::Interface::as_raw(this), deep, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).NamespaceUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).LocalName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).Prefix)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Normalize)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPrefix)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, xpath: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNode)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodes<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, xpath: Param0) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodes)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn SelectSingleNodeNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNodeNS)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), namespaces.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodesNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodesNS)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), namespaces.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetXml)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).InnerText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetInnerText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetInnerText)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Target(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Target)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Data(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetData)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for XmlProcessingInstruction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XmlProcessingInstruction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XmlProcessingInstruction {}
impl ::core::fmt::Debug for XmlProcessingInstruction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlProcessingInstruction").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XmlProcessingInstruction {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlProcessingInstruction;{2707fd1e-1e92-4ece-b6f4-26f069078ddc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for XmlProcessingInstruction {
    type Vtable = IXmlProcessingInstruction_Vtbl;
    const IID: ::windows_core::GUID = <IXmlProcessingInstruction as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for XmlProcessingInstruction {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlProcessingInstruction";
}
impl ::core::convert::From<XmlProcessingInstruction> for ::windows_core::IUnknown {
    fn from(value: XmlProcessingInstruction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlProcessingInstruction> for ::windows_core::IUnknown {
    fn from(value: &XmlProcessingInstruction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for XmlProcessingInstruction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a XmlProcessingInstruction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XmlProcessingInstruction> for ::windows_core::IInspectable {
    fn from(value: XmlProcessingInstruction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlProcessingInstruction> for ::windows_core::IInspectable {
    fn from(value: &XmlProcessingInstruction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for XmlProcessingInstruction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a XmlProcessingInstruction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<XmlProcessingInstruction> for IXmlNode {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlProcessingInstruction) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlProcessingInstruction> for IXmlNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlProcessingInstruction) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNode> for XmlProcessingInstruction {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNode> for &XmlProcessingInstruction {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNode> {
        ::core::convert::TryInto::<IXmlNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlProcessingInstruction> for IXmlNodeSelector {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlProcessingInstruction) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlProcessingInstruction> for IXmlNodeSelector {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlProcessingInstruction) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSelector> for XmlProcessingInstruction {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSelector> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSelector> for &XmlProcessingInstruction {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSelector> {
        ::core::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlProcessingInstruction> for IXmlNodeSerializer {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlProcessingInstruction) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlProcessingInstruction> for IXmlNodeSerializer {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlProcessingInstruction) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSerializer> for XmlProcessingInstruction {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSerializer> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSerializer> for &XmlProcessingInstruction {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSerializer> {
        ::core::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for XmlProcessingInstruction {}
unsafe impl ::core::marker::Sync for XmlProcessingInstruction {}
#[repr(transparent)]
pub struct XmlText(::windows_core::IUnknown);
impl XmlText {
    pub fn Data(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetData)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Length(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Length)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SubstringData(&self, offset: u32, count: u32) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SubstringData)(::windows_core::Interface::as_raw(this), offset, count, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AppendData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, data: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AppendData)(::windows_core::Interface::as_raw(this), data.into_param().abi()).ok() }
    }
    pub fn InsertData<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, offset: u32, data: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).InsertData)(::windows_core::Interface::as_raw(this), offset, data.into_param().abi()).ok() }
    }
    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).DeleteData)(::windows_core::Interface::as_raw(this), offset, count).ok() }
    }
    pub fn ReplaceData<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, offset: u32, count: u32, data: Param2) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceData)(::windows_core::Interface::as_raw(this), offset, count, data.into_param().abi()).ok() }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn SetNodeValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeType(&self) -> ::windows_core::Result<NodeType> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<NodeType>::zeroed();
            (::windows_core::Interface::vtable(this).NodeType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NodeType>(result__)
        }
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ParentNode(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ParentNode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ChildNodes(&self) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ChildNodes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FirstChild)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LastChild)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn PreviousSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousSibling)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NextSibling(&self) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NextSibling)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows_core::Result<XmlNamedNodeMap> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Attributes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlNamedNodeMap>(result__)
        }
    }
    pub fn HasChildNodes(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasChildNodes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn OwnerDocument(&self) -> ::windows_core::Result<XmlDocument> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OwnerDocument)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XmlDocument>(result__)
        }
    }
    pub fn InsertBefore<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>, Param1: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InsertBefore)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), referencechild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn ReplaceChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>, Param1: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0, referencechild: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReplaceChild)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), referencechild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn RemoveChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>>(&self, childnode: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoveChild)(::windows_core::Interface::as_raw(this), childnode.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn AppendChild<'a, Param0: ::windows_core::IntoParam<'a, IXmlNode>>(&self, newchild: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppendChild)(::windows_core::Interface::as_raw(this), newchild.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn CloneNode(&self, deep: bool) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CloneNode)(::windows_core::Interface::as_raw(this), deep, result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn NamespaceUri(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).NamespaceUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn LocalName(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).LocalName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn Prefix(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).Prefix)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn Normalize(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Normalize)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPrefix)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectSingleNode<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, xpath: Param0) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNode)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodes<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, xpath: Param0) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodes)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn SelectSingleNodeNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows_core::Result<IXmlNode> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectSingleNodeNS)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), namespaces.into_param().abi(), result__.as_mut_ptr()).from_abi::<IXmlNode>(result__)
        }
    }
    pub fn SelectNodesNS<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, xpath: Param0, namespaces: Param1) -> ::windows_core::Result<XmlNodeList> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectNodesNS)(::windows_core::Interface::as_raw(this), xpath.into_param().abi(), namespaces.into_param().abi(), result__.as_mut_ptr()).from_abi::<XmlNodeList>(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetXml)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn InnerText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).InnerText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetInnerText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetInnerText)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SplitText(&self, offset: u32) -> ::windows_core::Result<IXmlText> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplitText)(::windows_core::Interface::as_raw(this), offset, result__.as_mut_ptr()).from_abi::<IXmlText>(result__)
        }
    }
}
impl ::core::clone::Clone for XmlText {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XmlText {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XmlText {}
impl ::core::fmt::Debug for XmlText {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlText").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XmlText {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Dom.XmlText;{f931a4cb-308d-4760-a1d5-43b67450ac7e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for XmlText {
    type Vtable = IXmlText_Vtbl;
    const IID: ::windows_core::GUID = <IXmlText as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for XmlText {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlText";
}
impl ::core::convert::From<XmlText> for ::windows_core::IUnknown {
    fn from(value: XmlText) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlText> for ::windows_core::IUnknown {
    fn from(value: &XmlText) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for XmlText {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a XmlText {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XmlText> for ::windows_core::IInspectable {
    fn from(value: XmlText) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XmlText> for ::windows_core::IInspectable {
    fn from(value: &XmlText) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for XmlText {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a XmlText {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<XmlText> for IXmlCharacterData {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlText) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlText> for IXmlCharacterData {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlText) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlCharacterData> for XmlText {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlCharacterData> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlCharacterData> for &XmlText {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlCharacterData> {
        ::core::convert::TryInto::<IXmlCharacterData>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlText> for IXmlNode {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlText) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlText> for IXmlNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlText) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNode> for XmlText {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNode> for &XmlText {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNode> {
        ::core::convert::TryInto::<IXmlNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlText> for IXmlNodeSelector {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlText) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlText> for IXmlNodeSelector {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlText) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSelector> for XmlText {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSelector> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSelector> for &XmlText {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSelector> {
        ::core::convert::TryInto::<IXmlNodeSelector>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlText> for IXmlNodeSerializer {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlText) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlText> for IXmlNodeSerializer {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlText) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSerializer> for XmlText {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSerializer> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlNodeSerializer> for &XmlText {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlNodeSerializer> {
        ::core::convert::TryInto::<IXmlNodeSerializer>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<XmlText> for IXmlText {
    type Error = ::windows_core::Error;
    fn try_from(value: XmlText) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XmlText> for IXmlText {
    type Error = ::windows_core::Error;
    fn try_from(value: &XmlText) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlText> for XmlText {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlText> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IXmlText> for &XmlText {
    fn into_param(self) -> ::windows_core::Param<'a, IXmlText> {
        ::core::convert::TryInto::<IXmlText>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for XmlText {}
unsafe impl ::core::marker::Sync for XmlText {}
