pub type EyesPose = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Perception_People\"`*"]
#[repr(transparent)]
pub struct HandJointKind(pub i32);
impl HandJointKind {
    pub const Palm: Self = Self(0i32);
    pub const Wrist: Self = Self(1i32);
    pub const ThumbMetacarpal: Self = Self(2i32);
    pub const ThumbProximal: Self = Self(3i32);
    pub const ThumbDistal: Self = Self(4i32);
    pub const ThumbTip: Self = Self(5i32);
    pub const IndexMetacarpal: Self = Self(6i32);
    pub const IndexProximal: Self = Self(7i32);
    pub const IndexIntermediate: Self = Self(8i32);
    pub const IndexDistal: Self = Self(9i32);
    pub const IndexTip: Self = Self(10i32);
    pub const MiddleMetacarpal: Self = Self(11i32);
    pub const MiddleProximal: Self = Self(12i32);
    pub const MiddleIntermediate: Self = Self(13i32);
    pub const MiddleDistal: Self = Self(14i32);
    pub const MiddleTip: Self = Self(15i32);
    pub const RingMetacarpal: Self = Self(16i32);
    pub const RingProximal: Self = Self(17i32);
    pub const RingIntermediate: Self = Self(18i32);
    pub const RingDistal: Self = Self(19i32);
    pub const RingTip: Self = Self(20i32);
    pub const LittleMetacarpal: Self = Self(21i32);
    pub const LittleProximal: Self = Self(22i32);
    pub const LittleIntermediate: Self = Self(23i32);
    pub const LittleDistal: Self = Self(24i32);
    pub const LittleTip: Self = Self(25i32);
}
impl ::core::marker::Copy for HandJointKind {}
impl ::core::clone::Clone for HandJointKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type HandMeshObserver = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"Perception_People\"`, `\"Foundation_Numerics\"`*"]
#[cfg(feature = "Foundation_Numerics")]
pub struct HandMeshVertex {
    pub Position: super::super::Foundation::Numerics::Vector3,
    pub Normal: super::super::Foundation::Numerics::Vector3,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for HandMeshVertex {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for HandMeshVertex {
    fn clone(&self) -> Self {
        *self
    }
}
pub type HandMeshVertexState = *mut ::core::ffi::c_void;
pub type HandPose = *mut ::core::ffi::c_void;
pub type HeadPose = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IEyesPose {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsCalibrationValid: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub Gaze: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    Gaze: usize,
    pub UpdateTimestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEyesPose {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1747622691, data2: 35358, data3: 23430, data4: [160, 96, 144, 111, 250, 203, 98, 164] };
}
#[repr(C)]
pub struct IEyesPoseStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))]
    RequestAccessAsync: usize,
}
impl ::windows_sys::core::Interface for IEyesPoseStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 486503443, data2: 45599, data3: 21696, data4: [128, 193, 230, 13, 153, 76, 165, 140] };
}
#[repr(C)]
pub struct IHandMeshObserver {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Input_Spatial")]
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Input_Spatial"))]
    Source: usize,
    pub TriangleIndexCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub VertexCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetTriangleIndices: unsafe extern "system" fn(this: *mut *mut Self, indices_array_size: u32, indices: *mut u16) -> ::windows_sys::core::HRESULT,
    pub GetVertexStateForPose: unsafe extern "system" fn(this: *mut *mut Self, handpose: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NeutralPose: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NeutralPoseVersion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ModelId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHandMeshObserver {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2242785483, data2: 28611, data3: 21956, data4: [167, 180, 41, 227, 56, 150, 202, 105] };
}
#[repr(C)]
pub struct IHandMeshVertexState {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Perception_Spatial")]
    pub CoordinateSystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    CoordinateSystem: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub GetVertices: unsafe extern "system" fn(this: *mut *mut Self, vertices_array_size: u32, vertices: *mut HandMeshVertex) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    GetVertices: usize,
    pub UpdateTimestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHandMeshVertexState {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 74211311, data2: 7563, data3: 21982, data4: [171, 44, 28, 212, 36, 136, 109, 143] };
}
#[repr(C)]
pub struct IHandPose {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub TryGetJoint: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, joint: HandJointKind, jointpose: *mut JointPose, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    TryGetJoint: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub TryGetJoints: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, joints_array_size: u32, joints: *const HandJointKind, jointPoses_array_size: u32, jointposes: *mut JointPose, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    TryGetJoints: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub GetRelativeJoint: unsafe extern "system" fn(this: *mut *mut Self, joint: HandJointKind, referencejoint: HandJointKind, result__: *mut JointPose) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    GetRelativeJoint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub GetRelativeJoints: unsafe extern "system" fn(this: *mut *mut Self, joints_array_size: u32, joints: *const HandJointKind, referenceJoints_array_size: u32, referencejoints: *const HandJointKind, jointPoses_array_size: u32, jointposes: *mut JointPose) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    GetRelativeJoints: usize,
}
impl ::windows_sys::core::Interface for IHandPose {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1301866394, data2: 47880, data3: 23817, data4: [145, 222, 223, 13, 211, 250, 228, 108] };
}
#[repr(C)]
pub struct IHeadPose {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Position: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub ForwardDirection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ForwardDirection: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub UpDirection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    UpDirection: usize,
}
impl ::windows_sys::core::Interface for IHeadPose {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2136655269, data2: 18907, data3: 14239, data4: [148, 41, 50, 162, 250, 243, 79, 166] };
}
#[repr(C)]
#[doc = "*Required features: `\"Perception_People\"`, `\"Foundation_Numerics\"`*"]
#[cfg(feature = "Foundation_Numerics")]
pub struct JointPose {
    pub Orientation: super::super::Foundation::Numerics::Quaternion,
    pub Position: super::super::Foundation::Numerics::Vector3,
    pub Radius: f32,
    pub Accuracy: JointPoseAccuracy,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for JointPose {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for JointPose {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Perception_People\"`*"]
#[repr(transparent)]
pub struct JointPoseAccuracy(pub i32);
impl JointPoseAccuracy {
    pub const High: Self = Self(0i32);
    pub const Approximate: Self = Self(1i32);
}
impl ::core::marker::Copy for JointPoseAccuracy {}
impl ::core::clone::Clone for JointPoseAccuracy {
    fn clone(&self) -> Self {
        *self
    }
}
