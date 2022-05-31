#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWindowsMediaLibrarySharingDevice(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWindowsMediaLibrarySharingDevice {
    pub unsafe fn DeviceID(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).DeviceID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Authorization(&self) -> ::windows_core::Result<WindowsMediaLibrarySharingDeviceAuthorizationStatus> {
        let mut result__ = ::core::mem::MaybeUninit::<WindowsMediaLibrarySharingDeviceAuthorizationStatus>::zeroed();
        (::windows_core::Interface::vtable(self).Authorization)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WindowsMediaLibrarySharingDeviceAuthorizationStatus>(result__)
    }
    pub unsafe fn SetAuthorization(&self, authorization: WindowsMediaLibrarySharingDeviceAuthorizationStatus) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAuthorization)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(authorization)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows_core::Result<IWindowsMediaLibrarySharingDeviceProperties> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Properties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWindowsMediaLibrarySharingDeviceProperties>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsMediaLibrarySharingDevice> for ::windows_core::IUnknown {
    fn from(value: IWindowsMediaLibrarySharingDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsMediaLibrarySharingDevice> for ::windows_core::IUnknown {
    fn from(value: &IWindowsMediaLibrarySharingDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWindowsMediaLibrarySharingDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWindowsMediaLibrarySharingDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsMediaLibrarySharingDevice> for ::win32_system::Com::IDispatch {
    fn from(value: IWindowsMediaLibrarySharingDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsMediaLibrarySharingDevice> for ::win32_system::Com::IDispatch {
    fn from(value: &IWindowsMediaLibrarySharingDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IWindowsMediaLibrarySharingDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IWindowsMediaLibrarySharingDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWindowsMediaLibrarySharingDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWindowsMediaLibrarySharingDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWindowsMediaLibrarySharingDevice {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWindowsMediaLibrarySharingDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWindowsMediaLibrarySharingDevice").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWindowsMediaLibrarySharingDevice {
    type Vtable = IWindowsMediaLibrarySharingDevice_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3dccc293_4fd9_4191_a25b_8e57c5d27bd4);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsMediaLibrarySharingDevice_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub DeviceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Authorization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, authorization: *mut WindowsMediaLibrarySharingDeviceAuthorizationStatus) -> ::windows_core::HRESULT,
    pub SetAuthorization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, authorization: WindowsMediaLibrarySharingDeviceAuthorizationStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceproperties: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWindowsMediaLibrarySharingDeviceProperties(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWindowsMediaLibrarySharingDeviceProperties {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows_core::Result<IWindowsMediaLibrarySharingDeviceProperty> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWindowsMediaLibrarySharingDeviceProperty>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetProperty<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<IWindowsMediaLibrarySharingDeviceProperty> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), name.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWindowsMediaLibrarySharingDeviceProperty>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsMediaLibrarySharingDeviceProperties> for ::windows_core::IUnknown {
    fn from(value: IWindowsMediaLibrarySharingDeviceProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsMediaLibrarySharingDeviceProperties> for ::windows_core::IUnknown {
    fn from(value: &IWindowsMediaLibrarySharingDeviceProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWindowsMediaLibrarySharingDeviceProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWindowsMediaLibrarySharingDeviceProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsMediaLibrarySharingDeviceProperties> for ::win32_system::Com::IDispatch {
    fn from(value: IWindowsMediaLibrarySharingDeviceProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsMediaLibrarySharingDeviceProperties> for ::win32_system::Com::IDispatch {
    fn from(value: &IWindowsMediaLibrarySharingDeviceProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IWindowsMediaLibrarySharingDeviceProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IWindowsMediaLibrarySharingDeviceProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWindowsMediaLibrarySharingDeviceProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWindowsMediaLibrarySharingDeviceProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWindowsMediaLibrarySharingDeviceProperties {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWindowsMediaLibrarySharingDeviceProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWindowsMediaLibrarySharingDeviceProperties").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWindowsMediaLibrarySharingDeviceProperties {
    type Vtable = IWindowsMediaLibrarySharingDeviceProperties_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc4623214_6b06_40c5_a623_b2ff4c076bfd);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsMediaLibrarySharingDeviceProperties_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, property: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, property: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetProperty: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWindowsMediaLibrarySharingDeviceProperty(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWindowsMediaLibrarySharingDeviceProperty {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Value(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Value)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsMediaLibrarySharingDeviceProperty> for ::windows_core::IUnknown {
    fn from(value: IWindowsMediaLibrarySharingDeviceProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsMediaLibrarySharingDeviceProperty> for ::windows_core::IUnknown {
    fn from(value: &IWindowsMediaLibrarySharingDeviceProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWindowsMediaLibrarySharingDeviceProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWindowsMediaLibrarySharingDeviceProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsMediaLibrarySharingDeviceProperty> for ::win32_system::Com::IDispatch {
    fn from(value: IWindowsMediaLibrarySharingDeviceProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsMediaLibrarySharingDeviceProperty> for ::win32_system::Com::IDispatch {
    fn from(value: &IWindowsMediaLibrarySharingDeviceProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IWindowsMediaLibrarySharingDeviceProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IWindowsMediaLibrarySharingDeviceProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWindowsMediaLibrarySharingDeviceProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWindowsMediaLibrarySharingDeviceProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWindowsMediaLibrarySharingDeviceProperty {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWindowsMediaLibrarySharingDeviceProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWindowsMediaLibrarySharingDeviceProperty").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWindowsMediaLibrarySharingDeviceProperty {
    type Vtable = IWindowsMediaLibrarySharingDeviceProperty_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x81e26927_7a7d_40a7_81d4_bddc02960e3e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsMediaLibrarySharingDeviceProperty_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Value: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWindowsMediaLibrarySharingDevices(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWindowsMediaLibrarySharingDevices {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows_core::Result<IWindowsMediaLibrarySharingDevice> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWindowsMediaLibrarySharingDevice>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDevice<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, deviceid: Param0) -> ::windows_core::Result<IWindowsMediaLibrarySharingDevice> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetDevice)(::windows_core::Interface::as_raw(self), deviceid.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWindowsMediaLibrarySharingDevice>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsMediaLibrarySharingDevices> for ::windows_core::IUnknown {
    fn from(value: IWindowsMediaLibrarySharingDevices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsMediaLibrarySharingDevices> for ::windows_core::IUnknown {
    fn from(value: &IWindowsMediaLibrarySharingDevices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWindowsMediaLibrarySharingDevices {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWindowsMediaLibrarySharingDevices {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsMediaLibrarySharingDevices> for ::win32_system::Com::IDispatch {
    fn from(value: IWindowsMediaLibrarySharingDevices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsMediaLibrarySharingDevices> for ::win32_system::Com::IDispatch {
    fn from(value: &IWindowsMediaLibrarySharingDevices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IWindowsMediaLibrarySharingDevices {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IWindowsMediaLibrarySharingDevices {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWindowsMediaLibrarySharingDevices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWindowsMediaLibrarySharingDevices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWindowsMediaLibrarySharingDevices {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWindowsMediaLibrarySharingDevices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWindowsMediaLibrarySharingDevices").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWindowsMediaLibrarySharingDevices {
    type Vtable = IWindowsMediaLibrarySharingDevices_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1803f9d6_fe6d_4546_bf5b_992fe8ec12d1);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsMediaLibrarySharingDevices_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, device: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, device: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDevice: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWindowsMediaLibrarySharingServices(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWindowsMediaLibrarySharingServices {
    pub unsafe fn showShareMediaCPL<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, device: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).showShareMediaCPL)(::windows_core::Interface::as_raw(self), device.into_param().abi()).ok()
    }
    pub unsafe fn userHomeMediaSharingState(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).userHomeMediaSharingState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetuserHomeMediaSharingState(&self, sharingenabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetuserHomeMediaSharingState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(sharingenabled)).ok()
    }
    pub unsafe fn userHomeMediaSharingLibraryName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).userHomeMediaSharingLibraryName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetuserHomeMediaSharingLibraryName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, libraryname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetuserHomeMediaSharingLibraryName)(::windows_core::Interface::as_raw(self), libraryname.into_param().abi()).ok()
    }
    pub unsafe fn computerHomeMediaSharingAllowedState(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).computerHomeMediaSharingAllowedState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetcomputerHomeMediaSharingAllowedState(&self, sharingallowed: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetcomputerHomeMediaSharingAllowedState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(sharingallowed)).ok()
    }
    pub unsafe fn userInternetMediaSharingState(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).userInternetMediaSharingState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetuserInternetMediaSharingState(&self, sharingenabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetuserInternetMediaSharingState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(sharingenabled)).ok()
    }
    pub unsafe fn computerInternetMediaSharingAllowedState(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).computerInternetMediaSharingAllowedState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetcomputerInternetMediaSharingAllowedState(&self, sharingallowed: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetcomputerInternetMediaSharingAllowedState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(sharingallowed)).ok()
    }
    pub unsafe fn internetMediaSharingSecurityGroup(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).internetMediaSharingSecurityGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetinternetMediaSharingSecurityGroup<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, securitygroup: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetinternetMediaSharingSecurityGroup)(::windows_core::Interface::as_raw(self), securitygroup.into_param().abi()).ok()
    }
    pub unsafe fn allowSharingToAllDevices(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).allowSharingToAllDevices)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetallowSharingToAllDevices(&self, sharingenabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetallowSharingToAllDevices)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(sharingenabled)).ok()
    }
    pub unsafe fn setDefaultAuthorization<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, macaddresses: Param0, friendlyname: Param1, authorization: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).setDefaultAuthorization)(::windows_core::Interface::as_raw(self), macaddresses.into_param().abi(), friendlyname.into_param().abi(), ::core::mem::transmute(authorization)).ok()
    }
    pub unsafe fn setAuthorizationState<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, macaddress: Param0, authorizationstate: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).setAuthorizationState)(::windows_core::Interface::as_raw(self), macaddress.into_param().abi(), ::core::mem::transmute(authorizationstate)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getAllDevices(&self) -> ::windows_core::Result<IWindowsMediaLibrarySharingDevices> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).getAllDevices)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWindowsMediaLibrarySharingDevices>(result__)
    }
    pub unsafe fn customSettingsApplied(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).customSettingsApplied)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsMediaLibrarySharingServices> for ::windows_core::IUnknown {
    fn from(value: IWindowsMediaLibrarySharingServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsMediaLibrarySharingServices> for ::windows_core::IUnknown {
    fn from(value: &IWindowsMediaLibrarySharingServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWindowsMediaLibrarySharingServices {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWindowsMediaLibrarySharingServices {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsMediaLibrarySharingServices> for ::win32_system::Com::IDispatch {
    fn from(value: IWindowsMediaLibrarySharingServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsMediaLibrarySharingServices> for ::win32_system::Com::IDispatch {
    fn from(value: &IWindowsMediaLibrarySharingServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IWindowsMediaLibrarySharingServices {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IWindowsMediaLibrarySharingServices {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWindowsMediaLibrarySharingServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWindowsMediaLibrarySharingServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWindowsMediaLibrarySharingServices {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWindowsMediaLibrarySharingServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWindowsMediaLibrarySharingServices").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWindowsMediaLibrarySharingServices {
    type Vtable = IWindowsMediaLibrarySharingServices_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x01f5f85e_0a81_40da_a7c8_21ef3af8440c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsMediaLibrarySharingServices_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub showShareMediaCPL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, device: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub userHomeMediaSharingState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharingenabled: *mut i16) -> ::windows_core::HRESULT,
    pub SetuserHomeMediaSharingState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharingenabled: i16) -> ::windows_core::HRESULT,
    pub userHomeMediaSharingLibraryName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, libraryname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetuserHomeMediaSharingLibraryName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, libraryname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub computerHomeMediaSharingAllowedState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharingallowed: *mut i16) -> ::windows_core::HRESULT,
    pub SetcomputerHomeMediaSharingAllowedState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharingallowed: i16) -> ::windows_core::HRESULT,
    pub userInternetMediaSharingState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharingenabled: *mut i16) -> ::windows_core::HRESULT,
    pub SetuserInternetMediaSharingState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharingenabled: i16) -> ::windows_core::HRESULT,
    pub computerInternetMediaSharingAllowedState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharingallowed: *mut i16) -> ::windows_core::HRESULT,
    pub SetcomputerInternetMediaSharingAllowedState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharingallowed: i16) -> ::windows_core::HRESULT,
    pub internetMediaSharingSecurityGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, securitygroup: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetinternetMediaSharingSecurityGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, securitygroup: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub allowSharingToAllDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharingenabled: *mut i16) -> ::windows_core::HRESULT,
    pub SetallowSharingToAllDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharingenabled: i16) -> ::windows_core::HRESULT,
    pub setDefaultAuthorization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, macaddresses: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, friendlyname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, authorization: i16) -> ::windows_core::HRESULT,
    pub setAuthorizationState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, macaddress: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, authorizationstate: i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub getAllDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, devices: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getAllDevices: usize,
    pub customSettingsApplied: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customsettingsapplied: *mut i16) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WindowsMediaLibrarySharingDeviceAuthorizationStatus(pub i32);
pub const DEVICE_AUTHORIZATION_UNKNOWN: WindowsMediaLibrarySharingDeviceAuthorizationStatus = WindowsMediaLibrarySharingDeviceAuthorizationStatus(0i32);
pub const DEVICE_AUTHORIZATION_ALLOWED: WindowsMediaLibrarySharingDeviceAuthorizationStatus = WindowsMediaLibrarySharingDeviceAuthorizationStatus(1i32);
pub const DEVICE_AUTHORIZATION_DENIED: WindowsMediaLibrarySharingDeviceAuthorizationStatus = WindowsMediaLibrarySharingDeviceAuthorizationStatus(2i32);
impl ::core::marker::Copy for WindowsMediaLibrarySharingDeviceAuthorizationStatus {}
impl ::core::clone::Clone for WindowsMediaLibrarySharingDeviceAuthorizationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WindowsMediaLibrarySharingDeviceAuthorizationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WindowsMediaLibrarySharingDeviceAuthorizationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for WindowsMediaLibrarySharingDeviceAuthorizationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsMediaLibrarySharingDeviceAuthorizationStatus").field(&self.0).finish()
    }
}
pub const WindowsMediaLibrarySharingServices: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xad581b00_7b64_4e59_a38d_d2c5bf51ddb3);
