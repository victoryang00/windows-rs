#[repr(C)]
pub struct IUIAnimationInterpolator {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetInitialValueAndVelocity: unsafe extern "system" fn(this: *mut *mut Self, initialvalue: f64, initialvelocity: f64) -> ::windows_sys::core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(this: *mut *mut Self, duration: f64) -> ::windows_sys::core::HRESULT,
    pub GetDuration: unsafe extern "system" fn(this: *mut *mut Self, duration: *mut f64) -> ::windows_sys::core::HRESULT,
    pub GetFinalValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut f64) -> ::windows_sys::core::HRESULT,
    pub InterpolateValue: unsafe extern "system" fn(this: *mut *mut Self, offset: f64, value: *mut f64) -> ::windows_sys::core::HRESULT,
    pub InterpolateVelocity: unsafe extern "system" fn(this: *mut *mut Self, offset: f64, velocity: *mut f64) -> ::windows_sys::core::HRESULT,
    pub GetDependencies: unsafe extern "system" fn(this: *mut *mut Self, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUIAnimationInterpolator {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2014694330, data2: 56823, data3: 18316, data4: [164, 108, 123, 108, 115, 139, 121, 120] };
}
#[repr(C)]
pub struct IUIAnimationInterpolator2 {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetDimension: unsafe extern "system" fn(this: *mut *mut Self, dimension: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetInitialValueAndVelocity: unsafe extern "system" fn(this: *mut *mut Self, initialvalue: *const f64, initialvelocity: *const f64, cdimension: u32) -> ::windows_sys::core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(this: *mut *mut Self, duration: f64) -> ::windows_sys::core::HRESULT,
    pub GetDuration: unsafe extern "system" fn(this: *mut *mut Self, duration: *mut f64) -> ::windows_sys::core::HRESULT,
    pub GetFinalValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut f64, cdimension: u32) -> ::windows_sys::core::HRESULT,
    pub InterpolateValue: unsafe extern "system" fn(this: *mut *mut Self, offset: f64, value: *mut f64, cdimension: u32) -> ::windows_sys::core::HRESULT,
    pub InterpolateVelocity: unsafe extern "system" fn(this: *mut *mut Self, offset: f64, velocity: *mut f64, cdimension: u32) -> ::windows_sys::core::HRESULT,
    pub GetPrimitiveInterpolation: unsafe extern "system" fn(this: *mut *mut Self, interpolation: *mut ::core::ffi::c_void, cdimension: u32) -> ::windows_sys::core::HRESULT,
    pub GetDependencies: unsafe extern "system" fn(this: *mut *mut Self, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUIAnimationInterpolator2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3933646840, data2: 59938, data3: 18979, data4: [160, 239, 166, 169, 102, 112, 53, 24] };
}
#[repr(C)]
pub struct IUIAnimationLoopIterationChangeHandler2 {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnLoopIterationChanged: unsafe extern "system" fn(this: *mut *mut Self, storyboard: *mut ::core::ffi::c_void, id: usize, newiterationcount: u32, olditerationcount: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUIAnimationLoopIterationChangeHandler2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 758846884, data2: 18274, data3: 18347, data4: [160, 48, 178, 50, 33, 223, 58, 224] };
}
#[repr(C)]
pub struct IUIAnimationManager {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateAnimationVariable: unsafe extern "system" fn(this: *mut *mut Self, initialvalue: f64, variable: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ScheduleTransition: unsafe extern "system" fn(this: *mut *mut Self, variable: *mut ::core::ffi::c_void, transition: *mut ::core::ffi::c_void, timenow: f64) -> ::windows_sys::core::HRESULT,
    pub CreateStoryboard: unsafe extern "system" fn(this: *mut *mut Self, storyboard: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FinishAllStoryboards: unsafe extern "system" fn(this: *mut *mut Self, completiondeadline: f64) -> ::windows_sys::core::HRESULT,
    pub AbandonAllStoryboards: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Update: unsafe extern "system" fn(this: *mut *mut Self, timenow: f64, updateresult: *mut UI_ANIMATION_UPDATE_RESULT) -> ::windows_sys::core::HRESULT,
    pub GetVariableFromTag: unsafe extern "system" fn(this: *mut *mut Self, object: *mut ::core::ffi::c_void, id: u32, variable: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStoryboardFromTag: unsafe extern "system" fn(this: *mut *mut Self, object: *mut ::core::ffi::c_void, id: u32, storyboard: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, status: *mut UI_ANIMATION_MANAGER_STATUS) -> ::windows_sys::core::HRESULT,
    pub SetAnimationMode: unsafe extern "system" fn(this: *mut *mut Self, mode: UI_ANIMATION_MODE) -> ::windows_sys::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetManagerEventHandler: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCancelPriorityComparison: unsafe extern "system" fn(this: *mut *mut Self, comparison: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTrimPriorityComparison: unsafe extern "system" fn(this: *mut *mut Self, comparison: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCompressPriorityComparison: unsafe extern "system" fn(this: *mut *mut Self, comparison: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetConcludePriorityComparison: unsafe extern "system" fn(this: *mut *mut Self, comparison: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetDefaultLongestAcceptableDelay: unsafe extern "system" fn(this: *mut *mut Self, delay: f64) -> ::windows_sys::core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUIAnimationManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2439612780, data2: 44173, data3: 20093, data4: [148, 229, 103, 250, 77, 194, 242, 232] };
}
#[repr(C)]
pub struct IUIAnimationManager2 {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateAnimationVectorVariable: unsafe extern "system" fn(this: *mut *mut Self, initialvalue: *const f64, cdimension: u32, variable: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateAnimationVariable: unsafe extern "system" fn(this: *mut *mut Self, initialvalue: f64, variable: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ScheduleTransition: unsafe extern "system" fn(this: *mut *mut Self, variable: *mut ::core::ffi::c_void, transition: *mut ::core::ffi::c_void, timenow: f64) -> ::windows_sys::core::HRESULT,
    pub CreateStoryboard: unsafe extern "system" fn(this: *mut *mut Self, storyboard: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FinishAllStoryboards: unsafe extern "system" fn(this: *mut *mut Self, completiondeadline: f64) -> ::windows_sys::core::HRESULT,
    pub AbandonAllStoryboards: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Update: unsafe extern "system" fn(this: *mut *mut Self, timenow: f64, updateresult: *mut UI_ANIMATION_UPDATE_RESULT) -> ::windows_sys::core::HRESULT,
    pub GetVariableFromTag: unsafe extern "system" fn(this: *mut *mut Self, object: *mut ::core::ffi::c_void, id: u32, variable: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStoryboardFromTag: unsafe extern "system" fn(this: *mut *mut Self, object: *mut ::core::ffi::c_void, id: u32, storyboard: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EstimateNextEventTime: unsafe extern "system" fn(this: *mut *mut Self, seconds: *mut f64) -> ::windows_sys::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, status: *mut UI_ANIMATION_MANAGER_STATUS) -> ::windows_sys::core::HRESULT,
    pub SetAnimationMode: unsafe extern "system" fn(this: *mut *mut Self, mode: UI_ANIMATION_MODE) -> ::windows_sys::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetManagerEventHandler: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetManagerEventHandler: usize,
    pub SetCancelPriorityComparison: unsafe extern "system" fn(this: *mut *mut Self, comparison: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTrimPriorityComparison: unsafe extern "system" fn(this: *mut *mut Self, comparison: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCompressPriorityComparison: unsafe extern "system" fn(this: *mut *mut Self, comparison: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetConcludePriorityComparison: unsafe extern "system" fn(this: *mut *mut Self, comparison: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetDefaultLongestAcceptableDelay: unsafe extern "system" fn(this: *mut *mut Self, delay: f64) -> ::windows_sys::core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUIAnimationManager2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3635869652, data2: 16649, data3: 19775, data4: [172, 238, 135, 153, 38, 150, 140, 177] };
}
#[repr(C)]
pub struct IUIAnimationManagerEventHandler {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnManagerStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUIAnimationManagerEventHandler {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2016616941, data2: 30883, data3: 17254, data4: [181, 116, 106, 246, 7, 166, 71, 136] };
}
#[repr(C)]
pub struct IUIAnimationManagerEventHandler2 {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnManagerStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUIAnimationManagerEventHandler2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4141884090, data2: 49139, data3: 17132, data4: [144, 51, 224, 115, 243, 62, 131, 195] };
}
#[repr(C)]
pub struct IUIAnimationPrimitiveInterpolation {
    pub base__: ::windows_sys::core::IUnknown,
    pub AddCubic: unsafe extern "system" fn(this: *mut *mut Self, dimension: u32, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> ::windows_sys::core::HRESULT,
    pub AddSinusoidal: unsafe extern "system" fn(this: *mut *mut Self, dimension: u32, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUIAnimationPrimitiveInterpolation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3132231011, data2: 17249, data3: 17882, data4: [162, 79, 171, 133, 8, 132, 107, 91] };
}
#[repr(C)]
pub struct IUIAnimationPriorityComparison {
    pub base__: ::windows_sys::core::IUnknown,
    pub HasPriority: unsafe extern "system" fn(this: *mut *mut Self, scheduledstoryboard: *mut ::core::ffi::c_void, newstoryboard: *mut ::core::ffi::c_void, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUIAnimationPriorityComparison {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2214239092, data2: 24454, data3: 17944, data4: [188, 106, 162, 250, 193, 155, 63, 68] };
}
#[repr(C)]
pub struct IUIAnimationPriorityComparison2 {
    pub base__: ::windows_sys::core::IUnknown,
    pub HasPriority: unsafe extern "system" fn(this: *mut *mut Self, scheduledstoryboard: *mut ::core::ffi::c_void, newstoryboard: *mut ::core::ffi::c_void, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUIAnimationPriorityComparison2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1533901367, data2: 17953, data3: 18044, data4: [139, 5, 112, 19, 29, 230, 45, 219] };
}
#[repr(C)]
pub struct IUIAnimationStoryboard {
    pub base__: ::windows_sys::core::IUnknown,
    pub AddTransition: unsafe extern "system" fn(this: *mut *mut Self, variable: *mut ::core::ffi::c_void, transition: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AddKeyframeAtOffset: unsafe extern "system" fn(this: *mut *mut Self, existingkeyframe: UI_ANIMATION_KEYFRAME, offset: f64, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows_sys::core::HRESULT,
    pub AddKeyframeAfterTransition: unsafe extern "system" fn(this: *mut *mut Self, transition: *mut ::core::ffi::c_void, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows_sys::core::HRESULT,
    pub AddTransitionAtKeyframe: unsafe extern "system" fn(this: *mut *mut Self, variable: *mut ::core::ffi::c_void, transition: *mut ::core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows_sys::core::HRESULT,
    pub AddTransitionBetweenKeyframes: unsafe extern "system" fn(this: *mut *mut Self, variable: *mut ::core::ffi::c_void, transition: *mut ::core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows_sys::core::HRESULT,
    pub RepeatBetweenKeyframes: unsafe extern "system" fn(this: *mut *mut Self, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME, repetitioncount: i32) -> ::windows_sys::core::HRESULT,
    pub HoldVariable: unsafe extern "system" fn(this: *mut *mut Self, variable: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetLongestAcceptableDelay: unsafe extern "system" fn(this: *mut *mut Self, delay: f64) -> ::windows_sys::core::HRESULT,
    pub Schedule: unsafe extern "system" fn(this: *mut *mut Self, timenow: f64, schedulingresult: *mut UI_ANIMATION_SCHEDULING_RESULT) -> ::windows_sys::core::HRESULT,
    pub Conclude: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Finish: unsafe extern "system" fn(this: *mut *mut Self, completiondeadline: f64) -> ::windows_sys::core::HRESULT,
    pub Abandon: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut *mut Self, object: *mut ::core::ffi::c_void, id: u32) -> ::windows_sys::core::HRESULT,
    pub GetTag: unsafe extern "system" fn(this: *mut *mut Self, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, status: *mut UI_ANIMATION_STORYBOARD_STATUS) -> ::windows_sys::core::HRESULT,
    pub GetElapsedTime: unsafe extern "system" fn(this: *mut *mut Self, elapsedtime: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetStoryboardEventHandler: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUIAnimationStoryboard {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2835288719, data2: 39929, data3: 19185, data4: [158, 103, 229, 228, 16, 222, 251, 132] };
}
#[repr(C)]
pub struct IUIAnimationStoryboard2 {
    pub base__: ::windows_sys::core::IUnknown,
    pub AddTransition: unsafe extern "system" fn(this: *mut *mut Self, variable: *mut ::core::ffi::c_void, transition: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AddKeyframeAtOffset: unsafe extern "system" fn(this: *mut *mut Self, existingkeyframe: UI_ANIMATION_KEYFRAME, offset: f64, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows_sys::core::HRESULT,
    pub AddKeyframeAfterTransition: unsafe extern "system" fn(this: *mut *mut Self, transition: *mut ::core::ffi::c_void, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows_sys::core::HRESULT,
    pub AddTransitionAtKeyframe: unsafe extern "system" fn(this: *mut *mut Self, variable: *mut ::core::ffi::c_void, transition: *mut ::core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows_sys::core::HRESULT,
    pub AddTransitionBetweenKeyframes: unsafe extern "system" fn(this: *mut *mut Self, variable: *mut ::core::ffi::c_void, transition: *mut ::core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RepeatBetweenKeyframes: unsafe extern "system" fn(this: *mut *mut Self, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME, crepetition: f64, repeatmode: UI_ANIMATION_REPEAT_MODE, piterationchangehandler: *mut ::core::ffi::c_void, id: usize, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RepeatBetweenKeyframes: usize,
    pub HoldVariable: unsafe extern "system" fn(this: *mut *mut Self, variable: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetLongestAcceptableDelay: unsafe extern "system" fn(this: *mut *mut Self, delay: f64) -> ::windows_sys::core::HRESULT,
    pub SetSkipDuration: unsafe extern "system" fn(this: *mut *mut Self, secondsduration: f64) -> ::windows_sys::core::HRESULT,
    pub Schedule: unsafe extern "system" fn(this: *mut *mut Self, timenow: f64, schedulingresult: *mut UI_ANIMATION_SCHEDULING_RESULT) -> ::windows_sys::core::HRESULT,
    pub Conclude: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Finish: unsafe extern "system" fn(this: *mut *mut Self, completiondeadline: f64) -> ::windows_sys::core::HRESULT,
    pub Abandon: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut *mut Self, object: *mut ::core::ffi::c_void, id: u32) -> ::windows_sys::core::HRESULT,
    pub GetTag: unsafe extern "system" fn(this: *mut *mut Self, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, status: *mut UI_ANIMATION_STORYBOARD_STATUS) -> ::windows_sys::core::HRESULT,
    pub GetElapsedTime: unsafe extern "system" fn(this: *mut *mut Self, elapsedtime: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetStoryboardEventHandler: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, fregisterstatuschangefornextanimationevent: super::super::Foundation::BOOL, fregisterupdatefornextanimationevent: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetStoryboardEventHandler: usize,
}
impl ::windows_sys::core::Interface for IUIAnimationStoryboard2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2921897170, data2: 4820, data3: 18757, data4: [148, 25, 158, 65, 190, 3, 77, 242] };
}
#[repr(C)]
pub struct IUIAnimationStoryboardEventHandler {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnStoryboardStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, storyboard: *mut ::core::ffi::c_void, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> ::windows_sys::core::HRESULT,
    pub OnStoryboardUpdated: unsafe extern "system" fn(this: *mut *mut Self, storyboard: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUIAnimationStoryboardEventHandler {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1029476360, data2: 60540, data3: 17252, data4: [159, 138, 154, 243, 197, 140, 186, 230] };
}
#[repr(C)]
pub struct IUIAnimationStoryboardEventHandler2 {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnStoryboardStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, storyboard: *mut ::core::ffi::c_void, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> ::windows_sys::core::HRESULT,
    pub OnStoryboardUpdated: unsafe extern "system" fn(this: *mut *mut Self, storyboard: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUIAnimationStoryboardEventHandler2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3133535578, data2: 47740, data3: 16716, data4: [181, 153, 251, 248, 80, 245, 83, 198] };
}
#[repr(C)]
pub struct IUIAnimationTimer {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetTimerUpdateHandler: unsafe extern "system" fn(this: *mut *mut Self, updatehandler: *mut ::core::ffi::c_void, idlebehavior: UI_ANIMATION_IDLE_BEHAVIOR) -> ::windows_sys::core::HRESULT,
    pub SetTimerEventHandler: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Enable: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Disable: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetTime: unsafe extern "system" fn(this: *mut *mut Self, seconds: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetFrameRateThreshold: unsafe extern "system" fn(this: *mut *mut Self, framespersecond: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUIAnimationTimer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1796143825, data2: 41043, data3: 16854, data4: [144, 133, 51, 166, 137, 20, 70, 101] };
}
#[repr(C)]
pub struct IUIAnimationTimerClientEventHandler {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnTimerClientStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, newstatus: UI_ANIMATION_TIMER_CLIENT_STATUS, previousstatus: UI_ANIMATION_TIMER_CLIENT_STATUS) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUIAnimationTimerClientEventHandler {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3202043318, data2: 38138, data3: 19451, data4: [164, 127, 239, 45, 158, 64, 140, 37] };
}
#[repr(C)]
pub struct IUIAnimationTimerEventHandler {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnPreUpdate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub OnPostUpdate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub OnRenderingTooSlow: unsafe extern "system" fn(this: *mut *mut Self, framespersecond: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUIAnimationTimerEventHandler {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 659193322, data2: 55153, data3: 16533, data4: [171, 189, 141, 247, 171, 210, 60, 227] };
}
#[repr(C)]
pub struct IUIAnimationTimerUpdateHandler {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnUpdate: unsafe extern "system" fn(this: *mut *mut Self, timenow: f64, result: *mut UI_ANIMATION_UPDATE_RESULT) -> ::windows_sys::core::HRESULT,
    pub SetTimerClientEventHandler: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ClearTimerClientEventHandler: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUIAnimationTimerUpdateHandler {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 425003447, data2: 23902, data3: 20030, data4: [178, 120, 238, 55, 89, 179, 103, 173] };
}
#[repr(C)]
pub struct IUIAnimationTransition {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetInitialValue: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub SetInitialVelocity: unsafe extern "system" fn(this: *mut *mut Self, velocity: f64) -> ::windows_sys::core::HRESULT,
    pub IsDurationKnown: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetDuration: unsafe extern "system" fn(this: *mut *mut Self, duration: *mut f64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUIAnimationTransition {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3698123346, data2: 63281, data3: 16847, data4: [182, 16, 97, 75, 108, 160, 73, 173] };
}
#[repr(C)]
pub struct IUIAnimationTransition2 {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetDimension: unsafe extern "system" fn(this: *mut *mut Self, dimension: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetInitialValue: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub SetInitialVectorValue: unsafe extern "system" fn(this: *mut *mut Self, value: *const f64, cdimension: u32) -> ::windows_sys::core::HRESULT,
    pub SetInitialVelocity: unsafe extern "system" fn(this: *mut *mut Self, velocity: f64) -> ::windows_sys::core::HRESULT,
    pub SetInitialVectorVelocity: unsafe extern "system" fn(this: *mut *mut Self, velocity: *const f64, cdimension: u32) -> ::windows_sys::core::HRESULT,
    pub IsDurationKnown: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetDuration: unsafe extern "system" fn(this: *mut *mut Self, duration: *mut f64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUIAnimationTransition2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1660916003, data2: 43098, data3: 20123, data4: [162, 24, 67, 90, 147, 226, 104, 253] };
}
#[repr(C)]
pub struct IUIAnimationTransitionFactory {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateTransition: unsafe extern "system" fn(this: *mut *mut Self, interpolator: *mut ::core::ffi::c_void, transition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUIAnimationTransitionFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4242087427, data2: 15931, data3: 17837, data4: [187, 177, 109, 252, 129, 83, 116, 61] };
}
#[repr(C)]
pub struct IUIAnimationTransitionFactory2 {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateTransition: unsafe extern "system" fn(this: *mut *mut Self, interpolator: *mut ::core::ffi::c_void, transition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUIAnimationTransitionFactory2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2474461462, data2: 49574, data3: 17109, data4: [136, 216, 48, 52, 77, 110, 254, 49] };
}
#[repr(C)]
pub struct IUIAnimationTransitionLibrary {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateInstantaneousTransition: unsafe extern "system" fn(this: *mut *mut Self, finalvalue: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateConstantTransition: unsafe extern "system" fn(this: *mut *mut Self, duration: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateDiscreteTransition: unsafe extern "system" fn(this: *mut *mut Self, delay: f64, finalvalue: f64, hold: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateLinearTransition: unsafe extern "system" fn(this: *mut *mut Self, duration: f64, finalvalue: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateLinearTransitionFromSpeed: unsafe extern "system" fn(this: *mut *mut Self, speed: f64, finalvalue: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSinusoidalTransitionFromVelocity: unsafe extern "system" fn(this: *mut *mut Self, duration: f64, period: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSinusoidalTransitionFromRange: unsafe extern "system" fn(this: *mut *mut Self, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE, transition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateAccelerateDecelerateTransition: unsafe extern "system" fn(this: *mut *mut Self, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateReversalTransition: unsafe extern "system" fn(this: *mut *mut Self, duration: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateCubicTransition: unsafe extern "system" fn(this: *mut *mut Self, duration: f64, finalvalue: f64, finalvelocity: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSmoothStopTransition: unsafe extern "system" fn(this: *mut *mut Self, maximumduration: f64, finalvalue: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateParabolicTransitionFromAcceleration: unsafe extern "system" fn(this: *mut *mut Self, finalvalue: f64, finalvelocity: f64, acceleration: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUIAnimationTransitionLibrary {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3394901169, data2: 53839, data3: 18616, data4: [143, 228, 199, 129, 105, 186, 149, 78] };
}
#[repr(C)]
pub struct IUIAnimationTransitionLibrary2 {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateInstantaneousTransition: unsafe extern "system" fn(this: *mut *mut Self, finalvalue: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateInstantaneousVectorTransition: unsafe extern "system" fn(this: *mut *mut Self, finalvalue: *const f64, cdimension: u32, transition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateConstantTransition: unsafe extern "system" fn(this: *mut *mut Self, duration: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateDiscreteTransition: unsafe extern "system" fn(this: *mut *mut Self, delay: f64, finalvalue: f64, hold: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateDiscreteVectorTransition: unsafe extern "system" fn(this: *mut *mut Self, delay: f64, finalvalue: *const f64, cdimension: u32, hold: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateLinearTransition: unsafe extern "system" fn(this: *mut *mut Self, duration: f64, finalvalue: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateLinearVectorTransition: unsafe extern "system" fn(this: *mut *mut Self, duration: f64, finalvalue: *const f64, cdimension: u32, transition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateLinearTransitionFromSpeed: unsafe extern "system" fn(this: *mut *mut Self, speed: f64, finalvalue: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateLinearVectorTransitionFromSpeed: unsafe extern "system" fn(this: *mut *mut Self, speed: f64, finalvalue: *const f64, cdimension: u32, transition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSinusoidalTransitionFromVelocity: unsafe extern "system" fn(this: *mut *mut Self, duration: f64, period: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSinusoidalTransitionFromRange: unsafe extern "system" fn(this: *mut *mut Self, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE, transition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateAccelerateDecelerateTransition: unsafe extern "system" fn(this: *mut *mut Self, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateReversalTransition: unsafe extern "system" fn(this: *mut *mut Self, duration: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateCubicTransition: unsafe extern "system" fn(this: *mut *mut Self, duration: f64, finalvalue: f64, finalvelocity: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateCubicVectorTransition: unsafe extern "system" fn(this: *mut *mut Self, duration: f64, finalvalue: *const f64, finalvelocity: *const f64, cdimension: u32, transition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSmoothStopTransition: unsafe extern "system" fn(this: *mut *mut Self, maximumduration: f64, finalvalue: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateParabolicTransitionFromAcceleration: unsafe extern "system" fn(this: *mut *mut Self, finalvalue: f64, finalvelocity: f64, acceleration: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateCubicBezierLinearTransition: unsafe extern "system" fn(this: *mut *mut Self, duration: f64, finalvalue: f64, x1: f64, y1: f64, x2: f64, y2: f64, pptransition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateCubicBezierLinearVectorTransition: unsafe extern "system" fn(this: *mut *mut Self, duration: f64, finalvalue: *const f64, cdimension: u32, x1: f64, y1: f64, x2: f64, y2: f64, pptransition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUIAnimationTransitionLibrary2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 63942227, data2: 38272, data3: 20195, data4: [179, 99, 46, 206, 81, 180, 175, 106] };
}
#[repr(C)]
pub struct IUIAnimationVariable {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut f64) -> ::windows_sys::core::HRESULT,
    pub GetFinalValue: unsafe extern "system" fn(this: *mut *mut Self, finalvalue: *mut f64) -> ::windows_sys::core::HRESULT,
    pub GetPreviousValue: unsafe extern "system" fn(this: *mut *mut Self, previousvalue: *mut f64) -> ::windows_sys::core::HRESULT,
    pub GetIntegerValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetFinalIntegerValue: unsafe extern "system" fn(this: *mut *mut Self, finalvalue: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetPreviousIntegerValue: unsafe extern "system" fn(this: *mut *mut Self, previousvalue: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetCurrentStoryboard: unsafe extern "system" fn(this: *mut *mut Self, storyboard: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetLowerBound: unsafe extern "system" fn(this: *mut *mut Self, bound: f64) -> ::windows_sys::core::HRESULT,
    pub SetUpperBound: unsafe extern "system" fn(this: *mut *mut Self, bound: f64) -> ::windows_sys::core::HRESULT,
    pub SetRoundingMode: unsafe extern "system" fn(this: *mut *mut Self, mode: UI_ANIMATION_ROUNDING_MODE) -> ::windows_sys::core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut *mut Self, object: *mut ::core::ffi::c_void, id: u32) -> ::windows_sys::core::HRESULT,
    pub GetTag: unsafe extern "system" fn(this: *mut *mut Self, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetVariableChangeHandler: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetVariableIntegerChangeHandler: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUIAnimationVariable {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2364453205, data2: 10313, data3: 19685, data4: [148, 72, 145, 255, 112, 225, 228, 217] };
}
#[repr(C)]
pub struct IUIAnimationVariable2 {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetDimension: unsafe extern "system" fn(this: *mut *mut Self, dimension: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut f64) -> ::windows_sys::core::HRESULT,
    pub GetVectorValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut f64, cdimension: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_DirectComposition")]
    pub GetCurve: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_DirectComposition"))]
    GetCurve: usize,
    #[cfg(feature = "Win32_Graphics_DirectComposition")]
    pub GetVectorCurve: unsafe extern "system" fn(this: *mut *mut Self, animation: *const *mut ::core::ffi::c_void, cdimension: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_DirectComposition"))]
    GetVectorCurve: usize,
    pub GetFinalValue: unsafe extern "system" fn(this: *mut *mut Self, finalvalue: *mut f64) -> ::windows_sys::core::HRESULT,
    pub GetFinalVectorValue: unsafe extern "system" fn(this: *mut *mut Self, finalvalue: *mut f64, cdimension: u32) -> ::windows_sys::core::HRESULT,
    pub GetPreviousValue: unsafe extern "system" fn(this: *mut *mut Self, previousvalue: *mut f64) -> ::windows_sys::core::HRESULT,
    pub GetPreviousVectorValue: unsafe extern "system" fn(this: *mut *mut Self, previousvalue: *mut f64, cdimension: u32) -> ::windows_sys::core::HRESULT,
    pub GetIntegerValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetIntegerVectorValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32, cdimension: u32) -> ::windows_sys::core::HRESULT,
    pub GetFinalIntegerValue: unsafe extern "system" fn(this: *mut *mut Self, finalvalue: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetFinalIntegerVectorValue: unsafe extern "system" fn(this: *mut *mut Self, finalvalue: *mut i32, cdimension: u32) -> ::windows_sys::core::HRESULT,
    pub GetPreviousIntegerValue: unsafe extern "system" fn(this: *mut *mut Self, previousvalue: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetPreviousIntegerVectorValue: unsafe extern "system" fn(this: *mut *mut Self, previousvalue: *mut i32, cdimension: u32) -> ::windows_sys::core::HRESULT,
    pub GetCurrentStoryboard: unsafe extern "system" fn(this: *mut *mut Self, storyboard: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetLowerBound: unsafe extern "system" fn(this: *mut *mut Self, bound: f64) -> ::windows_sys::core::HRESULT,
    pub SetLowerBoundVector: unsafe extern "system" fn(this: *mut *mut Self, bound: *const f64, cdimension: u32) -> ::windows_sys::core::HRESULT,
    pub SetUpperBound: unsafe extern "system" fn(this: *mut *mut Self, bound: f64) -> ::windows_sys::core::HRESULT,
    pub SetUpperBoundVector: unsafe extern "system" fn(this: *mut *mut Self, bound: *const f64, cdimension: u32) -> ::windows_sys::core::HRESULT,
    pub SetRoundingMode: unsafe extern "system" fn(this: *mut *mut Self, mode: UI_ANIMATION_ROUNDING_MODE) -> ::windows_sys::core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut *mut Self, object: *mut ::core::ffi::c_void, id: u32) -> ::windows_sys::core::HRESULT,
    pub GetTag: unsafe extern "system" fn(this: *mut *mut Self, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetVariableChangeHandler: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetVariableChangeHandler: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetVariableIntegerChangeHandler: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetVariableIntegerChangeHandler: usize,
    pub SetVariableCurveChangeHandler: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUIAnimationVariable2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1226093316, data2: 38571, data3: 17625, data4: [158, 119, 213, 16, 155, 126, 116, 102] };
}
#[repr(C)]
pub struct IUIAnimationVariableChangeHandler {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnValueChanged: unsafe extern "system" fn(this: *mut *mut Self, storyboard: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, newvalue: f64, previousvalue: f64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUIAnimationVariableChangeHandler {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1666758586, data2: 34770, data3: 17109, data4: [191, 113, 130, 233, 25, 221, 88, 98] };
}
#[repr(C)]
pub struct IUIAnimationVariableChangeHandler2 {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnValueChanged: unsafe extern "system" fn(this: *mut *mut Self, storyboard: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, newvalue: *const f64, previousvalue: *const f64, cdimension: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUIAnimationVariableChangeHandler2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1672267986, data2: 28334, data3: 19376, data4: [184, 121, 88, 109, 216, 207, 190, 66] };
}
#[repr(C)]
pub struct IUIAnimationVariableCurveChangeHandler2 {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnCurveChanged: unsafe extern "system" fn(this: *mut *mut Self, variable: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUIAnimationVariableCurveChangeHandler2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921605265, data2: 325, data3: 19489, data4: [145, 146, 90, 171, 64, 237, 223, 128] };
}
#[repr(C)]
pub struct IUIAnimationVariableIntegerChangeHandler {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnIntegerValueChanged: unsafe extern "system" fn(this: *mut *mut Self, storyboard: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, newvalue: i32, previousvalue: i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUIAnimationVariableIntegerChangeHandler {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3141408080, data2: 13678, data3: 17584, data4: [153, 218, 133, 172, 96, 23, 134, 94] };
}
#[repr(C)]
pub struct IUIAnimationVariableIntegerChangeHandler2 {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnIntegerValueChanged: unsafe extern "system" fn(this: *mut *mut Self, storyboard: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, newvalue: *const i32, previousvalue: *const i32, cdimension: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUIAnimationVariableIntegerChangeHandler2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2191224049, data2: 20282, data3: 17426, data4: [174, 9, 178, 67, 235, 76, 107, 88] };
}
pub const UIAnimationManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1277150778, data2: 26972, data3: 18408, data4: [163, 57, 26, 25, 75, 227, 208, 184] };
pub const UIAnimationManager2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3529345090, data2: 34948, data3: 19018, data4: [179, 33, 9, 19, 20, 55, 155, 221] };
pub const UIAnimationTimer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3217902092, data2: 1718, data3: 17284, data4: [183, 104, 13, 170, 121, 44, 56, 14] };
pub const UIAnimationTransitionFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2325421277, data2: 64727, data3: 16796, data4: [139, 68, 66, 253, 23, 219, 24, 135] };
pub const UIAnimationTransitionFactory2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2217750423, data2: 32635, data3: 16448, data4: [177, 144, 114, 172, 157, 24, 228, 32] };
pub const UIAnimationTransitionLibrary: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 493036205, data2: 43653, data3: 20213, data4: [168, 40, 134, 215, 16, 103, 209, 69] };
pub const UIAnimationTransitionLibrary2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2167379018, data2: 50632, data3: 19673, data4: [176, 166, 179, 218, 128, 47, 34, 141] };
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub type UI_ANIMATION_DEPENDENCIES = u32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_DEPENDENCY_NONE: UI_ANIMATION_DEPENDENCIES = 0u32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_DEPENDENCY_INTERMEDIATE_VALUES: UI_ANIMATION_DEPENDENCIES = 1u32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_DEPENDENCY_FINAL_VALUE: UI_ANIMATION_DEPENDENCIES = 2u32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_DEPENDENCY_FINAL_VELOCITY: UI_ANIMATION_DEPENDENCIES = 4u32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_DEPENDENCY_DURATION: UI_ANIMATION_DEPENDENCIES = 8u32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub type UI_ANIMATION_IDLE_BEHAVIOR = i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_IDLE_BEHAVIOR_CONTINUE: UI_ANIMATION_IDLE_BEHAVIOR = 0i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_IDLE_BEHAVIOR_DISABLE: UI_ANIMATION_IDLE_BEHAVIOR = 1i32;
pub type UI_ANIMATION_KEYFRAME = isize;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub type UI_ANIMATION_MANAGER_STATUS = i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_MANAGER_IDLE: UI_ANIMATION_MANAGER_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_MANAGER_BUSY: UI_ANIMATION_MANAGER_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub type UI_ANIMATION_MODE = i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_MODE_DISABLED: UI_ANIMATION_MODE = 0i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_MODE_SYSTEM_DEFAULT: UI_ANIMATION_MODE = 1i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_MODE_ENABLED: UI_ANIMATION_MODE = 2i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub type UI_ANIMATION_PRIORITY_EFFECT = i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_PRIORITY_EFFECT_FAILURE: UI_ANIMATION_PRIORITY_EFFECT = 0i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_PRIORITY_EFFECT_DELAY: UI_ANIMATION_PRIORITY_EFFECT = 1i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_REPEAT_INDEFINITELY: i32 = -1i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_REPEAT_INDEFINITELY_CONCLUDE_AT_END: i32 = -1i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_REPEAT_INDEFINITELY_CONCLUDE_AT_START: i32 = -2i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub type UI_ANIMATION_REPEAT_MODE = i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_REPEAT_MODE_NORMAL: UI_ANIMATION_REPEAT_MODE = 0i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_REPEAT_MODE_ALTERNATE: UI_ANIMATION_REPEAT_MODE = 1i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub type UI_ANIMATION_ROUNDING_MODE = i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_ROUNDING_NEAREST: UI_ANIMATION_ROUNDING_MODE = 0i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_ROUNDING_FLOOR: UI_ANIMATION_ROUNDING_MODE = 1i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_ROUNDING_CEILING: UI_ANIMATION_ROUNDING_MODE = 2i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub type UI_ANIMATION_SCHEDULING_RESULT = i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_SCHEDULING_UNEXPECTED_FAILURE: UI_ANIMATION_SCHEDULING_RESULT = 0i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_SCHEDULING_INSUFFICIENT_PRIORITY: UI_ANIMATION_SCHEDULING_RESULT = 1i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_SCHEDULING_ALREADY_SCHEDULED: UI_ANIMATION_SCHEDULING_RESULT = 2i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_SCHEDULING_SUCCEEDED: UI_ANIMATION_SCHEDULING_RESULT = 3i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_SCHEDULING_DEFERRED: UI_ANIMATION_SCHEDULING_RESULT = 4i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_SECONDS_EVENTUALLY: i32 = -1i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_SECONDS_INFINITE: i32 = -1i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub type UI_ANIMATION_SLOPE = i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_SLOPE_INCREASING: UI_ANIMATION_SLOPE = 0i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_SLOPE_DECREASING: UI_ANIMATION_SLOPE = 1i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub type UI_ANIMATION_STORYBOARD_STATUS = i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_STORYBOARD_BUILDING: UI_ANIMATION_STORYBOARD_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_STORYBOARD_SCHEDULED: UI_ANIMATION_STORYBOARD_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_STORYBOARD_CANCELLED: UI_ANIMATION_STORYBOARD_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_STORYBOARD_PLAYING: UI_ANIMATION_STORYBOARD_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_STORYBOARD_TRUNCATED: UI_ANIMATION_STORYBOARD_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_STORYBOARD_FINISHED: UI_ANIMATION_STORYBOARD_STATUS = 5i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_STORYBOARD_READY: UI_ANIMATION_STORYBOARD_STATUS = 6i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_STORYBOARD_INSUFFICIENT_PRIORITY: UI_ANIMATION_STORYBOARD_STATUS = 7i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub type UI_ANIMATION_TIMER_CLIENT_STATUS = i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_TIMER_CLIENT_IDLE: UI_ANIMATION_TIMER_CLIENT_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_TIMER_CLIENT_BUSY: UI_ANIMATION_TIMER_CLIENT_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub type UI_ANIMATION_UPDATE_RESULT = i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_UPDATE_NO_CHANGE: UI_ANIMATION_UPDATE_RESULT = 0i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_UPDATE_VARIABLES_CHANGED: UI_ANIMATION_UPDATE_RESULT = 1i32;
