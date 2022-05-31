#[repr(transparent)]
pub struct AppRecordingManager(::windows_core::IUnknown);
impl AppRecordingManager {
    pub fn GetStatus(&self) -> ::windows_core::Result<AppRecordingStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppRecordingStatus>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn StartRecordingToFileAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::StorageFile>>(&self, file: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<AppRecordingResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartRecordingToFileAsync)(::windows_core::Interface::as_raw(this), file.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<AppRecordingResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn RecordTimeSpanToFileAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>, Param2: ::windows_core::IntoParam<'a, ::winrt_storage::StorageFile>>(&self, starttime: Param0, duration: Param1, file: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<AppRecordingResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RecordTimeSpanToFileAsync)(::windows_core::Interface::as_raw(this), starttime.into_param().abi(), duration.into_param().abi(), file.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<AppRecordingResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SupportedScreenshotMediaEncodingSubtypes(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedScreenshotMediaEncodingSubtypes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage"))]
    pub fn SaveScreenshotToFilesAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::StorageFolder>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, folder: Param0, filenameprefix: Param1, option: AppRecordingSaveScreenshotOption, requestedformats: Param3) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<AppRecordingSaveScreenshotResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SaveScreenshotToFilesAsync)(::windows_core::Interface::as_raw(this), folder.into_param().abi(), filenameprefix.into_param().abi(), option, requestedformats.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<AppRecordingSaveScreenshotResult>>(result__)
        }
    }
    pub fn GetDefault() -> ::windows_core::Result<AppRecordingManager> {
        Self::IAppRecordingManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppRecordingManager>(result__)
        })
    }
    pub fn IAppRecordingManagerStatics<R, F: FnOnce(&IAppRecordingManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AppRecordingManager, IAppRecordingManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AppRecordingManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppRecordingManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppRecordingManager {}
impl ::core::fmt::Debug for AppRecordingManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppRecordingManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppRecordingManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.AppRecording.AppRecordingManager;{e7e26076-a044-48e2-a512-3094d574c7cc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppRecordingManager {
    type Vtable = IAppRecordingManager_Vtbl;
    const IID: ::windows_core::GUID = <IAppRecordingManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppRecordingManager {
    const NAME: &'static str = "Windows.Media.AppRecording.AppRecordingManager";
}
impl ::core::convert::From<AppRecordingManager> for ::windows_core::IUnknown {
    fn from(value: AppRecordingManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppRecordingManager> for ::windows_core::IUnknown {
    fn from(value: &AppRecordingManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppRecordingManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppRecordingManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppRecordingManager> for ::windows_core::IInspectable {
    fn from(value: AppRecordingManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppRecordingManager> for ::windows_core::IInspectable {
    fn from(value: &AppRecordingManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppRecordingManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppRecordingManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppRecordingManager {}
unsafe impl ::core::marker::Sync for AppRecordingManager {}
#[repr(transparent)]
pub struct AppRecordingResult(::windows_core::IUnknown);
impl AppRecordingResult {
    pub fn Succeeded(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Succeeded)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
    pub fn Duration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn IsFileTruncated(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsFileTruncated)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for AppRecordingResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppRecordingResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppRecordingResult {}
impl ::core::fmt::Debug for AppRecordingResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppRecordingResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppRecordingResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.AppRecording.AppRecordingResult;{3a900864-c66d-46f9-b2d9-5bc2dad070d7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppRecordingResult {
    type Vtable = IAppRecordingResult_Vtbl;
    const IID: ::windows_core::GUID = <IAppRecordingResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppRecordingResult {
    const NAME: &'static str = "Windows.Media.AppRecording.AppRecordingResult";
}
impl ::core::convert::From<AppRecordingResult> for ::windows_core::IUnknown {
    fn from(value: AppRecordingResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppRecordingResult> for ::windows_core::IUnknown {
    fn from(value: &AppRecordingResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppRecordingResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppRecordingResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppRecordingResult> for ::windows_core::IInspectable {
    fn from(value: AppRecordingResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppRecordingResult> for ::windows_core::IInspectable {
    fn from(value: &AppRecordingResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppRecordingResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppRecordingResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppRecordingResult {}
unsafe impl ::core::marker::Sync for AppRecordingResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppRecordingSaveScreenshotOption(pub i32);
impl AppRecordingSaveScreenshotOption {
    pub const None: Self = Self(0i32);
    pub const HdrContentVisible: Self = Self(1i32);
}
impl ::core::marker::Copy for AppRecordingSaveScreenshotOption {}
impl ::core::clone::Clone for AppRecordingSaveScreenshotOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppRecordingSaveScreenshotOption {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppRecordingSaveScreenshotOption {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppRecordingSaveScreenshotOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppRecordingSaveScreenshotOption").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppRecordingSaveScreenshotOption {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.AppRecording.AppRecordingSaveScreenshotOption;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AppRecordingSaveScreenshotResult(::windows_core::IUnknown);
impl AppRecordingSaveScreenshotResult {
    pub fn Succeeded(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Succeeded)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SavedScreenshotInfos(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<AppRecordingSavedScreenshotInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SavedScreenshotInfos)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<AppRecordingSavedScreenshotInfo>>(result__)
        }
    }
}
impl ::core::clone::Clone for AppRecordingSaveScreenshotResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppRecordingSaveScreenshotResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppRecordingSaveScreenshotResult {}
impl ::core::fmt::Debug for AppRecordingSaveScreenshotResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppRecordingSaveScreenshotResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppRecordingSaveScreenshotResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.AppRecording.AppRecordingSaveScreenshotResult;{9c5b8d0a-0abb-4457-aaee-24f9c12ec778})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppRecordingSaveScreenshotResult {
    type Vtable = IAppRecordingSaveScreenshotResult_Vtbl;
    const IID: ::windows_core::GUID = <IAppRecordingSaveScreenshotResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppRecordingSaveScreenshotResult {
    const NAME: &'static str = "Windows.Media.AppRecording.AppRecordingSaveScreenshotResult";
}
impl ::core::convert::From<AppRecordingSaveScreenshotResult> for ::windows_core::IUnknown {
    fn from(value: AppRecordingSaveScreenshotResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppRecordingSaveScreenshotResult> for ::windows_core::IUnknown {
    fn from(value: &AppRecordingSaveScreenshotResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppRecordingSaveScreenshotResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppRecordingSaveScreenshotResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppRecordingSaveScreenshotResult> for ::windows_core::IInspectable {
    fn from(value: AppRecordingSaveScreenshotResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppRecordingSaveScreenshotResult> for ::windows_core::IInspectable {
    fn from(value: &AppRecordingSaveScreenshotResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppRecordingSaveScreenshotResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppRecordingSaveScreenshotResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppRecordingSaveScreenshotResult {}
unsafe impl ::core::marker::Sync for AppRecordingSaveScreenshotResult {}
#[repr(transparent)]
pub struct AppRecordingSavedScreenshotInfo(::windows_core::IUnknown);
impl AppRecordingSavedScreenshotInfo {
    #[cfg(feature = "winrt-storage")]
    pub fn File(&self) -> ::windows_core::Result<::winrt_storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).File)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::StorageFile>(result__)
        }
    }
    pub fn MediaEncodingSubtype(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).MediaEncodingSubtype)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for AppRecordingSavedScreenshotInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppRecordingSavedScreenshotInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppRecordingSavedScreenshotInfo {}
impl ::core::fmt::Debug for AppRecordingSavedScreenshotInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppRecordingSavedScreenshotInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppRecordingSavedScreenshotInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.AppRecording.AppRecordingSavedScreenshotInfo;{9b642d0a-189a-4d00-bf25-e1bb1249d594})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppRecordingSavedScreenshotInfo {
    type Vtable = IAppRecordingSavedScreenshotInfo_Vtbl;
    const IID: ::windows_core::GUID = <IAppRecordingSavedScreenshotInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppRecordingSavedScreenshotInfo {
    const NAME: &'static str = "Windows.Media.AppRecording.AppRecordingSavedScreenshotInfo";
}
impl ::core::convert::From<AppRecordingSavedScreenshotInfo> for ::windows_core::IUnknown {
    fn from(value: AppRecordingSavedScreenshotInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppRecordingSavedScreenshotInfo> for ::windows_core::IUnknown {
    fn from(value: &AppRecordingSavedScreenshotInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppRecordingSavedScreenshotInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppRecordingSavedScreenshotInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppRecordingSavedScreenshotInfo> for ::windows_core::IInspectable {
    fn from(value: AppRecordingSavedScreenshotInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppRecordingSavedScreenshotInfo> for ::windows_core::IInspectable {
    fn from(value: &AppRecordingSavedScreenshotInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppRecordingSavedScreenshotInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppRecordingSavedScreenshotInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppRecordingSavedScreenshotInfo {}
unsafe impl ::core::marker::Sync for AppRecordingSavedScreenshotInfo {}
#[repr(transparent)]
pub struct AppRecordingStatus(::windows_core::IUnknown);
impl AppRecordingStatus {
    pub fn CanRecord(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanRecord)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CanRecordTimeSpan(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanRecordTimeSpan)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn HistoricalBufferDuration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).HistoricalBufferDuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn Details(&self) -> ::windows_core::Result<AppRecordingStatusDetails> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Details)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppRecordingStatusDetails>(result__)
        }
    }
}
impl ::core::clone::Clone for AppRecordingStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppRecordingStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppRecordingStatus {}
impl ::core::fmt::Debug for AppRecordingStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppRecordingStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppRecordingStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.AppRecording.AppRecordingStatus;{1d0cc82c-bc18-4b8a-a6ef-127efab3b5d9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppRecordingStatus {
    type Vtable = IAppRecordingStatus_Vtbl;
    const IID: ::windows_core::GUID = <IAppRecordingStatus as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppRecordingStatus {
    const NAME: &'static str = "Windows.Media.AppRecording.AppRecordingStatus";
}
impl ::core::convert::From<AppRecordingStatus> for ::windows_core::IUnknown {
    fn from(value: AppRecordingStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppRecordingStatus> for ::windows_core::IUnknown {
    fn from(value: &AppRecordingStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppRecordingStatus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppRecordingStatus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppRecordingStatus> for ::windows_core::IInspectable {
    fn from(value: AppRecordingStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppRecordingStatus> for ::windows_core::IInspectable {
    fn from(value: &AppRecordingStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppRecordingStatus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppRecordingStatus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppRecordingStatus {}
unsafe impl ::core::marker::Sync for AppRecordingStatus {}
#[repr(transparent)]
pub struct AppRecordingStatusDetails(::windows_core::IUnknown);
impl AppRecordingStatusDetails {
    pub fn IsAnyAppBroadcasting(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsAnyAppBroadcasting)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsCaptureResourceUnavailable(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCaptureResourceUnavailable)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsGameStreamInProgress(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsGameStreamInProgress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsTimeSpanRecordingDisabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsTimeSpanRecordingDisabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsGpuConstrained(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsGpuConstrained)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsAppInactive(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsAppInactive)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsBlockedForApp(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsBlockedForApp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsDisabledByUser(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDisabledByUser)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsDisabledBySystem(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDisabledBySystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for AppRecordingStatusDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppRecordingStatusDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppRecordingStatusDetails {}
impl ::core::fmt::Debug for AppRecordingStatusDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppRecordingStatusDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppRecordingStatusDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.AppRecording.AppRecordingStatusDetails;{b538a9b0-14ed-4412-ac45-6d672c9c9949})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppRecordingStatusDetails {
    type Vtable = IAppRecordingStatusDetails_Vtbl;
    const IID: ::windows_core::GUID = <IAppRecordingStatusDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppRecordingStatusDetails {
    const NAME: &'static str = "Windows.Media.AppRecording.AppRecordingStatusDetails";
}
impl ::core::convert::From<AppRecordingStatusDetails> for ::windows_core::IUnknown {
    fn from(value: AppRecordingStatusDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppRecordingStatusDetails> for ::windows_core::IUnknown {
    fn from(value: &AppRecordingStatusDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppRecordingStatusDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppRecordingStatusDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppRecordingStatusDetails> for ::windows_core::IInspectable {
    fn from(value: AppRecordingStatusDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppRecordingStatusDetails> for ::windows_core::IInspectable {
    fn from(value: &AppRecordingStatusDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppRecordingStatusDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppRecordingStatusDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppRecordingStatusDetails {}
unsafe impl ::core::marker::Sync for AppRecordingStatusDetails {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppRecordingManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppRecordingManager {
    type Vtable = IAppRecordingManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe7e26076_a044_48e2_a512_3094d574c7cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppRecordingManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub StartRecordingToFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    StartRecordingToFileAsync: usize,
    #[cfg(feature = "winrt-storage")]
    pub RecordTimeSpanToFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: ::winrt_foundation::DateTime, duration: ::winrt_foundation::TimeSpan, file: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    RecordTimeSpanToFileAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub SupportedScreenshotMediaEncodingSubtypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SupportedScreenshotMediaEncodingSubtypes: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage"))]
    pub SaveScreenshotToFilesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folder: ::windows_core::RawPtr, filenameprefix: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, option: AppRecordingSaveScreenshotOption, requestedformats: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-storage")))]
    SaveScreenshotToFilesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppRecordingManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppRecordingManagerStatics {
    type Vtable = IAppRecordingManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x50e709f7_38ce_4bd3_9db2_e72bbe9de11d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppRecordingManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppRecordingResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppRecordingResult {
    type Vtable = IAppRecordingResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3a900864_c66d_46f9_b2d9_5bc2dad070d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppRecordingResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Succeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub IsFileTruncated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppRecordingSaveScreenshotResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppRecordingSaveScreenshotResult {
    type Vtable = IAppRecordingSaveScreenshotResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9c5b8d0a_0abb_4457_aaee_24f9c12ec778);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppRecordingSaveScreenshotResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Succeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub SavedScreenshotInfos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SavedScreenshotInfos: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppRecordingSavedScreenshotInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppRecordingSavedScreenshotInfo {
    type Vtable = IAppRecordingSavedScreenshotInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9b642d0a_189a_4d00_bf25_e1bb1249d594);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppRecordingSavedScreenshotInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-storage")]
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    File: usize,
    pub MediaEncodingSubtype: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppRecordingStatus(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppRecordingStatus {
    type Vtable = IAppRecordingStatus_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d0cc82c_bc18_4b8a_a6ef_127efab3b5d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppRecordingStatus_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CanRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CanRecordTimeSpan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub HistoricalBufferDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub Details: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppRecordingStatusDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppRecordingStatusDetails {
    type Vtable = IAppRecordingStatusDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb538a9b0_14ed_4412_ac45_6d672c9c9949);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppRecordingStatusDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsAnyAppBroadcasting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsCaptureResourceUnavailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsGameStreamInProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsTimeSpanRecordingDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsGpuConstrained: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsAppInactive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsBlockedForApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsDisabledByUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsDisabledBySystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
