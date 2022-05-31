#[repr(transparent)]
pub struct Enterprise(::windows_core::IUnknown);
impl Enterprise {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn WorkplaceId(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).WorkplaceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn EnrollmentValidFrom(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).EnrollmentValidFrom)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn EnrollmentValidTo(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).EnrollmentValidTo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<EnterpriseStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EnterpriseStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EnterpriseStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for Enterprise {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Enterprise {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Enterprise {}
impl ::core::fmt::Debug for Enterprise {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Enterprise").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Enterprise {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Phone.Management.Deployment.Enterprise;{96592f8d-856c-4426-a947-b06307718078})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Enterprise {
    type Vtable = IEnterprise_Vtbl;
    const IID: ::windows_core::GUID = <IEnterprise as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Enterprise {
    const NAME: &'static str = "Windows.Phone.Management.Deployment.Enterprise";
}
impl ::core::convert::From<Enterprise> for ::windows_core::IUnknown {
    fn from(value: Enterprise) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Enterprise> for ::windows_core::IUnknown {
    fn from(value: &Enterprise) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Enterprise {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Enterprise {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Enterprise> for ::windows_core::IInspectable {
    fn from(value: Enterprise) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Enterprise> for ::windows_core::IInspectable {
    fn from(value: &Enterprise) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Enterprise {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Enterprise {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Enterprise {}
unsafe impl ::core::marker::Sync for Enterprise {}
pub struct EnterpriseEnrollmentManager;
impl EnterpriseEnrollmentManager {
    #[cfg(feature = "Foundation_Collections")]
    pub fn EnrolledEnterprises() -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<Enterprise>> {
        Self::IEnterpriseEnrollmentManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EnrolledEnterprises)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<Enterprise>>(result__)
        })
    }
    pub fn CurrentEnterprise() -> ::windows_core::Result<Enterprise> {
        Self::IEnterpriseEnrollmentManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentEnterprise)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Enterprise>(result__)
        })
    }
    pub fn ValidateEnterprisesAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        Self::IEnterpriseEnrollmentManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ValidateEnterprisesAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        })
    }
    pub fn RequestEnrollmentAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(enrollmenttoken: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<EnterpriseEnrollmentResult>> {
        Self::IEnterpriseEnrollmentManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestEnrollmentAsync)(::windows_core::Interface::as_raw(this), enrollmenttoken.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<EnterpriseEnrollmentResult>>(result__)
        })
    }
    pub fn RequestUnenrollmentAsync<'a, Param0: ::windows_core::IntoParam<'a, Enterprise>>(enterprise: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        Self::IEnterpriseEnrollmentManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestUnenrollmentAsync)(::windows_core::Interface::as_raw(this), enterprise.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        })
    }
    pub fn IEnterpriseEnrollmentManager<R, F: FnOnce(&IEnterpriseEnrollmentManager) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EnterpriseEnrollmentManager, IEnterpriseEnrollmentManager> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for EnterpriseEnrollmentManager {
    const NAME: &'static str = "Windows.Phone.Management.Deployment.EnterpriseEnrollmentManager";
}
#[repr(transparent)]
pub struct EnterpriseEnrollmentResult(::windows_core::IUnknown);
impl EnterpriseEnrollmentResult {
    pub fn EnrolledEnterprise(&self) -> ::windows_core::Result<Enterprise> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EnrolledEnterprise)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Enterprise>(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<EnterpriseEnrollmentStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EnterpriseEnrollmentStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EnterpriseEnrollmentStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for EnterpriseEnrollmentResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EnterpriseEnrollmentResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EnterpriseEnrollmentResult {}
impl ::core::fmt::Debug for EnterpriseEnrollmentResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnterpriseEnrollmentResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EnterpriseEnrollmentResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Phone.Management.Deployment.EnterpriseEnrollmentResult;{9ff71ce6-90db-4342-b326-1729aa91301c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EnterpriseEnrollmentResult {
    type Vtable = IEnterpriseEnrollmentResult_Vtbl;
    const IID: ::windows_core::GUID = <IEnterpriseEnrollmentResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EnterpriseEnrollmentResult {
    const NAME: &'static str = "Windows.Phone.Management.Deployment.EnterpriseEnrollmentResult";
}
impl ::core::convert::From<EnterpriseEnrollmentResult> for ::windows_core::IUnknown {
    fn from(value: EnterpriseEnrollmentResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EnterpriseEnrollmentResult> for ::windows_core::IUnknown {
    fn from(value: &EnterpriseEnrollmentResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EnterpriseEnrollmentResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EnterpriseEnrollmentResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EnterpriseEnrollmentResult> for ::windows_core::IInspectable {
    fn from(value: EnterpriseEnrollmentResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EnterpriseEnrollmentResult> for ::windows_core::IInspectable {
    fn from(value: &EnterpriseEnrollmentResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EnterpriseEnrollmentResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EnterpriseEnrollmentResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EnterpriseEnrollmentStatus(pub i32);
impl EnterpriseEnrollmentStatus {
    pub const Success: Self = Self(0i32);
    pub const CancelledByUser: Self = Self(1i32);
    pub const UnknownFailure: Self = Self(2i32);
}
impl ::core::marker::Copy for EnterpriseEnrollmentStatus {}
impl ::core::clone::Clone for EnterpriseEnrollmentStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EnterpriseEnrollmentStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EnterpriseEnrollmentStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for EnterpriseEnrollmentStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnterpriseEnrollmentStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EnterpriseEnrollmentStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Phone.Management.Deployment.EnterpriseEnrollmentStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EnterpriseStatus(pub i32);
impl EnterpriseStatus {
    pub const Enrolled: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
    pub const Revoked: Self = Self(2i32);
    pub const Expired: Self = Self(3i32);
}
impl ::core::marker::Copy for EnterpriseStatus {}
impl ::core::clone::Clone for EnterpriseStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EnterpriseStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EnterpriseStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for EnterpriseStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnterpriseStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EnterpriseStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Phone.Management.Deployment.EnterpriseStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEnterprise(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEnterprise {
    type Vtable = IEnterprise_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96592f8d_856c_4426_a947_b06307718078);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnterprise_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub WorkplaceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub EnrollmentValidFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub EnrollmentValidTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EnterpriseStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEnterpriseEnrollmentManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEnterpriseEnrollmentManager {
    type Vtable = IEnterpriseEnrollmentManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x20f9f390_2c69_41d8_88e6_e4b3884026cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnterpriseEnrollmentManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub EnrolledEnterprises: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EnrolledEnterprises: usize,
    pub CurrentEnterprise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ValidateEnterprisesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestEnrollmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enrollmenttoken: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestUnenrollmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enterprise: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEnterpriseEnrollmentResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEnterpriseEnrollmentResult {
    type Vtable = IEnterpriseEnrollmentResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9ff71ce6_90db_4342_b326_1729aa91301c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnterpriseEnrollmentResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub EnrolledEnterprise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EnterpriseEnrollmentStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInstallationManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInstallationManagerStatics {
    type Vtable = IInstallationManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x929aa738_8d49_42ac_80c9_b4ad793c43f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInstallationManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AddPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, sourcelocation: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AddPackagePreloadedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, sourcelocation: ::windows_core::RawPtr, instanceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, offerid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, license: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPendingPackageInstalls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPendingPackageInstalls: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesForCurrentPublisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesForCurrentPublisher: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackages: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInstallationManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInstallationManagerStatics2 {
    type Vtable = IInstallationManagerStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7c6c2cbd_fa4a_4c8e_ab97_d959452f19e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInstallationManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Management_Deployment")]
    pub RemovePackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, removaloptions: ::winrt_management::Deployment::RemovalOptions, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Management_Deployment"))]
    RemovePackageAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Management_Deployment"))]
    pub RegisterPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manifesturi: ::windows_core::RawPtr, dependencypackageuris: ::windows_core::RawPtr, deploymentoptions: ::winrt_management::Deployment::DeploymentOptions, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Management_Deployment")))]
    RegisterPackageAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByNamePublisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, packagepublisher: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByNamePublisher: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageInstallResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageInstallResult {
    type Vtable = IPackageInstallResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x33e8eed5_0f7e_4473_967c_7d6e1c0e7de1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageInstallResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Management_Deployment")]
    pub InstallState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_management::Deployment::PackageInstallState) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Management_Deployment"))]
    InstallState: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageInstallResult2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageInstallResult2 {
    type Vtable = IPackageInstallResult2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7149d909_3ff9_41ed_a717_2bc65ffc61d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageInstallResult2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ErrorText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
pub struct InstallationManager;
impl InstallationManager {
    pub fn AddPackageAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(title: Param0, sourcelocation: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>> {
        Self::IInstallationManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AddPackageAsync)(::windows_core::Interface::as_raw(this), title.into_param().abi(), sourcelocation.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>(result__)
        })
    }
    pub fn AddPackagePreloadedAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param4: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(title: Param0, sourcelocation: Param1, instanceid: Param2, offerid: Param3, license: Param4) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>> {
        Self::IInstallationManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AddPackagePreloadedAsync)(::windows_core::Interface::as_raw(this), title.into_param().abi(), sourcelocation.into_param().abi(), instanceid.into_param().abi(), offerid.into_param().abi(), license.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPendingPackageInstalls() -> ::windows_core::Result<::winrt_foundation::Collections::IIterable<::winrt_foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>> {
        Self::IInstallationManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPendingPackageInstalls)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterable<::winrt_foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>>(result__)
        })
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesForCurrentPublisher() -> ::windows_core::Result<::winrt_foundation::Collections::IIterable<::winrt_applicationmodel::Package>> {
        Self::IInstallationManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindPackagesForCurrentPublisher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterable<::winrt_applicationmodel::Package>>(result__)
        })
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackages() -> ::windows_core::Result<::winrt_foundation::Collections::IIterable<::winrt_applicationmodel::Package>> {
        Self::IInstallationManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindPackages)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterable<::winrt_applicationmodel::Package>>(result__)
        })
    }
    #[cfg(feature = "Management_Deployment")]
    pub fn RemovePackageAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(packagefullname: Param0, removaloptions: ::winrt_management::Deployment::RemovalOptions) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>> {
        Self::IInstallationManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemovePackageAsync)(::windows_core::Interface::as_raw(this), packagefullname.into_param().abi(), removaloptions, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Management_Deployment"))]
    pub fn RegisterPackageAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Uri>>>(manifesturi: Param0, dependencypackageuris: Param1, deploymentoptions: ::winrt_management::Deployment::DeploymentOptions) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>> {
        Self::IInstallationManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RegisterPackageAsync)(::windows_core::Interface::as_raw(this), manifesturi.into_param().abi(), dependencypackageuris.into_param().abi(), deploymentoptions, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<PackageInstallResult, u32>>(result__)
        })
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByNamePublisher<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(packagename: Param0, packagepublisher: Param1) -> ::windows_core::Result<::winrt_foundation::Collections::IIterable<::winrt_applicationmodel::Package>> {
        Self::IInstallationManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindPackagesByNamePublisher)(::windows_core::Interface::as_raw(this), packagename.into_param().abi(), packagepublisher.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterable<::winrt_applicationmodel::Package>>(result__)
        })
    }
    pub fn IInstallationManagerStatics<R, F: FnOnce(&IInstallationManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<InstallationManager, IInstallationManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IInstallationManagerStatics2<R, F: FnOnce(&IInstallationManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<InstallationManager, IInstallationManagerStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for InstallationManager {
    const NAME: &'static str = "Windows.Phone.Management.Deployment.InstallationManager";
}
#[repr(transparent)]
pub struct PackageInstallResult(::windows_core::IUnknown);
impl PackageInstallResult {
    pub fn ProductId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ProductId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Management_Deployment")]
    pub fn InstallState(&self) -> ::windows_core::Result<::winrt_management::Deployment::PackageInstallState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_management::Deployment::PackageInstallState>::zeroed();
            (::windows_core::Interface::vtable(this).InstallState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_management::Deployment::PackageInstallState>(result__)
        }
    }
    pub fn ErrorText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IPackageInstallResult2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ErrorText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for PackageInstallResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageInstallResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageInstallResult {}
impl ::core::fmt::Debug for PackageInstallResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageInstallResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PackageInstallResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Phone.Management.Deployment.PackageInstallResult;{33e8eed5-0f7e-4473-967c-7d6e1c0e7de1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PackageInstallResult {
    type Vtable = IPackageInstallResult_Vtbl;
    const IID: ::windows_core::GUID = <IPackageInstallResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PackageInstallResult {
    const NAME: &'static str = "Windows.Phone.Management.Deployment.PackageInstallResult";
}
impl ::core::convert::From<PackageInstallResult> for ::windows_core::IUnknown {
    fn from(value: PackageInstallResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageInstallResult> for ::windows_core::IUnknown {
    fn from(value: &PackageInstallResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PackageInstallResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PackageInstallResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PackageInstallResult> for ::windows_core::IInspectable {
    fn from(value: PackageInstallResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageInstallResult> for ::windows_core::IInspectable {
    fn from(value: &PackageInstallResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PackageInstallResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PackageInstallResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
