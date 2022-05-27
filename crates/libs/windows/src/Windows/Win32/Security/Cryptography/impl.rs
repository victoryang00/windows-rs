#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertSrvSetup_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CAErrorId(&self) -> ::windows_core::Result<i32>;
    fn CAErrorString(&self) -> ::windows_core::Result<super::super::Foundation::BSTR>;
    fn InitializeDefaults(&self, bserver: i16, bclient: i16) -> ::windows_core::Result<()>;
    fn GetCASetupProperty(&self, propertyid: CASetupProperty) -> ::windows_core::Result<super::super::System::Com::VARIANT>;
    fn SetCASetupProperty(&self, propertyid: CASetupProperty, ppropertyvalue: *const super::super::System::Com::VARIANT) -> ::windows_core::Result<()>;
    fn IsPropertyEditable(&self, propertyid: CASetupProperty) -> ::windows_core::Result<i16>;
    fn GetSupportedCATypes(&self) -> ::windows_core::Result<super::super::System::Com::VARIANT>;
    fn GetProviderNameList(&self) -> ::windows_core::Result<super::super::System::Com::VARIANT>;
    fn GetKeyLengthList(&self, bstrprovidername: &super::super::Foundation::BSTR) -> ::windows_core::Result<super::super::System::Com::VARIANT>;
    fn GetHashAlgorithmList(&self, bstrprovidername: &super::super::Foundation::BSTR) -> ::windows_core::Result<super::super::System::Com::VARIANT>;
    fn GetPrivateKeyContainerList(&self, bstrprovidername: &super::super::Foundation::BSTR) -> ::windows_core::Result<super::super::System::Com::VARIANT>;
    fn GetExistingCACertificates(&self) -> ::windows_core::Result<ICertSrvSetupKeyInformationCollection>;
    fn CAImportPFX(&self, bstrfilename: &super::super::Foundation::BSTR, bstrpasswd: &super::super::Foundation::BSTR, boverwriteexistingkey: i16) -> ::windows_core::Result<ICertSrvSetupKeyInformation>;
    fn SetCADistinguishedName(&self, bstrcadn: &super::super::Foundation::BSTR, bignoreunicode: i16, boverwriteexistingkey: i16, boverwriteexistingcainds: i16) -> ::windows_core::Result<()>;
    fn SetDatabaseInformation(&self, bstrdbdirectory: &super::super::Foundation::BSTR, bstrlogdirectory: &super::super::Foundation::BSTR, bstrsharedfolder: &super::super::Foundation::BSTR, bforceoverwrite: i16) -> ::windows_core::Result<()>;
    fn SetParentCAInformation(&self, bstrcaconfiguration: &super::super::Foundation::BSTR) -> ::windows_core::Result<()>;
    fn SetWebCAInformation(&self, bstrcaconfiguration: &super::super::Foundation::BSTR) -> ::windows_core::Result<()>;
    fn Install(&self) -> ::windows_core::Result<()>;
    fn PreUnInstall(&self, bclientonly: i16) -> ::windows_core::Result<()>;
    fn PostUnInstall(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows_core::RuntimeName for ICertSrvSetup {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertSrvSetup_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>() -> ICertSrvSetup_Vtbl {
        unsafe extern "system" fn CAErrorId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CAErrorId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CAErrorString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CAErrorString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeDefaults<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bserver: i16, bclient: i16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitializeDefaults(::core::mem::transmute_copy(&bserver), ::core::mem::transmute_copy(&bclient)).into()
        }
        unsafe extern "system" fn GetCASetupProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: CASetupProperty, ppropertyvalue: *mut super::super::System::Com::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCASetupProperty(::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppropertyvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCASetupProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: CASetupProperty, ppropertyvalue: *const super::super::System::Com::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCASetupProperty(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&ppropertyvalue)).into()
        }
        unsafe extern "system" fn IsPropertyEditable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: CASetupProperty, pbeditable: *mut i16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsPropertyEditable(::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbeditable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedCATypes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcatypes: *mut super::super::System::Com::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSupportedCATypes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcatypes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderNameList<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::System::Com::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProviderNameList() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeyLengthList<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprovidername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pval: *mut super::super::System::Com::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetKeyLengthList(::core::mem::transmute(&bstrprovidername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHashAlgorithmList<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprovidername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pval: *mut super::super::System::Com::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHashAlgorithmList(::core::mem::transmute(&bstrprovidername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrivateKeyContainerList<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprovidername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pval: *mut super::super::System::Com::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPrivateKeyContainerList(::core::mem::transmute(&bstrprovidername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExistingCACertificates<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetExistingCACertificates() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CAImportPFX<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpasswd: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, boverwriteexistingkey: i16, ppval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CAImportPFX(::core::mem::transmute(&bstrfilename), ::core::mem::transmute(&bstrpasswd), ::core::mem::transmute_copy(&boverwriteexistingkey)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCADistinguishedName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcadn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bignoreunicode: i16, boverwriteexistingkey: i16, boverwriteexistingcainds: i16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCADistinguishedName(::core::mem::transmute(&bstrcadn), ::core::mem::transmute_copy(&bignoreunicode), ::core::mem::transmute_copy(&boverwriteexistingkey), ::core::mem::transmute_copy(&boverwriteexistingcainds)).into()
        }
        unsafe extern "system" fn SetDatabaseInformation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdbdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrlogdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsharedfolder: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bforceoverwrite: i16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDatabaseInformation(::core::mem::transmute(&bstrdbdirectory), ::core::mem::transmute(&bstrlogdirectory), ::core::mem::transmute(&bstrsharedfolder), ::core::mem::transmute_copy(&bforceoverwrite)).into()
        }
        unsafe extern "system" fn SetParentCAInformation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcaconfiguration: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetParentCAInformation(::core::mem::transmute(&bstrcaconfiguration)).into()
        }
        unsafe extern "system" fn SetWebCAInformation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcaconfiguration: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWebCAInformation(::core::mem::transmute(&bstrcaconfiguration)).into()
        }
        unsafe extern "system" fn Install<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Install().into()
        }
        unsafe extern "system" fn PreUnInstall<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bclientonly: i16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PreUnInstall(::core::mem::transmute_copy(&bclientonly)).into()
        }
        unsafe extern "system" fn PostUnInstall<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PostUnInstall().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CAErrorId: CAErrorId::<Identity, Impl, OFFSET>,
            CAErrorString: CAErrorString::<Identity, Impl, OFFSET>,
            InitializeDefaults: InitializeDefaults::<Identity, Impl, OFFSET>,
            GetCASetupProperty: GetCASetupProperty::<Identity, Impl, OFFSET>,
            SetCASetupProperty: SetCASetupProperty::<Identity, Impl, OFFSET>,
            IsPropertyEditable: IsPropertyEditable::<Identity, Impl, OFFSET>,
            GetSupportedCATypes: GetSupportedCATypes::<Identity, Impl, OFFSET>,
            GetProviderNameList: GetProviderNameList::<Identity, Impl, OFFSET>,
            GetKeyLengthList: GetKeyLengthList::<Identity, Impl, OFFSET>,
            GetHashAlgorithmList: GetHashAlgorithmList::<Identity, Impl, OFFSET>,
            GetPrivateKeyContainerList: GetPrivateKeyContainerList::<Identity, Impl, OFFSET>,
            GetExistingCACertificates: GetExistingCACertificates::<Identity, Impl, OFFSET>,
            CAImportPFX: CAImportPFX::<Identity, Impl, OFFSET>,
            SetCADistinguishedName: SetCADistinguishedName::<Identity, Impl, OFFSET>,
            SetDatabaseInformation: SetDatabaseInformation::<Identity, Impl, OFFSET>,
            SetParentCAInformation: SetParentCAInformation::<Identity, Impl, OFFSET>,
            SetWebCAInformation: SetWebCAInformation::<Identity, Impl, OFFSET>,
            Install: Install::<Identity, Impl, OFFSET>,
            PreUnInstall: PreUnInstall::<Identity, Impl, OFFSET>,
            PostUnInstall: PostUnInstall::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertSrvSetup as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertSrvSetupKeyInformation_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ProviderName(&self) -> ::windows_core::Result<super::super::Foundation::BSTR>;
    fn SetProviderName(&self, bstrval: &super::super::Foundation::BSTR) -> ::windows_core::Result<()>;
    fn Length(&self) -> ::windows_core::Result<i32>;
    fn SetLength(&self, lval: i32) -> ::windows_core::Result<()>;
    fn Existing(&self) -> ::windows_core::Result<i16>;
    fn SetExisting(&self, bval: i16) -> ::windows_core::Result<()>;
    fn ContainerName(&self) -> ::windows_core::Result<super::super::Foundation::BSTR>;
    fn SetContainerName(&self, bstrval: &super::super::Foundation::BSTR) -> ::windows_core::Result<()>;
    fn HashAlgorithm(&self) -> ::windows_core::Result<super::super::Foundation::BSTR>;
    fn SetHashAlgorithm(&self, bstrval: &super::super::Foundation::BSTR) -> ::windows_core::Result<()>;
    fn ExistingCACertificate(&self) -> ::windows_core::Result<super::super::System::Com::VARIANT>;
    fn SetExistingCACertificate(&self, varval: &super::super::System::Com::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows_core::RuntimeName for ICertSrvSetupKeyInformation {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertSrvSetupKeyInformation_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>() -> ICertSrvSetupKeyInformation_Vtbl {
        unsafe extern "system" fn ProviderName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProviderName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProviderName(::core::mem::transmute(&bstrval)).into()
        }
        unsafe extern "system" fn Length<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Length() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLength<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lval: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLength(::core::mem::transmute_copy(&lval)).into()
        }
        unsafe extern "system" fn Existing<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Existing() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExisting<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bval: i16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetExisting(::core::mem::transmute_copy(&bval)).into()
        }
        unsafe extern "system" fn ContainerName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ContainerName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContainerName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetContainerName(::core::mem::transmute(&bstrval)).into()
        }
        unsafe extern "system" fn HashAlgorithm<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HashAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHashAlgorithm(::core::mem::transmute(&bstrval)).into()
        }
        unsafe extern "system" fn ExistingCACertificate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::System::Com::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExistingCACertificate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExistingCACertificate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetExistingCACertificate(::core::mem::transmute(&varval)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ProviderName: ProviderName::<Identity, Impl, OFFSET>,
            SetProviderName: SetProviderName::<Identity, Impl, OFFSET>,
            Length: Length::<Identity, Impl, OFFSET>,
            SetLength: SetLength::<Identity, Impl, OFFSET>,
            Existing: Existing::<Identity, Impl, OFFSET>,
            SetExisting: SetExisting::<Identity, Impl, OFFSET>,
            ContainerName: ContainerName::<Identity, Impl, OFFSET>,
            SetContainerName: SetContainerName::<Identity, Impl, OFFSET>,
            HashAlgorithm: HashAlgorithm::<Identity, Impl, OFFSET>,
            SetHashAlgorithm: SetHashAlgorithm::<Identity, Impl, OFFSET>,
            ExistingCACertificate: ExistingCACertificate::<Identity, Impl, OFFSET>,
            SetExistingCACertificate: SetExistingCACertificate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertSrvSetupKeyInformation as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertSrvSetupKeyInformationCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn get_Item(&self, index: i32) -> ::windows_core::Result<super::super::System::Com::VARIANT>;
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn Add(&self, pikeyinformation: &::core::option::Option<ICertSrvSetupKeyInformation>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows_core::RuntimeName for ICertSrvSetupKeyInformationCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertSrvSetupKeyInformationCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetupKeyInformationCollection_Impl, const OFFSET: isize>() -> ICertSrvSetupKeyInformationCollection_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetupKeyInformationCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetupKeyInformationCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut super::super::System::Com::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetupKeyInformationCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetupKeyInformationCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pikeyinformation: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Add(::core::mem::transmute(&pikeyinformation)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertSrvSetupKeyInformationCollection as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertificateEnrollmentPolicyServerSetup_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ErrorString(&self) -> ::windows_core::Result<super::super::Foundation::BSTR>;
    fn InitializeInstallDefaults(&self) -> ::windows_core::Result<()>;
    fn GetProperty(&self, propertyid: CEPSetupProperty) -> ::windows_core::Result<super::super::System::Com::VARIANT>;
    fn SetProperty(&self, propertyid: CEPSetupProperty, ppropertyvalue: *const super::super::System::Com::VARIANT) -> ::windows_core::Result<()>;
    fn Install(&self) -> ::windows_core::Result<()>;
    fn UnInstall(&self, pauthkeybasedrenewal: *const super::super::System::Com::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows_core::RuntimeName for ICertificateEnrollmentPolicyServerSetup {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertificateEnrollmentPolicyServerSetup_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertificateEnrollmentPolicyServerSetup_Impl, const OFFSET: isize>() -> ICertificateEnrollmentPolicyServerSetup_Vtbl {
        unsafe extern "system" fn ErrorString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertificateEnrollmentPolicyServerSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ErrorString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeInstallDefaults<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertificateEnrollmentPolicyServerSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitializeInstallDefaults().into()
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertificateEnrollmentPolicyServerSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: CEPSetupProperty, ppropertyvalue: *mut super::super::System::Com::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperty(::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppropertyvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertificateEnrollmentPolicyServerSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: CEPSetupProperty, ppropertyvalue: *const super::super::System::Com::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&ppropertyvalue)).into()
        }
        unsafe extern "system" fn Install<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertificateEnrollmentPolicyServerSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Install().into()
        }
        unsafe extern "system" fn UnInstall<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertificateEnrollmentPolicyServerSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pauthkeybasedrenewal: *const super::super::System::Com::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnInstall(::core::mem::transmute_copy(&pauthkeybasedrenewal)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ErrorString: ErrorString::<Identity, Impl, OFFSET>,
            InitializeInstallDefaults: InitializeInstallDefaults::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            Install: Install::<Identity, Impl, OFFSET>,
            UnInstall: UnInstall::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertificateEnrollmentPolicyServerSetup as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertificateEnrollmentServerSetup_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ErrorString(&self) -> ::windows_core::Result<super::super::Foundation::BSTR>;
    fn InitializeInstallDefaults(&self) -> ::windows_core::Result<()>;
    fn GetProperty(&self, propertyid: CESSetupProperty) -> ::windows_core::Result<super::super::System::Com::VARIANT>;
    fn SetProperty(&self, propertyid: CESSetupProperty, ppropertyvalue: *const super::super::System::Com::VARIANT) -> ::windows_core::Result<()>;
    fn SetApplicationPoolCredentials(&self, bstrusername: &super::super::Foundation::BSTR, bstrpassword: &super::super::Foundation::BSTR) -> ::windows_core::Result<()>;
    fn Install(&self) -> ::windows_core::Result<()>;
    fn UnInstall(&self, pcaconfig: *const super::super::System::Com::VARIANT, pauthentication: *const super::super::System::Com::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows_core::RuntimeName for ICertificateEnrollmentServerSetup {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertificateEnrollmentServerSetup_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertificateEnrollmentServerSetup_Impl, const OFFSET: isize>() -> ICertificateEnrollmentServerSetup_Vtbl {
        unsafe extern "system" fn ErrorString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertificateEnrollmentServerSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ErrorString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeInstallDefaults<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertificateEnrollmentServerSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitializeInstallDefaults().into()
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertificateEnrollmentServerSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: CESSetupProperty, ppropertyvalue: *mut super::super::System::Com::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperty(::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppropertyvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertificateEnrollmentServerSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: CESSetupProperty, ppropertyvalue: *const super::super::System::Com::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&ppropertyvalue)).into()
        }
        unsafe extern "system" fn SetApplicationPoolCredentials<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertificateEnrollmentServerSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetApplicationPoolCredentials(::core::mem::transmute(&bstrusername), ::core::mem::transmute(&bstrpassword)).into()
        }
        unsafe extern "system" fn Install<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertificateEnrollmentServerSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Install().into()
        }
        unsafe extern "system" fn UnInstall<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertificateEnrollmentServerSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcaconfig: *const super::super::System::Com::VARIANT, pauthentication: *const super::super::System::Com::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnInstall(::core::mem::transmute_copy(&pcaconfig), ::core::mem::transmute_copy(&pauthentication)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ErrorString: ErrorString::<Identity, Impl, OFFSET>,
            InitializeInstallDefaults: InitializeInstallDefaults::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            SetApplicationPoolCredentials: SetApplicationPoolCredentials::<Identity, Impl, OFFSET>,
            Install: Install::<Identity, Impl, OFFSET>,
            UnInstall: UnInstall::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertificateEnrollmentServerSetup as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSCEPSetup_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn MSCEPErrorId(&self) -> ::windows_core::Result<i32>;
    fn MSCEPErrorString(&self) -> ::windows_core::Result<super::super::Foundation::BSTR>;
    fn InitializeDefaults(&self) -> ::windows_core::Result<()>;
    fn GetMSCEPSetupProperty(&self, propertyid: MSCEPSetupProperty) -> ::windows_core::Result<super::super::System::Com::VARIANT>;
    fn SetMSCEPSetupProperty(&self, propertyid: MSCEPSetupProperty, ppropertyvalue: *const super::super::System::Com::VARIANT) -> ::windows_core::Result<()>;
    fn SetAccountInformation(&self, bstrusername: &super::super::Foundation::BSTR, bstrpassword: &super::super::Foundation::BSTR) -> ::windows_core::Result<()>;
    fn IsMSCEPStoreEmpty(&self) -> ::windows_core::Result<i16>;
    fn GetProviderNameList(&self, bexchange: i16) -> ::windows_core::Result<super::super::System::Com::VARIANT>;
    fn GetKeyLengthList(&self, bexchange: i16, bstrprovidername: &super::super::Foundation::BSTR) -> ::windows_core::Result<super::super::System::Com::VARIANT>;
    fn Install(&self) -> ::windows_core::Result<()>;
    fn PreUnInstall(&self) -> ::windows_core::Result<()>;
    fn PostUnInstall(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows_core::RuntimeName for IMSCEPSetup {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSCEPSetup_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: isize>() -> IMSCEPSetup_Vtbl {
        unsafe extern "system" fn MSCEPErrorId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MSCEPErrorId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MSCEPErrorString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MSCEPErrorString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeDefaults<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitializeDefaults().into()
        }
        unsafe extern "system" fn GetMSCEPSetupProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: MSCEPSetupProperty, pval: *mut super::super::System::Com::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMSCEPSetupProperty(::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMSCEPSetupProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: MSCEPSetupProperty, ppropertyvalue: *const super::super::System::Com::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMSCEPSetupProperty(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&ppropertyvalue)).into()
        }
        unsafe extern "system" fn SetAccountInformation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAccountInformation(::core::mem::transmute(&bstrusername), ::core::mem::transmute(&bstrpassword)).into()
        }
        unsafe extern "system" fn IsMSCEPStoreEmpty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbempty: *mut i16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsMSCEPStoreEmpty() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbempty, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderNameList<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bexchange: i16, pval: *mut super::super::System::Com::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProviderNameList(::core::mem::transmute_copy(&bexchange)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeyLengthList<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bexchange: i16, bstrprovidername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pval: *mut super::super::System::Com::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetKeyLengthList(::core::mem::transmute_copy(&bexchange), ::core::mem::transmute(&bstrprovidername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Install<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Install().into()
        }
        unsafe extern "system" fn PreUnInstall<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PreUnInstall().into()
        }
        unsafe extern "system" fn PostUnInstall<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PostUnInstall().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            MSCEPErrorId: MSCEPErrorId::<Identity, Impl, OFFSET>,
            MSCEPErrorString: MSCEPErrorString::<Identity, Impl, OFFSET>,
            InitializeDefaults: InitializeDefaults::<Identity, Impl, OFFSET>,
            GetMSCEPSetupProperty: GetMSCEPSetupProperty::<Identity, Impl, OFFSET>,
            SetMSCEPSetupProperty: SetMSCEPSetupProperty::<Identity, Impl, OFFSET>,
            SetAccountInformation: SetAccountInformation::<Identity, Impl, OFFSET>,
            IsMSCEPStoreEmpty: IsMSCEPStoreEmpty::<Identity, Impl, OFFSET>,
            GetProviderNameList: GetProviderNameList::<Identity, Impl, OFFSET>,
            GetKeyLengthList: GetKeyLengthList::<Identity, Impl, OFFSET>,
            Install: Install::<Identity, Impl, OFFSET>,
            PreUnInstall: PreUnInstall::<Identity, Impl, OFFSET>,
            PostUnInstall: PostUnInstall::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSCEPSetup as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
