use super::*;

// TODO: move VARIANT to windows::core (depends on BSTR)

pub fn gen() -> TokenStream {
    quote! {
        #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
        impl VARIANT {
            pub fn is_empty(&self) -> bool {
                unsafe {
                    self.Anonymous.Anonymous.vt == 0
                }
            }
            pub fn clear(&mut self) {
                if !self.is_empty() {
                    unsafe {
                        let _ = super::Ole::VariantClear(self);
                    }
                }
            }
            pub fn vt(&self) -> super::Ole::VARENUM {
                unsafe {
                    super::Ole::VARENUM(self.Anonymous.Anonymous.vt as _)
                }
            }
            pub fn cast(&self, vt: super::Ole::VARENUM) -> ::windows::core::Result<Self> {
                let mut v = Self::default();
                unsafe {
                    super::Ole::VariantChangeType(&mut v, self, 0, vt.0 as _)?;
                }
                Ok(v)
            }
        }
        #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
        impl ::std::ops::Drop for VARIANT {
            fn drop(&mut self) {
                self.clear();
            }
        }
        #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
        impl ::std::convert::From<bool> for VARIANT {
            fn from(value: bool) -> Self {
                unsafe {
                    Self { 
                        Anonymous: VARIANT_0 { 
                            Anonymous: ::std::mem::transmute(VARIANT_0_0 {
                                vt: super::Ole::VT_BOOL.0 as _,
                                wReserved1: 0,
                                wReserved2: 0,
                                wReserved3: 0,
                                Anonymous: VARIANT_0_0_0 {
                                    boolVal: if value {
                                        -1
                                    } else {
                                        0
                                    }
                                }
                            })
                        }
                    }
                }
            }
        }
        impl ::std::convert::TryFrom<&VARIANT> for bool {
            type Error = ::windows::core::Error;        
            fn try_from(from: &VARIANT) -> ::windows::core::Result<Self> {
                let from = from.cast(super::Ole::VT_BOOL)?;
                Ok(unsafe { from.Anonymous.Anonymous.Anonymous.boolVal } != 0)
            }
        }
        impl core::convert::TryFrom<VARIANT> for bool {
            type Error = ::windows::core::Error;                
            fn try_from(from: VARIANT) -> ::windows::core::Result<Self> {
                Self::try_from(&from)
            }
        }
    }
}
