#[repr(C)]
pub struct IQuickLink {
    pub base__: ::windows_sys::core::IInspectable,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetThumbnail: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetThumbnail: usize,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedDataFormats: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedDataFormats: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedFileTypes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedFileTypes: usize,
}
impl ::windows_sys::core::Interface for IQuickLink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1614693128, data2: 61630, data3: 19164, data4: [172, 201, 139, 39, 171, 156, 245, 86] };
}
#[repr(C)]
pub struct IShareOperation {
    pub base__: ::windows_sys::core::IInspectable,
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub QuickLinkId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RemoveThisQuickLink: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ReportStarted: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ReportDataRetrieved: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ReportSubmittedBackgroundTask: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ReportCompletedWithQuickLink: unsafe extern "system" fn(this: *mut *mut Self, quicklink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ReportError: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IShareOperation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 575060664, data2: 53496, data3: 16833, data4: [168, 42, 65, 55, 219, 101, 4, 251] };
}
#[repr(C)]
pub struct IShareOperation2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub DismissUI: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IShareOperation2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 268146625, data2: 38776, data3: 18953, data4: [142, 91, 203, 94, 72, 45, 5, 85] };
}
#[repr(C)]
pub struct IShareOperation3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections"))]
    pub Contacts: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections")))]
    Contacts: usize,
}
impl ::windows_sys::core::Interface for IShareOperation3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1593226114, data2: 47015, data3: 17777, data4: [162, 166, 153, 74, 3, 73, 136, 178] };
}
pub type QuickLink = *mut ::core::ffi::c_void;
pub type ShareOperation = *mut ::core::ffi::c_void;
