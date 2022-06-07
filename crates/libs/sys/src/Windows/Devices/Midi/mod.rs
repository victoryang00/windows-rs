#[repr(C)]
pub struct IMidiChannelPressureMessage {
    pub base__: ::windows_sys::core::IInspectable,
    pub Channel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Pressure: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMidiChannelPressureMessage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3189745760, data2: 25268, data3: 19794, data4: [163, 126, 146, 229, 77, 53, 185, 9] };
}
#[repr(C)]
pub struct IMidiChannelPressureMessageFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateMidiChannelPressureMessage: unsafe extern "system" fn(this: *mut *mut Self, channel: u8, pressure: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMidiChannelPressureMessageFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1645800751, data2: 8836, data3: 16682, data4: [148, 207, 16, 251, 4, 132, 44, 108] };
}
#[repr(C)]
pub struct IMidiControlChangeMessage {
    pub base__: ::windows_sys::core::IInspectable,
    pub Channel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Controller: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub ControlValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMidiControlChangeMessage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3085000579, data2: 30733, data3: 16479, data4: [183, 129, 62, 21, 152, 201, 127, 64] };
}
#[repr(C)]
pub struct IMidiControlChangeMessageFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateMidiControlChangeMessage: unsafe extern "system" fn(this: *mut *mut Self, channel: u8, controller: u8, controlvalue: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMidiControlChangeMessageFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 716260129, data2: 38252, data3: 18093, data4: [151, 82, 248, 127, 85, 5, 47, 227] };
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
impl ::windows_sys::core::Interface for IMidiInPort {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3586251227, data2: 38682, data3: 20143, data4: [162, 61, 234, 25, 254, 96, 127, 249] };
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
impl ::windows_sys::core::Interface for IMidiInPortStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1153710556, data2: 26623, data3: 19054, data4: [139, 172, 253, 182, 97, 12, 242, 150] };
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
impl ::windows_sys::core::Interface for IMidiMessage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2037807429, data2: 4244, data3: 17027, data4: [155, 224, 40, 159, 192, 238, 131, 52] };
}
#[repr(C)]
pub struct IMidiMessageReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Message: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMidiMessageReceivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1985375830, data2: 62248, data3: 19281, data4: [144, 125, 179, 168, 206, 150, 191, 128] };
}
#[repr(C)]
pub struct IMidiNoteOffMessage {
    pub base__: ::windows_sys::core::IInspectable,
    pub Channel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Note: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Velocity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMidiNoteOffMessage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 385714932, data2: 6542, data3: 19855, data4: [166, 84, 211, 5, 162, 147, 84, 143] };
}
#[repr(C)]
pub struct IMidiNoteOffMessageFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateMidiNoteOffMessage: unsafe extern "system" fn(this: *mut *mut Self, channel: u8, note: u8, velocity: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMidiNoteOffMessageFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2796699872, data2: 42825, data3: 16991, data4: [138, 244, 164, 217, 121, 204, 21, 181] };
}
#[repr(C)]
pub struct IMidiNoteOnMessage {
    pub base__: ::windows_sys::core::IInspectable,
    pub Channel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Note: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Velocity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMidiNoteOnMessage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3760343797, data2: 24961, data3: 18141, data4: [175, 162, 65, 0, 4, 192, 87, 170] };
}
#[repr(C)]
pub struct IMidiNoteOnMessageFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateMidiNoteOnMessage: unsafe extern "system" fn(this: *mut *mut Self, channel: u8, note: u8, velocity: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMidiNoteOnMessageFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2604826784, data2: 22977, data3: 16910, data4: [181, 23, 21, 161, 10, 169, 96, 107] };
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
impl ::windows_sys::core::Interface for IMidiOutPort {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2468179359, data2: 22434, data3: 19002, data4: [173, 184, 70, 64, 136, 111, 102, 147] };
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
impl ::windows_sys::core::Interface for IMidiOutPortStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 106742761, data2: 3976, data3: 17547, data4: [155, 100, 169, 88, 38, 198, 91, 143] };
}
#[repr(C)]
pub struct IMidiPitchBendChangeMessage {
    pub base__: ::windows_sys::core::IInspectable,
    pub Channel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Bend: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMidiPitchBendChangeMessage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 702500017, data2: 11935, data3: 20399, data4: [140, 43, 156, 184, 42, 144, 121, 202] };
}
#[repr(C)]
pub struct IMidiPitchBendChangeMessageFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateMidiPitchBendChangeMessage: unsafe extern "system" fn(this: *mut *mut Self, channel: u8, bend: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMidiPitchBendChangeMessageFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4126072661, data2: 53192, data3: 18726, data4: [179, 14, 163, 98, 35, 147, 48, 108] };
}
#[repr(C)]
pub struct IMidiPolyphonicKeyPressureMessage {
    pub base__: ::windows_sys::core::IInspectable,
    pub Channel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Note: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Pressure: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMidiPolyphonicKeyPressureMessage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 527644670, data2: 44264, data3: 18592, data4: [134, 142, 124, 219, 242, 15, 4, 214] };
}
#[repr(C)]
pub struct IMidiPolyphonicKeyPressureMessageFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateMidiPolyphonicKeyPressureMessage: unsafe extern "system" fn(this: *mut *mut Self, channel: u8, note: u8, pressure: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMidiPolyphonicKeyPressureMessageFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3918481470, data2: 50355, data3: 19922, data4: [145, 124, 227, 73, 129, 90, 27, 59] };
}
#[repr(C)]
pub struct IMidiProgramChangeMessage {
    pub base__: ::windows_sys::core::IInspectable,
    pub Channel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Program: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMidiProgramChangeMessage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2629516408, data2: 31294, data3: 17191, data4: [170, 152, 32, 184, 228, 72, 90, 248] };
}
#[repr(C)]
pub struct IMidiProgramChangeMessageFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateMidiProgramChangeMessage: unsafe extern "system" fn(this: *mut *mut Self, channel: u8, program: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMidiProgramChangeMessageFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3601875847, data2: 21067, data3: 16644, data4: [156, 153, 101, 114, 191, 210, 226, 97] };
}
#[repr(C)]
pub struct IMidiSongPositionPointerMessage {
    pub base__: ::windows_sys::core::IInspectable,
    pub Beats: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMidiSongPositionPointerMessage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1285885014, data2: 60510, data3: 19172, data4: [161, 21, 136, 220, 87, 204, 43, 121] };
}
#[repr(C)]
pub struct IMidiSongPositionPointerMessageFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateMidiSongPositionPointerMessage: unsafe extern "system" fn(this: *mut *mut Self, beats: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMidiSongPositionPointerMessageFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2617305494, data2: 61707, data3: 20458, data4: [179, 149, 245, 214, 207, 128, 246, 78] };
}
#[repr(C)]
pub struct IMidiSongSelectMessage {
    pub base__: ::windows_sys::core::IInspectable,
    pub Song: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMidiSongSelectMessage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1240527487, data2: 28035, data3: 18241, data4: [165, 191, 70, 41, 246, 190, 151, 79] };
}
#[repr(C)]
pub struct IMidiSongSelectMessageFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateMidiSongSelectMessage: unsafe extern "system" fn(this: *mut *mut Self, song: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMidiSongSelectMessageFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2223536356, data2: 34632, data3: 16681, data4: [166, 108, 160, 84, 147, 247, 93, 170] };
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
impl ::windows_sys::core::Interface for IMidiSynthesizer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4040824158, data2: 56208, data3: 16479, data4: [184, 174, 33, 210, 225, 127, 46, 69] };
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
impl ::windows_sys::core::Interface for IMidiSynthesizerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1109715624, data2: 26153, data3: 19819, data4: [170, 143, 212, 82, 26, 90, 49, 206] };
}
#[repr(C)]
pub struct IMidiSystemExclusiveMessageFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub CreateMidiSystemExclusiveMessage: unsafe extern "system" fn(this: *mut *mut Self, rawdata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateMidiSystemExclusiveMessage: usize,
}
impl ::windows_sys::core::Interface for IMidiSystemExclusiveMessageFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 138273314, data2: 15220, data3: 17184, data4: [155, 66, 12, 168, 84, 95, 138, 36] };
}
#[repr(C)]
pub struct IMidiTimeCodeMessage {
    pub base__: ::windows_sys::core::IInspectable,
    pub FrameType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Values: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMidiTimeCodeMessage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 200738941, data2: 64099, data3: 18972, data4: [141, 235, 192, 232, 119, 150, 166, 215] };
}
#[repr(C)]
pub struct IMidiTimeCodeMessageFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateMidiTimeCodeMessage: unsafe extern "system" fn(this: *mut *mut Self, frametype: u8, values: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMidiTimeCodeMessageFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3945830853, data2: 30492, data3: 16606, data4: [185, 97, 23, 90, 116, 137, 168, 94] };
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
