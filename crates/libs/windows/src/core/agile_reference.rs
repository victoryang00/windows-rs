use super::*;
use core::marker::PhantomData;
use windows_sys::Win32::System::WinRT::*;

/// A type representing an agile reference to a COM/WinRT object.
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq)]
pub struct AgileReference<T>(ComPtr<IAgileReference>, PhantomData<T>);

impl<T: Interface> AgileReference<T> {
    /// Creates an agile reference to the object.
    pub fn new(object: &T) -> Result<Self> {
        let mut reference = ComPtr::<IAgileReference>::null();
        let code = unsafe { RoGetAgileReference(AGILEREFERENCE_DEFAULT, (&T::IID).into(), object.as_raw() as _, reference.put()).into() };
        if reference.is_null() {
            Err(Error { code, info: None })
        } else {
            Ok(Self(reference, Default::default()))
        }
    }

    /// Retrieves a proxy to the target of the `AgileReference` object that may safely be used within any thread context in which get is called.
    pub fn resolve(&self) -> Result<T> {
        let mut value = Option::<T>::None;
        let code = unsafe { (self.0.get().Resolve)(self.0.this(), (&T::IID).into(), &mut value as *mut _ as _).into() };
        value.ok_or_else(|| Error { code, info: None })
    }
}

unsafe impl<T: Interface> Send for AgileReference<T> {}
unsafe impl<T: Interface> Sync for AgileReference<T> {}
