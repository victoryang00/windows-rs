#[repr(transparent)]
pub struct BufferProtectUnprotectResult(::windows_core::IUnknown);
impl BufferProtectUnprotectResult {
    #[cfg(feature = "winrt-storage")]
    pub fn Buffer(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Buffer)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    pub fn ProtectionInfo(&self) -> ::windows_core::Result<DataProtectionInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DataProtectionInfo>(result__)
        }
    }
}
impl ::core::clone::Clone for BufferProtectUnprotectResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BufferProtectUnprotectResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BufferProtectUnprotectResult {}
impl ::core::fmt::Debug for BufferProtectUnprotectResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BufferProtectUnprotectResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BufferProtectUnprotectResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.BufferProtectUnprotectResult;{47995edc-6cec-4e3a-b251-9e7485d79e7a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BufferProtectUnprotectResult {
    type Vtable = IBufferProtectUnprotectResult_Vtbl;
    const IID: ::windows_core::GUID = <IBufferProtectUnprotectResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BufferProtectUnprotectResult {
    const NAME: &'static str = "Windows.Security.EnterpriseData.BufferProtectUnprotectResult";
}
impl ::core::convert::From<BufferProtectUnprotectResult> for ::windows_core::IUnknown {
    fn from(value: BufferProtectUnprotectResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BufferProtectUnprotectResult> for ::windows_core::IUnknown {
    fn from(value: &BufferProtectUnprotectResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BufferProtectUnprotectResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BufferProtectUnprotectResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BufferProtectUnprotectResult> for ::windows_core::IInspectable {
    fn from(value: BufferProtectUnprotectResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BufferProtectUnprotectResult> for ::windows_core::IInspectable {
    fn from(value: &BufferProtectUnprotectResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BufferProtectUnprotectResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BufferProtectUnprotectResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BufferProtectUnprotectResult {}
unsafe impl ::core::marker::Sync for BufferProtectUnprotectResult {}
#[repr(transparent)]
pub struct DataProtectionInfo(::windows_core::IUnknown);
impl DataProtectionInfo {
    pub fn Status(&self) -> ::windows_core::Result<DataProtectionStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DataProtectionStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DataProtectionStatus>(result__)
        }
    }
    pub fn Identity(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Identity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for DataProtectionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DataProtectionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DataProtectionInfo {}
impl ::core::fmt::Debug for DataProtectionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataProtectionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DataProtectionInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.DataProtectionInfo;{8420b0c1-5e31-4405-9540-3f943af0cb26})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DataProtectionInfo {
    type Vtable = IDataProtectionInfo_Vtbl;
    const IID: ::windows_core::GUID = <IDataProtectionInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DataProtectionInfo {
    const NAME: &'static str = "Windows.Security.EnterpriseData.DataProtectionInfo";
}
impl ::core::convert::From<DataProtectionInfo> for ::windows_core::IUnknown {
    fn from(value: DataProtectionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DataProtectionInfo> for ::windows_core::IUnknown {
    fn from(value: &DataProtectionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DataProtectionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DataProtectionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DataProtectionInfo> for ::windows_core::IInspectable {
    fn from(value: DataProtectionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DataProtectionInfo> for ::windows_core::IInspectable {
    fn from(value: &DataProtectionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DataProtectionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DataProtectionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DataProtectionInfo {}
unsafe impl ::core::marker::Sync for DataProtectionInfo {}
pub struct DataProtectionManager;
impl DataProtectionManager {
    #[cfg(feature = "winrt-storage")]
    pub fn ProtectAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(data: Param0, identity: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<BufferProtectUnprotectResult>> {
        Self::IDataProtectionManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProtectAsync)(::windows_core::Interface::as_raw(this), data.into_param().abi(), identity.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<BufferProtectUnprotectResult>>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn UnprotectAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(data: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<BufferProtectUnprotectResult>> {
        Self::IDataProtectionManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UnprotectAsync)(::windows_core::Interface::as_raw(this), data.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<BufferProtectUnprotectResult>>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn ProtectStreamAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IInputStream>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IOutputStream>>(unprotectedstream: Param0, identity: Param1, protectedstream: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<DataProtectionInfo>> {
        Self::IDataProtectionManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProtectStreamAsync)(::windows_core::Interface::as_raw(this), unprotectedstream.into_param().abi(), identity.into_param().abi(), protectedstream.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<DataProtectionInfo>>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn UnprotectStreamAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IInputStream>, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IOutputStream>>(protectedstream: Param0, unprotectedstream: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<DataProtectionInfo>> {
        Self::IDataProtectionManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UnprotectStreamAsync)(::windows_core::Interface::as_raw(this), protectedstream.into_param().abi(), unprotectedstream.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<DataProtectionInfo>>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn GetProtectionInfoAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(protecteddata: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<DataProtectionInfo>> {
        Self::IDataProtectionManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetProtectionInfoAsync)(::windows_core::Interface::as_raw(this), protecteddata.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<DataProtectionInfo>>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn GetStreamProtectionInfoAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IInputStream>>(protectedstream: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<DataProtectionInfo>> {
        Self::IDataProtectionManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetStreamProtectionInfoAsync)(::windows_core::Interface::as_raw(this), protectedstream.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<DataProtectionInfo>>(result__)
        })
    }
    pub fn IDataProtectionManagerStatics<R, F: FnOnce(&IDataProtectionManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DataProtectionManager, IDataProtectionManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for DataProtectionManager {
    const NAME: &'static str = "Windows.Security.EnterpriseData.DataProtectionManager";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DataProtectionStatus(pub i32);
impl DataProtectionStatus {
    pub const ProtectedToOtherIdentity: Self = Self(0i32);
    pub const Protected: Self = Self(1i32);
    pub const Revoked: Self = Self(2i32);
    pub const Unprotected: Self = Self(3i32);
    pub const LicenseExpired: Self = Self(4i32);
    pub const AccessSuspended: Self = Self(5i32);
}
impl ::core::marker::Copy for DataProtectionStatus {}
impl ::core::clone::Clone for DataProtectionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DataProtectionStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DataProtectionStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for DataProtectionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataProtectionStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DataProtectionStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Security.EnterpriseData.DataProtectionStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EnforcementLevel(pub i32);
impl EnforcementLevel {
    pub const NoProtection: Self = Self(0i32);
    pub const Silent: Self = Self(1i32);
    pub const Override: Self = Self(2i32);
    pub const Block: Self = Self(3i32);
}
impl ::core::marker::Copy for EnforcementLevel {}
impl ::core::clone::Clone for EnforcementLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EnforcementLevel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EnforcementLevel {
    type Abi = Self;
}
impl ::core::fmt::Debug for EnforcementLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnforcementLevel").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EnforcementLevel {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Security.EnterpriseData.EnforcementLevel;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct FileProtectionInfo(::windows_core::IUnknown);
impl FileProtectionInfo {
    pub fn Status(&self) -> ::windows_core::Result<FileProtectionStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<FileProtectionStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FileProtectionStatus>(result__)
        }
    }
    pub fn IsRoamable(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsRoamable)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Identity(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Identity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IsProtectWhileOpenSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IFileProtectionInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsProtectWhileOpenSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for FileProtectionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FileProtectionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileProtectionInfo {}
impl ::core::fmt::Debug for FileProtectionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileProtectionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FileProtectionInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.FileProtectionInfo;{4ee96486-147e-4dd0-8faf-5253ed91ad0c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FileProtectionInfo {
    type Vtable = IFileProtectionInfo_Vtbl;
    const IID: ::windows_core::GUID = <IFileProtectionInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FileProtectionInfo {
    const NAME: &'static str = "Windows.Security.EnterpriseData.FileProtectionInfo";
}
impl ::core::convert::From<FileProtectionInfo> for ::windows_core::IUnknown {
    fn from(value: FileProtectionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileProtectionInfo> for ::windows_core::IUnknown {
    fn from(value: &FileProtectionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FileProtectionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FileProtectionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FileProtectionInfo> for ::windows_core::IInspectable {
    fn from(value: FileProtectionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileProtectionInfo> for ::windows_core::IInspectable {
    fn from(value: &FileProtectionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FileProtectionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FileProtectionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for FileProtectionInfo {}
unsafe impl ::core::marker::Sync for FileProtectionInfo {}
pub struct FileProtectionManager;
impl FileProtectionManager {
    #[cfg(feature = "winrt-storage")]
    pub fn ProtectAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageItem>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(target: Param0, identity: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<FileProtectionInfo>> {
        Self::IFileProtectionManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProtectAsync)(::windows_core::Interface::as_raw(this), target.into_param().abi(), identity.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<FileProtectionInfo>>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn CopyProtectionAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageItem>, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageItem>>(source: Param0, target: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        Self::IFileProtectionManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CopyProtectionAsync)(::windows_core::Interface::as_raw(this), source.into_param().abi(), target.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn GetProtectionInfoAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageItem>>(source: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<FileProtectionInfo>> {
        Self::IFileProtectionManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetProtectionInfoAsync)(::windows_core::Interface::as_raw(this), source.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<FileProtectionInfo>>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn SaveFileAsContainerAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageFile>>(protectedfile: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ProtectedContainerExportResult>> {
        Self::IFileProtectionManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SaveFileAsContainerAsync)(::windows_core::Interface::as_raw(this), protectedfile.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ProtectedContainerExportResult>>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn LoadFileFromContainerAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageFile>>(containerfile: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ProtectedContainerImportResult>> {
        Self::IFileProtectionManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LoadFileFromContainerAsync)(::windows_core::Interface::as_raw(this), containerfile.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ProtectedContainerImportResult>>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn LoadFileFromContainerWithTargetAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageFile>, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageItem>>(containerfile: Param0, target: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ProtectedContainerImportResult>> {
        Self::IFileProtectionManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LoadFileFromContainerWithTargetAsync)(::windows_core::Interface::as_raw(this), containerfile.into_param().abi(), target.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ProtectedContainerImportResult>>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn CreateProtectedAndOpenAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageFolder>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(parentfolder: Param0, desiredname: Param1, identity: Param2, collisionoption: ::winrt_storage::CreationCollisionOption) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ProtectedFileCreateResult>> {
        Self::IFileProtectionManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateProtectedAndOpenAsync)(::windows_core::Interface::as_raw(this), parentfolder.into_param().abi(), desiredname.into_param().abi(), identity.into_param().abi(), collisionoption, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ProtectedFileCreateResult>>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn IsContainerAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageFile>>(file: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        Self::IFileProtectionManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsContainerAsync)(::windows_core::Interface::as_raw(this), file.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn LoadFileFromContainerWithTargetAndNameCollisionOptionAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageFile>, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageItem>>(containerfile: Param0, target: Param1, collisionoption: ::winrt_storage::NameCollisionOption) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ProtectedContainerImportResult>> {
        Self::IFileProtectionManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LoadFileFromContainerWithTargetAndNameCollisionOptionAsync)(::windows_core::Interface::as_raw(this), containerfile.into_param().abi(), target.into_param().abi(), collisionoption, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ProtectedContainerImportResult>>(result__)
        })
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage"))]
    pub fn SaveFileAsContainerWithSharingAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageFile>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(protectedfile: Param0, sharedwithidentities: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ProtectedContainerExportResult>> {
        Self::IFileProtectionManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SaveFileAsContainerWithSharingAsync)(::windows_core::Interface::as_raw(this), protectedfile.into_param().abi(), sharedwithidentities.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ProtectedContainerExportResult>>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn UnprotectAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageItem>>(target: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<FileProtectionInfo>> {
        Self::IFileProtectionManagerStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UnprotectAsync)(::windows_core::Interface::as_raw(this), target.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<FileProtectionInfo>>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn UnprotectWithOptionsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageItem>, Param1: ::windows_core::IntoParam<'a, FileUnprotectOptions>>(target: Param0, options: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<FileProtectionInfo>> {
        Self::IFileProtectionManagerStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UnprotectWithOptionsAsync)(::windows_core::Interface::as_raw(this), target.into_param().abi(), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<FileProtectionInfo>>(result__)
        })
    }
    pub fn IFileProtectionManagerStatics<R, F: FnOnce(&IFileProtectionManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<FileProtectionManager, IFileProtectionManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IFileProtectionManagerStatics2<R, F: FnOnce(&IFileProtectionManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<FileProtectionManager, IFileProtectionManagerStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IFileProtectionManagerStatics3<R, F: FnOnce(&IFileProtectionManagerStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<FileProtectionManager, IFileProtectionManagerStatics3> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for FileProtectionManager {
    const NAME: &'static str = "Windows.Security.EnterpriseData.FileProtectionManager";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FileProtectionStatus(pub i32);
impl FileProtectionStatus {
    pub const Undetermined: Self = Self(0i32);
    pub const Unknown: Self = Self(0i32);
    pub const Unprotected: Self = Self(1i32);
    pub const Revoked: Self = Self(2i32);
    pub const Protected: Self = Self(3i32);
    pub const ProtectedByOtherUser: Self = Self(4i32);
    pub const ProtectedToOtherEnterprise: Self = Self(5i32);
    pub const NotProtectable: Self = Self(6i32);
    pub const ProtectedToOtherIdentity: Self = Self(7i32);
    pub const LicenseExpired: Self = Self(8i32);
    pub const AccessSuspended: Self = Self(9i32);
    pub const FileInUse: Self = Self(10i32);
}
impl ::core::marker::Copy for FileProtectionStatus {}
impl ::core::clone::Clone for FileProtectionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FileProtectionStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FileProtectionStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for FileProtectionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileProtectionStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FileProtectionStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Security.EnterpriseData.FileProtectionStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "winrt-")]
pub struct FileRevocationManager;
#[cfg(feature = "winrt-")]
impl FileRevocationManager {
    #[cfg(all(feature = "winrt-storage", feature = "winrt-"))]
    pub fn ProtectAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageItem>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(storageitem: Param0, enterpriseidentity: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<FileProtectionStatus>> {
        Self::IFileRevocationManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProtectAsync)(::windows_core::Interface::as_raw(this), storageitem.into_param().abi(), enterpriseidentity.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<FileProtectionStatus>>(result__)
        })
    }
    #[cfg(all(feature = "winrt-storage", feature = "winrt-"))]
    pub fn CopyProtectionAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageItem>, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageItem>>(sourcestorageitem: Param0, targetstorageitem: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        Self::IFileRevocationManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CopyProtectionAsync)(::windows_core::Interface::as_raw(this), sourcestorageitem.into_param().abi(), targetstorageitem.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn Revoke<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(enterpriseidentity: Param0) -> ::windows_core::Result<()> {
        Self::IFileRevocationManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).Revoke)(::windows_core::Interface::as_raw(this), enterpriseidentity.into_param().abi()).ok() })
    }
    #[cfg(all(feature = "winrt-storage", feature = "winrt-"))]
    pub fn GetStatusAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageItem>>(storageitem: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<FileProtectionStatus>> {
        Self::IFileRevocationManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetStatusAsync)(::windows_core::Interface::as_raw(this), storageitem.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<FileProtectionStatus>>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn IFileRevocationManagerStatics<R, F: FnOnce(&IFileRevocationManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<FileRevocationManager, IFileRevocationManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for FileRevocationManager {
    const NAME: &'static str = "Windows.Security.EnterpriseData.FileRevocationManager";
}
#[repr(transparent)]
pub struct FileUnprotectOptions(::windows_core::IUnknown);
impl FileUnprotectOptions {
    pub fn SetAudit(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAudit)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Audit(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Audit)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Create(audit: bool) -> ::windows_core::Result<FileUnprotectOptions> {
        Self::IFileUnprotectOptionsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), audit, result__.as_mut_ptr()).from_abi::<FileUnprotectOptions>(result__)
        })
    }
    pub fn IFileUnprotectOptionsFactory<R, F: FnOnce(&IFileUnprotectOptionsFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<FileUnprotectOptions, IFileUnprotectOptionsFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for FileUnprotectOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FileUnprotectOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileUnprotectOptions {}
impl ::core::fmt::Debug for FileUnprotectOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileUnprotectOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FileUnprotectOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.FileUnprotectOptions;{7d1312f1-3b0d-4dd8-a1f8-1ec53822e2f3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FileUnprotectOptions {
    type Vtable = IFileUnprotectOptions_Vtbl;
    const IID: ::windows_core::GUID = <IFileUnprotectOptions as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FileUnprotectOptions {
    const NAME: &'static str = "Windows.Security.EnterpriseData.FileUnprotectOptions";
}
impl ::core::convert::From<FileUnprotectOptions> for ::windows_core::IUnknown {
    fn from(value: FileUnprotectOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileUnprotectOptions> for ::windows_core::IUnknown {
    fn from(value: &FileUnprotectOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FileUnprotectOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FileUnprotectOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FileUnprotectOptions> for ::windows_core::IInspectable {
    fn from(value: FileUnprotectOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileUnprotectOptions> for ::windows_core::IInspectable {
    fn from(value: &FileUnprotectOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FileUnprotectOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FileUnprotectOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for FileUnprotectOptions {}
unsafe impl ::core::marker::Sync for FileUnprotectOptions {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBufferProtectUnprotectResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBufferProtectUnprotectResult {
    type Vtable = IBufferProtectUnprotectResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x47995edc_6cec_4e3a_b251_9e7485d79e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBufferProtectUnprotectResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-storage")]
    pub Buffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    Buffer: usize,
    pub ProtectionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataProtectionInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDataProtectionInfo {
    type Vtable = IDataProtectionInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8420b0c1_5e31_4405_9540_3f943af0cb26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataProtectionInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DataProtectionStatus) -> ::windows_core::HRESULT,
    pub Identity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataProtectionManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDataProtectionManagerStatics {
    type Vtable = IDataProtectionManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb6149b74_9144_4ee4_8a8a_30b5f361430e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataProtectionManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-storage")]
    pub ProtectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::windows_core::RawPtr, identity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    ProtectAsync: usize,
    #[cfg(feature = "winrt-storage")]
    pub UnprotectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    UnprotectAsync: usize,
    #[cfg(feature = "winrt-storage")]
    pub ProtectStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unprotectedstream: ::windows_core::RawPtr, identity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, protectedstream: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    ProtectStreamAsync: usize,
    #[cfg(feature = "winrt-storage")]
    pub UnprotectStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protectedstream: ::windows_core::RawPtr, unprotectedstream: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    UnprotectStreamAsync: usize,
    #[cfg(feature = "winrt-storage")]
    pub GetProtectionInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protecteddata: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    GetProtectionInfoAsync: usize,
    #[cfg(feature = "winrt-storage")]
    pub GetStreamProtectionInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protectedstream: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    GetStreamProtectionInfoAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileProtectionInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFileProtectionInfo {
    type Vtable = IFileProtectionInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4ee96486_147e_4dd0_8faf_5253ed91ad0c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileProtectionInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FileProtectionStatus) -> ::windows_core::HRESULT,
    pub IsRoamable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Identity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileProtectionInfo2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFileProtectionInfo2 {
    type Vtable = IFileProtectionInfo2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x82123a4c_557a_498d_8e94_944cd5836432);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileProtectionInfo2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsProtectWhileOpenSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileProtectionManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFileProtectionManagerStatics {
    type Vtable = IFileProtectionManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5846fc9b_e613_426b_bb38_88cba1dc9adb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileProtectionManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-storage")]
    pub ProtectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: ::windows_core::RawPtr, identity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    ProtectAsync: usize,
    #[cfg(feature = "winrt-storage")]
    pub CopyProtectionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: ::windows_core::RawPtr, target: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    CopyProtectionAsync: usize,
    #[cfg(feature = "winrt-storage")]
    pub GetProtectionInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    GetProtectionInfoAsync: usize,
    #[cfg(feature = "winrt-storage")]
    pub SaveFileAsContainerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protectedfile: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    SaveFileAsContainerAsync: usize,
    #[cfg(feature = "winrt-storage")]
    pub LoadFileFromContainerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, containerfile: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    LoadFileFromContainerAsync: usize,
    #[cfg(feature = "winrt-storage")]
    pub LoadFileFromContainerWithTargetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, containerfile: ::windows_core::RawPtr, target: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    LoadFileFromContainerWithTargetAsync: usize,
    #[cfg(feature = "winrt-storage")]
    pub CreateProtectedAndOpenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parentfolder: ::windows_core::RawPtr, desiredname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, identity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, collisionoption: ::winrt_storage::CreationCollisionOption, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    CreateProtectedAndOpenAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileProtectionManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFileProtectionManagerStatics2 {
    type Vtable = IFileProtectionManagerStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x83d2a745_0483_41ab_b2d5_bc7f23d74ebb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileProtectionManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-storage")]
    pub IsContainerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    IsContainerAsync: usize,
    #[cfg(feature = "winrt-storage")]
    pub LoadFileFromContainerWithTargetAndNameCollisionOptionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, containerfile: ::windows_core::RawPtr, target: ::windows_core::RawPtr, collisionoption: ::winrt_storage::NameCollisionOption, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    LoadFileFromContainerWithTargetAndNameCollisionOptionAsync: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage"))]
    pub SaveFileAsContainerWithSharingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protectedfile: ::windows_core::RawPtr, sharedwithidentities: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-storage")))]
    SaveFileAsContainerWithSharingAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileProtectionManagerStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFileProtectionManagerStatics3 {
    type Vtable = IFileProtectionManagerStatics3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6918849a_624f_46d6_b241_e9cd5fdf3e3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileProtectionManagerStatics3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-storage")]
    pub UnprotectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    UnprotectAsync: usize,
    #[cfg(feature = "winrt-storage")]
    pub UnprotectWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: ::windows_core::RawPtr, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    UnprotectWithOptionsAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IFileRevocationManagerStatics(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IFileRevocationManagerStatics {
    type Vtable = IFileRevocationManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x256bbc3d_1c5d_4260_8c75_9144cfb78ba9);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileRevocationManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "winrt-storage", feature = "winrt-"))]
    pub ProtectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storageitem: ::windows_core::RawPtr, enterpriseidentity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-storage", feature = "winrt-")))]
    ProtectAsync: usize,
    #[cfg(all(feature = "winrt-storage", feature = "winrt-"))]
    pub CopyProtectionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcestorageitem: ::windows_core::RawPtr, targetstorageitem: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-storage", feature = "winrt-")))]
    CopyProtectionAsync: usize,
    #[cfg(feature = "winrt-")]
    pub Revoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enterpriseidentity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Revoke: usize,
    #[cfg(all(feature = "winrt-storage", feature = "winrt-"))]
    pub GetStatusAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storageitem: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-storage", feature = "winrt-")))]
    GetStatusAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileUnprotectOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFileUnprotectOptions {
    type Vtable = IFileUnprotectOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d1312f1_3b0d_4dd8_a1f8_1ec53822e2f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileUnprotectOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetAudit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Audit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileUnprotectOptionsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFileUnprotectOptionsFactory {
    type Vtable = IFileUnprotectOptionsFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x51aeb39c_da8c_4c3f_9bfb_cb73a7cce0dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileUnprotectOptionsFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audit: bool, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProtectedAccessResumedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProtectedAccessResumedEventArgs {
    type Vtable = IProtectedAccessResumedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xac4dca59_5d80_4e95_8c5f_8539450eebe0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectedAccessResumedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Identities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Identities: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProtectedAccessSuspendingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProtectedAccessSuspendingEventArgs {
    type Vtable = IProtectedAccessSuspendingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x75a193e0_a344_429f_b975_04fc1f88c185);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectedAccessSuspendingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Identities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Identities: usize,
    pub Deadline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProtectedContainerExportResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProtectedContainerExportResult {
    type Vtable = IProtectedContainerExportResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3948ef95_f7fb_4b42_afb0_df70b41543c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectedContainerExportResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ProtectedImportExportStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    File: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProtectedContainerImportResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProtectedContainerImportResult {
    type Vtable = IProtectedContainerImportResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcdb780d1_e7bb_4d1a_9339_34dc41149f9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectedContainerImportResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ProtectedImportExportStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    File: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProtectedContentRevokedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProtectedContentRevokedEventArgs {
    type Vtable = IProtectedContentRevokedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x63686821_58b9_47ee_93d9_f0f741cf43f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectedContentRevokedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Identities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Identities: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProtectedFileCreateResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProtectedFileCreateResult {
    type Vtable = IProtectedFileCreateResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x28e3ed6a_e9e7_4a03_9f53_bdb16172699b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectedFileCreateResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-storage")]
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    File: usize,
    #[cfg(feature = "winrt-storage")]
    pub Stream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    Stream: usize,
    pub ProtectionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProtectionPolicyAuditInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProtectionPolicyAuditInfo {
    type Vtable = IProtectionPolicyAuditInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x425ab7e4_feb7_44fc_b3bb_c3c4d7ecbebb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyAuditInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ProtectionPolicyAuditAction) -> ::windows_core::HRESULT,
    pub Action: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ProtectionPolicyAuditAction) -> ::windows_core::HRESULT,
    pub SetDataDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DataDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetSourceDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SourceDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTargetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TargetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProtectionPolicyAuditInfoFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProtectionPolicyAuditInfoFactory {
    type Vtable = IProtectionPolicyAuditInfoFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7ed4180b_92e8_42d5_83d4_25440b423549);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyAuditInfoFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, action: ProtectionPolicyAuditAction, datadescription: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, sourcedescription: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, targetdescription: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateWithActionAndDataDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, action: ProtectionPolicyAuditAction, datadescription: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProtectionPolicyManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProtectionPolicyManager {
    type Vtable = IProtectionPolicyManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd5703e18_a08d_47e6_a240_9934d7165eb5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Identity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProtectionPolicyManager2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProtectionPolicyManager2 {
    type Vtable = IProtectionPolicyManager2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xabf7527a_8435_417f_99b6_51beaf365888);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManager2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetShowEnterpriseIndicator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ShowEnterpriseIndicator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProtectionPolicyManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProtectionPolicyManagerStatics {
    type Vtable = IProtectionPolicyManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc0bffc66_8c3d_4d56_8804_c68f0ad32ec5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsIdentityManaged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
    pub TryApplyProcessUIPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ClearProcessUIPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateCurrentThreadNetworkContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-networking")]
    pub GetPrimaryManagedIdentityForNetworkEndpointAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpointhost: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-networking"))]
    GetPrimaryManagedIdentityForNetworkEndpointAsync: usize,
    pub RevokeContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ProtectedAccessSuspending: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveProtectedAccessSuspending: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ProtectedAccessResumed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveProtectedAccessResumed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ProtectedContentRevoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveProtectedContentRevoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub CheckAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceidentity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ProtectionPolicyEvaluationResult) -> ::windows_core::HRESULT,
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceidentity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProtectionPolicyManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProtectionPolicyManagerStatics2 {
    type Vtable = IProtectionPolicyManagerStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb68f9a8c_39e0_4649_b2e4_070ab8a579b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub HasContentBeenRevokedSince: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, since: ::winrt_foundation::DateTime, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CheckAccessForApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceidentity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ProtectionPolicyEvaluationResult) -> ::windows_core::HRESULT,
    pub RequestAccessForAppAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceidentity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetEnforcementLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut EnforcementLevel) -> ::windows_core::HRESULT,
    pub IsUserDecryptionAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsProtectionUnderLockRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
    pub PolicyChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePolicyChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub IsProtectionEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProtectionPolicyManagerStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProtectionPolicyManagerStatics3 {
    type Vtable = IProtectionPolicyManagerStatics3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x48ff9e8c_6a6f_4d9f_bced_18ab537aa015);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerStatics3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RequestAccessWithAuditingInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceidentity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, auditinfo: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestAccessWithMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceidentity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, auditinfo: ::windows_core::RawPtr, messagefromapp: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestAccessForAppWithAuditingInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceidentity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, auditinfo: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestAccessForAppWithMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceidentity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, auditinfo: ::windows_core::RawPtr, messagefromapp: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LogAuditEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceidentity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, auditinfo: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProtectionPolicyManagerStatics4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProtectionPolicyManagerStatics4 {
    type Vtable = IProtectionPolicyManagerStatics4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x20b794db_ccbd_490f_8c83_49ccb77aea6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerStatics4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsRoamableProtectionEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
    pub RequestAccessWithBehaviorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceidentity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, auditinfo: ::windows_core::RawPtr, messagefromapp: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, behavior: ProtectionPolicyRequestAccessBehavior, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestAccessForAppWithBehaviorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceidentity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, auditinfo: ::windows_core::RawPtr, messagefromapp: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, behavior: ProtectionPolicyRequestAccessBehavior, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage"))]
    pub RequestAccessToFilesForAppAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceitemlist: ::windows_core::RawPtr, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, auditinfo: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-storage")))]
    RequestAccessToFilesForAppAsync: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage"))]
    pub RequestAccessToFilesForAppWithMessageAndBehaviorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceitemlist: ::windows_core::RawPtr, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, auditinfo: ::windows_core::RawPtr, messagefromapp: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, behavior: ProtectionPolicyRequestAccessBehavior, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-storage")))]
    RequestAccessToFilesForAppWithMessageAndBehaviorAsync: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage"))]
    pub RequestAccessToFilesForProcessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceitemlist: ::windows_core::RawPtr, processid: u32, auditinfo: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-storage")))]
    RequestAccessToFilesForProcessAsync: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage"))]
    pub RequestAccessToFilesForProcessWithMessageAndBehaviorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceitemlist: ::windows_core::RawPtr, processid: u32, auditinfo: ::windows_core::RawPtr, messagefromapp: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, behavior: ProtectionPolicyRequestAccessBehavior, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-storage")))]
    RequestAccessToFilesForProcessWithMessageAndBehaviorAsync: usize,
    #[cfg(feature = "winrt-storage")]
    pub IsFileProtectionRequiredAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: ::windows_core::RawPtr, identity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    IsFileProtectionRequiredAsync: usize,
    #[cfg(feature = "winrt-storage")]
    pub IsFileProtectionRequiredForNewFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parentfolder: ::windows_core::RawPtr, identity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, desiredname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    IsFileProtectionRequiredForNewFileAsync: usize,
    pub PrimaryManagedIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetPrimaryManagedIdentityForIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IThreadNetworkContext(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IThreadNetworkContext {
    type Vtable = IThreadNetworkContext_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfa4ea8e9_ef13_405a_b12c_d7348c6f41fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IThreadNetworkContext_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[repr(transparent)]
pub struct ProtectedAccessResumedEventArgs(::windows_core::IUnknown);
impl ProtectedAccessResumedEventArgs {
    #[cfg(feature = "winrt-foundation")]
    pub fn Identities(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Identities)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
}
impl ::core::clone::Clone for ProtectedAccessResumedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProtectedAccessResumedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProtectedAccessResumedEventArgs {}
impl ::core::fmt::Debug for ProtectedAccessResumedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtectedAccessResumedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProtectedAccessResumedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.ProtectedAccessResumedEventArgs;{ac4dca59-5d80-4e95-8c5f-8539450eebe0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ProtectedAccessResumedEventArgs {
    type Vtable = IProtectedAccessResumedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IProtectedAccessResumedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ProtectedAccessResumedEventArgs {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectedAccessResumedEventArgs";
}
impl ::core::convert::From<ProtectedAccessResumedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ProtectedAccessResumedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtectedAccessResumedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ProtectedAccessResumedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ProtectedAccessResumedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ProtectedAccessResumedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProtectedAccessResumedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ProtectedAccessResumedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtectedAccessResumedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ProtectedAccessResumedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ProtectedAccessResumedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ProtectedAccessResumedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProtectedAccessResumedEventArgs {}
unsafe impl ::core::marker::Sync for ProtectedAccessResumedEventArgs {}
#[repr(transparent)]
pub struct ProtectedAccessSuspendingEventArgs(::windows_core::IUnknown);
impl ProtectedAccessSuspendingEventArgs {
    #[cfg(feature = "winrt-foundation")]
    pub fn Identities(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Identities)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn Deadline(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Deadline)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for ProtectedAccessSuspendingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProtectedAccessSuspendingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProtectedAccessSuspendingEventArgs {}
impl ::core::fmt::Debug for ProtectedAccessSuspendingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtectedAccessSuspendingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProtectedAccessSuspendingEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.ProtectedAccessSuspendingEventArgs;{75a193e0-a344-429f-b975-04fc1f88c185})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ProtectedAccessSuspendingEventArgs {
    type Vtable = IProtectedAccessSuspendingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IProtectedAccessSuspendingEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ProtectedAccessSuspendingEventArgs {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectedAccessSuspendingEventArgs";
}
impl ::core::convert::From<ProtectedAccessSuspendingEventArgs> for ::windows_core::IUnknown {
    fn from(value: ProtectedAccessSuspendingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtectedAccessSuspendingEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ProtectedAccessSuspendingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ProtectedAccessSuspendingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ProtectedAccessSuspendingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProtectedAccessSuspendingEventArgs> for ::windows_core::IInspectable {
    fn from(value: ProtectedAccessSuspendingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtectedAccessSuspendingEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ProtectedAccessSuspendingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ProtectedAccessSuspendingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ProtectedAccessSuspendingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProtectedAccessSuspendingEventArgs {}
unsafe impl ::core::marker::Sync for ProtectedAccessSuspendingEventArgs {}
#[repr(transparent)]
pub struct ProtectedContainerExportResult(::windows_core::IUnknown);
impl ProtectedContainerExportResult {
    pub fn Status(&self) -> ::windows_core::Result<ProtectedImportExportStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ProtectedImportExportStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ProtectedImportExportStatus>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn File(&self) -> ::windows_core::Result<::winrt_storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).File)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::StorageFile>(result__)
        }
    }
}
impl ::core::clone::Clone for ProtectedContainerExportResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProtectedContainerExportResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProtectedContainerExportResult {}
impl ::core::fmt::Debug for ProtectedContainerExportResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtectedContainerExportResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProtectedContainerExportResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.ProtectedContainerExportResult;{3948ef95-f7fb-4b42-afb0-df70b41543c1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ProtectedContainerExportResult {
    type Vtable = IProtectedContainerExportResult_Vtbl;
    const IID: ::windows_core::GUID = <IProtectedContainerExportResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ProtectedContainerExportResult {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectedContainerExportResult";
}
impl ::core::convert::From<ProtectedContainerExportResult> for ::windows_core::IUnknown {
    fn from(value: ProtectedContainerExportResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtectedContainerExportResult> for ::windows_core::IUnknown {
    fn from(value: &ProtectedContainerExportResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ProtectedContainerExportResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ProtectedContainerExportResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProtectedContainerExportResult> for ::windows_core::IInspectable {
    fn from(value: ProtectedContainerExportResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtectedContainerExportResult> for ::windows_core::IInspectable {
    fn from(value: &ProtectedContainerExportResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ProtectedContainerExportResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ProtectedContainerExportResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProtectedContainerExportResult {}
unsafe impl ::core::marker::Sync for ProtectedContainerExportResult {}
#[repr(transparent)]
pub struct ProtectedContainerImportResult(::windows_core::IUnknown);
impl ProtectedContainerImportResult {
    pub fn Status(&self) -> ::windows_core::Result<ProtectedImportExportStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ProtectedImportExportStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ProtectedImportExportStatus>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn File(&self) -> ::windows_core::Result<::winrt_storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).File)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::StorageFile>(result__)
        }
    }
}
impl ::core::clone::Clone for ProtectedContainerImportResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProtectedContainerImportResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProtectedContainerImportResult {}
impl ::core::fmt::Debug for ProtectedContainerImportResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtectedContainerImportResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProtectedContainerImportResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.ProtectedContainerImportResult;{cdb780d1-e7bb-4d1a-9339-34dc41149f9b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ProtectedContainerImportResult {
    type Vtable = IProtectedContainerImportResult_Vtbl;
    const IID: ::windows_core::GUID = <IProtectedContainerImportResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ProtectedContainerImportResult {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectedContainerImportResult";
}
impl ::core::convert::From<ProtectedContainerImportResult> for ::windows_core::IUnknown {
    fn from(value: ProtectedContainerImportResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtectedContainerImportResult> for ::windows_core::IUnknown {
    fn from(value: &ProtectedContainerImportResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ProtectedContainerImportResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ProtectedContainerImportResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProtectedContainerImportResult> for ::windows_core::IInspectable {
    fn from(value: ProtectedContainerImportResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtectedContainerImportResult> for ::windows_core::IInspectable {
    fn from(value: &ProtectedContainerImportResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ProtectedContainerImportResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ProtectedContainerImportResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProtectedContainerImportResult {}
unsafe impl ::core::marker::Sync for ProtectedContainerImportResult {}
#[repr(transparent)]
pub struct ProtectedContentRevokedEventArgs(::windows_core::IUnknown);
impl ProtectedContentRevokedEventArgs {
    #[cfg(feature = "winrt-foundation")]
    pub fn Identities(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Identities)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
}
impl ::core::clone::Clone for ProtectedContentRevokedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProtectedContentRevokedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProtectedContentRevokedEventArgs {}
impl ::core::fmt::Debug for ProtectedContentRevokedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtectedContentRevokedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProtectedContentRevokedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.ProtectedContentRevokedEventArgs;{63686821-58b9-47ee-93d9-f0f741cf43f0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ProtectedContentRevokedEventArgs {
    type Vtable = IProtectedContentRevokedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IProtectedContentRevokedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ProtectedContentRevokedEventArgs {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectedContentRevokedEventArgs";
}
impl ::core::convert::From<ProtectedContentRevokedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ProtectedContentRevokedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtectedContentRevokedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ProtectedContentRevokedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ProtectedContentRevokedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ProtectedContentRevokedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProtectedContentRevokedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ProtectedContentRevokedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtectedContentRevokedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ProtectedContentRevokedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ProtectedContentRevokedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ProtectedContentRevokedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProtectedContentRevokedEventArgs {}
unsafe impl ::core::marker::Sync for ProtectedContentRevokedEventArgs {}
#[repr(transparent)]
pub struct ProtectedFileCreateResult(::windows_core::IUnknown);
impl ProtectedFileCreateResult {
    #[cfg(feature = "winrt-storage")]
    pub fn File(&self) -> ::windows_core::Result<::winrt_storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).File)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::StorageFile>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn Stream(&self) -> ::windows_core::Result<::winrt_storage::Streams::IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Stream)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IRandomAccessStream>(result__)
        }
    }
    pub fn ProtectionInfo(&self) -> ::windows_core::Result<FileProtectionInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FileProtectionInfo>(result__)
        }
    }
}
impl ::core::clone::Clone for ProtectedFileCreateResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProtectedFileCreateResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProtectedFileCreateResult {}
impl ::core::fmt::Debug for ProtectedFileCreateResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtectedFileCreateResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProtectedFileCreateResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.ProtectedFileCreateResult;{28e3ed6a-e9e7-4a03-9f53-bdb16172699b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ProtectedFileCreateResult {
    type Vtable = IProtectedFileCreateResult_Vtbl;
    const IID: ::windows_core::GUID = <IProtectedFileCreateResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ProtectedFileCreateResult {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectedFileCreateResult";
}
impl ::core::convert::From<ProtectedFileCreateResult> for ::windows_core::IUnknown {
    fn from(value: ProtectedFileCreateResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtectedFileCreateResult> for ::windows_core::IUnknown {
    fn from(value: &ProtectedFileCreateResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ProtectedFileCreateResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ProtectedFileCreateResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProtectedFileCreateResult> for ::windows_core::IInspectable {
    fn from(value: ProtectedFileCreateResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtectedFileCreateResult> for ::windows_core::IInspectable {
    fn from(value: &ProtectedFileCreateResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ProtectedFileCreateResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ProtectedFileCreateResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProtectedFileCreateResult {}
unsafe impl ::core::marker::Sync for ProtectedFileCreateResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ProtectedImportExportStatus(pub i32);
impl ProtectedImportExportStatus {
    pub const Ok: Self = Self(0i32);
    pub const Undetermined: Self = Self(1i32);
    pub const Unprotected: Self = Self(2i32);
    pub const Revoked: Self = Self(3i32);
    pub const NotRoamable: Self = Self(4i32);
    pub const ProtectedToOtherIdentity: Self = Self(5i32);
    pub const LicenseExpired: Self = Self(6i32);
    pub const AccessSuspended: Self = Self(7i32);
}
impl ::core::marker::Copy for ProtectedImportExportStatus {}
impl ::core::clone::Clone for ProtectedImportExportStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ProtectedImportExportStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ProtectedImportExportStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for ProtectedImportExportStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtectedImportExportStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProtectedImportExportStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Security.EnterpriseData.ProtectedImportExportStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ProtectionPolicyAuditAction(pub i32);
impl ProtectionPolicyAuditAction {
    pub const Decrypt: Self = Self(0i32);
    pub const CopyToLocation: Self = Self(1i32);
    pub const SendToRecipient: Self = Self(2i32);
    pub const Other: Self = Self(3i32);
}
impl ::core::marker::Copy for ProtectionPolicyAuditAction {}
impl ::core::clone::Clone for ProtectionPolicyAuditAction {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ProtectionPolicyAuditAction {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ProtectionPolicyAuditAction {
    type Abi = Self;
}
impl ::core::fmt::Debug for ProtectionPolicyAuditAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtectionPolicyAuditAction").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProtectionPolicyAuditAction {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Security.EnterpriseData.ProtectionPolicyAuditAction;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ProtectionPolicyAuditInfo(::windows_core::IUnknown);
impl ProtectionPolicyAuditInfo {
    pub fn SetAction(&self, value: ProtectionPolicyAuditAction) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAction)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Action(&self) -> ::windows_core::Result<ProtectionPolicyAuditAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ProtectionPolicyAuditAction>::zeroed();
            (::windows_core::Interface::vtable(this).Action)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ProtectionPolicyAuditAction>(result__)
        }
    }
    pub fn SetDataDescription<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDataDescription)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn DataDescription(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DataDescription)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetSourceDescription<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSourceDescription)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SourceDescription(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SourceDescription)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetTargetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTargetDescription)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn TargetDescription(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TargetDescription)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Create<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(action: ProtectionPolicyAuditAction, datadescription: Param1, sourcedescription: Param2, targetdescription: Param3) -> ::windows_core::Result<ProtectionPolicyAuditInfo> {
        Self::IProtectionPolicyAuditInfoFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), action, datadescription.into_param().abi(), sourcedescription.into_param().abi(), targetdescription.into_param().abi(), result__.as_mut_ptr()).from_abi::<ProtectionPolicyAuditInfo>(result__)
        })
    }
    pub fn CreateWithActionAndDataDescription<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(action: ProtectionPolicyAuditAction, datadescription: Param1) -> ::windows_core::Result<ProtectionPolicyAuditInfo> {
        Self::IProtectionPolicyAuditInfoFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithActionAndDataDescription)(::windows_core::Interface::as_raw(this), action, datadescription.into_param().abi(), result__.as_mut_ptr()).from_abi::<ProtectionPolicyAuditInfo>(result__)
        })
    }
    pub fn IProtectionPolicyAuditInfoFactory<R, F: FnOnce(&IProtectionPolicyAuditInfoFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ProtectionPolicyAuditInfo, IProtectionPolicyAuditInfoFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ProtectionPolicyAuditInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProtectionPolicyAuditInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProtectionPolicyAuditInfo {}
impl ::core::fmt::Debug for ProtectionPolicyAuditInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtectionPolicyAuditInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProtectionPolicyAuditInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.ProtectionPolicyAuditInfo;{425ab7e4-feb7-44fc-b3bb-c3c4d7ecbebb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ProtectionPolicyAuditInfo {
    type Vtable = IProtectionPolicyAuditInfo_Vtbl;
    const IID: ::windows_core::GUID = <IProtectionPolicyAuditInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ProtectionPolicyAuditInfo {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectionPolicyAuditInfo";
}
impl ::core::convert::From<ProtectionPolicyAuditInfo> for ::windows_core::IUnknown {
    fn from(value: ProtectionPolicyAuditInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtectionPolicyAuditInfo> for ::windows_core::IUnknown {
    fn from(value: &ProtectionPolicyAuditInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ProtectionPolicyAuditInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ProtectionPolicyAuditInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProtectionPolicyAuditInfo> for ::windows_core::IInspectable {
    fn from(value: ProtectionPolicyAuditInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtectionPolicyAuditInfo> for ::windows_core::IInspectable {
    fn from(value: &ProtectionPolicyAuditInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ProtectionPolicyAuditInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ProtectionPolicyAuditInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProtectionPolicyAuditInfo {}
unsafe impl ::core::marker::Sync for ProtectionPolicyAuditInfo {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ProtectionPolicyEvaluationResult(pub i32);
impl ProtectionPolicyEvaluationResult {
    pub const Allowed: Self = Self(0i32);
    pub const Blocked: Self = Self(1i32);
    pub const ConsentRequired: Self = Self(2i32);
}
impl ::core::marker::Copy for ProtectionPolicyEvaluationResult {}
impl ::core::clone::Clone for ProtectionPolicyEvaluationResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ProtectionPolicyEvaluationResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ProtectionPolicyEvaluationResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for ProtectionPolicyEvaluationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtectionPolicyEvaluationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProtectionPolicyEvaluationResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Security.EnterpriseData.ProtectionPolicyEvaluationResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ProtectionPolicyManager(::windows_core::IUnknown);
impl ProtectionPolicyManager {
    pub fn SetIdentity<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIdentity)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Identity(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Identity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetShowEnterpriseIndicator(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IProtectionPolicyManager2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetShowEnterpriseIndicator)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ShowEnterpriseIndicator(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IProtectionPolicyManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ShowEnterpriseIndicator)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsIdentityManaged<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(identity: Param0) -> ::windows_core::Result<bool> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsIdentityManaged)(::windows_core::Interface::as_raw(this), identity.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn TryApplyProcessUIPolicy<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(identity: Param0) -> ::windows_core::Result<bool> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryApplyProcessUIPolicy)(::windows_core::Interface::as_raw(this), identity.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn ClearProcessUIPolicy() -> ::windows_core::Result<()> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).ClearProcessUIPolicy)(::windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn CreateCurrentThreadNetworkContext<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(identity: Param0) -> ::windows_core::Result<ThreadNetworkContext> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateCurrentThreadNetworkContext)(::windows_core::Interface::as_raw(this), identity.into_param().abi(), result__.as_mut_ptr()).from_abi::<ThreadNetworkContext>(result__)
        })
    }
    #[cfg(feature = "winrt-networking")]
    pub fn GetPrimaryManagedIdentityForNetworkEndpointAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_networking::HostName>>(endpointhost: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPrimaryManagedIdentityForNetworkEndpointAsync)(::windows_core::Interface::as_raw(this), endpointhost.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        })
    }
    pub fn RevokeContent<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(identity: Param0) -> ::windows_core::Result<()> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RevokeContent)(::windows_core::Interface::as_raw(this), identity.into_param().abi()).ok() })
    }
    pub fn GetForCurrentView() -> ::windows_core::Result<ProtectionPolicyManager> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ProtectionPolicyManager>(result__)
        })
    }
    pub fn ProtectedAccessSuspending<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<ProtectedAccessSuspendingEventArgs>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ProtectedAccessSuspending)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveProtectedAccessSuspending<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveProtectedAccessSuspending)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn ProtectedAccessResumed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<ProtectedAccessResumedEventArgs>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ProtectedAccessResumed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveProtectedAccessResumed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveProtectedAccessResumed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn ProtectedContentRevoked<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<ProtectedContentRevokedEventArgs>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ProtectedContentRevoked)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveProtectedContentRevoked<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveProtectedContentRevoked)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn CheckAccess<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(sourceidentity: Param0, targetidentity: Param1) -> ::windows_core::Result<ProtectionPolicyEvaluationResult> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ProtectionPolicyEvaluationResult>::zeroed();
            (::windows_core::Interface::vtable(this).CheckAccess)(::windows_core::Interface::as_raw(this), sourceidentity.into_param().abi(), targetidentity.into_param().abi(), result__.as_mut_ptr()).from_abi::<ProtectionPolicyEvaluationResult>(result__)
        })
    }
    pub fn RequestAccessAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(sourceidentity: Param0, targetidentity: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessAsync)(::windows_core::Interface::as_raw(this), sourceidentity.into_param().abi(), targetidentity.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>(result__)
        })
    }
    pub fn HasContentBeenRevokedSince<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>>(identity: Param0, since: Param1) -> ::windows_core::Result<bool> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasContentBeenRevokedSince)(::windows_core::Interface::as_raw(this), identity.into_param().abi(), since.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn CheckAccessForApp<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(sourceidentity: Param0, apppackagefamilyname: Param1) -> ::windows_core::Result<ProtectionPolicyEvaluationResult> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ProtectionPolicyEvaluationResult>::zeroed();
            (::windows_core::Interface::vtable(this).CheckAccessForApp)(::windows_core::Interface::as_raw(this), sourceidentity.into_param().abi(), apppackagefamilyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<ProtectionPolicyEvaluationResult>(result__)
        })
    }
    pub fn RequestAccessForAppAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(sourceidentity: Param0, apppackagefamilyname: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessForAppAsync)(::windows_core::Interface::as_raw(this), sourceidentity.into_param().abi(), apppackagefamilyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>(result__)
        })
    }
    pub fn GetEnforcementLevel<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(identity: Param0) -> ::windows_core::Result<EnforcementLevel> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EnforcementLevel>::zeroed();
            (::windows_core::Interface::vtable(this).GetEnforcementLevel)(::windows_core::Interface::as_raw(this), identity.into_param().abi(), result__.as_mut_ptr()).from_abi::<EnforcementLevel>(result__)
        })
    }
    pub fn IsUserDecryptionAllowed<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(identity: Param0) -> ::windows_core::Result<bool> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsUserDecryptionAllowed)(::windows_core::Interface::as_raw(this), identity.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn IsProtectionUnderLockRequired<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(identity: Param0) -> ::windows_core::Result<bool> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsProtectionUnderLockRequired)(::windows_core::Interface::as_raw(this), identity.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn PolicyChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PolicyChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemovePolicyChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).RemovePolicyChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn IsProtectionEnabled() -> ::windows_core::Result<bool> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsProtectionEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn RequestAccessWithAuditingInfoAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ProtectionPolicyAuditInfo>>(sourceidentity: Param0, targetidentity: Param1, auditinfo: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        Self::IProtectionPolicyManagerStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessWithAuditingInfoAsync)(::windows_core::Interface::as_raw(this), sourceidentity.into_param().abi(), targetidentity.into_param().abi(), auditinfo.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>(result__)
        })
    }
    pub fn RequestAccessWithMessageAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ProtectionPolicyAuditInfo>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(sourceidentity: Param0, targetidentity: Param1, auditinfo: Param2, messagefromapp: Param3) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        Self::IProtectionPolicyManagerStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessWithMessageAsync)(::windows_core::Interface::as_raw(this), sourceidentity.into_param().abi(), targetidentity.into_param().abi(), auditinfo.into_param().abi(), messagefromapp.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>(result__)
        })
    }
    pub fn RequestAccessForAppWithAuditingInfoAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ProtectionPolicyAuditInfo>>(sourceidentity: Param0, apppackagefamilyname: Param1, auditinfo: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        Self::IProtectionPolicyManagerStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessForAppWithAuditingInfoAsync)(::windows_core::Interface::as_raw(this), sourceidentity.into_param().abi(), apppackagefamilyname.into_param().abi(), auditinfo.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>(result__)
        })
    }
    pub fn RequestAccessForAppWithMessageAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ProtectionPolicyAuditInfo>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(sourceidentity: Param0, apppackagefamilyname: Param1, auditinfo: Param2, messagefromapp: Param3) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        Self::IProtectionPolicyManagerStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessForAppWithMessageAsync)(::windows_core::Interface::as_raw(this), sourceidentity.into_param().abi(), apppackagefamilyname.into_param().abi(), auditinfo.into_param().abi(), messagefromapp.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>(result__)
        })
    }
    pub fn LogAuditEvent<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ProtectionPolicyAuditInfo>>(sourceidentity: Param0, targetidentity: Param1, auditinfo: Param2) -> ::windows_core::Result<()> {
        Self::IProtectionPolicyManagerStatics3(|this| unsafe { (::windows_core::Interface::vtable(this).LogAuditEvent)(::windows_core::Interface::as_raw(this), sourceidentity.into_param().abi(), targetidentity.into_param().abi(), auditinfo.into_param().abi()).ok() })
    }
    pub fn IsRoamableProtectionEnabled<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(identity: Param0) -> ::windows_core::Result<bool> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsRoamableProtectionEnabled)(::windows_core::Interface::as_raw(this), identity.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn RequestAccessWithBehaviorAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ProtectionPolicyAuditInfo>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(sourceidentity: Param0, targetidentity: Param1, auditinfo: Param2, messagefromapp: Param3, behavior: ProtectionPolicyRequestAccessBehavior) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessWithBehaviorAsync)(::windows_core::Interface::as_raw(this), sourceidentity.into_param().abi(), targetidentity.into_param().abi(), auditinfo.into_param().abi(), messagefromapp.into_param().abi(), behavior, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>(result__)
        })
    }
    pub fn RequestAccessForAppWithBehaviorAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ProtectionPolicyAuditInfo>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(sourceidentity: Param0, apppackagefamilyname: Param1, auditinfo: Param2, messagefromapp: Param3, behavior: ProtectionPolicyRequestAccessBehavior) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessForAppWithBehaviorAsync)(::windows_core::Interface::as_raw(this), sourceidentity.into_param().abi(), apppackagefamilyname.into_param().abi(), auditinfo.into_param().abi(), messagefromapp.into_param().abi(), behavior, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>(result__)
        })
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage"))]
    pub fn RequestAccessToFilesForAppAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_storage::IStorageItem>>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ProtectionPolicyAuditInfo>>(sourceitemlist: Param0, apppackagefamilyname: Param1, auditinfo: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessToFilesForAppAsync)(::windows_core::Interface::as_raw(this), sourceitemlist.into_param().abi(), apppackagefamilyname.into_param().abi(), auditinfo.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>(result__)
        })
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage"))]
    pub fn RequestAccessToFilesForAppWithMessageAndBehaviorAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_storage::IStorageItem>>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ProtectionPolicyAuditInfo>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(sourceitemlist: Param0, apppackagefamilyname: Param1, auditinfo: Param2, messagefromapp: Param3, behavior: ProtectionPolicyRequestAccessBehavior) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessToFilesForAppWithMessageAndBehaviorAsync)(::windows_core::Interface::as_raw(this), sourceitemlist.into_param().abi(), apppackagefamilyname.into_param().abi(), auditinfo.into_param().abi(), messagefromapp.into_param().abi(), behavior, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>(result__)
        })
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage"))]
    pub fn RequestAccessToFilesForProcessAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_storage::IStorageItem>>, Param2: ::windows_core::IntoParam<'a, ProtectionPolicyAuditInfo>>(sourceitemlist: Param0, processid: u32, auditinfo: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessToFilesForProcessAsync)(::windows_core::Interface::as_raw(this), sourceitemlist.into_param().abi(), processid, auditinfo.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>(result__)
        })
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage"))]
    pub fn RequestAccessToFilesForProcessWithMessageAndBehaviorAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_storage::IStorageItem>>, Param2: ::windows_core::IntoParam<'a, ProtectionPolicyAuditInfo>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(sourceitemlist: Param0, processid: u32, auditinfo: Param2, messagefromapp: Param3, behavior: ProtectionPolicyRequestAccessBehavior) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessToFilesForProcessWithMessageAndBehaviorAsync)(::windows_core::Interface::as_raw(this), sourceitemlist.into_param().abi(), processid, auditinfo.into_param().abi(), messagefromapp.into_param().abi(), behavior, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn IsFileProtectionRequiredAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageItem>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(target: Param0, identity: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsFileProtectionRequiredAsync)(::windows_core::Interface::as_raw(this), target.into_param().abi(), identity.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn IsFileProtectionRequiredForNewFileAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageFolder>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(parentfolder: Param0, identity: Param1, desiredname: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsFileProtectionRequiredForNewFileAsync)(::windows_core::Interface::as_raw(this), parentfolder.into_param().abi(), identity.into_param().abi(), desiredname.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        })
    }
    pub fn PrimaryManagedIdentity() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PrimaryManagedIdentity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn GetPrimaryManagedIdentityForIdentity<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(identity: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetPrimaryManagedIdentityForIdentity)(::windows_core::Interface::as_raw(this), identity.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn IProtectionPolicyManagerStatics<R, F: FnOnce(&IProtectionPolicyManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ProtectionPolicyManager, IProtectionPolicyManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IProtectionPolicyManagerStatics2<R, F: FnOnce(&IProtectionPolicyManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ProtectionPolicyManager, IProtectionPolicyManagerStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IProtectionPolicyManagerStatics3<R, F: FnOnce(&IProtectionPolicyManagerStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ProtectionPolicyManager, IProtectionPolicyManagerStatics3> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IProtectionPolicyManagerStatics4<R, F: FnOnce(&IProtectionPolicyManagerStatics4) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ProtectionPolicyManager, IProtectionPolicyManagerStatics4> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ProtectionPolicyManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProtectionPolicyManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProtectionPolicyManager {}
impl ::core::fmt::Debug for ProtectionPolicyManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtectionPolicyManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProtectionPolicyManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.ProtectionPolicyManager;{d5703e18-a08d-47e6-a240-9934d7165eb5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ProtectionPolicyManager {
    type Vtable = IProtectionPolicyManager_Vtbl;
    const IID: ::windows_core::GUID = <IProtectionPolicyManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ProtectionPolicyManager {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectionPolicyManager";
}
impl ::core::convert::From<ProtectionPolicyManager> for ::windows_core::IUnknown {
    fn from(value: ProtectionPolicyManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtectionPolicyManager> for ::windows_core::IUnknown {
    fn from(value: &ProtectionPolicyManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ProtectionPolicyManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ProtectionPolicyManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProtectionPolicyManager> for ::windows_core::IInspectable {
    fn from(value: ProtectionPolicyManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtectionPolicyManager> for ::windows_core::IInspectable {
    fn from(value: &ProtectionPolicyManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ProtectionPolicyManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ProtectionPolicyManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProtectionPolicyManager {}
unsafe impl ::core::marker::Sync for ProtectionPolicyManager {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ProtectionPolicyRequestAccessBehavior(pub i32);
impl ProtectionPolicyRequestAccessBehavior {
    pub const Decrypt: Self = Self(0i32);
    pub const TreatOverridePolicyAsBlock: Self = Self(1i32);
}
impl ::core::marker::Copy for ProtectionPolicyRequestAccessBehavior {}
impl ::core::clone::Clone for ProtectionPolicyRequestAccessBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ProtectionPolicyRequestAccessBehavior {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ProtectionPolicyRequestAccessBehavior {
    type Abi = Self;
}
impl ::core::fmt::Debug for ProtectionPolicyRequestAccessBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtectionPolicyRequestAccessBehavior").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProtectionPolicyRequestAccessBehavior {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Security.EnterpriseData.ProtectionPolicyRequestAccessBehavior;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ThreadNetworkContext(::windows_core::IUnknown);
impl ThreadNetworkContext {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for ThreadNetworkContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ThreadNetworkContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ThreadNetworkContext {}
impl ::core::fmt::Debug for ThreadNetworkContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ThreadNetworkContext").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ThreadNetworkContext {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.ThreadNetworkContext;{fa4ea8e9-ef13-405a-b12c-d7348c6f41fc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ThreadNetworkContext {
    type Vtable = IThreadNetworkContext_Vtbl;
    const IID: ::windows_core::GUID = <IThreadNetworkContext as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ThreadNetworkContext {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ThreadNetworkContext";
}
impl ::core::convert::From<ThreadNetworkContext> for ::windows_core::IUnknown {
    fn from(value: ThreadNetworkContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ThreadNetworkContext> for ::windows_core::IUnknown {
    fn from(value: &ThreadNetworkContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ThreadNetworkContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ThreadNetworkContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ThreadNetworkContext> for ::windows_core::IInspectable {
    fn from(value: ThreadNetworkContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ThreadNetworkContext> for ::windows_core::IInspectable {
    fn from(value: &ThreadNetworkContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ThreadNetworkContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ThreadNetworkContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ThreadNetworkContext> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: ThreadNetworkContext) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ThreadNetworkContext> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &ThreadNetworkContext) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for ThreadNetworkContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &ThreadNetworkContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ThreadNetworkContext {}
unsafe impl ::core::marker::Sync for ThreadNetworkContext {}
