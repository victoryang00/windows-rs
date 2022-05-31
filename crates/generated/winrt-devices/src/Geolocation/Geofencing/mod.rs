#[repr(transparent)]
pub struct Geofence(::windows_core::IUnknown);
impl Geofence {
    pub fn StartTime(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn Duration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn DwellTime(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).DwellTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn MonitoredStates(&self) -> ::windows_core::Result<MonitoredGeofenceStates> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MonitoredGeofenceStates>::zeroed();
            (::windows_core::Interface::vtable(this).MonitoredStates)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MonitoredGeofenceStates>(result__)
        }
    }
    pub fn Geoshape(&self) -> ::windows_core::Result<super::IGeoshape> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Geoshape)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::IGeoshape>(result__)
        }
    }
    pub fn SingleUse(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).SingleUse)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::IGeoshape>>(id: Param0, geoshape: Param1) -> ::windows_core::Result<Geofence> {
        Self::IGeofenceFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), id.into_param().abi(), geoshape.into_param().abi(), result__.as_mut_ptr()).from_abi::<Geofence>(result__)
        })
    }
    pub fn CreateWithMonitorStates<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::IGeoshape>>(id: Param0, geoshape: Param1, monitoredstates: MonitoredGeofenceStates, singleuse: bool) -> ::windows_core::Result<Geofence> {
        Self::IGeofenceFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithMonitorStates)(::windows_core::Interface::as_raw(this), id.into_param().abi(), geoshape.into_param().abi(), monitoredstates, singleuse, result__.as_mut_ptr()).from_abi::<Geofence>(result__)
        })
    }
    pub fn CreateWithMonitorStatesAndDwellTime<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::IGeoshape>, Param4: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(id: Param0, geoshape: Param1, monitoredstates: MonitoredGeofenceStates, singleuse: bool, dwelltime: Param4) -> ::windows_core::Result<Geofence> {
        Self::IGeofenceFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithMonitorStatesAndDwellTime)(::windows_core::Interface::as_raw(this), id.into_param().abi(), geoshape.into_param().abi(), monitoredstates, singleuse, dwelltime.into_param().abi(), result__.as_mut_ptr()).from_abi::<Geofence>(result__)
        })
    }
    pub fn CreateWithMonitorStatesDwellTimeStartTimeAndDuration<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::IGeoshape>, Param4: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>, Param5: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>, Param6: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(id: Param0, geoshape: Param1, monitoredstates: MonitoredGeofenceStates, singleuse: bool, dwelltime: Param4, starttime: Param5, duration: Param6) -> ::windows_core::Result<Geofence> {
        Self::IGeofenceFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithMonitorStatesDwellTimeStartTimeAndDuration)(::windows_core::Interface::as_raw(this), id.into_param().abi(), geoshape.into_param().abi(), monitoredstates, singleuse, dwelltime.into_param().abi(), starttime.into_param().abi(), duration.into_param().abi(), result__.as_mut_ptr()).from_abi::<Geofence>(result__)
        })
    }
    pub fn IGeofenceFactory<R, F: FnOnce(&IGeofenceFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Geofence, IGeofenceFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Geofence {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Geofence {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Geofence {}
impl ::core::fmt::Debug for Geofence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Geofence").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Geofence {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geofencing.Geofence;{9c090823-edb8-47e0-8245-5bf61d321f2d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Geofence {
    type Vtable = IGeofence_Vtbl;
    const IID: ::windows_core::GUID = <IGeofence as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Geofence {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geofencing.Geofence";
}
impl ::core::convert::From<Geofence> for ::windows_core::IUnknown {
    fn from(value: Geofence) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Geofence> for ::windows_core::IUnknown {
    fn from(value: &Geofence) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Geofence {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Geofence {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Geofence> for ::windows_core::IInspectable {
    fn from(value: Geofence) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Geofence> for ::windows_core::IInspectable {
    fn from(value: &Geofence) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Geofence {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Geofence {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Geofence {}
unsafe impl ::core::marker::Sync for Geofence {}
#[repr(transparent)]
pub struct GeofenceMonitor(::windows_core::IUnknown);
impl GeofenceMonitor {
    pub fn Status(&self) -> ::windows_core::Result<GeofenceMonitorStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GeofenceMonitorStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GeofenceMonitorStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Geofences(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<Geofence>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Geofences)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<Geofence>>(result__)
        }
    }
    pub fn LastKnownGeoposition(&self) -> ::windows_core::Result<super::Geoposition> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LastKnownGeoposition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Geoposition>(result__)
        }
    }
    pub fn GeofenceStateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<GeofenceMonitor, ::windows_core::IInspectable>>>(&self, eventhandler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).GeofenceStateChanged)(::windows_core::Interface::as_raw(this), eventhandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveGeofenceStateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveGeofenceStateChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadReports(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<GeofenceStateChangeReport>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReadReports)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<GeofenceStateChangeReport>>(result__)
        }
    }
    pub fn StatusChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<GeofenceMonitor, ::windows_core::IInspectable>>>(&self, eventhandler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).StatusChanged)(::windows_core::Interface::as_raw(this), eventhandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveStatusChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStatusChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Current() -> ::windows_core::Result<GeofenceMonitor> {
        Self::IGeofenceMonitorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Current)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GeofenceMonitor>(result__)
        })
    }
    pub fn IGeofenceMonitorStatics<R, F: FnOnce(&IGeofenceMonitorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<GeofenceMonitor, IGeofenceMonitorStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for GeofenceMonitor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GeofenceMonitor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GeofenceMonitor {}
impl ::core::fmt::Debug for GeofenceMonitor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeofenceMonitor").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GeofenceMonitor {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geofencing.GeofenceMonitor;{4c0f5f78-1c1f-4621-bbbd-833b92247226})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GeofenceMonitor {
    type Vtable = IGeofenceMonitor_Vtbl;
    const IID: ::windows_core::GUID = <IGeofenceMonitor as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GeofenceMonitor {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geofencing.GeofenceMonitor";
}
impl ::core::convert::From<GeofenceMonitor> for ::windows_core::IUnknown {
    fn from(value: GeofenceMonitor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GeofenceMonitor> for ::windows_core::IUnknown {
    fn from(value: &GeofenceMonitor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GeofenceMonitor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GeofenceMonitor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GeofenceMonitor> for ::windows_core::IInspectable {
    fn from(value: GeofenceMonitor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GeofenceMonitor> for ::windows_core::IInspectable {
    fn from(value: &GeofenceMonitor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GeofenceMonitor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GeofenceMonitor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GeofenceMonitor {}
unsafe impl ::core::marker::Sync for GeofenceMonitor {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GeofenceMonitorStatus(pub i32);
impl GeofenceMonitorStatus {
    pub const Ready: Self = Self(0i32);
    pub const Initializing: Self = Self(1i32);
    pub const NoData: Self = Self(2i32);
    pub const Disabled: Self = Self(3i32);
    pub const NotInitialized: Self = Self(4i32);
    pub const NotAvailable: Self = Self(5i32);
}
impl ::core::marker::Copy for GeofenceMonitorStatus {}
impl ::core::clone::Clone for GeofenceMonitorStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GeofenceMonitorStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GeofenceMonitorStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for GeofenceMonitorStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeofenceMonitorStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GeofenceMonitorStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.Geofencing.GeofenceMonitorStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GeofenceRemovalReason(pub i32);
impl GeofenceRemovalReason {
    pub const Used: Self = Self(0i32);
    pub const Expired: Self = Self(1i32);
}
impl ::core::marker::Copy for GeofenceRemovalReason {}
impl ::core::clone::Clone for GeofenceRemovalReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GeofenceRemovalReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GeofenceRemovalReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for GeofenceRemovalReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeofenceRemovalReason").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GeofenceRemovalReason {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.Geofencing.GeofenceRemovalReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GeofenceState(pub u32);
impl GeofenceState {
    pub const None: Self = Self(0u32);
    pub const Entered: Self = Self(1u32);
    pub const Exited: Self = Self(2u32);
    pub const Removed: Self = Self(4u32);
}
impl ::core::marker::Copy for GeofenceState {}
impl ::core::clone::Clone for GeofenceState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GeofenceState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GeofenceState {
    type Abi = Self;
}
impl ::core::fmt::Debug for GeofenceState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeofenceState").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for GeofenceState {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GeofenceState {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GeofenceState {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GeofenceState {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GeofenceState {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for GeofenceState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.Geofencing.GeofenceState;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct GeofenceStateChangeReport(::windows_core::IUnknown);
impl GeofenceStateChangeReport {
    pub fn NewState(&self) -> ::windows_core::Result<GeofenceState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GeofenceState>::zeroed();
            (::windows_core::Interface::vtable(this).NewState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GeofenceState>(result__)
        }
    }
    pub fn Geofence(&self) -> ::windows_core::Result<Geofence> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Geofence)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Geofence>(result__)
        }
    }
    pub fn Geoposition(&self) -> ::windows_core::Result<super::Geoposition> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Geoposition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Geoposition>(result__)
        }
    }
    pub fn RemovalReason(&self) -> ::windows_core::Result<GeofenceRemovalReason> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GeofenceRemovalReason>::zeroed();
            (::windows_core::Interface::vtable(this).RemovalReason)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GeofenceRemovalReason>(result__)
        }
    }
}
impl ::core::clone::Clone for GeofenceStateChangeReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GeofenceStateChangeReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GeofenceStateChangeReport {}
impl ::core::fmt::Debug for GeofenceStateChangeReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeofenceStateChangeReport").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GeofenceStateChangeReport {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geofencing.GeofenceStateChangeReport;{9a243c18-2464-4c89-be05-b3ffff5babc5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GeofenceStateChangeReport {
    type Vtable = IGeofenceStateChangeReport_Vtbl;
    const IID: ::windows_core::GUID = <IGeofenceStateChangeReport as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GeofenceStateChangeReport {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geofencing.GeofenceStateChangeReport";
}
impl ::core::convert::From<GeofenceStateChangeReport> for ::windows_core::IUnknown {
    fn from(value: GeofenceStateChangeReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GeofenceStateChangeReport> for ::windows_core::IUnknown {
    fn from(value: &GeofenceStateChangeReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GeofenceStateChangeReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GeofenceStateChangeReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GeofenceStateChangeReport> for ::windows_core::IInspectable {
    fn from(value: GeofenceStateChangeReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GeofenceStateChangeReport> for ::windows_core::IInspectable {
    fn from(value: &GeofenceStateChangeReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GeofenceStateChangeReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GeofenceStateChangeReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GeofenceStateChangeReport {}
unsafe impl ::core::marker::Sync for GeofenceStateChangeReport {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeofence(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeofence {
    type Vtable = IGeofence_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9c090823_edb8_47e0_8245_5bf61d321f2d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeofence_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub DwellTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MonitoredStates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MonitoredGeofenceStates) -> ::windows_core::HRESULT,
    pub Geoshape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SingleUse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeofenceFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeofenceFactory {
    type Vtable = IGeofenceFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x841f624b_325f_4b90_bca7_2b8022a93796);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeofenceFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, geoshape: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateWithMonitorStates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, geoshape: ::windows_core::RawPtr, monitoredstates: MonitoredGeofenceStates, singleuse: bool, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateWithMonitorStatesAndDwellTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, geoshape: ::windows_core::RawPtr, monitoredstates: MonitoredGeofenceStates, singleuse: bool, dwelltime: ::winrt_foundation::TimeSpan, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateWithMonitorStatesDwellTimeStartTimeAndDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, geoshape: ::windows_core::RawPtr, monitoredstates: MonitoredGeofenceStates, singleuse: bool, dwelltime: ::winrt_foundation::TimeSpan, starttime: ::winrt_foundation::DateTime, duration: ::winrt_foundation::TimeSpan, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeofenceMonitor(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeofenceMonitor {
    type Vtable = IGeofenceMonitor_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4c0f5f78_1c1f_4621_bbbd_833b92247226);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeofenceMonitor_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GeofenceMonitorStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Geofences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Geofences: usize,
    pub LastKnownGeoposition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GeofenceStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveGeofenceStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadReports: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadReports: usize,
    pub StatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeofenceMonitorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeofenceMonitorStatics {
    type Vtable = IGeofenceMonitorStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2dd32fcf_7e75_4899_ace3_2bd0a65cce06);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeofenceMonitorStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeofenceStateChangeReport(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeofenceStateChangeReport {
    type Vtable = IGeofenceStateChangeReport_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9a243c18_2464_4c89_be05_b3ffff5babc5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeofenceStateChangeReport_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub NewState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GeofenceState) -> ::windows_core::HRESULT,
    pub Geofence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Geoposition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemovalReason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GeofenceRemovalReason) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MonitoredGeofenceStates(pub u32);
impl MonitoredGeofenceStates {
    pub const None: Self = Self(0u32);
    pub const Entered: Self = Self(1u32);
    pub const Exited: Self = Self(2u32);
    pub const Removed: Self = Self(4u32);
}
impl ::core::marker::Copy for MonitoredGeofenceStates {}
impl ::core::clone::Clone for MonitoredGeofenceStates {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MonitoredGeofenceStates {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MonitoredGeofenceStates {
    type Abi = Self;
}
impl ::core::fmt::Debug for MonitoredGeofenceStates {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MonitoredGeofenceStates").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MonitoredGeofenceStates {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MonitoredGeofenceStates {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MonitoredGeofenceStates {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MonitoredGeofenceStates {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MonitoredGeofenceStates {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for MonitoredGeofenceStates {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.Geofencing.MonitoredGeofenceStates;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
