#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
pub mod AppointmentsProvider;
#[cfg(feature = "ApplicationModel_Appointments_DataProvider")]
pub mod DataProvider;
pub type Appointment = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
pub struct AppointmentBusyStatus(pub i32);
impl AppointmentBusyStatus {
    pub const Busy: Self = Self(0i32);
    pub const Tentative: Self = Self(1i32);
    pub const Free: Self = Self(2i32);
    pub const OutOfOffice: Self = Self(3i32);
    pub const WorkingElsewhere: Self = Self(4i32);
}
impl ::core::marker::Copy for AppointmentBusyStatus {}
impl ::core::clone::Clone for AppointmentBusyStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppointmentCalendar = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
pub struct AppointmentCalendarOtherAppReadAccess(pub i32);
impl AppointmentCalendarOtherAppReadAccess {
    pub const SystemOnly: Self = Self(0i32);
    pub const Limited: Self = Self(1i32);
    pub const Full: Self = Self(2i32);
    pub const None: Self = Self(3i32);
}
impl ::core::marker::Copy for AppointmentCalendarOtherAppReadAccess {}
impl ::core::clone::Clone for AppointmentCalendarOtherAppReadAccess {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
pub struct AppointmentCalendarOtherAppWriteAccess(pub i32);
impl AppointmentCalendarOtherAppWriteAccess {
    pub const None: Self = Self(0i32);
    pub const SystemOnly: Self = Self(1i32);
    pub const Limited: Self = Self(2i32);
}
impl ::core::marker::Copy for AppointmentCalendarOtherAppWriteAccess {}
impl ::core::clone::Clone for AppointmentCalendarOtherAppWriteAccess {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppointmentCalendarSyncManager = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
pub struct AppointmentCalendarSyncStatus(pub i32);
impl AppointmentCalendarSyncStatus {
    pub const Idle: Self = Self(0i32);
    pub const Syncing: Self = Self(1i32);
    pub const UpToDate: Self = Self(2i32);
    pub const AuthenticationError: Self = Self(3i32);
    pub const PolicyError: Self = Self(4i32);
    pub const UnknownError: Self = Self(5i32);
    pub const ManualAccountRemovalRequired: Self = Self(6i32);
}
impl ::core::marker::Copy for AppointmentCalendarSyncStatus {}
impl ::core::clone::Clone for AppointmentCalendarSyncStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppointmentConflictResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
pub struct AppointmentConflictType(pub i32);
impl AppointmentConflictType {
    pub const None: Self = Self(0i32);
    pub const Adjacent: Self = Self(1i32);
    pub const Overlap: Self = Self(2i32);
}
impl ::core::marker::Copy for AppointmentConflictType {}
impl ::core::clone::Clone for AppointmentConflictType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
pub struct AppointmentDaysOfWeek(pub u32);
impl AppointmentDaysOfWeek {
    pub const None: Self = Self(0u32);
    pub const Sunday: Self = Self(1u32);
    pub const Monday: Self = Self(2u32);
    pub const Tuesday: Self = Self(4u32);
    pub const Wednesday: Self = Self(8u32);
    pub const Thursday: Self = Self(16u32);
    pub const Friday: Self = Self(32u32);
    pub const Saturday: Self = Self(64u32);
}
impl ::core::marker::Copy for AppointmentDaysOfWeek {}
impl ::core::clone::Clone for AppointmentDaysOfWeek {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
pub struct AppointmentDetailsKind(pub i32);
impl AppointmentDetailsKind {
    pub const PlainText: Self = Self(0i32);
    pub const Html: Self = Self(1i32);
}
impl ::core::marker::Copy for AppointmentDetailsKind {}
impl ::core::clone::Clone for AppointmentDetailsKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppointmentException = *mut ::core::ffi::c_void;
pub type AppointmentInvitee = *mut ::core::ffi::c_void;
pub type AppointmentManagerForUser = *mut ::core::ffi::c_void;
pub type AppointmentOrganizer = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
pub struct AppointmentParticipantResponse(pub i32);
impl AppointmentParticipantResponse {
    pub const None: Self = Self(0i32);
    pub const Tentative: Self = Self(1i32);
    pub const Accepted: Self = Self(2i32);
    pub const Declined: Self = Self(3i32);
    pub const Unknown: Self = Self(4i32);
}
impl ::core::marker::Copy for AppointmentParticipantResponse {}
impl ::core::clone::Clone for AppointmentParticipantResponse {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
pub struct AppointmentParticipantRole(pub i32);
impl AppointmentParticipantRole {
    pub const RequiredAttendee: Self = Self(0i32);
    pub const OptionalAttendee: Self = Self(1i32);
    pub const Resource: Self = Self(2i32);
}
impl ::core::marker::Copy for AppointmentParticipantRole {}
impl ::core::clone::Clone for AppointmentParticipantRole {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppointmentRecurrence = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
pub struct AppointmentRecurrenceUnit(pub i32);
impl AppointmentRecurrenceUnit {
    pub const Daily: Self = Self(0i32);
    pub const Weekly: Self = Self(1i32);
    pub const Monthly: Self = Self(2i32);
    pub const MonthlyOnDay: Self = Self(3i32);
    pub const Yearly: Self = Self(4i32);
    pub const YearlyOnDay: Self = Self(5i32);
}
impl ::core::marker::Copy for AppointmentRecurrenceUnit {}
impl ::core::clone::Clone for AppointmentRecurrenceUnit {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
pub struct AppointmentSensitivity(pub i32);
impl AppointmentSensitivity {
    pub const Public: Self = Self(0i32);
    pub const Private: Self = Self(1i32);
}
impl ::core::marker::Copy for AppointmentSensitivity {}
impl ::core::clone::Clone for AppointmentSensitivity {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppointmentStore = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
pub struct AppointmentStoreAccessType(pub i32);
impl AppointmentStoreAccessType {
    pub const AppCalendarsReadWrite: Self = Self(0i32);
    pub const AllCalendarsReadOnly: Self = Self(1i32);
    pub const AllCalendarsReadWrite: Self = Self(2i32);
}
impl ::core::marker::Copy for AppointmentStoreAccessType {}
impl ::core::clone::Clone for AppointmentStoreAccessType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppointmentStoreChange = *mut ::core::ffi::c_void;
pub type AppointmentStoreChangeReader = *mut ::core::ffi::c_void;
pub type AppointmentStoreChangeTracker = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
pub struct AppointmentStoreChangeType(pub i32);
impl AppointmentStoreChangeType {
    pub const AppointmentCreated: Self = Self(0i32);
    pub const AppointmentModified: Self = Self(1i32);
    pub const AppointmentDeleted: Self = Self(2i32);
    pub const ChangeTrackingLost: Self = Self(3i32);
    pub const CalendarCreated: Self = Self(4i32);
    pub const CalendarModified: Self = Self(5i32);
    pub const CalendarDeleted: Self = Self(6i32);
}
impl ::core::marker::Copy for AppointmentStoreChangeType {}
impl ::core::clone::Clone for AppointmentStoreChangeType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppointmentStoreChangedDeferral = *mut ::core::ffi::c_void;
pub type AppointmentStoreChangedEventArgs = *mut ::core::ffi::c_void;
pub type AppointmentStoreNotificationTriggerDetails = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
pub struct AppointmentSummaryCardView(pub i32);
impl AppointmentSummaryCardView {
    pub const System: Self = Self(0i32);
    pub const App: Self = Self(1i32);
}
impl ::core::marker::Copy for AppointmentSummaryCardView {}
impl ::core::clone::Clone for AppointmentSummaryCardView {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
pub struct AppointmentWeekOfMonth(pub i32);
impl AppointmentWeekOfMonth {
    pub const First: Self = Self(0i32);
    pub const Second: Self = Self(1i32);
    pub const Third: Self = Self(2i32);
    pub const Fourth: Self = Self(3i32);
    pub const Last: Self = Self(4i32);
}
impl ::core::marker::Copy for AppointmentWeekOfMonth {}
impl ::core::clone::Clone for AppointmentWeekOfMonth {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
pub struct FindAppointmentCalendarsOptions(pub u32);
impl FindAppointmentCalendarsOptions {
    pub const None: Self = Self(0u32);
    pub const IncludeHidden: Self = Self(1u32);
}
impl ::core::marker::Copy for FindAppointmentCalendarsOptions {}
impl ::core::clone::Clone for FindAppointmentCalendarsOptions {
    fn clone(&self) -> Self {
        *self
    }
}
pub type FindAppointmentsOptions = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAppointment {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartTime: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Foundation")]
    pub SetDuration: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDuration: usize,
    pub Location: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetLocation: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Subject: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSubject: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Details: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDetails: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Reminder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Reminder: usize,
    #[cfg(feature = "Foundation")]
    pub SetReminder: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetReminder: usize,
    pub Organizer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetOrganizer: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Invitees: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Invitees: usize,
    pub Recurrence: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRecurrence: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BusyStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppointmentBusyStatus) -> ::windows_sys::core::HRESULT,
    pub SetBusyStatus: unsafe extern "system" fn(this: *mut *mut Self, value: AppointmentBusyStatus) -> ::windows_sys::core::HRESULT,
    pub AllDay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllDay: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Sensitivity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppointmentSensitivity) -> ::windows_sys::core::HRESULT,
    pub SetSensitivity: unsafe extern "system" fn(this: *mut *mut Self, value: AppointmentSensitivity) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub SetUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUri: usize,
}
impl ::windows_sys::core::Interface for IAppointment {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3707776815, data2: 11229, data3: 16502, data4: [144, 163, 34, 194, 117, 49, 41, 101] };
}
#[repr(C)]
pub struct IAppointment2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LocalId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CalendarId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RoamingId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetRoamingId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub OriginalStartTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OriginalStartTime: usize,
    pub IsResponseRequested: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsResponseRequested: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AllowNewTimeProposal: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowNewTimeProposal: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub OnlineMeetingLink: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetOnlineMeetingLink: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReplyTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReplyTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetReplyTime: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetReplyTime: usize,
    pub UserResponse: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppointmentParticipantResponse) -> ::windows_sys::core::HRESULT,
    pub SetUserResponse: unsafe extern "system" fn(this: *mut *mut Self, value: AppointmentParticipantResponse) -> ::windows_sys::core::HRESULT,
    pub HasInvitees: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsCanceledMeeting: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsCanceledMeeting: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsOrganizedByUser: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsOrganizedByUser: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppointment2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1585813564, data2: 21519, data3: 13394, data4: [155, 92, 13, 215, 173, 76, 101, 162] };
}
#[repr(C)]
pub struct IAppointment3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ChangeNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub RemoteChangeNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub SetRemoteChangeNumber: unsafe extern "system" fn(this: *mut *mut Self, value: u64) -> ::windows_sys::core::HRESULT,
    pub DetailsKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppointmentDetailsKind) -> ::windows_sys::core::HRESULT,
    pub SetDetailsKind: unsafe extern "system" fn(this: *mut *mut Self, value: AppointmentDetailsKind) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppointment3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3217835433, data2: 35169, data3: 18833, data4: [147, 75, 196, 135, 104, 229, 169, 108] };
}
#[repr(C)]
pub struct IAppointmentCalendar {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI")]
    pub DisplayColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    DisplayColor: usize,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LocalId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsHidden: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub OtherAppReadAccess: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppointmentCalendarOtherAppReadAccess) -> ::windows_sys::core::HRESULT,
    pub SetOtherAppReadAccess: unsafe extern "system" fn(this: *mut *mut Self, value: AppointmentCalendarOtherAppReadAccess) -> ::windows_sys::core::HRESULT,
    pub OtherAppWriteAccess: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppointmentCalendarOtherAppWriteAccess) -> ::windows_sys::core::HRESULT,
    pub SetOtherAppWriteAccess: unsafe extern "system" fn(this: *mut *mut Self, value: AppointmentCalendarOtherAppWriteAccess) -> ::windows_sys::core::HRESULT,
    pub SourceDisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SummaryCardView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppointmentSummaryCardView) -> ::windows_sys::core::HRESULT,
    pub SetSummaryCardView: unsafe extern "system" fn(this: *mut *mut Self, value: AppointmentSummaryCardView) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAppointmentsAsync: unsafe extern "system" fn(this: *mut *mut Self, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAppointmentsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAppointmentsAsyncWithOptions: unsafe extern "system" fn(this: *mut *mut Self, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAppointmentsAsyncWithOptions: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindExceptionsFromMasterAsync: unsafe extern "system" fn(this: *mut *mut Self, masterlocalid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindExceptionsFromMasterAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllInstancesAsync: unsafe extern "system" fn(this: *mut *mut Self, masterlocalid: ::windows_sys::core::HSTRING, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllInstancesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllInstancesAsyncWithOptions: unsafe extern "system" fn(this: *mut *mut Self, masterlocalid: ::windows_sys::core::HSTRING, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan, poptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllInstancesAsyncWithOptions: usize,
    #[cfg(feature = "Foundation")]
    pub GetAppointmentAsync: unsafe extern "system" fn(this: *mut *mut Self, localid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAppointmentAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAppointmentInstanceAsync: unsafe extern "system" fn(this: *mut *mut Self, localid: ::windows_sys::core::HSTRING, instancestarttime: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAppointmentInstanceAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindUnexpandedAppointmentsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindUnexpandedAppointmentsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindUnexpandedAppointmentsAsyncWithOptions: unsafe extern "system" fn(this: *mut *mut Self, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindUnexpandedAppointmentsAsyncWithOptions: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SaveAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAppointmentAsync: unsafe extern "system" fn(this: *mut *mut Self, localid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAppointmentAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAppointmentInstanceAsync: unsafe extern "system" fn(this: *mut *mut Self, localid: ::windows_sys::core::HSTRING, instancestarttime: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAppointmentInstanceAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SaveAppointmentAsync: unsafe extern "system" fn(this: *mut *mut Self, pappointment: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveAppointmentAsync: usize,
}
impl ::windows_sys::core::Interface for IAppointmentCalendar {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1383301533, data2: 33593, data3: 15695, data4: [160, 47, 100, 8, 68, 82, 187, 93] };
}
#[repr(C)]
pub struct IAppointmentCalendar2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SyncManager: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoteId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI")]
    pub SetDisplayColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetDisplayColor: usize,
    pub SetIsHidden: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub UserDataAccountId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CanCreateOrUpdateAppointments: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCanCreateOrUpdateAppointments: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CanCancelMeetings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCanCancelMeetings: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CanForwardMeetings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCanForwardMeetings: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CanProposeNewTimeForMeetings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCanProposeNewTimeForMeetings: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CanUpdateMeetingResponses: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCanUpdateMeetingResponses: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CanNotifyInvitees: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCanNotifyInvitees: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub MustNofityInvitees: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetMustNofityInvitees: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryCreateOrUpdateAppointmentAsync: unsafe extern "system" fn(this: *mut *mut Self, appointment: *mut ::core::ffi::c_void, notifyinvitees: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryCreateOrUpdateAppointmentAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryCancelMeetingAsync: unsafe extern "system" fn(this: *mut *mut Self, meeting: *mut ::core::ffi::c_void, subject: ::windows_sys::core::HSTRING, comment: ::windows_sys::core::HSTRING, notifyinvitees: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryCancelMeetingAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TryForwardMeetingAsync: unsafe extern "system" fn(this: *mut *mut Self, meeting: *mut ::core::ffi::c_void, invitees: *mut ::core::ffi::c_void, subject: ::windows_sys::core::HSTRING, forwardheader: ::windows_sys::core::HSTRING, comment: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TryForwardMeetingAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryProposeNewTimeForMeetingAsync: unsafe extern "system" fn(this: *mut *mut Self, meeting: *mut ::core::ffi::c_void, newstarttime: super::super::Foundation::DateTime, newduration: super::super::Foundation::TimeSpan, subject: ::windows_sys::core::HSTRING, comment: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryProposeNewTimeForMeetingAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryUpdateMeetingResponseAsync: unsafe extern "system" fn(this: *mut *mut Self, meeting: *mut ::core::ffi::c_void, response: AppointmentParticipantResponse, subject: ::windows_sys::core::HSTRING, comment: ::windows_sys::core::HSTRING, sendupdate: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryUpdateMeetingResponseAsync: usize,
}
impl ::windows_sys::core::Interface for IAppointmentCalendar2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 417850402, data2: 9319, data3: 19996, data4: [164, 89, 216, 162, 147, 3, 208, 146] };
}
#[repr(C)]
pub struct IAppointmentCalendar3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RegisterSyncManagerAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RegisterSyncManagerAsync: usize,
}
impl ::windows_sys::core::Interface for IAppointmentCalendar3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3944993323, data2: 42629, data3: 17070, data4: [132, 149, 179, 17, 154, 219, 65, 103] };
}
#[repr(C)]
pub struct IAppointmentCalendarSyncManager {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppointmentCalendarSyncStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LastSuccessfulSyncTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastSuccessfulSyncTime: usize,
    #[cfg(feature = "Foundation")]
    pub LastAttemptedSyncTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastAttemptedSyncTime: usize,
    #[cfg(feature = "Foundation")]
    pub SyncAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SyncAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SyncStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SyncStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSyncStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSyncStatusChanged: usize,
}
impl ::windows_sys::core::Interface for IAppointmentCalendarSyncManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 723628960, data2: 19199, data3: 17298, data4: [188, 95, 86, 69, 255, 207, 251, 23] };
}
#[repr(C)]
pub struct IAppointmentCalendarSyncManager2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetStatus: unsafe extern "system" fn(this: *mut *mut Self, value: AppointmentCalendarSyncStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetLastSuccessfulSyncTime: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLastSuccessfulSyncTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetLastAttemptedSyncTime: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLastAttemptedSyncTime: usize,
}
impl ::windows_sys::core::Interface for IAppointmentCalendarSyncManager2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1685399725, data2: 3369, data3: 19580, data4: [170, 167, 191, 153, 104, 5, 83, 124] };
}
#[repr(C)]
pub struct IAppointmentConflictResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppointmentConflictType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Date: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Date: usize,
}
impl ::windows_sys::core::Interface for IAppointmentConflictResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3587043518, data2: 12079, data3: 15229, data4: [175, 10, 167, 226, 15, 58, 70, 227] };
}
#[repr(C)]
pub struct IAppointmentException {
    pub base__: ::windows_sys::core::IInspectable,
    pub Appointment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ExceptionProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExceptionProperties: usize,
    pub IsDeleted: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppointmentException {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2718394215, data2: 5878, data3: 19406, data4: [159, 90, 134, 0, 184, 1, 159, 203] };
}
#[repr(C)]
pub struct IAppointmentInvitee {
    pub base__: ::windows_sys::core::IInspectable,
    pub Role: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppointmentParticipantRole) -> ::windows_sys::core::HRESULT,
    pub SetRole: unsafe extern "system" fn(this: *mut *mut Self, value: AppointmentParticipantRole) -> ::windows_sys::core::HRESULT,
    pub Response: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppointmentParticipantResponse) -> ::windows_sys::core::HRESULT,
    pub SetResponse: unsafe extern "system" fn(this: *mut *mut Self, value: AppointmentParticipantResponse) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppointmentInvitee {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 331286422, data2: 38978, data3: 18779, data4: [176, 231, 239, 143, 121, 192, 112, 29] };
}
#[repr(C)]
pub struct IAppointmentManagerForUser {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ShowAddAppointmentAsync: unsafe extern "system" fn(this: *mut *mut Self, appointment: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAddAppointmentAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowAddAppointmentWithPlacementAsync: unsafe extern "system" fn(this: *mut *mut Self, appointment: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowAddAppointmentWithPlacementAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowReplaceAppointmentAsync: unsafe extern "system" fn(this: *mut *mut Self, appointmentid: ::windows_sys::core::HSTRING, appointment: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowReplaceAppointmentAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowReplaceAppointmentWithPlacementAsync: unsafe extern "system" fn(this: *mut *mut Self, appointmentid: ::windows_sys::core::HSTRING, appointment: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowReplaceAppointmentWithPlacementAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowReplaceAppointmentWithPlacementAndDateAsync: unsafe extern "system" fn(this: *mut *mut Self, appointmentid: ::windows_sys::core::HSTRING, appointment: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowReplaceAppointmentWithPlacementAndDateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowRemoveAppointmentAsync: unsafe extern "system" fn(this: *mut *mut Self, appointmentid: ::windows_sys::core::HSTRING, selection: super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowRemoveAppointmentAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowRemoveAppointmentWithPlacementAsync: unsafe extern "system" fn(this: *mut *mut Self, appointmentid: ::windows_sys::core::HSTRING, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowRemoveAppointmentWithPlacementAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowRemoveAppointmentWithPlacementAndDateAsync: unsafe extern "system" fn(this: *mut *mut Self, appointmentid: ::windows_sys::core::HSTRING, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowRemoveAppointmentWithPlacementAndDateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowTimeFrameAsync: unsafe extern "system" fn(this: *mut *mut Self, timetoshow: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowTimeFrameAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAppointmentDetailsAsync: unsafe extern "system" fn(this: *mut *mut Self, appointmentid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAppointmentDetailsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAppointmentDetailsWithDateAsync: unsafe extern "system" fn(this: *mut *mut Self, appointmentid: ::windows_sys::core::HSTRING, instancestartdate: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAppointmentDetailsWithDateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowEditNewAppointmentAsync: unsafe extern "system" fn(this: *mut *mut Self, appointment: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowEditNewAppointmentAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut *mut Self, options: AppointmentStoreAccessType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
impl ::windows_sys::core::Interface for IAppointmentManagerForUser {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1881543715, data2: 29644, data3: 18016, data4: [179, 24, 176, 19, 101, 48, 42, 3] };
}
#[repr(C)]
pub struct IAppointmentManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ShowAddAppointmentAsync: unsafe extern "system" fn(this: *mut *mut Self, appointment: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAddAppointmentAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowAddAppointmentWithPlacementAsync: unsafe extern "system" fn(this: *mut *mut Self, appointment: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowAddAppointmentWithPlacementAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowReplaceAppointmentAsync: unsafe extern "system" fn(this: *mut *mut Self, appointmentid: ::windows_sys::core::HSTRING, appointment: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowReplaceAppointmentAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowReplaceAppointmentWithPlacementAsync: unsafe extern "system" fn(this: *mut *mut Self, appointmentid: ::windows_sys::core::HSTRING, appointment: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowReplaceAppointmentWithPlacementAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowReplaceAppointmentWithPlacementAndDateAsync: unsafe extern "system" fn(this: *mut *mut Self, appointmentid: ::windows_sys::core::HSTRING, appointment: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowReplaceAppointmentWithPlacementAndDateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowRemoveAppointmentAsync: unsafe extern "system" fn(this: *mut *mut Self, appointmentid: ::windows_sys::core::HSTRING, selection: super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowRemoveAppointmentAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowRemoveAppointmentWithPlacementAsync: unsafe extern "system" fn(this: *mut *mut Self, appointmentid: ::windows_sys::core::HSTRING, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowRemoveAppointmentWithPlacementAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowRemoveAppointmentWithPlacementAndDateAsync: unsafe extern "system" fn(this: *mut *mut Self, appointmentid: ::windows_sys::core::HSTRING, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowRemoveAppointmentWithPlacementAndDateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowTimeFrameAsync: unsafe extern "system" fn(this: *mut *mut Self, timetoshow: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowTimeFrameAsync: usize,
}
impl ::windows_sys::core::Interface for IAppointmentManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 976288257, data2: 23616, data3: 18845, data4: [179, 63, 164, 48, 80, 247, 79, 196] };
}
#[repr(C)]
pub struct IAppointmentManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ShowAppointmentDetailsAsync: unsafe extern "system" fn(this: *mut *mut Self, appointmentid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAppointmentDetailsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAppointmentDetailsWithDateAsync: unsafe extern "system" fn(this: *mut *mut Self, appointmentid: ::windows_sys::core::HSTRING, instancestartdate: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAppointmentDetailsWithDateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowEditNewAppointmentAsync: unsafe extern "system" fn(this: *mut *mut Self, appointment: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowEditNewAppointmentAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut *mut Self, options: AppointmentStoreAccessType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
}
impl ::windows_sys::core::Interface for IAppointmentManagerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 176289293, data2: 53327, data3: 16436, data4: [175, 114, 163, 101, 115, 180, 95, 240] };
}
#[repr(C)]
pub struct IAppointmentManagerStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
impl ::windows_sys::core::Interface for IAppointmentManagerStatics3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 798679196, data2: 45900, data3: 19911, data4: [163, 93, 202, 253, 136, 174, 62, 198] };
}
#[repr(C)]
pub struct IAppointmentParticipant {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Address: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetAddress: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppointmentParticipant {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1633560834, data2: 38680, data3: 18043, data4: [131, 251, 178, 147, 161, 145, 33, 222] };
}
#[repr(C)]
pub struct IAppointmentPropertiesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Subject: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Location: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub StartTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Reminder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub BusyStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Sensitivity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub OriginalStartTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsResponseRequested: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AllowNewTimeProposal: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AllDay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Details: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub OnlineMeetingLink: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ReplyTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Organizer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub UserResponse: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub HasInvitees: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsCanceledMeeting: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsOrganizedByUser: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Recurrence: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Invitees: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub DefaultProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DefaultProperties: usize,
}
impl ::windows_sys::core::Interface for IAppointmentPropertiesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 622075881, data2: 26798, data3: 15022, data4: [133, 95, 188, 68, 65, 202, 162, 52] };
}
#[repr(C)]
pub struct IAppointmentPropertiesStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ChangeNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RemoteChangeNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DetailsKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppointmentPropertiesStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757851467, data2: 45079, data3: 17885, data4: [138, 245, 209, 99, 209, 8, 1, 187] };
}
#[repr(C)]
pub struct IAppointmentRecurrence {
    pub base__: ::windows_sys::core::IInspectable,
    pub Unit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppointmentRecurrenceUnit) -> ::windows_sys::core::HRESULT,
    pub SetUnit: unsafe extern "system" fn(this: *mut *mut Self, value: AppointmentRecurrenceUnit) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Occurrences: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Occurrences: usize,
    #[cfg(feature = "Foundation")]
    pub SetOccurrences: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetOccurrences: usize,
    #[cfg(feature = "Foundation")]
    pub Until: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Until: usize,
    #[cfg(feature = "Foundation")]
    pub SetUntil: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUntil: usize,
    pub Interval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetInterval: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub DaysOfWeek: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppointmentDaysOfWeek) -> ::windows_sys::core::HRESULT,
    pub SetDaysOfWeek: unsafe extern "system" fn(this: *mut *mut Self, value: AppointmentDaysOfWeek) -> ::windows_sys::core::HRESULT,
    pub WeekOfMonth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppointmentWeekOfMonth) -> ::windows_sys::core::HRESULT,
    pub SetWeekOfMonth: unsafe extern "system" fn(this: *mut *mut Self, value: AppointmentWeekOfMonth) -> ::windows_sys::core::HRESULT,
    pub Month: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetMonth: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub Day: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetDay: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppointmentRecurrence {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3631955587, data2: 5542, data3: 18555, data4: [185, 89, 12, 54, 30, 96, 233, 84] };
}
#[repr(C)]
pub struct IAppointmentRecurrence2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub RecurrenceType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RecurrenceType) -> ::windows_sys::core::HRESULT,
    pub TimeZone: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTimeZone: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppointmentRecurrence2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1039377120, data2: 1447, data3: 20304, data4: [159, 134, 176, 63, 148, 54, 37, 77] };
}
#[repr(C)]
pub struct IAppointmentRecurrence3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CalendarIdentifier: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppointmentRecurrence3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2315228889, data2: 55885, data3: 18967, data4: [141, 210, 28, 235, 194, 181, 255, 157] };
}
#[repr(C)]
pub struct IAppointmentStore {
    pub base__: ::windows_sys::core::IInspectable,
    pub ChangeTracker: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateAppointmentCalendarAsync: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAppointmentCalendarAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAppointmentCalendarAsync: unsafe extern "system" fn(this: *mut *mut Self, calendarid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAppointmentCalendarAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAppointmentAsync: unsafe extern "system" fn(this: *mut *mut Self, localid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAppointmentAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAppointmentInstanceAsync: unsafe extern "system" fn(this: *mut *mut Self, localid: ::windows_sys::core::HSTRING, instancestarttime: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAppointmentInstanceAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAppointmentCalendarsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAppointmentCalendarsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAppointmentCalendarsAsyncWithOptions: unsafe extern "system" fn(this: *mut *mut Self, options: FindAppointmentCalendarsOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAppointmentCalendarsAsyncWithOptions: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAppointmentsAsync: unsafe extern "system" fn(this: *mut *mut Self, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAppointmentsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAppointmentsAsyncWithOptions: unsafe extern "system" fn(this: *mut *mut Self, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAppointmentsAsyncWithOptions: usize,
    #[cfg(feature = "Foundation")]
    pub FindConflictAsync: unsafe extern "system" fn(this: *mut *mut Self, appointment: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FindConflictAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FindConflictAsyncWithInstanceStart: unsafe extern "system" fn(this: *mut *mut Self, appointment: *mut ::core::ffi::c_void, instancestarttime: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FindConflictAsyncWithInstanceStart: usize,
    #[cfg(feature = "Foundation")]
    pub MoveAppointmentAsync: unsafe extern "system" fn(this: *mut *mut Self, appointment: *mut ::core::ffi::c_void, destinationcalendar: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MoveAppointmentAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAddAppointmentAsync: unsafe extern "system" fn(this: *mut *mut Self, appointment: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAddAppointmentAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowReplaceAppointmentAsync: unsafe extern "system" fn(this: *mut *mut Self, localid: ::windows_sys::core::HSTRING, appointment: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowReplaceAppointmentAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowReplaceAppointmentWithPlacementAndDateAsync: unsafe extern "system" fn(this: *mut *mut Self, localid: ::windows_sys::core::HSTRING, appointment: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowReplaceAppointmentWithPlacementAndDateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowRemoveAppointmentAsync: unsafe extern "system" fn(this: *mut *mut Self, localid: ::windows_sys::core::HSTRING, selection: super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowRemoveAppointmentAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowRemoveAppointmentWithPlacementAndDateAsync: unsafe extern "system" fn(this: *mut *mut Self, localid: ::windows_sys::core::HSTRING, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowRemoveAppointmentWithPlacementAndDateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAppointmentDetailsAsync: unsafe extern "system" fn(this: *mut *mut Self, localid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAppointmentDetailsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAppointmentDetailsWithDateAsync: unsafe extern "system" fn(this: *mut *mut Self, localid: ::windows_sys::core::HSTRING, instancestartdate: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAppointmentDetailsWithDateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowEditNewAppointmentAsync: unsafe extern "system" fn(this: *mut *mut Self, appointment: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowEditNewAppointmentAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindLocalIdsFromRoamingIdAsync: unsafe extern "system" fn(this: *mut *mut Self, roamingid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindLocalIdsFromRoamingIdAsync: usize,
}
impl ::windows_sys::core::Interface for IAppointmentStore {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2757857676, data2: 31303, data3: 19862, data4: [150, 201, 21, 205, 138, 5, 167, 53] };
}
#[repr(C)]
pub struct IAppointmentStore2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub StoreChanged: unsafe extern "system" fn(this: *mut *mut Self, phandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StoreChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStoreChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStoreChanged: usize,
    #[cfg(feature = "Foundation")]
    pub CreateAppointmentCalendarInAccountAsync: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, userdataaccountid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAppointmentCalendarInAccountAsync: usize,
}
impl ::windows_sys::core::Interface for IAppointmentStore2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 633637920, data2: 7233, data3: 16975, data4: [128, 132, 103, 193, 207, 224, 168, 84] };
}
#[repr(C)]
pub struct IAppointmentStore3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetChangeTracker: unsafe extern "system" fn(this: *mut *mut Self, identity: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppointmentStore3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1112642571, data2: 45176, data3: 18186, data4: [154, 64, 194, 224, 23, 97, 247, 47] };
}
#[repr(C)]
pub struct IAppointmentStoreChange {
    pub base__: ::windows_sys::core::IInspectable,
    pub Appointment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ChangeType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppointmentStoreChangeType) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppointmentStoreChange {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2779177013, data2: 2611, data3: 13908, data4: [132, 99, 181, 67, 233, 12, 59, 121] };
}
#[repr(C)]
pub struct IAppointmentStoreChange2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AppointmentCalendar: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppointmentStoreChange2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3011317198, data2: 21009, data3: 17410, data4: [166, 8, 169, 111, 231, 11, 142, 226] };
}
#[repr(C)]
pub struct IAppointmentStoreChangeReader {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchAsync: usize,
    pub AcceptChanges: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub AcceptChangesThrough: unsafe extern "system" fn(this: *mut *mut Self, lastchangetoaccept: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppointmentStoreChangeReader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2334394865, data2: 26099, data3: 17056, data4: [150, 29, 76, 32, 155, 243, 3, 112] };
}
#[repr(C)]
pub struct IAppointmentStoreChangeTracker {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetChangeReader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Enable: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppointmentStoreChangeTracker {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 455472305, data2: 36558, data3: 20247, data4: [147, 200, 230, 65, 36, 88, 253, 92] };
}
#[repr(C)]
pub struct IAppointmentStoreChangeTracker2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsTracking: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppointmentStoreChangeTracker2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3060444997, data2: 38210, data3: 19703, data4: [133, 80, 235, 55, 14, 12, 8, 211] };
}
#[repr(C)]
pub struct IAppointmentStoreChangedDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppointmentStoreChangedDeferral {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1287135270, data2: 65243, data3: 19395, data4: [150, 98, 149, 169, 190, 253, 244, 223] };
}
#[repr(C)]
pub struct IAppointmentStoreChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppointmentStoreChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 579205305, data2: 1937, data3: 16766, data4: [191, 234, 204, 109, 65, 99, 108, 140] };
}
#[repr(C)]
pub struct IAppointmentStoreNotificationTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IAppointmentStoreNotificationTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2603862801, data2: 49921, data3: 16926, data4: [175, 239, 4, 126, 207, 167, 106, 219] };
}
#[repr(C)]
pub struct IFindAppointmentsOptions {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub CalendarIds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CalendarIds: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FetchProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FetchProperties: usize,
    pub IncludeHidden: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIncludeHidden: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub MaxCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetMaxCount: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFindAppointmentsOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1442307157, data2: 39234, data3: 12422, data4: [130, 181, 44, 178, 159, 100, 213, 245] };
}
#[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
#[repr(transparent)]
pub struct RecurrenceType(pub i32);
impl RecurrenceType {
    pub const Master: Self = Self(0i32);
    pub const Instance: Self = Self(1i32);
    pub const ExceptionInstance: Self = Self(2i32);
}
impl ::core::marker::Copy for RecurrenceType {}
impl ::core::clone::Clone for RecurrenceType {
    fn clone(&self) -> Self {
        *self
    }
}
