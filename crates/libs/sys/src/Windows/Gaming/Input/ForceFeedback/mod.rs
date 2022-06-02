pub type ConditionForceEffect = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
#[repr(transparent)]
pub struct ConditionForceEffectKind(pub i32);
impl ConditionForceEffectKind {
    pub const Spring: Self = Self(0i32);
    pub const Damper: Self = Self(1i32);
    pub const Inertia: Self = Self(2i32);
    pub const Friction: Self = Self(3i32);
}
impl ::core::marker::Copy for ConditionForceEffectKind {}
impl ::core::clone::Clone for ConditionForceEffectKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ConstantForceEffect = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
#[repr(transparent)]
pub struct ForceFeedbackEffectAxes(pub u32);
impl ForceFeedbackEffectAxes {
    pub const None: Self = Self(0u32);
    pub const X: Self = Self(1u32);
    pub const Y: Self = Self(2u32);
    pub const Z: Self = Self(4u32);
}
impl ::core::marker::Copy for ForceFeedbackEffectAxes {}
impl ::core::clone::Clone for ForceFeedbackEffectAxes {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
#[repr(transparent)]
pub struct ForceFeedbackEffectState(pub i32);
impl ForceFeedbackEffectState {
    pub const Stopped: Self = Self(0i32);
    pub const Running: Self = Self(1i32);
    pub const Paused: Self = Self(2i32);
    pub const Faulted: Self = Self(3i32);
}
impl ::core::marker::Copy for ForceFeedbackEffectState {}
impl ::core::clone::Clone for ForceFeedbackEffectState {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
#[repr(transparent)]
pub struct ForceFeedbackLoadEffectResult(pub i32);
impl ForceFeedbackLoadEffectResult {
    pub const Succeeded: Self = Self(0i32);
    pub const EffectStorageFull: Self = Self(1i32);
    pub const EffectNotSupported: Self = Self(2i32);
}
impl ::core::marker::Copy for ForceFeedbackLoadEffectResult {}
impl ::core::clone::Clone for ForceFeedbackLoadEffectResult {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ForceFeedbackMotor = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IConditionForceEffect {
    pub base__: ::windows_sys::core::IInspectable,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ConditionForceEffectKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetParameters: unsafe extern "system" fn(this: *mut *mut Self, direction: super::super::super::Foundation::Numerics::Vector3, positivecoefficient: f32, negativecoefficient: f32, maxpositivemagnitude: f32, maxnegativemagnitude: f32, deadzone: f32, bias: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetParameters: usize,
}
#[repr(C)]
pub struct IConditionForceEffectFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, effectkind: ConditionForceEffectKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IConstantForceEffect {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetParameters: unsafe extern "system" fn(this: *mut *mut Self, vector: super::super::super::Foundation::Numerics::Vector3, duration: super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetParameters: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetParametersWithEnvelope: unsafe extern "system" fn(this: *mut *mut Self, vector: super::super::super::Foundation::Numerics::Vector3, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: super::super::super::Foundation::TimeSpan, attackduration: super::super::super::Foundation::TimeSpan, sustainduration: super::super::super::Foundation::TimeSpan, releaseduration: super::super::super::Foundation::TimeSpan, repeatcount: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetParametersWithEnvelope: usize,
}
#[repr(C)]
pub struct IForceFeedbackEffect {
    pub base__: ::windows_sys::core::IInspectable,
    pub Gain: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetGain: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ForceFeedbackEffectState) -> ::windows_sys::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IForceFeedbackMotor {
    pub base__: ::windows_sys::core::IInspectable,
    pub AreEffectsPaused: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub MasterGain: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetMasterGain: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SupportedAxes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ForceFeedbackEffectAxes) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LoadEffectAsync: unsafe extern "system" fn(this: *mut *mut Self, effect: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadEffectAsync: usize,
    pub PauseAllEffects: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ResumeAllEffects: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub StopAllEffects: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryDisableAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryDisableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryEnableAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryEnableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryResetAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryResetAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryUnloadEffectAsync: unsafe extern "system" fn(this: *mut *mut Self, effect: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryUnloadEffectAsync: usize,
}
#[repr(C)]
pub struct IPeriodicForceEffect {
    pub base__: ::windows_sys::core::IInspectable,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PeriodicForceEffectKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetParameters: unsafe extern "system" fn(this: *mut *mut Self, vector: super::super::super::Foundation::Numerics::Vector3, frequency: f32, phase: f32, bias: f32, duration: super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetParameters: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetParametersWithEnvelope: unsafe extern "system" fn(this: *mut *mut Self, vector: super::super::super::Foundation::Numerics::Vector3, frequency: f32, phase: f32, bias: f32, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: super::super::super::Foundation::TimeSpan, attackduration: super::super::super::Foundation::TimeSpan, sustainduration: super::super::super::Foundation::TimeSpan, releaseduration: super::super::super::Foundation::TimeSpan, repeatcount: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetParametersWithEnvelope: usize,
}
#[repr(C)]
pub struct IPeriodicForceEffectFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, effectkind: PeriodicForceEffectKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRampForceEffect {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetParameters: unsafe extern "system" fn(this: *mut *mut Self, startvector: super::super::super::Foundation::Numerics::Vector3, endvector: super::super::super::Foundation::Numerics::Vector3, duration: super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetParameters: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetParametersWithEnvelope: unsafe extern "system" fn(this: *mut *mut Self, startvector: super::super::super::Foundation::Numerics::Vector3, endvector: super::super::super::Foundation::Numerics::Vector3, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: super::super::super::Foundation::TimeSpan, attackduration: super::super::super::Foundation::TimeSpan, sustainduration: super::super::super::Foundation::TimeSpan, releaseduration: super::super::super::Foundation::TimeSpan, repeatcount: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetParametersWithEnvelope: usize,
}
pub type PeriodicForceEffect = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
#[repr(transparent)]
pub struct PeriodicForceEffectKind(pub i32);
impl PeriodicForceEffectKind {
    pub const SquareWave: Self = Self(0i32);
    pub const SineWave: Self = Self(1i32);
    pub const TriangleWave: Self = Self(2i32);
    pub const SawtoothWaveUp: Self = Self(3i32);
    pub const SawtoothWaveDown: Self = Self(4i32);
}
impl ::core::marker::Copy for PeriodicForceEffectKind {}
impl ::core::clone::Clone for PeriodicForceEffectKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RampForceEffect = *mut ::core::ffi::c_void;
