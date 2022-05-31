pub const COMPOSITIONOBJECT_READ: i32 = 1i32;
pub const COMPOSITIONOBJECT_WRITE: i32 = 2i32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COMPOSITION_FRAME_ID_TYPE(pub i32);
pub const COMPOSITION_FRAME_ID_CREATED: COMPOSITION_FRAME_ID_TYPE = COMPOSITION_FRAME_ID_TYPE(0i32);
pub const COMPOSITION_FRAME_ID_CONFIRMED: COMPOSITION_FRAME_ID_TYPE = COMPOSITION_FRAME_ID_TYPE(1i32);
pub const COMPOSITION_FRAME_ID_COMPLETED: COMPOSITION_FRAME_ID_TYPE = COMPOSITION_FRAME_ID_TYPE(2i32);
impl ::core::marker::Copy for COMPOSITION_FRAME_ID_TYPE {}
impl ::core::clone::Clone for COMPOSITION_FRAME_ID_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMPOSITION_FRAME_ID_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for COMPOSITION_FRAME_ID_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMPOSITION_FRAME_ID_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMPOSITION_FRAME_ID_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct COMPOSITION_FRAME_STATS {
    pub startTime: u64,
    pub targetTime: u64,
    pub framePeriod: u64,
}
impl ::core::marker::Copy for COMPOSITION_FRAME_STATS {}
impl ::core::clone::Clone for COMPOSITION_FRAME_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COMPOSITION_FRAME_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMPOSITION_FRAME_STATS").field("startTime", &self.startTime).field("targetTime", &self.targetTime).field("framePeriod", &self.framePeriod).finish()
    }
}
unsafe impl ::windows_core::Abi for COMPOSITION_FRAME_STATS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COMPOSITION_FRAME_STATS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COMPOSITION_FRAME_STATS>()) == 0 }
    }
}
impl ::core::cmp::Eq for COMPOSITION_FRAME_STATS {}
impl ::core::default::Default for COMPOSITION_FRAME_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct COMPOSITION_STATS {
    pub presentCount: u32,
    pub refreshCount: u32,
    pub virtualRefreshCount: u32,
    pub time: u64,
}
impl ::core::marker::Copy for COMPOSITION_STATS {}
impl ::core::clone::Clone for COMPOSITION_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COMPOSITION_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMPOSITION_STATS").field("presentCount", &self.presentCount).field("refreshCount", &self.refreshCount).field("virtualRefreshCount", &self.virtualRefreshCount).field("time", &self.time).finish()
    }
}
unsafe impl ::windows_core::Abi for COMPOSITION_STATS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COMPOSITION_STATS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COMPOSITION_STATS>()) == 0 }
    }
}
impl ::core::cmp::Eq for COMPOSITION_STATS {}
impl ::core::default::Default for COMPOSITION_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const COMPOSITION_STATS_MAX_TARGETS: u32 = 256u32;
#[repr(C)]
pub struct COMPOSITION_TARGET_ID {
    pub displayAdapterLuid: ::win32_foundation::LUID,
    pub renderAdapterLuid: ::win32_foundation::LUID,
    pub vidPnSourceId: u32,
    pub vidPnTargetId: u32,
    pub uniqueId: u32,
}
impl ::core::marker::Copy for COMPOSITION_TARGET_ID {}
impl ::core::clone::Clone for COMPOSITION_TARGET_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COMPOSITION_TARGET_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMPOSITION_TARGET_ID").field("displayAdapterLuid", &self.displayAdapterLuid).field("renderAdapterLuid", &self.renderAdapterLuid).field("vidPnSourceId", &self.vidPnSourceId).field("vidPnTargetId", &self.vidPnTargetId).field("uniqueId", &self.uniqueId).finish()
    }
}
unsafe impl ::windows_core::Abi for COMPOSITION_TARGET_ID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COMPOSITION_TARGET_ID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COMPOSITION_TARGET_ID>()) == 0 }
    }
}
impl ::core::cmp::Eq for COMPOSITION_TARGET_ID {}
impl ::core::default::Default for COMPOSITION_TARGET_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct COMPOSITION_TARGET_STATS {
    pub outstandingPresents: u32,
    pub presentTime: u64,
    pub vblankDuration: u64,
    pub presentedStats: COMPOSITION_STATS,
    pub completedStats: COMPOSITION_STATS,
}
impl ::core::marker::Copy for COMPOSITION_TARGET_STATS {}
impl ::core::clone::Clone for COMPOSITION_TARGET_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COMPOSITION_TARGET_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMPOSITION_TARGET_STATS").field("outstandingPresents", &self.outstandingPresents).field("presentTime", &self.presentTime).field("vblankDuration", &self.vblankDuration).field("presentedStats", &self.presentedStats).field("completedStats", &self.completedStats).finish()
    }
}
unsafe impl ::windows_core::Abi for COMPOSITION_TARGET_STATS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COMPOSITION_TARGET_STATS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COMPOSITION_TARGET_STATS>()) == 0 }
    }
}
impl ::core::cmp::Eq for COMPOSITION_TARGET_STATS {}
impl ::core::default::Default for COMPOSITION_TARGET_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DCOMPOSITION_BACKFACE_VISIBILITY(pub i32);
pub const DCOMPOSITION_BACKFACE_VISIBILITY_VISIBLE: DCOMPOSITION_BACKFACE_VISIBILITY = DCOMPOSITION_BACKFACE_VISIBILITY(0i32);
pub const DCOMPOSITION_BACKFACE_VISIBILITY_HIDDEN: DCOMPOSITION_BACKFACE_VISIBILITY = DCOMPOSITION_BACKFACE_VISIBILITY(1i32);
pub const DCOMPOSITION_BACKFACE_VISIBILITY_INHERIT: DCOMPOSITION_BACKFACE_VISIBILITY = DCOMPOSITION_BACKFACE_VISIBILITY(-1i32);
impl ::core::marker::Copy for DCOMPOSITION_BACKFACE_VISIBILITY {}
impl ::core::clone::Clone for DCOMPOSITION_BACKFACE_VISIBILITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DCOMPOSITION_BACKFACE_VISIBILITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DCOMPOSITION_BACKFACE_VISIBILITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for DCOMPOSITION_BACKFACE_VISIBILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DCOMPOSITION_BACKFACE_VISIBILITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DCOMPOSITION_BITMAP_INTERPOLATION_MODE(pub i32);
pub const DCOMPOSITION_BITMAP_INTERPOLATION_MODE_NEAREST_NEIGHBOR: DCOMPOSITION_BITMAP_INTERPOLATION_MODE = DCOMPOSITION_BITMAP_INTERPOLATION_MODE(0i32);
pub const DCOMPOSITION_BITMAP_INTERPOLATION_MODE_LINEAR: DCOMPOSITION_BITMAP_INTERPOLATION_MODE = DCOMPOSITION_BITMAP_INTERPOLATION_MODE(1i32);
pub const DCOMPOSITION_BITMAP_INTERPOLATION_MODE_INHERIT: DCOMPOSITION_BITMAP_INTERPOLATION_MODE = DCOMPOSITION_BITMAP_INTERPOLATION_MODE(-1i32);
impl ::core::marker::Copy for DCOMPOSITION_BITMAP_INTERPOLATION_MODE {}
impl ::core::clone::Clone for DCOMPOSITION_BITMAP_INTERPOLATION_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DCOMPOSITION_BITMAP_INTERPOLATION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DCOMPOSITION_BITMAP_INTERPOLATION_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DCOMPOSITION_BITMAP_INTERPOLATION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DCOMPOSITION_BITMAP_INTERPOLATION_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DCOMPOSITION_BORDER_MODE(pub i32);
pub const DCOMPOSITION_BORDER_MODE_SOFT: DCOMPOSITION_BORDER_MODE = DCOMPOSITION_BORDER_MODE(0i32);
pub const DCOMPOSITION_BORDER_MODE_HARD: DCOMPOSITION_BORDER_MODE = DCOMPOSITION_BORDER_MODE(1i32);
pub const DCOMPOSITION_BORDER_MODE_INHERIT: DCOMPOSITION_BORDER_MODE = DCOMPOSITION_BORDER_MODE(-1i32);
impl ::core::marker::Copy for DCOMPOSITION_BORDER_MODE {}
impl ::core::clone::Clone for DCOMPOSITION_BORDER_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DCOMPOSITION_BORDER_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DCOMPOSITION_BORDER_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DCOMPOSITION_BORDER_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DCOMPOSITION_BORDER_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DCOMPOSITION_COMPOSITE_MODE(pub i32);
pub const DCOMPOSITION_COMPOSITE_MODE_SOURCE_OVER: DCOMPOSITION_COMPOSITE_MODE = DCOMPOSITION_COMPOSITE_MODE(0i32);
pub const DCOMPOSITION_COMPOSITE_MODE_DESTINATION_INVERT: DCOMPOSITION_COMPOSITE_MODE = DCOMPOSITION_COMPOSITE_MODE(1i32);
pub const DCOMPOSITION_COMPOSITE_MODE_MIN_BLEND: DCOMPOSITION_COMPOSITE_MODE = DCOMPOSITION_COMPOSITE_MODE(2i32);
pub const DCOMPOSITION_COMPOSITE_MODE_INHERIT: DCOMPOSITION_COMPOSITE_MODE = DCOMPOSITION_COMPOSITE_MODE(-1i32);
impl ::core::marker::Copy for DCOMPOSITION_COMPOSITE_MODE {}
impl ::core::clone::Clone for DCOMPOSITION_COMPOSITE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DCOMPOSITION_COMPOSITE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DCOMPOSITION_COMPOSITE_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DCOMPOSITION_COMPOSITE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DCOMPOSITION_COMPOSITE_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DCOMPOSITION_DEPTH_MODE(pub i32);
pub const DCOMPOSITION_DEPTH_MODE_TREE: DCOMPOSITION_DEPTH_MODE = DCOMPOSITION_DEPTH_MODE(0i32);
pub const DCOMPOSITION_DEPTH_MODE_SPATIAL: DCOMPOSITION_DEPTH_MODE = DCOMPOSITION_DEPTH_MODE(1i32);
pub const DCOMPOSITION_DEPTH_MODE_SORTED: DCOMPOSITION_DEPTH_MODE = DCOMPOSITION_DEPTH_MODE(3i32);
pub const DCOMPOSITION_DEPTH_MODE_INHERIT: DCOMPOSITION_DEPTH_MODE = DCOMPOSITION_DEPTH_MODE(-1i32);
impl ::core::marker::Copy for DCOMPOSITION_DEPTH_MODE {}
impl ::core::clone::Clone for DCOMPOSITION_DEPTH_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DCOMPOSITION_DEPTH_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DCOMPOSITION_DEPTH_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DCOMPOSITION_DEPTH_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DCOMPOSITION_DEPTH_MODE").field(&self.0).finish()
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct DCOMPOSITION_FRAME_STATISTICS {
    pub lastFrameTime: i64,
    pub currentCompositionRate: super::Dxgi::Common::DXGI_RATIONAL,
    pub currentTime: i64,
    pub timeFrequency: i64,
    pub nextEstimatedFrameTime: i64,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for DCOMPOSITION_FRAME_STATISTICS {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for DCOMPOSITION_FRAME_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for DCOMPOSITION_FRAME_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DCOMPOSITION_FRAME_STATISTICS").field("lastFrameTime", &self.lastFrameTime).field("currentCompositionRate", &self.currentCompositionRate).field("currentTime", &self.currentTime).field("timeFrequency", &self.timeFrequency).field("nextEstimatedFrameTime", &self.nextEstimatedFrameTime).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows_core::Abi for DCOMPOSITION_FRAME_STATISTICS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for DCOMPOSITION_FRAME_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DCOMPOSITION_FRAME_STATISTICS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for DCOMPOSITION_FRAME_STATISTICS {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for DCOMPOSITION_FRAME_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DCOMPOSITION_MAX_WAITFORCOMPOSITORCLOCK_OBJECTS: u32 = 32u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DCOMPOSITION_OPACITY_MODE(pub i32);
pub const DCOMPOSITION_OPACITY_MODE_LAYER: DCOMPOSITION_OPACITY_MODE = DCOMPOSITION_OPACITY_MODE(0i32);
pub const DCOMPOSITION_OPACITY_MODE_MULTIPLY: DCOMPOSITION_OPACITY_MODE = DCOMPOSITION_OPACITY_MODE(1i32);
pub const DCOMPOSITION_OPACITY_MODE_INHERIT: DCOMPOSITION_OPACITY_MODE = DCOMPOSITION_OPACITY_MODE(-1i32);
impl ::core::marker::Copy for DCOMPOSITION_OPACITY_MODE {}
impl ::core::clone::Clone for DCOMPOSITION_OPACITY_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DCOMPOSITION_OPACITY_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DCOMPOSITION_OPACITY_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DCOMPOSITION_OPACITY_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DCOMPOSITION_OPACITY_MODE").field(&self.0).finish()
    }
}
#[inline]
pub unsafe fn DCompositionAttachMouseDragToHwnd<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionVisual>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(visual: Param0, hwnd: Param1, enable: Param2) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionAttachMouseDragToHwnd(visual: ::windows_core::RawPtr, hwnd: ::win32_foundation::HWND, enable: ::win32_foundation::BOOL) -> ::windows_core::HRESULT;
        }
        DCompositionAttachMouseDragToHwnd(visual.into_param().abi(), hwnd.into_param().abi(), enable.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DCompositionAttachMouseWheelToHwnd<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionVisual>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(visual: Param0, hwnd: Param1, enable: Param2) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionAttachMouseWheelToHwnd(visual: ::windows_core::RawPtr, hwnd: ::win32_foundation::HWND, enable: ::win32_foundation::BOOL) -> ::windows_core::HRESULT;
        }
        DCompositionAttachMouseWheelToHwnd(visual.into_param().abi(), hwnd.into_param().abi(), enable.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DCompositionBoostCompositorClock<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(enable: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionBoostCompositorClock(enable: ::win32_foundation::BOOL) -> ::windows_core::HRESULT;
        }
        DCompositionBoostCompositorClock(enable.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
#[inline]
pub unsafe fn DCompositionCreateDevice<'a, Param0: ::windows_core::IntoParam<'a, super::Dxgi::IDXGIDevice>>(dxgidevice: Param0, iid: *const ::windows_core::GUID, dcompositiondevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionCreateDevice(dxgidevice: ::windows_core::RawPtr, iid: *const ::windows_core::GUID, dcompositiondevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        DCompositionCreateDevice(dxgidevice.into_param().abi(), ::core::mem::transmute(iid), ::core::mem::transmute(dcompositiondevice)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DCompositionCreateDevice2<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(renderingdevice: Param0, iid: *const ::windows_core::GUID, dcompositiondevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionCreateDevice2(renderingdevice: *mut ::core::ffi::c_void, iid: *const ::windows_core::GUID, dcompositiondevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        DCompositionCreateDevice2(renderingdevice.into_param().abi(), ::core::mem::transmute(iid), ::core::mem::transmute(dcompositiondevice)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DCompositionCreateDevice3<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(renderingdevice: Param0, iid: *const ::windows_core::GUID, dcompositiondevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionCreateDevice3(renderingdevice: *mut ::core::ffi::c_void, iid: *const ::windows_core::GUID, dcompositiondevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        DCompositionCreateDevice3(renderingdevice.into_param().abi(), ::core::mem::transmute(iid), ::core::mem::transmute(dcompositiondevice)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn DCompositionCreateSurfaceHandle(desiredaccess: u32, securityattributes: *const ::win32_security::SECURITY_ATTRIBUTES) -> ::windows_core::Result<::win32_foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionCreateSurfaceHandle(desiredaccess: u32, securityattributes: *const ::win32_security::SECURITY_ATTRIBUTES, surfacehandle: *mut ::win32_foundation::HANDLE) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::HANDLE>::zeroed();
        DCompositionCreateSurfaceHandle(::core::mem::transmute(desiredaccess), ::core::mem::transmute(securityattributes), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::HANDLE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DCompositionGetFrameId(frameidtype: COMPOSITION_FRAME_ID_TYPE) -> ::windows_core::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionGetFrameId(frameidtype: COMPOSITION_FRAME_ID_TYPE, frameid: *mut u64) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        DCompositionGetFrameId(::core::mem::transmute(frameidtype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DCompositionGetStatistics(frameid: u64, framestats: *mut COMPOSITION_FRAME_STATS, targetidcount: u32, targetids: *mut COMPOSITION_TARGET_ID, actualtargetidcount: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionGetStatistics(frameid: u64, framestats: *mut COMPOSITION_FRAME_STATS, targetidcount: u32, targetids: *mut COMPOSITION_TARGET_ID, actualtargetidcount: *mut u32) -> ::windows_core::HRESULT;
        }
        DCompositionGetStatistics(::core::mem::transmute(frameid), ::core::mem::transmute(framestats), ::core::mem::transmute(targetidcount), ::core::mem::transmute(targetids), ::core::mem::transmute(actualtargetidcount)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DCompositionGetTargetStatistics(frameid: u64, targetid: *const COMPOSITION_TARGET_ID) -> ::windows_core::Result<COMPOSITION_TARGET_STATS> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionGetTargetStatistics(frameid: u64, targetid: *const COMPOSITION_TARGET_ID, targetstats: *mut COMPOSITION_TARGET_STATS) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<COMPOSITION_TARGET_STATS>::zeroed();
        DCompositionGetTargetStatistics(::core::mem::transmute(frameid), ::core::mem::transmute(targetid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<COMPOSITION_TARGET_STATS>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
pub struct DCompositionInkTrailPoint {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}
impl ::core::marker::Copy for DCompositionInkTrailPoint {}
impl ::core::clone::Clone for DCompositionInkTrailPoint {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DCompositionInkTrailPoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DCompositionInkTrailPoint").field("x", &self.x).field("y", &self.y).field("radius", &self.radius).finish()
    }
}
unsafe impl ::windows_core::Abi for DCompositionInkTrailPoint {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DCompositionInkTrailPoint {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DCompositionInkTrailPoint>()) == 0 }
    }
}
impl ::core::cmp::Eq for DCompositionInkTrailPoint {}
impl ::core::default::Default for DCompositionInkTrailPoint {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[inline]
pub unsafe fn DCompositionWaitForCompositorClock(handles: &[::win32_foundation::HANDLE], timeoutinms: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionWaitForCompositorClock(count: u32, handles: *const ::win32_foundation::HANDLE, timeoutinms: u32) -> u32;
        }
        ::core::mem::transmute(DCompositionWaitForCompositorClock(handles.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(handles)), ::core::mem::transmute(timeoutinms)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
pub struct IDCompositionAffineTransform2DEffect(::windows_core::IUnknown);
impl IDCompositionAffineTransform2DEffect {
    pub unsafe fn SetInput<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetInterpolationMode(&self, interpolationmode: super::Direct2D::Common::D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInterpolationMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(interpolationmode)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetBorderMode(&self, bordermode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBorderMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(bordermode)).ok()
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransformMatrix(&self, transformmatrix: *const ::winrt_foundation::Numerics::Matrix3x2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTransformMatrix)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(transformmatrix)).ok()
    }
    pub unsafe fn SetTransformMatrixElement<'a, Param2: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, row: i32, column: i32, animation: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTransformMatrixElement)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(row), ::core::mem::transmute(column), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetTransformMatrixElement2(&self, row: i32, column: i32, value: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTransformMatrixElement2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(row), ::core::mem::transmute(column), ::core::mem::transmute(value)).ok()
    }
    pub unsafe fn SetSharpness<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSharpness)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetSharpness2(&self, sharpness: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSharpness2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(sharpness)).ok()
    }
}
impl ::core::convert::From<IDCompositionAffineTransform2DEffect> for ::windows_core::IUnknown {
    fn from(value: IDCompositionAffineTransform2DEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionAffineTransform2DEffect> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionAffineTransform2DEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionAffineTransform2DEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionAffineTransform2DEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionAffineTransform2DEffect> for IDCompositionEffect {
    fn from(value: IDCompositionAffineTransform2DEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionAffineTransform2DEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionAffineTransform2DEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for IDCompositionAffineTransform2DEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionAffineTransform2DEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionAffineTransform2DEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionAffineTransform2DEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionAffineTransform2DEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionAffineTransform2DEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionFilterEffect> for IDCompositionAffineTransform2DEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionFilterEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionFilterEffect> for &'a IDCompositionAffineTransform2DEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionFilterEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionAffineTransform2DEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionAffineTransform2DEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionAffineTransform2DEffect {}
impl ::core::fmt::Debug for IDCompositionAffineTransform2DEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionAffineTransform2DEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionAffineTransform2DEffect {
    type Vtable = IDCompositionAffineTransform2DEffect_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0b74b9e8_cdd6_492f_bbbc_5ed32157026d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionAffineTransform2DEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetInterpolationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interpolationmode: super::Direct2D::Common::D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetInterpolationMode: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetBorderMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bordermode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetBorderMode: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTransformMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transformmatrix: *const ::winrt_foundation::Numerics::Matrix3x2) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTransformMatrix: usize,
    pub SetTransformMatrixElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetTransformMatrixElement2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows_core::HRESULT,
    pub SetSharpness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetSharpness2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharpness: f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDCompositionAnimation(::windows_core::IUnknown);
impl IDCompositionAnimation {
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetAbsoluteBeginTime(&self, begintime: i64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAbsoluteBeginTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(begintime)).ok()
    }
    pub unsafe fn AddCubic(&self, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddCubic)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(beginoffset), ::core::mem::transmute(constantcoefficient), ::core::mem::transmute(linearcoefficient), ::core::mem::transmute(quadraticcoefficient), ::core::mem::transmute(cubiccoefficient)).ok()
    }
    pub unsafe fn AddSinusoidal(&self, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddSinusoidal)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(beginoffset), ::core::mem::transmute(bias), ::core::mem::transmute(amplitude), ::core::mem::transmute(frequency), ::core::mem::transmute(phase)).ok()
    }
    pub unsafe fn AddRepeat(&self, beginoffset: f64, durationtorepeat: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddRepeat)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(beginoffset), ::core::mem::transmute(durationtorepeat)).ok()
    }
    pub unsafe fn End(&self, endoffset: f64, endvalue: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).End)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(endoffset), ::core::mem::transmute(endvalue)).ok()
    }
}
impl ::core::convert::From<IDCompositionAnimation> for ::windows_core::IUnknown {
    fn from(value: IDCompositionAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionAnimation> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionAnimation {}
impl ::core::fmt::Debug for IDCompositionAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionAnimation {
    type Vtable = IDCompositionAnimation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcbfd91d9_51b2_45e4_b3de_d19ccfb863c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionAnimation_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetAbsoluteBeginTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, begintime: i64) -> ::windows_core::HRESULT,
    pub AddCubic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> ::windows_core::HRESULT,
    pub AddSinusoidal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> ::windows_core::HRESULT,
    pub AddRepeat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, beginoffset: f64, durationtorepeat: f64) -> ::windows_core::HRESULT,
    pub End: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endoffset: f64, endvalue: f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDCompositionArithmeticCompositeEffect(::windows_core::IUnknown);
impl IDCompositionArithmeticCompositeEffect {
    pub unsafe fn SetInput<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetCoefficients(&self, coefficients: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCoefficients)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(coefficients)).ok()
    }
    pub unsafe fn SetClampOutput<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, clampoutput: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetClampOutput)(::windows_core::Interface::as_raw(self), clampoutput.into_param().abi()).ok()
    }
    pub unsafe fn SetCoefficient1<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCoefficient1)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetCoefficient12(&self, coeffcient1: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCoefficient12)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(coeffcient1)).ok()
    }
    pub unsafe fn SetCoefficient2<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCoefficient2)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetCoefficient22(&self, coefficient2: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCoefficient22)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(coefficient2)).ok()
    }
    pub unsafe fn SetCoefficient3<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCoefficient3)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetCoefficient32(&self, coefficient3: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCoefficient32)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(coefficient3)).ok()
    }
    pub unsafe fn SetCoefficient4<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCoefficient4)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetCoefficient42(&self, coefficient4: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCoefficient42)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(coefficient4)).ok()
    }
}
impl ::core::convert::From<IDCompositionArithmeticCompositeEffect> for ::windows_core::IUnknown {
    fn from(value: IDCompositionArithmeticCompositeEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionArithmeticCompositeEffect> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionArithmeticCompositeEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionArithmeticCompositeEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionArithmeticCompositeEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionArithmeticCompositeEffect> for IDCompositionEffect {
    fn from(value: IDCompositionArithmeticCompositeEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionArithmeticCompositeEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionArithmeticCompositeEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for IDCompositionArithmeticCompositeEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionArithmeticCompositeEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionArithmeticCompositeEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionArithmeticCompositeEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionArithmeticCompositeEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionArithmeticCompositeEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionFilterEffect> for IDCompositionArithmeticCompositeEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionFilterEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionFilterEffect> for &'a IDCompositionArithmeticCompositeEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionFilterEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionArithmeticCompositeEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionArithmeticCompositeEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionArithmeticCompositeEffect {}
impl ::core::fmt::Debug for IDCompositionArithmeticCompositeEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionArithmeticCompositeEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionArithmeticCompositeEffect {
    type Vtable = IDCompositionArithmeticCompositeEffect_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3b67dfa8_e3dd_4e61_b640_46c2f3d739dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionArithmeticCompositeEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetCoefficients: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coefficients: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetCoefficients: usize,
    pub SetClampOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clampoutput: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetCoefficient1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetCoefficient12: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coeffcient1: f32) -> ::windows_core::HRESULT,
    pub SetCoefficient2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetCoefficient22: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coefficient2: f32) -> ::windows_core::HRESULT,
    pub SetCoefficient3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetCoefficient32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coefficient3: f32) -> ::windows_core::HRESULT,
    pub SetCoefficient4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetCoefficient42: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coefficient4: f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDCompositionBlendEffect(::windows_core::IUnknown);
impl IDCompositionBlendEffect {
    pub unsafe fn SetInput<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetMode(&self, mode: super::Direct2D::Common::D2D1_BLEND_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(mode)).ok()
    }
}
impl ::core::convert::From<IDCompositionBlendEffect> for ::windows_core::IUnknown {
    fn from(value: IDCompositionBlendEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionBlendEffect> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionBlendEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionBlendEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionBlendEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionBlendEffect> for IDCompositionEffect {
    fn from(value: IDCompositionBlendEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionBlendEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionBlendEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for IDCompositionBlendEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionBlendEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionBlendEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionBlendEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionBlendEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionBlendEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionFilterEffect> for IDCompositionBlendEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionFilterEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionFilterEffect> for &'a IDCompositionBlendEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionFilterEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionBlendEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionBlendEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionBlendEffect {}
impl ::core::fmt::Debug for IDCompositionBlendEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionBlendEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionBlendEffect {
    type Vtable = IDCompositionBlendEffect_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x33ecdc0a_578a_4a11_9c14_0cb90517f9c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionBlendEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_BLEND_MODE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetMode: usize,
}
#[repr(transparent)]
pub struct IDCompositionBrightnessEffect(::windows_core::IUnknown);
impl IDCompositionBrightnessEffect {
    pub unsafe fn SetInput<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetWhitePoint(&self, whitepoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetWhitePoint)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(whitepoint)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetBlackPoint(&self, blackpoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBlackPoint)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(blackpoint)).ok()
    }
    pub unsafe fn SetWhitePointX<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetWhitePointX)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetWhitePointX2(&self, whitepointx: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetWhitePointX2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(whitepointx)).ok()
    }
    pub unsafe fn SetWhitePointY<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetWhitePointY)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetWhitePointY2(&self, whitepointy: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetWhitePointY2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(whitepointy)).ok()
    }
    pub unsafe fn SetBlackPointX<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBlackPointX)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetBlackPointX2(&self, blackpointx: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBlackPointX2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(blackpointx)).ok()
    }
    pub unsafe fn SetBlackPointY<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBlackPointY)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetBlackPointY2(&self, blackpointy: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBlackPointY2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(blackpointy)).ok()
    }
}
impl ::core::convert::From<IDCompositionBrightnessEffect> for ::windows_core::IUnknown {
    fn from(value: IDCompositionBrightnessEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionBrightnessEffect> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionBrightnessEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionBrightnessEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionBrightnessEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionBrightnessEffect> for IDCompositionEffect {
    fn from(value: IDCompositionBrightnessEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionBrightnessEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionBrightnessEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for IDCompositionBrightnessEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionBrightnessEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionBrightnessEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionBrightnessEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionBrightnessEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionBrightnessEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionFilterEffect> for IDCompositionBrightnessEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionFilterEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionFilterEffect> for &'a IDCompositionBrightnessEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionFilterEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionBrightnessEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionBrightnessEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionBrightnessEffect {}
impl ::core::fmt::Debug for IDCompositionBrightnessEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionBrightnessEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionBrightnessEffect {
    type Vtable = IDCompositionBrightnessEffect_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6027496e_cb3a_49ab_934f_d798da4f7da6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionBrightnessEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetWhitePoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, whitepoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetWhitePoint: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetBlackPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blackpoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetBlackPoint: usize,
    pub SetWhitePointX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetWhitePointX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, whitepointx: f32) -> ::windows_core::HRESULT,
    pub SetWhitePointY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetWhitePointY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, whitepointy: f32) -> ::windows_core::HRESULT,
    pub SetBlackPointX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetBlackPointX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blackpointx: f32) -> ::windows_core::HRESULT,
    pub SetBlackPointY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetBlackPointY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blackpointy: f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDCompositionClip(::windows_core::IUnknown);
impl IDCompositionClip {}
impl ::core::convert::From<IDCompositionClip> for ::windows_core::IUnknown {
    fn from(value: IDCompositionClip) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionClip> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionClip) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionClip {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionClip {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionClip {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionClip {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionClip {}
impl ::core::fmt::Debug for IDCompositionClip {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionClip").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionClip {
    type Vtable = IDCompositionClip_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x64ac3703_9d3f_45ec_a109_7cac0e7a13a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionClip_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
}
#[repr(transparent)]
pub struct IDCompositionColorMatrixEffect(::windows_core::IUnknown);
impl IDCompositionColorMatrixEffect {
    pub unsafe fn SetInput<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetMatrix(&self, matrix: *const super::Direct2D::Common::D2D_MATRIX_5X4_F) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMatrix)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(matrix)).ok()
    }
    pub unsafe fn SetMatrixElement<'a, Param2: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, row: i32, column: i32, animation: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMatrixElement)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(row), ::core::mem::transmute(column), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetMatrixElement2(&self, row: i32, column: i32, value: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMatrixElement2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(row), ::core::mem::transmute(column), ::core::mem::transmute(value)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetAlphaMode(&self, mode: super::Direct2D::Common::D2D1_COLORMATRIX_ALPHA_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAlphaMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(mode)).ok()
    }
    pub unsafe fn SetClampOutput<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, clamp: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetClampOutput)(::windows_core::Interface::as_raw(self), clamp.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IDCompositionColorMatrixEffect> for ::windows_core::IUnknown {
    fn from(value: IDCompositionColorMatrixEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionColorMatrixEffect> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionColorMatrixEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionColorMatrixEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionColorMatrixEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionColorMatrixEffect> for IDCompositionEffect {
    fn from(value: IDCompositionColorMatrixEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionColorMatrixEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionColorMatrixEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for IDCompositionColorMatrixEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionColorMatrixEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionColorMatrixEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionColorMatrixEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionColorMatrixEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionColorMatrixEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionFilterEffect> for IDCompositionColorMatrixEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionFilterEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionFilterEffect> for &'a IDCompositionColorMatrixEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionFilterEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionColorMatrixEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionColorMatrixEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionColorMatrixEffect {}
impl ::core::fmt::Debug for IDCompositionColorMatrixEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionColorMatrixEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionColorMatrixEffect {
    type Vtable = IDCompositionColorMatrixEffect_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc1170a22_3ce2_4966_90d4_55408bfc84c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionColorMatrixEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const super::Direct2D::Common::D2D_MATRIX_5X4_F) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetMatrix: usize,
    pub SetMatrixElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetMatrixElement2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetAlphaMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_COLORMATRIX_ALPHA_MODE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetAlphaMode: usize,
    pub SetClampOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clamp: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDCompositionCompositeEffect(::windows_core::IUnknown);
impl IDCompositionCompositeEffect {
    pub unsafe fn SetInput<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetMode(&self, mode: super::Direct2D::Common::D2D1_COMPOSITE_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(mode)).ok()
    }
}
impl ::core::convert::From<IDCompositionCompositeEffect> for ::windows_core::IUnknown {
    fn from(value: IDCompositionCompositeEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionCompositeEffect> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionCompositeEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionCompositeEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionCompositeEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionCompositeEffect> for IDCompositionEffect {
    fn from(value: IDCompositionCompositeEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionCompositeEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionCompositeEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for IDCompositionCompositeEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionCompositeEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionCompositeEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionCompositeEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionCompositeEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionCompositeEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionFilterEffect> for IDCompositionCompositeEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionFilterEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionFilterEffect> for &'a IDCompositionCompositeEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionFilterEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionCompositeEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionCompositeEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionCompositeEffect {}
impl ::core::fmt::Debug for IDCompositionCompositeEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionCompositeEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionCompositeEffect {
    type Vtable = IDCompositionCompositeEffect_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x576616c0_a231_494d_a38d_00fd5ec4db46);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionCompositeEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_COMPOSITE_MODE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetMode: usize,
}
#[repr(transparent)]
pub struct IDCompositionDelegatedInkTrail(::windows_core::IUnknown);
impl IDCompositionDelegatedInkTrail {
    pub unsafe fn AddTrailPoints(&self, inkpoints: &[DCompositionInkTrailPoint]) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).AddTrailPoints)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(inkpoints)), inkpoints.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn AddTrailPointsWithPrediction(&self, inkpoints: &[DCompositionInkTrailPoint], predictedinkpoints: &[DCompositionInkTrailPoint]) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).AddTrailPointsWithPrediction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(inkpoints)), inkpoints.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(predictedinkpoints)), predictedinkpoints.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn RemoveTrailPoints(&self, generationid: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveTrailPoints)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(generationid)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn StartNewTrail(&self, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StartNewTrail)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(color)).ok()
    }
}
impl ::core::convert::From<IDCompositionDelegatedInkTrail> for ::windows_core::IUnknown {
    fn from(value: IDCompositionDelegatedInkTrail) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionDelegatedInkTrail> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionDelegatedInkTrail) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionDelegatedInkTrail {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionDelegatedInkTrail {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionDelegatedInkTrail {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionDelegatedInkTrail {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionDelegatedInkTrail {}
impl ::core::fmt::Debug for IDCompositionDelegatedInkTrail {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionDelegatedInkTrail").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionDelegatedInkTrail {
    type Vtable = IDCompositionDelegatedInkTrail_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc2448e9b_547d_4057_8cf5_8144ede1c2da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDelegatedInkTrail_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub AddTrailPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32, generationid: *mut u32) -> ::windows_core::HRESULT,
    pub AddTrailPointsWithPrediction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32, predictedinkpoints: *const DCompositionInkTrailPoint, predictedinkpointscount: u32, generationid: *mut u32) -> ::windows_core::HRESULT,
    pub RemoveTrailPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, generationid: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub StartNewTrail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    StartNewTrail: usize,
}
#[repr(transparent)]
pub struct IDCompositionDesktopDevice(::windows_core::IUnknown);
impl IDCompositionDesktopDevice {
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WaitForCommitCompletion(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.WaitForCommitCompletion)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetFrameStatistics(&self) -> ::windows_core::Result<DCOMPOSITION_FRAME_STATISTICS> {
        let mut result__ = ::core::mem::MaybeUninit::<DCOMPOSITION_FRAME_STATISTICS>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetFrameStatistics)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DCOMPOSITION_FRAME_STATISTICS>(result__)
    }
    pub unsafe fn CreateVisual(&self) -> ::windows_core::Result<IDCompositionVisual2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateVisual)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionVisual2>(result__)
    }
    pub unsafe fn CreateSurfaceFactory<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, renderingdevice: Param0) -> ::windows_core::Result<IDCompositionSurfaceFactory> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateSurfaceFactory)(::windows_core::Interface::as_raw(self), renderingdevice.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionSurfaceFactory>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows_core::Result<IDCompositionSurface> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateSurface)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(pixelformat), ::core::mem::transmute(alphamode), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionSurface>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows_core::Result<IDCompositionVirtualSurface> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateVirtualSurface)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(initialwidth), ::core::mem::transmute(initialheight), ::core::mem::transmute(pixelformat), ::core::mem::transmute(alphamode), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionVirtualSurface>(result__)
    }
    pub unsafe fn CreateTranslateTransform(&self) -> ::windows_core::Result<IDCompositionTranslateTransform> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateTranslateTransform)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionTranslateTransform>(result__)
    }
    pub unsafe fn CreateScaleTransform(&self) -> ::windows_core::Result<IDCompositionScaleTransform> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateScaleTransform)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionScaleTransform>(result__)
    }
    pub unsafe fn CreateRotateTransform(&self) -> ::windows_core::Result<IDCompositionRotateTransform> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateRotateTransform)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionRotateTransform>(result__)
    }
    pub unsafe fn CreateSkewTransform(&self) -> ::windows_core::Result<IDCompositionSkewTransform> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateSkewTransform)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionSkewTransform>(result__)
    }
    pub unsafe fn CreateMatrixTransform(&self) -> ::windows_core::Result<IDCompositionMatrixTransform> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateMatrixTransform)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionMatrixTransform>(result__)
    }
    pub unsafe fn CreateTransformGroup(&self, transforms: &[::core::option::Option<IDCompositionTransform>]) -> ::windows_core::Result<IDCompositionTransform> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateTransformGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(transforms)), transforms.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionTransform>(result__)
    }
    pub unsafe fn CreateTranslateTransform3D(&self) -> ::windows_core::Result<IDCompositionTranslateTransform3D> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateTranslateTransform3D)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionTranslateTransform3D>(result__)
    }
    pub unsafe fn CreateScaleTransform3D(&self) -> ::windows_core::Result<IDCompositionScaleTransform3D> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateScaleTransform3D)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionScaleTransform3D>(result__)
    }
    pub unsafe fn CreateRotateTransform3D(&self) -> ::windows_core::Result<IDCompositionRotateTransform3D> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateRotateTransform3D)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionRotateTransform3D>(result__)
    }
    pub unsafe fn CreateMatrixTransform3D(&self) -> ::windows_core::Result<IDCompositionMatrixTransform3D> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateMatrixTransform3D)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionMatrixTransform3D>(result__)
    }
    pub unsafe fn CreateTransform3DGroup(&self, transforms3d: &[::core::option::Option<IDCompositionTransform3D>]) -> ::windows_core::Result<IDCompositionTransform3D> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateTransform3DGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(transforms3d)), transforms3d.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionTransform3D>(result__)
    }
    pub unsafe fn CreateEffectGroup(&self) -> ::windows_core::Result<IDCompositionEffectGroup> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateEffectGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionEffectGroup>(result__)
    }
    pub unsafe fn CreateRectangleClip(&self) -> ::windows_core::Result<IDCompositionRectangleClip> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateRectangleClip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionRectangleClip>(result__)
    }
    pub unsafe fn CreateAnimation(&self) -> ::windows_core::Result<IDCompositionAnimation> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateAnimation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionAnimation>(result__)
    }
    pub unsafe fn CreateTargetForHwnd<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, hwnd: Param0, topmost: Param1) -> ::windows_core::Result<IDCompositionTarget> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateTargetForHwnd)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), topmost.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionTarget>(result__)
    }
    pub unsafe fn CreateSurfaceFromHandle<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(&self, handle: Param0) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).CreateSurfaceFromHandle)(::windows_core::Interface::as_raw(self), handle.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn CreateSurfaceFromHwnd<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(&self, hwnd: Param0) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).CreateSurfaceFromHwnd)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
}
impl ::core::convert::From<IDCompositionDesktopDevice> for ::windows_core::IUnknown {
    fn from(value: IDCompositionDesktopDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionDesktopDevice> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionDesktopDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionDesktopDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionDesktopDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionDesktopDevice> for IDCompositionDevice2 {
    fn from(value: IDCompositionDesktopDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionDesktopDevice> for IDCompositionDevice2 {
    fn from(value: &IDCompositionDesktopDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionDevice2> for IDCompositionDesktopDevice {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionDevice2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionDevice2> for &'a IDCompositionDesktopDevice {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionDevice2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionDesktopDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionDesktopDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionDesktopDevice {}
impl ::core::fmt::Debug for IDCompositionDesktopDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionDesktopDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionDesktopDevice {
    type Vtable = IDCompositionDesktopDevice_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5f4633fe_1e08_4cb8_8c75_ce24333f5602);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDesktopDevice_Vtbl {
    pub base__: IDCompositionDevice2_Vtbl,
    pub CreateTargetForHwnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: ::win32_foundation::HWND, topmost: ::win32_foundation::BOOL, target: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateSurfaceFromHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handle: ::win32_foundation::HANDLE, surface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateSurfaceFromHwnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: ::win32_foundation::HWND, surface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDCompositionDevice(::windows_core::IUnknown);
impl IDCompositionDevice {
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WaitForCommitCompletion(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WaitForCommitCompletion)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetFrameStatistics(&self) -> ::windows_core::Result<DCOMPOSITION_FRAME_STATISTICS> {
        let mut result__ = ::core::mem::MaybeUninit::<DCOMPOSITION_FRAME_STATISTICS>::zeroed();
        (::windows_core::Interface::vtable(self).GetFrameStatistics)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DCOMPOSITION_FRAME_STATISTICS>(result__)
    }
    pub unsafe fn CreateTargetForHwnd<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, hwnd: Param0, topmost: Param1) -> ::windows_core::Result<IDCompositionTarget> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateTargetForHwnd)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), topmost.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionTarget>(result__)
    }
    pub unsafe fn CreateVisual(&self) -> ::windows_core::Result<IDCompositionVisual> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateVisual)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionVisual>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows_core::Result<IDCompositionSurface> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateSurface)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(pixelformat), ::core::mem::transmute(alphamode), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionSurface>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows_core::Result<IDCompositionVirtualSurface> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateVirtualSurface)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(initialwidth), ::core::mem::transmute(initialheight), ::core::mem::transmute(pixelformat), ::core::mem::transmute(alphamode), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionVirtualSurface>(result__)
    }
    pub unsafe fn CreateSurfaceFromHandle<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(&self, handle: Param0) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).CreateSurfaceFromHandle)(::windows_core::Interface::as_raw(self), handle.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn CreateSurfaceFromHwnd<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(&self, hwnd: Param0) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).CreateSurfaceFromHwnd)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn CreateTranslateTransform(&self) -> ::windows_core::Result<IDCompositionTranslateTransform> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateTranslateTransform)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionTranslateTransform>(result__)
    }
    pub unsafe fn CreateScaleTransform(&self) -> ::windows_core::Result<IDCompositionScaleTransform> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateScaleTransform)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionScaleTransform>(result__)
    }
    pub unsafe fn CreateRotateTransform(&self) -> ::windows_core::Result<IDCompositionRotateTransform> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateRotateTransform)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionRotateTransform>(result__)
    }
    pub unsafe fn CreateSkewTransform(&self) -> ::windows_core::Result<IDCompositionSkewTransform> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateSkewTransform)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionSkewTransform>(result__)
    }
    pub unsafe fn CreateMatrixTransform(&self) -> ::windows_core::Result<IDCompositionMatrixTransform> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateMatrixTransform)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionMatrixTransform>(result__)
    }
    pub unsafe fn CreateTransformGroup(&self, transforms: &[::core::option::Option<IDCompositionTransform>]) -> ::windows_core::Result<IDCompositionTransform> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateTransformGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(transforms)), transforms.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionTransform>(result__)
    }
    pub unsafe fn CreateTranslateTransform3D(&self) -> ::windows_core::Result<IDCompositionTranslateTransform3D> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateTranslateTransform3D)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionTranslateTransform3D>(result__)
    }
    pub unsafe fn CreateScaleTransform3D(&self) -> ::windows_core::Result<IDCompositionScaleTransform3D> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateScaleTransform3D)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionScaleTransform3D>(result__)
    }
    pub unsafe fn CreateRotateTransform3D(&self) -> ::windows_core::Result<IDCompositionRotateTransform3D> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateRotateTransform3D)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionRotateTransform3D>(result__)
    }
    pub unsafe fn CreateMatrixTransform3D(&self) -> ::windows_core::Result<IDCompositionMatrixTransform3D> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateMatrixTransform3D)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionMatrixTransform3D>(result__)
    }
    pub unsafe fn CreateTransform3DGroup(&self, transforms3d: &[::core::option::Option<IDCompositionTransform3D>]) -> ::windows_core::Result<IDCompositionTransform3D> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateTransform3DGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(transforms3d)), transforms3d.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionTransform3D>(result__)
    }
    pub unsafe fn CreateEffectGroup(&self) -> ::windows_core::Result<IDCompositionEffectGroup> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateEffectGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionEffectGroup>(result__)
    }
    pub unsafe fn CreateRectangleClip(&self) -> ::windows_core::Result<IDCompositionRectangleClip> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateRectangleClip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionRectangleClip>(result__)
    }
    pub unsafe fn CreateAnimation(&self) -> ::windows_core::Result<IDCompositionAnimation> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateAnimation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionAnimation>(result__)
    }
    pub unsafe fn CheckDeviceState(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).CheckDeviceState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IDCompositionDevice> for ::windows_core::IUnknown {
    fn from(value: IDCompositionDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionDevice> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionDevice {}
impl ::core::fmt::Debug for IDCompositionDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionDevice {
    type Vtable = IDCompositionDevice_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc37ea93a_e7aa_450d_b16f_9746cb0407f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDevice_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub WaitForCommitCompletion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetFrameStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetFrameStatistics: usize,
    pub CreateTargetForHwnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: ::win32_foundation::HWND, topmost: ::win32_foundation::BOOL, target: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateSurface: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateVirtualSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateVirtualSurface: usize,
    pub CreateSurfaceFromHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handle: ::win32_foundation::HANDLE, surface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateSurfaceFromHwnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: ::win32_foundation::HWND, surface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateTranslateTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, translatetransform: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateScaleTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scaletransform: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateRotateTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rotatetransform: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateSkewTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, skewtransform: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateMatrixTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateTransformGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transforms: *const ::windows_core::RawPtr, elements: u32, transformgroup: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateTranslateTransform3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, translatetransform3d: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateScaleTransform3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scaletransform3d: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateRotateTransform3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rotatetransform3d: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateMatrixTransform3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform3d: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateTransform3DGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transforms3d: *const ::windows_core::RawPtr, elements: u32, transform3dgroup: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateEffectGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effectgroup: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateRectangleClip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clip: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateAnimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CheckDeviceState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfvalid: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDCompositionDevice2(::windows_core::IUnknown);
impl IDCompositionDevice2 {
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WaitForCommitCompletion(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WaitForCommitCompletion)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetFrameStatistics(&self) -> ::windows_core::Result<DCOMPOSITION_FRAME_STATISTICS> {
        let mut result__ = ::core::mem::MaybeUninit::<DCOMPOSITION_FRAME_STATISTICS>::zeroed();
        (::windows_core::Interface::vtable(self).GetFrameStatistics)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DCOMPOSITION_FRAME_STATISTICS>(result__)
    }
    pub unsafe fn CreateVisual(&self) -> ::windows_core::Result<IDCompositionVisual2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateVisual)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionVisual2>(result__)
    }
    pub unsafe fn CreateSurfaceFactory<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, renderingdevice: Param0) -> ::windows_core::Result<IDCompositionSurfaceFactory> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateSurfaceFactory)(::windows_core::Interface::as_raw(self), renderingdevice.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionSurfaceFactory>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows_core::Result<IDCompositionSurface> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateSurface)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(pixelformat), ::core::mem::transmute(alphamode), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionSurface>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows_core::Result<IDCompositionVirtualSurface> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateVirtualSurface)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(initialwidth), ::core::mem::transmute(initialheight), ::core::mem::transmute(pixelformat), ::core::mem::transmute(alphamode), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionVirtualSurface>(result__)
    }
    pub unsafe fn CreateTranslateTransform(&self) -> ::windows_core::Result<IDCompositionTranslateTransform> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateTranslateTransform)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionTranslateTransform>(result__)
    }
    pub unsafe fn CreateScaleTransform(&self) -> ::windows_core::Result<IDCompositionScaleTransform> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateScaleTransform)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionScaleTransform>(result__)
    }
    pub unsafe fn CreateRotateTransform(&self) -> ::windows_core::Result<IDCompositionRotateTransform> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateRotateTransform)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionRotateTransform>(result__)
    }
    pub unsafe fn CreateSkewTransform(&self) -> ::windows_core::Result<IDCompositionSkewTransform> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateSkewTransform)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionSkewTransform>(result__)
    }
    pub unsafe fn CreateMatrixTransform(&self) -> ::windows_core::Result<IDCompositionMatrixTransform> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateMatrixTransform)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionMatrixTransform>(result__)
    }
    pub unsafe fn CreateTransformGroup(&self, transforms: &[::core::option::Option<IDCompositionTransform>]) -> ::windows_core::Result<IDCompositionTransform> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateTransformGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(transforms)), transforms.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionTransform>(result__)
    }
    pub unsafe fn CreateTranslateTransform3D(&self) -> ::windows_core::Result<IDCompositionTranslateTransform3D> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateTranslateTransform3D)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionTranslateTransform3D>(result__)
    }
    pub unsafe fn CreateScaleTransform3D(&self) -> ::windows_core::Result<IDCompositionScaleTransform3D> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateScaleTransform3D)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionScaleTransform3D>(result__)
    }
    pub unsafe fn CreateRotateTransform3D(&self) -> ::windows_core::Result<IDCompositionRotateTransform3D> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateRotateTransform3D)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionRotateTransform3D>(result__)
    }
    pub unsafe fn CreateMatrixTransform3D(&self) -> ::windows_core::Result<IDCompositionMatrixTransform3D> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateMatrixTransform3D)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionMatrixTransform3D>(result__)
    }
    pub unsafe fn CreateTransform3DGroup(&self, transforms3d: &[::core::option::Option<IDCompositionTransform3D>]) -> ::windows_core::Result<IDCompositionTransform3D> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateTransform3DGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(transforms3d)), transforms3d.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionTransform3D>(result__)
    }
    pub unsafe fn CreateEffectGroup(&self) -> ::windows_core::Result<IDCompositionEffectGroup> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateEffectGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionEffectGroup>(result__)
    }
    pub unsafe fn CreateRectangleClip(&self) -> ::windows_core::Result<IDCompositionRectangleClip> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateRectangleClip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionRectangleClip>(result__)
    }
    pub unsafe fn CreateAnimation(&self) -> ::windows_core::Result<IDCompositionAnimation> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateAnimation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionAnimation>(result__)
    }
}
impl ::core::convert::From<IDCompositionDevice2> for ::windows_core::IUnknown {
    fn from(value: IDCompositionDevice2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionDevice2> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionDevice2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionDevice2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionDevice2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionDevice2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionDevice2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionDevice2 {}
impl ::core::fmt::Debug for IDCompositionDevice2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionDevice2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionDevice2 {
    type Vtable = IDCompositionDevice2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x75f6468d_1b8e_447c_9bc6_75fea80b5b25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDevice2_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub WaitForCommitCompletion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetFrameStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetFrameStatistics: usize,
    pub CreateVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateSurfaceFactory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, renderingdevice: *mut ::core::ffi::c_void, surfacefactory: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateSurface: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateVirtualSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateVirtualSurface: usize,
    pub CreateTranslateTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, translatetransform: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateScaleTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scaletransform: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateRotateTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rotatetransform: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateSkewTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, skewtransform: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateMatrixTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateTransformGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transforms: *const ::windows_core::RawPtr, elements: u32, transformgroup: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateTranslateTransform3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, translatetransform3d: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateScaleTransform3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scaletransform3d: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateRotateTransform3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rotatetransform3d: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateMatrixTransform3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform3d: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateTransform3DGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transforms3d: *const ::windows_core::RawPtr, elements: u32, transform3dgroup: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateEffectGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effectgroup: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateRectangleClip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clip: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateAnimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDCompositionDevice3(::windows_core::IUnknown);
impl IDCompositionDevice3 {
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WaitForCommitCompletion(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.WaitForCommitCompletion)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetFrameStatistics(&self) -> ::windows_core::Result<DCOMPOSITION_FRAME_STATISTICS> {
        let mut result__ = ::core::mem::MaybeUninit::<DCOMPOSITION_FRAME_STATISTICS>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetFrameStatistics)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DCOMPOSITION_FRAME_STATISTICS>(result__)
    }
    pub unsafe fn CreateVisual(&self) -> ::windows_core::Result<IDCompositionVisual2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateVisual)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionVisual2>(result__)
    }
    pub unsafe fn CreateSurfaceFactory<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, renderingdevice: Param0) -> ::windows_core::Result<IDCompositionSurfaceFactory> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateSurfaceFactory)(::windows_core::Interface::as_raw(self), renderingdevice.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionSurfaceFactory>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows_core::Result<IDCompositionSurface> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateSurface)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(pixelformat), ::core::mem::transmute(alphamode), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionSurface>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows_core::Result<IDCompositionVirtualSurface> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateVirtualSurface)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(initialwidth), ::core::mem::transmute(initialheight), ::core::mem::transmute(pixelformat), ::core::mem::transmute(alphamode), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionVirtualSurface>(result__)
    }
    pub unsafe fn CreateTranslateTransform(&self) -> ::windows_core::Result<IDCompositionTranslateTransform> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateTranslateTransform)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionTranslateTransform>(result__)
    }
    pub unsafe fn CreateScaleTransform(&self) -> ::windows_core::Result<IDCompositionScaleTransform> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateScaleTransform)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionScaleTransform>(result__)
    }
    pub unsafe fn CreateRotateTransform(&self) -> ::windows_core::Result<IDCompositionRotateTransform> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateRotateTransform)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionRotateTransform>(result__)
    }
    pub unsafe fn CreateSkewTransform(&self) -> ::windows_core::Result<IDCompositionSkewTransform> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateSkewTransform)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionSkewTransform>(result__)
    }
    pub unsafe fn CreateMatrixTransform(&self) -> ::windows_core::Result<IDCompositionMatrixTransform> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateMatrixTransform)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionMatrixTransform>(result__)
    }
    pub unsafe fn CreateTransformGroup(&self, transforms: &[::core::option::Option<IDCompositionTransform>]) -> ::windows_core::Result<IDCompositionTransform> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateTransformGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(transforms)), transforms.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionTransform>(result__)
    }
    pub unsafe fn CreateTranslateTransform3D(&self) -> ::windows_core::Result<IDCompositionTranslateTransform3D> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateTranslateTransform3D)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionTranslateTransform3D>(result__)
    }
    pub unsafe fn CreateScaleTransform3D(&self) -> ::windows_core::Result<IDCompositionScaleTransform3D> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateScaleTransform3D)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionScaleTransform3D>(result__)
    }
    pub unsafe fn CreateRotateTransform3D(&self) -> ::windows_core::Result<IDCompositionRotateTransform3D> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateRotateTransform3D)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionRotateTransform3D>(result__)
    }
    pub unsafe fn CreateMatrixTransform3D(&self) -> ::windows_core::Result<IDCompositionMatrixTransform3D> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateMatrixTransform3D)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionMatrixTransform3D>(result__)
    }
    pub unsafe fn CreateTransform3DGroup(&self, transforms3d: &[::core::option::Option<IDCompositionTransform3D>]) -> ::windows_core::Result<IDCompositionTransform3D> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateTransform3DGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(transforms3d)), transforms3d.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionTransform3D>(result__)
    }
    pub unsafe fn CreateEffectGroup(&self) -> ::windows_core::Result<IDCompositionEffectGroup> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateEffectGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionEffectGroup>(result__)
    }
    pub unsafe fn CreateRectangleClip(&self) -> ::windows_core::Result<IDCompositionRectangleClip> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateRectangleClip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionRectangleClip>(result__)
    }
    pub unsafe fn CreateAnimation(&self) -> ::windows_core::Result<IDCompositionAnimation> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateAnimation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionAnimation>(result__)
    }
    pub unsafe fn CreateGaussianBlurEffect(&self) -> ::windows_core::Result<IDCompositionGaussianBlurEffect> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateGaussianBlurEffect)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionGaussianBlurEffect>(result__)
    }
    pub unsafe fn CreateBrightnessEffect(&self) -> ::windows_core::Result<IDCompositionBrightnessEffect> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateBrightnessEffect)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionBrightnessEffect>(result__)
    }
    pub unsafe fn CreateColorMatrixEffect(&self) -> ::windows_core::Result<IDCompositionColorMatrixEffect> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateColorMatrixEffect)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionColorMatrixEffect>(result__)
    }
    pub unsafe fn CreateShadowEffect(&self) -> ::windows_core::Result<IDCompositionShadowEffect> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateShadowEffect)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionShadowEffect>(result__)
    }
    pub unsafe fn CreateHueRotationEffect(&self) -> ::windows_core::Result<IDCompositionHueRotationEffect> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateHueRotationEffect)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionHueRotationEffect>(result__)
    }
    pub unsafe fn CreateSaturationEffect(&self) -> ::windows_core::Result<IDCompositionSaturationEffect> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateSaturationEffect)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionSaturationEffect>(result__)
    }
    pub unsafe fn CreateTurbulenceEffect(&self) -> ::windows_core::Result<IDCompositionTurbulenceEffect> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateTurbulenceEffect)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionTurbulenceEffect>(result__)
    }
    pub unsafe fn CreateLinearTransferEffect(&self) -> ::windows_core::Result<IDCompositionLinearTransferEffect> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateLinearTransferEffect)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionLinearTransferEffect>(result__)
    }
    pub unsafe fn CreateTableTransferEffect(&self) -> ::windows_core::Result<IDCompositionTableTransferEffect> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateTableTransferEffect)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionTableTransferEffect>(result__)
    }
    pub unsafe fn CreateCompositeEffect(&self) -> ::windows_core::Result<IDCompositionCompositeEffect> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateCompositeEffect)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionCompositeEffect>(result__)
    }
    pub unsafe fn CreateBlendEffect(&self) -> ::windows_core::Result<IDCompositionBlendEffect> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateBlendEffect)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionBlendEffect>(result__)
    }
    pub unsafe fn CreateArithmeticCompositeEffect(&self) -> ::windows_core::Result<IDCompositionArithmeticCompositeEffect> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateArithmeticCompositeEffect)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionArithmeticCompositeEffect>(result__)
    }
    pub unsafe fn CreateAffineTransform2DEffect(&self) -> ::windows_core::Result<IDCompositionAffineTransform2DEffect> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateAffineTransform2DEffect)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionAffineTransform2DEffect>(result__)
    }
}
impl ::core::convert::From<IDCompositionDevice3> for ::windows_core::IUnknown {
    fn from(value: IDCompositionDevice3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionDevice3> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionDevice3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionDevice3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionDevice3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionDevice3> for IDCompositionDevice2 {
    fn from(value: IDCompositionDevice3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionDevice3> for IDCompositionDevice2 {
    fn from(value: &IDCompositionDevice3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionDevice2> for IDCompositionDevice3 {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionDevice2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionDevice2> for &'a IDCompositionDevice3 {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionDevice2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionDevice3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionDevice3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionDevice3 {}
impl ::core::fmt::Debug for IDCompositionDevice3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionDevice3").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionDevice3 {
    type Vtable = IDCompositionDevice3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0987cb06_f916_48bf_8d35_ce7641781bd9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDevice3_Vtbl {
    pub base__: IDCompositionDevice2_Vtbl,
    pub CreateGaussianBlurEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gaussianblureffect: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateBrightnessEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brightnesseffect: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateColorMatrixEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colormatrixeffect: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateShadowEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shadoweffect: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateHueRotationEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, huerotationeffect: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateSaturationEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, saturationeffect: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateTurbulenceEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, turbulenceeffect: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateLinearTransferEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lineartransfereffect: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateTableTransferEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tabletransfereffect: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateCompositeEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositeeffect: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateBlendEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blendeffect: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateArithmeticCompositeEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, arithmeticcompositeeffect: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateAffineTransform2DEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, affinetransform2deffect: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDCompositionDeviceDebug(::windows_core::IUnknown);
impl IDCompositionDeviceDebug {
    pub unsafe fn EnableDebugCounters(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnableDebugCounters)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DisableDebugCounters(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DisableDebugCounters)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IDCompositionDeviceDebug> for ::windows_core::IUnknown {
    fn from(value: IDCompositionDeviceDebug) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionDeviceDebug> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionDeviceDebug) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionDeviceDebug {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionDeviceDebug {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionDeviceDebug {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionDeviceDebug {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionDeviceDebug {}
impl ::core::fmt::Debug for IDCompositionDeviceDebug {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionDeviceDebug").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionDeviceDebug {
    type Vtable = IDCompositionDeviceDebug_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa1a3c64a_224f_4a81_9773_4f03a89d3c6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDeviceDebug_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub EnableDebugCounters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DisableDebugCounters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDCompositionEffect(::windows_core::IUnknown);
impl IDCompositionEffect {}
impl ::core::convert::From<IDCompositionEffect> for ::windows_core::IUnknown {
    fn from(value: IDCompositionEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionEffect> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionEffect {}
impl ::core::fmt::Debug for IDCompositionEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionEffect {
    type Vtable = IDCompositionEffect_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xec81b08f_bfcb_4e8d_b193_a915587999e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionEffect_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
}
#[repr(transparent)]
pub struct IDCompositionEffectGroup(::windows_core::IUnknown);
impl IDCompositionEffectGroup {
    pub unsafe fn SetOpacity<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOpacity)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetOpacity2(&self, opacity: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOpacity2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(opacity)).ok()
    }
    pub unsafe fn SetTransform3D<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionTransform3D>>(&self, transform3d: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTransform3D)(::windows_core::Interface::as_raw(self), transform3d.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IDCompositionEffectGroup> for ::windows_core::IUnknown {
    fn from(value: IDCompositionEffectGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionEffectGroup> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionEffectGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionEffectGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionEffectGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionEffectGroup> for IDCompositionEffect {
    fn from(value: IDCompositionEffectGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionEffectGroup> for IDCompositionEffect {
    fn from(value: &IDCompositionEffectGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for IDCompositionEffectGroup {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionEffectGroup {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionEffectGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionEffectGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionEffectGroup {}
impl ::core::fmt::Debug for IDCompositionEffectGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionEffectGroup").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionEffectGroup {
    type Vtable = IDCompositionEffectGroup_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa7929a74_e6b2_4bd6_8b95_4040119ca34d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionEffectGroup_Vtbl {
    pub base__: IDCompositionEffect_Vtbl,
    pub SetOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetOpacity2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows_core::HRESULT,
    pub SetTransform3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform3d: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDCompositionFilterEffect(::windows_core::IUnknown);
impl IDCompositionFilterEffect {
    pub unsafe fn SetInput<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
}
impl ::core::convert::From<IDCompositionFilterEffect> for ::windows_core::IUnknown {
    fn from(value: IDCompositionFilterEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionFilterEffect> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionFilterEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionFilterEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionFilterEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionFilterEffect> for IDCompositionEffect {
    fn from(value: IDCompositionFilterEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionFilterEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionFilterEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for IDCompositionFilterEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionFilterEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionFilterEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionFilterEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionFilterEffect {}
impl ::core::fmt::Debug for IDCompositionFilterEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionFilterEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionFilterEffect {
    type Vtable = IDCompositionFilterEffect_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x30c421d5_8cb2_4e9f_b133_37be270d4ac2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionFilterEffect_Vtbl {
    pub base__: IDCompositionEffect_Vtbl,
    pub SetInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, input: *mut ::core::ffi::c_void, flags: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDCompositionGaussianBlurEffect(::windows_core::IUnknown);
impl IDCompositionGaussianBlurEffect {
    pub unsafe fn SetInput<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    pub unsafe fn SetStandardDeviation<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStandardDeviation)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetStandardDeviation2(&self, amount: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStandardDeviation2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(amount)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetBorderMode(&self, mode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBorderMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(mode)).ok()
    }
}
impl ::core::convert::From<IDCompositionGaussianBlurEffect> for ::windows_core::IUnknown {
    fn from(value: IDCompositionGaussianBlurEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionGaussianBlurEffect> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionGaussianBlurEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionGaussianBlurEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionGaussianBlurEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionGaussianBlurEffect> for IDCompositionEffect {
    fn from(value: IDCompositionGaussianBlurEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionGaussianBlurEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionGaussianBlurEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for IDCompositionGaussianBlurEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionGaussianBlurEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionGaussianBlurEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionGaussianBlurEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionGaussianBlurEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionGaussianBlurEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionFilterEffect> for IDCompositionGaussianBlurEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionFilterEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionFilterEffect> for &'a IDCompositionGaussianBlurEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionFilterEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionGaussianBlurEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionGaussianBlurEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionGaussianBlurEffect {}
impl ::core::fmt::Debug for IDCompositionGaussianBlurEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionGaussianBlurEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionGaussianBlurEffect {
    type Vtable = IDCompositionGaussianBlurEffect_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x45d4d0b7_1bd4_454e_8894_2bfa68443033);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionGaussianBlurEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    pub SetStandardDeviation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetStandardDeviation2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetBorderMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetBorderMode: usize,
}
#[repr(transparent)]
pub struct IDCompositionHueRotationEffect(::windows_core::IUnknown);
impl IDCompositionHueRotationEffect {
    pub unsafe fn SetInput<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    pub unsafe fn SetAngle<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAngle)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetAngle2(&self, amountdegrees: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAngle2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(amountdegrees)).ok()
    }
}
impl ::core::convert::From<IDCompositionHueRotationEffect> for ::windows_core::IUnknown {
    fn from(value: IDCompositionHueRotationEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionHueRotationEffect> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionHueRotationEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionHueRotationEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionHueRotationEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionHueRotationEffect> for IDCompositionEffect {
    fn from(value: IDCompositionHueRotationEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionHueRotationEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionHueRotationEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for IDCompositionHueRotationEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionHueRotationEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionHueRotationEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionHueRotationEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionHueRotationEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionHueRotationEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionFilterEffect> for IDCompositionHueRotationEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionFilterEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionFilterEffect> for &'a IDCompositionHueRotationEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionFilterEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionHueRotationEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionHueRotationEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionHueRotationEffect {}
impl ::core::fmt::Debug for IDCompositionHueRotationEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionHueRotationEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionHueRotationEffect {
    type Vtable = IDCompositionHueRotationEffect_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6db9f920_0770_4781_b0c6_381912f9d167);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionHueRotationEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    pub SetAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetAngle2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, amountdegrees: f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDCompositionInkTrailDevice(::windows_core::IUnknown);
impl IDCompositionInkTrailDevice {
    pub unsafe fn CreateDelegatedInkTrail(&self) -> ::windows_core::Result<IDCompositionDelegatedInkTrail> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateDelegatedInkTrail)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionDelegatedInkTrail>(result__)
    }
    pub unsafe fn CreateDelegatedInkTrailForSwapChain<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, swapchain: Param0) -> ::windows_core::Result<IDCompositionDelegatedInkTrail> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateDelegatedInkTrailForSwapChain)(::windows_core::Interface::as_raw(self), swapchain.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionDelegatedInkTrail>(result__)
    }
}
impl ::core::convert::From<IDCompositionInkTrailDevice> for ::windows_core::IUnknown {
    fn from(value: IDCompositionInkTrailDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionInkTrailDevice> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionInkTrailDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionInkTrailDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionInkTrailDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionInkTrailDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionInkTrailDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionInkTrailDevice {}
impl ::core::fmt::Debug for IDCompositionInkTrailDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionInkTrailDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionInkTrailDevice {
    type Vtable = IDCompositionInkTrailDevice_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdf0c7cec_cdeb_4d4a_b91c_721bf22f4e6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionInkTrailDevice_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub CreateDelegatedInkTrail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inktrail: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateDelegatedInkTrailForSwapChain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, swapchain: *mut ::core::ffi::c_void, inktrail: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDCompositionLinearTransferEffect(::windows_core::IUnknown);
impl IDCompositionLinearTransferEffect {
    pub unsafe fn SetInput<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    pub unsafe fn SetRedYIntercept<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRedYIntercept)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetRedYIntercept2(&self, redyintercept: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRedYIntercept2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(redyintercept)).ok()
    }
    pub unsafe fn SetRedSlope<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRedSlope)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetRedSlope2(&self, redslope: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRedSlope2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(redslope)).ok()
    }
    pub unsafe fn SetRedDisable<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, reddisable: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRedDisable)(::windows_core::Interface::as_raw(self), reddisable.into_param().abi()).ok()
    }
    pub unsafe fn SetGreenYIntercept<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetGreenYIntercept)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetGreenYIntercept2(&self, greenyintercept: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetGreenYIntercept2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(greenyintercept)).ok()
    }
    pub unsafe fn SetGreenSlope<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetGreenSlope)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetGreenSlope2(&self, greenslope: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetGreenSlope2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(greenslope)).ok()
    }
    pub unsafe fn SetGreenDisable<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, greendisable: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetGreenDisable)(::windows_core::Interface::as_raw(self), greendisable.into_param().abi()).ok()
    }
    pub unsafe fn SetBlueYIntercept<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBlueYIntercept)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetBlueYIntercept2(&self, blueyintercept: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBlueYIntercept2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(blueyintercept)).ok()
    }
    pub unsafe fn SetBlueSlope<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBlueSlope)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetBlueSlope2(&self, blueslope: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBlueSlope2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(blueslope)).ok()
    }
    pub unsafe fn SetBlueDisable<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, bluedisable: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBlueDisable)(::windows_core::Interface::as_raw(self), bluedisable.into_param().abi()).ok()
    }
    pub unsafe fn SetAlphaYIntercept<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAlphaYIntercept)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetAlphaYIntercept2(&self, alphayintercept: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAlphaYIntercept2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(alphayintercept)).ok()
    }
    pub unsafe fn SetAlphaSlope<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAlphaSlope)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetAlphaSlope2(&self, alphaslope: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAlphaSlope2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(alphaslope)).ok()
    }
    pub unsafe fn SetAlphaDisable<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, alphadisable: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAlphaDisable)(::windows_core::Interface::as_raw(self), alphadisable.into_param().abi()).ok()
    }
    pub unsafe fn SetClampOutput<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, clampoutput: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetClampOutput)(::windows_core::Interface::as_raw(self), clampoutput.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IDCompositionLinearTransferEffect> for ::windows_core::IUnknown {
    fn from(value: IDCompositionLinearTransferEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionLinearTransferEffect> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionLinearTransferEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionLinearTransferEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionLinearTransferEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionLinearTransferEffect> for IDCompositionEffect {
    fn from(value: IDCompositionLinearTransferEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionLinearTransferEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionLinearTransferEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for IDCompositionLinearTransferEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionLinearTransferEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionLinearTransferEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionLinearTransferEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionLinearTransferEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionLinearTransferEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionFilterEffect> for IDCompositionLinearTransferEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionFilterEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionFilterEffect> for &'a IDCompositionLinearTransferEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionFilterEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionLinearTransferEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionLinearTransferEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionLinearTransferEffect {}
impl ::core::fmt::Debug for IDCompositionLinearTransferEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionLinearTransferEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionLinearTransferEffect {
    type Vtable = IDCompositionLinearTransferEffect_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4305ee5b_c4a0_4c88_9385_67124e017683);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionLinearTransferEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    pub SetRedYIntercept: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetRedYIntercept2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, redyintercept: f32) -> ::windows_core::HRESULT,
    pub SetRedSlope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetRedSlope2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, redslope: f32) -> ::windows_core::HRESULT,
    pub SetRedDisable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reddisable: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetGreenYIntercept: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetGreenYIntercept2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, greenyintercept: f32) -> ::windows_core::HRESULT,
    pub SetGreenSlope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetGreenSlope2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, greenslope: f32) -> ::windows_core::HRESULT,
    pub SetGreenDisable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, greendisable: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetBlueYIntercept: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetBlueYIntercept2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blueyintercept: f32) -> ::windows_core::HRESULT,
    pub SetBlueSlope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetBlueSlope2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blueslope: f32) -> ::windows_core::HRESULT,
    pub SetBlueDisable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bluedisable: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetAlphaYIntercept: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetAlphaYIntercept2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alphayintercept: f32) -> ::windows_core::HRESULT,
    pub SetAlphaSlope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetAlphaSlope2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alphaslope: f32) -> ::windows_core::HRESULT,
    pub SetAlphaDisable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alphadisable: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetClampOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clampoutput: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDCompositionMatrixTransform(::windows_core::IUnknown);
impl IDCompositionMatrixTransform {
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetMatrix(&self, matrix: *const ::winrt_foundation::Numerics::Matrix3x2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMatrix)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(matrix)).ok()
    }
    pub unsafe fn SetMatrixElement<'a, Param2: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, row: i32, column: i32, animation: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMatrixElement)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(row), ::core::mem::transmute(column), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetMatrixElement2(&self, row: i32, column: i32, value: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMatrixElement2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(row), ::core::mem::transmute(column), ::core::mem::transmute(value)).ok()
    }
}
impl ::core::convert::From<IDCompositionMatrixTransform> for ::windows_core::IUnknown {
    fn from(value: IDCompositionMatrixTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionMatrixTransform> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionMatrixTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionMatrixTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionMatrixTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionMatrixTransform> for IDCompositionEffect {
    fn from(value: IDCompositionMatrixTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionMatrixTransform> for IDCompositionEffect {
    fn from(value: &IDCompositionMatrixTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for IDCompositionMatrixTransform {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionMatrixTransform {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionMatrixTransform> for IDCompositionTransform3D {
    fn from(value: IDCompositionMatrixTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionMatrixTransform> for IDCompositionTransform3D {
    fn from(value: &IDCompositionMatrixTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionTransform3D> for IDCompositionMatrixTransform {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionTransform3D> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionTransform3D> for &'a IDCompositionMatrixTransform {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionTransform3D> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionMatrixTransform> for IDCompositionTransform {
    fn from(value: IDCompositionMatrixTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionMatrixTransform> for IDCompositionTransform {
    fn from(value: &IDCompositionMatrixTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionTransform> for IDCompositionMatrixTransform {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionTransform> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionTransform> for &'a IDCompositionMatrixTransform {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionTransform> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionMatrixTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionMatrixTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionMatrixTransform {}
impl ::core::fmt::Debug for IDCompositionMatrixTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionMatrixTransform").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionMatrixTransform {
    type Vtable = IDCompositionMatrixTransform_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x16cdff07_c503_419c_83f2_0965c7af1fa6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionMatrixTransform_Vtbl {
    pub base__: IDCompositionTransform_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const ::winrt_foundation::Numerics::Matrix3x2) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetMatrix: usize,
    pub SetMatrixElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetMatrixElement2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDCompositionMatrixTransform3D(::windows_core::IUnknown);
impl IDCompositionMatrixTransform3D {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn SetMatrix(&self, matrix: *const super::Direct3D::D3DMATRIX) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMatrix)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(matrix)).ok()
    }
    pub unsafe fn SetMatrixElement<'a, Param2: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, row: i32, column: i32, animation: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMatrixElement)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(row), ::core::mem::transmute(column), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetMatrixElement2(&self, row: i32, column: i32, value: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMatrixElement2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(row), ::core::mem::transmute(column), ::core::mem::transmute(value)).ok()
    }
}
impl ::core::convert::From<IDCompositionMatrixTransform3D> for ::windows_core::IUnknown {
    fn from(value: IDCompositionMatrixTransform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionMatrixTransform3D> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionMatrixTransform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionMatrixTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionMatrixTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionMatrixTransform3D> for IDCompositionEffect {
    fn from(value: IDCompositionMatrixTransform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionMatrixTransform3D> for IDCompositionEffect {
    fn from(value: &IDCompositionMatrixTransform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for IDCompositionMatrixTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionMatrixTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionMatrixTransform3D> for IDCompositionTransform3D {
    fn from(value: IDCompositionMatrixTransform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionMatrixTransform3D> for IDCompositionTransform3D {
    fn from(value: &IDCompositionMatrixTransform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionTransform3D> for IDCompositionMatrixTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionTransform3D> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionTransform3D> for &'a IDCompositionMatrixTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionTransform3D> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionMatrixTransform3D {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionMatrixTransform3D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionMatrixTransform3D {}
impl ::core::fmt::Debug for IDCompositionMatrixTransform3D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionMatrixTransform3D").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionMatrixTransform3D {
    type Vtable = IDCompositionMatrixTransform3D_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4b3363f0_643b_41b7_b6e0_ccf22d34467c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionMatrixTransform3D_Vtbl {
    pub base__: IDCompositionTransform3D_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub SetMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const super::Direct3D::D3DMATRIX) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    SetMatrix: usize,
    pub SetMatrixElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetMatrixElement2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDCompositionRectangleClip(::windows_core::IUnknown);
impl IDCompositionRectangleClip {
    pub unsafe fn SetLeft<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLeft)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetLeft2(&self, left: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLeft2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(left)).ok()
    }
    pub unsafe fn SetTop<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTop)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetTop2(&self, top: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTop2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(top)).ok()
    }
    pub unsafe fn SetRight<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRight)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetRight2(&self, right: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRight2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(right)).ok()
    }
    pub unsafe fn SetBottom<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBottom)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetBottom2(&self, bottom: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBottom2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(bottom)).ok()
    }
    pub unsafe fn SetTopLeftRadiusX<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTopLeftRadiusX)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetTopLeftRadiusX2(&self, radius: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTopLeftRadiusX2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(radius)).ok()
    }
    pub unsafe fn SetTopLeftRadiusY<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTopLeftRadiusY)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetTopLeftRadiusY2(&self, radius: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTopLeftRadiusY2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(radius)).ok()
    }
    pub unsafe fn SetTopRightRadiusX<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTopRightRadiusX)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetTopRightRadiusX2(&self, radius: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTopRightRadiusX2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(radius)).ok()
    }
    pub unsafe fn SetTopRightRadiusY<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTopRightRadiusY)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetTopRightRadiusY2(&self, radius: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTopRightRadiusY2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(radius)).ok()
    }
    pub unsafe fn SetBottomLeftRadiusX<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBottomLeftRadiusX)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetBottomLeftRadiusX2(&self, radius: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBottomLeftRadiusX2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(radius)).ok()
    }
    pub unsafe fn SetBottomLeftRadiusY<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBottomLeftRadiusY)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetBottomLeftRadiusY2(&self, radius: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBottomLeftRadiusY2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(radius)).ok()
    }
    pub unsafe fn SetBottomRightRadiusX<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBottomRightRadiusX)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetBottomRightRadiusX2(&self, radius: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBottomRightRadiusX2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(radius)).ok()
    }
    pub unsafe fn SetBottomRightRadiusY<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBottomRightRadiusY)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetBottomRightRadiusY2(&self, radius: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBottomRightRadiusY2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(radius)).ok()
    }
}
impl ::core::convert::From<IDCompositionRectangleClip> for ::windows_core::IUnknown {
    fn from(value: IDCompositionRectangleClip) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionRectangleClip> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionRectangleClip) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionRectangleClip {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionRectangleClip {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionRectangleClip> for IDCompositionClip {
    fn from(value: IDCompositionRectangleClip) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionRectangleClip> for IDCompositionClip {
    fn from(value: &IDCompositionRectangleClip) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionClip> for IDCompositionRectangleClip {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionClip> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionClip> for &'a IDCompositionRectangleClip {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionClip> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionRectangleClip {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionRectangleClip {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionRectangleClip {}
impl ::core::fmt::Debug for IDCompositionRectangleClip {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionRectangleClip").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionRectangleClip {
    type Vtable = IDCompositionRectangleClip_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9842ad7d_d9cf_4908_aed7_48b51da5e7c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionRectangleClip_Vtbl {
    pub base__: IDCompositionClip_Vtbl,
    pub SetLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetLeft2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, left: f32) -> ::windows_core::HRESULT,
    pub SetTop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetTop2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, top: f32) -> ::windows_core::HRESULT,
    pub SetRight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetRight2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, right: f32) -> ::windows_core::HRESULT,
    pub SetBottom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetBottom2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bottom: f32) -> ::windows_core::HRESULT,
    pub SetTopLeftRadiusX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetTopLeftRadiusX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows_core::HRESULT,
    pub SetTopLeftRadiusY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetTopLeftRadiusY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows_core::HRESULT,
    pub SetTopRightRadiusX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetTopRightRadiusX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows_core::HRESULT,
    pub SetTopRightRadiusY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetTopRightRadiusY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows_core::HRESULT,
    pub SetBottomLeftRadiusX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetBottomLeftRadiusX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows_core::HRESULT,
    pub SetBottomLeftRadiusY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetBottomLeftRadiusY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows_core::HRESULT,
    pub SetBottomRightRadiusX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetBottomRightRadiusX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows_core::HRESULT,
    pub SetBottomRightRadiusY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetBottomRightRadiusY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDCompositionRotateTransform(::windows_core::IUnknown);
impl IDCompositionRotateTransform {
    pub unsafe fn SetAngle<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAngle)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetAngle2(&self, angle: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAngle2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(angle)).ok()
    }
    pub unsafe fn SetCenterX<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCenterX)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCenterX2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(centerx)).ok()
    }
    pub unsafe fn SetCenterY<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCenterY)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetCenterY2(&self, centery: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCenterY2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(centery)).ok()
    }
}
impl ::core::convert::From<IDCompositionRotateTransform> for ::windows_core::IUnknown {
    fn from(value: IDCompositionRotateTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionRotateTransform> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionRotateTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionRotateTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionRotateTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionRotateTransform> for IDCompositionEffect {
    fn from(value: IDCompositionRotateTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionRotateTransform> for IDCompositionEffect {
    fn from(value: &IDCompositionRotateTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for IDCompositionRotateTransform {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionRotateTransform {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionRotateTransform> for IDCompositionTransform3D {
    fn from(value: IDCompositionRotateTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionRotateTransform> for IDCompositionTransform3D {
    fn from(value: &IDCompositionRotateTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionTransform3D> for IDCompositionRotateTransform {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionTransform3D> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionTransform3D> for &'a IDCompositionRotateTransform {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionTransform3D> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionRotateTransform> for IDCompositionTransform {
    fn from(value: IDCompositionRotateTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionRotateTransform> for IDCompositionTransform {
    fn from(value: &IDCompositionRotateTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionTransform> for IDCompositionRotateTransform {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionTransform> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionTransform> for &'a IDCompositionRotateTransform {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionTransform> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionRotateTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionRotateTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionRotateTransform {}
impl ::core::fmt::Debug for IDCompositionRotateTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionRotateTransform").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionRotateTransform {
    type Vtable = IDCompositionRotateTransform_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x641ed83c_ae96_46c5_90dc_32774cc5c6d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionRotateTransform_Vtbl {
    pub base__: IDCompositionTransform_Vtbl,
    pub SetAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetAngle2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, angle: f32) -> ::windows_core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetCenterX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows_core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetCenterY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDCompositionRotateTransform3D(::windows_core::IUnknown);
impl IDCompositionRotateTransform3D {
    pub unsafe fn SetAngle<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAngle)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetAngle2(&self, angle: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAngle2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(angle)).ok()
    }
    pub unsafe fn SetAxisX<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAxisX)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetAxisX2(&self, axisx: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAxisX2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(axisx)).ok()
    }
    pub unsafe fn SetAxisY<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAxisY)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetAxisY2(&self, axisy: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAxisY2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(axisy)).ok()
    }
    pub unsafe fn SetAxisZ<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAxisZ)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetAxisZ2(&self, axisz: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAxisZ2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(axisz)).ok()
    }
    pub unsafe fn SetCenterX<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCenterX)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCenterX2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(centerx)).ok()
    }
    pub unsafe fn SetCenterY<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCenterY)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetCenterY2(&self, centery: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCenterY2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(centery)).ok()
    }
    pub unsafe fn SetCenterZ<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCenterZ)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetCenterZ2(&self, centerz: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCenterZ2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(centerz)).ok()
    }
}
impl ::core::convert::From<IDCompositionRotateTransform3D> for ::windows_core::IUnknown {
    fn from(value: IDCompositionRotateTransform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionRotateTransform3D> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionRotateTransform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionRotateTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionRotateTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionRotateTransform3D> for IDCompositionEffect {
    fn from(value: IDCompositionRotateTransform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionRotateTransform3D> for IDCompositionEffect {
    fn from(value: &IDCompositionRotateTransform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for IDCompositionRotateTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionRotateTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionRotateTransform3D> for IDCompositionTransform3D {
    fn from(value: IDCompositionRotateTransform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionRotateTransform3D> for IDCompositionTransform3D {
    fn from(value: &IDCompositionRotateTransform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionTransform3D> for IDCompositionRotateTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionTransform3D> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionTransform3D> for &'a IDCompositionRotateTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionTransform3D> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionRotateTransform3D {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionRotateTransform3D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionRotateTransform3D {}
impl ::core::fmt::Debug for IDCompositionRotateTransform3D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionRotateTransform3D").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionRotateTransform3D {
    type Vtable = IDCompositionRotateTransform3D_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd8f5b23f_d429_4a91_b55a_d2f45fd75b18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionRotateTransform3D_Vtbl {
    pub base__: IDCompositionTransform3D_Vtbl,
    pub SetAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetAngle2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, angle: f32) -> ::windows_core::HRESULT,
    pub SetAxisX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetAxisX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, axisx: f32) -> ::windows_core::HRESULT,
    pub SetAxisY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetAxisY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, axisy: f32) -> ::windows_core::HRESULT,
    pub SetAxisZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetAxisZ2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, axisz: f32) -> ::windows_core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetCenterX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows_core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetCenterY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows_core::HRESULT,
    pub SetCenterZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetCenterZ2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centerz: f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDCompositionSaturationEffect(::windows_core::IUnknown);
impl IDCompositionSaturationEffect {
    pub unsafe fn SetInput<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    pub unsafe fn SetSaturation<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSaturation)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetSaturation2(&self, ratio: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSaturation2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ratio)).ok()
    }
}
impl ::core::convert::From<IDCompositionSaturationEffect> for ::windows_core::IUnknown {
    fn from(value: IDCompositionSaturationEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionSaturationEffect> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionSaturationEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionSaturationEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionSaturationEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionSaturationEffect> for IDCompositionEffect {
    fn from(value: IDCompositionSaturationEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionSaturationEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionSaturationEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for IDCompositionSaturationEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionSaturationEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionSaturationEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionSaturationEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionSaturationEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionSaturationEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionFilterEffect> for IDCompositionSaturationEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionFilterEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionFilterEffect> for &'a IDCompositionSaturationEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionFilterEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionSaturationEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionSaturationEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionSaturationEffect {}
impl ::core::fmt::Debug for IDCompositionSaturationEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionSaturationEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionSaturationEffect {
    type Vtable = IDCompositionSaturationEffect_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa08debda_3258_4fa4_9f16_9174d3fe93b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionSaturationEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    pub SetSaturation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetSaturation2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ratio: f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDCompositionScaleTransform(::windows_core::IUnknown);
impl IDCompositionScaleTransform {
    pub unsafe fn SetScaleX<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetScaleX)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetScaleX2(&self, scalex: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetScaleX2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(scalex)).ok()
    }
    pub unsafe fn SetScaleY<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetScaleY)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetScaleY2(&self, scaley: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetScaleY2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(scaley)).ok()
    }
    pub unsafe fn SetCenterX<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCenterX)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCenterX2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(centerx)).ok()
    }
    pub unsafe fn SetCenterY<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCenterY)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetCenterY2(&self, centery: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCenterY2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(centery)).ok()
    }
}
impl ::core::convert::From<IDCompositionScaleTransform> for ::windows_core::IUnknown {
    fn from(value: IDCompositionScaleTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionScaleTransform> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionScaleTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionScaleTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionScaleTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionScaleTransform> for IDCompositionEffect {
    fn from(value: IDCompositionScaleTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionScaleTransform> for IDCompositionEffect {
    fn from(value: &IDCompositionScaleTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for IDCompositionScaleTransform {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionScaleTransform {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionScaleTransform> for IDCompositionTransform3D {
    fn from(value: IDCompositionScaleTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionScaleTransform> for IDCompositionTransform3D {
    fn from(value: &IDCompositionScaleTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionTransform3D> for IDCompositionScaleTransform {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionTransform3D> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionTransform3D> for &'a IDCompositionScaleTransform {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionTransform3D> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionScaleTransform> for IDCompositionTransform {
    fn from(value: IDCompositionScaleTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionScaleTransform> for IDCompositionTransform {
    fn from(value: &IDCompositionScaleTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionTransform> for IDCompositionScaleTransform {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionTransform> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionTransform> for &'a IDCompositionScaleTransform {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionTransform> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionScaleTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionScaleTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionScaleTransform {}
impl ::core::fmt::Debug for IDCompositionScaleTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionScaleTransform").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionScaleTransform {
    type Vtable = IDCompositionScaleTransform_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x71fde914_40ef_45ef_bd51_68b037c339f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionScaleTransform_Vtbl {
    pub base__: IDCompositionTransform_Vtbl,
    pub SetScaleX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetScaleX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scalex: f32) -> ::windows_core::HRESULT,
    pub SetScaleY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetScaleY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scaley: f32) -> ::windows_core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetCenterX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows_core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetCenterY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDCompositionScaleTransform3D(::windows_core::IUnknown);
impl IDCompositionScaleTransform3D {
    pub unsafe fn SetScaleX<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetScaleX)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetScaleX2(&self, scalex: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetScaleX2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(scalex)).ok()
    }
    pub unsafe fn SetScaleY<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetScaleY)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetScaleY2(&self, scaley: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetScaleY2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(scaley)).ok()
    }
    pub unsafe fn SetScaleZ<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetScaleZ)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetScaleZ2(&self, scalez: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetScaleZ2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(scalez)).ok()
    }
    pub unsafe fn SetCenterX<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCenterX)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCenterX2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(centerx)).ok()
    }
    pub unsafe fn SetCenterY<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCenterY)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetCenterY2(&self, centery: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCenterY2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(centery)).ok()
    }
    pub unsafe fn SetCenterZ<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCenterZ)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetCenterZ2(&self, centerz: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCenterZ2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(centerz)).ok()
    }
}
impl ::core::convert::From<IDCompositionScaleTransform3D> for ::windows_core::IUnknown {
    fn from(value: IDCompositionScaleTransform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionScaleTransform3D> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionScaleTransform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionScaleTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionScaleTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionScaleTransform3D> for IDCompositionEffect {
    fn from(value: IDCompositionScaleTransform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionScaleTransform3D> for IDCompositionEffect {
    fn from(value: &IDCompositionScaleTransform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for IDCompositionScaleTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionScaleTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionScaleTransform3D> for IDCompositionTransform3D {
    fn from(value: IDCompositionScaleTransform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionScaleTransform3D> for IDCompositionTransform3D {
    fn from(value: &IDCompositionScaleTransform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionTransform3D> for IDCompositionScaleTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionTransform3D> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionTransform3D> for &'a IDCompositionScaleTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionTransform3D> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionScaleTransform3D {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionScaleTransform3D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionScaleTransform3D {}
impl ::core::fmt::Debug for IDCompositionScaleTransform3D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionScaleTransform3D").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionScaleTransform3D {
    type Vtable = IDCompositionScaleTransform3D_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2a9e9ead_364b_4b15_a7c4_a1997f78b389);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionScaleTransform3D_Vtbl {
    pub base__: IDCompositionTransform3D_Vtbl,
    pub SetScaleX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetScaleX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scalex: f32) -> ::windows_core::HRESULT,
    pub SetScaleY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetScaleY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scaley: f32) -> ::windows_core::HRESULT,
    pub SetScaleZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetScaleZ2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scalez: f32) -> ::windows_core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetCenterX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows_core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetCenterY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows_core::HRESULT,
    pub SetCenterZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetCenterZ2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centerz: f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDCompositionShadowEffect(::windows_core::IUnknown);
impl IDCompositionShadowEffect {
    pub unsafe fn SetInput<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    pub unsafe fn SetStandardDeviation<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStandardDeviation)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetStandardDeviation2(&self, amount: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStandardDeviation2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(amount)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetColor(&self, color: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetColor)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(color)).ok()
    }
    pub unsafe fn SetRed<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRed)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetRed2(&self, amount: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRed2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(amount)).ok()
    }
    pub unsafe fn SetGreen<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetGreen)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetGreen2(&self, amount: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetGreen2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(amount)).ok()
    }
    pub unsafe fn SetBlue<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBlue)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetBlue2(&self, amount: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBlue2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(amount)).ok()
    }
    pub unsafe fn SetAlpha<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAlpha)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetAlpha2(&self, amount: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAlpha2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(amount)).ok()
    }
}
impl ::core::convert::From<IDCompositionShadowEffect> for ::windows_core::IUnknown {
    fn from(value: IDCompositionShadowEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionShadowEffect> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionShadowEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionShadowEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionShadowEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionShadowEffect> for IDCompositionEffect {
    fn from(value: IDCompositionShadowEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionShadowEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionShadowEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for IDCompositionShadowEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionShadowEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionShadowEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionShadowEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionShadowEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionShadowEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionFilterEffect> for IDCompositionShadowEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionFilterEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionFilterEffect> for &'a IDCompositionShadowEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionFilterEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionShadowEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionShadowEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionShadowEffect {}
impl ::core::fmt::Debug for IDCompositionShadowEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionShadowEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionShadowEffect {
    type Vtable = IDCompositionShadowEffect_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4ad18ac0_cfd2_4c2f_bb62_96e54fdb6879);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionShadowEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    pub SetStandardDeviation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetStandardDeviation2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetColor: usize,
    pub SetRed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetRed2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows_core::HRESULT,
    pub SetGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetGreen2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows_core::HRESULT,
    pub SetBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetBlue2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows_core::HRESULT,
    pub SetAlpha: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetAlpha2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDCompositionSkewTransform(::windows_core::IUnknown);
impl IDCompositionSkewTransform {
    pub unsafe fn SetAngleX<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAngleX)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetAngleX2(&self, anglex: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAngleX2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(anglex)).ok()
    }
    pub unsafe fn SetAngleY<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAngleY)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetAngleY2(&self, angley: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAngleY2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(angley)).ok()
    }
    pub unsafe fn SetCenterX<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCenterX)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCenterX2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(centerx)).ok()
    }
    pub unsafe fn SetCenterY<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCenterY)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetCenterY2(&self, centery: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCenterY2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(centery)).ok()
    }
}
impl ::core::convert::From<IDCompositionSkewTransform> for ::windows_core::IUnknown {
    fn from(value: IDCompositionSkewTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionSkewTransform> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionSkewTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionSkewTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionSkewTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionSkewTransform> for IDCompositionEffect {
    fn from(value: IDCompositionSkewTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionSkewTransform> for IDCompositionEffect {
    fn from(value: &IDCompositionSkewTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for IDCompositionSkewTransform {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionSkewTransform {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionSkewTransform> for IDCompositionTransform3D {
    fn from(value: IDCompositionSkewTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionSkewTransform> for IDCompositionTransform3D {
    fn from(value: &IDCompositionSkewTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionTransform3D> for IDCompositionSkewTransform {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionTransform3D> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionTransform3D> for &'a IDCompositionSkewTransform {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionTransform3D> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionSkewTransform> for IDCompositionTransform {
    fn from(value: IDCompositionSkewTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionSkewTransform> for IDCompositionTransform {
    fn from(value: &IDCompositionSkewTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionTransform> for IDCompositionSkewTransform {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionTransform> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionTransform> for &'a IDCompositionSkewTransform {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionTransform> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionSkewTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionSkewTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionSkewTransform {}
impl ::core::fmt::Debug for IDCompositionSkewTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionSkewTransform").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionSkewTransform {
    type Vtable = IDCompositionSkewTransform_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe57aa735_dcdb_4c72_9c61_0591f58889ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionSkewTransform_Vtbl {
    pub base__: IDCompositionTransform_Vtbl,
    pub SetAngleX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetAngleX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, anglex: f32) -> ::windows_core::HRESULT,
    pub SetAngleY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetAngleY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, angley: f32) -> ::windows_core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetCenterX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows_core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetCenterY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDCompositionSurface(::windows_core::IUnknown);
impl IDCompositionSurface {
    pub unsafe fn BeginDraw(&self, updaterect: *const ::win32_foundation::RECT, iid: *const ::windows_core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut ::win32_foundation::POINT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginDraw)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(updaterect), ::core::mem::transmute(iid), ::core::mem::transmute(updateobject), ::core::mem::transmute(updateoffset)).ok()
    }
    pub unsafe fn EndDraw(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndDraw)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SuspendDraw(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SuspendDraw)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ResumeDraw(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ResumeDraw)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Scroll(&self, scrollrect: *const ::win32_foundation::RECT, cliprect: *const ::win32_foundation::RECT, offsetx: i32, offsety: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Scroll)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(scrollrect), ::core::mem::transmute(cliprect), ::core::mem::transmute(offsetx), ::core::mem::transmute(offsety)).ok()
    }
}
impl ::core::convert::From<IDCompositionSurface> for ::windows_core::IUnknown {
    fn from(value: IDCompositionSurface) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionSurface> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionSurface) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionSurface {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionSurface {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionSurface {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionSurface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionSurface {}
impl ::core::fmt::Debug for IDCompositionSurface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionSurface").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionSurface {
    type Vtable = IDCompositionSurface_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbb8a4953_2c99_4f5a_96f5_4819027fa3ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionSurface_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub BeginDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updaterect: *const ::win32_foundation::RECT, iid: *const ::windows_core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut ::win32_foundation::POINT) -> ::windows_core::HRESULT,
    pub EndDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SuspendDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ResumeDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Scroll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scrollrect: *const ::win32_foundation::RECT, cliprect: *const ::win32_foundation::RECT, offsetx: i32, offsety: i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDCompositionSurfaceFactory(::windows_core::IUnknown);
impl IDCompositionSurfaceFactory {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows_core::Result<IDCompositionSurface> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateSurface)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(pixelformat), ::core::mem::transmute(alphamode), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionSurface>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows_core::Result<IDCompositionVirtualSurface> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateVirtualSurface)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(initialwidth), ::core::mem::transmute(initialheight), ::core::mem::transmute(pixelformat), ::core::mem::transmute(alphamode), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDCompositionVirtualSurface>(result__)
    }
}
impl ::core::convert::From<IDCompositionSurfaceFactory> for ::windows_core::IUnknown {
    fn from(value: IDCompositionSurfaceFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionSurfaceFactory> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionSurfaceFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionSurfaceFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionSurfaceFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionSurfaceFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionSurfaceFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionSurfaceFactory {}
impl ::core::fmt::Debug for IDCompositionSurfaceFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionSurfaceFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionSurfaceFactory {
    type Vtable = IDCompositionSurfaceFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe334bc12_3937_4e02_85eb_fcf4eb30d2c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionSurfaceFactory_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateSurface: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateVirtualSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateVirtualSurface: usize,
}
#[repr(transparent)]
pub struct IDCompositionTableTransferEffect(::windows_core::IUnknown);
impl IDCompositionTableTransferEffect {
    pub unsafe fn SetInput<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    pub unsafe fn SetRedTable(&self, tablevalues: &[f32]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRedTable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(tablevalues)), tablevalues.len() as _).ok()
    }
    pub unsafe fn SetGreenTable(&self, tablevalues: &[f32]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetGreenTable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(tablevalues)), tablevalues.len() as _).ok()
    }
    pub unsafe fn SetBlueTable(&self, tablevalues: &[f32]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBlueTable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(tablevalues)), tablevalues.len() as _).ok()
    }
    pub unsafe fn SetAlphaTable(&self, tablevalues: &[f32]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAlphaTable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(tablevalues)), tablevalues.len() as _).ok()
    }
    pub unsafe fn SetRedDisable<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, reddisable: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRedDisable)(::windows_core::Interface::as_raw(self), reddisable.into_param().abi()).ok()
    }
    pub unsafe fn SetGreenDisable<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, greendisable: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetGreenDisable)(::windows_core::Interface::as_raw(self), greendisable.into_param().abi()).ok()
    }
    pub unsafe fn SetBlueDisable<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, bluedisable: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBlueDisable)(::windows_core::Interface::as_raw(self), bluedisable.into_param().abi()).ok()
    }
    pub unsafe fn SetAlphaDisable<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, alphadisable: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAlphaDisable)(::windows_core::Interface::as_raw(self), alphadisable.into_param().abi()).ok()
    }
    pub unsafe fn SetClampOutput<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, clampoutput: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetClampOutput)(::windows_core::Interface::as_raw(self), clampoutput.into_param().abi()).ok()
    }
    pub unsafe fn SetRedTableValue<'a, Param1: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, index: u32, animation: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRedTableValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetRedTableValue2(&self, index: u32, value: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRedTableValue2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(value)).ok()
    }
    pub unsafe fn SetGreenTableValue<'a, Param1: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, index: u32, animation: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetGreenTableValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetGreenTableValue2(&self, index: u32, value: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetGreenTableValue2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(value)).ok()
    }
    pub unsafe fn SetBlueTableValue<'a, Param1: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, index: u32, animation: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBlueTableValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetBlueTableValue2(&self, index: u32, value: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBlueTableValue2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(value)).ok()
    }
    pub unsafe fn SetAlphaTableValue<'a, Param1: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, index: u32, animation: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAlphaTableValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetAlphaTableValue2(&self, index: u32, value: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAlphaTableValue2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(value)).ok()
    }
}
impl ::core::convert::From<IDCompositionTableTransferEffect> for ::windows_core::IUnknown {
    fn from(value: IDCompositionTableTransferEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionTableTransferEffect> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionTableTransferEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionTableTransferEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionTableTransferEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionTableTransferEffect> for IDCompositionEffect {
    fn from(value: IDCompositionTableTransferEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionTableTransferEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionTableTransferEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for IDCompositionTableTransferEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionTableTransferEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionTableTransferEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionTableTransferEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionTableTransferEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionTableTransferEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionFilterEffect> for IDCompositionTableTransferEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionFilterEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionFilterEffect> for &'a IDCompositionTableTransferEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionFilterEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionTableTransferEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionTableTransferEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionTableTransferEffect {}
impl ::core::fmt::Debug for IDCompositionTableTransferEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionTableTransferEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionTableTransferEffect {
    type Vtable = IDCompositionTableTransferEffect_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9b7e82e2_69c5_4eb4_a5f5_a7033f5132cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTableTransferEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    pub SetRedTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows_core::HRESULT,
    pub SetGreenTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows_core::HRESULT,
    pub SetBlueTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows_core::HRESULT,
    pub SetAlphaTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows_core::HRESULT,
    pub SetRedDisable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reddisable: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetGreenDisable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, greendisable: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetBlueDisable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bluedisable: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetAlphaDisable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alphadisable: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetClampOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clampoutput: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetRedTableValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetRedTableValue2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows_core::HRESULT,
    pub SetGreenTableValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetGreenTableValue2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows_core::HRESULT,
    pub SetBlueTableValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetBlueTableValue2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows_core::HRESULT,
    pub SetAlphaTableValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetAlphaTableValue2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDCompositionTarget(::windows_core::IUnknown);
impl IDCompositionTarget {
    pub unsafe fn SetRoot<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRoot)(::windows_core::Interface::as_raw(self), visual.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IDCompositionTarget> for ::windows_core::IUnknown {
    fn from(value: IDCompositionTarget) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionTarget> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionTarget) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionTarget {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionTarget {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionTarget {}
impl ::core::fmt::Debug for IDCompositionTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionTarget").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionTarget {
    type Vtable = IDCompositionTarget_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeacdd04c_117e_4e17_88f4_d1b12b0e3d89);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTarget_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetRoot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDCompositionTransform(::windows_core::IUnknown);
impl IDCompositionTransform {}
impl ::core::convert::From<IDCompositionTransform> for ::windows_core::IUnknown {
    fn from(value: IDCompositionTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionTransform> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionTransform> for IDCompositionEffect {
    fn from(value: IDCompositionTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionTransform> for IDCompositionEffect {
    fn from(value: &IDCompositionTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for IDCompositionTransform {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionTransform {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionTransform> for IDCompositionTransform3D {
    fn from(value: IDCompositionTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionTransform> for IDCompositionTransform3D {
    fn from(value: &IDCompositionTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionTransform3D> for IDCompositionTransform {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionTransform3D> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionTransform3D> for &'a IDCompositionTransform {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionTransform3D> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionTransform {}
impl ::core::fmt::Debug for IDCompositionTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionTransform").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionTransform {
    type Vtable = IDCompositionTransform_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfd55faa7_37e0_4c20_95d2_9be45bc33f55);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTransform_Vtbl {
    pub base__: IDCompositionTransform3D_Vtbl,
}
#[repr(transparent)]
pub struct IDCompositionTransform3D(::windows_core::IUnknown);
impl IDCompositionTransform3D {}
impl ::core::convert::From<IDCompositionTransform3D> for ::windows_core::IUnknown {
    fn from(value: IDCompositionTransform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionTransform3D> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionTransform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionTransform3D> for IDCompositionEffect {
    fn from(value: IDCompositionTransform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionTransform3D> for IDCompositionEffect {
    fn from(value: &IDCompositionTransform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for IDCompositionTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionTransform3D {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionTransform3D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionTransform3D {}
impl ::core::fmt::Debug for IDCompositionTransform3D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionTransform3D").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionTransform3D {
    type Vtable = IDCompositionTransform3D_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x71185722_246b_41f2_aad1_0443f7f4bfc2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTransform3D_Vtbl {
    pub base__: IDCompositionEffect_Vtbl,
}
#[repr(transparent)]
pub struct IDCompositionTranslateTransform(::windows_core::IUnknown);
impl IDCompositionTranslateTransform {
    pub unsafe fn SetOffsetX<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOffsetX)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOffsetX2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(offsetx)).ok()
    }
    pub unsafe fn SetOffsetY<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOffsetY)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOffsetY2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(offsety)).ok()
    }
}
impl ::core::convert::From<IDCompositionTranslateTransform> for ::windows_core::IUnknown {
    fn from(value: IDCompositionTranslateTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionTranslateTransform> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionTranslateTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionTranslateTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionTranslateTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionTranslateTransform> for IDCompositionEffect {
    fn from(value: IDCompositionTranslateTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionTranslateTransform> for IDCompositionEffect {
    fn from(value: &IDCompositionTranslateTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for IDCompositionTranslateTransform {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionTranslateTransform {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionTranslateTransform> for IDCompositionTransform3D {
    fn from(value: IDCompositionTranslateTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionTranslateTransform> for IDCompositionTransform3D {
    fn from(value: &IDCompositionTranslateTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionTransform3D> for IDCompositionTranslateTransform {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionTransform3D> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionTransform3D> for &'a IDCompositionTranslateTransform {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionTransform3D> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionTranslateTransform> for IDCompositionTransform {
    fn from(value: IDCompositionTranslateTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionTranslateTransform> for IDCompositionTransform {
    fn from(value: &IDCompositionTranslateTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionTransform> for IDCompositionTranslateTransform {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionTransform> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionTransform> for &'a IDCompositionTranslateTransform {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionTransform> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionTranslateTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionTranslateTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionTranslateTransform {}
impl ::core::fmt::Debug for IDCompositionTranslateTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionTranslateTransform").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionTranslateTransform {
    type Vtable = IDCompositionTranslateTransform_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x06791122_c6f0_417d_8323_269e987f5954);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTranslateTransform_Vtbl {
    pub base__: IDCompositionTransform_Vtbl,
    pub SetOffsetX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetOffsetX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsetx: f32) -> ::windows_core::HRESULT,
    pub SetOffsetY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetOffsetY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsety: f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDCompositionTranslateTransform3D(::windows_core::IUnknown);
impl IDCompositionTranslateTransform3D {
    pub unsafe fn SetOffsetX<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOffsetX)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOffsetX2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(offsetx)).ok()
    }
    pub unsafe fn SetOffsetY<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOffsetY)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOffsetY2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(offsety)).ok()
    }
    pub unsafe fn SetOffsetZ<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOffsetZ)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetOffsetZ2(&self, offsetz: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOffsetZ2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(offsetz)).ok()
    }
}
impl ::core::convert::From<IDCompositionTranslateTransform3D> for ::windows_core::IUnknown {
    fn from(value: IDCompositionTranslateTransform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionTranslateTransform3D> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionTranslateTransform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionTranslateTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionTranslateTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionTranslateTransform3D> for IDCompositionEffect {
    fn from(value: IDCompositionTranslateTransform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionTranslateTransform3D> for IDCompositionEffect {
    fn from(value: &IDCompositionTranslateTransform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for IDCompositionTranslateTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionTranslateTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionTranslateTransform3D> for IDCompositionTransform3D {
    fn from(value: IDCompositionTranslateTransform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionTranslateTransform3D> for IDCompositionTransform3D {
    fn from(value: &IDCompositionTranslateTransform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionTransform3D> for IDCompositionTranslateTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionTransform3D> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionTransform3D> for &'a IDCompositionTranslateTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionTransform3D> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionTranslateTransform3D {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionTranslateTransform3D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionTranslateTransform3D {}
impl ::core::fmt::Debug for IDCompositionTranslateTransform3D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionTranslateTransform3D").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionTranslateTransform3D {
    type Vtable = IDCompositionTranslateTransform3D_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x91636d4b_9ba1_4532_aaf7_e3344994d788);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTranslateTransform3D_Vtbl {
    pub base__: IDCompositionTransform3D_Vtbl,
    pub SetOffsetX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetOffsetX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsetx: f32) -> ::windows_core::HRESULT,
    pub SetOffsetY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetOffsetY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsety: f32) -> ::windows_core::HRESULT,
    pub SetOffsetZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetOffsetZ2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsetz: f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDCompositionTurbulenceEffect(::windows_core::IUnknown);
impl IDCompositionTurbulenceEffect {
    pub unsafe fn SetInput<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetOffset(&self, offset: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOffset)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(offset)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetBaseFrequency(&self, frequency: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBaseFrequency)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(frequency)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetSize(&self, size: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(size)).ok()
    }
    pub unsafe fn SetNumOctaves(&self, numoctaves: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetNumOctaves)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(numoctaves)).ok()
    }
    pub unsafe fn SetSeed(&self, seed: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSeed)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(seed)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetNoise(&self, noise: super::Direct2D::Common::D2D1_TURBULENCE_NOISE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetNoise)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(noise)).ok()
    }
    pub unsafe fn SetStitchable<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, stitchable: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStitchable)(::windows_core::Interface::as_raw(self), stitchable.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IDCompositionTurbulenceEffect> for ::windows_core::IUnknown {
    fn from(value: IDCompositionTurbulenceEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionTurbulenceEffect> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionTurbulenceEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionTurbulenceEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionTurbulenceEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionTurbulenceEffect> for IDCompositionEffect {
    fn from(value: IDCompositionTurbulenceEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionTurbulenceEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionTurbulenceEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for IDCompositionTurbulenceEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionTurbulenceEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionTurbulenceEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionTurbulenceEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionTurbulenceEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionTurbulenceEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionFilterEffect> for IDCompositionTurbulenceEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionFilterEffect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionFilterEffect> for &'a IDCompositionTurbulenceEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionFilterEffect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionTurbulenceEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionTurbulenceEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionTurbulenceEffect {}
impl ::core::fmt::Debug for IDCompositionTurbulenceEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionTurbulenceEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionTurbulenceEffect {
    type Vtable = IDCompositionTurbulenceEffect_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa6a55bda_c09c_49f3_9193_a41922c89715);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTurbulenceEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetOffset: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetBaseFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frequency: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetBaseFrequency: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetSize: usize,
    pub SetNumOctaves: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numoctaves: u32) -> ::windows_core::HRESULT,
    pub SetSeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, seed: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetNoise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, noise: super::Direct2D::Common::D2D1_TURBULENCE_NOISE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetNoise: usize,
    pub SetStitchable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stitchable: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDCompositionVirtualSurface(::windows_core::IUnknown);
impl IDCompositionVirtualSurface {
    pub unsafe fn BeginDraw(&self, updaterect: *const ::win32_foundation::RECT, iid: *const ::windows_core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut ::win32_foundation::POINT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.BeginDraw)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(updaterect), ::core::mem::transmute(iid), ::core::mem::transmute(updateobject), ::core::mem::transmute(updateoffset)).ok()
    }
    pub unsafe fn EndDraw(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EndDraw)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SuspendDraw(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SuspendDraw)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ResumeDraw(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ResumeDraw)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Scroll(&self, scrollrect: *const ::win32_foundation::RECT, cliprect: *const ::win32_foundation::RECT, offsetx: i32, offsety: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Scroll)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(scrollrect), ::core::mem::transmute(cliprect), ::core::mem::transmute(offsetx), ::core::mem::transmute(offsety)).ok()
    }
    pub unsafe fn Resize(&self, width: u32, height: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Resize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(width), ::core::mem::transmute(height)).ok()
    }
    pub unsafe fn Trim(&self, rectangles: &[::win32_foundation::RECT]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Trim)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(rectangles)), rectangles.len() as _).ok()
    }
}
impl ::core::convert::From<IDCompositionVirtualSurface> for ::windows_core::IUnknown {
    fn from(value: IDCompositionVirtualSurface) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionVirtualSurface> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionVirtualSurface) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionVirtualSurface {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionVirtualSurface {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionVirtualSurface> for IDCompositionSurface {
    fn from(value: IDCompositionVirtualSurface) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionVirtualSurface> for IDCompositionSurface {
    fn from(value: &IDCompositionVirtualSurface) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionSurface> for IDCompositionVirtualSurface {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionSurface> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionSurface> for &'a IDCompositionVirtualSurface {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionSurface> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionVirtualSurface {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionVirtualSurface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionVirtualSurface {}
impl ::core::fmt::Debug for IDCompositionVirtualSurface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionVirtualSurface").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionVirtualSurface {
    type Vtable = IDCompositionVirtualSurface_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xae471c51_5f53_4a24_8d3e_d0c39c30b3f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionVirtualSurface_Vtbl {
    pub base__: IDCompositionSurface_Vtbl,
    pub Resize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows_core::HRESULT,
    pub Trim: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rectangles: *const ::win32_foundation::RECT, count: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDCompositionVisual(::windows_core::IUnknown);
impl IDCompositionVisual {
    pub unsafe fn SetOffsetX<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOffsetX)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOffsetX2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(offsetx)).ok()
    }
    pub unsafe fn SetOffsetY<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOffsetY)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOffsetY2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(offsety)).ok()
    }
    pub unsafe fn SetTransform<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionTransform>>(&self, transform: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTransform)(::windows_core::Interface::as_raw(self), transform.into_param().abi()).ok()
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform2(&self, matrix: *const ::winrt_foundation::Numerics::Matrix3x2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTransform2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(matrix)).ok()
    }
    pub unsafe fn SetTransformParent<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTransformParent)(::windows_core::Interface::as_raw(self), visual.into_param().abi()).ok()
    }
    pub unsafe fn SetEffect<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionEffect>>(&self, effect: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEffect)(::windows_core::Interface::as_raw(self), effect.into_param().abi()).ok()
    }
    pub unsafe fn SetBitmapInterpolationMode(&self, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBitmapInterpolationMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(interpolationmode)).ok()
    }
    pub unsafe fn SetBorderMode(&self, bordermode: DCOMPOSITION_BORDER_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBorderMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(bordermode)).ok()
    }
    pub unsafe fn SetClip<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionClip>>(&self, clip: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetClip)(::windows_core::Interface::as_raw(self), clip.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetClip2(&self, rect: *const super::Direct2D::Common::D2D_RECT_F) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetClip2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(rect)).ok()
    }
    pub unsafe fn SetContent<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, content: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetContent)(::windows_core::Interface::as_raw(self), content.into_param().abi()).ok()
    }
    pub unsafe fn AddVisual<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionVisual>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>, Param2: ::windows_core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0, insertabove: Param1, referencevisual: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddVisual)(::windows_core::Interface::as_raw(self), visual.into_param().abi(), insertabove.into_param().abi(), referencevisual.into_param().abi()).ok()
    }
    pub unsafe fn RemoveVisual<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveVisual)(::windows_core::Interface::as_raw(self), visual.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAllVisuals(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAllVisuals)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetCompositeMode(&self, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCompositeMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(compositemode)).ok()
    }
}
impl ::core::convert::From<IDCompositionVisual> for ::windows_core::IUnknown {
    fn from(value: IDCompositionVisual) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionVisual> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionVisual) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionVisual {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionVisual {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionVisual {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionVisual {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionVisual {}
impl ::core::fmt::Debug for IDCompositionVisual {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionVisual").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionVisual {
    type Vtable = IDCompositionVisual_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4d93059d_097b_4651_9a60_f0f25116e2f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionVisual_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetOffsetX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetOffsetX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsetx: f32) -> ::windows_core::HRESULT,
    pub SetOffsetY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetOffsetY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsety: f32) -> ::windows_core::HRESULT,
    pub SetTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTransform2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const ::winrt_foundation::Numerics::Matrix3x2) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTransform2: usize,
    pub SetTransformParent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effect: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetBitmapInterpolationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> ::windows_core::HRESULT,
    pub SetBorderMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bordermode: DCOMPOSITION_BORDER_MODE) -> ::windows_core::HRESULT,
    pub SetClip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clip: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetClip2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rect: *const super::Direct2D::Common::D2D_RECT_F) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetClip2: usize,
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AddVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: ::windows_core::RawPtr, insertabove: ::win32_foundation::BOOL, referencevisual: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveAllVisuals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetCompositeMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDCompositionVisual2(::windows_core::IUnknown);
impl IDCompositionVisual2 {
    pub unsafe fn SetOffsetX<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetOffsetX)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetOffsetX2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(offsetx)).ok()
    }
    pub unsafe fn SetOffsetY<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetOffsetY)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetOffsetY2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(offsety)).ok()
    }
    pub unsafe fn SetTransform<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionTransform>>(&self, transform: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetTransform)(::windows_core::Interface::as_raw(self), transform.into_param().abi()).ok()
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform2(&self, matrix: *const ::winrt_foundation::Numerics::Matrix3x2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetTransform2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(matrix)).ok()
    }
    pub unsafe fn SetTransformParent<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetTransformParent)(::windows_core::Interface::as_raw(self), visual.into_param().abi()).ok()
    }
    pub unsafe fn SetEffect<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionEffect>>(&self, effect: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEffect)(::windows_core::Interface::as_raw(self), effect.into_param().abi()).ok()
    }
    pub unsafe fn SetBitmapInterpolationMode(&self, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetBitmapInterpolationMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(interpolationmode)).ok()
    }
    pub unsafe fn SetBorderMode(&self, bordermode: DCOMPOSITION_BORDER_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetBorderMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(bordermode)).ok()
    }
    pub unsafe fn SetClip<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionClip>>(&self, clip: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetClip)(::windows_core::Interface::as_raw(self), clip.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetClip2(&self, rect: *const super::Direct2D::Common::D2D_RECT_F) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetClip2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(rect)).ok()
    }
    pub unsafe fn SetContent<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, content: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetContent)(::windows_core::Interface::as_raw(self), content.into_param().abi()).ok()
    }
    pub unsafe fn AddVisual<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionVisual>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>, Param2: ::windows_core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0, insertabove: Param1, referencevisual: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddVisual)(::windows_core::Interface::as_raw(self), visual.into_param().abi(), insertabove.into_param().abi(), referencevisual.into_param().abi()).ok()
    }
    pub unsafe fn RemoveVisual<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RemoveVisual)(::windows_core::Interface::as_raw(self), visual.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAllVisuals(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RemoveAllVisuals)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetCompositeMode(&self, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetCompositeMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(compositemode)).ok()
    }
    pub unsafe fn SetOpacityMode(&self, mode: DCOMPOSITION_OPACITY_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOpacityMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(mode)).ok()
    }
    pub unsafe fn SetBackFaceVisibility(&self, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBackFaceVisibility)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(visibility)).ok()
    }
}
impl ::core::convert::From<IDCompositionVisual2> for ::windows_core::IUnknown {
    fn from(value: IDCompositionVisual2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionVisual2> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionVisual2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionVisual2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionVisual2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionVisual2> for IDCompositionVisual {
    fn from(value: IDCompositionVisual2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionVisual2> for IDCompositionVisual {
    fn from(value: &IDCompositionVisual2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionVisual> for IDCompositionVisual2 {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionVisual> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionVisual> for &'a IDCompositionVisual2 {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionVisual> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionVisual2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionVisual2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionVisual2 {}
impl ::core::fmt::Debug for IDCompositionVisual2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionVisual2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionVisual2 {
    type Vtable = IDCompositionVisual2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe8de1639_4331_4b26_bc5f_6a321d347a85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionVisual2_Vtbl {
    pub base__: IDCompositionVisual_Vtbl,
    pub SetOpacityMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: DCOMPOSITION_OPACITY_MODE) -> ::windows_core::HRESULT,
    pub SetBackFaceVisibility: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDCompositionVisual3(::windows_core::IUnknown);
impl IDCompositionVisual3 {
    pub unsafe fn SetOffsetX<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetOffsetX)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetOffsetX2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(offsetx)).ok()
    }
    pub unsafe fn SetOffsetY<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetOffsetY)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetOffsetY2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(offsety)).ok()
    }
    pub unsafe fn SetTransform<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionTransform>>(&self, transform: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetTransform)(::windows_core::Interface::as_raw(self), transform.into_param().abi()).ok()
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform2(&self, matrix: *const ::winrt_foundation::Numerics::Matrix3x2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetTransform2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(matrix)).ok()
    }
    pub unsafe fn SetTransformParent<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetTransformParent)(::windows_core::Interface::as_raw(self), visual.into_param().abi()).ok()
    }
    pub unsafe fn SetEffect<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionEffect>>(&self, effect: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetEffect)(::windows_core::Interface::as_raw(self), effect.into_param().abi()).ok()
    }
    pub unsafe fn SetBitmapInterpolationMode(&self, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetBitmapInterpolationMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(interpolationmode)).ok()
    }
    pub unsafe fn SetBorderMode(&self, bordermode: DCOMPOSITION_BORDER_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetBorderMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(bordermode)).ok()
    }
    pub unsafe fn SetClip<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionClip>>(&self, clip: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetClip)(::windows_core::Interface::as_raw(self), clip.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetClip2(&self, rect: *const super::Direct2D::Common::D2D_RECT_F) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetClip2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(rect)).ok()
    }
    pub unsafe fn SetContent<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, content: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetContent)(::windows_core::Interface::as_raw(self), content.into_param().abi()).ok()
    }
    pub unsafe fn AddVisual<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionVisual>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>, Param2: ::windows_core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0, insertabove: Param1, referencevisual: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.AddVisual)(::windows_core::Interface::as_raw(self), visual.into_param().abi(), insertabove.into_param().abi(), referencevisual.into_param().abi()).ok()
    }
    pub unsafe fn RemoveVisual<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.RemoveVisual)(::windows_core::Interface::as_raw(self), visual.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAllVisuals(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.RemoveAllVisuals)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetCompositeMode(&self, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetCompositeMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(compositemode)).ok()
    }
    pub unsafe fn SetOpacityMode(&self, mode: DCOMPOSITION_OPACITY_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetOpacityMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(mode)).ok()
    }
    pub unsafe fn SetBackFaceVisibility(&self, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetBackFaceVisibility)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(visibility)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn EnableHeatMap(&self, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EnableHeatMap)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(color)).ok()
    }
    pub unsafe fn DisableHeatMap(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DisableHeatMap)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnableRedrawRegions(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EnableRedrawRegions)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DisableRedrawRegions(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DisableRedrawRegions)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetDepthMode(&self, mode: DCOMPOSITION_DEPTH_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDepthMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(mode)).ok()
    }
    pub unsafe fn SetOffsetZ<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOffsetZ)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetOffsetZ2(&self, offsetz: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOffsetZ2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(offsetz)).ok()
    }
    pub unsafe fn SetOpacity<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOpacity)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetOpacity2(&self, opacity: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOpacity2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(opacity)).ok()
    }
    pub unsafe fn SetTransform3<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionTransform3D>>(&self, transform: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTransform3)(::windows_core::Interface::as_raw(self), transform.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetTransform4(&self, matrix: *const super::Direct2D::Common::D2D_MATRIX_4X4_F) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTransform4)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(matrix)).ok()
    }
    pub unsafe fn SetVisible<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, visible: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetVisible)(::windows_core::Interface::as_raw(self), visible.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IDCompositionVisual3> for ::windows_core::IUnknown {
    fn from(value: IDCompositionVisual3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionVisual3> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionVisual3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionVisual3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionVisual3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionVisual3> for IDCompositionVisual {
    fn from(value: IDCompositionVisual3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionVisual3> for IDCompositionVisual {
    fn from(value: &IDCompositionVisual3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionVisual> for IDCompositionVisual3 {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionVisual> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionVisual> for &'a IDCompositionVisual3 {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionVisual> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionVisual3> for IDCompositionVisual2 {
    fn from(value: IDCompositionVisual3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionVisual3> for IDCompositionVisual2 {
    fn from(value: &IDCompositionVisual3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionVisual2> for IDCompositionVisual3 {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionVisual2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionVisual2> for &'a IDCompositionVisual3 {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionVisual2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionVisual3> for IDCompositionVisualDebug {
    fn from(value: IDCompositionVisual3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionVisual3> for IDCompositionVisualDebug {
    fn from(value: &IDCompositionVisual3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionVisualDebug> for IDCompositionVisual3 {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionVisualDebug> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionVisualDebug> for &'a IDCompositionVisual3 {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionVisualDebug> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionVisual3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionVisual3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionVisual3 {}
impl ::core::fmt::Debug for IDCompositionVisual3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionVisual3").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionVisual3 {
    type Vtable = IDCompositionVisual3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2775f462_b6c1_4015_b0be_b3e7d6a4976d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionVisual3_Vtbl {
    pub base__: IDCompositionVisualDebug_Vtbl,
    pub SetDepthMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: DCOMPOSITION_DEPTH_MODE) -> ::windows_core::HRESULT,
    pub SetOffsetZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetOffsetZ2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsetz: f32) -> ::windows_core::HRESULT,
    pub SetOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetOpacity2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows_core::HRESULT,
    pub SetTransform3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetTransform4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const super::Direct2D::Common::D2D_MATRIX_4X4_F) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetTransform4: usize,
    pub SetVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visible: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDCompositionVisualDebug(::windows_core::IUnknown);
impl IDCompositionVisualDebug {
    pub unsafe fn SetOffsetX<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetOffsetX)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetOffsetX2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(offsetx)).ok()
    }
    pub unsafe fn SetOffsetY<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetOffsetY)(::windows_core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetOffsetY2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(offsety)).ok()
    }
    pub unsafe fn SetTransform<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionTransform>>(&self, transform: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetTransform)(::windows_core::Interface::as_raw(self), transform.into_param().abi()).ok()
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform2(&self, matrix: *const ::winrt_foundation::Numerics::Matrix3x2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetTransform2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(matrix)).ok()
    }
    pub unsafe fn SetTransformParent<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetTransformParent)(::windows_core::Interface::as_raw(self), visual.into_param().abi()).ok()
    }
    pub unsafe fn SetEffect<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionEffect>>(&self, effect: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetEffect)(::windows_core::Interface::as_raw(self), effect.into_param().abi()).ok()
    }
    pub unsafe fn SetBitmapInterpolationMode(&self, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetBitmapInterpolationMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(interpolationmode)).ok()
    }
    pub unsafe fn SetBorderMode(&self, bordermode: DCOMPOSITION_BORDER_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetBorderMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(bordermode)).ok()
    }
    pub unsafe fn SetClip<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionClip>>(&self, clip: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetClip)(::windows_core::Interface::as_raw(self), clip.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetClip2(&self, rect: *const super::Direct2D::Common::D2D_RECT_F) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetClip2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(rect)).ok()
    }
    pub unsafe fn SetContent<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, content: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetContent)(::windows_core::Interface::as_raw(self), content.into_param().abi()).ok()
    }
    pub unsafe fn AddVisual<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionVisual>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>, Param2: ::windows_core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0, insertabove: Param1, referencevisual: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.AddVisual)(::windows_core::Interface::as_raw(self), visual.into_param().abi(), insertabove.into_param().abi(), referencevisual.into_param().abi()).ok()
    }
    pub unsafe fn RemoveVisual<'a, Param0: ::windows_core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.RemoveVisual)(::windows_core::Interface::as_raw(self), visual.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAllVisuals(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.RemoveAllVisuals)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetCompositeMode(&self, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetCompositeMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(compositemode)).ok()
    }
    pub unsafe fn SetOpacityMode(&self, mode: DCOMPOSITION_OPACITY_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetOpacityMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(mode)).ok()
    }
    pub unsafe fn SetBackFaceVisibility(&self, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetBackFaceVisibility)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(visibility)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn EnableHeatMap(&self, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnableHeatMap)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(color)).ok()
    }
    pub unsafe fn DisableHeatMap(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DisableHeatMap)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnableRedrawRegions(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnableRedrawRegions)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DisableRedrawRegions(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DisableRedrawRegions)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IDCompositionVisualDebug> for ::windows_core::IUnknown {
    fn from(value: IDCompositionVisualDebug) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionVisualDebug> for ::windows_core::IUnknown {
    fn from(value: &IDCompositionVisualDebug) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDCompositionVisualDebug {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDCompositionVisualDebug {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionVisualDebug> for IDCompositionVisual {
    fn from(value: IDCompositionVisualDebug) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionVisualDebug> for IDCompositionVisual {
    fn from(value: &IDCompositionVisualDebug) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionVisual> for IDCompositionVisualDebug {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionVisual> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionVisual> for &'a IDCompositionVisualDebug {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionVisual> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionVisualDebug> for IDCompositionVisual2 {
    fn from(value: IDCompositionVisualDebug) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionVisualDebug> for IDCompositionVisual2 {
    fn from(value: &IDCompositionVisualDebug) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionVisual2> for IDCompositionVisualDebug {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionVisual2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDCompositionVisual2> for &'a IDCompositionVisualDebug {
    fn into_param(self) -> ::windows_core::Param<'a, IDCompositionVisual2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionVisualDebug {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionVisualDebug {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionVisualDebug {}
impl ::core::fmt::Debug for IDCompositionVisualDebug {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionVisualDebug").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDCompositionVisualDebug {
    type Vtable = IDCompositionVisualDebug_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfed2b808_5eb4_43a0_aea3_35f65280f91b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionVisualDebug_Vtbl {
    pub base__: IDCompositionVisual2_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub EnableHeatMap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    EnableHeatMap: usize,
    pub DisableHeatMap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EnableRedrawRegions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DisableRedrawRegions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
