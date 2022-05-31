#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GuidanceAudioMeasurementSystem(pub i32);
impl GuidanceAudioMeasurementSystem {
    pub const Meters: Self = Self(0i32);
    pub const MilesAndYards: Self = Self(1i32);
    pub const MilesAndFeet: Self = Self(2i32);
}
impl ::core::marker::Copy for GuidanceAudioMeasurementSystem {}
impl ::core::clone::Clone for GuidanceAudioMeasurementSystem {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GuidanceAudioMeasurementSystem {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GuidanceAudioMeasurementSystem {
    type Abi = Self;
}
impl ::core::fmt::Debug for GuidanceAudioMeasurementSystem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceAudioMeasurementSystem").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GuidanceAudioMeasurementSystem {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.Guidance.GuidanceAudioMeasurementSystem;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GuidanceAudioNotificationKind(pub i32);
impl GuidanceAudioNotificationKind {
    pub const Maneuver: Self = Self(0i32);
    pub const Route: Self = Self(1i32);
    pub const Gps: Self = Self(2i32);
    pub const SpeedLimit: Self = Self(3i32);
    pub const Traffic: Self = Self(4i32);
    pub const TrafficCamera: Self = Self(5i32);
}
impl ::core::marker::Copy for GuidanceAudioNotificationKind {}
impl ::core::clone::Clone for GuidanceAudioNotificationKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GuidanceAudioNotificationKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GuidanceAudioNotificationKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for GuidanceAudioNotificationKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceAudioNotificationKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GuidanceAudioNotificationKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.Guidance.GuidanceAudioNotificationKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct GuidanceAudioNotificationRequestedEventArgs(::windows_core::IUnknown);
impl GuidanceAudioNotificationRequestedEventArgs {
    pub fn AudioNotification(&self) -> ::windows_core::Result<GuidanceAudioNotificationKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GuidanceAudioNotificationKind>::zeroed();
            (::windows_core::Interface::vtable(this).AudioNotification)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GuidanceAudioNotificationKind>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn AudioFilePaths(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AudioFilePaths)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn AudioText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AudioText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for GuidanceAudioNotificationRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GuidanceAudioNotificationRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GuidanceAudioNotificationRequestedEventArgs {}
impl ::core::fmt::Debug for GuidanceAudioNotificationRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceAudioNotificationRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GuidanceAudioNotificationRequestedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceAudioNotificationRequestedEventArgs;{ca2aa24a-c7c2-4d4c-9d7c-499576bceddb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GuidanceAudioNotificationRequestedEventArgs {
    type Vtable = IGuidanceAudioNotificationRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IGuidanceAudioNotificationRequestedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GuidanceAudioNotificationRequestedEventArgs {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceAudioNotificationRequestedEventArgs";
}
impl ::core::convert::From<GuidanceAudioNotificationRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: GuidanceAudioNotificationRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GuidanceAudioNotificationRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &GuidanceAudioNotificationRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GuidanceAudioNotificationRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GuidanceAudioNotificationRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GuidanceAudioNotificationRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: GuidanceAudioNotificationRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GuidanceAudioNotificationRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &GuidanceAudioNotificationRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GuidanceAudioNotificationRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GuidanceAudioNotificationRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GuidanceAudioNotificationRequestedEventArgs {}
unsafe impl ::core::marker::Sync for GuidanceAudioNotificationRequestedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GuidanceAudioNotifications(pub u32);
impl GuidanceAudioNotifications {
    pub const None: Self = Self(0u32);
    pub const Maneuver: Self = Self(1u32);
    pub const Route: Self = Self(2u32);
    pub const Gps: Self = Self(4u32);
    pub const SpeedLimit: Self = Self(8u32);
    pub const Traffic: Self = Self(16u32);
    pub const TrafficCamera: Self = Self(32u32);
}
impl ::core::marker::Copy for GuidanceAudioNotifications {}
impl ::core::clone::Clone for GuidanceAudioNotifications {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GuidanceAudioNotifications {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GuidanceAudioNotifications {
    type Abi = Self;
}
impl ::core::fmt::Debug for GuidanceAudioNotifications {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceAudioNotifications").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for GuidanceAudioNotifications {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GuidanceAudioNotifications {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GuidanceAudioNotifications {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GuidanceAudioNotifications {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GuidanceAudioNotifications {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for GuidanceAudioNotifications {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.Guidance.GuidanceAudioNotifications;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct GuidanceLaneInfo(::windows_core::IUnknown);
impl GuidanceLaneInfo {
    pub fn LaneMarkers(&self) -> ::windows_core::Result<GuidanceLaneMarkers> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GuidanceLaneMarkers>::zeroed();
            (::windows_core::Interface::vtable(this).LaneMarkers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GuidanceLaneMarkers>(result__)
        }
    }
    pub fn IsOnRoute(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsOnRoute)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for GuidanceLaneInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GuidanceLaneInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GuidanceLaneInfo {}
impl ::core::fmt::Debug for GuidanceLaneInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceLaneInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GuidanceLaneInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceLaneInfo;{8404d114-6581-43b7-ac15-c9079bf90df1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GuidanceLaneInfo {
    type Vtable = IGuidanceLaneInfo_Vtbl;
    const IID: ::windows_core::GUID = <IGuidanceLaneInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GuidanceLaneInfo {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceLaneInfo";
}
impl ::core::convert::From<GuidanceLaneInfo> for ::windows_core::IUnknown {
    fn from(value: GuidanceLaneInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GuidanceLaneInfo> for ::windows_core::IUnknown {
    fn from(value: &GuidanceLaneInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GuidanceLaneInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GuidanceLaneInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GuidanceLaneInfo> for ::windows_core::IInspectable {
    fn from(value: GuidanceLaneInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GuidanceLaneInfo> for ::windows_core::IInspectable {
    fn from(value: &GuidanceLaneInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GuidanceLaneInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GuidanceLaneInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GuidanceLaneInfo {}
unsafe impl ::core::marker::Sync for GuidanceLaneInfo {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GuidanceLaneMarkers(pub u32);
impl GuidanceLaneMarkers {
    pub const None: Self = Self(0u32);
    pub const LightRight: Self = Self(1u32);
    pub const Right: Self = Self(2u32);
    pub const HardRight: Self = Self(4u32);
    pub const Straight: Self = Self(8u32);
    pub const UTurnLeft: Self = Self(16u32);
    pub const HardLeft: Self = Self(32u32);
    pub const Left: Self = Self(64u32);
    pub const LightLeft: Self = Self(128u32);
    pub const UTurnRight: Self = Self(256u32);
    pub const Unknown: Self = Self(4294967295u32);
}
impl ::core::marker::Copy for GuidanceLaneMarkers {}
impl ::core::clone::Clone for GuidanceLaneMarkers {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GuidanceLaneMarkers {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GuidanceLaneMarkers {
    type Abi = Self;
}
impl ::core::fmt::Debug for GuidanceLaneMarkers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceLaneMarkers").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for GuidanceLaneMarkers {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GuidanceLaneMarkers {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GuidanceLaneMarkers {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GuidanceLaneMarkers {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GuidanceLaneMarkers {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for GuidanceLaneMarkers {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.Guidance.GuidanceLaneMarkers;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct GuidanceManeuver(::windows_core::IUnknown);
impl GuidanceManeuver {
    #[cfg(feature = "winrt-devices")]
    pub fn StartLocation(&self) -> ::windows_core::Result<::winrt_devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartLocation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Geolocation::Geopoint>(result__)
        }
    }
    pub fn DistanceFromRouteStart(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).DistanceFromRouteStart)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn DistanceFromPreviousManeuver(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).DistanceFromPreviousManeuver)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn DepartureRoadName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DepartureRoadName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn NextRoadName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NextRoadName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn DepartureShortRoadName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DepartureShortRoadName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn NextShortRoadName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NextShortRoadName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<GuidanceManeuverKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GuidanceManeuverKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GuidanceManeuverKind>(result__)
        }
    }
    pub fn StartAngle(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).StartAngle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn EndAngle(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).EndAngle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn RoadSignpost(&self) -> ::windows_core::Result<GuidanceRoadSignpost> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RoadSignpost)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GuidanceRoadSignpost>(result__)
        }
    }
    pub fn InstructionText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).InstructionText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for GuidanceManeuver {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GuidanceManeuver {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GuidanceManeuver {}
impl ::core::fmt::Debug for GuidanceManeuver {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceManeuver").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GuidanceManeuver {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceManeuver;{fc09326c-ecc9-4928-a2a1-7232b99b94a1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GuidanceManeuver {
    type Vtable = IGuidanceManeuver_Vtbl;
    const IID: ::windows_core::GUID = <IGuidanceManeuver as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GuidanceManeuver {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceManeuver";
}
impl ::core::convert::From<GuidanceManeuver> for ::windows_core::IUnknown {
    fn from(value: GuidanceManeuver) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GuidanceManeuver> for ::windows_core::IUnknown {
    fn from(value: &GuidanceManeuver) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GuidanceManeuver {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GuidanceManeuver {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GuidanceManeuver> for ::windows_core::IInspectable {
    fn from(value: GuidanceManeuver) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GuidanceManeuver> for ::windows_core::IInspectable {
    fn from(value: &GuidanceManeuver) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GuidanceManeuver {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GuidanceManeuver {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GuidanceManeuver {}
unsafe impl ::core::marker::Sync for GuidanceManeuver {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GuidanceManeuverKind(pub i32);
impl GuidanceManeuverKind {
    pub const None: Self = Self(0i32);
    pub const GoStraight: Self = Self(1i32);
    pub const UTurnRight: Self = Self(2i32);
    pub const UTurnLeft: Self = Self(3i32);
    pub const TurnKeepRight: Self = Self(4i32);
    pub const TurnLightRight: Self = Self(5i32);
    pub const TurnRight: Self = Self(6i32);
    pub const TurnHardRight: Self = Self(7i32);
    pub const KeepMiddle: Self = Self(8i32);
    pub const TurnKeepLeft: Self = Self(9i32);
    pub const TurnLightLeft: Self = Self(10i32);
    pub const TurnLeft: Self = Self(11i32);
    pub const TurnHardLeft: Self = Self(12i32);
    pub const FreewayEnterRight: Self = Self(13i32);
    pub const FreewayEnterLeft: Self = Self(14i32);
    pub const FreewayLeaveRight: Self = Self(15i32);
    pub const FreewayLeaveLeft: Self = Self(16i32);
    pub const FreewayKeepRight: Self = Self(17i32);
    pub const FreewayKeepLeft: Self = Self(18i32);
    pub const TrafficCircleRight1: Self = Self(19i32);
    pub const TrafficCircleRight2: Self = Self(20i32);
    pub const TrafficCircleRight3: Self = Self(21i32);
    pub const TrafficCircleRight4: Self = Self(22i32);
    pub const TrafficCircleRight5: Self = Self(23i32);
    pub const TrafficCircleRight6: Self = Self(24i32);
    pub const TrafficCircleRight7: Self = Self(25i32);
    pub const TrafficCircleRight8: Self = Self(26i32);
    pub const TrafficCircleRight9: Self = Self(27i32);
    pub const TrafficCircleRight10: Self = Self(28i32);
    pub const TrafficCircleRight11: Self = Self(29i32);
    pub const TrafficCircleRight12: Self = Self(30i32);
    pub const TrafficCircleLeft1: Self = Self(31i32);
    pub const TrafficCircleLeft2: Self = Self(32i32);
    pub const TrafficCircleLeft3: Self = Self(33i32);
    pub const TrafficCircleLeft4: Self = Self(34i32);
    pub const TrafficCircleLeft5: Self = Self(35i32);
    pub const TrafficCircleLeft6: Self = Self(36i32);
    pub const TrafficCircleLeft7: Self = Self(37i32);
    pub const TrafficCircleLeft8: Self = Self(38i32);
    pub const TrafficCircleLeft9: Self = Self(39i32);
    pub const TrafficCircleLeft10: Self = Self(40i32);
    pub const TrafficCircleLeft11: Self = Self(41i32);
    pub const TrafficCircleLeft12: Self = Self(42i32);
    pub const Start: Self = Self(43i32);
    pub const End: Self = Self(44i32);
    pub const TakeFerry: Self = Self(45i32);
    pub const PassTransitStation: Self = Self(46i32);
    pub const LeaveTransitStation: Self = Self(47i32);
}
impl ::core::marker::Copy for GuidanceManeuverKind {}
impl ::core::clone::Clone for GuidanceManeuverKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GuidanceManeuverKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GuidanceManeuverKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for GuidanceManeuverKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceManeuverKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GuidanceManeuverKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.Guidance.GuidanceManeuverKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct GuidanceMapMatchedCoordinate(::windows_core::IUnknown);
impl GuidanceMapMatchedCoordinate {
    #[cfg(feature = "winrt-devices")]
    pub fn Location(&self) -> ::windows_core::Result<::winrt_devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Location)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Geolocation::Geopoint>(result__)
        }
    }
    pub fn CurrentHeading(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentHeading)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn CurrentSpeed(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentSpeed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn IsOnStreet(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsOnStreet)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Road(&self) -> ::windows_core::Result<GuidanceRoadSegment> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Road)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GuidanceRoadSegment>(result__)
        }
    }
}
impl ::core::clone::Clone for GuidanceMapMatchedCoordinate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GuidanceMapMatchedCoordinate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GuidanceMapMatchedCoordinate {}
impl ::core::fmt::Debug for GuidanceMapMatchedCoordinate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceMapMatchedCoordinate").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GuidanceMapMatchedCoordinate {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceMapMatchedCoordinate;{b7acb168-2912-4a99-aff1-798609b981fe})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GuidanceMapMatchedCoordinate {
    type Vtable = IGuidanceMapMatchedCoordinate_Vtbl;
    const IID: ::windows_core::GUID = <IGuidanceMapMatchedCoordinate as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GuidanceMapMatchedCoordinate {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceMapMatchedCoordinate";
}
impl ::core::convert::From<GuidanceMapMatchedCoordinate> for ::windows_core::IUnknown {
    fn from(value: GuidanceMapMatchedCoordinate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GuidanceMapMatchedCoordinate> for ::windows_core::IUnknown {
    fn from(value: &GuidanceMapMatchedCoordinate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GuidanceMapMatchedCoordinate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GuidanceMapMatchedCoordinate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GuidanceMapMatchedCoordinate> for ::windows_core::IInspectable {
    fn from(value: GuidanceMapMatchedCoordinate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GuidanceMapMatchedCoordinate> for ::windows_core::IInspectable {
    fn from(value: &GuidanceMapMatchedCoordinate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GuidanceMapMatchedCoordinate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GuidanceMapMatchedCoordinate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GuidanceMapMatchedCoordinate {}
unsafe impl ::core::marker::Sync for GuidanceMapMatchedCoordinate {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GuidanceMode(pub i32);
impl GuidanceMode {
    pub const None: Self = Self(0i32);
    pub const Simulation: Self = Self(1i32);
    pub const Navigation: Self = Self(2i32);
    pub const Tracking: Self = Self(3i32);
}
impl ::core::marker::Copy for GuidanceMode {}
impl ::core::clone::Clone for GuidanceMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GuidanceMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GuidanceMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for GuidanceMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GuidanceMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.Guidance.GuidanceMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct GuidanceNavigator(::windows_core::IUnknown);
impl GuidanceNavigator {
    pub fn StartNavigating<'a, Param0: ::windows_core::IntoParam<'a, GuidanceRoute>>(&self, route: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StartNavigating)(::windows_core::Interface::as_raw(this), route.into_param().abi()).ok() }
    }
    pub fn StartSimulating<'a, Param0: ::windows_core::IntoParam<'a, GuidanceRoute>>(&self, route: Param0, speedinmeterspersecond: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StartSimulating)(::windows_core::Interface::as_raw(this), route.into_param().abi(), speedinmeterspersecond).ok() }
    }
    pub fn StartTracking(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StartTracking)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Pause(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Pause)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Resume(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Resume)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn RepeatLastAudioNotification(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RepeatLastAudioNotification)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn AudioMeasurementSystem(&self) -> ::windows_core::Result<GuidanceAudioMeasurementSystem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GuidanceAudioMeasurementSystem>::zeroed();
            (::windows_core::Interface::vtable(this).AudioMeasurementSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GuidanceAudioMeasurementSystem>(result__)
        }
    }
    pub fn SetAudioMeasurementSystem(&self, value: GuidanceAudioMeasurementSystem) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAudioMeasurementSystem)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AudioNotifications(&self) -> ::windows_core::Result<GuidanceAudioNotifications> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GuidanceAudioNotifications>::zeroed();
            (::windows_core::Interface::vtable(this).AudioNotifications)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GuidanceAudioNotifications>(result__)
        }
    }
    pub fn SetAudioNotifications(&self, value: GuidanceAudioNotifications) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAudioNotifications)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GuidanceUpdated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<GuidanceNavigator, GuidanceUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).GuidanceUpdated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveGuidanceUpdated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveGuidanceUpdated)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn DestinationReached<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<GuidanceNavigator, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).DestinationReached)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDestinationReached<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDestinationReached)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Rerouting<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<GuidanceNavigator, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Rerouting)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveRerouting<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRerouting)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Rerouted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<GuidanceNavigator, GuidanceReroutedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Rerouted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveRerouted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRerouted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn RerouteFailed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<GuidanceNavigator, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).RerouteFailed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveRerouteFailed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRerouteFailed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn UserLocationLost<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<GuidanceNavigator, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).UserLocationLost)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveUserLocationLost<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUserLocationLost)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn UserLocationRestored<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<GuidanceNavigator, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).UserLocationRestored)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveUserLocationRestored<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUserLocationRestored)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn SetGuidanceVoice<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, voiceid: i32, voicefolder: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetGuidanceVoice)(::windows_core::Interface::as_raw(this), voiceid, voicefolder.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn UpdateUserLocation<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_devices::Geolocation::Geocoordinate>>(&self, userlocation: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).UpdateUserLocation)(::windows_core::Interface::as_raw(this), userlocation.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn UpdateUserLocationWithPositionOverride<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_devices::Geolocation::Geocoordinate>, Param1: ::windows_core::IntoParam<'a, ::winrt_devices::Geolocation::BasicGeoposition>>(&self, userlocation: Param0, positionoverride: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).UpdateUserLocationWithPositionOverride)(::windows_core::Interface::as_raw(this), userlocation.into_param().abi(), positionoverride.into_param().abi()).ok() }
    }
    pub fn AudioNotificationRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<GuidanceNavigator, GuidanceAudioNotificationRequestedEventArgs>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IGuidanceNavigator2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).AudioNotificationRequested)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAudioNotificationRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGuidanceNavigator2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAudioNotificationRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn IsGuidanceAudioMuted(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IGuidanceNavigator2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsGuidanceAudioMuted)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsGuidanceAudioMuted(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGuidanceNavigator2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsGuidanceAudioMuted)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetCurrent() -> ::windows_core::Result<GuidanceNavigator> {
        Self::IGuidanceNavigatorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GuidanceNavigator>(result__)
        })
    }
    pub fn UseAppProvidedVoice() -> ::windows_core::Result<bool> {
        Self::IGuidanceNavigatorStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).UseAppProvidedVoice)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn IGuidanceNavigatorStatics<R, F: FnOnce(&IGuidanceNavigatorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<GuidanceNavigator, IGuidanceNavigatorStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IGuidanceNavigatorStatics2<R, F: FnOnce(&IGuidanceNavigatorStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<GuidanceNavigator, IGuidanceNavigatorStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for GuidanceNavigator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GuidanceNavigator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GuidanceNavigator {}
impl ::core::fmt::Debug for GuidanceNavigator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceNavigator").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GuidanceNavigator {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceNavigator;{08f17ef7-8e3f-4d9a-be8a-108f9a012c67})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GuidanceNavigator {
    type Vtable = IGuidanceNavigator_Vtbl;
    const IID: ::windows_core::GUID = <IGuidanceNavigator as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GuidanceNavigator {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceNavigator";
}
impl ::core::convert::From<GuidanceNavigator> for ::windows_core::IUnknown {
    fn from(value: GuidanceNavigator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GuidanceNavigator> for ::windows_core::IUnknown {
    fn from(value: &GuidanceNavigator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GuidanceNavigator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GuidanceNavigator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GuidanceNavigator> for ::windows_core::IInspectable {
    fn from(value: GuidanceNavigator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GuidanceNavigator> for ::windows_core::IInspectable {
    fn from(value: &GuidanceNavigator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GuidanceNavigator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GuidanceNavigator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GuidanceNavigator {}
unsafe impl ::core::marker::Sync for GuidanceNavigator {}
#[repr(transparent)]
pub struct GuidanceReroutedEventArgs(::windows_core::IUnknown);
impl GuidanceReroutedEventArgs {
    pub fn Route(&self) -> ::windows_core::Result<GuidanceRoute> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Route)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GuidanceRoute>(result__)
        }
    }
}
impl ::core::clone::Clone for GuidanceReroutedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GuidanceReroutedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GuidanceReroutedEventArgs {}
impl ::core::fmt::Debug for GuidanceReroutedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceReroutedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GuidanceReroutedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceReroutedEventArgs;{115d4008-d528-454e-bb94-a50341d2c9f1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GuidanceReroutedEventArgs {
    type Vtable = IGuidanceReroutedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IGuidanceReroutedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GuidanceReroutedEventArgs {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceReroutedEventArgs";
}
impl ::core::convert::From<GuidanceReroutedEventArgs> for ::windows_core::IUnknown {
    fn from(value: GuidanceReroutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GuidanceReroutedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &GuidanceReroutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GuidanceReroutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GuidanceReroutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GuidanceReroutedEventArgs> for ::windows_core::IInspectable {
    fn from(value: GuidanceReroutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GuidanceReroutedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &GuidanceReroutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GuidanceReroutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GuidanceReroutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GuidanceReroutedEventArgs {}
unsafe impl ::core::marker::Sync for GuidanceReroutedEventArgs {}
#[repr(transparent)]
pub struct GuidanceRoadSegment(::windows_core::IUnknown);
impl GuidanceRoadSegment {
    pub fn RoadName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RoadName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ShortRoadName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ShortRoadName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SpeedLimit(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).SpeedLimit)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn TravelTime(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).TravelTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn Path(&self) -> ::windows_core::Result<::winrt_devices::Geolocation::Geopath> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Path)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Geolocation::Geopath>(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IsHighway(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsHighway)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsTunnel(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsTunnel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsTollRoad(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsTollRoad)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsScenic(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IGuidanceRoadSegment2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsScenic)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for GuidanceRoadSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GuidanceRoadSegment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GuidanceRoadSegment {}
impl ::core::fmt::Debug for GuidanceRoadSegment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceRoadSegment").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GuidanceRoadSegment {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceRoadSegment;{b32758a6-be78-4c63-afe7-6c2957479b3e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GuidanceRoadSegment {
    type Vtable = IGuidanceRoadSegment_Vtbl;
    const IID: ::windows_core::GUID = <IGuidanceRoadSegment as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GuidanceRoadSegment {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceRoadSegment";
}
impl ::core::convert::From<GuidanceRoadSegment> for ::windows_core::IUnknown {
    fn from(value: GuidanceRoadSegment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GuidanceRoadSegment> for ::windows_core::IUnknown {
    fn from(value: &GuidanceRoadSegment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GuidanceRoadSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GuidanceRoadSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GuidanceRoadSegment> for ::windows_core::IInspectable {
    fn from(value: GuidanceRoadSegment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GuidanceRoadSegment> for ::windows_core::IInspectable {
    fn from(value: &GuidanceRoadSegment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GuidanceRoadSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GuidanceRoadSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GuidanceRoadSegment {}
unsafe impl ::core::marker::Sync for GuidanceRoadSegment {}
#[repr(transparent)]
pub struct GuidanceRoadSignpost(::windows_core::IUnknown);
impl GuidanceRoadSignpost {
    pub fn ExitNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ExitNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Exit(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Exit)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn BackgroundColor(&self) -> ::windows_core::Result<::winrt_ui::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_ui::Color>::zeroed();
            (::windows_core::Interface::vtable(this).BackgroundColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_ui::Color>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn ForegroundColor(&self) -> ::windows_core::Result<::winrt_ui::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_ui::Color>::zeroed();
            (::windows_core::Interface::vtable(this).ForegroundColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_ui::Color>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ExitDirections(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExitDirections)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
}
impl ::core::clone::Clone for GuidanceRoadSignpost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GuidanceRoadSignpost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GuidanceRoadSignpost {}
impl ::core::fmt::Debug for GuidanceRoadSignpost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceRoadSignpost").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GuidanceRoadSignpost {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceRoadSignpost;{f1a728b6-f77a-4742-8312-53300f9845f0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GuidanceRoadSignpost {
    type Vtable = IGuidanceRoadSignpost_Vtbl;
    const IID: ::windows_core::GUID = <IGuidanceRoadSignpost as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GuidanceRoadSignpost {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceRoadSignpost";
}
impl ::core::convert::From<GuidanceRoadSignpost> for ::windows_core::IUnknown {
    fn from(value: GuidanceRoadSignpost) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GuidanceRoadSignpost> for ::windows_core::IUnknown {
    fn from(value: &GuidanceRoadSignpost) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GuidanceRoadSignpost {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GuidanceRoadSignpost {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GuidanceRoadSignpost> for ::windows_core::IInspectable {
    fn from(value: GuidanceRoadSignpost) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GuidanceRoadSignpost> for ::windows_core::IInspectable {
    fn from(value: &GuidanceRoadSignpost) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GuidanceRoadSignpost {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GuidanceRoadSignpost {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GuidanceRoadSignpost {}
unsafe impl ::core::marker::Sync for GuidanceRoadSignpost {}
#[repr(transparent)]
pub struct GuidanceRoute(::windows_core::IUnknown);
impl GuidanceRoute {
    pub fn Duration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn Distance(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Distance)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Maneuvers(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<GuidanceManeuver>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Maneuvers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<GuidanceManeuver>>(result__)
        }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn BoundingBox(&self) -> ::windows_core::Result<::winrt_devices::Geolocation::GeoboundingBox> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BoundingBox)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Geolocation::GeoboundingBox>(result__)
        }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn Path(&self) -> ::windows_core::Result<::winrt_devices::Geolocation::Geopath> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Path)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Geolocation::Geopath>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RoadSegments(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<GuidanceRoadSegment>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RoadSegments)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<GuidanceRoadSegment>>(result__)
        }
    }
    pub fn ConvertToMapRoute(&self) -> ::windows_core::Result<super::MapRoute> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ConvertToMapRoute)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MapRoute>(result__)
        }
    }
    pub fn CanCreateFromMapRoute<'a, Param0: ::windows_core::IntoParam<'a, super::MapRoute>>(maproute: Param0) -> ::windows_core::Result<bool> {
        Self::IGuidanceRouteStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanCreateFromMapRoute)(::windows_core::Interface::as_raw(this), maproute.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn TryCreateFromMapRoute<'a, Param0: ::windows_core::IntoParam<'a, super::MapRoute>>(maproute: Param0) -> ::windows_core::Result<GuidanceRoute> {
        Self::IGuidanceRouteStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateFromMapRoute)(::windows_core::Interface::as_raw(this), maproute.into_param().abi(), result__.as_mut_ptr()).from_abi::<GuidanceRoute>(result__)
        })
    }
    pub fn IGuidanceRouteStatics<R, F: FnOnce(&IGuidanceRouteStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<GuidanceRoute, IGuidanceRouteStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for GuidanceRoute {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GuidanceRoute {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GuidanceRoute {}
impl ::core::fmt::Debug for GuidanceRoute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceRoute").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GuidanceRoute {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceRoute;{3a14545d-801a-40bd-a286-afb2010cce6c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GuidanceRoute {
    type Vtable = IGuidanceRoute_Vtbl;
    const IID: ::windows_core::GUID = <IGuidanceRoute as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GuidanceRoute {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceRoute";
}
impl ::core::convert::From<GuidanceRoute> for ::windows_core::IUnknown {
    fn from(value: GuidanceRoute) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GuidanceRoute> for ::windows_core::IUnknown {
    fn from(value: &GuidanceRoute) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GuidanceRoute {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GuidanceRoute {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GuidanceRoute> for ::windows_core::IInspectable {
    fn from(value: GuidanceRoute) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GuidanceRoute> for ::windows_core::IInspectable {
    fn from(value: &GuidanceRoute) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GuidanceRoute {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GuidanceRoute {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GuidanceRoute {}
unsafe impl ::core::marker::Sync for GuidanceRoute {}
#[repr(transparent)]
pub struct GuidanceTelemetryCollector(::windows_core::IUnknown);
impl GuidanceTelemetryCollector {
    pub fn Enabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Enabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ClearLocalData(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ClearLocalData)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SpeedTrigger(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).SpeedTrigger)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetSpeedTrigger(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSpeedTrigger)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn UploadFrequency(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).UploadFrequency)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetUploadFrequency(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUploadFrequency)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetCurrent() -> ::windows_core::Result<GuidanceTelemetryCollector> {
        Self::IGuidanceTelemetryCollectorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GuidanceTelemetryCollector>(result__)
        })
    }
    pub fn IGuidanceTelemetryCollectorStatics<R, F: FnOnce(&IGuidanceTelemetryCollectorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<GuidanceTelemetryCollector, IGuidanceTelemetryCollectorStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for GuidanceTelemetryCollector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GuidanceTelemetryCollector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GuidanceTelemetryCollector {}
impl ::core::fmt::Debug for GuidanceTelemetryCollector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceTelemetryCollector").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GuidanceTelemetryCollector {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceTelemetryCollector;{db1f8da5-b878-4d92-98dd-347d23d38262})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GuidanceTelemetryCollector {
    type Vtable = IGuidanceTelemetryCollector_Vtbl;
    const IID: ::windows_core::GUID = <IGuidanceTelemetryCollector as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GuidanceTelemetryCollector {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceTelemetryCollector";
}
impl ::core::convert::From<GuidanceTelemetryCollector> for ::windows_core::IUnknown {
    fn from(value: GuidanceTelemetryCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GuidanceTelemetryCollector> for ::windows_core::IUnknown {
    fn from(value: &GuidanceTelemetryCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GuidanceTelemetryCollector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GuidanceTelemetryCollector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GuidanceTelemetryCollector> for ::windows_core::IInspectable {
    fn from(value: GuidanceTelemetryCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GuidanceTelemetryCollector> for ::windows_core::IInspectable {
    fn from(value: &GuidanceTelemetryCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GuidanceTelemetryCollector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GuidanceTelemetryCollector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GuidanceTelemetryCollector {}
unsafe impl ::core::marker::Sync for GuidanceTelemetryCollector {}
#[repr(transparent)]
pub struct GuidanceUpdatedEventArgs(::windows_core::IUnknown);
impl GuidanceUpdatedEventArgs {
    pub fn Mode(&self) -> ::windows_core::Result<GuidanceMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GuidanceMode>::zeroed();
            (::windows_core::Interface::vtable(this).Mode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GuidanceMode>(result__)
        }
    }
    pub fn NextManeuver(&self) -> ::windows_core::Result<GuidanceManeuver> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NextManeuver)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GuidanceManeuver>(result__)
        }
    }
    pub fn NextManeuverDistance(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).NextManeuverDistance)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn AfterNextManeuver(&self) -> ::windows_core::Result<GuidanceManeuver> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AfterNextManeuver)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GuidanceManeuver>(result__)
        }
    }
    pub fn AfterNextManeuverDistance(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).AfterNextManeuverDistance)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn DistanceToDestination(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).DistanceToDestination)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn ElapsedDistance(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).ElapsedDistance)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn ElapsedTime(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).ElapsedTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn TimeToDestination(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).TimeToDestination)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn RoadName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RoadName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Route(&self) -> ::windows_core::Result<GuidanceRoute> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Route)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GuidanceRoute>(result__)
        }
    }
    pub fn CurrentLocation(&self) -> ::windows_core::Result<GuidanceMapMatchedCoordinate> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentLocation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GuidanceMapMatchedCoordinate>(result__)
        }
    }
    pub fn IsNewManeuver(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsNewManeuver)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn LaneInfo(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<GuidanceLaneInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LaneInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<GuidanceLaneInfo>>(result__)
        }
    }
}
impl ::core::clone::Clone for GuidanceUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GuidanceUpdatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GuidanceUpdatedEventArgs {}
impl ::core::fmt::Debug for GuidanceUpdatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceUpdatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GuidanceUpdatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceUpdatedEventArgs;{fdac160b-9e8d-4de3-a9fa-b06321d18db9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GuidanceUpdatedEventArgs {
    type Vtable = IGuidanceUpdatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IGuidanceUpdatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GuidanceUpdatedEventArgs {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceUpdatedEventArgs";
}
impl ::core::convert::From<GuidanceUpdatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: GuidanceUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GuidanceUpdatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &GuidanceUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GuidanceUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GuidanceUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GuidanceUpdatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: GuidanceUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GuidanceUpdatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &GuidanceUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GuidanceUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GuidanceUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GuidanceUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for GuidanceUpdatedEventArgs {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceAudioNotificationRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGuidanceAudioNotificationRequestedEventArgs {
    type Vtable = IGuidanceAudioNotificationRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xca2aa24a_c7c2_4d4c_9d7c_499576bceddb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceAudioNotificationRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AudioNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GuidanceAudioNotificationKind) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub AudioFilePaths: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    AudioFilePaths: usize,
    pub AudioText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceLaneInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGuidanceLaneInfo {
    type Vtable = IGuidanceLaneInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8404d114_6581_43b7_ac15_c9079bf90df1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceLaneInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LaneMarkers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GuidanceLaneMarkers) -> ::windows_core::HRESULT,
    pub IsOnRoute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceManeuver(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGuidanceManeuver {
    type Vtable = IGuidanceManeuver_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfc09326c_ecc9_4928_a2a1_7232b99b94a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceManeuver_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-devices")]
    pub StartLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    StartLocation: usize,
    pub DistanceFromRouteStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub DistanceFromPreviousManeuver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub DepartureRoadName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NextRoadName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DepartureShortRoadName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NextShortRoadName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GuidanceManeuverKind) -> ::windows_core::HRESULT,
    pub StartAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub EndAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub RoadSignpost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub InstructionText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceMapMatchedCoordinate(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGuidanceMapMatchedCoordinate {
    type Vtable = IGuidanceMapMatchedCoordinate_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb7acb168_2912_4a99_aff1_798609b981fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceMapMatchedCoordinate_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-devices")]
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    Location: usize,
    pub CurrentHeading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub CurrentSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub IsOnStreet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Road: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceNavigator(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGuidanceNavigator {
    type Vtable = IGuidanceNavigator_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x08f17ef7_8e3f_4d9a_be8a_108f9a012c67);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceNavigator_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StartNavigating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, route: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StartSimulating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, route: ::windows_core::RawPtr, speedinmeterspersecond: i32) -> ::windows_core::HRESULT,
    pub StartTracking: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RepeatLastAudioNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AudioMeasurementSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GuidanceAudioMeasurementSystem) -> ::windows_core::HRESULT,
    pub SetAudioMeasurementSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: GuidanceAudioMeasurementSystem) -> ::windows_core::HRESULT,
    pub AudioNotifications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GuidanceAudioNotifications) -> ::windows_core::HRESULT,
    pub SetAudioNotifications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: GuidanceAudioNotifications) -> ::windows_core::HRESULT,
    pub GuidanceUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveGuidanceUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub DestinationReached: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveDestinationReached: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Rerouting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveRerouting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Rerouted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveRerouted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RerouteFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveRerouteFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub UserLocationLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveUserLocationLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub UserLocationRestored: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveUserLocationRestored: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub SetGuidanceVoice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, voiceid: i32, voicefolder: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-devices")]
    pub UpdateUserLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userlocation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    UpdateUserLocation: usize,
    #[cfg(feature = "winrt-devices")]
    pub UpdateUserLocationWithPositionOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userlocation: ::windows_core::RawPtr, positionoverride: ::winrt_devices::Geolocation::BasicGeoposition) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    UpdateUserLocationWithPositionOverride: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceNavigator2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGuidanceNavigator2 {
    type Vtable = IGuidanceNavigator2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6cdc50d1_041c_4bf3_b633_a101fc2f6b57);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceNavigator2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AudioNotificationRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveAudioNotificationRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub IsGuidanceAudioMuted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsGuidanceAudioMuted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceNavigatorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGuidanceNavigatorStatics {
    type Vtable = IGuidanceNavigatorStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00fd9513_4456_4e66_a143_3add6be08426);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceNavigatorStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceNavigatorStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGuidanceNavigatorStatics2 {
    type Vtable = IGuidanceNavigatorStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x54c5c3e2_7784_4c85_8c95_d0c6efb43965);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceNavigatorStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub UseAppProvidedVoice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceReroutedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGuidanceReroutedEventArgs {
    type Vtable = IGuidanceReroutedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x115d4008_d528_454e_bb94_a50341d2c9f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceReroutedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Route: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceRoadSegment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGuidanceRoadSegment {
    type Vtable = IGuidanceRoadSegment_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb32758a6_be78_4c63_afe7_6c2957479b3e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceRoadSegment_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RoadName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ShortRoadName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SpeedLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub TravelTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-devices")]
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    Path: usize,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsHighway: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsTunnel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsTollRoad: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceRoadSegment2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGuidanceRoadSegment2 {
    type Vtable = IGuidanceRoadSegment2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2474a61d_1723_49f1_895b_47a2c4aa9c55);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceRoadSegment2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsScenic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceRoadSignpost(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGuidanceRoadSignpost {
    type Vtable = IGuidanceRoadSignpost_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf1a728b6_f77a_4742_8312_53300f9845f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceRoadSignpost_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExitNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Exit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub BackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    BackgroundColor: usize,
    #[cfg(feature = "winrt-ui")]
    pub ForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    ForegroundColor: usize,
    #[cfg(feature = "winrt-foundation")]
    pub ExitDirections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ExitDirections: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceRoute(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGuidanceRoute {
    type Vtable = IGuidanceRoute_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3a14545d_801a_40bd_a286_afb2010cce6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceRoute_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub Distance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Maneuvers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Maneuvers: usize,
    #[cfg(feature = "winrt-devices")]
    pub BoundingBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    BoundingBox: usize,
    #[cfg(feature = "winrt-devices")]
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    Path: usize,
    #[cfg(feature = "winrt-foundation")]
    pub RoadSegments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    RoadSegments: usize,
    pub ConvertToMapRoute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceRouteStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGuidanceRouteStatics {
    type Vtable = IGuidanceRouteStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf56d926a_55ed_49c1_b09c_4b8223b50db3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceRouteStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CanCreateFromMapRoute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maproute: ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
    pub TryCreateFromMapRoute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maproute: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceTelemetryCollector(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGuidanceTelemetryCollector {
    type Vtable = IGuidanceTelemetryCollector_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdb1f8da5_b878_4d92_98dd_347d23d38262);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceTelemetryCollector_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ClearLocalData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SpeedTrigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetSpeedTrigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub UploadFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetUploadFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceTelemetryCollectorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGuidanceTelemetryCollectorStatics {
    type Vtable = IGuidanceTelemetryCollectorStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x36532047_f160_44fb_b578_94577ca05990);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceTelemetryCollectorStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceUpdatedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGuidanceUpdatedEventArgs {
    type Vtable = IGuidanceUpdatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfdac160b_9e8d_4de3_a9fa_b06321d18db9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceUpdatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GuidanceMode) -> ::windows_core::HRESULT,
    pub NextManeuver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub NextManeuverDistance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub AfterNextManeuver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AfterNextManeuverDistance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub DistanceToDestination: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub ElapsedDistance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub ElapsedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub TimeToDestination: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub RoadName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Route: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CurrentLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsNewManeuver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub LaneInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    LaneInfo: usize,
}
