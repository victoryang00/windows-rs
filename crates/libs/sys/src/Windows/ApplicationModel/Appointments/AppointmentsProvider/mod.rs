pub type AddAppointmentOperation = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAddAppointmentOperation {
    pub base__: ::windows_sys::core::IInspectable,
    pub AppointmentInformation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SourcePackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut *mut Self, itemid: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ReportCanceled: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ReportError: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DismissUI: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAddAppointmentOperation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3964312307, data2: 25101, data3: 19561, data4: [173, 215, 151, 148, 233, 24, 8, 31] };
}
#[repr(C)]
pub struct IAppointmentsProviderLaunchActionVerbsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub AddAppointment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ReplaceAppointment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RemoveAppointment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ShowTimeFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppointmentsProviderLaunchActionVerbsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 920369704, data2: 40494, data3: 18886, data4: [142, 247, 58, 183, 165, 220, 200, 184] };
}
#[repr(C)]
pub struct IAppointmentsProviderLaunchActionVerbsStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ShowAppointmentDetails: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppointmentsProviderLaunchActionVerbsStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4019210660, data2: 44833, data3: 18236, data4: [136, 220, 118, 205, 137, 246, 12, 165] };
}
#[repr(C)]
pub struct IRemoveAppointmentOperation {
    pub base__: ::windows_sys::core::IInspectable,
    pub AppointmentId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub InstanceStartDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InstanceStartDate: usize,
    pub SourcePackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ReportCanceled: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ReportError: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DismissUI: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoveAppointmentOperation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 146172602, data2: 65075, data3: 18125, data4: [165, 12, 168, 255, 179, 38, 5, 55] };
}
#[repr(C)]
pub struct IReplaceAppointmentOperation {
    pub base__: ::windows_sys::core::IInspectable,
    pub AppointmentId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AppointmentInformation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub InstanceStartDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InstanceStartDate: usize,
    pub SourcePackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut *mut Self, itemid: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ReportCanceled: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ReportError: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DismissUI: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IReplaceAppointmentOperation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4103093659, data2: 40545, data3: 19938, data4: [167, 50, 38, 135, 192, 125, 29, 232] };
}
pub type RemoveAppointmentOperation = *mut ::core::ffi::c_void;
pub type ReplaceAppointmentOperation = *mut ::core::ffi::c_void;
