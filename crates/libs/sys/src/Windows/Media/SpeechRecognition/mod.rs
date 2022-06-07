#[repr(C)]
pub struct ISpeechContinuousRecognitionCompletedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpeechRecognitionResultStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpeechContinuousRecognitionCompletedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3822086587, data2: 58124, data3: 24088, data4: [66, 75, 127, 190, 129, 248, 251, 208] };
}
#[repr(C)]
pub struct ISpeechContinuousRecognitionResultGeneratedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Result: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpeechContinuousRecognitionResultGeneratedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 420027934, data2: 28286, data3: 23110, data4: [64, 251, 118, 89, 79, 120, 101, 4] };
}
#[repr(C)]
pub struct ISpeechContinuousRecognitionSession {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub AutoStopSilenceTimeout: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AutoStopSilenceTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub SetAutoStopSilenceTimeout: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAutoStopSilenceTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub StartAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StartWithModeAsync: unsafe extern "system" fn(this: *mut *mut Self, mode: SpeechContinuousRecognitionMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartWithModeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CancelAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CancelAsync: usize,
    #[cfg(feature = "Foundation")]
    pub PauseAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PauseAsync: usize,
    pub Resume: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Completed: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Completed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCompleted: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub ResultGenerated: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResultGenerated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveResultGenerated: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveResultGenerated: usize,
}
impl ::windows_sys::core::Interface for ISpeechContinuousRecognitionSession {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1780562948, data2: 26132, data3: 18936, data4: [153, 162, 181, 233, 179, 160, 133, 200] };
}
#[repr(C)]
pub struct ISpeechRecognitionCompilationResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpeechRecognitionResultStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpeechRecognitionCompilationResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1082027101, data2: 27335, data3: 19876, data4: [156, 193, 47, 206, 50, 207, 116, 137] };
}
#[repr(C)]
pub struct ISpeechRecognitionConstraint {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Tag: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpeechRecognitionConstraintType) -> ::windows_sys::core::HRESULT,
    pub Probability: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpeechRecognitionConstraintProbability) -> ::windows_sys::core::HRESULT,
    pub SetProbability: unsafe extern "system" fn(this: *mut *mut Self, value: SpeechRecognitionConstraintProbability) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpeechRecognitionConstraint {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2041321000, data2: 19816, data3: 17348, data4: [137, 17, 64, 220, 65, 1, 181, 91] };
}
#[repr(C)]
pub struct ISpeechRecognitionGrammarFileConstraint {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage")]
    pub GrammarFile: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    GrammarFile: usize,
}
impl ::windows_sys::core::Interface for ISpeechRecognitionGrammarFileConstraint {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3036879503, data2: 34250, data3: 20388, data4: [177, 26, 71, 79, 196, 27, 56, 53] };
}
#[repr(C)]
pub struct ISpeechRecognitionGrammarFileConstraintFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    Create: usize,
    #[cfg(feature = "Storage")]
    pub CreateWithTag: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, tag: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    CreateWithTag: usize,
}
impl ::windows_sys::core::Interface for ISpeechRecognitionGrammarFileConstraintFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1034383595, data2: 50297, data3: 19495, data4: [159, 25, 137, 151, 78, 243, 146, 209] };
}
#[repr(C)]
pub struct ISpeechRecognitionHypothesis {
    pub base__: ::windows_sys::core::IInspectable,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpeechRecognitionHypothesis {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2054890928, data2: 39365, data3: 20349, data4: [191, 132, 16, 170, 19, 2, 182, 52] };
}
#[repr(C)]
pub struct ISpeechRecognitionHypothesisGeneratedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Hypothesis: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpeechRecognitionHypothesisGeneratedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1427511930, data2: 32803, data3: 22630, data4: [65, 29, 18, 19, 187, 39, 20, 118] };
}
#[repr(C)]
pub struct ISpeechRecognitionListConstraint {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Commands: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Commands: usize,
}
impl ::windows_sys::core::Interface for ISpeechRecognitionListConstraint {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 163874793, data2: 58541, data3: 17702, data4: [129, 242, 73, 70, 251, 72, 29, 152] };
}
#[repr(C)]
pub struct ISpeechRecognitionListConstraintFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, commands: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithTag: unsafe extern "system" fn(this: *mut *mut Self, commands: *mut ::core::ffi::c_void, tag: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithTag: usize,
}
impl ::windows_sys::core::Interface for ISpeechRecognitionListConstraintFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1089719751, data2: 22058, data3: 17002, data4: [159, 59, 59, 78, 40, 43, 225, 213] };
}
#[repr(C)]
pub struct ISpeechRecognitionQualityDegradingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Problem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpeechRecognitionAudioProblem) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpeechRecognitionQualityDegradingEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1340227845, data2: 35898, data3: 19582, data4: [141, 10, 91, 212, 245, 177, 74, 216] };
}
#[repr(C)]
pub struct ISpeechRecognitionResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpeechRecognitionResultStatus) -> ::windows_sys::core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Confidence: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpeechRecognitionConfidence) -> ::windows_sys::core::HRESULT,
    pub SemanticInterpretation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAlternates: unsafe extern "system" fn(this: *mut *mut Self, maxalternates: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAlternates: usize,
    pub Constraint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RulePath: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RulePath: usize,
    pub RawConfidence: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpeechRecognitionResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1311781207, data2: 846, data3: 18002, data4: [133, 126, 208, 69, 76, 196, 190, 236] };
}
#[repr(C)]
pub struct ISpeechRecognitionResult2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PhraseStartTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PhraseStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub PhraseDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PhraseDuration: usize,
}
impl ::windows_sys::core::Interface for ISpeechRecognitionResult2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2944324026, data2: 17691, data3: 16742, data4: [160, 193, 31, 254, 132, 3, 45, 3] };
}
#[repr(C)]
pub struct ISpeechRecognitionSemanticInterpretation {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
impl ::windows_sys::core::Interface for ISpeechRecognitionSemanticInterpretation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2866928283, data2: 32306, data3: 19487, data4: [137, 254, 12, 101, 244, 134, 245, 46] };
}
#[repr(C)]
pub struct ISpeechRecognitionTopicConstraint {
    pub base__: ::windows_sys::core::IInspectable,
    pub Scenario: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpeechRecognitionScenario) -> ::windows_sys::core::HRESULT,
    pub TopicHint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpeechRecognitionTopicConstraint {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3211779865, data2: 33373, data3: 20073, data4: [166, 129, 54, 228, 140, 241, 201, 62] };
}
#[repr(C)]
pub struct ISpeechRecognitionTopicConstraintFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, scenario: SpeechRecognitionScenario, topichint: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithTag: unsafe extern "system" fn(this: *mut *mut Self, scenario: SpeechRecognitionScenario, topichint: ::windows_sys::core::HSTRING, tag: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpeechRecognitionTopicConstraintFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1852335071, data2: 60421, data3: 18391, data4: [165, 223, 86, 163, 67, 30, 88, 210] };
}
#[repr(C)]
pub struct ISpeechRecognitionVoiceCommandDefinitionConstraint {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ISpeechRecognitionVoiceCommandDefinitionConstraint {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4068023339, data2: 7924, data3: 19175, data4: [157, 119, 182, 255, 16, 184, 163, 194] };
}
#[repr(C)]
pub struct ISpeechRecognizer {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Globalization")]
    pub CurrentLanguage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    CurrentLanguage: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Constraints: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Constraints: usize,
    pub Timeouts: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UIOptions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CompileConstraintsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CompileConstraintsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RecognizeAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RecognizeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RecognizeWithUIAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RecognizeWithUIAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RecognitionQualityDegrading: unsafe extern "system" fn(this: *mut *mut Self, speechrecognitionqualitydegradinghandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RecognitionQualityDegrading: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRecognitionQualityDegrading: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRecognitionQualityDegrading: usize,
    #[cfg(feature = "Foundation")]
    pub StateChanged: unsafe extern "system" fn(this: *mut *mut Self, statechangedhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStateChanged: usize,
}
impl ::windows_sys::core::Interface for ISpeechRecognizer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 197380555, data2: 49770, data3: 16626, data4: [174, 181, 128, 150, 178, 228, 128, 115] };
}
#[repr(C)]
pub struct ISpeechRecognizer2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContinuousRecognitionSession: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpeechRecognizerState) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StopRecognitionAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopRecognitionAsync: usize,
    #[cfg(feature = "Foundation")]
    pub HypothesisGenerated: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HypothesisGenerated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHypothesisGenerated: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHypothesisGenerated: usize,
}
impl ::windows_sys::core::Interface for ISpeechRecognizer2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1674164977, data2: 37347, data3: 20132, data4: [134, 161, 124, 56, 103, 208, 132, 166] };
}
#[repr(C)]
pub struct ISpeechRecognizerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Globalization")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, language: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for ISpeechRecognizerFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1623492829, data2: 32696, data3: 16435, data4: [172, 112, 208, 70, 246, 72, 24, 225] };
}
#[repr(C)]
pub struct ISpeechRecognizerStateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpeechRecognizerState) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpeechRecognizerStateChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1446858505, data2: 47619, data3: 19373, data4: [173, 129, 221, 198, 196, 218, 176, 195] };
}
#[repr(C)]
pub struct ISpeechRecognizerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Globalization")]
    pub SystemSpeechLanguage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    SystemSpeechLanguage: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    pub SupportedTopicLanguages: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Globalization")))]
    SupportedTopicLanguages: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    pub SupportedGrammarLanguages: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Globalization")))]
    SupportedGrammarLanguages: usize,
}
impl ::windows_sys::core::Interface for ISpeechRecognizerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2275630764, data2: 42972, data3: 19211, data4: [188, 201, 36, 244, 124, 11, 126, 191] };
}
#[repr(C)]
pub struct ISpeechRecognizerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Globalization"))]
    pub TrySetSystemSpeechLanguageAsync: unsafe extern "system" fn(this: *mut *mut Self, speechlanguage: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Globalization")))]
    TrySetSystemSpeechLanguageAsync: usize,
}
impl ::windows_sys::core::Interface for ISpeechRecognizerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 488312213, data2: 30053, data3: 20217, data4: [162, 243, 186, 21, 22, 42, 150, 207] };
}
#[repr(C)]
pub struct ISpeechRecognizerTimeouts {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub InitialSilenceTimeout: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InitialSilenceTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub SetInitialSilenceTimeout: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetInitialSilenceTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub EndSilenceTimeout: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndSilenceTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub SetEndSilenceTimeout: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetEndSilenceTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub BabbleTimeout: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BabbleTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub SetBabbleTimeout: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBabbleTimeout: usize,
}
impl ::windows_sys::core::Interface for ISpeechRecognizerTimeouts {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 787967946, data2: 27196, data3: 19914, data4: [161, 83, 223, 27, 200, 138, 121, 175] };
}
#[repr(C)]
pub struct ISpeechRecognizerUIOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExampleText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetExampleText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AudiblePrompt: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetAudiblePrompt: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsReadBackEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsReadBackEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ShowConfirmation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetShowConfirmation: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpeechRecognizerUIOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2022233665, data2: 47403, data3: 17594, data4: [162, 95, 209, 134, 70, 48, 100, 31] };
}
#[repr(C)]
pub struct IVoiceCommandManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub InstallCommandSetsFromStorageFileAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    InstallCommandSetsFromStorageFileAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub InstalledCommandSets: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InstalledCommandSets: usize,
}
impl ::windows_sys::core::Interface for IVoiceCommandManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2855964117, data2: 46823, data3: 20194, data4: [186, 169, 221, 107, 172, 237, 10, 43] };
}
#[repr(C)]
pub struct IVoiceCommandSet {
    pub base__: ::windows_sys::core::IInspectable,
    pub Language: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SetPhraseListAsync: unsafe extern "system" fn(this: *mut *mut Self, phraselistname: ::windows_sys::core::HSTRING, phraselist: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetPhraseListAsync: usize,
}
impl ::windows_sys::core::Interface for IVoiceCommandSet {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 200137333, data2: 18150, data3: 19217, data4: [160, 136, 92, 104, 99, 40, 153, 181] };
}
pub type SpeechContinuousRecognitionCompletedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
#[repr(transparent)]
pub struct SpeechContinuousRecognitionMode(pub i32);
impl SpeechContinuousRecognitionMode {
    pub const Default: Self = Self(0i32);
    pub const PauseOnRecognition: Self = Self(1i32);
}
impl ::core::marker::Copy for SpeechContinuousRecognitionMode {}
impl ::core::clone::Clone for SpeechContinuousRecognitionMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SpeechContinuousRecognitionResultGeneratedEventArgs = *mut ::core::ffi::c_void;
pub type SpeechContinuousRecognitionSession = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
#[repr(transparent)]
pub struct SpeechRecognitionAudioProblem(pub i32);
impl SpeechRecognitionAudioProblem {
    pub const None: Self = Self(0i32);
    pub const TooNoisy: Self = Self(1i32);
    pub const NoSignal: Self = Self(2i32);
    pub const TooLoud: Self = Self(3i32);
    pub const TooQuiet: Self = Self(4i32);
    pub const TooFast: Self = Self(5i32);
    pub const TooSlow: Self = Self(6i32);
}
impl ::core::marker::Copy for SpeechRecognitionAudioProblem {}
impl ::core::clone::Clone for SpeechRecognitionAudioProblem {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SpeechRecognitionCompilationResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
#[repr(transparent)]
pub struct SpeechRecognitionConfidence(pub i32);
impl SpeechRecognitionConfidence {
    pub const High: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const Low: Self = Self(2i32);
    pub const Rejected: Self = Self(3i32);
}
impl ::core::marker::Copy for SpeechRecognitionConfidence {}
impl ::core::clone::Clone for SpeechRecognitionConfidence {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
#[repr(transparent)]
pub struct SpeechRecognitionConstraintProbability(pub i32);
impl SpeechRecognitionConstraintProbability {
    pub const Default: Self = Self(0i32);
    pub const Min: Self = Self(1i32);
    pub const Max: Self = Self(2i32);
}
impl ::core::marker::Copy for SpeechRecognitionConstraintProbability {}
impl ::core::clone::Clone for SpeechRecognitionConstraintProbability {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
#[repr(transparent)]
pub struct SpeechRecognitionConstraintType(pub i32);
impl SpeechRecognitionConstraintType {
    pub const Topic: Self = Self(0i32);
    pub const List: Self = Self(1i32);
    pub const Grammar: Self = Self(2i32);
    pub const VoiceCommandDefinition: Self = Self(3i32);
}
impl ::core::marker::Copy for SpeechRecognitionConstraintType {}
impl ::core::clone::Clone for SpeechRecognitionConstraintType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SpeechRecognitionGrammarFileConstraint = *mut ::core::ffi::c_void;
pub type SpeechRecognitionHypothesis = *mut ::core::ffi::c_void;
pub type SpeechRecognitionHypothesisGeneratedEventArgs = *mut ::core::ffi::c_void;
pub type SpeechRecognitionListConstraint = *mut ::core::ffi::c_void;
pub type SpeechRecognitionQualityDegradingEventArgs = *mut ::core::ffi::c_void;
pub type SpeechRecognitionResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
#[repr(transparent)]
pub struct SpeechRecognitionResultStatus(pub i32);
impl SpeechRecognitionResultStatus {
    pub const Success: Self = Self(0i32);
    pub const TopicLanguageNotSupported: Self = Self(1i32);
    pub const GrammarLanguageMismatch: Self = Self(2i32);
    pub const GrammarCompilationFailure: Self = Self(3i32);
    pub const AudioQualityFailure: Self = Self(4i32);
    pub const UserCanceled: Self = Self(5i32);
    pub const Unknown: Self = Self(6i32);
    pub const TimeoutExceeded: Self = Self(7i32);
    pub const PauseLimitExceeded: Self = Self(8i32);
    pub const NetworkFailure: Self = Self(9i32);
    pub const MicrophoneUnavailable: Self = Self(10i32);
}
impl ::core::marker::Copy for SpeechRecognitionResultStatus {}
impl ::core::clone::Clone for SpeechRecognitionResultStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
#[repr(transparent)]
pub struct SpeechRecognitionScenario(pub i32);
impl SpeechRecognitionScenario {
    pub const WebSearch: Self = Self(0i32);
    pub const Dictation: Self = Self(1i32);
    pub const FormFilling: Self = Self(2i32);
}
impl ::core::marker::Copy for SpeechRecognitionScenario {}
impl ::core::clone::Clone for SpeechRecognitionScenario {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SpeechRecognitionSemanticInterpretation = *mut ::core::ffi::c_void;
pub type SpeechRecognitionTopicConstraint = *mut ::core::ffi::c_void;
pub type SpeechRecognitionVoiceCommandDefinitionConstraint = *mut ::core::ffi::c_void;
pub type SpeechRecognizer = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
#[repr(transparent)]
pub struct SpeechRecognizerState(pub i32);
impl SpeechRecognizerState {
    pub const Idle: Self = Self(0i32);
    pub const Capturing: Self = Self(1i32);
    pub const Processing: Self = Self(2i32);
    pub const SoundStarted: Self = Self(3i32);
    pub const SoundEnded: Self = Self(4i32);
    pub const SpeechDetected: Self = Self(5i32);
    pub const Paused: Self = Self(6i32);
}
impl ::core::marker::Copy for SpeechRecognizerState {}
impl ::core::clone::Clone for SpeechRecognizerState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SpeechRecognizerStateChangedEventArgs = *mut ::core::ffi::c_void;
pub type SpeechRecognizerTimeouts = *mut ::core::ffi::c_void;
pub type SpeechRecognizerUIOptions = *mut ::core::ffi::c_void;
pub type VoiceCommandSet = *mut ::core::ffi::c_void;
