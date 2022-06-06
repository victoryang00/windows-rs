pub struct ComPtr<T>(*mut *mut ::windows_sys::core::IUnknown, core::marker::PhantomData<T>);

impl<T> ComPtr<T> {
    pub fn null() -> Self {
        Self(std::ptr::null_mut(), core::marker::PhantomData::<T>)
    }
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }
    pub fn get(&self) -> &T {
        unsafe { std::mem::transmute(*self.0) }
    }
    pub fn this(&self) -> *mut *mut T {
        unsafe { std::mem::transmute(self.0) }
    }
    pub fn put(&mut self) -> &mut *mut *mut T {
        self.release();
        unsafe { std::mem::transmute(&mut self.0)} 
    }
    pub fn release(&mut self) {
        if !self.0.is_null() {
            unsafe {
                ((*(*self.0)).Release)(self.0);
                self.0 = std::ptr::null_mut();
            }
        }
    }
}

impl<T> Clone for ComPtr<T> {
    fn clone(&self) -> Self {
        if !self.0.is_null() {
            unsafe {
                ((*(*self.0)).AddRef)(self.0);
            }
        }
        Self(self.0, core::marker::PhantomData::<T>)
    }
}

impl<T> Drop for ComPtr<T> {
    fn drop(&mut self) {
        self.release();
    }
}

impl<T> PartialEq for ComPtr<T> {
    fn eq(&self, other: &Self) -> bool {
        // Since COM objects may implement multiple interfaces, COM identity can only
        // be determined by querying for `IUnknown` explicitly and then comparing the
        // pointer values. This works since `QueryInterface` is required to return
        // the same pointer value for queries for `IUnknown`.
        // self.cast::<IUnknown>().unwrap().0 == other.cast::<IUnknown>().unwrap().0
        todo!()
    }
}

impl<T> Eq for ComPtr<T> {}
