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
#[repr(C)]
pub struct IAppointment3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ChangeNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub RemoteChangeNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub SetRemoteChangeNumber: unsafe extern "system" fn(this: *mut *mut Self, value: u64) -> ::windows_sys::core::HRESULT,
    pub DetailsKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppointmentDetailsKind) -> ::windows_sys::core::HRESULT,
    pub SetDetailsKind: unsafe extern "system" fn(this: *mut *mut Self, value: AppointmentDetailsKind) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IAppointmentCalendar3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RegisterSyncManagerAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RegisterSyncManagerAsync: usize,
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
#[repr(C)]
pub struct IAppointmentConflictResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppointmentConflictType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Date: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Date: usize,
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
#[repr(C)]
pub struct IAppointmentInvitee {
    pub base__: ::windows_sys::core::IInspectable,
    pub Role: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppointmentParticipantRole) -> ::windows_sys::core::HRESULT,
    pub SetRole: unsafe extern "system" fn(this: *mut *mut Self, value: AppointmentParticipantRole) -> ::windows_sys::core::HRESULT,
    pub Response: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppointmentParticipantResponse) -> ::windows_sys::core::HRESULT,
    pub SetResponse: unsafe extern "system" fn(this: *mut *mut Self, value: AppointmentParticipantResponse) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IAppointmentManagerStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
#[repr(C)]
pub struct IAppointmentParticipant {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Address: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetAddress: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IAppointmentPropertiesStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ChangeNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RemoteChangeNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DetailsKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IAppointmentRecurrence2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub RecurrenceType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RecurrenceType) -> ::windows_sys::core::HRESULT,
    pub TimeZone: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTimeZone: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppointmentRecurrence3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CalendarIdentifier: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IAppointmentStore3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetChangeTracker: unsafe extern "system" fn(this: *mut *mut Self, identity: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppointmentStoreChange {
    pub base__: ::windows_sys::core::IInspectable,
    pub Appointment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ChangeType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppointmentStoreChangeType) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppointmentStoreChange2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AppointmentCalendar: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IAppointmentStoreChangeTracker {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetChangeReader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Enable: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppointmentStoreChangeTracker2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsTracking: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppointmentStoreChangedDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppointmentStoreChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppointmentStoreNotificationTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
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
