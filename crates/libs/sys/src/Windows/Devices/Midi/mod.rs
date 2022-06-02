#[repr(C)]
pub struct IMidiChannelPressureMessage {
    pub base__: ::windows_sys::core::IInspectable,
    pub Channel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Pressure: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMidiChannelPressureMessageFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateMidiChannelPressureMessage: unsafe extern "system" fn(this: *mut *mut Self, channel: u8, pressure: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMidiControlChangeMessage {
    pub base__: ::windows_sys::core::IInspectable,
    pub Channel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Controller: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub ControlValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMidiControlChangeMessageFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateMidiControlChangeMessage: unsafe extern "system" fn(this: *mut *mut Self, channel: u8, controller: u8, controlvalue: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMidiInPort {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub MessageReceived: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MessageReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMessageReceived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMessageReceived: usize,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMidiInPortStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMidiMessage {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    #[cfg(feature = "Storage_Streams")]
    pub RawData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    RawData: usize,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MidiMessageType) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMidiMessageReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Message: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMidiNoteOffMessage {
    pub base__: ::windows_sys::core::IInspectable,
    pub Channel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Note: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Velocity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMidiNoteOffMessageFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateMidiNoteOffMessage: unsafe extern "system" fn(this: *mut *mut Self, channel: u8, note: u8, velocity: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMidiNoteOnMessage {
    pub base__: ::windows_sys::core::IInspectable,
    pub Channel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Note: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Velocity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMidiNoteOnMessageFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateMidiNoteOnMessage: unsafe extern "system" fn(this: *mut *mut Self, channel: u8, note: u8, velocity: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMidiOutPort {
    pub base__: ::windows_sys::core::IInspectable,
    pub SendMessage: unsafe extern "system" fn(this: *mut *mut Self, midimessage: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SendBuffer: unsafe extern "system" fn(this: *mut *mut Self, mididata: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SendBuffer: usize,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMidiOutPortStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMidiPitchBendChangeMessage {
    pub base__: ::windows_sys::core::IInspectable,
    pub Channel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Bend: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMidiPitchBendChangeMessageFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateMidiPitchBendChangeMessage: unsafe extern "system" fn(this: *mut *mut Self, channel: u8, bend: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMidiPolyphonicKeyPressureMessage {
    pub base__: ::windows_sys::core::IInspectable,
    pub Channel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Note: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Pressure: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMidiPolyphonicKeyPressureMessageFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateMidiPolyphonicKeyPressureMessage: unsafe extern "system" fn(this: *mut *mut Self, channel: u8, note: u8, pressure: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMidiProgramChangeMessage {
    pub base__: ::windows_sys::core::IInspectable,
    pub Channel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Program: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMidiProgramChangeMessageFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateMidiProgramChangeMessage: unsafe extern "system" fn(this: *mut *mut Self, channel: u8, program: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMidiSongPositionPointerMessage {
    pub base__: ::windows_sys::core::IInspectable,
    pub Beats: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMidiSongPositionPointerMessageFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateMidiSongPositionPointerMessage: unsafe extern "system" fn(this: *mut *mut Self, beats: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMidiSongSelectMessage {
    pub base__: ::windows_sys::core::IInspectable,
    pub Song: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMidiSongSelectMessageFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateMidiSongSelectMessage: unsafe extern "system" fn(this: *mut *mut Self, song: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMidiSynthesizer {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Enumeration")]
    pub AudioDevice: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    AudioDevice: usize,
    pub Volume: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetVolume: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMidiSynthesizerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAsync: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub CreateFromAudioDeviceAsync: unsafe extern "system" fn(this: *mut *mut Self, audiodevice: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation")))]
    CreateFromAudioDeviceAsync: usize,
    #[cfg(feature = "Devices_Enumeration")]
    pub IsSynthesizer: unsafe extern "system" fn(this: *mut *mut Self, mididevice: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    IsSynthesizer: usize,
}
#[repr(C)]
pub struct IMidiSystemExclusiveMessageFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub CreateMidiSystemExclusiveMessage: unsafe extern "system" fn(this: *mut *mut Self, rawdata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateMidiSystemExclusiveMessage: usize,
}
#[repr(C)]
pub struct IMidiTimeCodeMessage {
    pub base__: ::windows_sys::core::IInspectable,
    pub FrameType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Values: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMidiTimeCodeMessageFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateMidiTimeCodeMessage: unsafe extern "system" fn(this: *mut *mut Self, frametype: u8, values: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
pub type MidiActiveSensingMessage = *mut ::core::ffi::c_void;
pub type MidiChannelPressureMessage = *mut ::core::ffi::c_void;
pub type MidiContinueMessage = *mut ::core::ffi::c_void;
pub type MidiControlChangeMessage = *mut ::core::ffi::c_void;
pub type MidiInPort = *mut ::core::ffi::c_void;
pub type MidiMessageReceivedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiMessageType(pub i32);
impl MidiMessageType {
    pub const None: Self = Self(0i32);
    pub const NoteOff: Self = Self(128i32);
    pub const NoteOn: Self = Self(144i32);
    pub const PolyphonicKeyPressure: Self = Self(160i32);
    pub const ControlChange: Self = Self(176i32);
    pub const ProgramChange: Self = Self(192i32);
    pub const ChannelPressure: Self = Self(208i32);
    pub const PitchBendChange: Self = Self(224i32);
    pub const SystemExclusive: Self = Self(240i32);
    pub const MidiTimeCode: Self = Self(241i32);
    pub const SongPositionPointer: Self = Self(242i32);
    pub const SongSelect: Self = Self(243i32);
    pub const TuneRequest: Self = Self(246i32);
    pub const EndSystemExclusive: Self = Self(247i32);
    pub const TimingClock: Self = Self(248i32);
    pub const Start: Self = Self(250i32);
    pub const Continue: Self = Self(251i32);
    pub const Stop: Self = Self(252i32);
    pub const ActiveSensing: Self = Self(254i32);
    pub const SystemReset: Self = Self(255i32);
}
impl ::core::marker::Copy for MidiMessageType {}
impl ::core::clone::Clone for MidiMessageType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MidiNoteOffMessage = *mut ::core::ffi::c_void;
pub type MidiNoteOnMessage = *mut ::core::ffi::c_void;
pub type MidiOutPort = *mut ::core::ffi::c_void;
pub type MidiPitchBendChangeMessage = *mut ::core::ffi::c_void;
pub type MidiPolyphonicKeyPressureMessage = *mut ::core::ffi::c_void;
pub type MidiProgramChangeMessage = *mut ::core::ffi::c_void;
pub type MidiSongPositionPointerMessage = *mut ::core::ffi::c_void;
pub type MidiSongSelectMessage = *mut ::core::ffi::c_void;
pub type MidiStartMessage = *mut ::core::ffi::c_void;
pub type MidiStopMessage = *mut ::core::ffi::c_void;
pub type MidiSynthesizer = *mut ::core::ffi::c_void;
pub type MidiSystemExclusiveMessage = *mut ::core::ffi::c_void;
pub type MidiSystemResetMessage = *mut ::core::ffi::c_void;
pub type MidiTimeCodeMessage = *mut ::core::ffi::c_void;
pub type MidiTimingClockMessage = *mut ::core::ffi::c_void;
pub type MidiTuneRequestMessage = *mut ::core::ffi::c_void;
