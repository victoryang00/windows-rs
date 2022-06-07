pub type HidBooleanControl = *mut ::core::ffi::c_void;
pub type HidBooleanControlDescription = *mut ::core::ffi::c_void;
pub type HidCollection = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
#[repr(transparent)]
pub struct HidCollectionType(pub i32);
impl HidCollectionType {
    pub const Physical: Self = Self(0i32);
    pub const Application: Self = Self(1i32);
    pub const Logical: Self = Self(2i32);
    pub const Report: Self = Self(3i32);
    pub const NamedArray: Self = Self(4i32);
    pub const UsageSwitch: Self = Self(5i32);
    pub const UsageModifier: Self = Self(6i32);
    pub const Other: Self = Self(7i32);
}
impl ::core::marker::Copy for HidCollectionType {}
impl ::core::clone::Clone for HidCollectionType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type HidDevice = *mut ::core::ffi::c_void;
pub type HidFeatureReport = *mut ::core::ffi::c_void;
pub type HidInputReport = *mut ::core::ffi::c_void;
pub type HidInputReportReceivedEventArgs = *mut ::core::ffi::c_void;
pub type HidNumericControl = *mut ::core::ffi::c_void;
pub type HidNumericControlDescription = *mut ::core::ffi::c_void;
pub type HidOutputReport = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
#[repr(transparent)]
pub struct HidReportType(pub i32);
impl HidReportType {
    pub const Input: Self = Self(0i32);
    pub const Output: Self = Self(1i32);
    pub const Feature: Self = Self(2i32);
}
impl ::core::marker::Copy for HidReportType {}
impl ::core::clone::Clone for HidReportType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IHidBooleanControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub UsagePage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub UsageId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub IsActive: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsActive: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ControlDescription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHidBooleanControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1380840586, data2: 13973, data3: 16524, data4: [187, 162, 226, 235, 90, 191, 188, 32] };
}
#[repr(C)]
pub struct IHidBooleanControlDescription {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ReportId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub ReportType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HidReportType) -> ::windows_sys::core::HRESULT,
    pub UsagePage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub UsageId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ParentCollections: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ParentCollections: usize,
}
impl ::windows_sys::core::Interface for IHidBooleanControlDescription {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1637279043, data2: 10712, data3: 18986, data4: [134, 131, 132, 158, 32, 123, 190, 49] };
}
#[repr(C)]
pub struct IHidBooleanControlDescription2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsAbsolute: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHidBooleanControlDescription2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3371094762, data2: 35447, data3: 19510, data4: [170, 0, 95, 240, 68, 157, 62, 115] };
}
#[repr(C)]
pub struct IHidCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HidCollectionType) -> ::windows_sys::core::HRESULT,
    pub UsagePage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub UsageId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHidCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1904866723, data2: 13041, data3: 18147, data4: [190, 253, 68, 210, 102, 59, 126, 106] };
}
#[repr(C)]
pub struct IHidDevice {
    pub base__: ::windows_sys::core::IInspectable,
    pub VendorId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub ProductId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub UsagePage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub UsageId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetInputReportAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetInputReportAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetInputReportByIdAsync: unsafe extern "system" fn(this: *mut *mut Self, reportid: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetInputReportByIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFeatureReportAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFeatureReportAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFeatureReportByIdAsync: unsafe extern "system" fn(this: *mut *mut Self, reportid: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFeatureReportByIdAsync: usize,
    pub CreateOutputReport: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateOutputReportById: unsafe extern "system" fn(this: *mut *mut Self, reportid: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFeatureReport: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFeatureReportById: unsafe extern "system" fn(this: *mut *mut Self, reportid: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SendOutputReportAsync: unsafe extern "system" fn(this: *mut *mut Self, outputreport: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendOutputReportAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SendFeatureReportAsync: unsafe extern "system" fn(this: *mut *mut Self, featurereport: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendFeatureReportAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetBooleanControlDescriptions: unsafe extern "system" fn(this: *mut *mut Self, reporttype: HidReportType, usagepage: u16, usageid: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetBooleanControlDescriptions: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetNumericControlDescriptions: unsafe extern "system" fn(this: *mut *mut Self, reporttype: HidReportType, usagepage: u16, usageid: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetNumericControlDescriptions: usize,
    #[cfg(feature = "Foundation")]
    pub InputReportReceived: unsafe extern "system" fn(this: *mut *mut Self, reporthandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InputReportReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveInputReportReceived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveInputReportReceived: usize,
}
impl ::windows_sys::core::Interface for IHidDevice {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1602884839, data2: 8704, data3: 17198, data4: [149, 218, 208, 155, 135, 213, 116, 168] };
}
#[repr(C)]
pub struct IHidDeviceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, usagepage: u16, usageid: u16, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDeviceSelectorVidPid: unsafe extern "system" fn(this: *mut *mut Self, usagepage: u16, usageid: u16, vendorid: u16, productid: u16, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, accessmode: super::super::Storage::FileAccessMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    FromIdAsync: usize,
}
impl ::windows_sys::core::Interface for IHidDeviceStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2656666084, data2: 38998, data3: 16780, data4: [159, 115, 119, 222, 12, 216, 87, 84] };
}
#[repr(C)]
pub struct IHidFeatureReport {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetData: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetData: usize,
    pub GetBooleanControl: unsafe extern "system" fn(this: *mut *mut Self, usagepage: u16, usageid: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetBooleanControlByDescription: unsafe extern "system" fn(this: *mut *mut Self, controldescription: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetNumericControl: unsafe extern "system" fn(this: *mut *mut Self, usagepage: u16, usageid: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetNumericControlByDescription: unsafe extern "system" fn(this: *mut *mut Self, controldescription: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHidFeatureReport {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2216532857, data2: 23269, data3: 18147, data4: [130, 239, 31, 236, 92, 137, 66, 244] };
}
#[repr(C)]
pub struct IHidInputReport {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ActivatedBooleanControls: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ActivatedBooleanControls: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TransitionedBooleanControls: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TransitionedBooleanControls: usize,
    pub GetBooleanControl: unsafe extern "system" fn(this: *mut *mut Self, usagepage: u16, usageid: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetBooleanControlByDescription: unsafe extern "system" fn(this: *mut *mut Self, controldescription: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetNumericControl: unsafe extern "system" fn(this: *mut *mut Self, usagepage: u16, usageid: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetNumericControlByDescription: unsafe extern "system" fn(this: *mut *mut Self, controldescription: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHidInputReport {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3277655632, data2: 63463, data3: 20109, data4: [178, 62, 202, 187, 229, 107, 144, 233] };
}
#[repr(C)]
pub struct IHidInputReportReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Report: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHidInputReportReceivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1884931531, data2: 22962, data3: 19906, data4: [152, 92, 10, 220, 97, 54, 250, 45] };
}
#[repr(C)]
pub struct IHidNumericControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub IsGrouped: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub UsagePage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub UsageId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i64) -> ::windows_sys::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: i64) -> ::windows_sys::core::HRESULT,
    pub ScaledValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i64) -> ::windows_sys::core::HRESULT,
    pub SetScaledValue: unsafe extern "system" fn(this: *mut *mut Self, value: i64) -> ::windows_sys::core::HRESULT,
    pub ControlDescription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHidNumericControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3817476773, data2: 13735, data3: 19317, data4: [137, 200, 251, 31, 40, 177, 8, 35] };
}
#[repr(C)]
pub struct IHidNumericControlDescription {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ReportId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub ReportType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HidReportType) -> ::windows_sys::core::HRESULT,
    pub ReportSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ReportCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub UsagePage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub UsageId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub LogicalMinimum: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub LogicalMaximum: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub PhysicalMinimum: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub PhysicalMaximum: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub UnitExponent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Unit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub IsAbsolute: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub HasNull: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ParentCollections: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ParentCollections: usize,
}
impl ::windows_sys::core::Interface for IHidNumericControlDescription {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1670209158, data2: 7575, data3: 19573, data4: [146, 127, 95, 245, 139, 160, 94, 50] };
}
#[repr(C)]
pub struct IHidOutputReport {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetData: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetData: usize,
    pub GetBooleanControl: unsafe extern "system" fn(this: *mut *mut Self, usagepage: u16, usageid: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetBooleanControlByDescription: unsafe extern "system" fn(this: *mut *mut Self, controldescription: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetNumericControl: unsafe extern "system" fn(this: *mut *mut Self, usagepage: u16, usageid: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetNumericControlByDescription: unsafe extern "system" fn(this: *mut *mut Self, controldescription: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHidOutputReport {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1657480516, data2: 51350, data3: 17507, data4: [147, 193, 223, 157, 176, 83, 196, 80] };
}
