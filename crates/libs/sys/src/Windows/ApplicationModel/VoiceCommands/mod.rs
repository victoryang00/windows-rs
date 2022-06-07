#[repr(C)]
pub struct IVoiceCommand {
    pub base__: ::windows_sys::core::IInspectable,
    pub CommandName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    #[cfg(feature = "Media_SpeechRecognition")]
    pub SpeechRecognitionResult: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_SpeechRecognition"))]
    SpeechRecognitionResult: usize,
}
impl ::windows_sys::core::Interface for IVoiceCommand {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2473546355, data2: 60546, data3: 17062, data4: [165, 92, 210, 215, 158, 198, 249, 32] };
}
#[repr(C)]
pub struct IVoiceCommandCompletedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut VoiceCommandCompletionReason) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVoiceCommandCompletedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3361630045, data2: 65090, data3: 17196, data4: [153, 7, 9, 223, 159, 207, 100, 232] };
}
#[repr(C)]
pub struct IVoiceCommandConfirmationResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Confirmed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVoiceCommandConfirmationResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2686605630, data2: 33313, data3: 17702, data4: [176, 131, 132, 9, 114, 38, 34, 71] };
}
#[repr(C)]
pub struct IVoiceCommandContentTile {
    pub base__: ::windows_sys::core::IInspectable,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TextLine1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTextLine1: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TextLine2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTextLine2: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TextLine3: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTextLine3: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage")]
    pub Image: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    Image: usize,
    #[cfg(feature = "Storage")]
    pub SetImage: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SetImage: usize,
    pub AppContext: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAppContext: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AppLaunchArgument: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetAppLaunchArgument: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ContentTileType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut VoiceCommandContentTileType) -> ::windows_sys::core::HRESULT,
    pub SetContentTileType: unsafe extern "system" fn(this: *mut *mut Self, value: VoiceCommandContentTileType) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVoiceCommandContentTile {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1055910384, data2: 47303, data3: 19574, data4: [160, 222, 22, 7, 137, 94, 227, 39] };
}
#[repr(C)]
pub struct IVoiceCommandDefinition {
    pub base__: ::windows_sys::core::IInspectable,
    pub Language: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SetPhraseListAsync: unsafe extern "system" fn(this: *mut *mut Self, phraselistname: ::windows_sys::core::HSTRING, phraselist: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetPhraseListAsync: usize,
}
impl ::windows_sys::core::Interface for IVoiceCommandDefinition {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2037557968, data2: 2420, data3: 18809, data4: [152, 75, 203, 137, 89, 205, 97, 174] };
}
#[repr(C)]
pub struct IVoiceCommandDefinitionManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub InstallCommandDefinitionsFromStorageFileAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    InstallCommandDefinitionsFromStorageFileAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub InstalledCommandDefinitions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InstalledCommandDefinitions: usize,
}
impl ::windows_sys::core::Interface for IVoiceCommandDefinitionManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2414323358, data2: 1662, data3: 20246, data4: [161, 140, 91, 23, 233, 73, 153, 64] };
}
#[repr(C)]
pub struct IVoiceCommandDisambiguationResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectedItem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVoiceCommandDisambiguationResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3972435198, data2: 51628, data3: 17887, data4: [168, 234, 254, 234, 8, 239, 156, 94] };
}
#[repr(C)]
pub struct IVoiceCommandResponse {
    pub base__: ::windows_sys::core::IInspectable,
    pub Message: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetMessage: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RepeatMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRepeatMessage: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AppLaunchArgument: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetAppLaunchArgument: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub VoiceCommandContentTiles: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    VoiceCommandContentTiles: usize,
}
impl ::windows_sys::core::Interface for IVoiceCommandResponse {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 42251022, data2: 35387, data3: 19652, data4: [166, 161, 202, 213, 190, 39, 22, 181] };
}
#[repr(C)]
pub struct IVoiceCommandResponseStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxSupportedVoiceCommandContentTiles: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub CreateResponse: unsafe extern "system" fn(this: *mut *mut Self, usermessage: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateResponseWithTiles: unsafe extern "system" fn(this: *mut *mut Self, message: *mut ::core::ffi::c_void, contenttiles: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateResponseWithTiles: usize,
    pub CreateResponseForPrompt: unsafe extern "system" fn(this: *mut *mut Self, message: *mut ::core::ffi::c_void, repeatmessage: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateResponseForPromptWithTiles: unsafe extern "system" fn(this: *mut *mut Self, message: *mut ::core::ffi::c_void, repeatmessage: *mut ::core::ffi::c_void, contenttiles: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateResponseForPromptWithTiles: usize,
}
impl ::windows_sys::core::Interface for IVoiceCommandResponseStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 691206163, data2: 3387, data3: 18930, data4: [150, 221, 98, 80, 25, 189, 59, 93] };
}
#[repr(C)]
pub struct IVoiceCommandServiceConnection {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetVoiceCommandAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetVoiceCommandAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestConfirmationAsync: unsafe extern "system" fn(this: *mut *mut Self, response: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestConfirmationAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestDisambiguationAsync: unsafe extern "system" fn(this: *mut *mut Self, response: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestDisambiguationAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportProgressAsync: unsafe extern "system" fn(this: *mut *mut Self, response: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportProgressAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportSuccessAsync: unsafe extern "system" fn(this: *mut *mut Self, response: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportSuccessAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailureAsync: unsafe extern "system" fn(this: *mut *mut Self, response: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailureAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAppLaunchAsync: unsafe extern "system" fn(this: *mut *mut Self, response: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAppLaunchAsync: usize,
    #[cfg(feature = "Globalization")]
    pub Language: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    Language: usize,
    #[cfg(feature = "Foundation")]
    pub VoiceCommandCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VoiceCommandCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVoiceCommandCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVoiceCommandCompleted: usize,
}
impl ::windows_sys::core::Interface for IVoiceCommandServiceConnection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3633626015, data2: 8666, data3: 17572, data4: [152, 162, 251, 19, 25, 32, 169, 204] };
}
#[repr(C)]
pub struct IVoiceCommandServiceConnectionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_AppService")]
    pub FromAppServiceTriggerDetails: unsafe extern "system" fn(this: *mut *mut Self, triggerdetails: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_AppService"))]
    FromAppServiceTriggerDetails: usize,
}
impl ::windows_sys::core::Interface for IVoiceCommandServiceConnectionStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 923713531, data2: 11572, data3: 17119, data4: [135, 112, 7, 77, 15, 51, 70, 151] };
}
#[repr(C)]
pub struct IVoiceCommandUserMessage {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDisplayMessage: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SpokenMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSpokenMessage: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVoiceCommandUserMessage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1733211072, data2: 17654, data3: 20231, data4: [185, 121, 76, 114, 63, 192, 133, 151] };
}
pub type VoiceCommand = *mut ::core::ffi::c_void;
pub type VoiceCommandCompletedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_VoiceCommands\"`*"]
#[repr(transparent)]
pub struct VoiceCommandCompletionReason(pub i32);
impl VoiceCommandCompletionReason {
    pub const Unknown: Self = Self(0i32);
    pub const CommunicationFailed: Self = Self(1i32);
    pub const ResourceLimitsExceeded: Self = Self(2i32);
    pub const Canceled: Self = Self(3i32);
    pub const TimeoutExceeded: Self = Self(4i32);
    pub const AppLaunched: Self = Self(5i32);
    pub const Completed: Self = Self(6i32);
}
impl ::core::marker::Copy for VoiceCommandCompletionReason {}
impl ::core::clone::Clone for VoiceCommandCompletionReason {
    fn clone(&self) -> Self {
        *self
    }
}
pub type VoiceCommandConfirmationResult = *mut ::core::ffi::c_void;
pub type VoiceCommandContentTile = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_VoiceCommands\"`*"]
#[repr(transparent)]
pub struct VoiceCommandContentTileType(pub i32);
impl VoiceCommandContentTileType {
    pub const TitleOnly: Self = Self(0i32);
    pub const TitleWithText: Self = Self(1i32);
    pub const TitleWith68x68Icon: Self = Self(2i32);
    pub const TitleWith68x68IconAndText: Self = Self(3i32);
    pub const TitleWith68x92Icon: Self = Self(4i32);
    pub const TitleWith68x92IconAndText: Self = Self(5i32);
    pub const TitleWith280x140Icon: Self = Self(6i32);
    pub const TitleWith280x140IconAndText: Self = Self(7i32);
}
impl ::core::marker::Copy for VoiceCommandContentTileType {}
impl ::core::clone::Clone for VoiceCommandContentTileType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type VoiceCommandDefinition = *mut ::core::ffi::c_void;
pub type VoiceCommandDisambiguationResult = *mut ::core::ffi::c_void;
pub type VoiceCommandResponse = *mut ::core::ffi::c_void;
pub type VoiceCommandServiceConnection = *mut ::core::ffi::c_void;
pub type VoiceCommandUserMessage = *mut ::core::ffi::c_void;
