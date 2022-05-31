#[cfg(feature = "AppointmentsProvider")]
pub mod AppointmentsProvider;
#[cfg(feature = "DataProvider")]
pub mod DataProvider;
#[repr(transparent)]
pub struct Appointment(::windows_core::IUnknown);
impl Appointment {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Appointment, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn StartTime(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn SetStartTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStartTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Duration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetDuration<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Location(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Location)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetLocation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLocation)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Subject(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Subject)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetSubject<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSubject)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Details(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Details)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetDetails<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDetails)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Reminder(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Reminder)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn SetReminder<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetReminder)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Organizer(&self) -> ::windows_core::Result<AppointmentOrganizer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Organizer)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppointmentOrganizer>(result__)
        }
    }
    pub fn SetOrganizer<'a, Param0: ::windows_core::IntoParam<'a, AppointmentOrganizer>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOrganizer)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Invitees(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<AppointmentInvitee>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Invitees)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<AppointmentInvitee>>(result__)
        }
    }
    pub fn Recurrence(&self) -> ::windows_core::Result<AppointmentRecurrence> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Recurrence)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppointmentRecurrence>(result__)
        }
    }
    pub fn SetRecurrence<'a, Param0: ::windows_core::IntoParam<'a, AppointmentRecurrence>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRecurrence)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn BusyStatus(&self) -> ::windows_core::Result<AppointmentBusyStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppointmentBusyStatus>::zeroed();
            (::windows_core::Interface::vtable(this).BusyStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppointmentBusyStatus>(result__)
        }
    }
    pub fn SetBusyStatus(&self, value: AppointmentBusyStatus) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBusyStatus)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllDay(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AllDay)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllDay(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAllDay)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Sensitivity(&self) -> ::windows_core::Result<AppointmentSensitivity> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppointmentSensitivity>::zeroed();
            (::windows_core::Interface::vtable(this).Sensitivity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppointmentSensitivity>(result__)
        }
    }
    pub fn SetSensitivity(&self, value: AppointmentSensitivity) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSensitivity)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Uri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn LocalId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).LocalId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn CalendarId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CalendarId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn RoamingId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RoamingId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetRoamingId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRoamingId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn OriginalStartTime(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = &::windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OriginalStartTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
    pub fn IsResponseRequested(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsResponseRequested)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsResponseRequested(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsResponseRequested)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllowNewTimeProposal(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AllowNewTimeProposal)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowNewTimeProposal(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowNewTimeProposal)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OnlineMeetingLink(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).OnlineMeetingLink)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetOnlineMeetingLink<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetOnlineMeetingLink)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ReplyTime(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = &::windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReplyTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
    pub fn SetReplyTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::DateTime>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetReplyTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn UserResponse(&self) -> ::windows_core::Result<AppointmentParticipantResponse> {
        let this = &::windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppointmentParticipantResponse>::zeroed();
            (::windows_core::Interface::vtable(this).UserResponse)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppointmentParticipantResponse>(result__)
        }
    }
    pub fn SetUserResponse(&self, value: AppointmentParticipantResponse) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetUserResponse)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HasInvitees(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasInvitees)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsCanceledMeeting(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCanceledMeeting)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsCanceledMeeting(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsCanceledMeeting)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsOrganizedByUser(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsOrganizedByUser)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsOrganizedByUser(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsOrganizedByUser)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ChangeNumber(&self) -> ::windows_core::Result<u64> {
        let this = &::windows_core::Interface::cast::<IAppointment3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).ChangeNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn RemoteChangeNumber(&self) -> ::windows_core::Result<u64> {
        let this = &::windows_core::Interface::cast::<IAppointment3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).RemoteChangeNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn SetRemoteChangeNumber(&self, value: u64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppointment3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRemoteChangeNumber)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DetailsKind(&self) -> ::windows_core::Result<AppointmentDetailsKind> {
        let this = &::windows_core::Interface::cast::<IAppointment3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppointmentDetailsKind>::zeroed();
            (::windows_core::Interface::vtable(this).DetailsKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppointmentDetailsKind>(result__)
        }
    }
    pub fn SetDetailsKind(&self, value: AppointmentDetailsKind) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppointment3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDetailsKind)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for Appointment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Appointment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Appointment {}
impl ::core::fmt::Debug for Appointment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Appointment").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Appointment {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.Appointment;{dd002f2f-2bdd-4076-90a3-22c275312965})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Appointment {
    type Vtable = IAppointment_Vtbl;
    const IID: ::windows_core::GUID = <IAppointment as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Appointment {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.Appointment";
}
impl ::core::convert::From<Appointment> for ::windows_core::IUnknown {
    fn from(value: Appointment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Appointment> for ::windows_core::IUnknown {
    fn from(value: &Appointment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Appointment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Appointment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Appointment> for ::windows_core::IInspectable {
    fn from(value: Appointment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Appointment> for ::windows_core::IInspectable {
    fn from(value: &Appointment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Appointment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Appointment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Appointment {}
unsafe impl ::core::marker::Sync for Appointment {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for AppointmentBusyStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppointmentBusyStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppointmentBusyStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentBusyStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppointmentBusyStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentBusyStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AppointmentCalendar(::windows_core::IUnknown);
impl AppointmentCalendar {
    #[cfg(feature = "winrt-ui")]
    pub fn DisplayColor(&self) -> ::windows_core::Result<::winrt_ui::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_ui::Color>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_ui::Color>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetDisplayName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn LocalId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).LocalId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IsHidden(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsHidden)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn OtherAppReadAccess(&self) -> ::windows_core::Result<AppointmentCalendarOtherAppReadAccess> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppointmentCalendarOtherAppReadAccess>::zeroed();
            (::windows_core::Interface::vtable(this).OtherAppReadAccess)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppointmentCalendarOtherAppReadAccess>(result__)
        }
    }
    pub fn SetOtherAppReadAccess(&self, value: AppointmentCalendarOtherAppReadAccess) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOtherAppReadAccess)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OtherAppWriteAccess(&self) -> ::windows_core::Result<AppointmentCalendarOtherAppWriteAccess> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppointmentCalendarOtherAppWriteAccess>::zeroed();
            (::windows_core::Interface::vtable(this).OtherAppWriteAccess)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppointmentCalendarOtherAppWriteAccess>(result__)
        }
    }
    pub fn SetOtherAppWriteAccess(&self, value: AppointmentCalendarOtherAppWriteAccess) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOtherAppWriteAccess)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SourceDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SourceDisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SummaryCardView(&self) -> ::windows_core::Result<AppointmentSummaryCardView> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppointmentSummaryCardView>::zeroed();
            (::windows_core::Interface::vtable(this).SummaryCardView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppointmentSummaryCardView>(result__)
        }
    }
    pub fn SetSummaryCardView(&self, value: AppointmentSummaryCardView) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSummaryCardView)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn FindAppointmentsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, rangestart: Param0, rangelength: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<Appointment>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindAppointmentsAsync)(::windows_core::Interface::as_raw(this), rangestart.into_param().abi(), rangelength.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<Appointment>>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn FindAppointmentsAsyncWithOptions<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>, Param2: ::windows_core::IntoParam<'a, FindAppointmentsOptions>>(&self, rangestart: Param0, rangelength: Param1, options: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<Appointment>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindAppointmentsAsyncWithOptions)(::windows_core::Interface::as_raw(this), rangestart.into_param().abi(), rangelength.into_param().abi(), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<Appointment>>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn FindExceptionsFromMasterAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, masterlocalid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<AppointmentException>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindExceptionsFromMasterAsync)(::windows_core::Interface::as_raw(this), masterlocalid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<AppointmentException>>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn FindAllInstancesAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, masterlocalid: Param0, rangestart: Param1, rangelength: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<Appointment>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindAllInstancesAsync)(::windows_core::Interface::as_raw(this), masterlocalid.into_param().abi(), rangestart.into_param().abi(), rangelength.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<Appointment>>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn FindAllInstancesAsyncWithOptions<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>, Param3: ::windows_core::IntoParam<'a, FindAppointmentsOptions>>(&self, masterlocalid: Param0, rangestart: Param1, rangelength: Param2, poptions: Param3) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<Appointment>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindAllInstancesAsyncWithOptions)(::windows_core::Interface::as_raw(this), masterlocalid.into_param().abi(), rangestart.into_param().abi(), rangelength.into_param().abi(), poptions.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<Appointment>>>(result__)
        }
    }
    pub fn GetAppointmentAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, localid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<Appointment>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAppointmentAsync)(::windows_core::Interface::as_raw(this), localid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<Appointment>>(result__)
        }
    }
    pub fn GetAppointmentInstanceAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>>(&self, localid: Param0, instancestarttime: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<Appointment>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAppointmentInstanceAsync)(::windows_core::Interface::as_raw(this), localid.into_param().abi(), instancestarttime.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<Appointment>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn FindUnexpandedAppointmentsAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<Appointment>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindUnexpandedAppointmentsAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<Appointment>>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn FindUnexpandedAppointmentsAsyncWithOptions<'a, Param0: ::windows_core::IntoParam<'a, FindAppointmentsOptions>>(&self, options: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<Appointment>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindUnexpandedAppointmentsAsyncWithOptions)(::windows_core::Interface::as_raw(this), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<Appointment>>>(result__)
        }
    }
    pub fn DeleteAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeleteAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn SaveAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SaveAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn DeleteAppointmentAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, localid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeleteAppointmentAsync)(::windows_core::Interface::as_raw(this), localid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn DeleteAppointmentInstanceAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>>(&self, localid: Param0, instancestarttime: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeleteAppointmentInstanceAsync)(::windows_core::Interface::as_raw(this), localid.into_param().abi(), instancestarttime.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn SaveAppointmentAsync<'a, Param0: ::windows_core::IntoParam<'a, Appointment>>(&self, pappointment: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SaveAppointmentAsync)(::windows_core::Interface::as_raw(this), pappointment.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn SyncManager(&self) -> ::windows_core::Result<AppointmentCalendarSyncManager> {
        let this = &::windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SyncManager)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppointmentCalendarSyncManager>(result__)
        }
    }
    pub fn RemoteId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RemoteId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetRemoteId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRemoteId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn SetDisplayColor<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_ui::Color>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayColor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SetIsHidden(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsHidden)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn UserDataAccountId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).UserDataAccountId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn CanCreateOrUpdateAppointments(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanCreateOrUpdateAppointments)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCanCreateOrUpdateAppointments(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCanCreateOrUpdateAppointments)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanCancelMeetings(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanCancelMeetings)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCanCancelMeetings(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCanCancelMeetings)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanForwardMeetings(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanForwardMeetings)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCanForwardMeetings(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCanForwardMeetings)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanProposeNewTimeForMeetings(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanProposeNewTimeForMeetings)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCanProposeNewTimeForMeetings(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCanProposeNewTimeForMeetings)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanUpdateMeetingResponses(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanUpdateMeetingResponses)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCanUpdateMeetingResponses(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCanUpdateMeetingResponses)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanNotifyInvitees(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanNotifyInvitees)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCanNotifyInvitees(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCanNotifyInvitees)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MustNofityInvitees(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).MustNofityInvitees)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetMustNofityInvitees(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetMustNofityInvitees)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TryCreateOrUpdateAppointmentAsync<'a, Param0: ::windows_core::IntoParam<'a, Appointment>>(&self, appointment: Param0, notifyinvitees: bool) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateOrUpdateAppointmentAsync)(::windows_core::Interface::as_raw(this), appointment.into_param().abi(), notifyinvitees, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn TryCancelMeetingAsync<'a, Param0: ::windows_core::IntoParam<'a, Appointment>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, meeting: Param0, subject: Param1, comment: Param2, notifyinvitees: bool) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryCancelMeetingAsync)(::windows_core::Interface::as_raw(this), meeting.into_param().abi(), subject.into_param().abi(), comment.into_param().abi(), notifyinvitees, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn TryForwardMeetingAsync<'a, Param0: ::windows_core::IntoParam<'a, Appointment>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<AppointmentInvitee>>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param4: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, meeting: Param0, invitees: Param1, subject: Param2, forwardheader: Param3, comment: Param4) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryForwardMeetingAsync)(::windows_core::Interface::as_raw(this), meeting.into_param().abi(), invitees.into_param().abi(), subject.into_param().abi(), forwardheader.into_param().abi(), comment.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn TryProposeNewTimeForMeetingAsync<'a, Param0: ::windows_core::IntoParam<'a, Appointment>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param4: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, meeting: Param0, newstarttime: Param1, newduration: Param2, subject: Param3, comment: Param4) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryProposeNewTimeForMeetingAsync)(::windows_core::Interface::as_raw(this), meeting.into_param().abi(), newstarttime.into_param().abi(), newduration.into_param().abi(), subject.into_param().abi(), comment.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn TryUpdateMeetingResponseAsync<'a, Param0: ::windows_core::IntoParam<'a, Appointment>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, meeting: Param0, response: AppointmentParticipantResponse, subject: Param2, comment: Param3, sendupdate: bool) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryUpdateMeetingResponseAsync)(::windows_core::Interface::as_raw(this), meeting.into_param().abi(), response, subject.into_param().abi(), comment.into_param().abi(), sendupdate, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn RegisterSyncManagerAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IAppointmentCalendar3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RegisterSyncManagerAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentCalendar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentCalendar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentCalendar {}
impl ::core::fmt::Debug for AppointmentCalendar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentCalendar").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppointmentCalendar {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentCalendar;{5273819d-8339-3d4f-a02f-64084452bb5d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppointmentCalendar {
    type Vtable = IAppointmentCalendar_Vtbl;
    const IID: ::windows_core::GUID = <IAppointmentCalendar as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentCalendar {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentCalendar";
}
impl ::core::convert::From<AppointmentCalendar> for ::windows_core::IUnknown {
    fn from(value: AppointmentCalendar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentCalendar> for ::windows_core::IUnknown {
    fn from(value: &AppointmentCalendar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppointmentCalendar {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppointmentCalendar {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentCalendar> for ::windows_core::IInspectable {
    fn from(value: AppointmentCalendar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentCalendar> for ::windows_core::IInspectable {
    fn from(value: &AppointmentCalendar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppointmentCalendar {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppointmentCalendar {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentCalendar {}
unsafe impl ::core::marker::Sync for AppointmentCalendar {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for AppointmentCalendarOtherAppReadAccess {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppointmentCalendarOtherAppReadAccess {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppointmentCalendarOtherAppReadAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentCalendarOtherAppReadAccess").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppointmentCalendarOtherAppReadAccess {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentCalendarOtherAppReadAccess;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for AppointmentCalendarOtherAppWriteAccess {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppointmentCalendarOtherAppWriteAccess {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppointmentCalendarOtherAppWriteAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentCalendarOtherAppWriteAccess").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppointmentCalendarOtherAppWriteAccess {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentCalendarOtherAppWriteAccess;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AppointmentCalendarSyncManager(::windows_core::IUnknown);
impl AppointmentCalendarSyncManager {
    pub fn Status(&self) -> ::windows_core::Result<AppointmentCalendarSyncStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppointmentCalendarSyncStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppointmentCalendarSyncStatus>(result__)
        }
    }
    pub fn LastSuccessfulSyncTime(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).LastSuccessfulSyncTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn LastAttemptedSyncTime(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).LastAttemptedSyncTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn SyncAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SyncAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn SyncStatusChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AppointmentCalendarSyncManager, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SyncStatusChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveSyncStatusChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSyncStatusChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn SetStatus(&self, value: AppointmentCalendarSyncStatus) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppointmentCalendarSyncManager2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStatus)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetLastSuccessfulSyncTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppointmentCalendarSyncManager2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLastSuccessfulSyncTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SetLastAttemptedSyncTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppointmentCalendarSyncManager2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLastAttemptedSyncTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for AppointmentCalendarSyncManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentCalendarSyncManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentCalendarSyncManager {}
impl ::core::fmt::Debug for AppointmentCalendarSyncManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentCalendarSyncManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppointmentCalendarSyncManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentCalendarSyncManager;{2b21b3a0-4aff-4392-bc5f-5645ffcffb17})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppointmentCalendarSyncManager {
    type Vtable = IAppointmentCalendarSyncManager_Vtbl;
    const IID: ::windows_core::GUID = <IAppointmentCalendarSyncManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentCalendarSyncManager {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentCalendarSyncManager";
}
impl ::core::convert::From<AppointmentCalendarSyncManager> for ::windows_core::IUnknown {
    fn from(value: AppointmentCalendarSyncManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentCalendarSyncManager> for ::windows_core::IUnknown {
    fn from(value: &AppointmentCalendarSyncManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppointmentCalendarSyncManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppointmentCalendarSyncManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentCalendarSyncManager> for ::windows_core::IInspectable {
    fn from(value: AppointmentCalendarSyncManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentCalendarSyncManager> for ::windows_core::IInspectable {
    fn from(value: &AppointmentCalendarSyncManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppointmentCalendarSyncManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppointmentCalendarSyncManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentCalendarSyncManager {}
unsafe impl ::core::marker::Sync for AppointmentCalendarSyncManager {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for AppointmentCalendarSyncStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppointmentCalendarSyncStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppointmentCalendarSyncStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentCalendarSyncStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppointmentCalendarSyncStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentCalendarSyncStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AppointmentConflictResult(::windows_core::IUnknown);
impl AppointmentConflictResult {
    pub fn Type(&self) -> ::windows_core::Result<AppointmentConflictType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppointmentConflictType>::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppointmentConflictType>(result__)
        }
    }
    pub fn Date(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Date)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentConflictResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentConflictResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentConflictResult {}
impl ::core::fmt::Debug for AppointmentConflictResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentConflictResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppointmentConflictResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentConflictResult;{d5cdf0be-2f2f-3b7d-af0a-a7e20f3a46e3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppointmentConflictResult {
    type Vtable = IAppointmentConflictResult_Vtbl;
    const IID: ::windows_core::GUID = <IAppointmentConflictResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentConflictResult {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentConflictResult";
}
impl ::core::convert::From<AppointmentConflictResult> for ::windows_core::IUnknown {
    fn from(value: AppointmentConflictResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentConflictResult> for ::windows_core::IUnknown {
    fn from(value: &AppointmentConflictResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppointmentConflictResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppointmentConflictResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentConflictResult> for ::windows_core::IInspectable {
    fn from(value: AppointmentConflictResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentConflictResult> for ::windows_core::IInspectable {
    fn from(value: &AppointmentConflictResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppointmentConflictResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppointmentConflictResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentConflictResult {}
unsafe impl ::core::marker::Sync for AppointmentConflictResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for AppointmentConflictType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppointmentConflictType {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppointmentConflictType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentConflictType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppointmentConflictType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentConflictType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for AppointmentDaysOfWeek {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppointmentDaysOfWeek {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppointmentDaysOfWeek {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentDaysOfWeek").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for AppointmentDaysOfWeek {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AppointmentDaysOfWeek {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AppointmentDaysOfWeek {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AppointmentDaysOfWeek {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AppointmentDaysOfWeek {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for AppointmentDaysOfWeek {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentDaysOfWeek;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for AppointmentDetailsKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppointmentDetailsKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppointmentDetailsKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentDetailsKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppointmentDetailsKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentDetailsKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AppointmentException(::windows_core::IUnknown);
impl AppointmentException {
    pub fn Appointment(&self) -> ::windows_core::Result<Appointment> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Appointment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Appointment>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ExceptionProperties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExceptionProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn IsDeleted(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDeleted)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentException {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentException {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentException {}
impl ::core::fmt::Debug for AppointmentException {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentException").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppointmentException {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentException;{a2076767-16f6-4bce-9f5a-8600b8019fcb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppointmentException {
    type Vtable = IAppointmentException_Vtbl;
    const IID: ::windows_core::GUID = <IAppointmentException as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentException {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentException";
}
impl ::core::convert::From<AppointmentException> for ::windows_core::IUnknown {
    fn from(value: AppointmentException) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentException> for ::windows_core::IUnknown {
    fn from(value: &AppointmentException) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppointmentException {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppointmentException {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentException> for ::windows_core::IInspectable {
    fn from(value: AppointmentException) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentException> for ::windows_core::IInspectable {
    fn from(value: &AppointmentException) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppointmentException {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppointmentException {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentException {}
unsafe impl ::core::marker::Sync for AppointmentException {}
#[repr(transparent)]
pub struct AppointmentInvitee(::windows_core::IUnknown);
impl AppointmentInvitee {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AppointmentInvitee, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Role(&self) -> ::windows_core::Result<AppointmentParticipantRole> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppointmentParticipantRole>::zeroed();
            (::windows_core::Interface::vtable(this).Role)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppointmentParticipantRole>(result__)
        }
    }
    pub fn SetRole(&self, value: AppointmentParticipantRole) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRole)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Response(&self) -> ::windows_core::Result<AppointmentParticipantResponse> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppointmentParticipantResponse>::zeroed();
            (::windows_core::Interface::vtable(this).Response)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppointmentParticipantResponse>(result__)
        }
    }
    pub fn SetResponse(&self, value: AppointmentParticipantResponse) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetResponse)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAppointmentParticipant>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetDisplayName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppointmentParticipant>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Address(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAppointmentParticipant>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Address)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetAddress<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppointmentParticipant>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAddress)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for AppointmentInvitee {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentInvitee {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentInvitee {}
impl ::core::fmt::Debug for AppointmentInvitee {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentInvitee").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppointmentInvitee {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentInvitee;{13bf0796-9842-495b-b0e7-ef8f79c0701d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppointmentInvitee {
    type Vtable = IAppointmentInvitee_Vtbl;
    const IID: ::windows_core::GUID = <IAppointmentInvitee as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentInvitee {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentInvitee";
}
impl ::core::convert::From<AppointmentInvitee> for ::windows_core::IUnknown {
    fn from(value: AppointmentInvitee) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentInvitee> for ::windows_core::IUnknown {
    fn from(value: &AppointmentInvitee) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppointmentInvitee {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppointmentInvitee {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentInvitee> for ::windows_core::IInspectable {
    fn from(value: AppointmentInvitee) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentInvitee> for ::windows_core::IInspectable {
    fn from(value: &AppointmentInvitee) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppointmentInvitee {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppointmentInvitee {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AppointmentInvitee> for IAppointmentParticipant {
    type Error = ::windows_core::Error;
    fn try_from(value: AppointmentInvitee) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentInvitee> for IAppointmentParticipant {
    type Error = ::windows_core::Error;
    fn try_from(value: &AppointmentInvitee) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAppointmentParticipant> for AppointmentInvitee {
    fn into_param(self) -> ::windows_core::Param<'a, IAppointmentParticipant> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAppointmentParticipant> for &AppointmentInvitee {
    fn into_param(self) -> ::windows_core::Param<'a, IAppointmentParticipant> {
        ::core::convert::TryInto::<IAppointmentParticipant>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AppointmentInvitee {}
unsafe impl ::core::marker::Sync for AppointmentInvitee {}
pub struct AppointmentManager;
impl AppointmentManager {
    pub fn ShowAddAppointmentAsync<'a, Param0: ::windows_core::IntoParam<'a, Appointment>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(appointment: Param0, selection: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowAddAppointmentAsync)(::windows_core::Interface::as_raw(this), appointment.into_param().abi(), selection.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        })
    }
    #[cfg(feature = "winrt-ui")]
    pub fn ShowAddAppointmentWithPlacementAsync<'a, Param0: ::windows_core::IntoParam<'a, Appointment>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(appointment: Param0, selection: Param1, preferredplacement: ::winrt_ui::Popups::Placement) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowAddAppointmentWithPlacementAsync)(::windows_core::Interface::as_raw(this), appointment.into_param().abi(), selection.into_param().abi(), preferredplacement, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        })
    }
    pub fn ShowReplaceAppointmentAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, Appointment>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(appointmentid: Param0, appointment: Param1, selection: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowReplaceAppointmentAsync)(::windows_core::Interface::as_raw(this), appointmentid.into_param().abi(), appointment.into_param().abi(), selection.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        })
    }
    #[cfg(feature = "winrt-ui")]
    pub fn ShowReplaceAppointmentWithPlacementAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, Appointment>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(appointmentid: Param0, appointment: Param1, selection: Param2, preferredplacement: ::winrt_ui::Popups::Placement) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowReplaceAppointmentWithPlacementAsync)(::windows_core::Interface::as_raw(this), appointmentid.into_param().abi(), appointment.into_param().abi(), selection.into_param().abi(), preferredplacement, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        })
    }
    #[cfg(feature = "winrt-ui")]
    pub fn ShowReplaceAppointmentWithPlacementAndDateAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, Appointment>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>, Param4: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>>(appointmentid: Param0, appointment: Param1, selection: Param2, preferredplacement: ::winrt_ui::Popups::Placement, instancestartdate: Param4) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowReplaceAppointmentWithPlacementAndDateAsync)(::windows_core::Interface::as_raw(this), appointmentid.into_param().abi(), appointment.into_param().abi(), selection.into_param().abi(), preferredplacement, instancestartdate.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        })
    }
    pub fn ShowRemoveAppointmentAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(appointmentid: Param0, selection: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowRemoveAppointmentAsync)(::windows_core::Interface::as_raw(this), appointmentid.into_param().abi(), selection.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "winrt-ui")]
    pub fn ShowRemoveAppointmentWithPlacementAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(appointmentid: Param0, selection: Param1, preferredplacement: ::winrt_ui::Popups::Placement) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowRemoveAppointmentWithPlacementAsync)(::windows_core::Interface::as_raw(this), appointmentid.into_param().abi(), selection.into_param().abi(), preferredplacement, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "winrt-ui")]
    pub fn ShowRemoveAppointmentWithPlacementAndDateAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>, Param3: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>>(appointmentid: Param0, selection: Param1, preferredplacement: ::winrt_ui::Popups::Placement, instancestartdate: Param3) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowRemoveAppointmentWithPlacementAndDateAsync)(::windows_core::Interface::as_raw(this), appointmentid.into_param().abi(), selection.into_param().abi(), preferredplacement, instancestartdate.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        })
    }
    pub fn ShowTimeFrameAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(timetoshow: Param0, duration: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowTimeFrameAsync)(::windows_core::Interface::as_raw(this), timetoshow.into_param().abi(), duration.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        })
    }
    pub fn ShowAppointmentDetailsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(appointmentid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        Self::IAppointmentManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowAppointmentDetailsAsync)(::windows_core::Interface::as_raw(this), appointmentid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        })
    }
    pub fn ShowAppointmentDetailsWithDateAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>>(appointmentid: Param0, instancestartdate: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        Self::IAppointmentManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowAppointmentDetailsWithDateAsync)(::windows_core::Interface::as_raw(this), appointmentid.into_param().abi(), instancestartdate.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        })
    }
    pub fn ShowEditNewAppointmentAsync<'a, Param0: ::windows_core::IntoParam<'a, Appointment>>(appointment: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::IAppointmentManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowEditNewAppointmentAsync)(::windows_core::Interface::as_raw(this), appointment.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        })
    }
    pub fn RequestStoreAsync(options: AppointmentStoreAccessType) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<AppointmentStore>> {
        Self::IAppointmentManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestStoreAsync)(::windows_core::Interface::as_raw(this), options, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<AppointmentStore>>(result__)
        })
    }
    #[cfg(feature = "winrt-system")]
    pub fn GetForUser<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_system::User>>(user: Param0) -> ::windows_core::Result<AppointmentManagerForUser> {
        Self::IAppointmentManagerStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), result__.as_mut_ptr()).from_abi::<AppointmentManagerForUser>(result__)
        })
    }
    pub fn IAppointmentManagerStatics<R, F: FnOnce(&IAppointmentManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AppointmentManager, IAppointmentManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAppointmentManagerStatics2<R, F: FnOnce(&IAppointmentManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AppointmentManager, IAppointmentManagerStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAppointmentManagerStatics3<R, F: FnOnce(&IAppointmentManagerStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AppointmentManager, IAppointmentManagerStatics3> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for AppointmentManager {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentManager";
}
#[repr(transparent)]
pub struct AppointmentManagerForUser(::windows_core::IUnknown);
impl AppointmentManagerForUser {
    pub fn ShowAddAppointmentAsync<'a, Param0: ::windows_core::IntoParam<'a, Appointment>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(&self, appointment: Param0, selection: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowAddAppointmentAsync)(::windows_core::Interface::as_raw(this), appointment.into_param().abi(), selection.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn ShowAddAppointmentWithPlacementAsync<'a, Param0: ::windows_core::IntoParam<'a, Appointment>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(&self, appointment: Param0, selection: Param1, preferredplacement: ::winrt_ui::Popups::Placement) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowAddAppointmentWithPlacementAsync)(::windows_core::Interface::as_raw(this), appointment.into_param().abi(), selection.into_param().abi(), preferredplacement, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn ShowReplaceAppointmentAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, Appointment>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(&self, appointmentid: Param0, appointment: Param1, selection: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowReplaceAppointmentAsync)(::windows_core::Interface::as_raw(this), appointmentid.into_param().abi(), appointment.into_param().abi(), selection.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn ShowReplaceAppointmentWithPlacementAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, Appointment>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(&self, appointmentid: Param0, appointment: Param1, selection: Param2, preferredplacement: ::winrt_ui::Popups::Placement) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowReplaceAppointmentWithPlacementAsync)(::windows_core::Interface::as_raw(this), appointmentid.into_param().abi(), appointment.into_param().abi(), selection.into_param().abi(), preferredplacement, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn ShowReplaceAppointmentWithPlacementAndDateAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, Appointment>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>, Param4: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>>(&self, appointmentid: Param0, appointment: Param1, selection: Param2, preferredplacement: ::winrt_ui::Popups::Placement, instancestartdate: Param4) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowReplaceAppointmentWithPlacementAndDateAsync)(::windows_core::Interface::as_raw(this), appointmentid.into_param().abi(), appointment.into_param().abi(), selection.into_param().abi(), preferredplacement, instancestartdate.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn ShowRemoveAppointmentAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(&self, appointmentid: Param0, selection: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowRemoveAppointmentAsync)(::windows_core::Interface::as_raw(this), appointmentid.into_param().abi(), selection.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn ShowRemoveAppointmentWithPlacementAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(&self, appointmentid: Param0, selection: Param1, preferredplacement: ::winrt_ui::Popups::Placement) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowRemoveAppointmentWithPlacementAsync)(::windows_core::Interface::as_raw(this), appointmentid.into_param().abi(), selection.into_param().abi(), preferredplacement, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn ShowRemoveAppointmentWithPlacementAndDateAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>, Param3: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>>(&self, appointmentid: Param0, selection: Param1, preferredplacement: ::winrt_ui::Popups::Placement, instancestartdate: Param3) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowRemoveAppointmentWithPlacementAndDateAsync)(::windows_core::Interface::as_raw(this), appointmentid.into_param().abi(), selection.into_param().abi(), preferredplacement, instancestartdate.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn ShowTimeFrameAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, timetoshow: Param0, duration: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowTimeFrameAsync)(::windows_core::Interface::as_raw(this), timetoshow.into_param().abi(), duration.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn ShowAppointmentDetailsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, appointmentid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowAppointmentDetailsAsync)(::windows_core::Interface::as_raw(this), appointmentid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn ShowAppointmentDetailsWithDateAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>>(&self, appointmentid: Param0, instancestartdate: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowAppointmentDetailsWithDateAsync)(::windows_core::Interface::as_raw(this), appointmentid.into_param().abi(), instancestartdate.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn ShowEditNewAppointmentAsync<'a, Param0: ::windows_core::IntoParam<'a, Appointment>>(&self, appointment: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowEditNewAppointmentAsync)(::windows_core::Interface::as_raw(this), appointment.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn RequestStoreAsync(&self, options: AppointmentStoreAccessType) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<AppointmentStore>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestStoreAsync)(::windows_core::Interface::as_raw(this), options, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<AppointmentStore>>(result__)
        }
    }
    #[cfg(feature = "winrt-system")]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentManagerForUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentManagerForUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentManagerForUser {}
impl ::core::fmt::Debug for AppointmentManagerForUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentManagerForUser").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppointmentManagerForUser {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentManagerForUser;{70261423-73cc-4660-b318-b01365302a03})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppointmentManagerForUser {
    type Vtable = IAppointmentManagerForUser_Vtbl;
    const IID: ::windows_core::GUID = <IAppointmentManagerForUser as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentManagerForUser {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentManagerForUser";
}
impl ::core::convert::From<AppointmentManagerForUser> for ::windows_core::IUnknown {
    fn from(value: AppointmentManagerForUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentManagerForUser> for ::windows_core::IUnknown {
    fn from(value: &AppointmentManagerForUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppointmentManagerForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppointmentManagerForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentManagerForUser> for ::windows_core::IInspectable {
    fn from(value: AppointmentManagerForUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentManagerForUser> for ::windows_core::IInspectable {
    fn from(value: &AppointmentManagerForUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppointmentManagerForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppointmentManagerForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentManagerForUser {}
unsafe impl ::core::marker::Sync for AppointmentManagerForUser {}
#[repr(transparent)]
pub struct AppointmentOrganizer(::windows_core::IUnknown);
impl AppointmentOrganizer {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AppointmentOrganizer, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetDisplayName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Address(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Address)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetAddress<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAddress)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for AppointmentOrganizer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentOrganizer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentOrganizer {}
impl ::core::fmt::Debug for AppointmentOrganizer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentOrganizer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppointmentOrganizer {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentOrganizer;{615e2902-9718-467b-83fb-b293a19121de})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppointmentOrganizer {
    type Vtable = IAppointmentParticipant_Vtbl;
    const IID: ::windows_core::GUID = <IAppointmentParticipant as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentOrganizer {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentOrganizer";
}
impl ::core::convert::From<AppointmentOrganizer> for ::windows_core::IUnknown {
    fn from(value: AppointmentOrganizer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentOrganizer> for ::windows_core::IUnknown {
    fn from(value: &AppointmentOrganizer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppointmentOrganizer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppointmentOrganizer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentOrganizer> for ::windows_core::IInspectable {
    fn from(value: AppointmentOrganizer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentOrganizer> for ::windows_core::IInspectable {
    fn from(value: &AppointmentOrganizer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppointmentOrganizer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppointmentOrganizer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AppointmentOrganizer> for IAppointmentParticipant {
    type Error = ::windows_core::Error;
    fn try_from(value: AppointmentOrganizer) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentOrganizer> for IAppointmentParticipant {
    type Error = ::windows_core::Error;
    fn try_from(value: &AppointmentOrganizer) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAppointmentParticipant> for AppointmentOrganizer {
    fn into_param(self) -> ::windows_core::Param<'a, IAppointmentParticipant> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAppointmentParticipant> for &AppointmentOrganizer {
    fn into_param(self) -> ::windows_core::Param<'a, IAppointmentParticipant> {
        ::core::convert::TryInto::<IAppointmentParticipant>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AppointmentOrganizer {}
unsafe impl ::core::marker::Sync for AppointmentOrganizer {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for AppointmentParticipantResponse {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppointmentParticipantResponse {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppointmentParticipantResponse {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentParticipantResponse").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppointmentParticipantResponse {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentParticipantResponse;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for AppointmentParticipantRole {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppointmentParticipantRole {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppointmentParticipantRole {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentParticipantRole").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppointmentParticipantRole {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentParticipantRole;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
pub struct AppointmentProperties;
impl AppointmentProperties {
    pub fn Subject() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Subject)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Location() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Location)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn StartTime() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Duration() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Reminder() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Reminder)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn BusyStatus() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).BusyStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Sensitivity() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Sensitivity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn OriginalStartTime() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).OriginalStartTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn IsResponseRequested() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).IsResponseRequested)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn AllowNewTimeProposal() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AllowNewTimeProposal)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn AllDay() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AllDay)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Details() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Details)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn OnlineMeetingLink() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).OnlineMeetingLink)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn ReplyTime() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ReplyTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Organizer() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Organizer)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn UserResponse() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).UserResponse)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn HasInvitees() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).HasInvitees)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn IsCanceledMeeting() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).IsCanceledMeeting)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn IsOrganizedByUser() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).IsOrganizedByUser)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Recurrence() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Recurrence)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Uri() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Invitees() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Invitees)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn DefaultProperties() -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DefaultProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        })
    }
    pub fn ChangeNumber() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ChangeNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn RemoteChangeNumber() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RemoteChangeNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn DetailsKind() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DetailsKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn IAppointmentPropertiesStatics<R, F: FnOnce(&IAppointmentPropertiesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AppointmentProperties, IAppointmentPropertiesStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAppointmentPropertiesStatics2<R, F: FnOnce(&IAppointmentPropertiesStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AppointmentProperties, IAppointmentPropertiesStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for AppointmentProperties {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentProperties";
}
#[repr(transparent)]
pub struct AppointmentRecurrence(::windows_core::IUnknown);
impl AppointmentRecurrence {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AppointmentRecurrence, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Unit(&self) -> ::windows_core::Result<AppointmentRecurrenceUnit> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppointmentRecurrenceUnit>::zeroed();
            (::windows_core::Interface::vtable(this).Unit)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppointmentRecurrenceUnit>(result__)
        }
    }
    pub fn SetUnit(&self, value: AppointmentRecurrenceUnit) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUnit)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Occurrences(&self) -> ::windows_core::Result<::winrt_foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Occurrences)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u32>>(result__)
        }
    }
    pub fn SetOccurrences<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<u32>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOccurrences)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Until(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Until)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
    pub fn SetUntil<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::DateTime>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUntil)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Interval(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Interval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetInterval(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInterval)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DaysOfWeek(&self) -> ::windows_core::Result<AppointmentDaysOfWeek> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppointmentDaysOfWeek>::zeroed();
            (::windows_core::Interface::vtable(this).DaysOfWeek)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppointmentDaysOfWeek>(result__)
        }
    }
    pub fn SetDaysOfWeek(&self, value: AppointmentDaysOfWeek) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDaysOfWeek)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WeekOfMonth(&self) -> ::windows_core::Result<AppointmentWeekOfMonth> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppointmentWeekOfMonth>::zeroed();
            (::windows_core::Interface::vtable(this).WeekOfMonth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppointmentWeekOfMonth>(result__)
        }
    }
    pub fn SetWeekOfMonth(&self, value: AppointmentWeekOfMonth) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetWeekOfMonth)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Month(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Month)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetMonth(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMonth)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Day(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Day)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetDay(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDay)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RecurrenceType(&self) -> ::windows_core::Result<RecurrenceType> {
        let this = &::windows_core::Interface::cast::<IAppointmentRecurrence2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<RecurrenceType>::zeroed();
            (::windows_core::Interface::vtable(this).RecurrenceType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RecurrenceType>(result__)
        }
    }
    pub fn TimeZone(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAppointmentRecurrence2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TimeZone)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetTimeZone<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppointmentRecurrence2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetTimeZone)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CalendarIdentifier(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAppointmentRecurrence3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CalendarIdentifier)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentRecurrence {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentRecurrence {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentRecurrence {}
impl ::core::fmt::Debug for AppointmentRecurrence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentRecurrence").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppointmentRecurrence {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentRecurrence;{d87b3e83-15a6-487b-b959-0c361e60e954})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppointmentRecurrence {
    type Vtable = IAppointmentRecurrence_Vtbl;
    const IID: ::windows_core::GUID = <IAppointmentRecurrence as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentRecurrence {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentRecurrence";
}
impl ::core::convert::From<AppointmentRecurrence> for ::windows_core::IUnknown {
    fn from(value: AppointmentRecurrence) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentRecurrence> for ::windows_core::IUnknown {
    fn from(value: &AppointmentRecurrence) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppointmentRecurrence {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppointmentRecurrence {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentRecurrence> for ::windows_core::IInspectable {
    fn from(value: AppointmentRecurrence) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentRecurrence> for ::windows_core::IInspectable {
    fn from(value: &AppointmentRecurrence) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppointmentRecurrence {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppointmentRecurrence {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentRecurrence {}
unsafe impl ::core::marker::Sync for AppointmentRecurrence {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for AppointmentRecurrenceUnit {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppointmentRecurrenceUnit {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppointmentRecurrenceUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentRecurrenceUnit").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppointmentRecurrenceUnit {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentRecurrenceUnit;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for AppointmentSensitivity {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppointmentSensitivity {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppointmentSensitivity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentSensitivity").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppointmentSensitivity {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentSensitivity;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AppointmentStore(::windows_core::IUnknown);
impl AppointmentStore {
    pub fn ChangeTracker(&self) -> ::windows_core::Result<AppointmentStoreChangeTracker> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ChangeTracker)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppointmentStoreChangeTracker>(result__)
        }
    }
    pub fn CreateAppointmentCalendarAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<AppointmentCalendar>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateAppointmentCalendarAsync)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<AppointmentCalendar>>(result__)
        }
    }
    pub fn GetAppointmentCalendarAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, calendarid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<AppointmentCalendar>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAppointmentCalendarAsync)(::windows_core::Interface::as_raw(this), calendarid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<AppointmentCalendar>>(result__)
        }
    }
    pub fn GetAppointmentAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, localid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<Appointment>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAppointmentAsync)(::windows_core::Interface::as_raw(this), localid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<Appointment>>(result__)
        }
    }
    pub fn GetAppointmentInstanceAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>>(&self, localid: Param0, instancestarttime: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<Appointment>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAppointmentInstanceAsync)(::windows_core::Interface::as_raw(this), localid.into_param().abi(), instancestarttime.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<Appointment>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn FindAppointmentCalendarsAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<AppointmentCalendar>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindAppointmentCalendarsAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<AppointmentCalendar>>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn FindAppointmentCalendarsAsyncWithOptions(&self, options: FindAppointmentCalendarsOptions) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<AppointmentCalendar>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindAppointmentCalendarsAsyncWithOptions)(::windows_core::Interface::as_raw(this), options, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<AppointmentCalendar>>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn FindAppointmentsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, rangestart: Param0, rangelength: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<Appointment>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindAppointmentsAsync)(::windows_core::Interface::as_raw(this), rangestart.into_param().abi(), rangelength.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<Appointment>>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn FindAppointmentsAsyncWithOptions<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>, Param2: ::windows_core::IntoParam<'a, FindAppointmentsOptions>>(&self, rangestart: Param0, rangelength: Param1, options: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<Appointment>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindAppointmentsAsyncWithOptions)(::windows_core::Interface::as_raw(this), rangestart.into_param().abi(), rangelength.into_param().abi(), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<Appointment>>>(result__)
        }
    }
    pub fn FindConflictAsync<'a, Param0: ::windows_core::IntoParam<'a, Appointment>>(&self, appointment: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<AppointmentConflictResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindConflictAsync)(::windows_core::Interface::as_raw(this), appointment.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<AppointmentConflictResult>>(result__)
        }
    }
    pub fn FindConflictAsyncWithInstanceStart<'a, Param0: ::windows_core::IntoParam<'a, Appointment>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>>(&self, appointment: Param0, instancestarttime: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<AppointmentConflictResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindConflictAsyncWithInstanceStart)(::windows_core::Interface::as_raw(this), appointment.into_param().abi(), instancestarttime.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<AppointmentConflictResult>>(result__)
        }
    }
    pub fn MoveAppointmentAsync<'a, Param0: ::windows_core::IntoParam<'a, Appointment>, Param1: ::windows_core::IntoParam<'a, AppointmentCalendar>>(&self, appointment: Param0, destinationcalendar: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MoveAppointmentAsync)(::windows_core::Interface::as_raw(this), appointment.into_param().abi(), destinationcalendar.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn ShowAddAppointmentAsync<'a, Param0: ::windows_core::IntoParam<'a, Appointment>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(&self, appointment: Param0, selection: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowAddAppointmentAsync)(::windows_core::Interface::as_raw(this), appointment.into_param().abi(), selection.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn ShowReplaceAppointmentAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, Appointment>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(&self, localid: Param0, appointment: Param1, selection: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowReplaceAppointmentAsync)(::windows_core::Interface::as_raw(this), localid.into_param().abi(), appointment.into_param().abi(), selection.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn ShowReplaceAppointmentWithPlacementAndDateAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, Appointment>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>, Param4: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>>(&self, localid: Param0, appointment: Param1, selection: Param2, preferredplacement: ::winrt_ui::Popups::Placement, instancestartdate: Param4) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowReplaceAppointmentWithPlacementAndDateAsync)(::windows_core::Interface::as_raw(this), localid.into_param().abi(), appointment.into_param().abi(), selection.into_param().abi(), preferredplacement, instancestartdate.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn ShowRemoveAppointmentAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(&self, localid: Param0, selection: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowRemoveAppointmentAsync)(::windows_core::Interface::as_raw(this), localid.into_param().abi(), selection.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn ShowRemoveAppointmentWithPlacementAndDateAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>, Param3: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>>(&self, localid: Param0, selection: Param1, preferredplacement: ::winrt_ui::Popups::Placement, instancestartdate: Param3) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowRemoveAppointmentWithPlacementAndDateAsync)(::windows_core::Interface::as_raw(this), localid.into_param().abi(), selection.into_param().abi(), preferredplacement, instancestartdate.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn ShowAppointmentDetailsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, localid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowAppointmentDetailsAsync)(::windows_core::Interface::as_raw(this), localid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn ShowAppointmentDetailsWithDateAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>>(&self, localid: Param0, instancestartdate: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowAppointmentDetailsWithDateAsync)(::windows_core::Interface::as_raw(this), localid.into_param().abi(), instancestartdate.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn ShowEditNewAppointmentAsync<'a, Param0: ::windows_core::IntoParam<'a, Appointment>>(&self, appointment: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowEditNewAppointmentAsync)(::windows_core::Interface::as_raw(this), appointment.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn FindLocalIdsFromRoamingIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, roamingid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindLocalIdsFromRoamingIdAsync)(::windows_core::Interface::as_raw(this), roamingid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>>(result__)
        }
    }
    pub fn StoreChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AppointmentStore, AppointmentStoreChangedEventArgs>>>(&self, phandler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IAppointmentStore2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).StoreChanged)(::windows_core::Interface::as_raw(this), phandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveStoreChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppointmentStore2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStoreChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn CreateAppointmentCalendarInAccountAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0, userdataaccountid: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<AppointmentCalendar>> {
        let this = &::windows_core::Interface::cast::<IAppointmentStore2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateAppointmentCalendarInAccountAsync)(::windows_core::Interface::as_raw(this), name.into_param().abi(), userdataaccountid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<AppointmentCalendar>>(result__)
        }
    }
    pub fn GetChangeTracker<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, identity: Param0) -> ::windows_core::Result<AppointmentStoreChangeTracker> {
        let this = &::windows_core::Interface::cast::<IAppointmentStore3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetChangeTracker)(::windows_core::Interface::as_raw(this), identity.into_param().abi(), result__.as_mut_ptr()).from_abi::<AppointmentStoreChangeTracker>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentStore {}
impl ::core::fmt::Debug for AppointmentStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentStore").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppointmentStore {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentStore;{a461918c-7a47-4d96-96c9-15cd8a05a735})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppointmentStore {
    type Vtable = IAppointmentStore_Vtbl;
    const IID: ::windows_core::GUID = <IAppointmentStore as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentStore {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentStore";
}
impl ::core::convert::From<AppointmentStore> for ::windows_core::IUnknown {
    fn from(value: AppointmentStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentStore> for ::windows_core::IUnknown {
    fn from(value: &AppointmentStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppointmentStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppointmentStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentStore> for ::windows_core::IInspectable {
    fn from(value: AppointmentStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentStore> for ::windows_core::IInspectable {
    fn from(value: &AppointmentStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppointmentStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppointmentStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentStore {}
unsafe impl ::core::marker::Sync for AppointmentStore {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for AppointmentStoreAccessType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppointmentStoreAccessType {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppointmentStoreAccessType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentStoreAccessType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppointmentStoreAccessType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentStoreAccessType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AppointmentStoreChange(::windows_core::IUnknown);
impl AppointmentStoreChange {
    pub fn Appointment(&self) -> ::windows_core::Result<Appointment> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Appointment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Appointment>(result__)
        }
    }
    pub fn ChangeType(&self) -> ::windows_core::Result<AppointmentStoreChangeType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppointmentStoreChangeType>::zeroed();
            (::windows_core::Interface::vtable(this).ChangeType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppointmentStoreChangeType>(result__)
        }
    }
    pub fn AppointmentCalendar(&self) -> ::windows_core::Result<AppointmentCalendar> {
        let this = &::windows_core::Interface::cast::<IAppointmentStoreChange2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppointmentCalendar)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppointmentCalendar>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentStoreChange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentStoreChange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentStoreChange {}
impl ::core::fmt::Debug for AppointmentStoreChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentStoreChange").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppointmentStoreChange {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentStoreChange;{a5a6e035-0a33-3654-8463-b543e90c3b79})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppointmentStoreChange {
    type Vtable = IAppointmentStoreChange_Vtbl;
    const IID: ::windows_core::GUID = <IAppointmentStoreChange as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentStoreChange {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentStoreChange";
}
impl ::core::convert::From<AppointmentStoreChange> for ::windows_core::IUnknown {
    fn from(value: AppointmentStoreChange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentStoreChange> for ::windows_core::IUnknown {
    fn from(value: &AppointmentStoreChange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppointmentStoreChange {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppointmentStoreChange {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentStoreChange> for ::windows_core::IInspectable {
    fn from(value: AppointmentStoreChange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentStoreChange> for ::windows_core::IInspectable {
    fn from(value: &AppointmentStoreChange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppointmentStoreChange {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppointmentStoreChange {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentStoreChange {}
unsafe impl ::core::marker::Sync for AppointmentStoreChange {}
#[repr(transparent)]
pub struct AppointmentStoreChangeReader(::windows_core::IUnknown);
impl AppointmentStoreChangeReader {
    #[cfg(feature = "winrt-foundation")]
    pub fn ReadBatchAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<AppointmentStoreChange>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReadBatchAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<AppointmentStoreChange>>>(result__)
        }
    }
    pub fn AcceptChanges(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AcceptChanges)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn AcceptChangesThrough<'a, Param0: ::windows_core::IntoParam<'a, AppointmentStoreChange>>(&self, lastchangetoaccept: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AcceptChangesThrough)(::windows_core::Interface::as_raw(this), lastchangetoaccept.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for AppointmentStoreChangeReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentStoreChangeReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentStoreChangeReader {}
impl ::core::fmt::Debug for AppointmentStoreChangeReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentStoreChangeReader").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppointmentStoreChangeReader {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentStoreChangeReader;{8b2409f1-65f3-42a0-961d-4c209bf30370})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppointmentStoreChangeReader {
    type Vtable = IAppointmentStoreChangeReader_Vtbl;
    const IID: ::windows_core::GUID = <IAppointmentStoreChangeReader as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentStoreChangeReader {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentStoreChangeReader";
}
impl ::core::convert::From<AppointmentStoreChangeReader> for ::windows_core::IUnknown {
    fn from(value: AppointmentStoreChangeReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentStoreChangeReader> for ::windows_core::IUnknown {
    fn from(value: &AppointmentStoreChangeReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppointmentStoreChangeReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppointmentStoreChangeReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentStoreChangeReader> for ::windows_core::IInspectable {
    fn from(value: AppointmentStoreChangeReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentStoreChangeReader> for ::windows_core::IInspectable {
    fn from(value: &AppointmentStoreChangeReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppointmentStoreChangeReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppointmentStoreChangeReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentStoreChangeReader {}
unsafe impl ::core::marker::Sync for AppointmentStoreChangeReader {}
#[repr(transparent)]
pub struct AppointmentStoreChangeTracker(::windows_core::IUnknown);
impl AppointmentStoreChangeTracker {
    pub fn GetChangeReader(&self) -> ::windows_core::Result<AppointmentStoreChangeReader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetChangeReader)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppointmentStoreChangeReader>(result__)
        }
    }
    pub fn Enable(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Enable)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Reset)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn IsTracking(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppointmentStoreChangeTracker2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsTracking)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentStoreChangeTracker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentStoreChangeTracker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentStoreChangeTracker {}
impl ::core::fmt::Debug for AppointmentStoreChangeTracker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentStoreChangeTracker").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppointmentStoreChangeTracker {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentStoreChangeTracker;{1b25f4b1-8ece-4f17-93c8-e6412458fd5c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppointmentStoreChangeTracker {
    type Vtable = IAppointmentStoreChangeTracker_Vtbl;
    const IID: ::windows_core::GUID = <IAppointmentStoreChangeTracker as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentStoreChangeTracker {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentStoreChangeTracker";
}
impl ::core::convert::From<AppointmentStoreChangeTracker> for ::windows_core::IUnknown {
    fn from(value: AppointmentStoreChangeTracker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentStoreChangeTracker> for ::windows_core::IUnknown {
    fn from(value: &AppointmentStoreChangeTracker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppointmentStoreChangeTracker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppointmentStoreChangeTracker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentStoreChangeTracker> for ::windows_core::IInspectable {
    fn from(value: AppointmentStoreChangeTracker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentStoreChangeTracker> for ::windows_core::IInspectable {
    fn from(value: &AppointmentStoreChangeTracker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppointmentStoreChangeTracker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppointmentStoreChangeTracker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentStoreChangeTracker {}
unsafe impl ::core::marker::Sync for AppointmentStoreChangeTracker {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for AppointmentStoreChangeType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppointmentStoreChangeType {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppointmentStoreChangeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentStoreChangeType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppointmentStoreChangeType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentStoreChangeType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AppointmentStoreChangedDeferral(::windows_core::IUnknown);
impl AppointmentStoreChangedDeferral {
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for AppointmentStoreChangedDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentStoreChangedDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentStoreChangedDeferral {}
impl ::core::fmt::Debug for AppointmentStoreChangedDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentStoreChangedDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppointmentStoreChangedDeferral {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentStoreChangedDeferral;{4cb82026-fedb-4bc3-9662-95a9befdf4df})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppointmentStoreChangedDeferral {
    type Vtable = IAppointmentStoreChangedDeferral_Vtbl;
    const IID: ::windows_core::GUID = <IAppointmentStoreChangedDeferral as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentStoreChangedDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentStoreChangedDeferral";
}
impl ::core::convert::From<AppointmentStoreChangedDeferral> for ::windows_core::IUnknown {
    fn from(value: AppointmentStoreChangedDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentStoreChangedDeferral> for ::windows_core::IUnknown {
    fn from(value: &AppointmentStoreChangedDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppointmentStoreChangedDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppointmentStoreChangedDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentStoreChangedDeferral> for ::windows_core::IInspectable {
    fn from(value: AppointmentStoreChangedDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentStoreChangedDeferral> for ::windows_core::IInspectable {
    fn from(value: &AppointmentStoreChangedDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppointmentStoreChangedDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppointmentStoreChangedDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentStoreChangedDeferral {}
unsafe impl ::core::marker::Sync for AppointmentStoreChangedDeferral {}
#[repr(transparent)]
pub struct AppointmentStoreChangedEventArgs(::windows_core::IUnknown);
impl AppointmentStoreChangedEventArgs {
    pub fn GetDeferral(&self) -> ::windows_core::Result<AppointmentStoreChangedDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppointmentStoreChangedDeferral>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentStoreChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentStoreChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentStoreChangedEventArgs {}
impl ::core::fmt::Debug for AppointmentStoreChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentStoreChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppointmentStoreChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentStoreChangedEventArgs;{2285f8b9-0791-417e-bfea-cc6d41636c8c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppointmentStoreChangedEventArgs {
    type Vtable = IAppointmentStoreChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppointmentStoreChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentStoreChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentStoreChangedEventArgs";
}
impl ::core::convert::From<AppointmentStoreChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AppointmentStoreChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentStoreChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AppointmentStoreChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppointmentStoreChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppointmentStoreChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentStoreChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AppointmentStoreChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentStoreChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AppointmentStoreChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppointmentStoreChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppointmentStoreChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentStoreChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentStoreChangedEventArgs {}
#[repr(transparent)]
pub struct AppointmentStoreNotificationTriggerDetails(::windows_core::IUnknown);
impl AppointmentStoreNotificationTriggerDetails {}
impl ::core::clone::Clone for AppointmentStoreNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentStoreNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentStoreNotificationTriggerDetails {}
impl ::core::fmt::Debug for AppointmentStoreNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentStoreNotificationTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppointmentStoreNotificationTriggerDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentStoreNotificationTriggerDetails;{9b33cb11-c301-421e-afef-047ecfa76adb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppointmentStoreNotificationTriggerDetails {
    type Vtable = IAppointmentStoreNotificationTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = <IAppointmentStoreNotificationTriggerDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentStoreNotificationTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentStoreNotificationTriggerDetails";
}
impl ::core::convert::From<AppointmentStoreNotificationTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: AppointmentStoreNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentStoreNotificationTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: &AppointmentStoreNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppointmentStoreNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppointmentStoreNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentStoreNotificationTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: AppointmentStoreNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentStoreNotificationTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: &AppointmentStoreNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppointmentStoreNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppointmentStoreNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentStoreNotificationTriggerDetails {}
unsafe impl ::core::marker::Sync for AppointmentStoreNotificationTriggerDetails {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for AppointmentSummaryCardView {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppointmentSummaryCardView {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppointmentSummaryCardView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentSummaryCardView").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppointmentSummaryCardView {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentSummaryCardView;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for AppointmentWeekOfMonth {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppointmentWeekOfMonth {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppointmentWeekOfMonth {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentWeekOfMonth").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppointmentWeekOfMonth {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentWeekOfMonth;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for FindAppointmentCalendarsOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FindAppointmentCalendarsOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for FindAppointmentCalendarsOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FindAppointmentCalendarsOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for FindAppointmentCalendarsOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FindAppointmentCalendarsOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FindAppointmentCalendarsOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FindAppointmentCalendarsOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FindAppointmentCalendarsOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for FindAppointmentCalendarsOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.FindAppointmentCalendarsOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct FindAppointmentsOptions(::windows_core::IUnknown);
impl FindAppointmentsOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<FindAppointmentsOptions, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn CalendarIds(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CalendarIds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn FetchProperties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FetchProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn IncludeHidden(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IncludeHidden)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIncludeHidden(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIncludeHidden)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetMaxCount(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxCount)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for FindAppointmentsOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FindAppointmentsOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FindAppointmentsOptions {}
impl ::core::fmt::Debug for FindAppointmentsOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FindAppointmentsOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FindAppointmentsOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.FindAppointmentsOptions;{55f7dc55-9942-3086-82b5-2cb29f64d5f5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FindAppointmentsOptions {
    type Vtable = IFindAppointmentsOptions_Vtbl;
    const IID: ::windows_core::GUID = <IFindAppointmentsOptions as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FindAppointmentsOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.FindAppointmentsOptions";
}
impl ::core::convert::From<FindAppointmentsOptions> for ::windows_core::IUnknown {
    fn from(value: FindAppointmentsOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FindAppointmentsOptions> for ::windows_core::IUnknown {
    fn from(value: &FindAppointmentsOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FindAppointmentsOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FindAppointmentsOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FindAppointmentsOptions> for ::windows_core::IInspectable {
    fn from(value: FindAppointmentsOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FindAppointmentsOptions> for ::windows_core::IInspectable {
    fn from(value: &FindAppointmentsOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FindAppointmentsOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FindAppointmentsOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for FindAppointmentsOptions {}
unsafe impl ::core::marker::Sync for FindAppointmentsOptions {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointment {
    type Vtable = IAppointment_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdd002f2f_2bdd_4076_90a3_22c275312965);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointment_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub SetStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Subject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetSubject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Details: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Reminder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetReminder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Organizer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetOrganizer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Invitees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Invitees: usize,
    pub Recurrence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetRecurrence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub BusyStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentBusyStatus) -> ::windows_core::HRESULT,
    pub SetBusyStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentBusyStatus) -> ::windows_core::HRESULT,
    pub AllDay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllDay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Sensitivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentSensitivity) -> ::windows_core::HRESULT,
    pub SetSensitivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentSensitivity) -> ::windows_core::HRESULT,
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointment2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointment2 {
    type Vtable = IAppointment2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5e85983c_540f_3452_9b5c_0dd7ad4c65a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointment2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LocalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CalendarId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RoamingId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetRoamingId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub OriginalStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsResponseRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsResponseRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub AllowNewTimeProposal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllowNewTimeProposal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub OnlineMeetingLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetOnlineMeetingLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ReplyTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetReplyTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub UserResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentParticipantResponse) -> ::windows_core::HRESULT,
    pub SetUserResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentParticipantResponse) -> ::windows_core::HRESULT,
    pub HasInvitees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsCanceledMeeting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsCanceledMeeting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsOrganizedByUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsOrganizedByUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointment3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointment3 {
    type Vtable = IAppointment3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbfcc45a9_8961_4991_934b_c48768e5a96c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointment3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ChangeNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub RemoteChangeNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub SetRemoteChangeNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64) -> ::windows_core::HRESULT,
    pub DetailsKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentDetailsKind) -> ::windows_core::HRESULT,
    pub SetDetailsKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentDetailsKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentCalendar(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentCalendar {
    type Vtable = IAppointmentCalendar_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5273819d_8339_3d4f_a02f_64084452bb5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendar_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-ui")]
    pub DisplayColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    DisplayColor: usize,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LocalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsHidden: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub OtherAppReadAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentCalendarOtherAppReadAccess) -> ::windows_core::HRESULT,
    pub SetOtherAppReadAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentCalendarOtherAppReadAccess) -> ::windows_core::HRESULT,
    pub OtherAppWriteAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentCalendarOtherAppWriteAccess) -> ::windows_core::HRESULT,
    pub SetOtherAppWriteAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentCalendarOtherAppWriteAccess) -> ::windows_core::HRESULT,
    pub SourceDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SummaryCardView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentSummaryCardView) -> ::windows_core::HRESULT,
    pub SetSummaryCardView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentSummaryCardView) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub FindAppointmentsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rangestart: ::winrt_foundation::DateTime, rangelength: ::winrt_foundation::TimeSpan, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    FindAppointmentsAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub FindAppointmentsAsyncWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rangestart: ::winrt_foundation::DateTime, rangelength: ::winrt_foundation::TimeSpan, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    FindAppointmentsAsyncWithOptions: usize,
    #[cfg(feature = "winrt-foundation")]
    pub FindExceptionsFromMasterAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, masterlocalid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    FindExceptionsFromMasterAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub FindAllInstancesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, masterlocalid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, rangestart: ::winrt_foundation::DateTime, rangelength: ::winrt_foundation::TimeSpan, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    FindAllInstancesAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub FindAllInstancesAsyncWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, masterlocalid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, rangestart: ::winrt_foundation::DateTime, rangelength: ::winrt_foundation::TimeSpan, poptions: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    FindAllInstancesAsyncWithOptions: usize,
    pub GetAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetAppointmentInstanceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, instancestarttime: ::winrt_foundation::DateTime, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub FindUnexpandedAppointmentsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    FindUnexpandedAppointmentsAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub FindUnexpandedAppointmentsAsyncWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    FindUnexpandedAppointmentsAsyncWithOptions: usize,
    pub DeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SaveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DeleteAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DeleteAppointmentInstanceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, instancestarttime: ::winrt_foundation::DateTime, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SaveAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pappointment: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentCalendar2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentCalendar2 {
    type Vtable = IAppointmentCalendar2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x18e7e422_2467_4e1c_a459_d8a29303d092);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendar2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SyncManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub SetDisplayColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    SetDisplayColor: usize,
    pub SetIsHidden: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub UserDataAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CanCreateOrUpdateAppointments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCanCreateOrUpdateAppointments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CanCancelMeetings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCanCancelMeetings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CanForwardMeetings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCanForwardMeetings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CanProposeNewTimeForMeetings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCanProposeNewTimeForMeetings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CanUpdateMeetingResponses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCanUpdateMeetingResponses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CanNotifyInvitees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCanNotifyInvitees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub MustNofityInvitees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetMustNofityInvitees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub TryCreateOrUpdateAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointment: ::windows_core::RawPtr, notifyinvitees: bool, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryCancelMeetingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, meeting: ::windows_core::RawPtr, subject: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, comment: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, notifyinvitees: bool, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub TryForwardMeetingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, meeting: ::windows_core::RawPtr, invitees: ::windows_core::RawPtr, subject: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, forwardheader: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, comment: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    TryForwardMeetingAsync: usize,
    pub TryProposeNewTimeForMeetingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, meeting: ::windows_core::RawPtr, newstarttime: ::winrt_foundation::DateTime, newduration: ::winrt_foundation::TimeSpan, subject: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, comment: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryUpdateMeetingResponseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, meeting: ::windows_core::RawPtr, response: AppointmentParticipantResponse, subject: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, comment: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, sendupdate: bool, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentCalendar3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentCalendar3 {
    type Vtable = IAppointmentCalendar3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeb23d22b_a685_42ae_8495_b3119adb4167);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendar3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RegisterSyncManagerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentCalendarSyncManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentCalendarSyncManager {
    type Vtable = IAppointmentCalendarSyncManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2b21b3a0_4aff_4392_bc5f_5645ffcffb17);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendarSyncManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentCalendarSyncStatus) -> ::windows_core::HRESULT,
    pub LastSuccessfulSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub LastAttemptedSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub SyncAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SyncStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSyncStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentCalendarSyncManager2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentCalendarSyncManager2 {
    type Vtable = IAppointmentCalendarSyncManager2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x647528ad_0d29_4c7c_aaa7_bf996805537c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendarSyncManager2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentCalendarSyncStatus) -> ::windows_core::HRESULT,
    pub SetLastSuccessfulSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub SetLastAttemptedSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentConflictResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentConflictResult {
    type Vtable = IAppointmentConflictResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd5cdf0be_2f2f_3b7d_af0a_a7e20f3a46e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentConflictResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentConflictType) -> ::windows_core::HRESULT,
    pub Date: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentException(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentException {
    type Vtable = IAppointmentException_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa2076767_16f6_4bce_9f5a_8600b8019fcb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentException_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Appointment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub ExceptionProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ExceptionProperties: usize,
    pub IsDeleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentInvitee(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentInvitee {
    type Vtable = IAppointmentInvitee_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x13bf0796_9842_495b_b0e7_ef8f79c0701d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentInvitee_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Role: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentParticipantRole) -> ::windows_core::HRESULT,
    pub SetRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentParticipantRole) -> ::windows_core::HRESULT,
    pub Response: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentParticipantResponse) -> ::windows_core::HRESULT,
    pub SetResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentParticipantResponse) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentManagerForUser(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentManagerForUser {
    type Vtable = IAppointmentManagerForUser_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x70261423_73cc_4660_b318_b01365302a03);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentManagerForUser_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ShowAddAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointment: ::windows_core::RawPtr, selection: ::winrt_foundation::Rect, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub ShowAddAppointmentWithPlacementAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointment: ::windows_core::RawPtr, selection: ::winrt_foundation::Rect, preferredplacement: ::winrt_ui::Popups::Placement, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    ShowAddAppointmentWithPlacementAsync: usize,
    pub ShowReplaceAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, appointment: ::windows_core::RawPtr, selection: ::winrt_foundation::Rect, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub ShowReplaceAppointmentWithPlacementAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, appointment: ::windows_core::RawPtr, selection: ::winrt_foundation::Rect, preferredplacement: ::winrt_ui::Popups::Placement, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    ShowReplaceAppointmentWithPlacementAsync: usize,
    #[cfg(feature = "winrt-ui")]
    pub ShowReplaceAppointmentWithPlacementAndDateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, appointment: ::windows_core::RawPtr, selection: ::winrt_foundation::Rect, preferredplacement: ::winrt_ui::Popups::Placement, instancestartdate: ::winrt_foundation::DateTime, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    ShowReplaceAppointmentWithPlacementAndDateAsync: usize,
    pub ShowRemoveAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, selection: ::winrt_foundation::Rect, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub ShowRemoveAppointmentWithPlacementAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, selection: ::winrt_foundation::Rect, preferredplacement: ::winrt_ui::Popups::Placement, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    ShowRemoveAppointmentWithPlacementAsync: usize,
    #[cfg(feature = "winrt-ui")]
    pub ShowRemoveAppointmentWithPlacementAndDateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, selection: ::winrt_foundation::Rect, preferredplacement: ::winrt_ui::Popups::Placement, instancestartdate: ::winrt_foundation::DateTime, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    ShowRemoveAppointmentWithPlacementAndDateAsync: usize,
    pub ShowTimeFrameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timetoshow: ::winrt_foundation::DateTime, duration: ::winrt_foundation::TimeSpan, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ShowAppointmentDetailsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ShowAppointmentDetailsWithDateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, instancestartdate: ::winrt_foundation::DateTime, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ShowEditNewAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointment: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: AppointmentStoreAccessType, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-system")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentManagerStatics {
    type Vtable = IAppointmentManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3a30fa01_5c40_499d_b33f_a43050f74fc4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ShowAddAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointment: ::windows_core::RawPtr, selection: ::winrt_foundation::Rect, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub ShowAddAppointmentWithPlacementAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointment: ::windows_core::RawPtr, selection: ::winrt_foundation::Rect, preferredplacement: ::winrt_ui::Popups::Placement, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    ShowAddAppointmentWithPlacementAsync: usize,
    pub ShowReplaceAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, appointment: ::windows_core::RawPtr, selection: ::winrt_foundation::Rect, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub ShowReplaceAppointmentWithPlacementAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, appointment: ::windows_core::RawPtr, selection: ::winrt_foundation::Rect, preferredplacement: ::winrt_ui::Popups::Placement, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    ShowReplaceAppointmentWithPlacementAsync: usize,
    #[cfg(feature = "winrt-ui")]
    pub ShowReplaceAppointmentWithPlacementAndDateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, appointment: ::windows_core::RawPtr, selection: ::winrt_foundation::Rect, preferredplacement: ::winrt_ui::Popups::Placement, instancestartdate: ::winrt_foundation::DateTime, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    ShowReplaceAppointmentWithPlacementAndDateAsync: usize,
    pub ShowRemoveAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, selection: ::winrt_foundation::Rect, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub ShowRemoveAppointmentWithPlacementAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, selection: ::winrt_foundation::Rect, preferredplacement: ::winrt_ui::Popups::Placement, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    ShowRemoveAppointmentWithPlacementAsync: usize,
    #[cfg(feature = "winrt-ui")]
    pub ShowRemoveAppointmentWithPlacementAndDateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, selection: ::winrt_foundation::Rect, preferredplacement: ::winrt_ui::Popups::Placement, instancestartdate: ::winrt_foundation::DateTime, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    ShowRemoveAppointmentWithPlacementAndDateAsync: usize,
    pub ShowTimeFrameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timetoshow: ::winrt_foundation::DateTime, duration: ::winrt_foundation::TimeSpan, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentManagerStatics2 {
    type Vtable = IAppointmentManagerStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0a81f60d_d04f_4034_af72_a36573b45ff0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ShowAppointmentDetailsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ShowAppointmentDetailsWithDateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, instancestartdate: ::winrt_foundation::DateTime, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ShowEditNewAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointment: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: AppointmentStoreAccessType, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentManagerStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentManagerStatics3 {
    type Vtable = IAppointmentManagerStatics3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2f9ae09c_b34c_4dc7_a35d_cafd88ae3ec6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentManagerStatics3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-system")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    GetForUser: usize,
}
#[repr(transparent)]
pub struct IAppointmentParticipant(::windows_core::IUnknown);
impl IAppointmentParticipant {
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetDisplayName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Address(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Address)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetAddress<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAddress)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<IAppointmentParticipant> for ::windows_core::IUnknown {
    fn from(value: IAppointmentParticipant) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentParticipant> for ::windows_core::IUnknown {
    fn from(value: &IAppointmentParticipant) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAppointmentParticipant {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAppointmentParticipant {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAppointmentParticipant> for ::windows_core::IInspectable {
    fn from(value: IAppointmentParticipant) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentParticipant> for ::windows_core::IInspectable {
    fn from(value: &IAppointmentParticipant) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IAppointmentParticipant {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IAppointmentParticipant {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAppointmentParticipant {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppointmentParticipant {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppointmentParticipant {}
impl ::core::fmt::Debug for IAppointmentParticipant {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppointmentParticipant").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IAppointmentParticipant {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{615e2902-9718-467b-83fb-b293a19121de}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IAppointmentParticipant {
    type Vtable = IAppointmentParticipant_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x615e2902_9718_467b_83fb_b293a19121de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentParticipant_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Address: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentPropertiesStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentPropertiesStatics {
    type Vtable = IAppointmentPropertiesStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x25141fe9_68ae_3aae_855f_bc4441caa234);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentPropertiesStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Subject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Reminder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BusyStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Sensitivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub OriginalStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsResponseRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AllowNewTimeProposal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AllDay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Details: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub OnlineMeetingLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ReplyTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Organizer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub UserResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub HasInvitees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsCanceledMeeting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsOrganizedByUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Recurrence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Invitees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub DefaultProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    DefaultProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentPropertiesStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentPropertiesStatics2 {
    type Vtable = IAppointmentPropertiesStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdffc434b_b017_45dd_8af5_d163d10801bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentPropertiesStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ChangeNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RemoteChangeNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DetailsKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentRecurrence(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentRecurrence {
    type Vtable = IAppointmentRecurrence_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd87b3e83_15a6_487b_b959_0c361e60e954);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentRecurrence_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Unit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentRecurrenceUnit) -> ::windows_core::HRESULT,
    pub SetUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentRecurrenceUnit) -> ::windows_core::HRESULT,
    pub Occurrences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetOccurrences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Until: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetUntil: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Interval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub DaysOfWeek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentDaysOfWeek) -> ::windows_core::HRESULT,
    pub SetDaysOfWeek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentDaysOfWeek) -> ::windows_core::HRESULT,
    pub WeekOfMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentWeekOfMonth) -> ::windows_core::HRESULT,
    pub SetWeekOfMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentWeekOfMonth) -> ::windows_core::HRESULT,
    pub Month: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub Day: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetDay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentRecurrence2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentRecurrence2 {
    type Vtable = IAppointmentRecurrence2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3df3a2e0_05a7_4f50_9f86_b03f9436254d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentRecurrence2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RecurrenceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RecurrenceType) -> ::windows_core::HRESULT,
    pub TimeZone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTimeZone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentRecurrence3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentRecurrence3 {
    type Vtable = IAppointmentRecurrence3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x89ff96d9_da4d_4a17_8dd2_1cebc2b5ff9d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentRecurrence3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CalendarIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentStore(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentStore {
    type Vtable = IAppointmentStore_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa461918c_7a47_4d96_96c9_15cd8a05a735);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStore_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ChangeTracker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateAppointmentCalendarAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetAppointmentCalendarAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, calendarid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetAppointmentInstanceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, instancestarttime: ::winrt_foundation::DateTime, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub FindAppointmentCalendarsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    FindAppointmentCalendarsAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub FindAppointmentCalendarsAsyncWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: FindAppointmentCalendarsOptions, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    FindAppointmentCalendarsAsyncWithOptions: usize,
    #[cfg(feature = "winrt-foundation")]
    pub FindAppointmentsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rangestart: ::winrt_foundation::DateTime, rangelength: ::winrt_foundation::TimeSpan, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    FindAppointmentsAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub FindAppointmentsAsyncWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rangestart: ::winrt_foundation::DateTime, rangelength: ::winrt_foundation::TimeSpan, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    FindAppointmentsAsyncWithOptions: usize,
    pub FindConflictAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointment: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FindConflictAsyncWithInstanceStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointment: ::windows_core::RawPtr, instancestarttime: ::winrt_foundation::DateTime, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MoveAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointment: ::windows_core::RawPtr, destinationcalendar: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ShowAddAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointment: ::windows_core::RawPtr, selection: ::winrt_foundation::Rect, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ShowReplaceAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, appointment: ::windows_core::RawPtr, selection: ::winrt_foundation::Rect, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub ShowReplaceAppointmentWithPlacementAndDateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, appointment: ::windows_core::RawPtr, selection: ::winrt_foundation::Rect, preferredplacement: ::winrt_ui::Popups::Placement, instancestartdate: ::winrt_foundation::DateTime, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    ShowReplaceAppointmentWithPlacementAndDateAsync: usize,
    pub ShowRemoveAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, selection: ::winrt_foundation::Rect, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub ShowRemoveAppointmentWithPlacementAndDateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, selection: ::winrt_foundation::Rect, preferredplacement: ::winrt_ui::Popups::Placement, instancestartdate: ::winrt_foundation::DateTime, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    ShowRemoveAppointmentWithPlacementAndDateAsync: usize,
    pub ShowAppointmentDetailsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ShowAppointmentDetailsWithDateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, instancestartdate: ::winrt_foundation::DateTime, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ShowEditNewAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointment: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub FindLocalIdsFromRoamingIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, roamingid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    FindLocalIdsFromRoamingIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentStore2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentStore2 {
    type Vtable = IAppointmentStore2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x25c48c20_1c41_424f_8084_67c1cfe0a854);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStore2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StoreChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phandler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveStoreChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub CreateAppointmentCalendarInAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, userdataaccountid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentStore3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentStore3 {
    type Vtable = IAppointmentStore3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4251940b_b078_470a_9a40_c2e01761f72f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStore3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetChangeTracker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentStoreChange(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentStoreChange {
    type Vtable = IAppointmentStoreChange_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa5a6e035_0a33_3654_8463_b543e90c3b79);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStoreChange_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Appointment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ChangeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentStoreChangeType) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentStoreChange2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentStoreChange2 {
    type Vtable = IAppointmentStoreChange2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb37d0dce_5211_4402_a608_a96fe70b8ee2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStoreChange2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AppointmentCalendar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentStoreChangeReader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentStoreChangeReader {
    type Vtable = IAppointmentStoreChangeReader_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8b2409f1_65f3_42a0_961d_4c209bf30370);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStoreChangeReader_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ReadBatchAsync: usize,
    pub AcceptChanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AcceptChangesThrough: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastchangetoaccept: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentStoreChangeTracker(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentStoreChangeTracker {
    type Vtable = IAppointmentStoreChangeTracker_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1b25f4b1_8ece_4f17_93c8_e6412458fd5c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStoreChangeTracker_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetChangeReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Enable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentStoreChangeTracker2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentStoreChangeTracker2 {
    type Vtable = IAppointmentStoreChangeTracker2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb66aaf45_9542_4cf7_8550_eb370e0c08d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStoreChangeTracker2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsTracking: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentStoreChangedDeferral(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentStoreChangedDeferral {
    type Vtable = IAppointmentStoreChangedDeferral_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4cb82026_fedb_4bc3_9662_95a9befdf4df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStoreChangedDeferral_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentStoreChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentStoreChangedEventArgs {
    type Vtable = IAppointmentStoreChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2285f8b9_0791_417e_bfea_cc6d41636c8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStoreChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentStoreNotificationTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentStoreNotificationTriggerDetails {
    type Vtable = IAppointmentStoreNotificationTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9b33cb11_c301_421e_afef_047ecfa76adb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStoreNotificationTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFindAppointmentsOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFindAppointmentsOptions {
    type Vtable = IFindAppointmentsOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x55f7dc55_9942_3086_82b5_2cb29f64d5f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFindAppointmentsOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub CalendarIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    CalendarIds: usize,
    #[cfg(feature = "winrt-foundation")]
    pub FetchProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    FetchProperties: usize,
    pub IncludeHidden: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIncludeHidden: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub MaxCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetMaxCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for RecurrenceType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RecurrenceType {
    type Abi = Self;
}
impl ::core::fmt::Debug for RecurrenceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RecurrenceType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RecurrenceType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.RecurrenceType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
