#[repr(C)]
pub struct IChannelCredentials {
    pub base__: super::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub SetWindowsCredential: unsafe extern "system" fn(this: *mut *mut Self, domain: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, username: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, password: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, impersonationlevel: i32, allowntlm: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetWindowsCredential: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUserNameCredential: unsafe extern "system" fn(this: *mut *mut Self, username: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, password: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUserNameCredential: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub SetClientCertificateFromStore: unsafe extern "system" fn(this: *mut *mut Self, storelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, findyype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, findvalue: ::core::mem::ManuallyDrop<super::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    SetClientCertificateFromStore: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClientCertificateFromStoreByName: unsafe extern "system" fn(this: *mut *mut Self, subjectname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClientCertificateFromStoreByName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClientCertificateFromFile: unsafe extern "system" fn(this: *mut *mut Self, filename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, password: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, keystorageflags: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClientCertificateFromFile: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub SetDefaultServiceCertificateFromStore: unsafe extern "system" fn(this: *mut *mut Self, storelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, findtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, findvalue: ::core::mem::ManuallyDrop<super::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    SetDefaultServiceCertificateFromStore: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDefaultServiceCertificateFromStoreByName: unsafe extern "system" fn(this: *mut *mut Self, subjectname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDefaultServiceCertificateFromStoreByName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDefaultServiceCertificateFromFile: unsafe extern "system" fn(this: *mut *mut Self, filename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, password: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, keystorageflags: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDefaultServiceCertificateFromFile: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetServiceCertificateAuthentication: unsafe extern "system" fn(this: *mut *mut Self, storelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, revocationmode: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, certificatevalidationmode: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetServiceCertificateAuthentication: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIssuedToken: unsafe extern "system" fn(this: *mut *mut Self, localissueraddres: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, localissuerbindingtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, localissuerbinding: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIssuedToken: usize,
}
impl ::windows_sys::core::Interface for IChannelCredentials {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 404440204, data2: 49532, data3: 19223, data4: [172, 109, 6, 105, 155, 147, 25, 143] };
}
