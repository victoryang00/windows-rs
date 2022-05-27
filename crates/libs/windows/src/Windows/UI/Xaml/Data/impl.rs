#[cfg(feature = "Foundation_Collections")]
pub trait ICollectionView_Impl: Sized + super::super::super::Foundation::Collections::IIterable_Impl<::windows_core::IInspectable> + super::super::super::Foundation::Collections::IObservableVector_Impl<::windows_core::IInspectable> + super::super::super::Foundation::Collections::IVector_Impl<::windows_core::IInspectable> {
    fn CurrentItem(&self) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn CurrentPosition(&self) -> ::windows_core::Result<i32>;
    fn IsCurrentAfterLast(&self) -> ::windows_core::Result<bool>;
    fn IsCurrentBeforeFirst(&self) -> ::windows_core::Result<bool>;
    fn CollectionGroups(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IObservableVector<::windows_core::IInspectable>>;
    fn HasMoreItems(&self) -> ::windows_core::Result<bool>;
    fn CurrentChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<::windows_core::IInspectable>>) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCurrentChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn CurrentChanging(&self, handler: &::core::option::Option<CurrentChangingEventHandler>) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCurrentChanging(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn MoveCurrentTo(&self, item: &::core::option::Option<::windows_core::IInspectable>) -> ::windows_core::Result<bool>;
    fn MoveCurrentToPosition(&self, index: i32) -> ::windows_core::Result<bool>;
    fn MoveCurrentToFirst(&self) -> ::windows_core::Result<bool>;
    fn MoveCurrentToLast(&self) -> ::windows_core::Result<bool>;
    fn MoveCurrentToNext(&self) -> ::windows_core::Result<bool>;
    fn MoveCurrentToPrevious(&self) -> ::windows_core::Result<bool>;
    fn LoadMoreItemsAsync(&self, count: u32) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<LoadMoreItemsResult>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for ICollectionView {
    const NAME: &'static str = "Windows.UI.Xaml.Data.ICollectionView";
}
#[cfg(feature = "Foundation_Collections")]
impl ICollectionView_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICollectionView_Impl, const OFFSET: isize>() -> ICollectionView_Vtbl {
        unsafe extern "system" fn CurrentItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentItem() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentPosition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCurrentAfterLast<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsCurrentAfterLast() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCurrentBeforeFirst<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsCurrentBeforeFirst() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CollectionGroups<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CollectionGroups() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasMoreItems<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HasMoreItems() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentChanged(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCurrentChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveCurrentChanged(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn CurrentChanging<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentChanging(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCurrentChanging<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveCurrentChanging(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn MoveCurrentTo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveCurrentTo(::core::mem::transmute(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveCurrentToPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveCurrentToPosition(index) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveCurrentToFirst<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveCurrentToFirst() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveCurrentToLast<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveCurrentToLast() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveCurrentToNext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveCurrentToNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveCurrentToPrevious<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveCurrentToPrevious() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadMoreItemsAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LoadMoreItemsAsync(count) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectableVtbl::new::<Identity, ICollectionView, OFFSET>(),
            CurrentItem: CurrentItem::<Identity, Impl, OFFSET>,
            CurrentPosition: CurrentPosition::<Identity, Impl, OFFSET>,
            IsCurrentAfterLast: IsCurrentAfterLast::<Identity, Impl, OFFSET>,
            IsCurrentBeforeFirst: IsCurrentBeforeFirst::<Identity, Impl, OFFSET>,
            CollectionGroups: CollectionGroups::<Identity, Impl, OFFSET>,
            HasMoreItems: HasMoreItems::<Identity, Impl, OFFSET>,
            CurrentChanged: CurrentChanged::<Identity, Impl, OFFSET>,
            RemoveCurrentChanged: RemoveCurrentChanged::<Identity, Impl, OFFSET>,
            CurrentChanging: CurrentChanging::<Identity, Impl, OFFSET>,
            RemoveCurrentChanging: RemoveCurrentChanging::<Identity, Impl, OFFSET>,
            MoveCurrentTo: MoveCurrentTo::<Identity, Impl, OFFSET>,
            MoveCurrentToPosition: MoveCurrentToPosition::<Identity, Impl, OFFSET>,
            MoveCurrentToFirst: MoveCurrentToFirst::<Identity, Impl, OFFSET>,
            MoveCurrentToLast: MoveCurrentToLast::<Identity, Impl, OFFSET>,
            MoveCurrentToNext: MoveCurrentToNext::<Identity, Impl, OFFSET>,
            MoveCurrentToPrevious: MoveCurrentToPrevious::<Identity, Impl, OFFSET>,
            LoadMoreItemsAsync: LoadMoreItemsAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICollectionView as ::windows_core::Interface>::IID
    }
}
pub trait ICollectionViewFactory_Impl: Sized {
    fn CreateView(&self) -> ::windows_core::Result<ICollectionView>;
}
impl ::windows_core::RuntimeName for ICollectionViewFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Data.ICollectionViewFactory";
}
impl ICollectionViewFactory_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICollectionViewFactory_Impl, const OFFSET: isize>() -> ICollectionViewFactory_Vtbl {
        unsafe extern "system" fn CreateView<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICollectionViewFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateView() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IInspectableVtbl::new::<Identity, ICollectionViewFactory, OFFSET>(), CreateView: CreateView::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICollectionViewFactory as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait ICollectionViewGroup_Impl: Sized {
    fn Group(&self) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn GroupItems(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IObservableVector<::windows_core::IInspectable>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for ICollectionViewGroup {
    const NAME: &'static str = "Windows.UI.Xaml.Data.ICollectionViewGroup";
}
#[cfg(feature = "Foundation_Collections")]
impl ICollectionViewGroup_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICollectionViewGroup_Impl, const OFFSET: isize>() -> ICollectionViewGroup_Vtbl {
        unsafe extern "system" fn Group<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICollectionViewGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Group() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GroupItems<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICollectionViewGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GroupItems() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectableVtbl::new::<Identity, ICollectionViewGroup, OFFSET>(),
            Group: Group::<Identity, Impl, OFFSET>,
            GroupItems: GroupItems::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICollectionViewGroup as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "UI_Xaml_Interop")]
pub trait ICustomProperty_Impl: Sized {
    fn Type(&self) -> ::windows_core::Result<super::Interop::TypeName>;
    fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn GetValue(&self, target: &::core::option::Option<::windows_core::IInspectable>) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn SetValue(&self, target: &::core::option::Option<::windows_core::IInspectable>, value: &::core::option::Option<::windows_core::IInspectable>) -> ::windows_core::Result<()>;
    fn GetIndexedValue(&self, target: &::core::option::Option<::windows_core::IInspectable>, index: &::core::option::Option<::windows_core::IInspectable>) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn SetIndexedValue(&self, target: &::core::option::Option<::windows_core::IInspectable>, value: &::core::option::Option<::windows_core::IInspectable>, index: &::core::option::Option<::windows_core::IInspectable>) -> ::windows_core::Result<()>;
    fn CanWrite(&self) -> ::windows_core::Result<bool>;
    fn CanRead(&self) -> ::windows_core::Result<bool>;
}
#[cfg(feature = "UI_Xaml_Interop")]
impl ::windows_core::RuntimeName for ICustomProperty {
    const NAME: &'static str = "Windows.UI.Xaml.Data.ICustomProperty";
}
#[cfg(feature = "UI_Xaml_Interop")]
impl ICustomProperty_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICustomProperty_Impl, const OFFSET: isize>() -> ICustomProperty_Vtbl {
        unsafe extern "system" fn Type<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICustomProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICustomProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICustomProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetValue(::core::mem::transmute(&target)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICustomProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValue(::core::mem::transmute(&target), ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn GetIndexedValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICustomProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void, index: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIndexedValue(::core::mem::transmute(&target), ::core::mem::transmute(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIndexedValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICustomProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, index: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIndexedValue(::core::mem::transmute(&target), ::core::mem::transmute(&value), ::core::mem::transmute(&index)).into()
        }
        unsafe extern "system" fn CanWrite<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICustomProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanWrite() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanRead<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICustomProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanRead() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectableVtbl::new::<Identity, ICustomProperty, OFFSET>(),
            Type: Type::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            GetIndexedValue: GetIndexedValue::<Identity, Impl, OFFSET>,
            SetIndexedValue: SetIndexedValue::<Identity, Impl, OFFSET>,
            CanWrite: CanWrite::<Identity, Impl, OFFSET>,
            CanRead: CanRead::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICustomProperty as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "UI_Xaml_Interop")]
pub trait ICustomPropertyProvider_Impl: Sized {
    fn GetCustomProperty(&self, name: &::windows_core::HSTRING) -> ::windows_core::Result<ICustomProperty>;
    fn GetIndexedProperty(&self, name: &::windows_core::HSTRING, r#type: &super::Interop::TypeName) -> ::windows_core::Result<ICustomProperty>;
    fn GetStringRepresentation(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Type(&self) -> ::windows_core::Result<super::Interop::TypeName>;
}
#[cfg(feature = "UI_Xaml_Interop")]
impl ::windows_core::RuntimeName for ICustomPropertyProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Data.ICustomPropertyProvider";
}
#[cfg(feature = "UI_Xaml_Interop")]
impl ICustomPropertyProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICustomPropertyProvider_Impl, const OFFSET: isize>() -> ICustomPropertyProvider_Vtbl {
        unsafe extern "system" fn GetCustomProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICustomPropertyProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCustomProperty(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndexedProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICustomPropertyProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, r#type: ::core::mem::ManuallyDrop<super::Interop::TypeName>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIndexedProperty(::core::mem::transmute(&name), ::core::mem::transmute(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringRepresentation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICustomPropertyProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStringRepresentation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICustomPropertyProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectableVtbl::new::<Identity, ICustomPropertyProvider, OFFSET>(),
            GetCustomProperty: GetCustomProperty::<Identity, Impl, OFFSET>,
            GetIndexedProperty: GetIndexedProperty::<Identity, Impl, OFFSET>,
            GetStringRepresentation: GetStringRepresentation::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICustomPropertyProvider as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IItemsRangeInfo_Impl: Sized + super::super::super::Foundation::IClosable_Impl {
    fn RangesChanged(&self, visiblerange: &::core::option::Option<ItemIndexRange>, trackeditems: &::core::option::Option<super::super::super::Foundation::Collections::IVectorView<ItemIndexRange>>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for IItemsRangeInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Data.IItemsRangeInfo";
}
#[cfg(feature = "Foundation_Collections")]
impl IItemsRangeInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IItemsRangeInfo_Impl, const OFFSET: isize>() -> IItemsRangeInfo_Vtbl {
        unsafe extern "system" fn RangesChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IItemsRangeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visiblerange: ::windows_core::RawPtr, trackeditems: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RangesChanged(::core::mem::transmute(&visiblerange), ::core::mem::transmute(&trackeditems)).into()
        }
        Self { base__: ::windows_core::IInspectableVtbl::new::<Identity, IItemsRangeInfo, OFFSET>(), RangesChanged: RangesChanged::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IItemsRangeInfo as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait INotifyPropertyChanged_Impl: Sized {
    fn PropertyChanged(&self, handler: &::core::option::Option<PropertyChangedEventHandler>) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePropertyChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::RuntimeName for INotifyPropertyChanged {
    const NAME: &'static str = "Windows.UI.Xaml.Data.INotifyPropertyChanged";
}
#[cfg(feature = "Foundation")]
impl INotifyPropertyChanged_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INotifyPropertyChanged_Impl, const OFFSET: isize>() -> INotifyPropertyChanged_Vtbl {
        unsafe extern "system" fn PropertyChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INotifyPropertyChanged_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PropertyChanged(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePropertyChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INotifyPropertyChanged_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemovePropertyChanged(::core::mem::transmute(&token)).into()
        }
        Self {
            base__: ::windows_core::IInspectableVtbl::new::<Identity, INotifyPropertyChanged, OFFSET>(),
            PropertyChanged: PropertyChanged::<Identity, Impl, OFFSET>,
            RemovePropertyChanged: RemovePropertyChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INotifyPropertyChanged as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait ISelectionInfo_Impl: Sized {
    fn SelectRange(&self, itemindexrange: &::core::option::Option<ItemIndexRange>) -> ::windows_core::Result<()>;
    fn DeselectRange(&self, itemindexrange: &::core::option::Option<ItemIndexRange>) -> ::windows_core::Result<()>;
    fn IsSelected(&self, index: i32) -> ::windows_core::Result<bool>;
    fn GetSelectedRanges(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<ItemIndexRange>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for ISelectionInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Data.ISelectionInfo";
}
#[cfg(feature = "Foundation_Collections")]
impl ISelectionInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISelectionInfo_Impl, const OFFSET: isize>() -> ISelectionInfo_Vtbl {
        unsafe extern "system" fn SelectRange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISelectionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemindexrange: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SelectRange(::core::mem::transmute(&itemindexrange)).into()
        }
        unsafe extern "system" fn DeselectRange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISelectionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemindexrange: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeselectRange(::core::mem::transmute(&itemindexrange)).into()
        }
        unsafe extern "system" fn IsSelected<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISelectionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsSelected(index) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelectedRanges<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISelectionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSelectedRanges() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectableVtbl::new::<Identity, ISelectionInfo, OFFSET>(),
            SelectRange: SelectRange::<Identity, Impl, OFFSET>,
            DeselectRange: DeselectRange::<Identity, Impl, OFFSET>,
            IsSelected: IsSelected::<Identity, Impl, OFFSET>,
            GetSelectedRanges: GetSelectedRanges::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISelectionInfo as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait ISupportIncrementalLoading_Impl: Sized {
    fn LoadMoreItemsAsync(&self, count: u32) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<LoadMoreItemsResult>>;
    fn HasMoreItems(&self) -> ::windows_core::Result<bool>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::RuntimeName for ISupportIncrementalLoading {
    const NAME: &'static str = "Windows.UI.Xaml.Data.ISupportIncrementalLoading";
}
#[cfg(feature = "Foundation")]
impl ISupportIncrementalLoading_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISupportIncrementalLoading_Impl, const OFFSET: isize>() -> ISupportIncrementalLoading_Vtbl {
        unsafe extern "system" fn LoadMoreItemsAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISupportIncrementalLoading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LoadMoreItemsAsync(count) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasMoreItems<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISupportIncrementalLoading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HasMoreItems() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectableVtbl::new::<Identity, ISupportIncrementalLoading, OFFSET>(),
            LoadMoreItemsAsync: LoadMoreItemsAsync::<Identity, Impl, OFFSET>,
            HasMoreItems: HasMoreItems::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISupportIncrementalLoading as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "UI_Xaml_Interop")]
pub trait IValueConverter_Impl: Sized {
    fn Convert(&self, value: &::core::option::Option<::windows_core::IInspectable>, targettype: &super::Interop::TypeName, parameter: &::core::option::Option<::windows_core::IInspectable>, language: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn ConvertBack(&self, value: &::core::option::Option<::windows_core::IInspectable>, targettype: &super::Interop::TypeName, parameter: &::core::option::Option<::windows_core::IInspectable>, language: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::IInspectable>;
}
#[cfg(feature = "UI_Xaml_Interop")]
impl ::windows_core::RuntimeName for IValueConverter {
    const NAME: &'static str = "Windows.UI.Xaml.Data.IValueConverter";
}
#[cfg(feature = "UI_Xaml_Interop")]
impl IValueConverter_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IValueConverter_Impl, const OFFSET: isize>() -> IValueConverter_Vtbl {
        unsafe extern "system" fn Convert<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IValueConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, targettype: ::core::mem::ManuallyDrop<super::Interop::TypeName>, parameter: *mut ::core::ffi::c_void, language: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Convert(::core::mem::transmute(&value), ::core::mem::transmute(&targettype), ::core::mem::transmute(&parameter), ::core::mem::transmute(&language)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertBack<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IValueConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, targettype: ::core::mem::ManuallyDrop<super::Interop::TypeName>, parameter: *mut ::core::ffi::c_void, language: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ConvertBack(::core::mem::transmute(&value), ::core::mem::transmute(&targettype), ::core::mem::transmute(&parameter), ::core::mem::transmute(&language)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectableVtbl::new::<Identity, IValueConverter, OFFSET>(),
            Convert: Convert::<Identity, Impl, OFFSET>,
            ConvertBack: ConvertBack::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IValueConverter as ::windows_core::Interface>::IID
    }
}
