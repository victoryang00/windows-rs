#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn ClearPropVariantArray(rgpropvar: &mut [super::super::super::System::Com::StructuredStorage::PROPVARIANT]) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ClearPropVariantArray(rgpropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT, cvars: u32);
        }
        ClearPropVariantArray(::core::mem::transmute(::windows_core::as_mut_ptr_or_null(rgpropvar)), rgpropvar.len() as _)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn ClearVariantArray(pvars: &mut [super::super::super::System::Com::VARIANT]) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ClearVariantArray(pvars: *mut super::super::super::System::Com::VARIANT, cvars: u32);
        }
        ClearVariantArray(::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pvars)), pvars.len() as _)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DRAWPROGRESSFLAGS(pub u32);
pub const DPF_NONE: DRAWPROGRESSFLAGS = DRAWPROGRESSFLAGS(0u32);
pub const DPF_MARQUEE: DRAWPROGRESSFLAGS = DRAWPROGRESSFLAGS(1u32);
pub const DPF_MARQUEE_COMPLETE: DRAWPROGRESSFLAGS = DRAWPROGRESSFLAGS(2u32);
pub const DPF_ERROR: DRAWPROGRESSFLAGS = DRAWPROGRESSFLAGS(4u32);
pub const DPF_WARNING: DRAWPROGRESSFLAGS = DRAWPROGRESSFLAGS(8u32);
pub const DPF_STOPPED: DRAWPROGRESSFLAGS = DRAWPROGRESSFLAGS(16u32);
impl ::core::marker::Copy for DRAWPROGRESSFLAGS {}
impl ::core::clone::Clone for DRAWPROGRESSFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRAWPROGRESSFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DRAWPROGRESSFLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DRAWPROGRESSFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRAWPROGRESSFLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DRAWPROGRESSFLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DRAWPROGRESSFLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DRAWPROGRESSFLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DRAWPROGRESSFLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DRAWPROGRESSFLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GETPROPERTYSTOREFLAGS(pub u32);
pub const GPS_DEFAULT: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(0u32);
pub const GPS_HANDLERPROPERTIESONLY: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(1u32);
pub const GPS_READWRITE: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(2u32);
pub const GPS_TEMPORARY: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(4u32);
pub const GPS_FASTPROPERTIESONLY: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(8u32);
pub const GPS_OPENSLOWITEM: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(16u32);
pub const GPS_DELAYCREATION: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(32u32);
pub const GPS_BESTEFFORT: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(64u32);
pub const GPS_NO_OPLOCK: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(128u32);
pub const GPS_PREFERQUERYPROPERTIES: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(256u32);
pub const GPS_EXTRINSICPROPERTIES: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(512u32);
pub const GPS_EXTRINSICPROPERTIESONLY: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(1024u32);
pub const GPS_VOLATILEPROPERTIES: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(2048u32);
pub const GPS_VOLATILEPROPERTIESONLY: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(4096u32);
pub const GPS_MASK_VALID: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(8191u32);
impl ::core::marker::Copy for GETPROPERTYSTOREFLAGS {}
impl ::core::clone::Clone for GETPROPERTYSTOREFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GETPROPERTYSTOREFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GETPROPERTYSTOREFLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for GETPROPERTYSTOREFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GETPROPERTYSTOREFLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for GETPROPERTYSTOREFLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GETPROPERTYSTOREFLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GETPROPERTYSTOREFLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GETPROPERTYSTOREFLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GETPROPERTYSTOREFLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
pub struct ICreateObject(::windows_core::IUnknown);
impl ICreateObject {
    pub unsafe fn CreateObject<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, T: ::windows_core::Interface>(&self, clsid: *const ::windows_core::GUID, punkouter: Param1) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).CreateObject)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(clsid), punkouter.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<ICreateObject> for ::windows_core::IUnknown {
    fn from(value: ICreateObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICreateObject> for ::windows_core::IUnknown {
    fn from(value: &ICreateObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICreateObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICreateObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICreateObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICreateObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICreateObject {}
impl ::core::fmt::Debug for ICreateObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICreateObject").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICreateObject {
    type Vtable = ICreateObject_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x75121952_e0d0_43e5_9380_1d80483acf72);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateObject_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub CreateObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: *const ::windows_core::GUID, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDelayedPropertyStoreFactory(::windows_core::IUnknown);
impl IDelayedPropertyStoreFactory {
    pub unsafe fn GetPropertyStore<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, T: ::windows_core::Interface>(&self, flags: GETPROPERTYSTOREFLAGS, punkfactory: Param1) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).base__.GetPropertyStore)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(flags), punkfactory.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn GetPropertyStoreForKeys<T: ::windows_core::Interface>(&self, rgkeys: *const PROPERTYKEY, ckeys: u32, flags: GETPROPERTYSTOREFLAGS) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).base__.GetPropertyStoreForKeys)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(rgkeys), ::core::mem::transmute(ckeys), ::core::mem::transmute(flags), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn GetDelayedPropertyStore<T: ::windows_core::Interface>(&self, flags: GETPROPERTYSTOREFLAGS, dwstoreid: u32) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).GetDelayedPropertyStore)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(flags), ::core::mem::transmute(dwstoreid), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IDelayedPropertyStoreFactory> for ::windows_core::IUnknown {
    fn from(value: IDelayedPropertyStoreFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDelayedPropertyStoreFactory> for ::windows_core::IUnknown {
    fn from(value: &IDelayedPropertyStoreFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDelayedPropertyStoreFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDelayedPropertyStoreFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDelayedPropertyStoreFactory> for IPropertyStoreFactory {
    fn from(value: IDelayedPropertyStoreFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDelayedPropertyStoreFactory> for IPropertyStoreFactory {
    fn from(value: &IDelayedPropertyStoreFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPropertyStoreFactory> for IDelayedPropertyStoreFactory {
    fn into_param(self) -> ::windows_core::Param<'a, IPropertyStoreFactory> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPropertyStoreFactory> for &'a IDelayedPropertyStoreFactory {
    fn into_param(self) -> ::windows_core::Param<'a, IPropertyStoreFactory> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDelayedPropertyStoreFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDelayedPropertyStoreFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDelayedPropertyStoreFactory {}
impl ::core::fmt::Debug for IDelayedPropertyStoreFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDelayedPropertyStoreFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDelayedPropertyStoreFactory {
    type Vtable = IDelayedPropertyStoreFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x40d4577f_e237_4bdb_bd69_58f089431b6a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDelayedPropertyStoreFactory_Vtbl {
    pub base__: IPropertyStoreFactory_Vtbl,
    pub GetDelayedPropertyStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: GETPROPERTYSTOREFLAGS, dwstoreid: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IInitializeWithFile(::windows_core::IUnknown);
impl IInitializeWithFile {
    pub unsafe fn Initialize<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszfilepath: Param0, grfmode: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), pszfilepath.into_param().abi(), ::core::mem::transmute(grfmode)).ok()
    }
}
impl ::core::convert::From<IInitializeWithFile> for ::windows_core::IUnknown {
    fn from(value: IInitializeWithFile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInitializeWithFile> for ::windows_core::IUnknown {
    fn from(value: &IInitializeWithFile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInitializeWithFile {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInitializeWithFile {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IInitializeWithFile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInitializeWithFile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInitializeWithFile {}
impl ::core::fmt::Debug for IInitializeWithFile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInitializeWithFile").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IInitializeWithFile {
    type Vtable = IInitializeWithFile_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb7d14566_0509_4cce_a71f_0a554233bd9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInitializeWithFile_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfilepath: ::windows_core::PCWSTR, grfmode: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IInitializeWithStream(::windows_core::IUnknown);
impl IInitializeWithStream {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, pstream: Param0, grfmode: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), pstream.into_param().abi(), ::core::mem::transmute(grfmode)).ok()
    }
}
impl ::core::convert::From<IInitializeWithStream> for ::windows_core::IUnknown {
    fn from(value: IInitializeWithStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInitializeWithStream> for ::windows_core::IUnknown {
    fn from(value: &IInitializeWithStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInitializeWithStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInitializeWithStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IInitializeWithStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInitializeWithStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInitializeWithStream {}
impl ::core::fmt::Debug for IInitializeWithStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInitializeWithStream").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IInitializeWithStream {
    type Vtable = IInitializeWithStream_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb824b49d_22ac_4161_ac8a_9916e8fa3f7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInitializeWithStream_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: ::windows_core::RawPtr, grfmode: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
}
#[repr(transparent)]
pub struct INamedPropertyStore(::windows_core::IUnknown);
impl INamedPropertyStore {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetNamedValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszname: Param0) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetNamedValue)(::windows_core::Interface::as_raw(self), pszname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetNamedValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszname: Param0, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetNamedValue)(::windows_core::Interface::as_raw(self), pszname.into_param().abi(), ::core::mem::transmute(propvar)).ok()
    }
    pub unsafe fn GetNameCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetNameCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNameAt(&self, iprop: u32) -> ::windows_core::Result<super::super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetNameAt)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iprop), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<INamedPropertyStore> for ::windows_core::IUnknown {
    fn from(value: INamedPropertyStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INamedPropertyStore> for ::windows_core::IUnknown {
    fn from(value: &INamedPropertyStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INamedPropertyStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INamedPropertyStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INamedPropertyStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INamedPropertyStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INamedPropertyStore {}
impl ::core::fmt::Debug for INamedPropertyStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INamedPropertyStore").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for INamedPropertyStore {
    type Vtable = INamedPropertyStore_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x71604b0f_97b0_4764_8577_2f13e98a1422);
}
#[repr(C)]
#[doc(hidden)]
pub struct INamedPropertyStore_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetNamedValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetNamedValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub SetNamedValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    SetNamedValue: usize,
    pub GetNameCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetNameAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iprop: u32, pbstrname: *mut super::super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetNameAt: usize,
}
#[repr(transparent)]
pub struct IObjectWithPropertyKey(::windows_core::IUnknown);
impl IObjectWithPropertyKey {
    pub unsafe fn SetPropertyKey(&self, key: *const PROPERTYKEY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPropertyKey)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key)).ok()
    }
    pub unsafe fn GetPropertyKey(&self) -> ::windows_core::Result<PROPERTYKEY> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPERTYKEY>::zeroed();
        (::windows_core::Interface::vtable(self).GetPropertyKey)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPERTYKEY>(result__)
    }
}
impl ::core::convert::From<IObjectWithPropertyKey> for ::windows_core::IUnknown {
    fn from(value: IObjectWithPropertyKey) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IObjectWithPropertyKey> for ::windows_core::IUnknown {
    fn from(value: &IObjectWithPropertyKey) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IObjectWithPropertyKey {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IObjectWithPropertyKey {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IObjectWithPropertyKey {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IObjectWithPropertyKey {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectWithPropertyKey {}
impl ::core::fmt::Debug for IObjectWithPropertyKey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectWithPropertyKey").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IObjectWithPropertyKey {
    type Vtable = IObjectWithPropertyKey_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfc0ca0a7_c316_4fd2_9031_3e628e6d4f23);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectWithPropertyKey_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetPropertyKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY) -> ::windows_core::HRESULT,
    pub GetPropertyKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pkey: *mut PROPERTYKEY) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPersistSerializedPropStorage(::windows_core::IUnknown);
impl IPersistSerializedPropStorage {
    pub unsafe fn SetFlags(&self, flags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(flags)).ok()
    }
    pub unsafe fn SetPropertyStorage(&self, psps: *const SERIALIZEDPROPSTORAGE, cb: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPropertyStorage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(psps), ::core::mem::transmute(cb)).ok()
    }
    pub unsafe fn GetPropertyStorage(&self, ppsps: *mut *mut SERIALIZEDPROPSTORAGE, pcb: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPropertyStorage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppsps), ::core::mem::transmute(pcb)).ok()
    }
}
impl ::core::convert::From<IPersistSerializedPropStorage> for ::windows_core::IUnknown {
    fn from(value: IPersistSerializedPropStorage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPersistSerializedPropStorage> for ::windows_core::IUnknown {
    fn from(value: &IPersistSerializedPropStorage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPersistSerializedPropStorage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPersistSerializedPropStorage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPersistSerializedPropStorage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPersistSerializedPropStorage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPersistSerializedPropStorage {}
impl ::core::fmt::Debug for IPersistSerializedPropStorage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistSerializedPropStorage").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPersistSerializedPropStorage {
    type Vtable = IPersistSerializedPropStorage_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe318ad57_0aa0_450f_aca5_6fab7103d917);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistSerializedPropStorage_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows_core::HRESULT,
    pub SetPropertyStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psps: *const SERIALIZEDPROPSTORAGE, cb: u32) -> ::windows_core::HRESULT,
    pub GetPropertyStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsps: *mut *mut SERIALIZEDPROPSTORAGE, pcb: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPersistSerializedPropStorage2(::windows_core::IUnknown);
impl IPersistSerializedPropStorage2 {
    pub unsafe fn SetFlags(&self, flags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(flags)).ok()
    }
    pub unsafe fn SetPropertyStorage(&self, psps: *const SERIALIZEDPROPSTORAGE, cb: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetPropertyStorage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(psps), ::core::mem::transmute(cb)).ok()
    }
    pub unsafe fn GetPropertyStorage(&self, ppsps: *mut *mut SERIALIZEDPROPSTORAGE, pcb: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetPropertyStorage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppsps), ::core::mem::transmute(pcb)).ok()
    }
    pub unsafe fn GetPropertyStorageSize(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetPropertyStorageSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetPropertyStorageBuffer(&self, psps: *mut SERIALIZEDPROPSTORAGE, cb: u32, pcbwritten: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPropertyStorageBuffer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(psps), ::core::mem::transmute(cb), ::core::mem::transmute(pcbwritten)).ok()
    }
}
impl ::core::convert::From<IPersistSerializedPropStorage2> for ::windows_core::IUnknown {
    fn from(value: IPersistSerializedPropStorage2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPersistSerializedPropStorage2> for ::windows_core::IUnknown {
    fn from(value: &IPersistSerializedPropStorage2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPersistSerializedPropStorage2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPersistSerializedPropStorage2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPersistSerializedPropStorage2> for IPersistSerializedPropStorage {
    fn from(value: IPersistSerializedPropStorage2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPersistSerializedPropStorage2> for IPersistSerializedPropStorage {
    fn from(value: &IPersistSerializedPropStorage2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPersistSerializedPropStorage> for IPersistSerializedPropStorage2 {
    fn into_param(self) -> ::windows_core::Param<'a, IPersistSerializedPropStorage> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPersistSerializedPropStorage> for &'a IPersistSerializedPropStorage2 {
    fn into_param(self) -> ::windows_core::Param<'a, IPersistSerializedPropStorage> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPersistSerializedPropStorage2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPersistSerializedPropStorage2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPersistSerializedPropStorage2 {}
impl ::core::fmt::Debug for IPersistSerializedPropStorage2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistSerializedPropStorage2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPersistSerializedPropStorage2 {
    type Vtable = IPersistSerializedPropStorage2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x77effa68_4f98_4366_ba72_573b3d880571);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistSerializedPropStorage2_Vtbl {
    pub base__: IPersistSerializedPropStorage_Vtbl,
    pub GetPropertyStorageSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcb: *mut u32) -> ::windows_core::HRESULT,
    pub GetPropertyStorageBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psps: *mut SERIALIZEDPROPSTORAGE, cb: u32, pcbwritten: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPropertyChange(::windows_core::IUnknown);
impl IPropertyChange {
    pub unsafe fn SetPropertyKey(&self, key: *const PROPERTYKEY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetPropertyKey)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key)).ok()
    }
    pub unsafe fn GetPropertyKey(&self) -> ::windows_core::Result<PROPERTYKEY> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPERTYKEY>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPropertyKey)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPERTYKEY>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn ApplyToPropVariant(&self, propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).ApplyToPropVariant)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(propvarin), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
}
impl ::core::convert::From<IPropertyChange> for ::windows_core::IUnknown {
    fn from(value: IPropertyChange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyChange> for ::windows_core::IUnknown {
    fn from(value: &IPropertyChange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPropertyChange {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPropertyChange {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPropertyChange> for IObjectWithPropertyKey {
    fn from(value: IPropertyChange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyChange> for IObjectWithPropertyKey {
    fn from(value: &IPropertyChange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IObjectWithPropertyKey> for IPropertyChange {
    fn into_param(self) -> ::windows_core::Param<'a, IObjectWithPropertyKey> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IObjectWithPropertyKey> for &'a IPropertyChange {
    fn into_param(self) -> ::windows_core::Param<'a, IObjectWithPropertyKey> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPropertyChange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPropertyChange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyChange {}
impl ::core::fmt::Debug for IPropertyChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyChange").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPropertyChange {
    type Vtable = IPropertyChange_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf917bc8a_1bba_4478_a245_1bde03eb9431);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyChange_Vtbl {
    pub base__: IObjectWithPropertyKey_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub ApplyToPropVariant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvarout: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    ApplyToPropVariant: usize,
}
#[repr(transparent)]
pub struct IPropertyChangeArray(::windows_core::IUnknown);
impl IPropertyChangeArray {
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetAt<T: ::windows_core::Interface>(&self, iindex: u32) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).GetAt)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iindex), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn InsertAt<'a, Param1: ::windows_core::IntoParam<'a, IPropertyChange>>(&self, iindex: u32, ppropchange: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InsertAt)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iindex), ppropchange.into_param().abi()).ok()
    }
    pub unsafe fn Append<'a, Param0: ::windows_core::IntoParam<'a, IPropertyChange>>(&self, ppropchange: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Append)(::windows_core::Interface::as_raw(self), ppropchange.into_param().abi()).ok()
    }
    pub unsafe fn AppendOrReplace<'a, Param0: ::windows_core::IntoParam<'a, IPropertyChange>>(&self, ppropchange: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AppendOrReplace)(::windows_core::Interface::as_raw(self), ppropchange.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, iindex: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAt)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iindex)).ok()
    }
    pub unsafe fn IsKeyInArray(&self, key: *const PROPERTYKEY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsKeyInArray)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key)).ok()
    }
}
impl ::core::convert::From<IPropertyChangeArray> for ::windows_core::IUnknown {
    fn from(value: IPropertyChangeArray) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyChangeArray> for ::windows_core::IUnknown {
    fn from(value: &IPropertyChangeArray) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPropertyChangeArray {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPropertyChangeArray {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPropertyChangeArray {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPropertyChangeArray {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyChangeArray {}
impl ::core::fmt::Debug for IPropertyChangeArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyChangeArray").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPropertyChangeArray {
    type Vtable = IPropertyChangeArray_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x380f5cad_1b5e_42f2_805d_637fd392d31e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyChangeArray_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcoperations: *mut u32) -> ::windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iindex: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iindex: u32, ppropchange: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropchange: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AppendOrReplace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropchange: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iindex: u32) -> ::windows_core::HRESULT,
    pub IsKeyInArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPropertyDescription(::windows_core::IUnknown);
impl IPropertyDescription {
    pub unsafe fn GetPropertyKey(&self) -> ::windows_core::Result<PROPERTYKEY> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPERTYKEY>::zeroed();
        (::windows_core::Interface::vtable(self).GetPropertyKey)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPERTYKEY>(result__)
    }
    pub unsafe fn GetCanonicalName(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetCanonicalName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetPropertyType(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).GetPropertyType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn GetDisplayName(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetDisplayName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetEditInvitation(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetEditInvitation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetTypeFlags(&self, mask: PROPDESC_TYPE_FLAGS) -> ::windows_core::Result<PROPDESC_TYPE_FLAGS> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPDESC_TYPE_FLAGS>::zeroed();
        (::windows_core::Interface::vtable(self).GetTypeFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(mask), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPDESC_TYPE_FLAGS>(result__)
    }
    pub unsafe fn GetViewFlags(&self) -> ::windows_core::Result<PROPDESC_VIEW_FLAGS> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPDESC_VIEW_FLAGS>::zeroed();
        (::windows_core::Interface::vtable(self).GetViewFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPDESC_VIEW_FLAGS>(result__)
    }
    pub unsafe fn GetDefaultColumnWidth(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetDefaultColumnWidth)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetDisplayType(&self) -> ::windows_core::Result<PROPDESC_DISPLAYTYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPDESC_DISPLAYTYPE>::zeroed();
        (::windows_core::Interface::vtable(self).GetDisplayType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPDESC_DISPLAYTYPE>(result__)
    }
    pub unsafe fn GetColumnState(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetColumnState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetGroupingRange(&self) -> ::windows_core::Result<PROPDESC_GROUPING_RANGE> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPDESC_GROUPING_RANGE>::zeroed();
        (::windows_core::Interface::vtable(self).GetGroupingRange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPDESC_GROUPING_RANGE>(result__)
    }
    pub unsafe fn GetRelativeDescriptionType(&self) -> ::windows_core::Result<PROPDESC_RELATIVEDESCRIPTION_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPDESC_RELATIVEDESCRIPTION_TYPE>::zeroed();
        (::windows_core::Interface::vtable(self).GetRelativeDescriptionType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPDESC_RELATIVEDESCRIPTION_TYPE>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetRelativeDescription(&self, propvar1: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszdesc1: *mut ::windows_core::PWSTR, ppszdesc2: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRelativeDescription)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(propvar1), ::core::mem::transmute(propvar2), ::core::mem::transmute(ppszdesc1), ::core::mem::transmute(ppszdesc2)).ok()
    }
    pub unsafe fn GetSortDescription(&self) -> ::windows_core::Result<PROPDESC_SORTDESCRIPTION> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPDESC_SORTDESCRIPTION>::zeroed();
        (::windows_core::Interface::vtable(self).GetSortDescription)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPDESC_SORTDESCRIPTION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSortDescriptionLabel<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fdescending: Param0) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetSortDescriptionLabel)(::windows_core::Interface::as_raw(self), fdescending.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetAggregationType(&self) -> ::windows_core::Result<PROPDESC_AGGREGATION_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPDESC_AGGREGATION_TYPE>::zeroed();
        (::windows_core::Interface::vtable(self).GetAggregationType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPDESC_AGGREGATION_TYPE>(result__)
    }
    #[cfg(feature = "Win32_System_Search_Common")]
    pub unsafe fn GetConditionType(&self, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetConditionType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcontype), ::core::mem::transmute(popdefault)).ok()
    }
    pub unsafe fn GetEnumTypeList<T: ::windows_core::Interface>(&self) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).GetEnumTypeList)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn CoerceToCanonicalValue(&self, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CoerceToCanonicalValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppropvar)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn FormatForDisplay(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).FormatForDisplay)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(propvar), ::core::mem::transmute(pdfflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn IsValueCanonical(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsValueCanonical)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(propvar)).ok()
    }
}
impl ::core::convert::From<IPropertyDescription> for ::windows_core::IUnknown {
    fn from(value: IPropertyDescription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyDescription> for ::windows_core::IUnknown {
    fn from(value: &IPropertyDescription) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPropertyDescription {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPropertyDescription {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPropertyDescription {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPropertyDescription {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyDescription {}
impl ::core::fmt::Debug for IPropertyDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyDescription").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPropertyDescription {
    type Vtable = IPropertyDescription_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6f79d558_3e96_4549_a1d1_7d75d2288814);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyDescription_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetPropertyKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pkey: *mut PROPERTYKEY) -> ::windows_core::HRESULT,
    pub GetCanonicalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetPropertyType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvartype: *mut u16) -> ::windows_core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetEditInvitation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszinvite: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetTypeFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mask: PROPDESC_TYPE_FLAGS, ppdtflags: *mut PROPDESC_TYPE_FLAGS) -> ::windows_core::HRESULT,
    pub GetViewFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdvflags: *mut PROPDESC_VIEW_FLAGS) -> ::windows_core::HRESULT,
    pub GetDefaultColumnWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcxchars: *mut u32) -> ::windows_core::HRESULT,
    pub GetDisplayType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdisplaytype: *mut PROPDESC_DISPLAYTYPE) -> ::windows_core::HRESULT,
    pub GetColumnState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcsflags: *mut u32) -> ::windows_core::HRESULT,
    pub GetGroupingRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgr: *mut PROPDESC_GROUPING_RANGE) -> ::windows_core::HRESULT,
    pub GetRelativeDescriptionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prdt: *mut PROPDESC_RELATIVEDESCRIPTION_TYPE) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetRelativeDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propvar1: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszdesc1: *mut ::windows_core::PWSTR, ppszdesc2: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetRelativeDescription: usize,
    pub GetSortDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psd: *mut PROPDESC_SORTDESCRIPTION) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSortDescriptionLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fdescending: super::super::super::Foundation::BOOL, ppszdescription: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSortDescriptionLabel: usize,
    pub GetAggregationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paggtype: *mut PROPDESC_AGGREGATION_TYPE) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Search_Common")]
    pub GetConditionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Search_Common"))]
    GetConditionType: usize,
    pub GetEnumTypeList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub CoerceToCanonicalValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    CoerceToCanonicalValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub FormatForDisplay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    FormatForDisplay: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub IsValueCanonical: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    IsValueCanonical: usize,
}
#[repr(transparent)]
pub struct IPropertyDescription2(::windows_core::IUnknown);
impl IPropertyDescription2 {
    pub unsafe fn GetPropertyKey(&self) -> ::windows_core::Result<PROPERTYKEY> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPERTYKEY>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPropertyKey)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPERTYKEY>(result__)
    }
    pub unsafe fn GetCanonicalName(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetCanonicalName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetPropertyType(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPropertyType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn GetDisplayName(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDisplayName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetEditInvitation(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEditInvitation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetTypeFlags(&self, mask: PROPDESC_TYPE_FLAGS) -> ::windows_core::Result<PROPDESC_TYPE_FLAGS> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPDESC_TYPE_FLAGS>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTypeFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(mask), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPDESC_TYPE_FLAGS>(result__)
    }
    pub unsafe fn GetViewFlags(&self) -> ::windows_core::Result<PROPDESC_VIEW_FLAGS> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPDESC_VIEW_FLAGS>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetViewFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPDESC_VIEW_FLAGS>(result__)
    }
    pub unsafe fn GetDefaultColumnWidth(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDefaultColumnWidth)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetDisplayType(&self) -> ::windows_core::Result<PROPDESC_DISPLAYTYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPDESC_DISPLAYTYPE>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDisplayType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPDESC_DISPLAYTYPE>(result__)
    }
    pub unsafe fn GetColumnState(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetColumnState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetGroupingRange(&self) -> ::windows_core::Result<PROPDESC_GROUPING_RANGE> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPDESC_GROUPING_RANGE>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetGroupingRange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPDESC_GROUPING_RANGE>(result__)
    }
    pub unsafe fn GetRelativeDescriptionType(&self) -> ::windows_core::Result<PROPDESC_RELATIVEDESCRIPTION_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPDESC_RELATIVEDESCRIPTION_TYPE>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetRelativeDescriptionType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPDESC_RELATIVEDESCRIPTION_TYPE>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetRelativeDescription(&self, propvar1: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszdesc1: *mut ::windows_core::PWSTR, ppszdesc2: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetRelativeDescription)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(propvar1), ::core::mem::transmute(propvar2), ::core::mem::transmute(ppszdesc1), ::core::mem::transmute(ppszdesc2)).ok()
    }
    pub unsafe fn GetSortDescription(&self) -> ::windows_core::Result<PROPDESC_SORTDESCRIPTION> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPDESC_SORTDESCRIPTION>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSortDescription)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPDESC_SORTDESCRIPTION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSortDescriptionLabel<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fdescending: Param0) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSortDescriptionLabel)(::windows_core::Interface::as_raw(self), fdescending.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetAggregationType(&self) -> ::windows_core::Result<PROPDESC_AGGREGATION_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPDESC_AGGREGATION_TYPE>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetAggregationType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPDESC_AGGREGATION_TYPE>(result__)
    }
    #[cfg(feature = "Win32_System_Search_Common")]
    pub unsafe fn GetConditionType(&self, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetConditionType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcontype), ::core::mem::transmute(popdefault)).ok()
    }
    pub unsafe fn GetEnumTypeList<T: ::windows_core::Interface>(&self) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).base__.GetEnumTypeList)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn CoerceToCanonicalValue(&self, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CoerceToCanonicalValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppropvar)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn FormatForDisplay(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).base__.FormatForDisplay)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(propvar), ::core::mem::transmute(pdfflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn IsValueCanonical(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.IsValueCanonical)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(propvar)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetImageReferenceForValue(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetImageReferenceForValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(propvar), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
}
impl ::core::convert::From<IPropertyDescription2> for ::windows_core::IUnknown {
    fn from(value: IPropertyDescription2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyDescription2> for ::windows_core::IUnknown {
    fn from(value: &IPropertyDescription2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPropertyDescription2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPropertyDescription2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPropertyDescription2> for IPropertyDescription {
    fn from(value: IPropertyDescription2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyDescription2> for IPropertyDescription {
    fn from(value: &IPropertyDescription2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPropertyDescription> for IPropertyDescription2 {
    fn into_param(self) -> ::windows_core::Param<'a, IPropertyDescription> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPropertyDescription> for &'a IPropertyDescription2 {
    fn into_param(self) -> ::windows_core::Param<'a, IPropertyDescription> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPropertyDescription2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPropertyDescription2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyDescription2 {}
impl ::core::fmt::Debug for IPropertyDescription2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyDescription2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPropertyDescription2 {
    type Vtable = IPropertyDescription2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x57d2eded_5062_400e_b107_5dae79fe57a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyDescription2_Vtbl {
    pub base__: IPropertyDescription_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetImageReferenceForValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszimageres: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetImageReferenceForValue: usize,
}
#[repr(transparent)]
pub struct IPropertyDescriptionAliasInfo(::windows_core::IUnknown);
impl IPropertyDescriptionAliasInfo {
    pub unsafe fn GetPropertyKey(&self) -> ::windows_core::Result<PROPERTYKEY> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPERTYKEY>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPropertyKey)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPERTYKEY>(result__)
    }
    pub unsafe fn GetCanonicalName(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetCanonicalName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetPropertyType(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPropertyType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn GetDisplayName(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDisplayName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetEditInvitation(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEditInvitation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetTypeFlags(&self, mask: PROPDESC_TYPE_FLAGS) -> ::windows_core::Result<PROPDESC_TYPE_FLAGS> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPDESC_TYPE_FLAGS>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTypeFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(mask), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPDESC_TYPE_FLAGS>(result__)
    }
    pub unsafe fn GetViewFlags(&self) -> ::windows_core::Result<PROPDESC_VIEW_FLAGS> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPDESC_VIEW_FLAGS>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetViewFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPDESC_VIEW_FLAGS>(result__)
    }
    pub unsafe fn GetDefaultColumnWidth(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDefaultColumnWidth)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetDisplayType(&self) -> ::windows_core::Result<PROPDESC_DISPLAYTYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPDESC_DISPLAYTYPE>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDisplayType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPDESC_DISPLAYTYPE>(result__)
    }
    pub unsafe fn GetColumnState(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetColumnState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetGroupingRange(&self) -> ::windows_core::Result<PROPDESC_GROUPING_RANGE> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPDESC_GROUPING_RANGE>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetGroupingRange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPDESC_GROUPING_RANGE>(result__)
    }
    pub unsafe fn GetRelativeDescriptionType(&self) -> ::windows_core::Result<PROPDESC_RELATIVEDESCRIPTION_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPDESC_RELATIVEDESCRIPTION_TYPE>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetRelativeDescriptionType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPDESC_RELATIVEDESCRIPTION_TYPE>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetRelativeDescription(&self, propvar1: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszdesc1: *mut ::windows_core::PWSTR, ppszdesc2: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetRelativeDescription)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(propvar1), ::core::mem::transmute(propvar2), ::core::mem::transmute(ppszdesc1), ::core::mem::transmute(ppszdesc2)).ok()
    }
    pub unsafe fn GetSortDescription(&self) -> ::windows_core::Result<PROPDESC_SORTDESCRIPTION> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPDESC_SORTDESCRIPTION>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSortDescription)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPDESC_SORTDESCRIPTION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSortDescriptionLabel<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fdescending: Param0) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSortDescriptionLabel)(::windows_core::Interface::as_raw(self), fdescending.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetAggregationType(&self) -> ::windows_core::Result<PROPDESC_AGGREGATION_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPDESC_AGGREGATION_TYPE>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetAggregationType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPDESC_AGGREGATION_TYPE>(result__)
    }
    #[cfg(feature = "Win32_System_Search_Common")]
    pub unsafe fn GetConditionType(&self, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetConditionType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcontype), ::core::mem::transmute(popdefault)).ok()
    }
    pub unsafe fn GetEnumTypeList<T: ::windows_core::Interface>(&self) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).base__.GetEnumTypeList)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn CoerceToCanonicalValue(&self, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CoerceToCanonicalValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppropvar)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn FormatForDisplay(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).base__.FormatForDisplay)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(propvar), ::core::mem::transmute(pdfflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn IsValueCanonical(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.IsValueCanonical)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(propvar)).ok()
    }
    pub unsafe fn GetSortByAlias<T: ::windows_core::Interface>(&self) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).GetSortByAlias)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn GetAdditionalSortByAliases<T: ::windows_core::Interface>(&self) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).GetAdditionalSortByAliases)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IPropertyDescriptionAliasInfo> for ::windows_core::IUnknown {
    fn from(value: IPropertyDescriptionAliasInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyDescriptionAliasInfo> for ::windows_core::IUnknown {
    fn from(value: &IPropertyDescriptionAliasInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPropertyDescriptionAliasInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPropertyDescriptionAliasInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPropertyDescriptionAliasInfo> for IPropertyDescription {
    fn from(value: IPropertyDescriptionAliasInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyDescriptionAliasInfo> for IPropertyDescription {
    fn from(value: &IPropertyDescriptionAliasInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPropertyDescription> for IPropertyDescriptionAliasInfo {
    fn into_param(self) -> ::windows_core::Param<'a, IPropertyDescription> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPropertyDescription> for &'a IPropertyDescriptionAliasInfo {
    fn into_param(self) -> ::windows_core::Param<'a, IPropertyDescription> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPropertyDescriptionAliasInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPropertyDescriptionAliasInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyDescriptionAliasInfo {}
impl ::core::fmt::Debug for IPropertyDescriptionAliasInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyDescriptionAliasInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPropertyDescriptionAliasInfo {
    type Vtable = IPropertyDescriptionAliasInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf67104fc_2af9_46fd_b32d_243c1404f3d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyDescriptionAliasInfo_Vtbl {
    pub base__: IPropertyDescription_Vtbl,
    pub GetSortByAlias: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetAdditionalSortByAliases: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPropertyDescriptionList(::windows_core::IUnknown);
impl IPropertyDescriptionList {
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetAt<T: ::windows_core::Interface>(&self, ielem: u32) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).GetAt)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ielem), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IPropertyDescriptionList> for ::windows_core::IUnknown {
    fn from(value: IPropertyDescriptionList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyDescriptionList> for ::windows_core::IUnknown {
    fn from(value: &IPropertyDescriptionList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPropertyDescriptionList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPropertyDescriptionList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPropertyDescriptionList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPropertyDescriptionList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyDescriptionList {}
impl ::core::fmt::Debug for IPropertyDescriptionList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyDescriptionList").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPropertyDescriptionList {
    type Vtable = IPropertyDescriptionList_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1f9fc1d0_c39b_4b26_817f_011967d3440e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyDescriptionList_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelem: *mut u32) -> ::windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ielem: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPropertyDescriptionRelatedPropertyInfo(::windows_core::IUnknown);
impl IPropertyDescriptionRelatedPropertyInfo {
    pub unsafe fn GetPropertyKey(&self) -> ::windows_core::Result<PROPERTYKEY> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPERTYKEY>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPropertyKey)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPERTYKEY>(result__)
    }
    pub unsafe fn GetCanonicalName(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetCanonicalName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetPropertyType(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPropertyType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn GetDisplayName(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDisplayName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetEditInvitation(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEditInvitation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetTypeFlags(&self, mask: PROPDESC_TYPE_FLAGS) -> ::windows_core::Result<PROPDESC_TYPE_FLAGS> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPDESC_TYPE_FLAGS>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTypeFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(mask), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPDESC_TYPE_FLAGS>(result__)
    }
    pub unsafe fn GetViewFlags(&self) -> ::windows_core::Result<PROPDESC_VIEW_FLAGS> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPDESC_VIEW_FLAGS>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetViewFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPDESC_VIEW_FLAGS>(result__)
    }
    pub unsafe fn GetDefaultColumnWidth(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDefaultColumnWidth)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetDisplayType(&self) -> ::windows_core::Result<PROPDESC_DISPLAYTYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPDESC_DISPLAYTYPE>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDisplayType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPDESC_DISPLAYTYPE>(result__)
    }
    pub unsafe fn GetColumnState(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetColumnState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetGroupingRange(&self) -> ::windows_core::Result<PROPDESC_GROUPING_RANGE> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPDESC_GROUPING_RANGE>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetGroupingRange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPDESC_GROUPING_RANGE>(result__)
    }
    pub unsafe fn GetRelativeDescriptionType(&self) -> ::windows_core::Result<PROPDESC_RELATIVEDESCRIPTION_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPDESC_RELATIVEDESCRIPTION_TYPE>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetRelativeDescriptionType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPDESC_RELATIVEDESCRIPTION_TYPE>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetRelativeDescription(&self, propvar1: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszdesc1: *mut ::windows_core::PWSTR, ppszdesc2: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetRelativeDescription)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(propvar1), ::core::mem::transmute(propvar2), ::core::mem::transmute(ppszdesc1), ::core::mem::transmute(ppszdesc2)).ok()
    }
    pub unsafe fn GetSortDescription(&self) -> ::windows_core::Result<PROPDESC_SORTDESCRIPTION> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPDESC_SORTDESCRIPTION>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSortDescription)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPDESC_SORTDESCRIPTION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSortDescriptionLabel<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fdescending: Param0) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSortDescriptionLabel)(::windows_core::Interface::as_raw(self), fdescending.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetAggregationType(&self) -> ::windows_core::Result<PROPDESC_AGGREGATION_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPDESC_AGGREGATION_TYPE>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetAggregationType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPDESC_AGGREGATION_TYPE>(result__)
    }
    #[cfg(feature = "Win32_System_Search_Common")]
    pub unsafe fn GetConditionType(&self, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetConditionType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcontype), ::core::mem::transmute(popdefault)).ok()
    }
    pub unsafe fn GetEnumTypeList<T: ::windows_core::Interface>(&self) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).base__.GetEnumTypeList)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn CoerceToCanonicalValue(&self, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CoerceToCanonicalValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppropvar)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn FormatForDisplay(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).base__.FormatForDisplay)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(propvar), ::core::mem::transmute(pdfflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn IsValueCanonical(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.IsValueCanonical)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(propvar)).ok()
    }
    pub unsafe fn GetRelatedProperty<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, T: ::windows_core::Interface>(&self, pszrelationshipname: Param0) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).GetRelatedProperty)(::windows_core::Interface::as_raw(self), pszrelationshipname.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IPropertyDescriptionRelatedPropertyInfo> for ::windows_core::IUnknown {
    fn from(value: IPropertyDescriptionRelatedPropertyInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyDescriptionRelatedPropertyInfo> for ::windows_core::IUnknown {
    fn from(value: &IPropertyDescriptionRelatedPropertyInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPropertyDescriptionRelatedPropertyInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPropertyDescriptionRelatedPropertyInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPropertyDescriptionRelatedPropertyInfo> for IPropertyDescription {
    fn from(value: IPropertyDescriptionRelatedPropertyInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyDescriptionRelatedPropertyInfo> for IPropertyDescription {
    fn from(value: &IPropertyDescriptionRelatedPropertyInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPropertyDescription> for IPropertyDescriptionRelatedPropertyInfo {
    fn into_param(self) -> ::windows_core::Param<'a, IPropertyDescription> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPropertyDescription> for &'a IPropertyDescriptionRelatedPropertyInfo {
    fn into_param(self) -> ::windows_core::Param<'a, IPropertyDescription> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPropertyDescriptionRelatedPropertyInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPropertyDescriptionRelatedPropertyInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyDescriptionRelatedPropertyInfo {}
impl ::core::fmt::Debug for IPropertyDescriptionRelatedPropertyInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyDescriptionRelatedPropertyInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPropertyDescriptionRelatedPropertyInfo {
    type Vtable = IPropertyDescriptionRelatedPropertyInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x507393f4_2a3d_4a60_b59e_d9c75716c2dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyDescriptionRelatedPropertyInfo_Vtbl {
    pub base__: IPropertyDescription_Vtbl,
    pub GetRelatedProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszrelationshipname: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPropertyDescriptionSearchInfo(::windows_core::IUnknown);
impl IPropertyDescriptionSearchInfo {
    pub unsafe fn GetPropertyKey(&self) -> ::windows_core::Result<PROPERTYKEY> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPERTYKEY>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPropertyKey)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPERTYKEY>(result__)
    }
    pub unsafe fn GetCanonicalName(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetCanonicalName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetPropertyType(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPropertyType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn GetDisplayName(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDisplayName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetEditInvitation(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEditInvitation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetTypeFlags(&self, mask: PROPDESC_TYPE_FLAGS) -> ::windows_core::Result<PROPDESC_TYPE_FLAGS> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPDESC_TYPE_FLAGS>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTypeFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(mask), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPDESC_TYPE_FLAGS>(result__)
    }
    pub unsafe fn GetViewFlags(&self) -> ::windows_core::Result<PROPDESC_VIEW_FLAGS> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPDESC_VIEW_FLAGS>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetViewFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPDESC_VIEW_FLAGS>(result__)
    }
    pub unsafe fn GetDefaultColumnWidth(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDefaultColumnWidth)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetDisplayType(&self) -> ::windows_core::Result<PROPDESC_DISPLAYTYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPDESC_DISPLAYTYPE>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDisplayType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPDESC_DISPLAYTYPE>(result__)
    }
    pub unsafe fn GetColumnState(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetColumnState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetGroupingRange(&self) -> ::windows_core::Result<PROPDESC_GROUPING_RANGE> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPDESC_GROUPING_RANGE>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetGroupingRange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPDESC_GROUPING_RANGE>(result__)
    }
    pub unsafe fn GetRelativeDescriptionType(&self) -> ::windows_core::Result<PROPDESC_RELATIVEDESCRIPTION_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPDESC_RELATIVEDESCRIPTION_TYPE>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetRelativeDescriptionType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPDESC_RELATIVEDESCRIPTION_TYPE>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetRelativeDescription(&self, propvar1: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszdesc1: *mut ::windows_core::PWSTR, ppszdesc2: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetRelativeDescription)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(propvar1), ::core::mem::transmute(propvar2), ::core::mem::transmute(ppszdesc1), ::core::mem::transmute(ppszdesc2)).ok()
    }
    pub unsafe fn GetSortDescription(&self) -> ::windows_core::Result<PROPDESC_SORTDESCRIPTION> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPDESC_SORTDESCRIPTION>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSortDescription)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPDESC_SORTDESCRIPTION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSortDescriptionLabel<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fdescending: Param0) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSortDescriptionLabel)(::windows_core::Interface::as_raw(self), fdescending.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetAggregationType(&self) -> ::windows_core::Result<PROPDESC_AGGREGATION_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPDESC_AGGREGATION_TYPE>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetAggregationType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPDESC_AGGREGATION_TYPE>(result__)
    }
    #[cfg(feature = "Win32_System_Search_Common")]
    pub unsafe fn GetConditionType(&self, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetConditionType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcontype), ::core::mem::transmute(popdefault)).ok()
    }
    pub unsafe fn GetEnumTypeList<T: ::windows_core::Interface>(&self) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).base__.GetEnumTypeList)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn CoerceToCanonicalValue(&self, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CoerceToCanonicalValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppropvar)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn FormatForDisplay(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).base__.FormatForDisplay)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(propvar), ::core::mem::transmute(pdfflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn IsValueCanonical(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.IsValueCanonical)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(propvar)).ok()
    }
    pub unsafe fn GetSearchInfoFlags(&self) -> ::windows_core::Result<PROPDESC_SEARCHINFO_FLAGS> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPDESC_SEARCHINFO_FLAGS>::zeroed();
        (::windows_core::Interface::vtable(self).GetSearchInfoFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPDESC_SEARCHINFO_FLAGS>(result__)
    }
    pub unsafe fn GetColumnIndexType(&self) -> ::windows_core::Result<PROPDESC_COLUMNINDEX_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPDESC_COLUMNINDEX_TYPE>::zeroed();
        (::windows_core::Interface::vtable(self).GetColumnIndexType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPDESC_COLUMNINDEX_TYPE>(result__)
    }
    pub unsafe fn GetProjectionString(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetProjectionString)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetMaxSize(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetMaxSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IPropertyDescriptionSearchInfo> for ::windows_core::IUnknown {
    fn from(value: IPropertyDescriptionSearchInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyDescriptionSearchInfo> for ::windows_core::IUnknown {
    fn from(value: &IPropertyDescriptionSearchInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPropertyDescriptionSearchInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPropertyDescriptionSearchInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPropertyDescriptionSearchInfo> for IPropertyDescription {
    fn from(value: IPropertyDescriptionSearchInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyDescriptionSearchInfo> for IPropertyDescription {
    fn from(value: &IPropertyDescriptionSearchInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPropertyDescription> for IPropertyDescriptionSearchInfo {
    fn into_param(self) -> ::windows_core::Param<'a, IPropertyDescription> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPropertyDescription> for &'a IPropertyDescriptionSearchInfo {
    fn into_param(self) -> ::windows_core::Param<'a, IPropertyDescription> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPropertyDescriptionSearchInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPropertyDescriptionSearchInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyDescriptionSearchInfo {}
impl ::core::fmt::Debug for IPropertyDescriptionSearchInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyDescriptionSearchInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPropertyDescriptionSearchInfo {
    type Vtable = IPropertyDescriptionSearchInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x078f91bd_29a2_440f_924e_46a291524520);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyDescriptionSearchInfo_Vtbl {
    pub base__: IPropertyDescription_Vtbl,
    pub GetSearchInfoFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdsiflags: *mut PROPDESC_SEARCHINFO_FLAGS) -> ::windows_core::HRESULT,
    pub GetColumnIndexType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdcitype: *mut PROPDESC_COLUMNINDEX_TYPE) -> ::windows_core::HRESULT,
    pub GetProjectionString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszprojection: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetMaxSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbmaxsize: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPropertyEnumType(::windows_core::IUnknown);
impl IPropertyEnumType {
    pub unsafe fn GetEnumType(&self) -> ::windows_core::Result<PROPENUMTYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPENUMTYPE>::zeroed();
        (::windows_core::Interface::vtable(self).GetEnumType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPENUMTYPE>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetValue(&self) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetRangeMinValue(&self) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetRangeMinValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetRangeSetValue(&self) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetRangeSetValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    pub unsafe fn GetDisplayText(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetDisplayText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
}
impl ::core::convert::From<IPropertyEnumType> for ::windows_core::IUnknown {
    fn from(value: IPropertyEnumType) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyEnumType> for ::windows_core::IUnknown {
    fn from(value: &IPropertyEnumType) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPropertyEnumType {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPropertyEnumType {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPropertyEnumType {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPropertyEnumType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyEnumType {}
impl ::core::fmt::Debug for IPropertyEnumType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyEnumType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPropertyEnumType {
    type Vtable = IPropertyEnumType_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x11e1fbf9_2d56_4a6b_8db3_7cd193a471f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyEnumType_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetEnumType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penumtype: *mut PROPENUMTYPE) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetRangeMinValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropvarmin: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetRangeMinValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetRangeSetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropvarset: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetRangeSetValue: usize,
    pub GetDisplayText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszdisplay: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPropertyEnumType2(::windows_core::IUnknown);
impl IPropertyEnumType2 {
    pub unsafe fn GetEnumType(&self) -> ::windows_core::Result<PROPENUMTYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPENUMTYPE>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEnumType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPENUMTYPE>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetValue(&self) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetRangeMinValue(&self) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetRangeMinValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetRangeSetValue(&self) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetRangeSetValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    pub unsafe fn GetDisplayText(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDisplayText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetImageReference(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetImageReference)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
}
impl ::core::convert::From<IPropertyEnumType2> for ::windows_core::IUnknown {
    fn from(value: IPropertyEnumType2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyEnumType2> for ::windows_core::IUnknown {
    fn from(value: &IPropertyEnumType2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPropertyEnumType2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPropertyEnumType2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPropertyEnumType2> for IPropertyEnumType {
    fn from(value: IPropertyEnumType2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyEnumType2> for IPropertyEnumType {
    fn from(value: &IPropertyEnumType2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPropertyEnumType> for IPropertyEnumType2 {
    fn into_param(self) -> ::windows_core::Param<'a, IPropertyEnumType> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPropertyEnumType> for &'a IPropertyEnumType2 {
    fn into_param(self) -> ::windows_core::Param<'a, IPropertyEnumType> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPropertyEnumType2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPropertyEnumType2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyEnumType2 {}
impl ::core::fmt::Debug for IPropertyEnumType2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyEnumType2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPropertyEnumType2 {
    type Vtable = IPropertyEnumType2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9b6e051c_5ddd_4321_9070_fe2acb55e794);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyEnumType2_Vtbl {
    pub base__: IPropertyEnumType_Vtbl,
    pub GetImageReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszimageres: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPropertyEnumTypeList(::windows_core::IUnknown);
impl IPropertyEnumTypeList {
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetAt<T: ::windows_core::Interface>(&self, itype: u32) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).GetAt)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(itype), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn GetConditionAt<T: ::windows_core::Interface>(&self, nindex: u32) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).GetConditionAt)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(nindex), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn FindMatchingIndex(&self, propvarcmp: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).FindMatchingIndex)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(propvarcmp), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IPropertyEnumTypeList> for ::windows_core::IUnknown {
    fn from(value: IPropertyEnumTypeList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyEnumTypeList> for ::windows_core::IUnknown {
    fn from(value: &IPropertyEnumTypeList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPropertyEnumTypeList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPropertyEnumTypeList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPropertyEnumTypeList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPropertyEnumTypeList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyEnumTypeList {}
impl ::core::fmt::Debug for IPropertyEnumTypeList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyEnumTypeList").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPropertyEnumTypeList {
    type Vtable = IPropertyEnumTypeList_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa99400f4_3d84_4557_94ba_1242fb2cc9a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyEnumTypeList_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctypes: *mut u32) -> ::windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itype: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetConditionAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub FindMatchingIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propvarcmp: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pnindex: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    FindMatchingIndex: usize,
}
#[repr(transparent)]
pub struct IPropertyStore(::windows_core::IUnknown);
impl IPropertyStore {
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetAt(&self, iprop: u32) -> ::windows_core::Result<PROPERTYKEY> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPERTYKEY>::zeroed();
        (::windows_core::Interface::vtable(self).GetAt)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iprop), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPERTYKEY>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetValue(&self, key: *const PROPERTYKEY) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetValue(&self, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(propvar)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IPropertyStore> for ::windows_core::IUnknown {
    fn from(value: IPropertyStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyStore> for ::windows_core::IUnknown {
    fn from(value: &IPropertyStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPropertyStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPropertyStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPropertyStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPropertyStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyStore {}
impl ::core::fmt::Debug for IPropertyStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyStore").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPropertyStore {
    type Vtable = IPropertyStore_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x886d8eeb_8cf2_4446_8d02_cdba1dbdcf99);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyStore_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cprops: *mut u32) -> ::windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iprop: u32, pkey: *mut PROPERTYKEY) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, pv: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    SetValue: usize,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPropertyStoreCache(::windows_core::IUnknown);
impl IPropertyStoreCache {
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetAt(&self, iprop: u32) -> ::windows_core::Result<PROPERTYKEY> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPERTYKEY>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetAt)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iprop), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPERTYKEY>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetValue(&self, key: *const PROPERTYKEY) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetValue(&self, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(propvar)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetState(&self, key: *const PROPERTYKEY) -> ::windows_core::Result<PSC_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::<PSC_STATE>::zeroed();
        (::windows_core::Interface::vtable(self).GetState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PSC_STATE>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetValueAndState(&self, key: *const PROPERTYKEY, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT, pstate: *mut PSC_STATE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetValueAndState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(ppropvar), ::core::mem::transmute(pstate)).ok()
    }
    pub unsafe fn SetState(&self, key: *const PROPERTYKEY, state: PSC_STATE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(state)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetValueAndState(&self, key: *const PROPERTYKEY, ppropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, state: PSC_STATE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetValueAndState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(ppropvar), ::core::mem::transmute(state)).ok()
    }
}
impl ::core::convert::From<IPropertyStoreCache> for ::windows_core::IUnknown {
    fn from(value: IPropertyStoreCache) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyStoreCache> for ::windows_core::IUnknown {
    fn from(value: &IPropertyStoreCache) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPropertyStoreCache {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPropertyStoreCache {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPropertyStoreCache> for IPropertyStore {
    fn from(value: IPropertyStoreCache) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyStoreCache> for IPropertyStore {
    fn from(value: &IPropertyStoreCache) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPropertyStore> for IPropertyStoreCache {
    fn into_param(self) -> ::windows_core::Param<'a, IPropertyStore> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPropertyStore> for &'a IPropertyStoreCache {
    fn into_param(self) -> ::windows_core::Param<'a, IPropertyStore> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPropertyStoreCache {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPropertyStoreCache {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyStoreCache {}
impl ::core::fmt::Debug for IPropertyStoreCache {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyStoreCache").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPropertyStoreCache {
    type Vtable = IPropertyStoreCache_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3017056d_9a91_4e90_937d_746c72abbf4f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyStoreCache_Vtbl {
    pub base__: IPropertyStore_Vtbl,
    pub GetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, pstate: *mut PSC_STATE) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetValueAndState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT, pstate: *mut PSC_STATE) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetValueAndState: usize,
    pub SetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, state: PSC_STATE) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub SetValueAndState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, ppropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, state: PSC_STATE) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    SetValueAndState: usize,
}
#[repr(transparent)]
pub struct IPropertyStoreCapabilities(::windows_core::IUnknown);
impl IPropertyStoreCapabilities {
    pub unsafe fn IsPropertyWritable(&self, key: *const PROPERTYKEY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsPropertyWritable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key)).ok()
    }
}
impl ::core::convert::From<IPropertyStoreCapabilities> for ::windows_core::IUnknown {
    fn from(value: IPropertyStoreCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyStoreCapabilities> for ::windows_core::IUnknown {
    fn from(value: &IPropertyStoreCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPropertyStoreCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPropertyStoreCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPropertyStoreCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPropertyStoreCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyStoreCapabilities {}
impl ::core::fmt::Debug for IPropertyStoreCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyStoreCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPropertyStoreCapabilities {
    type Vtable = IPropertyStoreCapabilities_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc8e2d566_186e_4d49_bf41_6909ead56acc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyStoreCapabilities_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub IsPropertyWritable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPropertyStoreFactory(::windows_core::IUnknown);
impl IPropertyStoreFactory {
    pub unsafe fn GetPropertyStore<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, T: ::windows_core::Interface>(&self, flags: GETPROPERTYSTOREFLAGS, punkfactory: Param1) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).GetPropertyStore)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(flags), punkfactory.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn GetPropertyStoreForKeys<T: ::windows_core::Interface>(&self, rgkeys: *const PROPERTYKEY, ckeys: u32, flags: GETPROPERTYSTOREFLAGS) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).GetPropertyStoreForKeys)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(rgkeys), ::core::mem::transmute(ckeys), ::core::mem::transmute(flags), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IPropertyStoreFactory> for ::windows_core::IUnknown {
    fn from(value: IPropertyStoreFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyStoreFactory> for ::windows_core::IUnknown {
    fn from(value: &IPropertyStoreFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPropertyStoreFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPropertyStoreFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPropertyStoreFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPropertyStoreFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyStoreFactory {}
impl ::core::fmt::Debug for IPropertyStoreFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyStoreFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPropertyStoreFactory {
    type Vtable = IPropertyStoreFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbc110b6d_57e8_4148_a9c6_91015ab2f3a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyStoreFactory_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetPropertyStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: GETPROPERTYSTOREFLAGS, punkfactory: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetPropertyStoreForKeys: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rgkeys: *const PROPERTYKEY, ckeys: u32, flags: GETPROPERTYSTOREFLAGS, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPropertySystem(::windows_core::IUnknown);
impl IPropertySystem {
    pub unsafe fn GetPropertyDescription<T: ::windows_core::Interface>(&self, propkey: *const PROPERTYKEY) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).GetPropertyDescription)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(propkey), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn GetPropertyDescriptionByName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, T: ::windows_core::Interface>(&self, pszcanonicalname: Param0) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).GetPropertyDescriptionByName)(::windows_core::Interface::as_raw(self), pszcanonicalname.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn GetPropertyDescriptionListFromString<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, T: ::windows_core::Interface>(&self, pszproplist: Param0) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).GetPropertyDescriptionListFromString)(::windows_core::Interface::as_raw(self), pszproplist.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn EnumeratePropertyDescriptions<T: ::windows_core::Interface>(&self, filteron: PROPDESC_ENUMFILTER) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).EnumeratePropertyDescriptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(filteron), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn FormatForDisplay(&self, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS, psztext: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FormatForDisplay)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(propvar), ::core::mem::transmute(pdff), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(psztext)), psztext.len() as _).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn FormatForDisplayAlloc(&self, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).FormatForDisplayAlloc)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(propvar), ::core::mem::transmute(pdff), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn RegisterPropertySchema<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszpath: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterPropertySchema)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi()).ok()
    }
    pub unsafe fn UnregisterPropertySchema<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszpath: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnregisterPropertySchema)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi()).ok()
    }
    pub unsafe fn RefreshPropertySchema(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RefreshPropertySchema)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IPropertySystem> for ::windows_core::IUnknown {
    fn from(value: IPropertySystem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertySystem> for ::windows_core::IUnknown {
    fn from(value: &IPropertySystem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPropertySystem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPropertySystem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPropertySystem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPropertySystem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertySystem {}
impl ::core::fmt::Debug for IPropertySystem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertySystem").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPropertySystem {
    type Vtable = IPropertySystem_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xca724e8a_c3e6_442b_88a4_6fb0db8035a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertySystem_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetPropertyDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propkey: *const PROPERTYKEY, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetPropertyDescriptionByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszcanonicalname: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetPropertyDescriptionListFromString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszproplist: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EnumeratePropertyDescriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filteron: PROPDESC_ENUMFILTER, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub FormatForDisplay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS, psztext: ::windows_core::PWSTR, cchtext: u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    FormatForDisplay: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub FormatForDisplayAlloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    FormatForDisplayAlloc: usize,
    pub RegisterPropertySchema: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub UnregisterPropertySchema: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub RefreshPropertySchema: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPropertySystemChangeNotify(::windows_core::IUnknown);
impl IPropertySystemChangeNotify {
    pub unsafe fn SchemaRefreshed(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SchemaRefreshed)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IPropertySystemChangeNotify> for ::windows_core::IUnknown {
    fn from(value: IPropertySystemChangeNotify) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertySystemChangeNotify> for ::windows_core::IUnknown {
    fn from(value: &IPropertySystemChangeNotify) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPropertySystemChangeNotify {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPropertySystemChangeNotify {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPropertySystemChangeNotify {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPropertySystemChangeNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertySystemChangeNotify {}
impl ::core::fmt::Debug for IPropertySystemChangeNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertySystemChangeNotify").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPropertySystemChangeNotify {
    type Vtable = IPropertySystemChangeNotify_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfa955fd9_38be_4879_a6ce_824cf52d609f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertySystemChangeNotify_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SchemaRefreshed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPropertyUI(::windows_core::IUnknown);
impl IPropertyUI {
    pub unsafe fn ParsePropertyName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszname: Param0, pfmtid: *mut ::windows_core::GUID, ppid: *mut u32, pcheaten: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ParsePropertyName)(::windows_core::Interface::as_raw(self), pszname.into_param().abi(), ::core::mem::transmute(pfmtid), ::core::mem::transmute(ppid), ::core::mem::transmute(pcheaten)).ok()
    }
    pub unsafe fn GetCannonicalName(&self, fmtid: *const ::windows_core::GUID, pid: u32, pwsztext: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCannonicalName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(fmtid), ::core::mem::transmute(pid), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pwsztext)), pwsztext.len() as _).ok()
    }
    pub unsafe fn GetDisplayName(&self, fmtid: *const ::windows_core::GUID, pid: u32, flags: PROPERTYUI_NAME_FLAGS, pwsztext: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDisplayName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(fmtid), ::core::mem::transmute(pid), ::core::mem::transmute(flags), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pwsztext)), pwsztext.len() as _).ok()
    }
    pub unsafe fn GetPropertyDescription(&self, fmtid: *const ::windows_core::GUID, pid: u32, pwsztext: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPropertyDescription)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(fmtid), ::core::mem::transmute(pid), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pwsztext)), pwsztext.len() as _).ok()
    }
    pub unsafe fn GetDefaultWidth(&self, fmtid: *const ::windows_core::GUID, pid: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetDefaultWidth)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(fmtid), ::core::mem::transmute(pid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetFlags(&self, fmtid: *const ::windows_core::GUID, pid: u32) -> ::windows_core::Result<PROPERTYUI_FLAGS> {
        let mut result__ = ::core::mem::MaybeUninit::<PROPERTYUI_FLAGS>::zeroed();
        (::windows_core::Interface::vtable(self).GetFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(fmtid), ::core::mem::transmute(pid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPERTYUI_FLAGS>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn FormatForDisplay(&self, fmtid: *const ::windows_core::GUID, pid: u32, ppropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, puiff: PROPERTYUI_FORMAT_FLAGS, pwsztext: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FormatForDisplay)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(fmtid), ::core::mem::transmute(pid), ::core::mem::transmute(ppropvar), ::core::mem::transmute(puiff), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pwsztext)), pwsztext.len() as _).ok()
    }
    pub unsafe fn GetHelpInfo(&self, fmtid: *const ::windows_core::GUID, pid: u32, pwszhelpfile: &mut [u16], puhelpid: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetHelpInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(fmtid), ::core::mem::transmute(pid), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pwszhelpfile)), pwszhelpfile.len() as _, ::core::mem::transmute(puhelpid)).ok()
    }
}
impl ::core::convert::From<IPropertyUI> for ::windows_core::IUnknown {
    fn from(value: IPropertyUI) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyUI> for ::windows_core::IUnknown {
    fn from(value: &IPropertyUI) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPropertyUI {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPropertyUI {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPropertyUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPropertyUI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyUI {}
impl ::core::fmt::Debug for IPropertyUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyUI").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPropertyUI {
    type Vtable = IPropertyUI_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x757a7d9f_919a_4118_99d7_dbb208c8cc66);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyUI_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub ParsePropertyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, pfmtid: *mut ::windows_core::GUID, ppid: *mut u32, pcheaten: *mut u32) -> ::windows_core::HRESULT,
    pub GetCannonicalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fmtid: *const ::windows_core::GUID, pid: u32, pwsztext: ::windows_core::PWSTR, cchtext: u32) -> ::windows_core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fmtid: *const ::windows_core::GUID, pid: u32, flags: PROPERTYUI_NAME_FLAGS, pwsztext: ::windows_core::PWSTR, cchtext: u32) -> ::windows_core::HRESULT,
    pub GetPropertyDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fmtid: *const ::windows_core::GUID, pid: u32, pwsztext: ::windows_core::PWSTR, cchtext: u32) -> ::windows_core::HRESULT,
    pub GetDefaultWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fmtid: *const ::windows_core::GUID, pid: u32, pcxchars: *mut u32) -> ::windows_core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fmtid: *const ::windows_core::GUID, pid: u32, pflags: *mut PROPERTYUI_FLAGS) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub FormatForDisplay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fmtid: *const ::windows_core::GUID, pid: u32, ppropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, puiff: PROPERTYUI_FORMAT_FLAGS, pwsztext: ::windows_core::PWSTR, cchtext: u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    FormatForDisplay: usize,
    pub GetHelpInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fmtid: *const ::windows_core::GUID, pid: u32, pwszhelpfile: ::windows_core::PWSTR, cch: u32, puhelpid: *mut u32) -> ::windows_core::HRESULT,
}
pub const InMemoryPropertyStore: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9a02e012_6303_4e1e_b9a1_630f802592c5);
pub const InMemoryPropertyStoreMarshalByValue: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd4ca0e2d_6da7_4b75_a97c_5f306f0eaedc);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromBooleanVector(prgf: &[super::super::super::Foundation::BOOL]) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromBooleanVector(prgf: *const super::super::super::Foundation::BOOL, celems: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        InitPropVariantFromBooleanVector(::core::mem::transmute(::windows_core::as_ptr_or_null(prgf)), prgf.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromBuffer(pv: *const ::core::ffi::c_void, cb: u32) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromBuffer(pv: *const ::core::ffi::c_void, cb: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        InitPropVariantFromBuffer(::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromCLSID(clsid: *const ::windows_core::GUID) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromCLSID(clsid: *const ::windows_core::GUID, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        InitPropVariantFromCLSID(::core::mem::transmute(clsid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromDoubleVector(prgn: &[f64]) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromDoubleVector(prgn: *const f64, celems: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        InitPropVariantFromDoubleVector(::core::mem::transmute(::windows_core::as_ptr_or_null(prgn)), prgn.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromFileTime(pftin: *const super::super::super::Foundation::FILETIME) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromFileTime(pftin: *const super::super::super::Foundation::FILETIME, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        InitPropVariantFromFileTime(::core::mem::transmute(pftin), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromFileTimeVector(prgft: &[super::super::super::Foundation::FILETIME]) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromFileTimeVector(prgft: *const super::super::super::Foundation::FILETIME, celems: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        InitPropVariantFromFileTimeVector(::core::mem::transmute(::windows_core::as_ptr_or_null(prgft)), prgft.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromGUIDAsString(guid: *const ::windows_core::GUID) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromGUIDAsString(guid: *const ::windows_core::GUID, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        InitPropVariantFromGUIDAsString(::core::mem::transmute(guid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromInt16Vector(prgn: &[i16]) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromInt16Vector(prgn: *const i16, celems: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        InitPropVariantFromInt16Vector(::core::mem::transmute(::windows_core::as_ptr_or_null(prgn)), prgn.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromInt32Vector(prgn: &[i32]) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromInt32Vector(prgn: *const i32, celems: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        InitPropVariantFromInt32Vector(::core::mem::transmute(::windows_core::as_ptr_or_null(prgn)), prgn.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromInt64Vector(prgn: &[i64]) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromInt64Vector(prgn: *const i64, celems: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        InitPropVariantFromInt64Vector(::core::mem::transmute(::windows_core::as_ptr_or_null(prgn)), prgn.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromPropVariantVectorElem(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromPropVariantVectorElem(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        InitPropVariantFromPropVariantVectorElem(::core::mem::transmute(propvarin), ::core::mem::transmute(ielem), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromResource<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HINSTANCE>>(hinst: Param0, id: u32) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromResource(hinst: super::super::super::Foundation::HINSTANCE, id: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        InitPropVariantFromResource(hinst.into_param().abi(), ::core::mem::transmute(id), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_Common"))]
#[inline]
pub unsafe fn InitPropVariantFromStrRet(pstrret: *mut super::Common::STRRET, pidl: *const super::Common::ITEMIDLIST, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromStrRet(pstrret: *mut super::Common::STRRET, pidl: *const super::Common::ITEMIDLIST, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT;
        }
        InitPropVariantFromStrRet(::core::mem::transmute(pstrret), ::core::mem::transmute(pidl), ::core::mem::transmute(ppropvar)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromStringAsVector<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(psz: Param0) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromStringAsVector(psz: ::windows_core::PCWSTR, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        InitPropVariantFromStringAsVector(psz.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromStringVector(prgsz: &[::windows_core::PWSTR]) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromStringVector(prgsz: *const ::windows_core::PWSTR, celems: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        InitPropVariantFromStringVector(::core::mem::transmute(::windows_core::as_ptr_or_null(prgsz)), prgsz.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromUInt16Vector(prgn: &[u16]) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromUInt16Vector(prgn: *const u16, celems: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        InitPropVariantFromUInt16Vector(::core::mem::transmute(::windows_core::as_ptr_or_null(prgn)), prgn.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromUInt32Vector(prgn: &[u32]) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromUInt32Vector(prgn: *const u32, celems: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        InitPropVariantFromUInt32Vector(::core::mem::transmute(::windows_core::as_ptr_or_null(prgn)), prgn.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromUInt64Vector(prgn: &[u64]) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantFromUInt64Vector(prgn: *const u64, celems: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        InitPropVariantFromUInt64Vector(::core::mem::transmute(::windows_core::as_ptr_or_null(prgn)), prgn.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantVectorFromPropVariant(propvarsingle: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitPropVariantVectorFromPropVariant(propvarsingle: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvarvector: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        InitPropVariantVectorFromPropVariant(::core::mem::transmute(propvarsingle), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromBooleanArray(prgf: &[super::super::super::Foundation::BOOL]) -> ::windows_core::Result<super::super::super::System::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromBooleanArray(prgf: *const super::super::super::Foundation::BOOL, celems: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>>::zeroed();
        InitVariantFromBooleanArray(::core::mem::transmute(::windows_core::as_ptr_or_null(prgf)), prgf.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromBuffer(pv: *const ::core::ffi::c_void, cb: u32) -> ::windows_core::Result<super::super::super::System::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromBuffer(pv: *const ::core::ffi::c_void, cb: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>>::zeroed();
        InitVariantFromBuffer(::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromDoubleArray(prgn: &[f64]) -> ::windows_core::Result<super::super::super::System::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromDoubleArray(prgn: *const f64, celems: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>>::zeroed();
        InitVariantFromDoubleArray(::core::mem::transmute(::windows_core::as_ptr_or_null(prgn)), prgn.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromFileTime(pft: *const super::super::super::Foundation::FILETIME) -> ::windows_core::Result<super::super::super::System::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromFileTime(pft: *const super::super::super::Foundation::FILETIME, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>>::zeroed();
        InitVariantFromFileTime(::core::mem::transmute(pft), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromFileTimeArray(prgft: &[super::super::super::Foundation::FILETIME]) -> ::windows_core::Result<super::super::super::System::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromFileTimeArray(prgft: *const super::super::super::Foundation::FILETIME, celems: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>>::zeroed();
        InitVariantFromFileTimeArray(::core::mem::transmute(::windows_core::as_ptr_or_null(prgft)), prgft.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromGUIDAsString(guid: *const ::windows_core::GUID) -> ::windows_core::Result<super::super::super::System::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromGUIDAsString(guid: *const ::windows_core::GUID, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>>::zeroed();
        InitVariantFromGUIDAsString(::core::mem::transmute(guid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromInt16Array(prgn: &[i16]) -> ::windows_core::Result<super::super::super::System::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromInt16Array(prgn: *const i16, celems: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>>::zeroed();
        InitVariantFromInt16Array(::core::mem::transmute(::windows_core::as_ptr_or_null(prgn)), prgn.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromInt32Array(prgn: &[i32]) -> ::windows_core::Result<super::super::super::System::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromInt32Array(prgn: *const i32, celems: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>>::zeroed();
        InitVariantFromInt32Array(::core::mem::transmute(::windows_core::as_ptr_or_null(prgn)), prgn.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromInt64Array(prgn: &[i64]) -> ::windows_core::Result<super::super::super::System::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromInt64Array(prgn: *const i64, celems: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>>::zeroed();
        InitVariantFromInt64Array(::core::mem::transmute(::windows_core::as_ptr_or_null(prgn)), prgn.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromResource<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HINSTANCE>>(hinst: Param0, id: u32) -> ::windows_core::Result<super::super::super::System::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromResource(hinst: super::super::super::Foundation::HINSTANCE, id: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>>::zeroed();
        InitVariantFromResource(hinst.into_param().abi(), ::core::mem::transmute(id), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
#[inline]
pub unsafe fn InitVariantFromStrRet(pstrret: *const super::Common::STRRET, pidl: *const super::Common::ITEMIDLIST) -> ::windows_core::Result<super::super::super::System::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromStrRet(pstrret: *const super::Common::STRRET, pidl: *const super::Common::ITEMIDLIST, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>>::zeroed();
        InitVariantFromStrRet(::core::mem::transmute(pstrret), ::core::mem::transmute(pidl), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromStringArray(prgsz: &[::windows_core::PWSTR]) -> ::windows_core::Result<super::super::super::System::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromStringArray(prgsz: *const ::windows_core::PWSTR, celems: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>>::zeroed();
        InitVariantFromStringArray(::core::mem::transmute(::windows_core::as_ptr_or_null(prgsz)), prgsz.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromUInt16Array(prgn: &[u16]) -> ::windows_core::Result<super::super::super::System::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromUInt16Array(prgn: *const u16, celems: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>>::zeroed();
        InitVariantFromUInt16Array(::core::mem::transmute(::windows_core::as_ptr_or_null(prgn)), prgn.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromUInt32Array(prgn: &[u32]) -> ::windows_core::Result<super::super::super::System::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromUInt32Array(prgn: *const u32, celems: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>>::zeroed();
        InitVariantFromUInt32Array(::core::mem::transmute(::windows_core::as_ptr_or_null(prgn)), prgn.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromUInt64Array(prgn: &[u64]) -> ::windows_core::Result<super::super::super::System::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromUInt64Array(prgn: *const u64, celems: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>>::zeroed();
        InitVariantFromUInt64Array(::core::mem::transmute(::windows_core::as_ptr_or_null(prgn)), prgn.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromVariantArrayElem(varin: *const super::super::super::System::Com::VARIANT, ielem: u32) -> ::windows_core::Result<super::super::super::System::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitVariantFromVariantArrayElem(varin: *const super::super::super::System::Com::VARIANT, ielem: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>>::zeroed();
        InitVariantFromVariantArrayElem(::core::mem::transmute(varin), ::core::mem::transmute(ielem), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PDOPSTATUS(pub i32);
pub const PDOPS_RUNNING: PDOPSTATUS = PDOPSTATUS(1i32);
pub const PDOPS_PAUSED: PDOPSTATUS = PDOPSTATUS(2i32);
pub const PDOPS_CANCELLED: PDOPSTATUS = PDOPSTATUS(3i32);
pub const PDOPS_STOPPED: PDOPSTATUS = PDOPSTATUS(4i32);
pub const PDOPS_ERRORS: PDOPSTATUS = PDOPSTATUS(5i32);
impl ::core::marker::Copy for PDOPSTATUS {}
impl ::core::clone::Clone for PDOPSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PDOPSTATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PDOPSTATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for PDOPSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PDOPSTATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PKA_FLAGS(pub u32);
pub const PKA_SET: PKA_FLAGS = PKA_FLAGS(0u32);
pub const PKA_APPEND: PKA_FLAGS = PKA_FLAGS(1u32);
pub const PKA_DELETE: PKA_FLAGS = PKA_FLAGS(2u32);
impl ::core::marker::Copy for PKA_FLAGS {}
impl ::core::clone::Clone for PKA_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PKA_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PKA_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for PKA_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PKA_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PKA_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PKA_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PKA_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PKA_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PKA_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const PKEY_PIDSTR_MAX: u32 = 10u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PLACEHOLDER_STATES(pub u32);
pub const PS_NONE: PLACEHOLDER_STATES = PLACEHOLDER_STATES(0u32);
pub const PS_MARKED_FOR_OFFLINE_AVAILABILITY: PLACEHOLDER_STATES = PLACEHOLDER_STATES(1u32);
pub const PS_FULL_PRIMARY_STREAM_AVAILABLE: PLACEHOLDER_STATES = PLACEHOLDER_STATES(2u32);
pub const PS_CREATE_FILE_ACCESSIBLE: PLACEHOLDER_STATES = PLACEHOLDER_STATES(4u32);
pub const PS_CLOUDFILE_PLACEHOLDER: PLACEHOLDER_STATES = PLACEHOLDER_STATES(8u32);
pub const PS_DEFAULT: PLACEHOLDER_STATES = PLACEHOLDER_STATES(7u32);
pub const PS_ALL: PLACEHOLDER_STATES = PLACEHOLDER_STATES(15u32);
impl ::core::marker::Copy for PLACEHOLDER_STATES {}
impl ::core::clone::Clone for PLACEHOLDER_STATES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PLACEHOLDER_STATES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PLACEHOLDER_STATES {
    type Abi = Self;
}
impl ::core::fmt::Debug for PLACEHOLDER_STATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PLACEHOLDER_STATES").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PLACEHOLDER_STATES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PLACEHOLDER_STATES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PLACEHOLDER_STATES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PLACEHOLDER_STATES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PLACEHOLDER_STATES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROPDESC_AGGREGATION_TYPE(pub i32);
pub const PDAT_DEFAULT: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(0i32);
pub const PDAT_FIRST: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(1i32);
pub const PDAT_SUM: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(2i32);
pub const PDAT_AVERAGE: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(3i32);
pub const PDAT_DATERANGE: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(4i32);
pub const PDAT_UNION: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(5i32);
pub const PDAT_MAX: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(6i32);
pub const PDAT_MIN: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(7i32);
impl ::core::marker::Copy for PROPDESC_AGGREGATION_TYPE {}
impl ::core::clone::Clone for PROPDESC_AGGREGATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPDESC_AGGREGATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PROPDESC_AGGREGATION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROPDESC_AGGREGATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPDESC_AGGREGATION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROPDESC_COLUMNINDEX_TYPE(pub i32);
pub const PDCIT_NONE: PROPDESC_COLUMNINDEX_TYPE = PROPDESC_COLUMNINDEX_TYPE(0i32);
pub const PDCIT_ONDISK: PROPDESC_COLUMNINDEX_TYPE = PROPDESC_COLUMNINDEX_TYPE(1i32);
pub const PDCIT_INMEMORY: PROPDESC_COLUMNINDEX_TYPE = PROPDESC_COLUMNINDEX_TYPE(2i32);
pub const PDCIT_ONDEMAND: PROPDESC_COLUMNINDEX_TYPE = PROPDESC_COLUMNINDEX_TYPE(3i32);
pub const PDCIT_ONDISKALL: PROPDESC_COLUMNINDEX_TYPE = PROPDESC_COLUMNINDEX_TYPE(4i32);
pub const PDCIT_ONDISKVECTOR: PROPDESC_COLUMNINDEX_TYPE = PROPDESC_COLUMNINDEX_TYPE(5i32);
impl ::core::marker::Copy for PROPDESC_COLUMNINDEX_TYPE {}
impl ::core::clone::Clone for PROPDESC_COLUMNINDEX_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPDESC_COLUMNINDEX_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PROPDESC_COLUMNINDEX_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROPDESC_COLUMNINDEX_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPDESC_COLUMNINDEX_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROPDESC_CONDITION_TYPE(pub i32);
pub const PDCOT_NONE: PROPDESC_CONDITION_TYPE = PROPDESC_CONDITION_TYPE(0i32);
pub const PDCOT_STRING: PROPDESC_CONDITION_TYPE = PROPDESC_CONDITION_TYPE(1i32);
pub const PDCOT_SIZE: PROPDESC_CONDITION_TYPE = PROPDESC_CONDITION_TYPE(2i32);
pub const PDCOT_DATETIME: PROPDESC_CONDITION_TYPE = PROPDESC_CONDITION_TYPE(3i32);
pub const PDCOT_BOOLEAN: PROPDESC_CONDITION_TYPE = PROPDESC_CONDITION_TYPE(4i32);
pub const PDCOT_NUMBER: PROPDESC_CONDITION_TYPE = PROPDESC_CONDITION_TYPE(5i32);
impl ::core::marker::Copy for PROPDESC_CONDITION_TYPE {}
impl ::core::clone::Clone for PROPDESC_CONDITION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPDESC_CONDITION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PROPDESC_CONDITION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROPDESC_CONDITION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPDESC_CONDITION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROPDESC_DISPLAYTYPE(pub i32);
pub const PDDT_STRING: PROPDESC_DISPLAYTYPE = PROPDESC_DISPLAYTYPE(0i32);
pub const PDDT_NUMBER: PROPDESC_DISPLAYTYPE = PROPDESC_DISPLAYTYPE(1i32);
pub const PDDT_BOOLEAN: PROPDESC_DISPLAYTYPE = PROPDESC_DISPLAYTYPE(2i32);
pub const PDDT_DATETIME: PROPDESC_DISPLAYTYPE = PROPDESC_DISPLAYTYPE(3i32);
pub const PDDT_ENUMERATED: PROPDESC_DISPLAYTYPE = PROPDESC_DISPLAYTYPE(4i32);
impl ::core::marker::Copy for PROPDESC_DISPLAYTYPE {}
impl ::core::clone::Clone for PROPDESC_DISPLAYTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPDESC_DISPLAYTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PROPDESC_DISPLAYTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROPDESC_DISPLAYTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPDESC_DISPLAYTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROPDESC_ENUMFILTER(pub i32);
pub const PDEF_ALL: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(0i32);
pub const PDEF_SYSTEM: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(1i32);
pub const PDEF_NONSYSTEM: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(2i32);
pub const PDEF_VIEWABLE: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(3i32);
pub const PDEF_QUERYABLE: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(4i32);
pub const PDEF_INFULLTEXTQUERY: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(5i32);
pub const PDEF_COLUMN: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(6i32);
impl ::core::marker::Copy for PROPDESC_ENUMFILTER {}
impl ::core::clone::Clone for PROPDESC_ENUMFILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPDESC_ENUMFILTER {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PROPDESC_ENUMFILTER {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROPDESC_ENUMFILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPDESC_ENUMFILTER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROPDESC_FORMAT_FLAGS(pub u32);
pub const PDFF_DEFAULT: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(0u32);
pub const PDFF_PREFIXNAME: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(1u32);
pub const PDFF_FILENAME: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(2u32);
pub const PDFF_ALWAYSKB: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(4u32);
pub const PDFF_RESERVED_RIGHTTOLEFT: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(8u32);
pub const PDFF_SHORTTIME: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(16u32);
pub const PDFF_LONGTIME: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(32u32);
pub const PDFF_HIDETIME: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(64u32);
pub const PDFF_SHORTDATE: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(128u32);
pub const PDFF_LONGDATE: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(256u32);
pub const PDFF_HIDEDATE: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(512u32);
pub const PDFF_RELATIVEDATE: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(1024u32);
pub const PDFF_USEEDITINVITATION: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(2048u32);
pub const PDFF_READONLY: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(4096u32);
pub const PDFF_NOAUTOREADINGORDER: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(8192u32);
impl ::core::marker::Copy for PROPDESC_FORMAT_FLAGS {}
impl ::core::clone::Clone for PROPDESC_FORMAT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPDESC_FORMAT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PROPDESC_FORMAT_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROPDESC_FORMAT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPDESC_FORMAT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PROPDESC_FORMAT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROPDESC_FORMAT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROPDESC_FORMAT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROPDESC_FORMAT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROPDESC_FORMAT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROPDESC_GROUPING_RANGE(pub i32);
pub const PDGR_DISCRETE: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(0i32);
pub const PDGR_ALPHANUMERIC: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(1i32);
pub const PDGR_SIZE: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(2i32);
pub const PDGR_DYNAMIC: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(3i32);
pub const PDGR_DATE: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(4i32);
pub const PDGR_PERCENT: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(5i32);
pub const PDGR_ENUMERATED: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(6i32);
impl ::core::marker::Copy for PROPDESC_GROUPING_RANGE {}
impl ::core::clone::Clone for PROPDESC_GROUPING_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPDESC_GROUPING_RANGE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PROPDESC_GROUPING_RANGE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROPDESC_GROUPING_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPDESC_GROUPING_RANGE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROPDESC_RELATIVEDESCRIPTION_TYPE(pub i32);
pub const PDRDT_GENERAL: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(0i32);
pub const PDRDT_DATE: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(1i32);
pub const PDRDT_SIZE: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(2i32);
pub const PDRDT_COUNT: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(3i32);
pub const PDRDT_REVISION: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(4i32);
pub const PDRDT_LENGTH: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(5i32);
pub const PDRDT_DURATION: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(6i32);
pub const PDRDT_SPEED: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(7i32);
pub const PDRDT_RATE: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(8i32);
pub const PDRDT_RATING: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(9i32);
pub const PDRDT_PRIORITY: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(10i32);
impl ::core::marker::Copy for PROPDESC_RELATIVEDESCRIPTION_TYPE {}
impl ::core::clone::Clone for PROPDESC_RELATIVEDESCRIPTION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPDESC_RELATIVEDESCRIPTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PROPDESC_RELATIVEDESCRIPTION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROPDESC_RELATIVEDESCRIPTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPDESC_RELATIVEDESCRIPTION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROPDESC_SEARCHINFO_FLAGS(pub u32);
pub const PDSIF_DEFAULT: PROPDESC_SEARCHINFO_FLAGS = PROPDESC_SEARCHINFO_FLAGS(0u32);
pub const PDSIF_ININVERTEDINDEX: PROPDESC_SEARCHINFO_FLAGS = PROPDESC_SEARCHINFO_FLAGS(1u32);
pub const PDSIF_ISCOLUMN: PROPDESC_SEARCHINFO_FLAGS = PROPDESC_SEARCHINFO_FLAGS(2u32);
pub const PDSIF_ISCOLUMNSPARSE: PROPDESC_SEARCHINFO_FLAGS = PROPDESC_SEARCHINFO_FLAGS(4u32);
pub const PDSIF_ALWAYSINCLUDE: PROPDESC_SEARCHINFO_FLAGS = PROPDESC_SEARCHINFO_FLAGS(8u32);
pub const PDSIF_USEFORTYPEAHEAD: PROPDESC_SEARCHINFO_FLAGS = PROPDESC_SEARCHINFO_FLAGS(16u32);
impl ::core::marker::Copy for PROPDESC_SEARCHINFO_FLAGS {}
impl ::core::clone::Clone for PROPDESC_SEARCHINFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPDESC_SEARCHINFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PROPDESC_SEARCHINFO_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROPDESC_SEARCHINFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPDESC_SEARCHINFO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PROPDESC_SEARCHINFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROPDESC_SEARCHINFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROPDESC_SEARCHINFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROPDESC_SEARCHINFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROPDESC_SEARCHINFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROPDESC_SORTDESCRIPTION(pub i32);
pub const PDSD_GENERAL: PROPDESC_SORTDESCRIPTION = PROPDESC_SORTDESCRIPTION(0i32);
pub const PDSD_A_Z: PROPDESC_SORTDESCRIPTION = PROPDESC_SORTDESCRIPTION(1i32);
pub const PDSD_LOWEST_HIGHEST: PROPDESC_SORTDESCRIPTION = PROPDESC_SORTDESCRIPTION(2i32);
pub const PDSD_SMALLEST_BIGGEST: PROPDESC_SORTDESCRIPTION = PROPDESC_SORTDESCRIPTION(3i32);
pub const PDSD_OLDEST_NEWEST: PROPDESC_SORTDESCRIPTION = PROPDESC_SORTDESCRIPTION(4i32);
impl ::core::marker::Copy for PROPDESC_SORTDESCRIPTION {}
impl ::core::clone::Clone for PROPDESC_SORTDESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPDESC_SORTDESCRIPTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PROPDESC_SORTDESCRIPTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROPDESC_SORTDESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPDESC_SORTDESCRIPTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROPDESC_TYPE_FLAGS(pub u32);
pub const PDTF_DEFAULT: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(0u32);
pub const PDTF_MULTIPLEVALUES: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(1u32);
pub const PDTF_ISINNATE: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(2u32);
pub const PDTF_ISGROUP: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(4u32);
pub const PDTF_CANGROUPBY: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(8u32);
pub const PDTF_CANSTACKBY: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(16u32);
pub const PDTF_ISTREEPROPERTY: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(32u32);
pub const PDTF_INCLUDEINFULLTEXTQUERY: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(64u32);
pub const PDTF_ISVIEWABLE: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(128u32);
pub const PDTF_ISQUERYABLE: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(256u32);
pub const PDTF_CANBEPURGED: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(512u32);
pub const PDTF_SEARCHRAWVALUE: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(1024u32);
pub const PDTF_DONTCOERCEEMPTYSTRINGS: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(2048u32);
pub const PDTF_ALWAYSINSUPPLEMENTALSTORE: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(4096u32);
pub const PDTF_ISSYSTEMPROPERTY: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(2147483648u32);
pub const PDTF_MASK_ALL: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(2147491839u32);
impl ::core::marker::Copy for PROPDESC_TYPE_FLAGS {}
impl ::core::clone::Clone for PROPDESC_TYPE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPDESC_TYPE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PROPDESC_TYPE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROPDESC_TYPE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPDESC_TYPE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PROPDESC_TYPE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROPDESC_TYPE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROPDESC_TYPE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROPDESC_TYPE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROPDESC_TYPE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROPDESC_VIEW_FLAGS(pub u32);
pub const PDVF_DEFAULT: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(0u32);
pub const PDVF_CENTERALIGN: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(1u32);
pub const PDVF_RIGHTALIGN: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(2u32);
pub const PDVF_BEGINNEWGROUP: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(4u32);
pub const PDVF_FILLAREA: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(8u32);
pub const PDVF_SORTDESCENDING: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(16u32);
pub const PDVF_SHOWONLYIFPRESENT: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(32u32);
pub const PDVF_SHOWBYDEFAULT: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(64u32);
pub const PDVF_SHOWINPRIMARYLIST: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(128u32);
pub const PDVF_SHOWINSECONDARYLIST: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(256u32);
pub const PDVF_HIDELABEL: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(512u32);
pub const PDVF_HIDDEN: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(2048u32);
pub const PDVF_CANWRAP: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(4096u32);
pub const PDVF_MASK_ALL: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(7167u32);
impl ::core::marker::Copy for PROPDESC_VIEW_FLAGS {}
impl ::core::clone::Clone for PROPDESC_VIEW_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPDESC_VIEW_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PROPDESC_VIEW_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROPDESC_VIEW_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPDESC_VIEW_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PROPDESC_VIEW_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROPDESC_VIEW_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROPDESC_VIEW_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROPDESC_VIEW_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROPDESC_VIEW_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROPENUMTYPE(pub i32);
pub const PET_DISCRETEVALUE: PROPENUMTYPE = PROPENUMTYPE(0i32);
pub const PET_RANGEDVALUE: PROPENUMTYPE = PROPENUMTYPE(1i32);
pub const PET_DEFAULTVALUE: PROPENUMTYPE = PROPENUMTYPE(2i32);
pub const PET_ENDRANGE: PROPENUMTYPE = PROPENUMTYPE(3i32);
impl ::core::marker::Copy for PROPENUMTYPE {}
impl ::core::clone::Clone for PROPENUMTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPENUMTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PROPENUMTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROPENUMTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPENUMTYPE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct PROPERTYKEY {
    pub fmtid: ::windows_core::GUID,
    pub pid: u32,
}
impl ::core::marker::Copy for PROPERTYKEY {}
impl ::core::clone::Clone for PROPERTYKEY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROPERTYKEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROPERTYKEY").field("fmtid", &self.fmtid).field("pid", &self.pid).finish()
    }
}
unsafe impl ::windows_core::Abi for PROPERTYKEY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PROPERTYKEY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPERTYKEY>()) == 0 }
    }
}
impl ::core::cmp::Eq for PROPERTYKEY {}
impl ::core::default::Default for PROPERTYKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROPERTYUI_FLAGS(pub u32);
pub const PUIF_DEFAULT: PROPERTYUI_FLAGS = PROPERTYUI_FLAGS(0u32);
pub const PUIF_RIGHTALIGN: PROPERTYUI_FLAGS = PROPERTYUI_FLAGS(1u32);
pub const PUIF_NOLABELININFOTIP: PROPERTYUI_FLAGS = PROPERTYUI_FLAGS(2u32);
impl ::core::marker::Copy for PROPERTYUI_FLAGS {}
impl ::core::clone::Clone for PROPERTYUI_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPERTYUI_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PROPERTYUI_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROPERTYUI_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPERTYUI_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PROPERTYUI_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROPERTYUI_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROPERTYUI_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROPERTYUI_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROPERTYUI_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROPERTYUI_FORMAT_FLAGS(pub u32);
pub const PUIFFDF_DEFAULT: PROPERTYUI_FORMAT_FLAGS = PROPERTYUI_FORMAT_FLAGS(0u32);
pub const PUIFFDF_RIGHTTOLEFT: PROPERTYUI_FORMAT_FLAGS = PROPERTYUI_FORMAT_FLAGS(1u32);
pub const PUIFFDF_SHORTFORMAT: PROPERTYUI_FORMAT_FLAGS = PROPERTYUI_FORMAT_FLAGS(2u32);
pub const PUIFFDF_NOTIME: PROPERTYUI_FORMAT_FLAGS = PROPERTYUI_FORMAT_FLAGS(4u32);
pub const PUIFFDF_FRIENDLYDATE: PROPERTYUI_FORMAT_FLAGS = PROPERTYUI_FORMAT_FLAGS(8u32);
impl ::core::marker::Copy for PROPERTYUI_FORMAT_FLAGS {}
impl ::core::clone::Clone for PROPERTYUI_FORMAT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPERTYUI_FORMAT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PROPERTYUI_FORMAT_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROPERTYUI_FORMAT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPERTYUI_FORMAT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PROPERTYUI_FORMAT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROPERTYUI_FORMAT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROPERTYUI_FORMAT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROPERTYUI_FORMAT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROPERTYUI_FORMAT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROPERTYUI_NAME_FLAGS(pub u32);
pub const PUIFNF_DEFAULT: PROPERTYUI_NAME_FLAGS = PROPERTYUI_NAME_FLAGS(0u32);
pub const PUIFNF_MNEMONIC: PROPERTYUI_NAME_FLAGS = PROPERTYUI_NAME_FLAGS(1u32);
impl ::core::marker::Copy for PROPERTYUI_NAME_FLAGS {}
impl ::core::clone::Clone for PROPERTYUI_NAME_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPERTYUI_NAME_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PROPERTYUI_NAME_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROPERTYUI_NAME_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPERTYUI_NAME_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PROPERTYUI_NAME_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROPERTYUI_NAME_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROPERTYUI_NAME_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROPERTYUI_NAME_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROPERTYUI_NAME_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct PROPPRG {
    pub flPrg: u16,
    pub flPrgInit: u16,
    pub achTitle: [super::super::super::Foundation::CHAR; 30],
    pub achCmdLine: [super::super::super::Foundation::CHAR; 128],
    pub achWorkDir: [super::super::super::Foundation::CHAR; 64],
    pub wHotKey: u16,
    pub achIconFile: [super::super::super::Foundation::CHAR; 80],
    pub wIconIndex: u16,
    pub dwEnhModeFlags: u32,
    pub dwRealModeFlags: u32,
    pub achOtherFile: [super::super::super::Foundation::CHAR; 80],
    pub achPIFFile: [super::super::super::Foundation::CHAR; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROPPRG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROPPRG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows_core::Abi for PROPPRG {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PROPPRG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPPRG>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PROPPRG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PROPPRG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROPVAR_CHANGE_FLAGS(pub u32);
pub const PVCHF_DEFAULT: PROPVAR_CHANGE_FLAGS = PROPVAR_CHANGE_FLAGS(0u32);
pub const PVCHF_NOVALUEPROP: PROPVAR_CHANGE_FLAGS = PROPVAR_CHANGE_FLAGS(1u32);
pub const PVCHF_ALPHABOOL: PROPVAR_CHANGE_FLAGS = PROPVAR_CHANGE_FLAGS(2u32);
pub const PVCHF_NOUSEROVERRIDE: PROPVAR_CHANGE_FLAGS = PROPVAR_CHANGE_FLAGS(4u32);
pub const PVCHF_LOCALBOOL: PROPVAR_CHANGE_FLAGS = PROPVAR_CHANGE_FLAGS(8u32);
pub const PVCHF_NOHEXSTRING: PROPVAR_CHANGE_FLAGS = PROPVAR_CHANGE_FLAGS(16u32);
impl ::core::marker::Copy for PROPVAR_CHANGE_FLAGS {}
impl ::core::clone::Clone for PROPVAR_CHANGE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPVAR_CHANGE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PROPVAR_CHANGE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROPVAR_CHANGE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPVAR_CHANGE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PROPVAR_CHANGE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROPVAR_CHANGE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROPVAR_CHANGE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROPVAR_CHANGE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROPVAR_CHANGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROPVAR_COMPARE_FLAGS(pub u32);
pub const PVCF_DEFAULT: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(0u32);
pub const PVCF_TREATEMPTYASGREATERTHAN: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(1u32);
pub const PVCF_USESTRCMP: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(2u32);
pub const PVCF_USESTRCMPC: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(4u32);
pub const PVCF_USESTRCMPI: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(8u32);
pub const PVCF_USESTRCMPIC: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(16u32);
pub const PVCF_DIGITSASNUMBERS_CASESENSITIVE: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(32u32);
impl ::core::marker::Copy for PROPVAR_COMPARE_FLAGS {}
impl ::core::clone::Clone for PROPVAR_COMPARE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPVAR_COMPARE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PROPVAR_COMPARE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROPVAR_COMPARE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPVAR_COMPARE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PROPVAR_COMPARE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROPVAR_COMPARE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROPVAR_COMPARE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROPVAR_COMPARE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROPVAR_COMPARE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROPVAR_COMPARE_UNIT(pub i32);
pub const PVCU_DEFAULT: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(0i32);
pub const PVCU_SECOND: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(1i32);
pub const PVCU_MINUTE: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(2i32);
pub const PVCU_HOUR: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(3i32);
pub const PVCU_DAY: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(4i32);
pub const PVCU_MONTH: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(5i32);
pub const PVCU_YEAR: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(6i32);
impl ::core::marker::Copy for PROPVAR_COMPARE_UNIT {}
impl ::core::clone::Clone for PROPVAR_COMPARE_UNIT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPVAR_COMPARE_UNIT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PROPVAR_COMPARE_UNIT {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROPVAR_COMPARE_UNIT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPVAR_COMPARE_UNIT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PSC_STATE(pub i32);
pub const PSC_NORMAL: PSC_STATE = PSC_STATE(0i32);
pub const PSC_NOTINSOURCE: PSC_STATE = PSC_STATE(1i32);
pub const PSC_DIRTY: PSC_STATE = PSC_STATE(2i32);
pub const PSC_READONLY: PSC_STATE = PSC_STATE(3i32);
impl ::core::marker::Copy for PSC_STATE {}
impl ::core::clone::Clone for PSC_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PSC_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PSC_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PSC_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PSC_STATE").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSCoerceToCanonicalValue(key: *const PROPERTYKEY, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSCoerceToCanonicalValue(key: *const PROPERTYKEY, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT;
        }
        PSCoerceToCanonicalValue(::core::mem::transmute(key), ::core::mem::transmute(ppropvar)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PSCreateAdapterFromPropertyStore<'a, Param0: ::windows_core::IntoParam<'a, IPropertyStore>>(pps: Param0, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSCreateAdapterFromPropertyStore(pps: ::windows_core::RawPtr, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        PSCreateAdapterFromPropertyStore(pps.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PSCreateDelayedMultiplexPropertyStore<'a, Param1: ::windows_core::IntoParam<'a, IDelayedPropertyStoreFactory>>(flags: GETPROPERTYSTOREFLAGS, pdpsf: Param1, rgstoreids: &[u32], riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSCreateDelayedMultiplexPropertyStore(flags: GETPROPERTYSTOREFLAGS, pdpsf: ::windows_core::RawPtr, rgstoreids: *const u32, cstores: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        PSCreateDelayedMultiplexPropertyStore(::core::mem::transmute(flags), pdpsf.into_param().abi(), ::core::mem::transmute(::windows_core::as_ptr_or_null(rgstoreids)), rgstoreids.len() as _, ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PSCreateMemoryPropertyStore(riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSCreateMemoryPropertyStore(riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        PSCreateMemoryPropertyStore(::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PSCreateMultiplexPropertyStore(prgpunkstores: &[::core::option::Option<::windows_core::IUnknown>], riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSCreateMultiplexPropertyStore(prgpunkstores: *const *mut ::core::ffi::c_void, cstores: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        PSCreateMultiplexPropertyStore(::core::mem::transmute(::windows_core::as_ptr_or_null(prgpunkstores)), prgpunkstores.len() as _, ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSCreatePropertyChangeArray(rgpropkey: *const PROPERTYKEY, rgflags: *const PKA_FLAGS, rgpropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, cchanges: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSCreatePropertyChangeArray(rgpropkey: *const PROPERTYKEY, rgflags: *const PKA_FLAGS, rgpropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, cchanges: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        PSCreatePropertyChangeArray(::core::mem::transmute(rgpropkey), ::core::mem::transmute(rgflags), ::core::mem::transmute(rgpropvar), ::core::mem::transmute(cchanges), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PSCreatePropertyStoreFromObject<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(punk: Param0, grfmode: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSCreatePropertyStoreFromObject(punk: *mut ::core::ffi::c_void, grfmode: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        PSCreatePropertyStoreFromObject(punk.into_param().abi(), ::core::mem::transmute(grfmode), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSCreatePropertyStoreFromPropertySetStorage<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertySetStorage>>(ppss: Param0, grfmode: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSCreatePropertyStoreFromPropertySetStorage(ppss: ::windows_core::RawPtr, grfmode: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        PSCreatePropertyStoreFromPropertySetStorage(ppss.into_param().abi(), ::core::mem::transmute(grfmode), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSCreateSimplePropertyChange(flags: PKA_FLAGS, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSCreateSimplePropertyChange(flags: PKA_FLAGS, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        PSCreateSimplePropertyChange(::core::mem::transmute(flags), ::core::mem::transmute(key), ::core::mem::transmute(propvar), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PSEnumeratePropertyDescriptions(filteron: PROPDESC_ENUMFILTER, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSEnumeratePropertyDescriptions(filteron: PROPDESC_ENUMFILTER, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        PSEnumeratePropertyDescriptions(::core::mem::transmute(filteron), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSFormatForDisplay(propkey: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS, pwsztext: &mut [u16]) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSFormatForDisplay(propkey: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS, pwsztext: ::windows_core::PWSTR, cchtext: u32) -> ::windows_core::HRESULT;
        }
        PSFormatForDisplay(::core::mem::transmute(propkey), ::core::mem::transmute(propvar), ::core::mem::transmute(pdfflags), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pwsztext)), pwsztext.len() as _).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSFormatForDisplayAlloc(key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS) -> ::windows_core::Result<::windows_core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSFormatForDisplayAlloc(key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        PSFormatForDisplayAlloc(::core::mem::transmute(key), ::core::mem::transmute(propvar), ::core::mem::transmute(pdff), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PSFormatPropertyValue<'a, Param0: ::windows_core::IntoParam<'a, IPropertyStore>, Param1: ::windows_core::IntoParam<'a, IPropertyDescription>>(pps: Param0, ppd: Param1, pdff: PROPDESC_FORMAT_FLAGS) -> ::windows_core::Result<::windows_core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSFormatPropertyValue(pps: ::windows_core::RawPtr, ppd: ::windows_core::RawPtr, pdff: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        PSFormatPropertyValue(pps.into_param().abi(), ppd.into_param().abi(), ::core::mem::transmute(pdff), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSGetImageReferenceForValue(propkey: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<::windows_core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSGetImageReferenceForValue(propkey: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszimageres: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        PSGetImageReferenceForValue(::core::mem::transmute(propkey), ::core::mem::transmute(propvar), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PSGetItemPropertyHandler<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param1: ::windows_core::IntoParam<'a, super::super::super::Foundation::BOOL>>(punkitem: Param0, freadwrite: Param1, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSGetItemPropertyHandler(punkitem: *mut ::core::ffi::c_void, freadwrite: super::super::super::Foundation::BOOL, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        PSGetItemPropertyHandler(punkitem.into_param().abi(), freadwrite.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PSGetItemPropertyHandlerWithCreateObject<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param1: ::windows_core::IntoParam<'a, super::super::super::Foundation::BOOL>, Param2: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(punkitem: Param0, freadwrite: Param1, punkcreateobject: Param2, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSGetItemPropertyHandlerWithCreateObject(punkitem: *mut ::core::ffi::c_void, freadwrite: super::super::super::Foundation::BOOL, punkcreateobject: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        PSGetItemPropertyHandlerWithCreateObject(punkitem.into_param().abi(), freadwrite.into_param().abi(), punkcreateobject.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PSGetNameFromPropertyKey(propkey: *const PROPERTYKEY) -> ::windows_core::Result<::windows_core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSGetNameFromPropertyKey(propkey: *const PROPERTYKEY, ppszcanonicalname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        PSGetNameFromPropertyKey(::core::mem::transmute(propkey), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSGetNamedPropertyFromPropertyStorage<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(psps: *const SERIALIZEDPROPSTORAGE, cb: u32, pszname: Param2) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSGetNamedPropertyFromPropertyStorage(psps: *const SERIALIZEDPROPSTORAGE, cb: u32, pszname: ::windows_core::PCWSTR, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        PSGetNamedPropertyFromPropertyStorage(::core::mem::transmute(psps), ::core::mem::transmute(cb), pszname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PSGetPropertyDescription(propkey: *const PROPERTYKEY, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSGetPropertyDescription(propkey: *const PROPERTYKEY, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        PSGetPropertyDescription(::core::mem::transmute(propkey), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PSGetPropertyDescriptionByName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pszcanonicalname: Param0, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSGetPropertyDescriptionByName(pszcanonicalname: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        PSGetPropertyDescriptionByName(pszcanonicalname.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PSGetPropertyDescriptionListFromString<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pszproplist: Param0, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSGetPropertyDescriptionListFromString(pszproplist: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        PSGetPropertyDescriptionListFromString(pszproplist.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSGetPropertyFromPropertyStorage(psps: *const SERIALIZEDPROPSTORAGE, cb: u32, rpkey: *const PROPERTYKEY) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSGetPropertyFromPropertyStorage(psps: *const SERIALIZEDPROPSTORAGE, cb: u32, rpkey: *const PROPERTYKEY, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        PSGetPropertyFromPropertyStorage(::core::mem::transmute(psps), ::core::mem::transmute(cb), ::core::mem::transmute(rpkey), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PSGetPropertyKeyFromName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pszname: Param0) -> ::windows_core::Result<PROPERTYKEY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSGetPropertyKeyFromName(pszname: ::windows_core::PCWSTR, ppropkey: *mut PROPERTYKEY) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<PROPERTYKEY>::zeroed();
        PSGetPropertyKeyFromName(pszname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPERTYKEY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PSGetPropertySystem(riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSGetPropertySystem(riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        PSGetPropertySystem(::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSGetPropertyValue<'a, Param0: ::windows_core::IntoParam<'a, IPropertyStore>, Param1: ::windows_core::IntoParam<'a, IPropertyDescription>>(pps: Param0, ppd: Param1) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSGetPropertyValue(pps: ::windows_core::RawPtr, ppd: ::windows_core::RawPtr, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        PSGetPropertyValue(pps.into_param().abi(), ppd.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PSLookupPropertyHandlerCLSID<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pszfilepath: Param0) -> ::windows_core::Result<::windows_core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSLookupPropertyHandlerCLSID(pszfilepath: ::windows_core::PCWSTR, pclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        PSLookupPropertyHandlerCLSID(pszfilepath.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_Delete<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(propbag: Param0, propname: Param1) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_Delete(propbag: ::windows_core::RawPtr, propname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT;
        }
        PSPropertyBag_Delete(propbag.into_param().abi(), propname.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadBOOL<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(propbag: Param0, propname: Param1) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadBOOL(propbag: ::windows_core::RawPtr, propname: ::windows_core::PCWSTR, value: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::BOOL>::zeroed();
        PSPropertyBag_ReadBOOL(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadBSTR<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(propbag: Param0, propname: Param1) -> ::windows_core::Result<super::super::super::Foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadBSTR(propbag: ::windows_core::RawPtr, propname: ::windows_core::PCWSTR, value: *mut super::super::super::Foundation::BSTR) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>>::zeroed();
        PSPropertyBag_ReadBSTR(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadDWORD<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(propbag: Param0, propname: Param1) -> ::windows_core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadDWORD(propbag: ::windows_core::RawPtr, propname: ::windows_core::PCWSTR, value: *mut u32) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        PSPropertyBag_ReadDWORD(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadGUID<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(propbag: Param0, propname: Param1) -> ::windows_core::Result<::windows_core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadGUID(propbag: ::windows_core::RawPtr, propname: ::windows_core::PCWSTR, value: *mut ::windows_core::GUID) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        PSPropertyBag_ReadGUID(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadInt<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(propbag: Param0, propname: Param1) -> ::windows_core::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadInt(propbag: ::windows_core::RawPtr, propname: ::windows_core::PCWSTR, value: *mut i32) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        PSPropertyBag_ReadInt(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadLONG<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(propbag: Param0, propname: Param1) -> ::windows_core::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadLONG(propbag: ::windows_core::RawPtr, propname: ::windows_core::PCWSTR, value: *mut i32) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        PSPropertyBag_ReadLONG(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadPOINTL<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(propbag: Param0, propname: Param1) -> ::windows_core::Result<super::super::super::Foundation::POINTL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadPOINTL(propbag: ::windows_core::RawPtr, propname: ::windows_core::PCWSTR, value: *mut super::super::super::Foundation::POINTL) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::POINTL>::zeroed();
        PSPropertyBag_ReadPOINTL(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::POINTL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadPOINTS<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(propbag: Param0, propname: Param1) -> ::windows_core::Result<super::super::super::Foundation::POINTS> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadPOINTS(propbag: ::windows_core::RawPtr, propname: ::windows_core::PCWSTR, value: *mut super::super::super::Foundation::POINTS) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::POINTS>::zeroed();
        PSPropertyBag_ReadPOINTS(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::POINTS>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadPropertyKey<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(propbag: Param0, propname: Param1) -> ::windows_core::Result<PROPERTYKEY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadPropertyKey(propbag: ::windows_core::RawPtr, propname: ::windows_core::PCWSTR, value: *mut PROPERTYKEY) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<PROPERTYKEY>::zeroed();
        PSPropertyBag_ReadPropertyKey(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPERTYKEY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadRECTL<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(propbag: Param0, propname: Param1) -> ::windows_core::Result<super::super::super::Foundation::RECTL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadRECTL(propbag: ::windows_core::RawPtr, propname: ::windows_core::PCWSTR, value: *mut super::super::super::Foundation::RECTL) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::RECTL>::zeroed();
        PSPropertyBag_ReadRECTL(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::RECTL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadSHORT<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(propbag: Param0, propname: Param1) -> ::windows_core::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadSHORT(propbag: ::windows_core::RawPtr, propname: ::windows_core::PCWSTR, value: *mut i16) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        PSPropertyBag_ReadSHORT(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadStr<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(propbag: Param0, propname: Param1, value: &mut [u16]) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadStr(propbag: ::windows_core::RawPtr, propname: ::windows_core::PCWSTR, value: ::windows_core::PWSTR, charactercount: i32) -> ::windows_core::HRESULT;
        }
        PSPropertyBag_ReadStr(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(value)), value.len() as _).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadStrAlloc<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(propbag: Param0, propname: Param1) -> ::windows_core::Result<::windows_core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadStrAlloc(propbag: ::windows_core::RawPtr, propname: ::windows_core::PCWSTR, value: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        PSPropertyBag_ReadStrAlloc(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadStream<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(propbag: Param0, propname: Param1) -> ::windows_core::Result<super::super::super::System::Com::IStream> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadStream(propbag: ::windows_core::RawPtr, propname: ::windows_core::PCWSTR, value: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        PSPropertyBag_ReadStream(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::IStream>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadType<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(propbag: Param0, propname: Param1, var: *mut super::super::super::System::Com::VARIANT, r#type: u16) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadType(propbag: ::windows_core::RawPtr, propname: ::windows_core::PCWSTR, var: *mut super::super::super::System::Com::VARIANT, r#type: u16) -> ::windows_core::HRESULT;
        }
        PSPropertyBag_ReadType(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(var), ::core::mem::transmute(r#type)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadULONGLONG<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(propbag: Param0, propname: Param1) -> ::windows_core::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadULONGLONG(propbag: ::windows_core::RawPtr, propname: ::windows_core::PCWSTR, value: *mut u64) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        PSPropertyBag_ReadULONGLONG(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadUnknown<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(propbag: Param0, propname: Param1, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_ReadUnknown(propbag: ::windows_core::RawPtr, propname: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        PSPropertyBag_ReadUnknown(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_WriteBOOL<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, super::super::super::Foundation::BOOL>>(propbag: Param0, propname: Param1, value: Param2) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WriteBOOL(propbag: ::windows_core::RawPtr, propname: ::windows_core::PCWSTR, value: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT;
        }
        PSPropertyBag_WriteBOOL(propbag.into_param().abi(), propname.into_param().abi(), value.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_WriteBSTR<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, super::super::super::Foundation::BSTR>>(propbag: Param0, propname: Param1, value: Param2) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WriteBSTR(propbag: ::windows_core::RawPtr, propname: ::windows_core::PCWSTR, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_core::HRESULT;
        }
        PSPropertyBag_WriteBSTR(propbag.into_param().abi(), propname.into_param().abi(), value.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteDWORD<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(propbag: Param0, propname: Param1, value: u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WriteDWORD(propbag: ::windows_core::RawPtr, propname: ::windows_core::PCWSTR, value: u32) -> ::windows_core::HRESULT;
        }
        PSPropertyBag_WriteDWORD(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteGUID<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(propbag: Param0, propname: Param1, value: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WriteGUID(propbag: ::windows_core::RawPtr, propname: ::windows_core::PCWSTR, value: *const ::windows_core::GUID) -> ::windows_core::HRESULT;
        }
        PSPropertyBag_WriteGUID(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteInt<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(propbag: Param0, propname: Param1, value: i32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WriteInt(propbag: ::windows_core::RawPtr, propname: ::windows_core::PCWSTR, value: i32) -> ::windows_core::HRESULT;
        }
        PSPropertyBag_WriteInt(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteLONG<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(propbag: Param0, propname: Param1, value: i32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WriteLONG(propbag: ::windows_core::RawPtr, propname: ::windows_core::PCWSTR, value: i32) -> ::windows_core::HRESULT;
        }
        PSPropertyBag_WriteLONG(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_WritePOINTL<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(propbag: Param0, propname: Param1, value: *const super::super::super::Foundation::POINTL) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WritePOINTL(propbag: ::windows_core::RawPtr, propname: ::windows_core::PCWSTR, value: *const super::super::super::Foundation::POINTL) -> ::windows_core::HRESULT;
        }
        PSPropertyBag_WritePOINTL(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_WritePOINTS<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(propbag: Param0, propname: Param1, value: *const super::super::super::Foundation::POINTS) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WritePOINTS(propbag: ::windows_core::RawPtr, propname: ::windows_core::PCWSTR, value: *const super::super::super::Foundation::POINTS) -> ::windows_core::HRESULT;
        }
        PSPropertyBag_WritePOINTS(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WritePropertyKey<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(propbag: Param0, propname: Param1, value: *const PROPERTYKEY) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WritePropertyKey(propbag: ::windows_core::RawPtr, propname: ::windows_core::PCWSTR, value: *const PROPERTYKEY) -> ::windows_core::HRESULT;
        }
        PSPropertyBag_WritePropertyKey(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_WriteRECTL<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(propbag: Param0, propname: Param1, value: *const super::super::super::Foundation::RECTL) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WriteRECTL(propbag: ::windows_core::RawPtr, propname: ::windows_core::PCWSTR, value: *const super::super::super::Foundation::RECTL) -> ::windows_core::HRESULT;
        }
        PSPropertyBag_WriteRECTL(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteSHORT<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(propbag: Param0, propname: Param1, value: i16) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WriteSHORT(propbag: ::windows_core::RawPtr, propname: ::windows_core::PCWSTR, value: i16) -> ::windows_core::HRESULT;
        }
        PSPropertyBag_WriteSHORT(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteStr<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(propbag: Param0, propname: Param1, value: Param2) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WriteStr(propbag: ::windows_core::RawPtr, propname: ::windows_core::PCWSTR, value: ::windows_core::PCWSTR) -> ::windows_core::HRESULT;
        }
        PSPropertyBag_WriteStr(propbag.into_param().abi(), propname.into_param().abi(), value.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteStream<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, super::super::super::System::Com::IStream>>(propbag: Param0, propname: Param1, value: Param2) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WriteStream(propbag: ::windows_core::RawPtr, propname: ::windows_core::PCWSTR, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        PSPropertyBag_WriteStream(propbag.into_param().abi(), propname.into_param().abi(), value.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteULONGLONG<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(propbag: Param0, propname: Param1, value: u64) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WriteULONGLONG(propbag: ::windows_core::RawPtr, propname: ::windows_core::PCWSTR, value: u64) -> ::windows_core::HRESULT;
        }
        PSPropertyBag_WriteULONGLONG(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteUnknown<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyBag>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(propbag: Param0, propname: Param1, punk: Param2) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyBag_WriteUnknown(propbag: ::windows_core::RawPtr, propname: ::windows_core::PCWSTR, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        PSPropertyBag_WriteUnknown(propbag.into_param().abi(), propname.into_param().abi(), punk.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PSPropertyKeyFromString<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pszstring: Param0) -> ::windows_core::Result<PROPERTYKEY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSPropertyKeyFromString(pszstring: ::windows_core::PCWSTR, pkey: *mut PROPERTYKEY) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<PROPERTYKEY>::zeroed();
        PSPropertyKeyFromString(pszstring.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPERTYKEY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PSRefreshPropertySchema() -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSRefreshPropertySchema() -> ::windows_core::HRESULT;
        }
        PSRefreshPropertySchema().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PSRegisterPropertySchema<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pszpath: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSRegisterPropertySchema(pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT;
        }
        PSRegisterPropertySchema(pszpath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSSetPropertyValue<'a, Param0: ::windows_core::IntoParam<'a, IPropertyStore>, Param1: ::windows_core::IntoParam<'a, IPropertyDescription>>(pps: Param0, ppd: Param1, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSSetPropertyValue(pps: ::windows_core::RawPtr, ppd: ::windows_core::RawPtr, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT;
        }
        PSSetPropertyValue(pps.into_param().abi(), ppd.into_param().abi(), ::core::mem::transmute(propvar)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PSStringFromPropertyKey(pkey: *const PROPERTYKEY, psz: &mut [u16]) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSStringFromPropertyKey(pkey: *const PROPERTYKEY, psz: ::windows_core::PWSTR, cch: u32) -> ::windows_core::HRESULT;
        }
        PSStringFromPropertyKey(::core::mem::transmute(pkey), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(psz)), psz.len() as _).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PSTIME_FLAGS(pub u32);
pub const PSTF_UTC: PSTIME_FLAGS = PSTIME_FLAGS(0u32);
pub const PSTF_LOCAL: PSTIME_FLAGS = PSTIME_FLAGS(1u32);
impl ::core::marker::Copy for PSTIME_FLAGS {}
impl ::core::clone::Clone for PSTIME_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PSTIME_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PSTIME_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for PSTIME_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PSTIME_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PSTIME_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PSTIME_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PSTIME_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PSTIME_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PSTIME_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[inline]
pub unsafe fn PSUnregisterPropertySchema<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pszpath: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PSUnregisterPropertySchema(pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT;
        }
        PSUnregisterPropertySchema(pszpath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PifMgr_CloseProperties<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(hprops: Param0, flopt: u32) -> super::super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PifMgr_CloseProperties(hprops: super::super::super::Foundation::HANDLE, flopt: u32) -> super::super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(PifMgr_CloseProperties(hprops.into_param().abi(), ::core::mem::transmute(flopt)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PifMgr_GetProperties<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HANDLE>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(hprops: Param0, pszgroup: Param1, lpprops: *mut ::core::ffi::c_void, cbprops: i32, flopt: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PifMgr_GetProperties(hprops: super::super::super::Foundation::HANDLE, pszgroup: ::windows_core::PCSTR, lpprops: *mut ::core::ffi::c_void, cbprops: i32, flopt: u32) -> i32;
        }
        ::core::mem::transmute(PifMgr_GetProperties(hprops.into_param().abi(), pszgroup.into_param().abi(), ::core::mem::transmute(lpprops), ::core::mem::transmute(cbprops), ::core::mem::transmute(flopt)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PifMgr_OpenProperties<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pszapp: Param0, pszpif: Param1, hinf: u32, flopt: u32) -> super::super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PifMgr_OpenProperties(pszapp: ::windows_core::PCWSTR, pszpif: ::windows_core::PCWSTR, hinf: u32, flopt: u32) -> super::super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(PifMgr_OpenProperties(pszapp.into_param().abi(), pszpif.into_param().abi(), ::core::mem::transmute(hinf), ::core::mem::transmute(flopt)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PifMgr_SetProperties<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HANDLE>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(hprops: Param0, pszgroup: Param1, lpprops: *const ::core::ffi::c_void, cbprops: i32, flopt: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PifMgr_SetProperties(hprops: super::super::super::Foundation::HANDLE, pszgroup: ::windows_core::PCSTR, lpprops: *const ::core::ffi::c_void, cbprops: i32, flopt: u32) -> i32;
        }
        ::core::mem::transmute(PifMgr_SetProperties(hprops.into_param().abi(), pszgroup.into_param().abi(), ::core::mem::transmute(lpprops), ::core::mem::transmute(cbprops), ::core::mem::transmute(flopt)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantChangeType(ppropvardest: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvarsrc: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, flags: PROPVAR_CHANGE_FLAGS, vt: u16) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantChangeType(ppropvardest: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvarsrc: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, flags: PROPVAR_CHANGE_FLAGS, vt: u16) -> ::windows_core::HRESULT;
        }
        PropVariantChangeType(::core::mem::transmute(ppropvardest), ::core::mem::transmute(propvarsrc), ::core::mem::transmute(flags), ::core::mem::transmute(vt)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantCompareEx(propvar1: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, unit: PROPVAR_COMPARE_UNIT, flags: PROPVAR_COMPARE_FLAGS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantCompareEx(propvar1: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, unit: PROPVAR_COMPARE_UNIT, flags: PROPVAR_COMPARE_FLAGS) -> i32;
        }
        ::core::mem::transmute(PropVariantCompareEx(::core::mem::transmute(propvar1), ::core::mem::transmute(propvar2), ::core::mem::transmute(unit), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantGetBooleanElem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantGetBooleanElem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32, pfval: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::BOOL>::zeroed();
        PropVariantGetBooleanElem(::core::mem::transmute(propvar), ::core::mem::transmute(ielem), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantGetDoubleElem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows_core::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantGetDoubleElem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32, pnval: *mut f64) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        PropVariantGetDoubleElem(::core::mem::transmute(propvar), ::core::mem::transmute(ielem), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantGetElementCount(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantGetElementCount(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> u32;
        }
        ::core::mem::transmute(PropVariantGetElementCount(::core::mem::transmute(propvar)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantGetFileTimeElem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows_core::Result<super::super::super::Foundation::FILETIME> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantGetFileTimeElem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32, pftval: *mut super::super::super::Foundation::FILETIME) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::FILETIME>::zeroed();
        PropVariantGetFileTimeElem(::core::mem::transmute(propvar), ::core::mem::transmute(ielem), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::FILETIME>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantGetInt16Elem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows_core::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantGetInt16Elem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32, pnval: *mut i16) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        PropVariantGetInt16Elem(::core::mem::transmute(propvar), ::core::mem::transmute(ielem), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantGetInt32Elem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows_core::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantGetInt32Elem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32, pnval: *mut i32) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        PropVariantGetInt32Elem(::core::mem::transmute(propvar), ::core::mem::transmute(ielem), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantGetInt64Elem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows_core::Result<i64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantGetInt64Elem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32, pnval: *mut i64) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<i64>::zeroed();
        PropVariantGetInt64Elem(::core::mem::transmute(propvar), ::core::mem::transmute(ielem), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantGetStringElem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows_core::Result<::windows_core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantGetStringElem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32, ppszval: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        PropVariantGetStringElem(::core::mem::transmute(propvar), ::core::mem::transmute(ielem), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantGetUInt16Elem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows_core::Result<u16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantGetUInt16Elem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32, pnval: *mut u16) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        PropVariantGetUInt16Elem(::core::mem::transmute(propvar), ::core::mem::transmute(ielem), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantGetUInt32Elem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows_core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantGetUInt32Elem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32, pnval: *mut u32) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        PropVariantGetUInt32Elem(::core::mem::transmute(propvar), ::core::mem::transmute(ielem), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantGetUInt64Elem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows_core::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantGetUInt64Elem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32, pnval: *mut u64) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        PropVariantGetUInt64Elem(::core::mem::transmute(propvar), ::core::mem::transmute(ielem), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToBSTR(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<super::super::super::Foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToBSTR(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pbstrout: *mut super::super::super::Foundation::BSTR) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>>::zeroed();
        PropVariantToBSTR(::core::mem::transmute(propvar), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToBoolean(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToBoolean(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pfret: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::BOOL>::zeroed();
        PropVariantToBoolean(::core::mem::transmute(propvarin), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToBooleanVector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgf: &mut [super::super::super::Foundation::BOOL], pcelem: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToBooleanVector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgf: *mut super::super::super::Foundation::BOOL, crgf: u32, pcelem: *mut u32) -> ::windows_core::HRESULT;
        }
        PropVariantToBooleanVector(::core::mem::transmute(propvar), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(prgf)), prgf.len() as _, ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToBooleanVectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgf: *mut *mut super::super::super::Foundation::BOOL, pcelem: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToBooleanVectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgf: *mut *mut super::super::super::Foundation::BOOL, pcelem: *mut u32) -> ::windows_core::HRESULT;
        }
        PropVariantToBooleanVectorAlloc(::core::mem::transmute(propvar), ::core::mem::transmute(pprgf), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToBooleanWithDefault<'a, Param1: ::windows_core::IntoParam<'a, super::super::super::Foundation::BOOL>>(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, fdefault: Param1) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToBooleanWithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, fdefault: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(PropVariantToBooleanWithDefault(::core::mem::transmute(propvarin), fdefault.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToBuffer(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pv: *mut ::core::ffi::c_void, cb: u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToBuffer(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pv: *mut ::core::ffi::c_void, cb: u32) -> ::windows_core::HRESULT;
        }
        PropVariantToBuffer(::core::mem::transmute(propvar), ::core::mem::transmute(pv), ::core::mem::transmute(cb)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToDouble(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToDouble(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdblret: *mut f64) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        PropVariantToDouble(::core::mem::transmute(propvarin), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToDoubleVector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: &mut [f64], pcelem: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToDoubleVector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: *mut f64, crgn: u32, pcelem: *mut u32) -> ::windows_core::HRESULT;
        }
        PropVariantToDoubleVector(::core::mem::transmute(propvar), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(prgn)), prgn.len() as _, ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToDoubleVectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut f64, pcelem: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToDoubleVectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut f64, pcelem: *mut u32) -> ::windows_core::HRESULT;
        }
        PropVariantToDoubleVectorAlloc(::core::mem::transmute(propvar), ::core::mem::transmute(pprgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToDoubleWithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, dbldefault: f64) -> f64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToDoubleWithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, dbldefault: f64) -> f64;
        }
        ::core::mem::transmute(PropVariantToDoubleWithDefault(::core::mem::transmute(propvarin), ::core::mem::transmute(dbldefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToFileTime(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pstfout: PSTIME_FLAGS) -> ::windows_core::Result<super::super::super::Foundation::FILETIME> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToFileTime(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pstfout: PSTIME_FLAGS, pftout: *mut super::super::super::Foundation::FILETIME) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::FILETIME>::zeroed();
        PropVariantToFileTime(::core::mem::transmute(propvar), ::core::mem::transmute(pstfout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::FILETIME>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToFileTimeVector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgft: &mut [super::super::super::Foundation::FILETIME], pcelem: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToFileTimeVector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgft: *mut super::super::super::Foundation::FILETIME, crgft: u32, pcelem: *mut u32) -> ::windows_core::HRESULT;
        }
        PropVariantToFileTimeVector(::core::mem::transmute(propvar), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(prgft)), prgft.len() as _, ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToFileTimeVectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgft: *mut *mut super::super::super::Foundation::FILETIME, pcelem: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToFileTimeVectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgft: *mut *mut super::super::super::Foundation::FILETIME, pcelem: *mut u32) -> ::windows_core::HRESULT;
        }
        PropVariantToFileTimeVectorAlloc(::core::mem::transmute(propvar), ::core::mem::transmute(pprgft), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToGUID(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<::windows_core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToGUID(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        PropVariantToGUID(::core::mem::transmute(propvar), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToInt16(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToInt16(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, piret: *mut i16) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        PropVariantToInt16(::core::mem::transmute(propvarin), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToInt16Vector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: &mut [i16], pcelem: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToInt16Vector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: *mut i16, crgn: u32, pcelem: *mut u32) -> ::windows_core::HRESULT;
        }
        PropVariantToInt16Vector(::core::mem::transmute(propvar), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(prgn)), prgn.len() as _, ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToInt16VectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut i16, pcelem: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToInt16VectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut i16, pcelem: *mut u32) -> ::windows_core::HRESULT;
        }
        PropVariantToInt16VectorAlloc(::core::mem::transmute(propvar), ::core::mem::transmute(pprgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToInt16WithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, idefault: i16) -> i16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToInt16WithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, idefault: i16) -> i16;
        }
        ::core::mem::transmute(PropVariantToInt16WithDefault(::core::mem::transmute(propvarin), ::core::mem::transmute(idefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToInt32(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToInt32(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, plret: *mut i32) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        PropVariantToInt32(::core::mem::transmute(propvarin), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToInt32Vector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: &mut [i32], pcelem: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToInt32Vector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: *mut i32, crgn: u32, pcelem: *mut u32) -> ::windows_core::HRESULT;
        }
        PropVariantToInt32Vector(::core::mem::transmute(propvar), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(prgn)), prgn.len() as _, ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToInt32VectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut i32, pcelem: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToInt32VectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut i32, pcelem: *mut u32) -> ::windows_core::HRESULT;
        }
        PropVariantToInt32VectorAlloc(::core::mem::transmute(propvar), ::core::mem::transmute(pprgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToInt32WithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ldefault: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToInt32WithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ldefault: i32) -> i32;
        }
        ::core::mem::transmute(PropVariantToInt32WithDefault(::core::mem::transmute(propvarin), ::core::mem::transmute(ldefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToInt64(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<i64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToInt64(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pllret: *mut i64) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<i64>::zeroed();
        PropVariantToInt64(::core::mem::transmute(propvarin), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToInt64Vector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: &mut [i64], pcelem: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToInt64Vector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: *mut i64, crgn: u32, pcelem: *mut u32) -> ::windows_core::HRESULT;
        }
        PropVariantToInt64Vector(::core::mem::transmute(propvar), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(prgn)), prgn.len() as _, ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToInt64VectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut i64, pcelem: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToInt64VectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut i64, pcelem: *mut u32) -> ::windows_core::HRESULT;
        }
        PropVariantToInt64VectorAlloc(::core::mem::transmute(propvar), ::core::mem::transmute(pprgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToInt64WithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, lldefault: i64) -> i64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToInt64WithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, lldefault: i64) -> i64;
        }
        ::core::mem::transmute(PropVariantToInt64WithDefault(::core::mem::transmute(propvarin), ::core::mem::transmute(lldefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_Common"))]
#[inline]
pub unsafe fn PropVariantToStrRet(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<super::Common::STRRET> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToStrRet(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pstrret: *mut super::Common::STRRET) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<super::Common::STRRET>::zeroed();
        PropVariantToStrRet(::core::mem::transmute(propvar), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Common::STRRET>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToString(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, psz: &mut [u16]) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToString(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, psz: ::windows_core::PWSTR, cch: u32) -> ::windows_core::HRESULT;
        }
        PropVariantToString(::core::mem::transmute(propvar), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(psz)), psz.len() as _).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToStringAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<::windows_core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToStringAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszout: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        PropVariantToStringAlloc(::core::mem::transmute(propvar), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToStringVector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgsz: &mut [::windows_core::PWSTR], pcelem: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToStringVector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgsz: *mut ::windows_core::PWSTR, crgsz: u32, pcelem: *mut u32) -> ::windows_core::HRESULT;
        }
        PropVariantToStringVector(::core::mem::transmute(propvar), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(prgsz)), prgsz.len() as _, ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToStringVectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgsz: *mut *mut ::windows_core::PWSTR, pcelem: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToStringVectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgsz: *mut *mut ::windows_core::PWSTR, pcelem: *mut u32) -> ::windows_core::HRESULT;
        }
        PropVariantToStringVectorAlloc(::core::mem::transmute(propvar), ::core::mem::transmute(pprgsz), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToStringWithDefault<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pszdefault: Param1) -> ::windows_core::PWSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToStringWithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pszdefault: ::windows_core::PCWSTR) -> ::windows_core::PWSTR;
        }
        ::core::mem::transmute(PropVariantToStringWithDefault(::core::mem::transmute(propvarin), pszdefault.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToUInt16(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<u16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToUInt16(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, puiret: *mut u16) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        PropVariantToUInt16(::core::mem::transmute(propvarin), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToUInt16Vector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: &mut [u16], pcelem: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToUInt16Vector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: *mut u16, crgn: u32, pcelem: *mut u32) -> ::windows_core::HRESULT;
        }
        PropVariantToUInt16Vector(::core::mem::transmute(propvar), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(prgn)), prgn.len() as _, ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToUInt16VectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut u16, pcelem: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToUInt16VectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut u16, pcelem: *mut u32) -> ::windows_core::HRESULT;
        }
        PropVariantToUInt16VectorAlloc(::core::mem::transmute(propvar), ::core::mem::transmute(pprgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToUInt16WithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, uidefault: u16) -> u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToUInt16WithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, uidefault: u16) -> u16;
        }
        ::core::mem::transmute(PropVariantToUInt16WithDefault(::core::mem::transmute(propvarin), ::core::mem::transmute(uidefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToUInt32(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToUInt32(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pulret: *mut u32) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        PropVariantToUInt32(::core::mem::transmute(propvarin), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToUInt32Vector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: &mut [u32], pcelem: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToUInt32Vector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: *mut u32, crgn: u32, pcelem: *mut u32) -> ::windows_core::HRESULT;
        }
        PropVariantToUInt32Vector(::core::mem::transmute(propvar), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(prgn)), prgn.len() as _, ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToUInt32VectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut u32, pcelem: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToUInt32VectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut u32, pcelem: *mut u32) -> ::windows_core::HRESULT;
        }
        PropVariantToUInt32VectorAlloc(::core::mem::transmute(propvar), ::core::mem::transmute(pprgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToUInt32WithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, uldefault: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToUInt32WithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, uldefault: u32) -> u32;
        }
        ::core::mem::transmute(PropVariantToUInt32WithDefault(::core::mem::transmute(propvarin), ::core::mem::transmute(uldefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToUInt64(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToUInt64(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pullret: *mut u64) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        PropVariantToUInt64(::core::mem::transmute(propvarin), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToUInt64Vector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: &mut [u64], pcelem: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToUInt64Vector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: *mut u64, crgn: u32, pcelem: *mut u32) -> ::windows_core::HRESULT;
        }
        PropVariantToUInt64Vector(::core::mem::transmute(propvar), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(prgn)), prgn.len() as _, ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToUInt64VectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut u64, pcelem: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToUInt64VectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut u64, pcelem: *mut u32) -> ::windows_core::HRESULT;
        }
        PropVariantToUInt64VectorAlloc(::core::mem::transmute(propvar), ::core::mem::transmute(pprgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToUInt64WithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ulldefault: u64) -> u64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToUInt64WithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ulldefault: u64) -> u64;
        }
        ::core::mem::transmute(PropVariantToUInt64WithDefault(::core::mem::transmute(propvarin), ::core::mem::transmute(ulldefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn PropVariantToVariant(ppropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<super::super::super::System::Com::VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToVariant(ppropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>>::zeroed();
        PropVariantToVariant(::core::mem::transmute(ppropvar), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToWinRTPropertyValue<T: ::windows_core::Interface>(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, result__: *mut ::core::option::Option<T>) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantToWinRTPropertyValue(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        PropVariantToWinRTPropertyValue(::core::mem::transmute(propvar), &<T as ::windows_core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const PropertySystem: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb8967f85_58ae_4f46_9fb2_5d7904798f4b);
#[repr(C)]
pub struct SERIALIZEDPROPSTORAGE(pub u8);
#[inline]
pub unsafe fn SHAddDefaultPropertiesByExt<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, IPropertyStore>>(pszext: Param0, ppropstore: Param1) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SHAddDefaultPropertiesByExt(pszext: ::windows_core::PCWSTR, ppropstore: ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        SHAddDefaultPropertiesByExt(pszext.into_param().abi(), ppropstore.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SHGetPropertyStoreForWindow<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>>(hwnd: Param0, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SHGetPropertyStoreForWindow(hwnd: super::super::super::Foundation::HWND, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        SHGetPropertyStoreForWindow(hwnd.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_Shell_Common")]
#[inline]
pub unsafe fn SHGetPropertyStoreFromIDList(pidl: *const super::Common::ITEMIDLIST, flags: GETPROPERTYSTOREFLAGS, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SHGetPropertyStoreFromIDList(pidl: *const super::Common::ITEMIDLIST, flags: GETPROPERTYSTOREFLAGS, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        SHGetPropertyStoreFromIDList(::core::mem::transmute(pidl), ::core::mem::transmute(flags), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SHGetPropertyStoreFromParsingName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, super::super::super::System::Com::IBindCtx>>(pszpath: Param0, pbc: Param1, flags: GETPROPERTYSTOREFLAGS, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SHGetPropertyStoreFromParsingName(pszpath: ::windows_core::PCWSTR, pbc: ::windows_core::RawPtr, flags: GETPROPERTYSTOREFLAGS, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        SHGetPropertyStoreFromParsingName(pszpath.into_param().abi(), pbc.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn SHPropStgCreate<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertySetStorage>>(psstg: Param0, fmtid: *const ::windows_core::GUID, pclsid: *const ::windows_core::GUID, grfflags: u32, grfmode: u32, dwdisposition: u32, ppstg: *mut ::core::option::Option<super::super::super::System::Com::StructuredStorage::IPropertyStorage>, pucodepage: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SHPropStgCreate(psstg: ::windows_core::RawPtr, fmtid: *const ::windows_core::GUID, pclsid: *const ::windows_core::GUID, grfflags: u32, grfmode: u32, dwdisposition: u32, ppstg: *mut ::windows_core::RawPtr, pucodepage: *mut u32) -> ::windows_core::HRESULT;
        }
        SHPropStgCreate(psstg.into_param().abi(), ::core::mem::transmute(fmtid), ::core::mem::transmute(pclsid), ::core::mem::transmute(grfflags), ::core::mem::transmute(grfmode), ::core::mem::transmute(dwdisposition), ::core::mem::transmute(ppstg), ::core::mem::transmute(pucodepage)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn SHPropStgReadMultiple<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyStorage>>(pps: Param0, ucodepage: u32, rgpspec: &[super::super::super::System::Com::StructuredStorage::PROPSPEC]) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SHPropStgReadMultiple(pps: ::windows_core::RawPtr, ucodepage: u32, cpspec: u32, rgpspec: *const super::super::super::System::Com::StructuredStorage::PROPSPEC, rgvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        SHPropStgReadMultiple(pps.into_param().abi(), ::core::mem::transmute(ucodepage), rgpspec.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(rgpspec)), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn SHPropStgWriteMultiple<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IPropertyStorage>>(pps: Param0, pucodepage: *mut u32, cpspec: u32, rgpspec: *const super::super::super::System::Com::StructuredStorage::PROPSPEC, rgvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT, propidnamefirst: u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SHPropStgWriteMultiple(pps: ::windows_core::RawPtr, pucodepage: *mut u32, cpspec: u32, rgpspec: *const super::super::super::System::Com::StructuredStorage::PROPSPEC, rgvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT, propidnamefirst: u32) -> ::windows_core::HRESULT;
        }
        SHPropStgWriteMultiple(pps.into_param().abi(), ::core::mem::transmute(pucodepage), ::core::mem::transmute(cpspec), ::core::mem::transmute(rgpspec), ::core::mem::transmute(rgvar), ::core::mem::transmute(propidnamefirst)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SYNC_ENGINE_STATE_FLAGS(pub u32);
pub const SESF_NONE: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(0u32);
pub const SESF_SERVICE_QUOTA_NEARING_LIMIT: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(1u32);
pub const SESF_SERVICE_QUOTA_EXCEEDED_LIMIT: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(2u32);
pub const SESF_AUTHENTICATION_ERROR: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(4u32);
pub const SESF_PAUSED_DUE_TO_METERED_NETWORK: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(8u32);
pub const SESF_PAUSED_DUE_TO_DISK_SPACE_FULL: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(16u32);
pub const SESF_PAUSED_DUE_TO_CLIENT_POLICY: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(32u32);
pub const SESF_PAUSED_DUE_TO_SERVICE_POLICY: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(64u32);
pub const SESF_SERVICE_UNAVAILABLE: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(128u32);
pub const SESF_PAUSED_DUE_TO_USER_REQUEST: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(256u32);
pub const SESF_ALL_FLAGS: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(511u32);
impl ::core::marker::Copy for SYNC_ENGINE_STATE_FLAGS {}
impl ::core::clone::Clone for SYNC_ENGINE_STATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYNC_ENGINE_STATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SYNC_ENGINE_STATE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for SYNC_ENGINE_STATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNC_ENGINE_STATE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SYNC_ENGINE_STATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SYNC_ENGINE_STATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SYNC_ENGINE_STATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SYNC_ENGINE_STATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SYNC_ENGINE_STATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SYNC_TRANSFER_STATUS(pub u32);
pub const STS_NONE: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(0u32);
pub const STS_NEEDSUPLOAD: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(1u32);
pub const STS_NEEDSDOWNLOAD: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(2u32);
pub const STS_TRANSFERRING: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(4u32);
pub const STS_PAUSED: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(8u32);
pub const STS_HASERROR: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(16u32);
pub const STS_FETCHING_METADATA: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(32u32);
pub const STS_USER_REQUESTED_REFRESH: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(64u32);
pub const STS_HASWARNING: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(128u32);
pub const STS_EXCLUDED: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(256u32);
pub const STS_INCOMPLETE: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(512u32);
pub const STS_PLACEHOLDER_IFEMPTY: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(1024u32);
impl ::core::marker::Copy for SYNC_TRANSFER_STATUS {}
impl ::core::clone::Clone for SYNC_TRANSFER_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYNC_TRANSFER_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SYNC_TRANSFER_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for SYNC_TRANSFER_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNC_TRANSFER_STATUS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SYNC_TRANSFER_STATUS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SYNC_TRANSFER_STATUS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SYNC_TRANSFER_STATUS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SYNC_TRANSFER_STATUS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SYNC_TRANSFER_STATUS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantCompare(var1: *const super::super::super::System::Com::VARIANT, var2: *const super::super::super::System::Com::VARIANT) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantCompare(var1: *const super::super::super::System::Com::VARIANT, var2: *const super::super::super::System::Com::VARIANT) -> i32;
        }
        ::core::mem::transmute(VariantCompare(::core::mem::transmute(var1), ::core::mem::transmute(var2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetBooleanElem(var: *const super::super::super::System::Com::VARIANT, ielem: u32) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantGetBooleanElem(var: *const super::super::super::System::Com::VARIANT, ielem: u32, pfval: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::BOOL>::zeroed();
        VariantGetBooleanElem(::core::mem::transmute(var), ::core::mem::transmute(ielem), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetDoubleElem(var: *const super::super::super::System::Com::VARIANT, ielem: u32) -> ::windows_core::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantGetDoubleElem(var: *const super::super::super::System::Com::VARIANT, ielem: u32, pnval: *mut f64) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        VariantGetDoubleElem(::core::mem::transmute(var), ::core::mem::transmute(ielem), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetElementCount(varin: *const super::super::super::System::Com::VARIANT) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantGetElementCount(varin: *const super::super::super::System::Com::VARIANT) -> u32;
        }
        ::core::mem::transmute(VariantGetElementCount(::core::mem::transmute(varin)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetInt16Elem(var: *const super::super::super::System::Com::VARIANT, ielem: u32) -> ::windows_core::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantGetInt16Elem(var: *const super::super::super::System::Com::VARIANT, ielem: u32, pnval: *mut i16) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        VariantGetInt16Elem(::core::mem::transmute(var), ::core::mem::transmute(ielem), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetInt32Elem(var: *const super::super::super::System::Com::VARIANT, ielem: u32) -> ::windows_core::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantGetInt32Elem(var: *const super::super::super::System::Com::VARIANT, ielem: u32, pnval: *mut i32) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        VariantGetInt32Elem(::core::mem::transmute(var), ::core::mem::transmute(ielem), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetInt64Elem(var: *const super::super::super::System::Com::VARIANT, ielem: u32) -> ::windows_core::Result<i64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantGetInt64Elem(var: *const super::super::super::System::Com::VARIANT, ielem: u32, pnval: *mut i64) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<i64>::zeroed();
        VariantGetInt64Elem(::core::mem::transmute(var), ::core::mem::transmute(ielem), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetStringElem(var: *const super::super::super::System::Com::VARIANT, ielem: u32) -> ::windows_core::Result<::windows_core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantGetStringElem(var: *const super::super::super::System::Com::VARIANT, ielem: u32, ppszval: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        VariantGetStringElem(::core::mem::transmute(var), ::core::mem::transmute(ielem), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetUInt16Elem(var: *const super::super::super::System::Com::VARIANT, ielem: u32) -> ::windows_core::Result<u16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantGetUInt16Elem(var: *const super::super::super::System::Com::VARIANT, ielem: u32, pnval: *mut u16) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        VariantGetUInt16Elem(::core::mem::transmute(var), ::core::mem::transmute(ielem), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetUInt32Elem(var: *const super::super::super::System::Com::VARIANT, ielem: u32) -> ::windows_core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantGetUInt32Elem(var: *const super::super::super::System::Com::VARIANT, ielem: u32, pnval: *mut u32) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        VariantGetUInt32Elem(::core::mem::transmute(var), ::core::mem::transmute(ielem), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetUInt64Elem(var: *const super::super::super::System::Com::VARIANT, ielem: u32) -> ::windows_core::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantGetUInt64Elem(var: *const super::super::super::System::Com::VARIANT, ielem: u32, pnval: *mut u64) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        VariantGetUInt64Elem(::core::mem::transmute(var), ::core::mem::transmute(ielem), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToBoolean(varin: *const super::super::super::System::Com::VARIANT) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToBoolean(varin: *const super::super::super::System::Com::VARIANT, pfret: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::BOOL>::zeroed();
        VariantToBoolean(::core::mem::transmute(varin), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToBooleanArray(var: *const super::super::super::System::Com::VARIANT, prgf: &mut [super::super::super::Foundation::BOOL], pcelem: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToBooleanArray(var: *const super::super::super::System::Com::VARIANT, prgf: *mut super::super::super::Foundation::BOOL, crgn: u32, pcelem: *mut u32) -> ::windows_core::HRESULT;
        }
        VariantToBooleanArray(::core::mem::transmute(var), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(prgf)), prgf.len() as _, ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToBooleanArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgf: *mut *mut super::super::super::Foundation::BOOL, pcelem: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToBooleanArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgf: *mut *mut super::super::super::Foundation::BOOL, pcelem: *mut u32) -> ::windows_core::HRESULT;
        }
        VariantToBooleanArrayAlloc(::core::mem::transmute(var), ::core::mem::transmute(pprgf), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToBooleanWithDefault<'a, Param1: ::windows_core::IntoParam<'a, super::super::super::Foundation::BOOL>>(varin: *const super::super::super::System::Com::VARIANT, fdefault: Param1) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToBooleanWithDefault(varin: *const super::super::super::System::Com::VARIANT, fdefault: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(VariantToBooleanWithDefault(::core::mem::transmute(varin), fdefault.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToBuffer(varin: *const super::super::super::System::Com::VARIANT, pv: *mut ::core::ffi::c_void, cb: u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToBuffer(varin: *const super::super::super::System::Com::VARIANT, pv: *mut ::core::ffi::c_void, cb: u32) -> ::windows_core::HRESULT;
        }
        VariantToBuffer(::core::mem::transmute(varin), ::core::mem::transmute(pv), ::core::mem::transmute(cb)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToDosDateTime(varin: *const super::super::super::System::Com::VARIANT, pwdate: *mut u16, pwtime: *mut u16) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToDosDateTime(varin: *const super::super::super::System::Com::VARIANT, pwdate: *mut u16, pwtime: *mut u16) -> ::windows_core::HRESULT;
        }
        VariantToDosDateTime(::core::mem::transmute(varin), ::core::mem::transmute(pwdate), ::core::mem::transmute(pwtime)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToDouble(varin: *const super::super::super::System::Com::VARIANT) -> ::windows_core::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToDouble(varin: *const super::super::super::System::Com::VARIANT, pdblret: *mut f64) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        VariantToDouble(::core::mem::transmute(varin), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToDoubleArray(var: *const super::super::super::System::Com::VARIANT, prgn: &mut [f64], pcelem: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToDoubleArray(var: *const super::super::super::System::Com::VARIANT, prgn: *mut f64, crgn: u32, pcelem: *mut u32) -> ::windows_core::HRESULT;
        }
        VariantToDoubleArray(::core::mem::transmute(var), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(prgn)), prgn.len() as _, ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToDoubleArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut f64, pcelem: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToDoubleArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut f64, pcelem: *mut u32) -> ::windows_core::HRESULT;
        }
        VariantToDoubleArrayAlloc(::core::mem::transmute(var), ::core::mem::transmute(pprgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToDoubleWithDefault(varin: *const super::super::super::System::Com::VARIANT, dbldefault: f64) -> f64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToDoubleWithDefault(varin: *const super::super::super::System::Com::VARIANT, dbldefault: f64) -> f64;
        }
        ::core::mem::transmute(VariantToDoubleWithDefault(::core::mem::transmute(varin), ::core::mem::transmute(dbldefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToFileTime(varin: *const super::super::super::System::Com::VARIANT, stfout: PSTIME_FLAGS) -> ::windows_core::Result<super::super::super::Foundation::FILETIME> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToFileTime(varin: *const super::super::super::System::Com::VARIANT, stfout: PSTIME_FLAGS, pftout: *mut super::super::super::Foundation::FILETIME) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::FILETIME>::zeroed();
        VariantToFileTime(::core::mem::transmute(varin), ::core::mem::transmute(stfout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::FILETIME>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToGUID(varin: *const super::super::super::System::Com::VARIANT) -> ::windows_core::Result<::windows_core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToGUID(varin: *const super::super::super::System::Com::VARIANT, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        VariantToGUID(::core::mem::transmute(varin), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt16(varin: *const super::super::super::System::Com::VARIANT) -> ::windows_core::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToInt16(varin: *const super::super::super::System::Com::VARIANT, piret: *mut i16) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        VariantToInt16(::core::mem::transmute(varin), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt16Array(var: *const super::super::super::System::Com::VARIANT, prgn: &mut [i16], pcelem: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToInt16Array(var: *const super::super::super::System::Com::VARIANT, prgn: *mut i16, crgn: u32, pcelem: *mut u32) -> ::windows_core::HRESULT;
        }
        VariantToInt16Array(::core::mem::transmute(var), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(prgn)), prgn.len() as _, ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt16ArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut i16, pcelem: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToInt16ArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut i16, pcelem: *mut u32) -> ::windows_core::HRESULT;
        }
        VariantToInt16ArrayAlloc(::core::mem::transmute(var), ::core::mem::transmute(pprgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt16WithDefault(varin: *const super::super::super::System::Com::VARIANT, idefault: i16) -> i16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToInt16WithDefault(varin: *const super::super::super::System::Com::VARIANT, idefault: i16) -> i16;
        }
        ::core::mem::transmute(VariantToInt16WithDefault(::core::mem::transmute(varin), ::core::mem::transmute(idefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt32(varin: *const super::super::super::System::Com::VARIANT) -> ::windows_core::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToInt32(varin: *const super::super::super::System::Com::VARIANT, plret: *mut i32) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        VariantToInt32(::core::mem::transmute(varin), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt32Array(var: *const super::super::super::System::Com::VARIANT, prgn: &mut [i32], pcelem: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToInt32Array(var: *const super::super::super::System::Com::VARIANT, prgn: *mut i32, crgn: u32, pcelem: *mut u32) -> ::windows_core::HRESULT;
        }
        VariantToInt32Array(::core::mem::transmute(var), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(prgn)), prgn.len() as _, ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt32ArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut i32, pcelem: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToInt32ArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut i32, pcelem: *mut u32) -> ::windows_core::HRESULT;
        }
        VariantToInt32ArrayAlloc(::core::mem::transmute(var), ::core::mem::transmute(pprgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt32WithDefault(varin: *const super::super::super::System::Com::VARIANT, ldefault: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToInt32WithDefault(varin: *const super::super::super::System::Com::VARIANT, ldefault: i32) -> i32;
        }
        ::core::mem::transmute(VariantToInt32WithDefault(::core::mem::transmute(varin), ::core::mem::transmute(ldefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt64(varin: *const super::super::super::System::Com::VARIANT) -> ::windows_core::Result<i64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToInt64(varin: *const super::super::super::System::Com::VARIANT, pllret: *mut i64) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<i64>::zeroed();
        VariantToInt64(::core::mem::transmute(varin), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt64Array(var: *const super::super::super::System::Com::VARIANT, prgn: &mut [i64], pcelem: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToInt64Array(var: *const super::super::super::System::Com::VARIANT, prgn: *mut i64, crgn: u32, pcelem: *mut u32) -> ::windows_core::HRESULT;
        }
        VariantToInt64Array(::core::mem::transmute(var), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(prgn)), prgn.len() as _, ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt64ArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut i64, pcelem: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToInt64ArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut i64, pcelem: *mut u32) -> ::windows_core::HRESULT;
        }
        VariantToInt64ArrayAlloc(::core::mem::transmute(var), ::core::mem::transmute(pprgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt64WithDefault(varin: *const super::super::super::System::Com::VARIANT, lldefault: i64) -> i64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToInt64WithDefault(varin: *const super::super::super::System::Com::VARIANT, lldefault: i64) -> i64;
        }
        ::core::mem::transmute(VariantToInt64WithDefault(::core::mem::transmute(varin), ::core::mem::transmute(lldefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToPropVariant(pvar: *const super::super::super::System::Com::VARIANT) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToPropVariant(pvar: *const super::super::super::System::Com::VARIANT, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        VariantToPropVariant(::core::mem::transmute(pvar), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
#[inline]
pub unsafe fn VariantToStrRet(varin: *const super::super::super::System::Com::VARIANT) -> ::windows_core::Result<super::Common::STRRET> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToStrRet(varin: *const super::super::super::System::Com::VARIANT, pstrret: *mut super::Common::STRRET) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<super::Common::STRRET>::zeroed();
        VariantToStrRet(::core::mem::transmute(varin), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Common::STRRET>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToString(varin: *const super::super::super::System::Com::VARIANT, pszbuf: &mut [u16]) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToString(varin: *const super::super::super::System::Com::VARIANT, pszbuf: ::windows_core::PWSTR, cchbuf: u32) -> ::windows_core::HRESULT;
        }
        VariantToString(::core::mem::transmute(varin), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pszbuf)), pszbuf.len() as _).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToStringAlloc(varin: *const super::super::super::System::Com::VARIANT) -> ::windows_core::Result<::windows_core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToStringAlloc(varin: *const super::super::super::System::Com::VARIANT, ppszbuf: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        VariantToStringAlloc(::core::mem::transmute(varin), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToStringArray(var: *const super::super::super::System::Com::VARIANT, prgsz: &mut [::windows_core::PWSTR], pcelem: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToStringArray(var: *const super::super::super::System::Com::VARIANT, prgsz: *mut ::windows_core::PWSTR, crgsz: u32, pcelem: *mut u32) -> ::windows_core::HRESULT;
        }
        VariantToStringArray(::core::mem::transmute(var), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(prgsz)), prgsz.len() as _, ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToStringArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgsz: *mut *mut ::windows_core::PWSTR, pcelem: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToStringArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgsz: *mut *mut ::windows_core::PWSTR, pcelem: *mut u32) -> ::windows_core::HRESULT;
        }
        VariantToStringArrayAlloc(::core::mem::transmute(var), ::core::mem::transmute(pprgsz), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToStringWithDefault<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(varin: *const super::super::super::System::Com::VARIANT, pszdefault: Param1) -> ::windows_core::PWSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToStringWithDefault(varin: *const super::super::super::System::Com::VARIANT, pszdefault: ::windows_core::PCWSTR) -> ::windows_core::PWSTR;
        }
        ::core::mem::transmute(VariantToStringWithDefault(::core::mem::transmute(varin), pszdefault.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt16(varin: *const super::super::super::System::Com::VARIANT) -> ::windows_core::Result<u16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToUInt16(varin: *const super::super::super::System::Com::VARIANT, puiret: *mut u16) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        VariantToUInt16(::core::mem::transmute(varin), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt16Array(var: *const super::super::super::System::Com::VARIANT, prgn: &mut [u16], pcelem: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToUInt16Array(var: *const super::super::super::System::Com::VARIANT, prgn: *mut u16, crgn: u32, pcelem: *mut u32) -> ::windows_core::HRESULT;
        }
        VariantToUInt16Array(::core::mem::transmute(var), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(prgn)), prgn.len() as _, ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt16ArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut u16, pcelem: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToUInt16ArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut u16, pcelem: *mut u32) -> ::windows_core::HRESULT;
        }
        VariantToUInt16ArrayAlloc(::core::mem::transmute(var), ::core::mem::transmute(pprgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt16WithDefault(varin: *const super::super::super::System::Com::VARIANT, uidefault: u16) -> u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToUInt16WithDefault(varin: *const super::super::super::System::Com::VARIANT, uidefault: u16) -> u16;
        }
        ::core::mem::transmute(VariantToUInt16WithDefault(::core::mem::transmute(varin), ::core::mem::transmute(uidefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt32(varin: *const super::super::super::System::Com::VARIANT) -> ::windows_core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToUInt32(varin: *const super::super::super::System::Com::VARIANT, pulret: *mut u32) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        VariantToUInt32(::core::mem::transmute(varin), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt32Array(var: *const super::super::super::System::Com::VARIANT, prgn: &mut [u32], pcelem: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToUInt32Array(var: *const super::super::super::System::Com::VARIANT, prgn: *mut u32, crgn: u32, pcelem: *mut u32) -> ::windows_core::HRESULT;
        }
        VariantToUInt32Array(::core::mem::transmute(var), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(prgn)), prgn.len() as _, ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt32ArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut u32, pcelem: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToUInt32ArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut u32, pcelem: *mut u32) -> ::windows_core::HRESULT;
        }
        VariantToUInt32ArrayAlloc(::core::mem::transmute(var), ::core::mem::transmute(pprgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt32WithDefault(varin: *const super::super::super::System::Com::VARIANT, uldefault: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToUInt32WithDefault(varin: *const super::super::super::System::Com::VARIANT, uldefault: u32) -> u32;
        }
        ::core::mem::transmute(VariantToUInt32WithDefault(::core::mem::transmute(varin), ::core::mem::transmute(uldefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt64(varin: *const super::super::super::System::Com::VARIANT) -> ::windows_core::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToUInt64(varin: *const super::super::super::System::Com::VARIANT, pullret: *mut u64) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        VariantToUInt64(::core::mem::transmute(varin), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt64Array(var: *const super::super::super::System::Com::VARIANT, prgn: &mut [u64], pcelem: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToUInt64Array(var: *const super::super::super::System::Com::VARIANT, prgn: *mut u64, crgn: u32, pcelem: *mut u32) -> ::windows_core::HRESULT;
        }
        VariantToUInt64Array(::core::mem::transmute(var), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(prgn)), prgn.len() as _, ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt64ArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut u64, pcelem: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToUInt64ArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut u64, pcelem: *mut u32) -> ::windows_core::HRESULT;
        }
        VariantToUInt64ArrayAlloc(::core::mem::transmute(var), ::core::mem::transmute(pprgn), ::core::mem::transmute(pcelem)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt64WithDefault(varin: *const super::super::super::System::Com::VARIANT, ulldefault: u64) -> u64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantToUInt64WithDefault(varin: *const super::super::super::System::Com::VARIANT, ulldefault: u64) -> u64;
        }
        ::core::mem::transmute(VariantToUInt64WithDefault(::core::mem::transmute(varin), ::core::mem::transmute(ulldefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn WinRTPropertyValueToPropVariant<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(punkpropertyvalue: Param0) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinRTPropertyValueToPropVariant(punkpropertyvalue: *mut ::core::ffi::c_void, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        WinRTPropertyValueToPropVariant(punkpropertyvalue.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct _PERSIST_SPROPSTORE_FLAGS(pub i32);
pub const FPSPS_DEFAULT: _PERSIST_SPROPSTORE_FLAGS = _PERSIST_SPROPSTORE_FLAGS(0i32);
pub const FPSPS_READONLY: _PERSIST_SPROPSTORE_FLAGS = _PERSIST_SPROPSTORE_FLAGS(1i32);
pub const FPSPS_TREAT_NEW_VALUES_AS_DIRTY: _PERSIST_SPROPSTORE_FLAGS = _PERSIST_SPROPSTORE_FLAGS(2i32);
impl ::core::marker::Copy for _PERSIST_SPROPSTORE_FLAGS {}
impl ::core::clone::Clone for _PERSIST_SPROPSTORE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _PERSIST_SPROPSTORE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for _PERSIST_SPROPSTORE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for _PERSIST_SPROPSTORE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_PERSIST_SPROPSTORE_FLAGS").field(&self.0).finish()
    }
}
