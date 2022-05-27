#[repr(transparent)]
pub struct Class(::windows_core::IUnknown);
impl Class {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Class, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Property(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Property)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetProperty(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetProperty)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for Class {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Class {}
impl ::core::fmt::Debug for Class {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Class").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Class {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(test_component.Class;{a3307453-713a-5c8d-80e4-d73a7bd5e500})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Class {
    type Vtable = IClass_Vtbl;
    const IID: ::windows_core::GUID = <IClass as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Class {
    const NAME: &'static str = "test_component.Class";
}
impl ::core::convert::From<Class> for ::windows_core::IUnknown {
    fn from(value: Class) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Class> for ::windows_core::IUnknown {
    fn from(value: &Class) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Class {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Class {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Class> for ::windows_core::IInspectable {
    fn from(value: Class) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Class> for ::windows_core::IInspectable {
    fn from(value: &Class) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Class {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Class {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Class {}
unsafe impl ::core::marker::Sync for Class {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClass(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IClass {
    type Vtable = IClass_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa3307453_713a_5c8d_80e4_d73a7bd5e500);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClass_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
}
pub trait IClass_Impl: Sized {
    fn Property(&self) -> ::windows_core::Result<i32>;
    fn SetProperty(&self, value: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IClass {
    const NAME: &'static str = "test_component.IClass";
}
impl IClass_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IClass_Impl, const OFFSET: isize>() -> IClass_Vtbl {
        unsafe extern "system" fn Property<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Property() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(value).into()
        }
        Self {
            base__: ::windows_core::IInspectableVtbl::new::<Identity, IClass, OFFSET>(),
            Property: Property::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IClass as ::windows_core::Interface>::IID
    }
}
