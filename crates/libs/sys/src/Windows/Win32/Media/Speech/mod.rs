#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DEFAULT_WEIGHT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPIDSPRG = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGId: DISPIDSPRG = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGRecoContext: DISPIDSPRG = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGState: DISPIDSPRG = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGRules: DISPIDSPRG = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGReset: DISPIDSPRG = 5i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGCommit: DISPIDSPRG = 6i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGCmdLoadFromFile: DISPIDSPRG = 7i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGCmdLoadFromObject: DISPIDSPRG = 8i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGCmdLoadFromResource: DISPIDSPRG = 9i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGCmdLoadFromMemory: DISPIDSPRG = 10i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGCmdLoadFromProprietaryGrammar: DISPIDSPRG = 11i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGCmdSetRuleState: DISPIDSPRG = 12i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGCmdSetRuleIdState: DISPIDSPRG = 13i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGDictationLoad: DISPIDSPRG = 14i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGDictationUnload: DISPIDSPRG = 15i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGDictationSetState: DISPIDSPRG = 16i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGSetWordSequenceData: DISPIDSPRG = 17i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGSetTextSelection: DISPIDSPRG = 18i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGIsPronounceable: DISPIDSPRG = 19i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPIDSPTSI = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPIDSPTSI_ActiveOffset: DISPIDSPTSI = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPIDSPTSI_ActiveLength: DISPIDSPTSI = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPIDSPTSI_SelectionOffset: DISPIDSPTSI = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPIDSPTSI_SelectionLength: DISPIDSPTSI = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechAudio = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SAStatus: DISPID_SpeechAudio = 200i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SABufferInfo: DISPID_SpeechAudio = 201i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SADefaultFormat: DISPID_SpeechAudio = 202i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SAVolume: DISPID_SpeechAudio = 203i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SABufferNotifySize: DISPID_SpeechAudio = 204i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SAEventHandle: DISPID_SpeechAudio = 205i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SASetState: DISPID_SpeechAudio = 206i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechAudioBufferInfo = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SABIMinNotification: DISPID_SpeechAudioBufferInfo = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SABIBufferSize: DISPID_SpeechAudioBufferInfo = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SABIEventBias: DISPID_SpeechAudioBufferInfo = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechAudioFormat = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SAFType: DISPID_SpeechAudioFormat = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SAFGuid: DISPID_SpeechAudioFormat = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SAFGetWaveFormatEx: DISPID_SpeechAudioFormat = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SAFSetWaveFormatEx: DISPID_SpeechAudioFormat = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechAudioStatus = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SASFreeBufferSpace: DISPID_SpeechAudioStatus = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SASNonBlockingIO: DISPID_SpeechAudioStatus = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SASState: DISPID_SpeechAudioStatus = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SASCurrentSeekPosition: DISPID_SpeechAudioStatus = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SASCurrentDevicePosition: DISPID_SpeechAudioStatus = 5i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechBaseStream = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SBSFormat: DISPID_SpeechBaseStream = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SBSRead: DISPID_SpeechBaseStream = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SBSWrite: DISPID_SpeechBaseStream = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SBSSeek: DISPID_SpeechBaseStream = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechCustomStream = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SCSBaseStream: DISPID_SpeechCustomStream = 100i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechDataKey = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SDKSetBinaryValue: DISPID_SpeechDataKey = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SDKGetBinaryValue: DISPID_SpeechDataKey = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SDKSetStringValue: DISPID_SpeechDataKey = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SDKGetStringValue: DISPID_SpeechDataKey = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SDKSetLongValue: DISPID_SpeechDataKey = 5i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SDKGetlongValue: DISPID_SpeechDataKey = 6i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SDKOpenKey: DISPID_SpeechDataKey = 7i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SDKCreateKey: DISPID_SpeechDataKey = 8i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SDKDeleteKey: DISPID_SpeechDataKey = 9i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SDKDeleteValue: DISPID_SpeechDataKey = 10i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SDKEnumKeys: DISPID_SpeechDataKey = 11i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SDKEnumValues: DISPID_SpeechDataKey = 12i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechFileStream = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SFSOpen: DISPID_SpeechFileStream = 100i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SFSClose: DISPID_SpeechFileStream = 101i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechGrammarRule = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRAttributes: DISPID_SpeechGrammarRule = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRInitialState: DISPID_SpeechGrammarRule = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRName: DISPID_SpeechGrammarRule = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRId: DISPID_SpeechGrammarRule = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRClear: DISPID_SpeechGrammarRule = 5i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRAddResource: DISPID_SpeechGrammarRule = 6i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRAddState: DISPID_SpeechGrammarRule = 7i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechGrammarRuleState = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRSRule: DISPID_SpeechGrammarRuleState = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRSTransitions: DISPID_SpeechGrammarRuleState = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRSAddWordTransition: DISPID_SpeechGrammarRuleState = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRSAddRuleTransition: DISPID_SpeechGrammarRuleState = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRSAddSpecialTransition: DISPID_SpeechGrammarRuleState = 5i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechGrammarRuleStateTransition = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRSTType: DISPID_SpeechGrammarRuleStateTransition = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRSTText: DISPID_SpeechGrammarRuleStateTransition = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRSTRule: DISPID_SpeechGrammarRuleStateTransition = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRSTWeight: DISPID_SpeechGrammarRuleStateTransition = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRSTPropertyName: DISPID_SpeechGrammarRuleStateTransition = 5i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRSTPropertyId: DISPID_SpeechGrammarRuleStateTransition = 6i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRSTPropertyValue: DISPID_SpeechGrammarRuleStateTransition = 7i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRSTNextState: DISPID_SpeechGrammarRuleStateTransition = 8i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechGrammarRuleStateTransitions = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRSTsCount: DISPID_SpeechGrammarRuleStateTransitions = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRSTsItem: DISPID_SpeechGrammarRuleStateTransitions = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRSTs_NewEnum: DISPID_SpeechGrammarRuleStateTransitions = -4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechGrammarRules = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRsCount: DISPID_SpeechGrammarRules = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRsDynamic: DISPID_SpeechGrammarRules = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRsAdd: DISPID_SpeechGrammarRules = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRsCommit: DISPID_SpeechGrammarRules = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRsCommitAndSave: DISPID_SpeechGrammarRules = 5i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRsFindRule: DISPID_SpeechGrammarRules = 6i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRsItem: DISPID_SpeechGrammarRules = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRs_NewEnum: DISPID_SpeechGrammarRules = -4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechLexicon = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLGenerationId: DISPID_SpeechLexicon = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLGetWords: DISPID_SpeechLexicon = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLAddPronunciation: DISPID_SpeechLexicon = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLAddPronunciationByPhoneIds: DISPID_SpeechLexicon = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLRemovePronunciation: DISPID_SpeechLexicon = 5i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLRemovePronunciationByPhoneIds: DISPID_SpeechLexicon = 6i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLGetPronunciations: DISPID_SpeechLexicon = 7i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLGetGenerationChange: DISPID_SpeechLexicon = 8i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechLexiconProns = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLPsCount: DISPID_SpeechLexiconProns = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLPsItem: DISPID_SpeechLexiconProns = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLPs_NewEnum: DISPID_SpeechLexiconProns = -4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechLexiconPronunciation = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLPType: DISPID_SpeechLexiconPronunciation = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLPLangId: DISPID_SpeechLexiconPronunciation = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLPPartOfSpeech: DISPID_SpeechLexiconPronunciation = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLPPhoneIds: DISPID_SpeechLexiconPronunciation = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLPSymbolic: DISPID_SpeechLexiconPronunciation = 5i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechLexiconWord = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLWLangId: DISPID_SpeechLexiconWord = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLWType: DISPID_SpeechLexiconWord = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLWWord: DISPID_SpeechLexiconWord = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLWPronunciations: DISPID_SpeechLexiconWord = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechLexiconWords = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLWsCount: DISPID_SpeechLexiconWords = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLWsItem: DISPID_SpeechLexiconWords = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLWs_NewEnum: DISPID_SpeechLexiconWords = -4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechMMSysAudio = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SMSADeviceId: DISPID_SpeechMMSysAudio = 300i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SMSALineId: DISPID_SpeechMMSysAudio = 301i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SMSAMMHandle: DISPID_SpeechMMSysAudio = 302i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechMemoryStream = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SMSSetData: DISPID_SpeechMemoryStream = 100i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SMSGetData: DISPID_SpeechMemoryStream = 101i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechObjectToken = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTId: DISPID_SpeechObjectToken = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTDataKey: DISPID_SpeechObjectToken = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTCategory: DISPID_SpeechObjectToken = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTGetDescription: DISPID_SpeechObjectToken = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTSetId: DISPID_SpeechObjectToken = 5i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTGetAttribute: DISPID_SpeechObjectToken = 6i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTCreateInstance: DISPID_SpeechObjectToken = 7i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTRemove: DISPID_SpeechObjectToken = 8i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTGetStorageFileName: DISPID_SpeechObjectToken = 9i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTRemoveStorageFileName: DISPID_SpeechObjectToken = 10i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTIsUISupported: DISPID_SpeechObjectToken = 11i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTDisplayUI: DISPID_SpeechObjectToken = 12i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTMatchesAttributes: DISPID_SpeechObjectToken = 13i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechObjectTokenCategory = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTCId: DISPID_SpeechObjectTokenCategory = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTCDefault: DISPID_SpeechObjectTokenCategory = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTCSetId: DISPID_SpeechObjectTokenCategory = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTCGetDataKey: DISPID_SpeechObjectTokenCategory = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTCEnumerateTokens: DISPID_SpeechObjectTokenCategory = 5i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechObjectTokens = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTsCount: DISPID_SpeechObjectTokens = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTsItem: DISPID_SpeechObjectTokens = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTs_NewEnum: DISPID_SpeechObjectTokens = -4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechPhoneConverter = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPCLangId: DISPID_SpeechPhoneConverter = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPCPhoneToId: DISPID_SpeechPhoneConverter = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPCIdToPhone: DISPID_SpeechPhoneConverter = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechPhraseAlternate = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPARecoResult: DISPID_SpeechPhraseAlternate = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPAStartElementInResult: DISPID_SpeechPhraseAlternate = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPANumberOfElementsInResult: DISPID_SpeechPhraseAlternate = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPAPhraseInfo: DISPID_SpeechPhraseAlternate = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPACommit: DISPID_SpeechPhraseAlternate = 5i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechPhraseAlternates = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPAsCount: DISPID_SpeechPhraseAlternates = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPAsItem: DISPID_SpeechPhraseAlternates = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPAs_NewEnum: DISPID_SpeechPhraseAlternates = -4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechPhraseBuilder = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPPBRestorePhraseFromMemory: DISPID_SpeechPhraseBuilder = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechPhraseElement = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPEAudioTimeOffset: DISPID_SpeechPhraseElement = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPEAudioSizeTime: DISPID_SpeechPhraseElement = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPEAudioStreamOffset: DISPID_SpeechPhraseElement = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPEAudioSizeBytes: DISPID_SpeechPhraseElement = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPERetainedStreamOffset: DISPID_SpeechPhraseElement = 5i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPERetainedSizeBytes: DISPID_SpeechPhraseElement = 6i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPEDisplayText: DISPID_SpeechPhraseElement = 7i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPELexicalForm: DISPID_SpeechPhraseElement = 8i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPEPronunciation: DISPID_SpeechPhraseElement = 9i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPEDisplayAttributes: DISPID_SpeechPhraseElement = 10i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPERequiredConfidence: DISPID_SpeechPhraseElement = 11i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPEActualConfidence: DISPID_SpeechPhraseElement = 12i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPEEngineConfidence: DISPID_SpeechPhraseElement = 13i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechPhraseElements = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPEsCount: DISPID_SpeechPhraseElements = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPEsItem: DISPID_SpeechPhraseElements = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPEs_NewEnum: DISPID_SpeechPhraseElements = -4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechPhraseInfo = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPILanguageId: DISPID_SpeechPhraseInfo = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPIGrammarId: DISPID_SpeechPhraseInfo = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPIStartTime: DISPID_SpeechPhraseInfo = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPIAudioStreamPosition: DISPID_SpeechPhraseInfo = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPIAudioSizeBytes: DISPID_SpeechPhraseInfo = 5i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPIRetainedSizeBytes: DISPID_SpeechPhraseInfo = 6i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPIAudioSizeTime: DISPID_SpeechPhraseInfo = 7i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPIRule: DISPID_SpeechPhraseInfo = 8i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPIProperties: DISPID_SpeechPhraseInfo = 9i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPIElements: DISPID_SpeechPhraseInfo = 10i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPIReplacements: DISPID_SpeechPhraseInfo = 11i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPIEngineId: DISPID_SpeechPhraseInfo = 12i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPIEnginePrivateData: DISPID_SpeechPhraseInfo = 13i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPISaveToMemory: DISPID_SpeechPhraseInfo = 14i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPIGetText: DISPID_SpeechPhraseInfo = 15i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPIGetDisplayAttributes: DISPID_SpeechPhraseInfo = 16i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechPhraseProperties = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPPsCount: DISPID_SpeechPhraseProperties = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPPsItem: DISPID_SpeechPhraseProperties = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPPs_NewEnum: DISPID_SpeechPhraseProperties = -4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechPhraseProperty = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPPName: DISPID_SpeechPhraseProperty = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPPId: DISPID_SpeechPhraseProperty = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPPValue: DISPID_SpeechPhraseProperty = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPPFirstElement: DISPID_SpeechPhraseProperty = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPPNumberOfElements: DISPID_SpeechPhraseProperty = 5i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPPEngineConfidence: DISPID_SpeechPhraseProperty = 6i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPPConfidence: DISPID_SpeechPhraseProperty = 7i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPPParent: DISPID_SpeechPhraseProperty = 8i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPPChildren: DISPID_SpeechPhraseProperty = 9i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechPhraseReplacement = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPRDisplayAttributes: DISPID_SpeechPhraseReplacement = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPRText: DISPID_SpeechPhraseReplacement = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPRFirstElement: DISPID_SpeechPhraseReplacement = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPRNumberOfElements: DISPID_SpeechPhraseReplacement = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechPhraseReplacements = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPRsCount: DISPID_SpeechPhraseReplacements = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPRsItem: DISPID_SpeechPhraseReplacements = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPRs_NewEnum: DISPID_SpeechPhraseReplacements = -4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechPhraseRule = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPRuleName: DISPID_SpeechPhraseRule = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPRuleId: DISPID_SpeechPhraseRule = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPRuleFirstElement: DISPID_SpeechPhraseRule = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPRuleNumberOfElements: DISPID_SpeechPhraseRule = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPRuleParent: DISPID_SpeechPhraseRule = 5i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPRuleChildren: DISPID_SpeechPhraseRule = 6i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPRuleConfidence: DISPID_SpeechPhraseRule = 7i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPRuleEngineConfidence: DISPID_SpeechPhraseRule = 8i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechPhraseRules = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPRulesCount: DISPID_SpeechPhraseRules = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPRulesItem: DISPID_SpeechPhraseRules = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPRules_NewEnum: DISPID_SpeechPhraseRules = -4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechRecoContext = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCRecognizer: DISPID_SpeechRecoContext = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCAudioInInterferenceStatus: DISPID_SpeechRecoContext = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCRequestedUIType: DISPID_SpeechRecoContext = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCVoice: DISPID_SpeechRecoContext = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRAllowVoiceFormatMatchingOnNextSet: DISPID_SpeechRecoContext = 5i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCVoicePurgeEvent: DISPID_SpeechRecoContext = 6i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCEventInterests: DISPID_SpeechRecoContext = 7i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCCmdMaxAlternates: DISPID_SpeechRecoContext = 8i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCState: DISPID_SpeechRecoContext = 9i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCRetainedAudio: DISPID_SpeechRecoContext = 10i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCRetainedAudioFormat: DISPID_SpeechRecoContext = 11i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCPause: DISPID_SpeechRecoContext = 12i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCResume: DISPID_SpeechRecoContext = 13i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCCreateGrammar: DISPID_SpeechRecoContext = 14i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCCreateResultFromMemory: DISPID_SpeechRecoContext = 15i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCBookmark: DISPID_SpeechRecoContext = 16i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCSetAdaptationData: DISPID_SpeechRecoContext = 17i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechRecoContextEvents = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCEStartStream: DISPID_SpeechRecoContextEvents = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCEEndStream: DISPID_SpeechRecoContextEvents = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCEBookmark: DISPID_SpeechRecoContextEvents = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCESoundStart: DISPID_SpeechRecoContextEvents = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCESoundEnd: DISPID_SpeechRecoContextEvents = 5i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCEPhraseStart: DISPID_SpeechRecoContextEvents = 6i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCERecognition: DISPID_SpeechRecoContextEvents = 7i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCEHypothesis: DISPID_SpeechRecoContextEvents = 8i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCEPropertyNumberChange: DISPID_SpeechRecoContextEvents = 9i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCEPropertyStringChange: DISPID_SpeechRecoContextEvents = 10i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCEFalseRecognition: DISPID_SpeechRecoContextEvents = 11i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCEInterference: DISPID_SpeechRecoContextEvents = 12i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCERequestUI: DISPID_SpeechRecoContextEvents = 13i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCERecognizerStateChange: DISPID_SpeechRecoContextEvents = 14i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCEAdaptation: DISPID_SpeechRecoContextEvents = 15i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCERecognitionForOtherContext: DISPID_SpeechRecoContextEvents = 16i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCEAudioLevel: DISPID_SpeechRecoContextEvents = 17i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCEEnginePrivate: DISPID_SpeechRecoContextEvents = 18i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechRecoResult = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRRRecoContext: DISPID_SpeechRecoResult = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRRTimes: DISPID_SpeechRecoResult = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRRAudioFormat: DISPID_SpeechRecoResult = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRRPhraseInfo: DISPID_SpeechRecoResult = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRRAlternates: DISPID_SpeechRecoResult = 5i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRRAudio: DISPID_SpeechRecoResult = 6i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRRSpeakAudio: DISPID_SpeechRecoResult = 7i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRRSaveToMemory: DISPID_SpeechRecoResult = 8i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRRDiscardResultInfo: DISPID_SpeechRecoResult = 9i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechRecoResult2 = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRRSetTextFeedback: DISPID_SpeechRecoResult2 = 12i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechRecoResultTimes = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRRTStreamTime: DISPID_SpeechRecoResultTimes = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRRTLength: DISPID_SpeechRecoResultTimes = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRRTTickCount: DISPID_SpeechRecoResultTimes = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRRTOffsetFromStart: DISPID_SpeechRecoResultTimes = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechRecognizer = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRRecognizer: DISPID_SpeechRecognizer = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRAllowAudioInputFormatChangesOnNextSet: DISPID_SpeechRecognizer = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRAudioInput: DISPID_SpeechRecognizer = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRAudioInputStream: DISPID_SpeechRecognizer = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRIsShared: DISPID_SpeechRecognizer = 5i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRState: DISPID_SpeechRecognizer = 6i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRStatus: DISPID_SpeechRecognizer = 7i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRProfile: DISPID_SpeechRecognizer = 8i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SREmulateRecognition: DISPID_SpeechRecognizer = 9i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCreateRecoContext: DISPID_SpeechRecognizer = 10i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGetFormat: DISPID_SpeechRecognizer = 11i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRSetPropertyNumber: DISPID_SpeechRecognizer = 12i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGetPropertyNumber: DISPID_SpeechRecognizer = 13i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRSetPropertyString: DISPID_SpeechRecognizer = 14i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGetPropertyString: DISPID_SpeechRecognizer = 15i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRIsUISupported: DISPID_SpeechRecognizer = 16i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRDisplayUI: DISPID_SpeechRecognizer = 17i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGetRecognizers: DISPID_SpeechRecognizer = 18i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVGetAudioInputs: DISPID_SpeechRecognizer = 19i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVGetProfiles: DISPID_SpeechRecognizer = 20i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechRecognizerStatus = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRSAudioStatus: DISPID_SpeechRecognizerStatus = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRSCurrentStreamPosition: DISPID_SpeechRecognizerStatus = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRSCurrentStreamNumber: DISPID_SpeechRecognizerStatus = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRSNumberOfActiveRules: DISPID_SpeechRecognizerStatus = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRSClsidEngine: DISPID_SpeechRecognizerStatus = 5i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRSSupportedLanguages: DISPID_SpeechRecognizerStatus = 6i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechVoice = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVStatus: DISPID_SpeechVoice = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVVoice: DISPID_SpeechVoice = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVAudioOutput: DISPID_SpeechVoice = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVAudioOutputStream: DISPID_SpeechVoice = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVRate: DISPID_SpeechVoice = 5i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVVolume: DISPID_SpeechVoice = 6i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVAllowAudioOuputFormatChangesOnNextSet: DISPID_SpeechVoice = 7i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVEventInterests: DISPID_SpeechVoice = 8i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVPriority: DISPID_SpeechVoice = 9i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVAlertBoundary: DISPID_SpeechVoice = 10i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVSyncronousSpeakTimeout: DISPID_SpeechVoice = 11i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVSpeak: DISPID_SpeechVoice = 12i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVSpeakStream: DISPID_SpeechVoice = 13i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVPause: DISPID_SpeechVoice = 14i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVResume: DISPID_SpeechVoice = 15i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVSkip: DISPID_SpeechVoice = 16i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVGetVoices: DISPID_SpeechVoice = 17i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVGetAudioOutputs: DISPID_SpeechVoice = 18i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVWaitUntilDone: DISPID_SpeechVoice = 19i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVSpeakCompleteEvent: DISPID_SpeechVoice = 20i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVIsUISupported: DISPID_SpeechVoice = 21i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVDisplayUI: DISPID_SpeechVoice = 22i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechVoiceEvent = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVEStreamStart: DISPID_SpeechVoiceEvent = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVEStreamEnd: DISPID_SpeechVoiceEvent = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVEVoiceChange: DISPID_SpeechVoiceEvent = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVEBookmark: DISPID_SpeechVoiceEvent = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVEWord: DISPID_SpeechVoiceEvent = 5i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVEPhoneme: DISPID_SpeechVoiceEvent = 6i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVESentenceBoundary: DISPID_SpeechVoiceEvent = 7i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVEViseme: DISPID_SpeechVoiceEvent = 8i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVEAudioLevel: DISPID_SpeechVoiceEvent = 9i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVEEnginePrivate: DISPID_SpeechVoiceEvent = 10i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechVoiceStatus = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVSCurrentStreamNumber: DISPID_SpeechVoiceStatus = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVSLastStreamNumberQueued: DISPID_SpeechVoiceStatus = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVSLastResult: DISPID_SpeechVoiceStatus = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVSRunningState: DISPID_SpeechVoiceStatus = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVSInputWordPosition: DISPID_SpeechVoiceStatus = 5i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVSInputWordLength: DISPID_SpeechVoiceStatus = 6i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVSInputSentencePosition: DISPID_SpeechVoiceStatus = 7i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVSInputSentenceLength: DISPID_SpeechVoiceStatus = 8i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVSLastBookmark: DISPID_SpeechVoiceStatus = 9i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVSLastBookmarkId: DISPID_SpeechVoiceStatus = 10i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVSPhonemeId: DISPID_SpeechVoiceStatus = 11i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVSVisemeId: DISPID_SpeechVoiceStatus = 12i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechWaveFormatEx = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SWFEFormatTag: DISPID_SpeechWaveFormatEx = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SWFEChannels: DISPID_SpeechWaveFormatEx = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SWFESamplesPerSec: DISPID_SpeechWaveFormatEx = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SWFEAvgBytesPerSec: DISPID_SpeechWaveFormatEx = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SWFEBlockAlign: DISPID_SpeechWaveFormatEx = 5i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SWFEBitsPerSample: DISPID_SpeechWaveFormatEx = 6i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SWFEExtraData: DISPID_SpeechWaveFormatEx = 7i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type DISPID_SpeechXMLRecoResult = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRRGetXMLResult: DISPID_SpeechXMLRecoResult = 10i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRRGetXMLErrorInfo: DISPID_SpeechXMLRecoResult = 11i32;
#[repr(C)]
pub struct IEnumSpObjectTokens {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, pelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, index: u32, pptoken: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self, pcount: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpAudio {
    pub base__: ISpStreamFormat,
    pub SetState: unsafe extern "system" fn(this: *mut *mut Self, newstate: SPAUDIOSTATE, ullreserved: u64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub SetFormat: unsafe extern "system" fn(this: *mut *mut Self, rguidfmtid: *const ::windows_sys::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    SetFormat: usize,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, pstatus: *mut SPAUDIOSTATUS) -> ::windows_sys::core::HRESULT,
    pub SetBufferInfo: unsafe extern "system" fn(this: *mut *mut Self, pbuffinfo: *const SPAUDIOBUFFERINFO) -> ::windows_sys::core::HRESULT,
    pub GetBufferInfo: unsafe extern "system" fn(this: *mut *mut Self, pbuffinfo: *mut SPAUDIOBUFFERINFO) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub GetDefaultFormat: unsafe extern "system" fn(this: *mut *mut Self, pformatid: *mut ::windows_sys::core::GUID, ppcomemwaveformatex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    GetDefaultFormat: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EventHandle: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::Foundation::HANDLE,
    #[cfg(not(feature = "Win32_Foundation"))]
    EventHandle: usize,
    pub GetVolumeLevel: unsafe extern "system" fn(this: *mut *mut Self, plevel: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetVolumeLevel: unsafe extern "system" fn(this: *mut *mut Self, level: u32) -> ::windows_sys::core::HRESULT,
    pub GetBufferNotifySize: unsafe extern "system" fn(this: *mut *mut Self, pcbsize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetBufferNotifySize: unsafe extern "system" fn(this: *mut *mut Self, cbsize: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpContainerLexicon {
    pub base__: ISpLexicon,
    pub AddLexicon: unsafe extern "system" fn(this: *mut *mut Self, paddlexicon: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpDataKey {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetData: unsafe extern "system" fn(this: *mut *mut Self, pszvaluename: ::windows_sys::core::PCWSTR, cbdata: u32, pdata: *const u8) -> ::windows_sys::core::HRESULT,
    pub GetData: unsafe extern "system" fn(this: *mut *mut Self, pszvaluename: ::windows_sys::core::PCWSTR, pcbdata: *mut u32, pdata: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetStringValue: unsafe extern "system" fn(this: *mut *mut Self, pszvaluename: ::windows_sys::core::PCWSTR, pszvalue: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetStringValue: unsafe extern "system" fn(this: *mut *mut Self, pszvaluename: ::windows_sys::core::PCWSTR, ppszvalue: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetDWORD: unsafe extern "system" fn(this: *mut *mut Self, pszvaluename: ::windows_sys::core::PCWSTR, dwvalue: u32) -> ::windows_sys::core::HRESULT,
    pub GetDWORD: unsafe extern "system" fn(this: *mut *mut Self, pszvaluename: ::windows_sys::core::PCWSTR, pdwvalue: *mut u32) -> ::windows_sys::core::HRESULT,
    pub OpenKey: unsafe extern "system" fn(this: *mut *mut Self, pszsubkeyname: ::windows_sys::core::PCWSTR, ppsubkey: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateKey: unsafe extern "system" fn(this: *mut *mut Self, pszsubkey: ::windows_sys::core::PCWSTR, ppsubkey: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DeleteKey: unsafe extern "system" fn(this: *mut *mut Self, pszsubkey: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub DeleteValue: unsafe extern "system" fn(this: *mut *mut Self, pszvaluename: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub EnumKeys: unsafe extern "system" fn(this: *mut *mut Self, index: u32, ppszsubkeyname: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub EnumValues: unsafe extern "system" fn(this: *mut *mut Self, index: u32, ppszvaluename: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpDisplayAlternates {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetDisplayAlternates: unsafe extern "system" fn(this: *mut *mut Self, pphrase: *const SPDISPLAYPHRASE, crequestcount: u32, ppcomemphrases: *mut *mut SPDISPLAYPHRASE, pcphrasesreturned: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetFullStopTrailSpace: unsafe extern "system" fn(this: *mut *mut Self, ultrailspace: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpEnginePronunciation {
    pub base__: ::windows_sys::core::IUnknown,
    pub Normalize: unsafe extern "system" fn(this: *mut *mut Self, pszword: ::windows_sys::core::PCWSTR, pszleftcontext: ::windows_sys::core::PCWSTR, pszrightcontext: ::windows_sys::core::PCWSTR, langid: u16, pnormalizationlist: *mut SPNORMALIZATIONLIST) -> ::windows_sys::core::HRESULT,
    pub GetPronunciations: unsafe extern "system" fn(this: *mut *mut Self, pszword: ::windows_sys::core::PCWSTR, pszleftcontext: ::windows_sys::core::PCWSTR, pszrightcontext: ::windows_sys::core::PCWSTR, langid: u16, penginepronunciationlist: *mut SPWORDPRONUNCIATIONLIST) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpEventSink {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub AddEvents: unsafe extern "system" fn(this: *mut *mut Self, peventarray: *const SPEVENT, ulcount: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddEvents: usize,
    pub GetEventInterest: unsafe extern "system" fn(this: *mut *mut Self, pulleventinterest: *mut u64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpEventSource {
    pub base__: ISpNotifySource,
    pub SetInterest: unsafe extern "system" fn(this: *mut *mut Self, ulleventinterest: u64, ullqueuedinterest: u64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEvents: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32, peventarray: *mut SPEVENT, pulfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEvents: usize,
    pub GetInfo: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *mut SPEVENTSOURCEINFO) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpEventSource2 {
    pub base__: ISpEventSource,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEventsEx: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32, peventarray: *mut SPEVENTEX, pulfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEventsEx: usize,
}
#[repr(C)]
pub struct ISpGrammarBuilder {
    pub base__: ::windows_sys::core::IUnknown,
    pub ResetGrammar: unsafe extern "system" fn(this: *mut *mut Self, newlanguage: u16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRule: unsafe extern "system" fn(this: *mut *mut Self, pszrulename: ::windows_sys::core::PCWSTR, dwruleid: u32, dwattributes: u32, fcreateifnotexist: super::super::Foundation::BOOL, phinitialstate: *mut *mut SPSTATEHANDLE__) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRule: usize,
    pub ClearRule: unsafe extern "system" fn(this: *mut *mut Self, hstate: *mut SPSTATEHANDLE__) -> ::windows_sys::core::HRESULT,
    pub CreateNewState: unsafe extern "system" fn(this: *mut *mut Self, hstate: *mut SPSTATEHANDLE__, phstate: *mut *mut SPSTATEHANDLE__) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddWordTransition: unsafe extern "system" fn(this: *mut *mut Self, hfromstate: *mut SPSTATEHANDLE__, htostate: *mut SPSTATEHANDLE__, psz: ::windows_sys::core::PCWSTR, pszseparators: ::windows_sys::core::PCWSTR, ewordtype: SPGRAMMARWORDTYPE, weight: f32, ppropinfo: *const SPPROPERTYINFO) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddWordTransition: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddRuleTransition: unsafe extern "system" fn(this: *mut *mut Self, hfromstate: *mut SPSTATEHANDLE__, htostate: *mut SPSTATEHANDLE__, hrule: *mut SPSTATEHANDLE__, weight: f32, ppropinfo: *const SPPROPERTYINFO) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddRuleTransition: usize,
    pub AddResource: unsafe extern "system" fn(this: *mut *mut Self, hrulestate: *mut SPSTATEHANDLE__, pszresourcename: ::windows_sys::core::PCWSTR, pszresourcevalue: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub Commit: unsafe extern "system" fn(this: *mut *mut Self, dwreserved: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpGrammarBuilder2 {
    pub base__: ::windows_sys::core::IUnknown,
    pub AddTextSubset: unsafe extern "system" fn(this: *mut *mut Self, hfromstate: *mut SPSTATEHANDLE__, htostate: *mut SPSTATEHANDLE__, psz: ::windows_sys::core::PCWSTR, ematchmode: SPMATCHINGMODE) -> ::windows_sys::core::HRESULT,
    pub SetPhoneticAlphabet: unsafe extern "system" fn(this: *mut *mut Self, phoneticalphabet: PHONETICALPHABET) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpLexicon {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetPronunciations: unsafe extern "system" fn(this: *mut *mut Self, pszword: ::windows_sys::core::PCWSTR, langid: u16, dwflags: u32, pwordpronunciationlist: *mut SPWORDPRONUNCIATIONLIST) -> ::windows_sys::core::HRESULT,
    pub AddPronunciation: unsafe extern "system" fn(this: *mut *mut Self, pszword: ::windows_sys::core::PCWSTR, langid: u16, epartofspeech: SPPARTOFSPEECH, pszpronunciation: *const u16) -> ::windows_sys::core::HRESULT,
    pub RemovePronunciation: unsafe extern "system" fn(this: *mut *mut Self, pszword: ::windows_sys::core::PCWSTR, langid: u16, epartofspeech: SPPARTOFSPEECH, pszpronunciation: *const u16) -> ::windows_sys::core::HRESULT,
    pub GetGeneration: unsafe extern "system" fn(this: *mut *mut Self, pdwgeneration: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetGenerationChange: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, pdwgeneration: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows_sys::core::HRESULT,
    pub GetWords: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, pdwgeneration: *mut u32, pdwcookie: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpMMSysAudio {
    pub base__: ISpAudio,
    pub GetDeviceId: unsafe extern "system" fn(this: *mut *mut Self, pudeviceid: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetDeviceId: unsafe extern "system" fn(this: *mut *mut Self, udeviceid: u32) -> ::windows_sys::core::HRESULT,
    pub GetMMHandle: unsafe extern "system" fn(this: *mut *mut Self, phandle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetLineId: unsafe extern "system" fn(this: *mut *mut Self, pulineid: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetLineId: unsafe extern "system" fn(this: *mut *mut Self, ulineid: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpNotifyCallback {
    #[cfg(feature = "Win32_Foundation")]
    pub NotifyCallback: unsafe extern "system" fn(this: *mut *mut Self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    NotifyCallback: usize,
}
#[repr(C)]
pub struct ISpNotifySink {
    pub base__: ::windows_sys::core::IUnknown,
    pub Notify: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpNotifySource {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetNotifySink: unsafe extern "system" fn(this: *mut *mut Self, pnotifysink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNotifyWindowMessage: unsafe extern "system" fn(this: *mut *mut Self, hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNotifyWindowMessage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNotifyCallbackFunction: unsafe extern "system" fn(this: *mut *mut Self, pfncallback: *mut *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNotifyCallbackFunction: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNotifyCallbackInterface: unsafe extern "system" fn(this: *mut *mut Self, pspcallback: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNotifyCallbackInterface: usize,
    pub SetNotifyWin32Event: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub WaitForNotifyEvent: unsafe extern "system" fn(this: *mut *mut Self, dwmilliseconds: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetNotifyEventHandle: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::Foundation::HANDLE,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetNotifyEventHandle: usize,
}
#[repr(C)]
pub struct ISpNotifyTranslator {
    pub base__: ISpNotifySink,
    #[cfg(feature = "Win32_Foundation")]
    pub InitWindowMessage: unsafe extern "system" fn(this: *mut *mut Self, hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitWindowMessage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitCallback: unsafe extern "system" fn(this: *mut *mut Self, pfncallback: *mut *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitCallback: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitSpNotifyCallback: unsafe extern "system" fn(this: *mut *mut Self, pspcallback: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitSpNotifyCallback: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitWin32Event: unsafe extern "system" fn(this: *mut *mut Self, hevent: super::super::Foundation::HANDLE, fclosehandleonrelease: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitWin32Event: usize,
    pub Wait: unsafe extern "system" fn(this: *mut *mut Self, dwmilliseconds: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEventHandle: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::Foundation::HANDLE,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEventHandle: usize,
}
#[repr(C)]
pub struct ISpObjectToken {
    pub base__: ISpDataKey,
    #[cfg(feature = "Win32_Foundation")]
    pub SetId: unsafe extern "system" fn(this: *mut *mut Self, pszcategoryid: ::windows_sys::core::PCWSTR, psztokenid: ::windows_sys::core::PCWSTR, fcreateifnotexist: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetId: usize,
    pub GetId: unsafe extern "system" fn(this: *mut *mut Self, ppszcomemtokenid: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetCategory: unsafe extern "system" fn(this: *mut *mut Self, pptokencategory: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, punkouter: *mut ::core::ffi::c_void, dwclscontext: u32, riid: *const ::windows_sys::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStorageFileName: unsafe extern "system" fn(this: *mut *mut Self, clsidcaller: *const ::windows_sys::core::GUID, pszvaluename: ::windows_sys::core::PCWSTR, pszfilenamespecifier: ::windows_sys::core::PCWSTR, nfolder: u32, ppszfilepath: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoveStorageFileName: unsafe extern "system" fn(this: *mut *mut Self, clsidcaller: *const ::windows_sys::core::GUID, pszkeyname: ::windows_sys::core::PCWSTR, fdeletefile: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoveStorageFileName: usize,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, pclsidcaller: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsUISupported: unsafe extern "system" fn(this: *mut *mut Self, psztypeofui: ::windows_sys::core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, punkobject: *mut ::core::ffi::c_void, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsUISupported: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayUI: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: super::super::Foundation::HWND, psztitle: ::windows_sys::core::PCWSTR, psztypeofui: ::windows_sys::core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, punkobject: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayUI: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MatchesAttributes: unsafe extern "system" fn(this: *mut *mut Self, pszattributes: ::windows_sys::core::PCWSTR, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MatchesAttributes: usize,
}
#[repr(C)]
pub struct ISpObjectTokenCategory {
    pub base__: ISpDataKey,
    #[cfg(feature = "Win32_Foundation")]
    pub SetId: unsafe extern "system" fn(this: *mut *mut Self, pszcategoryid: ::windows_sys::core::PCWSTR, fcreateifnotexist: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetId: usize,
    pub GetId: unsafe extern "system" fn(this: *mut *mut Self, ppszcomemcategoryid: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetDataKey: unsafe extern "system" fn(this: *mut *mut Self, spdkl: SPDATAKEYLOCATION, ppdatakey: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EnumTokens: unsafe extern "system" fn(this: *mut *mut Self, pzsreqattribs: ::windows_sys::core::PCWSTR, pszoptattribs: ::windows_sys::core::PCWSTR, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetDefaultTokenId: unsafe extern "system" fn(this: *mut *mut Self, psztokenid: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetDefaultTokenId: unsafe extern "system" fn(this: *mut *mut Self, ppszcomemtokenid: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpObjectTokenInit {
    pub base__: ISpObjectToken,
    pub InitFromDataKey: unsafe extern "system" fn(this: *mut *mut Self, pszcategoryid: ::windows_sys::core::PCWSTR, psztokenid: ::windows_sys::core::PCWSTR, pdatakey: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpObjectWithToken {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetObjectToken: unsafe extern "system" fn(this: *mut *mut Self, ptoken: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetObjectToken: unsafe extern "system" fn(this: *mut *mut Self, pptoken: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpPhoneConverter {
    pub base__: ISpObjectWithToken,
    pub PhoneToId: unsafe extern "system" fn(this: *mut *mut Self, pszphone: ::windows_sys::core::PCWSTR, pid: *mut u16) -> ::windows_sys::core::HRESULT,
    pub IdToPhone: unsafe extern "system" fn(this: *mut *mut Self, pid: *const u16, pszphone: ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpPhoneticAlphabetConverter {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetLangId: unsafe extern "system" fn(this: *mut *mut Self, plangid: *mut u16) -> ::windows_sys::core::HRESULT,
    pub SetLangId: unsafe extern "system" fn(this: *mut *mut Self, langid: u16) -> ::windows_sys::core::HRESULT,
    pub SAPI2UPS: unsafe extern "system" fn(this: *mut *mut Self, pszsapiid: *const u16, pszupsid: *mut u16, cmaxlength: u32) -> ::windows_sys::core::HRESULT,
    pub UPS2SAPI: unsafe extern "system" fn(this: *mut *mut Self, pszupsid: *const u16, pszsapiid: *mut u16, cmaxlength: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMaxConvertLength: unsafe extern "system" fn(this: *mut *mut Self, csrclength: u32, bsapi2ups: super::super::Foundation::BOOL, pcmaxdestlength: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMaxConvertLength: usize,
}
#[repr(C)]
pub struct ISpPhoneticAlphabetSelection {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub IsAlphabetUPS: unsafe extern "system" fn(this: *mut *mut Self, pfisups: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsAlphabetUPS: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAlphabetToUPS: unsafe extern "system" fn(this: *mut *mut Self, fforceups: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAlphabetToUPS: usize,
}
#[repr(C)]
pub struct ISpPhrase {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetPhrase: unsafe extern "system" fn(this: *mut *mut Self, ppcomemphrase: *mut *mut SPPHRASE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetPhrase: usize,
    pub GetSerializedPhrase: unsafe extern "system" fn(this: *mut *mut Self, ppcomemphrase: *mut *mut SPSERIALIZEDPHRASE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetText: unsafe extern "system" fn(this: *mut *mut Self, ulstart: u32, ulcount: u32, fusetextreplacements: super::super::Foundation::BOOL, ppszcomemtext: *mut ::windows_sys::core::PWSTR, pbdisplayattributes: *mut u8) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetText: usize,
    pub Discard: unsafe extern "system" fn(this: *mut *mut Self, dwvaluetypes: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpPhrase2 {
    pub base__: ISpPhrase,
    pub GetXMLResult: unsafe extern "system" fn(this: *mut *mut Self, ppszcomemxmlresult: *mut ::windows_sys::core::PWSTR, options: SPXMLRESULTOPTIONS) -> ::windows_sys::core::HRESULT,
    pub GetXMLErrorInfo: unsafe extern "system" fn(this: *mut *mut Self, psemanticerrorinfo: *mut SPSEMANTICERRORINFO) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetAudio: unsafe extern "system" fn(this: *mut *mut Self, ulstartelement: u32, celements: u32, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetAudio: usize,
}
#[repr(C)]
pub struct ISpPhraseAlt {
    pub base__: ISpPhrase,
    pub GetAltInfo: unsafe extern "system" fn(this: *mut *mut Self, ppparent: *mut *mut ::core::ffi::c_void, pulstartelementinparent: *mut u32, pcelementsinparent: *mut u32, pcelementsinalt: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Commit: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpProperties {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetPropertyNum: unsafe extern "system" fn(this: *mut *mut Self, pname: ::windows_sys::core::PCWSTR, lvalue: i32) -> ::windows_sys::core::HRESULT,
    pub GetPropertyNum: unsafe extern "system" fn(this: *mut *mut Self, pname: ::windows_sys::core::PCWSTR, plvalue: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetPropertyString: unsafe extern "system" fn(this: *mut *mut Self, pname: ::windows_sys::core::PCWSTR, pvalue: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetPropertyString: unsafe extern "system" fn(this: *mut *mut Self, pname: ::windows_sys::core::PCWSTR, ppcomemvalue: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpRecoContext {
    pub base__: ISpEventSource,
    pub GetRecognizer: unsafe extern "system" fn(this: *mut *mut Self, pprecognizer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateGrammar: unsafe extern "system" fn(this: *mut *mut Self, ullgrammarid: u64, ppgrammar: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, pstatus: *mut SPRECOCONTEXTSTATUS) -> ::windows_sys::core::HRESULT,
    pub GetMaxAlternates: unsafe extern "system" fn(this: *mut *mut Self, pcalternates: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetMaxAlternates: unsafe extern "system" fn(this: *mut *mut Self, calternates: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub SetAudioOptions: unsafe extern "system" fn(this: *mut *mut Self, options: SPAUDIOOPTIONS, paudioformatid: *const ::windows_sys::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    SetAudioOptions: usize,
    #[cfg(feature = "Win32_Media_Audio")]
    pub GetAudioOptions: unsafe extern "system" fn(this: *mut *mut Self, poptions: *mut SPAUDIOOPTIONS, paudioformatid: *mut ::windows_sys::core::GUID, ppcomemwfex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    GetAudioOptions: usize,
    pub DeserializeResult: unsafe extern "system" fn(this: *mut *mut Self, pserializedresult: *const SPSERIALIZEDRESULT, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Bookmark: unsafe extern "system" fn(this: *mut *mut Self, options: SPBOOKMARKOPTIONS, ullstreamposition: u64, lparamevent: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Bookmark: usize,
    pub SetAdaptationData: unsafe extern "system" fn(this: *mut *mut Self, padaptationdata: ::windows_sys::core::PCWSTR, cch: u32) -> ::windows_sys::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut *mut Self, dwreserved: u32) -> ::windows_sys::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut *mut Self, dwreserved: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetVoice: unsafe extern "system" fn(this: *mut *mut Self, pvoice: *mut ::core::ffi::c_void, fallowformatchanges: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetVoice: usize,
    pub GetVoice: unsafe extern "system" fn(this: *mut *mut Self, ppvoice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetVoicePurgeEvent: unsafe extern "system" fn(this: *mut *mut Self, ulleventinterest: u64) -> ::windows_sys::core::HRESULT,
    pub GetVoicePurgeEvent: unsafe extern "system" fn(this: *mut *mut Self, pulleventinterest: *mut u64) -> ::windows_sys::core::HRESULT,
    pub SetContextState: unsafe extern "system" fn(this: *mut *mut Self, econtextstate: SPCONTEXTSTATE) -> ::windows_sys::core::HRESULT,
    pub GetContextState: unsafe extern "system" fn(this: *mut *mut Self, pecontextstate: *mut SPCONTEXTSTATE) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpRecoContext2 {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetGrammarOptions: unsafe extern "system" fn(this: *mut *mut Self, egrammaroptions: u32) -> ::windows_sys::core::HRESULT,
    pub GetGrammarOptions: unsafe extern "system" fn(this: *mut *mut Self, pegrammaroptions: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetAdaptationData2: unsafe extern "system" fn(this: *mut *mut Self, padaptationdata: ::windows_sys::core::PCWSTR, cch: u32, ptopicname: ::windows_sys::core::PCWSTR, eadaptationsettings: u32, erelevance: SPADAPTATIONRELEVANCE) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpRecoGrammar {
    pub base__: ISpGrammarBuilder,
    pub GetGrammarId: unsafe extern "system" fn(this: *mut *mut Self, pullgrammarid: *mut u64) -> ::windows_sys::core::HRESULT,
    pub GetRecoContext: unsafe extern "system" fn(this: *mut *mut Self, pprecoctxt: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LoadCmdFromFile: unsafe extern "system" fn(this: *mut *mut Self, pszfilename: ::windows_sys::core::PCWSTR, options: SPLOADOPTIONS) -> ::windows_sys::core::HRESULT,
    pub LoadCmdFromObject: unsafe extern "system" fn(this: *mut *mut Self, rcid: *const ::windows_sys::core::GUID, pszgrammarname: ::windows_sys::core::PCWSTR, options: SPLOADOPTIONS) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LoadCmdFromResource: unsafe extern "system" fn(this: *mut *mut Self, hmodule: super::super::Foundation::HINSTANCE, pszresourcename: ::windows_sys::core::PCWSTR, pszresourcetype: ::windows_sys::core::PCWSTR, wlanguage: u16, options: SPLOADOPTIONS) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LoadCmdFromResource: usize,
    pub LoadCmdFromMemory: unsafe extern "system" fn(this: *mut *mut Self, pgrammar: *const SPBINARYGRAMMAR, options: SPLOADOPTIONS) -> ::windows_sys::core::HRESULT,
    pub LoadCmdFromProprietaryGrammar: unsafe extern "system" fn(this: *mut *mut Self, rguidparam: *const ::windows_sys::core::GUID, pszstringparam: ::windows_sys::core::PCWSTR, pvdataprarm: *const ::core::ffi::c_void, cbdatasize: u32, options: SPLOADOPTIONS) -> ::windows_sys::core::HRESULT,
    pub SetRuleState: unsafe extern "system" fn(this: *mut *mut Self, pszname: ::windows_sys::core::PCWSTR, preserved: *mut ::core::ffi::c_void, newstate: SPRULESTATE) -> ::windows_sys::core::HRESULT,
    pub SetRuleIdState: unsafe extern "system" fn(this: *mut *mut Self, ulruleid: u32, newstate: SPRULESTATE) -> ::windows_sys::core::HRESULT,
    pub LoadDictation: unsafe extern "system" fn(this: *mut *mut Self, psztopicname: ::windows_sys::core::PCWSTR, options: SPLOADOPTIONS) -> ::windows_sys::core::HRESULT,
    pub UnloadDictation: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetDictationState: unsafe extern "system" fn(this: *mut *mut Self, newstate: SPRULESTATE) -> ::windows_sys::core::HRESULT,
    pub SetWordSequenceData: unsafe extern "system" fn(this: *mut *mut Self, ptext: ::windows_sys::core::PCWSTR, cchtext: u32, pinfo: *const SPTEXTSELECTIONINFO) -> ::windows_sys::core::HRESULT,
    pub SetTextSelection: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const SPTEXTSELECTIONINFO) -> ::windows_sys::core::HRESULT,
    pub IsPronounceable: unsafe extern "system" fn(this: *mut *mut Self, pszword: ::windows_sys::core::PCWSTR, pwordpronounceable: *mut SPWORDPRONOUNCEABLE) -> ::windows_sys::core::HRESULT,
    pub SetGrammarState: unsafe extern "system" fn(this: *mut *mut Self, egrammarstate: SPGRAMMARSTATE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SaveCmd: unsafe extern "system" fn(this: *mut *mut Self, pstream: *mut ::core::ffi::c_void, ppszcomemerrortext: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SaveCmd: usize,
    pub GetGrammarState: unsafe extern "system" fn(this: *mut *mut Self, pegrammarstate: *mut SPGRAMMARSTATE) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpRecoGrammar2 {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetRules: unsafe extern "system" fn(this: *mut *mut Self, ppcomemrules: *mut *mut SPRULE, punumrules: *mut u32) -> ::windows_sys::core::HRESULT,
    pub LoadCmdFromFile2: unsafe extern "system" fn(this: *mut *mut Self, pszfilename: ::windows_sys::core::PCWSTR, options: SPLOADOPTIONS, pszsharinguri: ::windows_sys::core::PCWSTR, pszbaseuri: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub LoadCmdFromMemory2: unsafe extern "system" fn(this: *mut *mut Self, pgrammar: *const SPBINARYGRAMMAR, options: SPLOADOPTIONS, pszsharinguri: ::windows_sys::core::PCWSTR, pszbaseuri: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub SetRulePriority: unsafe extern "system" fn(this: *mut *mut Self, pszrulename: ::windows_sys::core::PCWSTR, ulruleid: u32, nrulepriority: i32) -> ::windows_sys::core::HRESULT,
    pub SetRuleWeight: unsafe extern "system" fn(this: *mut *mut Self, pszrulename: ::windows_sys::core::PCWSTR, ulruleid: u32, flweight: f32) -> ::windows_sys::core::HRESULT,
    pub SetDictationWeight: unsafe extern "system" fn(this: *mut *mut Self, flweight: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetGrammarLoader: unsafe extern "system" fn(this: *mut *mut Self, ploader: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetGrammarLoader: usize,
    #[cfg(feature = "Win32_System_Com_Urlmon")]
    pub SetSMLSecurityManager: unsafe extern "system" fn(this: *mut *mut Self, psmlsecuritymanager: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_Urlmon"))]
    SetSMLSecurityManager: usize,
}
#[repr(C)]
pub struct ISpRecoResult {
    pub base__: ISpPhrase,
    #[cfg(feature = "Win32_Foundation")]
    pub GetResultTimes: unsafe extern "system" fn(this: *mut *mut Self, ptimes: *mut SPRECORESULTTIMES) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetResultTimes: usize,
    pub GetAlternates: unsafe extern "system" fn(this: *mut *mut Self, ulstartelement: u32, celements: u32, ulrequestcount: u32, ppphrases: *mut *mut ::core::ffi::c_void, pcphrasesreturned: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetAudio: unsafe extern "system" fn(this: *mut *mut Self, ulstartelement: u32, celements: u32, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetAudio: usize,
    pub SpeakAudio: unsafe extern "system" fn(this: *mut *mut Self, ulstartelement: u32, celements: u32, dwflags: u32, pulstreamnumber: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Serialize: unsafe extern "system" fn(this: *mut *mut Self, ppcomemserializedresult: *mut *mut SPSERIALIZEDRESULT) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub ScaleAudio: unsafe extern "system" fn(this: *mut *mut Self, paudioformatid: *const ::windows_sys::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    ScaleAudio: usize,
    pub GetRecoContext: unsafe extern "system" fn(this: *mut *mut Self, pprecocontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpRecoResult2 {
    pub base__: ISpRecoResult,
    pub CommitAlternate: unsafe extern "system" fn(this: *mut *mut Self, pphrasealt: *mut ::core::ffi::c_void, ppnewresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CommitText: unsafe extern "system" fn(this: *mut *mut Self, ulstartelement: u32, celements: u32, pszcorrecteddata: ::windows_sys::core::PCWSTR, ecommitflags: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTextFeedback: unsafe extern "system" fn(this: *mut *mut Self, pszfeedback: ::windows_sys::core::PCWSTR, fsuccessful: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTextFeedback: usize,
}
#[repr(C)]
pub struct ISpRecognizer {
    pub base__: ISpProperties,
    pub SetRecognizer: unsafe extern "system" fn(this: *mut *mut Self, precognizer: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetRecognizer: unsafe extern "system" fn(this: *mut *mut Self, pprecognizer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetInput: unsafe extern "system" fn(this: *mut *mut Self, punkinput: *mut ::core::ffi::c_void, fallowformatchanges: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetInput: usize,
    pub GetInputObjectToken: unsafe extern "system" fn(this: *mut *mut Self, pptoken: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetInputStream: unsafe extern "system" fn(this: *mut *mut Self, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetInputStream: usize,
    pub CreateRecoContext: unsafe extern "system" fn(this: *mut *mut Self, ppnewctxt: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetRecoProfile: unsafe extern "system" fn(this: *mut *mut Self, pptoken: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRecoProfile: unsafe extern "system" fn(this: *mut *mut Self, ptoken: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsSharedInstance: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetRecoState: unsafe extern "system" fn(this: *mut *mut Self, pstate: *mut SPRECOSTATE) -> ::windows_sys::core::HRESULT,
    pub SetRecoState: unsafe extern "system" fn(this: *mut *mut Self, newstate: SPRECOSTATE) -> ::windows_sys::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, pstatus: *mut SPRECOGNIZERSTATUS) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub GetFormat: unsafe extern "system" fn(this: *mut *mut Self, waveformattype: SPWAVEFORMATTYPE, pformatid: *mut ::windows_sys::core::GUID, ppcomemwfex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    GetFormat: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsUISupported: unsafe extern "system" fn(this: *mut *mut Self, psztypeofui: ::windows_sys::core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsUISupported: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayUI: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: super::super::Foundation::HWND, psztitle: ::windows_sys::core::PCWSTR, psztypeofui: ::windows_sys::core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayUI: usize,
    pub EmulateRecognition: unsafe extern "system" fn(this: *mut *mut Self, pphrase: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpRecognizer2 {
    pub base__: ::windows_sys::core::IUnknown,
    pub EmulateRecognitionEx: unsafe extern "system" fn(this: *mut *mut Self, pphrase: *mut ::core::ffi::c_void, dwcompareflags: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTrainingState: unsafe extern "system" fn(this: *mut *mut Self, fdoingtraining: super::super::Foundation::BOOL, fadaptfromtrainingdata: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTrainingState: usize,
    pub ResetAcousticModelAdaptation: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpRegDataKey {
    pub base__: ISpDataKey,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub SetKey: unsafe extern "system" fn(this: *mut *mut Self, hkey: super::super::System::Registry::HKEY, freadonly: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Registry")))]
    SetKey: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpResourceManager {
    pub base__: super::super::System::Com::IServiceProvider,
    pub SetObject: unsafe extern "system" fn(this: *mut *mut Self, guidserviceid: *const ::windows_sys::core::GUID, punkobject: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetObject: unsafe extern "system" fn(this: *mut *mut Self, guidserviceid: *const ::windows_sys::core::GUID, objectclsid: *const ::windows_sys::core::GUID, objectiid: *const ::windows_sys::core::GUID, freleasewhenlastexternalrefreleased: super::super::Foundation::BOOL, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetObject: usize,
}
#[repr(C)]
pub struct ISpSerializeState {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetSerializedState: unsafe extern "system" fn(this: *mut *mut Self, ppbdata: *mut *mut u8, pulsize: *mut u32, dwreserved: u32) -> ::windows_sys::core::HRESULT,
    pub SetSerializedState: unsafe extern "system" fn(this: *mut *mut Self, pbdata: *const u8, ulsize: u32, dwreserved: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpShortcut {
    pub base__: ::windows_sys::core::IUnknown,
    pub AddShortcut: unsafe extern "system" fn(this: *mut *mut Self, pszdisplay: ::windows_sys::core::PCWSTR, langid: u16, pszspoken: ::windows_sys::core::PCWSTR, shtype: SPSHORTCUTTYPE) -> ::windows_sys::core::HRESULT,
    pub RemoveShortcut: unsafe extern "system" fn(this: *mut *mut Self, pszdisplay: ::windows_sys::core::PCWSTR, langid: u16, pszspoken: ::windows_sys::core::PCWSTR, shtype: SPSHORTCUTTYPE) -> ::windows_sys::core::HRESULT,
    pub GetShortcuts: unsafe extern "system" fn(this: *mut *mut Self, langid: u16, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> ::windows_sys::core::HRESULT,
    pub GetGeneration: unsafe extern "system" fn(this: *mut *mut Self, pdwgeneration: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetWordsFromGenerationChange: unsafe extern "system" fn(this: *mut *mut Self, pdwgeneration: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows_sys::core::HRESULT,
    pub GetWords: unsafe extern "system" fn(this: *mut *mut Self, pdwgeneration: *mut u32, pdwcookie: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows_sys::core::HRESULT,
    pub GetShortcutsForGeneration: unsafe extern "system" fn(this: *mut *mut Self, pdwgeneration: *mut u32, pdwcookie: *mut u32, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> ::windows_sys::core::HRESULT,
    pub GetGenerationChange: unsafe extern "system" fn(this: *mut *mut Self, pdwgeneration: *mut u32, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpStream {
    pub base__: ISpStreamFormat,
    #[cfg(all(feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
    pub SetBaseStream: unsafe extern "system" fn(this: *mut *mut Self, pstream: *mut ::core::ffi::c_void, rguidformat: *const ::windows_sys::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Media_Audio", feature = "Win32_System_Com")))]
    SetBaseStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetBaseStream: unsafe extern "system" fn(this: *mut *mut Self, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetBaseStream: usize,
    #[cfg(feature = "Win32_Media_Audio")]
    pub BindToFile: unsafe extern "system" fn(this: *mut *mut Self, pszfilename: ::windows_sys::core::PCWSTR, emode: SPFILEMODE, pformatid: *const ::windows_sys::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX, ulleventinterest: u64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    BindToFile: usize,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpStreamFormat {
    pub base__: super::super::System::Com::IStream,
    #[cfg(feature = "Win32_Media_Audio")]
    pub GetFormat: unsafe extern "system" fn(this: *mut *mut Self, pguidformatid: *const ::windows_sys::core::GUID, ppcomemwaveformatex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    GetFormat: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpStreamFormatConverter {
    pub base__: ISpStreamFormat,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub SetBaseStream: unsafe extern "system" fn(this: *mut *mut Self, pstream: *mut ::core::ffi::c_void, fsetformattobasestreamformat: super::super::Foundation::BOOL, fwritetobasestream: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    SetBaseStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetBaseStream: unsafe extern "system" fn(this: *mut *mut Self, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetBaseStream: usize,
    #[cfg(feature = "Win32_Media_Audio")]
    pub SetFormat: unsafe extern "system" fn(this: *mut *mut Self, rguidformatidofconvertedstream: *const ::windows_sys::core::GUID, pwaveformatexofconvertedstream: *const super::Audio::WAVEFORMATEX) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    SetFormat: usize,
    pub ResetSeekPosition: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ScaleConvertedToBaseOffset: unsafe extern "system" fn(this: *mut *mut Self, ulloffsetconvertedstream: u64, pulloffsetbasestream: *mut u64) -> ::windows_sys::core::HRESULT,
    pub ScaleBaseToConvertedOffset: unsafe extern "system" fn(this: *mut *mut Self, ulloffsetbasestream: u64, pulloffsetconvertedstream: *mut u64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpTranscript {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetTranscript: unsafe extern "system" fn(this: *mut *mut Self, ppsztranscript: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub AppendTranscript: unsafe extern "system" fn(this: *mut *mut Self, psztranscript: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpVoice {
    pub base__: ISpEventSource,
    #[cfg(feature = "Win32_Foundation")]
    pub SetOutput: unsafe extern "system" fn(this: *mut *mut Self, punkoutput: *mut ::core::ffi::c_void, fallowformatchanges: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetOutput: usize,
    pub GetOutputObjectToken: unsafe extern "system" fn(this: *mut *mut Self, ppobjecttoken: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetOutputStream: unsafe extern "system" fn(this: *mut *mut Self, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetOutputStream: usize,
    pub Pause: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetVoice: unsafe extern "system" fn(this: *mut *mut Self, ptoken: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetVoice: unsafe extern "system" fn(this: *mut *mut Self, pptoken: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Speak: unsafe extern "system" fn(this: *mut *mut Self, pwcs: ::windows_sys::core::PCWSTR, dwflags: u32, pulstreamnumber: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SpeakStream: unsafe extern "system" fn(this: *mut *mut Self, pstream: *mut ::core::ffi::c_void, dwflags: u32, pulstreamnumber: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SpeakStream: usize,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, pstatus: *mut SPVOICESTATUS, ppszlastbookmark: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, pitemtype: ::windows_sys::core::PCWSTR, lnumitems: i32, pulnumskipped: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut *mut Self, epriority: SPVPRIORITY) -> ::windows_sys::core::HRESULT,
    pub GetPriority: unsafe extern "system" fn(this: *mut *mut Self, pepriority: *mut SPVPRIORITY) -> ::windows_sys::core::HRESULT,
    pub SetAlertBoundary: unsafe extern "system" fn(this: *mut *mut Self, eboundary: SPEVENTENUM) -> ::windows_sys::core::HRESULT,
    pub GetAlertBoundary: unsafe extern "system" fn(this: *mut *mut Self, peboundary: *mut SPEVENTENUM) -> ::windows_sys::core::HRESULT,
    pub SetRate: unsafe extern "system" fn(this: *mut *mut Self, rateadjust: i32) -> ::windows_sys::core::HRESULT,
    pub GetRate: unsafe extern "system" fn(this: *mut *mut Self, prateadjust: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetVolume: unsafe extern "system" fn(this: *mut *mut Self, usvolume: u16) -> ::windows_sys::core::HRESULT,
    pub GetVolume: unsafe extern "system" fn(this: *mut *mut Self, pusvolume: *mut u16) -> ::windows_sys::core::HRESULT,
    pub WaitUntilDone: unsafe extern "system" fn(this: *mut *mut Self, mstimeout: u32) -> ::windows_sys::core::HRESULT,
    pub SetSyncSpeakTimeout: unsafe extern "system" fn(this: *mut *mut Self, mstimeout: u32) -> ::windows_sys::core::HRESULT,
    pub GetSyncSpeakTimeout: unsafe extern "system" fn(this: *mut *mut Self, pmstimeout: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SpeakCompleteEvent: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::Foundation::HANDLE,
    #[cfg(not(feature = "Win32_Foundation"))]
    SpeakCompleteEvent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsUISupported: unsafe extern "system" fn(this: *mut *mut Self, psztypeofui: ::windows_sys::core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsUISupported: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayUI: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: super::super::Foundation::HWND, psztitle: ::windows_sys::core::PCWSTR, psztypeofui: ::windows_sys::core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayUI: usize,
}
#[repr(C)]
pub struct ISpXMLRecoResult {
    pub base__: ISpRecoResult,
    pub GetXMLResult: unsafe extern "system" fn(this: *mut *mut Self, ppszcomemxmlresult: *mut ::windows_sys::core::PWSTR, options: SPXMLRESULTOPTIONS) -> ::windows_sys::core::HRESULT,
    pub GetXMLErrorInfo: unsafe extern "system" fn(this: *mut *mut Self, psemanticerrorinfo: *mut SPSEMANTICERRORINFO) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechAudio {
    pub base__: ISpeechBaseStream,
    #[cfg(feature = "Win32_System_Com")]
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, status: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Status: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub BufferInfo: unsafe extern "system" fn(this: *mut *mut Self, bufferinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BufferInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DefaultFormat: unsafe extern "system" fn(this: *mut *mut Self, streamformat: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DefaultFormat: usize,
    pub Volume: unsafe extern "system" fn(this: *mut *mut Self, volume: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetVolume: unsafe extern "system" fn(this: *mut *mut Self, volume: i32) -> ::windows_sys::core::HRESULT,
    pub BufferNotifySize: unsafe extern "system" fn(this: *mut *mut Self, buffernotifysize: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetBufferNotifySize: unsafe extern "system" fn(this: *mut *mut Self, buffernotifysize: i32) -> ::windows_sys::core::HRESULT,
    pub EventHandle: unsafe extern "system" fn(this: *mut *mut Self, eventhandle: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetState: unsafe extern "system" fn(this: *mut *mut Self, state: SpeechAudioState) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechAudioBufferInfo {
    pub base__: super::super::System::Com::IDispatch,
    pub MinNotification: unsafe extern "system" fn(this: *mut *mut Self, minnotification: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMinNotification: unsafe extern "system" fn(this: *mut *mut Self, minnotification: i32) -> ::windows_sys::core::HRESULT,
    pub BufferSize: unsafe extern "system" fn(this: *mut *mut Self, buffersize: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetBufferSize: unsafe extern "system" fn(this: *mut *mut Self, buffersize: i32) -> ::windows_sys::core::HRESULT,
    pub EventBias: unsafe extern "system" fn(this: *mut *mut Self, eventbias: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetEventBias: unsafe extern "system" fn(this: *mut *mut Self, eventbias: i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechAudioFormat {
    pub base__: super::super::System::Com::IDispatch,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, audioformat: *mut SpeechAudioFormatType) -> ::windows_sys::core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut *mut Self, audioformat: SpeechAudioFormatType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Guid: unsafe extern "system" fn(this: *mut *mut Self, guid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Guid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetGuid: unsafe extern "system" fn(this: *mut *mut Self, guid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetGuid: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetWaveFormatEx: unsafe extern "system" fn(this: *mut *mut Self, speechwaveformatex: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetWaveFormatEx: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetWaveFormatEx: unsafe extern "system" fn(this: *mut *mut Self, speechwaveformatex: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetWaveFormatEx: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechAudioStatus {
    pub base__: super::super::System::Com::IDispatch,
    pub FreeBufferSpace: unsafe extern "system" fn(this: *mut *mut Self, freebufferspace: *mut i32) -> ::windows_sys::core::HRESULT,
    pub NonBlockingIO: unsafe extern "system" fn(this: *mut *mut Self, nonblockingio: *mut i32) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, state: *mut SpeechAudioState) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CurrentSeekPosition: unsafe extern "system" fn(this: *mut *mut Self, currentseekposition: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CurrentSeekPosition: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CurrentDevicePosition: unsafe extern "system" fn(this: *mut *mut Self, currentdeviceposition: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CurrentDevicePosition: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechBaseStream {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Format: unsafe extern "system" fn(this: *mut *mut Self, audioformat: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Format: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_Format: unsafe extern "system" fn(this: *mut *mut Self, audioformat: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_Format: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Read: unsafe extern "system" fn(this: *mut *mut Self, buffer: *mut super::super::System::Com::VARIANT, numberofbytes: i32, bytesread: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Read: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Write: unsafe extern "system" fn(this: *mut *mut Self, buffer: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, byteswritten: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Write: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Seek: unsafe extern "system" fn(this: *mut *mut Self, position: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, origin: SpeechStreamSeekPositionType, newposition: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Seek: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechCustomStream {
    pub base__: ISpeechBaseStream,
    pub BaseStream: unsafe extern "system" fn(this: *mut *mut Self, ppunkstream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub putref_BaseStream: unsafe extern "system" fn(this: *mut *mut Self, punkstream: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechDataKey {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetBinaryValue: unsafe extern "system" fn(this: *mut *mut Self, valuename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetBinaryValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetBinaryValue: unsafe extern "system" fn(this: *mut *mut Self, valuename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetBinaryValue: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetStringValue: unsafe extern "system" fn(this: *mut *mut Self, valuename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetStringValue: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetStringValue: unsafe extern "system" fn(this: *mut *mut Self, valuename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetStringValue: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLongValue: unsafe extern "system" fn(this: *mut *mut Self, valuename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLongValue: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLongValue: unsafe extern "system" fn(this: *mut *mut Self, valuename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLongValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OpenKey: unsafe extern "system" fn(this: *mut *mut Self, subkeyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, subkey: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OpenKey: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateKey: unsafe extern "system" fn(this: *mut *mut Self, subkeyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, subkey: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateKey: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteKey: unsafe extern "system" fn(this: *mut *mut Self, subkeyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteKey: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteValue: unsafe extern "system" fn(this: *mut *mut Self, valuename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteValue: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumKeys: unsafe extern "system" fn(this: *mut *mut Self, index: i32, subkeyname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumKeys: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumValues: unsafe extern "system" fn(this: *mut *mut Self, index: i32, valuename: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumValues: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechFileStream {
    pub base__: ISpeechBaseStream,
    #[cfg(feature = "Win32_Foundation")]
    pub Open: unsafe extern "system" fn(this: *mut *mut Self, filename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filemode: SpeechStreamFileMode, doevents: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Open: usize,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechGrammarRule {
    pub base__: super::super::System::Com::IDispatch,
    pub Attributes: unsafe extern "system" fn(this: *mut *mut Self, attributes: *mut SpeechRuleAttributes) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InitialState: unsafe extern "system" fn(this: *mut *mut Self, state: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitialState: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, name: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, id: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddResource: unsafe extern "system" fn(this: *mut *mut Self, resourcename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, resourcevalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddResource: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddState: unsafe extern "system" fn(this: *mut *mut Self, state: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddState: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechGrammarRuleState {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Rule: unsafe extern "system" fn(this: *mut *mut Self, rule: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Rule: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Transitions: unsafe extern "system" fn(this: *mut *mut Self, transitions: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Transitions: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddWordTransition: unsafe extern "system" fn(this: *mut *mut Self, deststate: *mut ::core::ffi::c_void, words: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, separators: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, r#type: SpeechGrammarWordType, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyid: i32, propertyvalue: *const super::super::System::Com::VARIANT, weight: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddWordTransition: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddRuleTransition: unsafe extern "system" fn(this: *mut *mut Self, destinationstate: *mut ::core::ffi::c_void, rule: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyid: i32, propertyvalue: *const super::super::System::Com::VARIANT, weight: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddRuleTransition: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddSpecialTransition: unsafe extern "system" fn(this: *mut *mut Self, destinationstate: *mut ::core::ffi::c_void, r#type: SpeechSpecialTransitionType, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyid: i32, propertyvalue: *const super::super::System::Com::VARIANT, weight: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddSpecialTransition: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechGrammarRuleStateTransition {
    pub base__: super::super::System::Com::IDispatch,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, r#type: *mut SpeechGrammarRuleStateTransitionType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, text: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Text: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Rule: unsafe extern "system" fn(this: *mut *mut Self, rule: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Rule: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Weight: unsafe extern "system" fn(this: *mut *mut Self, weight: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Weight: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PropertyName: unsafe extern "system" fn(this: *mut *mut Self, propertyname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PropertyName: usize,
    pub PropertyId: unsafe extern "system" fn(this: *mut *mut Self, propertyid: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PropertyValue: unsafe extern "system" fn(this: *mut *mut Self, propertyvalue: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PropertyValue: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub NextState: unsafe extern "system" fn(this: *mut *mut Self, nextstate: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NextState: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechGrammarRuleStateTransitions {
    pub base__: super::super::System::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, count: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, transition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechGrammarRules {
    pub base__: super::super::System::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, count: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub FindRule: unsafe extern "system" fn(this: *mut *mut Self, rulenameorid: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, rule: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    FindRule: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, rule: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Dynamic: unsafe extern "system" fn(this: *mut *mut Self, dynamic: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, rulename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, attributes: SpeechRuleAttributes, ruleid: i32, rule: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Add: usize,
    pub Commit: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CommitAndSave: unsafe extern "system" fn(this: *mut *mut Self, errortext: *mut super::super::Foundation::BSTR, savestream: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CommitAndSave: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechLexicon {
    pub base__: super::super::System::Com::IDispatch,
    pub GenerationId: unsafe extern "system" fn(this: *mut *mut Self, generationid: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetWords: unsafe extern "system" fn(this: *mut *mut Self, flags: SpeechLexiconType, generationid: *mut i32, words: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetWords: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AddPronunciation: unsafe extern "system" fn(this: *mut *mut Self, bstrword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, langid: i32, partofspeech: SpeechPartOfSpeech, bstrpronunciation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddPronunciation: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPronunciationByPhoneIds: unsafe extern "system" fn(this: *mut *mut Self, bstrword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, langid: i32, partofspeech: SpeechPartOfSpeech, phoneids: *const super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPronunciationByPhoneIds: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RemovePronunciation: unsafe extern "system" fn(this: *mut *mut Self, bstrword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, langid: i32, partofspeech: SpeechPartOfSpeech, bstrpronunciation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemovePronunciation: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RemovePronunciationByPhoneIds: unsafe extern "system" fn(this: *mut *mut Self, bstrword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, langid: i32, partofspeech: SpeechPartOfSpeech, phoneids: *const super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RemovePronunciationByPhoneIds: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetPronunciations: unsafe extern "system" fn(this: *mut *mut Self, bstrword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, langid: i32, typeflags: SpeechLexiconType, pppronunciations: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetPronunciations: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetGenerationChange: unsafe extern "system" fn(this: *mut *mut Self, generationid: *mut i32, ppwords: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetGenerationChange: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechLexiconPronunciation {
    pub base__: super::super::System::Com::IDispatch,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, lexicontype: *mut SpeechLexiconType) -> ::windows_sys::core::HRESULT,
    pub LangId: unsafe extern "system" fn(this: *mut *mut Self, langid: *mut i32) -> ::windows_sys::core::HRESULT,
    pub PartOfSpeech: unsafe extern "system" fn(this: *mut *mut Self, partofspeech: *mut SpeechPartOfSpeech) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PhoneIds: unsafe extern "system" fn(this: *mut *mut Self, phoneids: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PhoneIds: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Symbolic: unsafe extern "system" fn(this: *mut *mut Self, symbolic: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Symbolic: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechLexiconPronunciations {
    pub base__: super::super::System::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, count: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pronunciation: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechLexiconWord {
    pub base__: super::super::System::Com::IDispatch,
    pub LangId: unsafe extern "system" fn(this: *mut *mut Self, langid: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, wordtype: *mut SpeechWordType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Word: unsafe extern "system" fn(this: *mut *mut Self, word: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Word: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Pronunciations: unsafe extern "system" fn(this: *mut *mut Self, pronunciations: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Pronunciations: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechLexiconWords {
    pub base__: super::super::System::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, count: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, word: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechMMSysAudio {
    pub base__: ISpeechAudio,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, deviceid: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetDeviceId: unsafe extern "system" fn(this: *mut *mut Self, deviceid: i32) -> ::windows_sys::core::HRESULT,
    pub LineId: unsafe extern "system" fn(this: *mut *mut Self, lineid: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetLineId: unsafe extern "system" fn(this: *mut *mut Self, lineid: i32) -> ::windows_sys::core::HRESULT,
    pub MMHandle: unsafe extern "system" fn(this: *mut *mut Self, handle: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechMemoryStream {
    pub base__: ISpeechBaseStream,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetData: unsafe extern "system" fn(this: *mut *mut Self, data: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetData: unsafe extern "system" fn(this: *mut *mut Self, pdata: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetData: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechObjectToken {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, objectid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Id: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DataKey: unsafe extern "system" fn(this: *mut *mut Self, datakey: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DataKey: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Category: unsafe extern "system" fn(this: *mut *mut Self, category: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Category: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDescription: unsafe extern "system" fn(this: *mut *mut Self, locale: i32, description: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDescription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetId: unsafe extern "system" fn(this: *mut *mut Self, id: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, categoryid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, createifnotexist: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAttribute: unsafe extern "system" fn(this: *mut *mut Self, attributename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, attributevalue: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAttribute: usize,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, punkouter: *mut ::core::ffi::c_void, clscontext: SpeechTokenContext, object: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, objectstorageclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Remove: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetStorageFileName: unsafe extern "system" fn(this: *mut *mut Self, objectstorageclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, keyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, folder: SpeechTokenShellFolder, filepath: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetStorageFileName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoveStorageFileName: unsafe extern "system" fn(this: *mut *mut Self, objectstorageclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, keyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, deletefilea: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoveStorageFileName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub IsUISupported: unsafe extern "system" fn(this: *mut *mut Self, typeofui: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, extradata: *const super::super::System::Com::VARIANT, object: *mut ::core::ffi::c_void, supported: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    IsUISupported: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DisplayUI: unsafe extern "system" fn(this: *mut *mut Self, hwnd: i32, title: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, typeofui: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, extradata: *const super::super::System::Com::VARIANT, object: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DisplayUI: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MatchesAttributes: unsafe extern "system" fn(this: *mut *mut Self, attributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, matches: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MatchesAttributes: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechObjectTokenCategory {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, id: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Id: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDefault: unsafe extern "system" fn(this: *mut *mut Self, tokenid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDefault: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Default: unsafe extern "system" fn(this: *mut *mut Self, tokenid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Default: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetId: unsafe extern "system" fn(this: *mut *mut Self, id: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, createifnotexist: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetId: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDataKey: unsafe extern "system" fn(this: *mut *mut Self, location: SpeechDataKeyLocation, datakey: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDataKey: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub EnumerateTokens: unsafe extern "system" fn(this: *mut *mut Self, requiredattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, optionalattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, tokens: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    EnumerateTokens: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechObjectTokens {
    pub base__: super::super::System::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, count: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, token: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppenumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechPhoneConverter {
    pub base__: super::super::System::Com::IDispatch,
    pub LanguageId: unsafe extern "system" fn(this: *mut *mut Self, languageid: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetLanguageId: unsafe extern "system" fn(this: *mut *mut Self, languageid: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PhoneToId: unsafe extern "system" fn(this: *mut *mut Self, phonemes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, idarray: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PhoneToId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub IdToPhone: unsafe extern "system" fn(this: *mut *mut Self, idarray: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, phonemes: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    IdToPhone: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechPhraseAlternate {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub RecoResult: unsafe extern "system" fn(this: *mut *mut Self, recoresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RecoResult: usize,
    pub StartElementInResult: unsafe extern "system" fn(this: *mut *mut Self, startelement: *mut i32) -> ::windows_sys::core::HRESULT,
    pub NumberOfElementsInResult: unsafe extern "system" fn(this: *mut *mut Self, numberofelements: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub PhraseInfo: unsafe extern "system" fn(this: *mut *mut Self, phraseinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PhraseInfo: usize,
    pub Commit: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechPhraseAlternates {
    pub base__: super::super::System::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, count: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, phrasealternate: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechPhraseElement {
    pub base__: super::super::System::Com::IDispatch,
    pub AudioTimeOffset: unsafe extern "system" fn(this: *mut *mut Self, audiotimeoffset: *mut i32) -> ::windows_sys::core::HRESULT,
    pub AudioSizeTime: unsafe extern "system" fn(this: *mut *mut Self, audiosizetime: *mut i32) -> ::windows_sys::core::HRESULT,
    pub AudioStreamOffset: unsafe extern "system" fn(this: *mut *mut Self, audiostreamoffset: *mut i32) -> ::windows_sys::core::HRESULT,
    pub AudioSizeBytes: unsafe extern "system" fn(this: *mut *mut Self, audiosizebytes: *mut i32) -> ::windows_sys::core::HRESULT,
    pub RetainedStreamOffset: unsafe extern "system" fn(this: *mut *mut Self, retainedstreamoffset: *mut i32) -> ::windows_sys::core::HRESULT,
    pub RetainedSizeBytes: unsafe extern "system" fn(this: *mut *mut Self, retainedsizebytes: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayText: unsafe extern "system" fn(this: *mut *mut Self, displaytext: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayText: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LexicalForm: unsafe extern "system" fn(this: *mut *mut Self, lexicalform: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LexicalForm: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Pronunciation: unsafe extern "system" fn(this: *mut *mut Self, pronunciation: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Pronunciation: usize,
    pub DisplayAttributes: unsafe extern "system" fn(this: *mut *mut Self, displayattributes: *mut SpeechDisplayAttributes) -> ::windows_sys::core::HRESULT,
    pub RequiredConfidence: unsafe extern "system" fn(this: *mut *mut Self, requiredconfidence: *mut SpeechEngineConfidence) -> ::windows_sys::core::HRESULT,
    pub ActualConfidence: unsafe extern "system" fn(this: *mut *mut Self, actualconfidence: *mut SpeechEngineConfidence) -> ::windows_sys::core::HRESULT,
    pub EngineConfidence: unsafe extern "system" fn(this: *mut *mut Self, engineconfidence: *mut f32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechPhraseElements {
    pub base__: super::super::System::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, count: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, element: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechPhraseInfo {
    pub base__: super::super::System::Com::IDispatch,
    pub LanguageId: unsafe extern "system" fn(this: *mut *mut Self, languageid: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GrammarId: unsafe extern "system" fn(this: *mut *mut Self, grammarid: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GrammarId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub StartTime: unsafe extern "system" fn(this: *mut *mut Self, starttime: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    StartTime: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AudioStreamPosition: unsafe extern "system" fn(this: *mut *mut Self, audiostreamposition: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AudioStreamPosition: usize,
    pub AudioSizeBytes: unsafe extern "system" fn(this: *mut *mut Self, paudiosizebytes: *mut i32) -> ::windows_sys::core::HRESULT,
    pub RetainedSizeBytes: unsafe extern "system" fn(this: *mut *mut Self, retainedsizebytes: *mut i32) -> ::windows_sys::core::HRESULT,
    pub AudioSizeTime: unsafe extern "system" fn(this: *mut *mut Self, audiosizetime: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Rule: unsafe extern "system" fn(this: *mut *mut Self, rule: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Rule: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, properties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Elements: unsafe extern "system" fn(this: *mut *mut Self, elements: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Elements: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Replacements: unsafe extern "system" fn(this: *mut *mut Self, replacements: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Replacements: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EngineId: unsafe extern "system" fn(this: *mut *mut Self, engineidguid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EngineId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub EnginePrivateData: unsafe extern "system" fn(this: *mut *mut Self, privatedata: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    EnginePrivateData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SaveToMemory: unsafe extern "system" fn(this: *mut *mut Self, phraseblock: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SaveToMemory: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetText: unsafe extern "system" fn(this: *mut *mut Self, startelement: i32, elements: i32, usereplacements: i16, text: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetText: usize,
    pub GetDisplayAttributes: unsafe extern "system" fn(this: *mut *mut Self, startelement: i32, elements: i32, usereplacements: i16, displayattributes: *mut SpeechDisplayAttributes) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechPhraseInfoBuilder {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RestorePhraseFromMemory: unsafe extern "system" fn(this: *mut *mut Self, phraseinmemory: *const super::super::System::Com::VARIANT, phraseinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RestorePhraseFromMemory: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechPhraseProperties {
    pub base__: super::super::System::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, count: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, property: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechPhraseProperty {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, name: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, id: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, value: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Value: usize,
    pub FirstElement: unsafe extern "system" fn(this: *mut *mut Self, firstelement: *mut i32) -> ::windows_sys::core::HRESULT,
    pub NumberOfElements: unsafe extern "system" fn(this: *mut *mut Self, numberofelements: *mut i32) -> ::windows_sys::core::HRESULT,
    pub EngineConfidence: unsafe extern "system" fn(this: *mut *mut Self, confidence: *mut f32) -> ::windows_sys::core::HRESULT,
    pub Confidence: unsafe extern "system" fn(this: *mut *mut Self, confidence: *mut SpeechEngineConfidence) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Parent: unsafe extern "system" fn(this: *mut *mut Self, parentproperty: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Parent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Children: unsafe extern "system" fn(this: *mut *mut Self, children: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Children: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechPhraseReplacement {
    pub base__: super::super::System::Com::IDispatch,
    pub DisplayAttributes: unsafe extern "system" fn(this: *mut *mut Self, displayattributes: *mut SpeechDisplayAttributes) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, text: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Text: usize,
    pub FirstElement: unsafe extern "system" fn(this: *mut *mut Self, firstelement: *mut i32) -> ::windows_sys::core::HRESULT,
    pub NumberOfElements: unsafe extern "system" fn(this: *mut *mut Self, numberofelements: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechPhraseReplacements {
    pub base__: super::super::System::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, count: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, reps: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechPhraseRule {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, name: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, id: *mut i32) -> ::windows_sys::core::HRESULT,
    pub FirstElement: unsafe extern "system" fn(this: *mut *mut Self, firstelement: *mut i32) -> ::windows_sys::core::HRESULT,
    pub NumberOfElements: unsafe extern "system" fn(this: *mut *mut Self, numberofelements: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Parent: unsafe extern "system" fn(this: *mut *mut Self, parent: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Parent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Children: unsafe extern "system" fn(this: *mut *mut Self, children: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Children: usize,
    pub Confidence: unsafe extern "system" fn(this: *mut *mut Self, actualconfidence: *mut SpeechEngineConfidence) -> ::windows_sys::core::HRESULT,
    pub EngineConfidence: unsafe extern "system" fn(this: *mut *mut Self, engineconfidence: *mut f32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechPhraseRules {
    pub base__: super::super::System::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, count: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, rule: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechRecoContext {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Recognizer: unsafe extern "system" fn(this: *mut *mut Self, recognizer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Recognizer: usize,
    pub AudioInputInterferenceStatus: unsafe extern "system" fn(this: *mut *mut Self, interference: *mut SpeechInterference) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestedUIType: unsafe extern "system" fn(this: *mut *mut Self, uitype: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestedUIType: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_Voice: unsafe extern "system" fn(this: *mut *mut Self, voice: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_Voice: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Voice: unsafe extern "system" fn(this: *mut *mut Self, voice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Voice: usize,
    pub SetAllowVoiceFormatMatchingOnNextSet: unsafe extern "system" fn(this: *mut *mut Self, allow: i16) -> ::windows_sys::core::HRESULT,
    pub AllowVoiceFormatMatchingOnNextSet: unsafe extern "system" fn(this: *mut *mut Self, pallow: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetVoicePurgeEvent: unsafe extern "system" fn(this: *mut *mut Self, eventinterest: SpeechRecoEvents) -> ::windows_sys::core::HRESULT,
    pub VoicePurgeEvent: unsafe extern "system" fn(this: *mut *mut Self, eventinterest: *mut SpeechRecoEvents) -> ::windows_sys::core::HRESULT,
    pub SetEventInterests: unsafe extern "system" fn(this: *mut *mut Self, eventinterest: SpeechRecoEvents) -> ::windows_sys::core::HRESULT,
    pub EventInterests: unsafe extern "system" fn(this: *mut *mut Self, eventinterest: *mut SpeechRecoEvents) -> ::windows_sys::core::HRESULT,
    pub SetCmdMaxAlternates: unsafe extern "system" fn(this: *mut *mut Self, maxalternates: i32) -> ::windows_sys::core::HRESULT,
    pub CmdMaxAlternates: unsafe extern "system" fn(this: *mut *mut Self, maxalternates: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetState: unsafe extern "system" fn(this: *mut *mut Self, state: SpeechRecoContextState) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, state: *mut SpeechRecoContextState) -> ::windows_sys::core::HRESULT,
    pub SetRetainedAudio: unsafe extern "system" fn(this: *mut *mut Self, option: SpeechRetainedAudioOptions) -> ::windows_sys::core::HRESULT,
    pub RetainedAudio: unsafe extern "system" fn(this: *mut *mut Self, option: *mut SpeechRetainedAudioOptions) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_RetainedAudioFormat: unsafe extern "system" fn(this: *mut *mut Self, format: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_RetainedAudioFormat: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RetainedAudioFormat: unsafe extern "system" fn(this: *mut *mut Self, format: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RetainedAudioFormat: usize,
    pub Pause: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateGrammar: unsafe extern "system" fn(this: *mut *mut Self, grammarid: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, grammar: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateGrammar: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateResultFromMemory: unsafe extern "system" fn(this: *mut *mut Self, resultblock: *const super::super::System::Com::VARIANT, result: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateResultFromMemory: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Bookmark: unsafe extern "system" fn(this: *mut *mut Self, options: SpeechBookmarkOptions, streampos: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, bookmarkid: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Bookmark: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAdaptationData: unsafe extern "system" fn(this: *mut *mut Self, adaptationstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAdaptationData: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechRecoGrammar {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, id: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Id: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RecoContext: unsafe extern "system" fn(this: *mut *mut Self, recocontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RecoContext: usize,
    pub SetState: unsafe extern "system" fn(this: *mut *mut Self, state: SpeechGrammarState) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, state: *mut SpeechGrammarState) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Rules: unsafe extern "system" fn(this: *mut *mut Self, rules: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Rules: usize,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self, newlanguage: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CmdLoadFromFile: unsafe extern "system" fn(this: *mut *mut Self, filename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loadoption: SpeechLoadOption) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CmdLoadFromFile: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CmdLoadFromObject: unsafe extern "system" fn(this: *mut *mut Self, classid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, grammarname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loadoption: SpeechLoadOption) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CmdLoadFromObject: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CmdLoadFromResource: unsafe extern "system" fn(this: *mut *mut Self, hmodule: i32, resourcename: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, resourcetype: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, languageid: i32, loadoption: SpeechLoadOption) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CmdLoadFromResource: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CmdLoadFromMemory: unsafe extern "system" fn(this: *mut *mut Self, grammardata: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, loadoption: SpeechLoadOption) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CmdLoadFromMemory: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CmdLoadFromProprietaryGrammar: unsafe extern "system" fn(this: *mut *mut Self, proprietaryguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, proprietarystring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, proprietarydata: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, loadoption: SpeechLoadOption) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CmdLoadFromProprietaryGrammar: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CmdSetRuleState: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, state: SpeechRuleState) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CmdSetRuleState: usize,
    pub CmdSetRuleIdState: unsafe extern "system" fn(this: *mut *mut Self, ruleid: i32, state: SpeechRuleState) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DictationLoad: unsafe extern "system" fn(this: *mut *mut Self, topicname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loadoption: SpeechLoadOption) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DictationLoad: usize,
    pub DictationUnload: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub DictationSetState: unsafe extern "system" fn(this: *mut *mut Self, state: SpeechRuleState) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub SetWordSequenceData: unsafe extern "system" fn(this: *mut *mut Self, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, textlength: i32, info: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    SetWordSequenceData: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetTextSelection: unsafe extern "system" fn(this: *mut *mut Self, info: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetTextSelection: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPronounceable: unsafe extern "system" fn(this: *mut *mut Self, word: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, wordpronounceable: *mut SpeechWordPronounceable) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPronounceable: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechRecoResult {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub RecoContext: unsafe extern "system" fn(this: *mut *mut Self, recocontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RecoContext: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Times: unsafe extern "system" fn(this: *mut *mut Self, times: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Times: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AudioFormat: unsafe extern "system" fn(this: *mut *mut Self, format: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AudioFormat: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AudioFormat: unsafe extern "system" fn(this: *mut *mut Self, format: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AudioFormat: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PhraseInfo: unsafe extern "system" fn(this: *mut *mut Self, phraseinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PhraseInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Alternates: unsafe extern "system" fn(this: *mut *mut Self, requestcount: i32, startelement: i32, elements: i32, alternates: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Alternates: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Audio: unsafe extern "system" fn(this: *mut *mut Self, startelement: i32, elements: i32, stream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Audio: usize,
    pub SpeakAudio: unsafe extern "system" fn(this: *mut *mut Self, startelement: i32, elements: i32, flags: SpeechVoiceSpeakFlags, streamnumber: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SaveToMemory: unsafe extern "system" fn(this: *mut *mut Self, resultblock: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SaveToMemory: usize,
    pub DiscardResultInfo: unsafe extern "system" fn(this: *mut *mut Self, valuetypes: SpeechDiscardType) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechRecoResult2 {
    pub base__: ISpeechRecoResult,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTextFeedback: unsafe extern "system" fn(this: *mut *mut Self, feedback: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, wassuccessful: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTextFeedback: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechRecoResultDispatch {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub RecoContext: unsafe extern "system" fn(this: *mut *mut Self, recocontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RecoContext: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Times: unsafe extern "system" fn(this: *mut *mut Self, times: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Times: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AudioFormat: unsafe extern "system" fn(this: *mut *mut Self, format: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AudioFormat: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AudioFormat: unsafe extern "system" fn(this: *mut *mut Self, format: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AudioFormat: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PhraseInfo: unsafe extern "system" fn(this: *mut *mut Self, phraseinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PhraseInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Alternates: unsafe extern "system" fn(this: *mut *mut Self, requestcount: i32, startelement: i32, elements: i32, alternates: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Alternates: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Audio: unsafe extern "system" fn(this: *mut *mut Self, startelement: i32, elements: i32, stream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Audio: usize,
    pub SpeakAudio: unsafe extern "system" fn(this: *mut *mut Self, startelement: i32, elements: i32, flags: SpeechVoiceSpeakFlags, streamnumber: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SaveToMemory: unsafe extern "system" fn(this: *mut *mut Self, resultblock: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SaveToMemory: usize,
    pub DiscardResultInfo: unsafe extern "system" fn(this: *mut *mut Self, valuetypes: SpeechDiscardType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetXMLResult: unsafe extern "system" fn(this: *mut *mut Self, options: SPXMLRESULTOPTIONS, presult: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetXMLResult: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetXMLErrorInfo: unsafe extern "system" fn(this: *mut *mut Self, linenumber: *mut i32, scriptline: *mut super::super::Foundation::BSTR, source: *mut super::super::Foundation::BSTR, description: *mut super::super::Foundation::BSTR, resultcode: *mut ::windows_sys::core::HRESULT, iserror: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetXMLErrorInfo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTextFeedback: unsafe extern "system" fn(this: *mut *mut Self, feedback: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, wassuccessful: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTextFeedback: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechRecoResultTimes {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub StreamTime: unsafe extern "system" fn(this: *mut *mut Self, time: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    StreamTime: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Length: unsafe extern "system" fn(this: *mut *mut Self, length: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Length: usize,
    pub TickCount: unsafe extern "system" fn(this: *mut *mut Self, tickcount: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OffsetFromStart: unsafe extern "system" fn(this: *mut *mut Self, offsetfromstart: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OffsetFromStart: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechRecognizer {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_Recognizer: unsafe extern "system" fn(this: *mut *mut Self, recognizer: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_Recognizer: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Recognizer: unsafe extern "system" fn(this: *mut *mut Self, recognizer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Recognizer: usize,
    pub SetAllowAudioInputFormatChangesOnNextSet: unsafe extern "system" fn(this: *mut *mut Self, allow: i16) -> ::windows_sys::core::HRESULT,
    pub AllowAudioInputFormatChangesOnNextSet: unsafe extern "system" fn(this: *mut *mut Self, allow: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AudioInput: unsafe extern "system" fn(this: *mut *mut Self, audioinput: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AudioInput: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AudioInput: unsafe extern "system" fn(this: *mut *mut Self, audioinput: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AudioInput: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AudioInputStream: unsafe extern "system" fn(this: *mut *mut Self, audioinputstream: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AudioInputStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AudioInputStream: unsafe extern "system" fn(this: *mut *mut Self, audioinputstream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AudioInputStream: usize,
    pub IsShared: unsafe extern "system" fn(this: *mut *mut Self, shared: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetState: unsafe extern "system" fn(this: *mut *mut Self, state: SpeechRecognizerState) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, state: *mut SpeechRecognizerState) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, status: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Status: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_Profile: unsafe extern "system" fn(this: *mut *mut Self, profile: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_Profile: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Profile: unsafe extern "system" fn(this: *mut *mut Self, profile: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Profile: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub EmulateRecognition: unsafe extern "system" fn(this: *mut *mut Self, textelements: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, elementdisplayattributes: *const super::super::System::Com::VARIANT, languageid: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    EmulateRecognition: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateRecoContext: unsafe extern "system" fn(this: *mut *mut Self, newcontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateRecoContext: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFormat: unsafe extern "system" fn(this: *mut *mut Self, r#type: SpeechFormatType, format: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFormat: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPropertyNumber: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: i32, supported: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPropertyNumber: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPropertyNumber: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: *mut i32, supported: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPropertyNumber: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPropertyString: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, supported: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPropertyString: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPropertyString: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: *mut super::super::Foundation::BSTR, supported: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPropertyString: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub IsUISupported: unsafe extern "system" fn(this: *mut *mut Self, typeofui: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, extradata: *const super::super::System::Com::VARIANT, supported: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    IsUISupported: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DisplayUI: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: i32, title: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, typeofui: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, extradata: *const super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DisplayUI: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetRecognizers: unsafe extern "system" fn(this: *mut *mut Self, requiredattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, optionalattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objecttokens: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetRecognizers: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetAudioInputs: unsafe extern "system" fn(this: *mut *mut Self, requiredattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, optionalattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objecttokens: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetAudioInputs: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetProfiles: unsafe extern "system" fn(this: *mut *mut Self, requiredattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, optionalattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objecttokens: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetProfiles: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechRecognizerStatus {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub AudioStatus: unsafe extern "system" fn(this: *mut *mut Self, audiostatus: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AudioStatus: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CurrentStreamPosition: unsafe extern "system" fn(this: *mut *mut Self, pcurrentstreampos: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CurrentStreamPosition: usize,
    pub CurrentStreamNumber: unsafe extern "system" fn(this: *mut *mut Self, streamnumber: *mut i32) -> ::windows_sys::core::HRESULT,
    pub NumberOfActiveRules: unsafe extern "system" fn(this: *mut *mut Self, numberofactiverules: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ClsidEngine: unsafe extern "system" fn(this: *mut *mut Self, clsidengine: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClsidEngine: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SupportedLanguages: unsafe extern "system" fn(this: *mut *mut Self, supportedlanguages: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SupportedLanguages: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechResourceLoader {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub LoadResource: unsafe extern "system" fn(this: *mut *mut Self, bstrresourceuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, falwaysreload: i16, pstream: *mut *mut ::core::ffi::c_void, pbstrmimetype: *mut super::super::Foundation::BSTR, pfmodified: *mut i16, pbstrredirecturl: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LoadResource: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLocalCopy: unsafe extern "system" fn(this: *mut *mut Self, bstrresourceuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrlocalpath: *mut super::super::Foundation::BSTR, pbstrmimetype: *mut super::super::Foundation::BSTR, pbstrredirecturl: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLocalCopy: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReleaseLocalCopy: unsafe extern "system" fn(this: *mut *mut Self, pbstrlocalpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReleaseLocalCopy: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechTextSelectionInformation {
    pub base__: super::super::System::Com::IDispatch,
    pub SetActiveOffset: unsafe extern "system" fn(this: *mut *mut Self, activeoffset: i32) -> ::windows_sys::core::HRESULT,
    pub ActiveOffset: unsafe extern "system" fn(this: *mut *mut Self, activeoffset: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetActiveLength: unsafe extern "system" fn(this: *mut *mut Self, activelength: i32) -> ::windows_sys::core::HRESULT,
    pub ActiveLength: unsafe extern "system" fn(this: *mut *mut Self, activelength: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetSelectionOffset: unsafe extern "system" fn(this: *mut *mut Self, selectionoffset: i32) -> ::windows_sys::core::HRESULT,
    pub SelectionOffset: unsafe extern "system" fn(this: *mut *mut Self, selectionoffset: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetSelectionLength: unsafe extern "system" fn(this: *mut *mut Self, selectionlength: i32) -> ::windows_sys::core::HRESULT,
    pub SelectionLength: unsafe extern "system" fn(this: *mut *mut Self, selectionlength: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechVoice {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, status: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Status: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Voice: unsafe extern "system" fn(this: *mut *mut Self, voice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Voice: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_Voice: unsafe extern "system" fn(this: *mut *mut Self, voice: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_Voice: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AudioOutput: unsafe extern "system" fn(this: *mut *mut Self, audiooutput: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AudioOutput: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AudioOutput: unsafe extern "system" fn(this: *mut *mut Self, audiooutput: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AudioOutput: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AudioOutputStream: unsafe extern "system" fn(this: *mut *mut Self, audiooutputstream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AudioOutputStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AudioOutputStream: unsafe extern "system" fn(this: *mut *mut Self, audiooutputstream: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AudioOutputStream: usize,
    pub Rate: unsafe extern "system" fn(this: *mut *mut Self, rate: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetRate: unsafe extern "system" fn(this: *mut *mut Self, rate: i32) -> ::windows_sys::core::HRESULT,
    pub Volume: unsafe extern "system" fn(this: *mut *mut Self, volume: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetVolume: unsafe extern "system" fn(this: *mut *mut Self, volume: i32) -> ::windows_sys::core::HRESULT,
    pub SetAllowAudioOutputFormatChangesOnNextSet: unsafe extern "system" fn(this: *mut *mut Self, allow: i16) -> ::windows_sys::core::HRESULT,
    pub AllowAudioOutputFormatChangesOnNextSet: unsafe extern "system" fn(this: *mut *mut Self, allow: *mut i16) -> ::windows_sys::core::HRESULT,
    pub EventInterests: unsafe extern "system" fn(this: *mut *mut Self, eventinterestflags: *mut SpeechVoiceEvents) -> ::windows_sys::core::HRESULT,
    pub SetEventInterests: unsafe extern "system" fn(this: *mut *mut Self, eventinterestflags: SpeechVoiceEvents) -> ::windows_sys::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut *mut Self, priority: SpeechVoicePriority) -> ::windows_sys::core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut *mut Self, priority: *mut SpeechVoicePriority) -> ::windows_sys::core::HRESULT,
    pub SetAlertBoundary: unsafe extern "system" fn(this: *mut *mut Self, boundary: SpeechVoiceEvents) -> ::windows_sys::core::HRESULT,
    pub AlertBoundary: unsafe extern "system" fn(this: *mut *mut Self, boundary: *mut SpeechVoiceEvents) -> ::windows_sys::core::HRESULT,
    pub SetSynchronousSpeakTimeout: unsafe extern "system" fn(this: *mut *mut Self, mstimeout: i32) -> ::windows_sys::core::HRESULT,
    pub SynchronousSpeakTimeout: unsafe extern "system" fn(this: *mut *mut Self, mstimeout: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Speak: unsafe extern "system" fn(this: *mut *mut Self, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: SpeechVoiceSpeakFlags, streamnumber: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Speak: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SpeakStream: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void, flags: SpeechVoiceSpeakFlags, streamnumber: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SpeakStream: usize,
    pub Pause: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, r#type: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, numitems: i32, numskipped: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Skip: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetVoices: unsafe extern "system" fn(this: *mut *mut Self, requiredattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, optionalattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objecttokens: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetVoices: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetAudioOutputs: unsafe extern "system" fn(this: *mut *mut Self, requiredattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, optionalattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objecttokens: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetAudioOutputs: usize,
    pub WaitUntilDone: unsafe extern "system" fn(this: *mut *mut Self, mstimeout: i32, done: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SpeakCompleteEvent: unsafe extern "system" fn(this: *mut *mut Self, handle: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub IsUISupported: unsafe extern "system" fn(this: *mut *mut Self, typeofui: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, extradata: *const super::super::System::Com::VARIANT, supported: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    IsUISupported: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DisplayUI: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: i32, title: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, typeofui: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, extradata: *const super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DisplayUI: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechVoiceStatus {
    pub base__: super::super::System::Com::IDispatch,
    pub CurrentStreamNumber: unsafe extern "system" fn(this: *mut *mut Self, streamnumber: *mut i32) -> ::windows_sys::core::HRESULT,
    pub LastStreamNumberQueued: unsafe extern "system" fn(this: *mut *mut Self, streamnumber: *mut i32) -> ::windows_sys::core::HRESULT,
    pub LastHResult: unsafe extern "system" fn(this: *mut *mut Self, hresult: *mut i32) -> ::windows_sys::core::HRESULT,
    pub RunningState: unsafe extern "system" fn(this: *mut *mut Self, state: *mut SpeechRunState) -> ::windows_sys::core::HRESULT,
    pub InputWordPosition: unsafe extern "system" fn(this: *mut *mut Self, position: *mut i32) -> ::windows_sys::core::HRESULT,
    pub InputWordLength: unsafe extern "system" fn(this: *mut *mut Self, length: *mut i32) -> ::windows_sys::core::HRESULT,
    pub InputSentencePosition: unsafe extern "system" fn(this: *mut *mut Self, position: *mut i32) -> ::windows_sys::core::HRESULT,
    pub InputSentenceLength: unsafe extern "system" fn(this: *mut *mut Self, length: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LastBookmark: unsafe extern "system" fn(this: *mut *mut Self, bookmark: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LastBookmark: usize,
    pub LastBookmarkId: unsafe extern "system" fn(this: *mut *mut Self, bookmarkid: *mut i32) -> ::windows_sys::core::HRESULT,
    pub PhonemeId: unsafe extern "system" fn(this: *mut *mut Self, phoneid: *mut i16) -> ::windows_sys::core::HRESULT,
    pub VisemeId: unsafe extern "system" fn(this: *mut *mut Self, visemeid: *mut i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechWaveFormatEx {
    pub base__: super::super::System::Com::IDispatch,
    pub FormatTag: unsafe extern "system" fn(this: *mut *mut Self, formattag: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetFormatTag: unsafe extern "system" fn(this: *mut *mut Self, formattag: i16) -> ::windows_sys::core::HRESULT,
    pub Channels: unsafe extern "system" fn(this: *mut *mut Self, channels: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetChannels: unsafe extern "system" fn(this: *mut *mut Self, channels: i16) -> ::windows_sys::core::HRESULT,
    pub SamplesPerSec: unsafe extern "system" fn(this: *mut *mut Self, samplespersec: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetSamplesPerSec: unsafe extern "system" fn(this: *mut *mut Self, samplespersec: i32) -> ::windows_sys::core::HRESULT,
    pub AvgBytesPerSec: unsafe extern "system" fn(this: *mut *mut Self, avgbytespersec: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetAvgBytesPerSec: unsafe extern "system" fn(this: *mut *mut Self, avgbytespersec: i32) -> ::windows_sys::core::HRESULT,
    pub BlockAlign: unsafe extern "system" fn(this: *mut *mut Self, blockalign: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetBlockAlign: unsafe extern "system" fn(this: *mut *mut Self, blockalign: i16) -> ::windows_sys::core::HRESULT,
    pub BitsPerSample: unsafe extern "system" fn(this: *mut *mut Self, bitspersample: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetBitsPerSample: unsafe extern "system" fn(this: *mut *mut Self, bitspersample: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ExtraData: unsafe extern "system" fn(this: *mut *mut Self, extradata: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ExtraData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetExtraData: unsafe extern "system" fn(this: *mut *mut Self, extradata: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetExtraData: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechXMLRecoResult {
    pub base__: ISpeechRecoResult,
    #[cfg(feature = "Win32_Foundation")]
    pub GetXMLResult: unsafe extern "system" fn(this: *mut *mut Self, options: SPXMLRESULTOPTIONS, presult: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetXMLResult: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetXMLErrorInfo: unsafe extern "system" fn(this: *mut *mut Self, linenumber: *mut i32, scriptline: *mut super::super::Foundation::BSTR, source: *mut super::super::Foundation::BSTR, description: *mut super::super::Foundation::BSTR, resultcode: *mut i32, iserror: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetXMLErrorInfo: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type PHONETICALPHABET = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const PA_Ipa: PHONETICALPHABET = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const PA_Ups: PHONETICALPHABET = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const PA_Sapi: PHONETICALPHABET = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAPI_ERROR_BASE: u32 = 20480u32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPADAPTATIONRELEVANCE = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPAR_Unknown: SPADAPTATIONRELEVANCE = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPAR_Low: SPADAPTATIONRELEVANCE = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPAR_Medium: SPADAPTATIONRELEVANCE = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPAR_High: SPADAPTATIONRELEVANCE = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPADAPTATIONSETTINGS = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPADS_Default: SPADAPTATIONSETTINGS = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPADS_CurrentRecognizer: SPADAPTATIONSETTINGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPADS_RecoProfile: SPADAPTATIONSETTINGS = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPADS_Immediate: SPADAPTATIONSETTINGS = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPADS_Reset: SPADAPTATIONSETTINGS = 8i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPADS_HighVolumeDataSource: SPADAPTATIONSETTINGS = 16i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPALTERNATESCLSID: &str = "AlternatesCLSID";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPAUDIOBUFFERINFO {
    pub ulMsMinNotification: u32,
    pub ulMsBufferSize: u32,
    pub ulMsEventBias: u32,
}
impl ::core::marker::Copy for SPAUDIOBUFFERINFO {}
impl ::core::clone::Clone for SPAUDIOBUFFERINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPAUDIOOPTIONS = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPAO_NONE: SPAUDIOOPTIONS = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPAO_RETAIN_AUDIO: SPAUDIOOPTIONS = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPAUDIOSTATE = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPAS_CLOSED: SPAUDIOSTATE = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPAS_STOP: SPAUDIOSTATE = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPAS_PAUSE: SPAUDIOSTATE = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPAS_RUN: SPAUDIOSTATE = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPAUDIOSTATUS {
    pub cbFreeBuffSpace: i32,
    pub cbNonBlockingIO: u32,
    pub State: SPAUDIOSTATE,
    pub CurSeekPos: u64,
    pub CurDevicePos: u64,
    pub dwAudioLevel: u32,
    pub dwReserved2: u32,
}
impl ::core::marker::Copy for SPAUDIOSTATUS {}
impl ::core::clone::Clone for SPAUDIOSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPBINARYGRAMMAR {
    pub ulTotalSerializedSize: u32,
}
impl ::core::marker::Copy for SPBINARYGRAMMAR {}
impl ::core::clone::Clone for SPBINARYGRAMMAR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPBOOKMARKOPTIONS = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPBO_NONE: SPBOOKMARKOPTIONS = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPBO_PAUSE: SPBOOKMARKOPTIONS = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPBO_AHEAD: SPBOOKMARKOPTIONS = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPBO_TIME_UNITS: SPBOOKMARKOPTIONS = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPCAT_APPLEXICONS: &str = "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\AppLexicons";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPCAT_AUDIOIN: &str = "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\AudioInput";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPCAT_AUDIOOUT: &str = "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\AudioOutput";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPCAT_PHONECONVERTERS: &str = "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\PhoneConverters";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPCAT_RECOGNIZERS: &str = "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\Recognizers";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPCAT_RECOPROFILES: &str = "HKEY_CURRENT_USER\\SOFTWARE\\Microsoft\\Speech\\RecoProfiles";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPCAT_TEXTNORMALIZERS: &str = "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\TextNormalizers";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPCAT_VOICES: &str = "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\Voices";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPCFGRULEATTRIBUTES = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRAF_TopLevel: SPCFGRULEATTRIBUTES = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRAF_Active: SPCFGRULEATTRIBUTES = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRAF_Export: SPCFGRULEATTRIBUTES = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRAF_Import: SPCFGRULEATTRIBUTES = 8i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRAF_Interpreter: SPCFGRULEATTRIBUTES = 16i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRAF_Dynamic: SPCFGRULEATTRIBUTES = 32i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRAF_Root: SPCFGRULEATTRIBUTES = 64i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRAF_AutoPause: SPCFGRULEATTRIBUTES = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRAF_UserDelimited: SPCFGRULEATTRIBUTES = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPCOMMITFLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPCF_NONE: SPCOMMITFLAGS = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPCF_ADD_TO_USER_LEXICON: SPCOMMITFLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPCF_DEFINITE_CORRECTION: SPCOMMITFLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPCONTEXTSTATE = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPCS_DISABLED: SPCONTEXTSTATE = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPCS_ENABLED: SPCONTEXTSTATE = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPCURRENT_USER_LEXICON_TOKEN_ID: &str = "HKEY_CURRENT_USER\\SOFTWARE\\Microsoft\\Speech\\CurrentUserLexicon";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPCURRENT_USER_SHORTCUT_TOKEN_ID: &str = "HKEY_CURRENT_USER\\SOFTWARE\\Microsoft\\Speech\\CurrentUserShortcut";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPDATAKEYLOCATION = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDKL_DefaultLocation: SPDATAKEYLOCATION = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDKL_CurrentUser: SPDATAKEYLOCATION = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDKL_LocalMachine: SPDATAKEYLOCATION = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDKL_CurrentConfig: SPDATAKEYLOCATION = 5i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDICTATION: &str = "*";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPDISPLAYPHRASE {
    pub ulNumTokens: u32,
    pub pTokens: *mut SPDISPLAYTOKEN,
}
impl ::core::marker::Copy for SPDISPLAYPHRASE {}
impl ::core::clone::Clone for SPDISPLAYPHRASE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPDISPLAYTOKEN {
    pub pszLexical: ::windows_sys::core::PCWSTR,
    pub pszDisplay: ::windows_sys::core::PCWSTR,
    pub bDisplayAttributes: u8,
}
impl ::core::marker::Copy for SPDISPLAYTOKEN {}
impl ::core::clone::Clone for SPDISPLAYTOKEN {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPDISPLYATTRIBUTES = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPAF_ONE_TRAILING_SPACE: SPDISPLYATTRIBUTES = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPAF_TWO_TRAILING_SPACES: SPDISPLYATTRIBUTES = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPAF_CONSUME_LEADING_SPACES: SPDISPLYATTRIBUTES = 8i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPAF_BUFFER_POSITION: SPDISPLYATTRIBUTES = 16i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPAF_ALL: SPDISPLYATTRIBUTES = 31i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPAF_USER_SPECIFIED: SPDISPLYATTRIBUTES = 128i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDUI_AddRemoveWord: &str = "AddRemoveWord";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDUI_AudioProperties: &str = "AudioProperties";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDUI_AudioVolume: &str = "AudioVolume";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDUI_EngineProperties: &str = "EngineProperties";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDUI_MicTraining: &str = "MicTraining";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDUI_RecoProfileProperties: &str = "RecoProfileProperties";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDUI_ShareData: &str = "ShareData";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDUI_Tutorial: &str = "Tutorial";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDUI_UserEnrollment: &str = "UserEnrollment";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDUI_UserTraining: &str = "UserTraining";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPEAKFLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPF_DEFAULT: SPEAKFLAGS = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPF_ASYNC: SPEAKFLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPF_PURGEBEFORESPEAK: SPEAKFLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPF_IS_FILENAME: SPEAKFLAGS = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPF_IS_XML: SPEAKFLAGS = 8i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPF_IS_NOT_XML: SPEAKFLAGS = 16i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPF_PERSIST_XML: SPEAKFLAGS = 32i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPF_NLP_SPEAK_PUNC: SPEAKFLAGS = 64i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPF_PARSE_SAPI: SPEAKFLAGS = 128i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPF_PARSE_SSML: SPEAKFLAGS = 256i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPF_PARSE_AUTODETECT: SPEAKFLAGS = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPF_NLP_MASK: SPEAKFLAGS = 64i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPF_PARSE_MASK: SPEAKFLAGS = 384i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPF_VOICE_MASK: SPEAKFLAGS = 511i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPF_UNUSED_FLAGS: SPEAKFLAGS = -512i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPENDSRSTREAMFLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPESF_NONE: SPENDSRSTREAMFLAGS = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPESF_STREAM_RELEASED: SPENDSRSTREAMFLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPESF_EMULATED: SPENDSRSTREAMFLAGS = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SPEVENT {
    pub _bitfield: i32,
    pub ulStreamNum: u32,
    pub ullAudioStreamOffset: u64,
    pub wParam: super::super::Foundation::WPARAM,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SPEVENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SPEVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPEVENTENUM = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_UNDEFINED: SPEVENTENUM = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_START_INPUT_STREAM: SPEVENTENUM = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_END_INPUT_STREAM: SPEVENTENUM = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_VOICE_CHANGE: SPEVENTENUM = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_TTS_BOOKMARK: SPEVENTENUM = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_WORD_BOUNDARY: SPEVENTENUM = 5i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_PHONEME: SPEVENTENUM = 6i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_SENTENCE_BOUNDARY: SPEVENTENUM = 7i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_VISEME: SPEVENTENUM = 8i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_TTS_AUDIO_LEVEL: SPEVENTENUM = 9i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_TTS_PRIVATE: SPEVENTENUM = 15i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_MIN_TTS: SPEVENTENUM = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_MAX_TTS: SPEVENTENUM = 15i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_END_SR_STREAM: SPEVENTENUM = 34i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_SOUND_START: SPEVENTENUM = 35i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_SOUND_END: SPEVENTENUM = 36i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_PHRASE_START: SPEVENTENUM = 37i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_RECOGNITION: SPEVENTENUM = 38i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_HYPOTHESIS: SPEVENTENUM = 39i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_SR_BOOKMARK: SPEVENTENUM = 40i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_PROPERTY_NUM_CHANGE: SPEVENTENUM = 41i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_PROPERTY_STRING_CHANGE: SPEVENTENUM = 42i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_FALSE_RECOGNITION: SPEVENTENUM = 43i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_INTERFERENCE: SPEVENTENUM = 44i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_REQUEST_UI: SPEVENTENUM = 45i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_RECO_STATE_CHANGE: SPEVENTENUM = 46i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_ADAPTATION: SPEVENTENUM = 47i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_START_SR_STREAM: SPEVENTENUM = 48i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_RECO_OTHER_CONTEXT: SPEVENTENUM = 49i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_SR_AUDIO_LEVEL: SPEVENTENUM = 50i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_SR_RETAINEDAUDIO: SPEVENTENUM = 51i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_SR_PRIVATE: SPEVENTENUM = 52i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_RESERVED4: SPEVENTENUM = 53i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_RESERVED5: SPEVENTENUM = 54i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_RESERVED6: SPEVENTENUM = 55i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_MIN_SR: SPEVENTENUM = 34i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_MAX_SR: SPEVENTENUM = 55i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_RESERVED1: SPEVENTENUM = 30i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_RESERVED2: SPEVENTENUM = 33i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_RESERVED3: SPEVENTENUM = 63i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SPEVENTEX {
    pub _bitfield: i32,
    pub ulStreamNum: u32,
    pub ullAudioStreamOffset: u64,
    pub wParam: super::super::Foundation::WPARAM,
    pub lParam: super::super::Foundation::LPARAM,
    pub ullAudioTimeOffset: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SPEVENTEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SPEVENTEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPEVENTLPARAMTYPE = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPET_LPARAM_IS_UNDEFINED: SPEVENTLPARAMTYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPET_LPARAM_IS_TOKEN: SPEVENTLPARAMTYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPET_LPARAM_IS_OBJECT: SPEVENTLPARAMTYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPET_LPARAM_IS_POINTER: SPEVENTLPARAMTYPE = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPET_LPARAM_IS_STRING: SPEVENTLPARAMTYPE = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPEVENTSOURCEINFO {
    pub ullEventInterest: u64,
    pub ullQueuedInterest: u64,
    pub ulCount: u32,
}
impl ::core::marker::Copy for SPEVENTSOURCEINFO {}
impl ::core::clone::Clone for SPEVENTSOURCEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPFILEMODE = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPFM_OPEN_READONLY: SPFILEMODE = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPFM_OPEN_READWRITE: SPFILEMODE = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPFM_CREATE: SPFILEMODE = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPFM_CREATE_ALWAYS: SPFILEMODE = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPFM_NUM_MODES: SPFILEMODE = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPGRAMMAROPTIONS = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPGO_SAPI: SPGRAMMAROPTIONS = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPGO_SRGS: SPGRAMMAROPTIONS = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPGO_UPS: SPGRAMMAROPTIONS = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPGO_SRGS_MS_SCRIPT: SPGRAMMAROPTIONS = 8i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPGO_SRGS_W3C_SCRIPT: SPGRAMMAROPTIONS = 256i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPGO_SRGS_STG_SCRIPT: SPGRAMMAROPTIONS = 512i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPGO_SRGS_SCRIPT: SPGRAMMAROPTIONS = 778i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPGO_FILE: SPGRAMMAROPTIONS = 16i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPGO_HTTP: SPGRAMMAROPTIONS = 32i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPGO_RES: SPGRAMMAROPTIONS = 64i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPGO_OBJECT: SPGRAMMAROPTIONS = 128i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPGO_DEFAULT: SPGRAMMAROPTIONS = 1019i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPGO_ALL: SPGRAMMAROPTIONS = 1023i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPGRAMMARSTATE = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPGS_DISABLED: SPGRAMMARSTATE = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPGS_ENABLED: SPGRAMMARSTATE = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPGS_EXCLUSIVE: SPGRAMMARSTATE = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPGRAMMARWORDTYPE = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPWT_DISPLAY: SPGRAMMARWORDTYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPWT_LEXICAL: SPGRAMMARWORDTYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPWT_PRONUNCIATION: SPGRAMMARWORDTYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPWT_LEXICAL_NO_SPECIAL_CHARS: SPGRAMMARWORDTYPE = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPINFDICTATION: &str = "*+";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPINTERFERENCE = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPINTERFERENCE_NONE: SPINTERFERENCE = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPINTERFERENCE_NOISE: SPINTERFERENCE = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPINTERFERENCE_NOSIGNAL: SPINTERFERENCE = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPINTERFERENCE_TOOLOUD: SPINTERFERENCE = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPINTERFERENCE_TOOQUIET: SPINTERFERENCE = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPINTERFERENCE_TOOFAST: SPINTERFERENCE = 5i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPINTERFERENCE_TOOSLOW: SPINTERFERENCE = 6i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPINTERFERENCE_LATENCY_WARNING: SPINTERFERENCE = 7i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPINTERFERENCE_LATENCY_TRUNCATE_BEGIN: SPINTERFERENCE = 8i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPINTERFERENCE_LATENCY_TRUNCATE_END: SPINTERFERENCE = 9i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPLEXICONTYPE = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_USER: SPLEXICONTYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_APP: SPLEXICONTYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_VENDORLEXICON: SPLEXICONTYPE = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_LETTERTOSOUND: SPLEXICONTYPE = 8i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_MORPHOLOGY: SPLEXICONTYPE = 16i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_RESERVED4: SPLEXICONTYPE = 32i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_USER_SHORTCUT: SPLEXICONTYPE = 64i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_RESERVED6: SPLEXICONTYPE = 128i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_RESERVED7: SPLEXICONTYPE = 256i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_RESERVED8: SPLEXICONTYPE = 512i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_RESERVED9: SPLEXICONTYPE = 1024i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_RESERVED10: SPLEXICONTYPE = 2048i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE1: SPLEXICONTYPE = 4096i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE2: SPLEXICONTYPE = 8192i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE3: SPLEXICONTYPE = 16384i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE4: SPLEXICONTYPE = 32768i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE5: SPLEXICONTYPE = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE6: SPLEXICONTYPE = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE7: SPLEXICONTYPE = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE8: SPLEXICONTYPE = 524288i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE9: SPLEXICONTYPE = 1048576i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE10: SPLEXICONTYPE = 2097152i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE11: SPLEXICONTYPE = 4194304i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE12: SPLEXICONTYPE = 8388608i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE13: SPLEXICONTYPE = 16777216i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE14: SPLEXICONTYPE = 33554432i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE15: SPLEXICONTYPE = 67108864i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE16: SPLEXICONTYPE = 134217728i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE17: SPLEXICONTYPE = 268435456i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE18: SPLEXICONTYPE = 536870912i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE19: SPLEXICONTYPE = 1073741824i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE20: SPLEXICONTYPE = -2147483648i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPLOADOPTIONS = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPLO_STATIC: SPLOADOPTIONS = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPLO_DYNAMIC: SPLOADOPTIONS = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPMATCHINGMODE = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const AllWords: SPMATCHINGMODE = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const Subsequence: SPMATCHINGMODE = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const OrderedSubset: SPMATCHINGMODE = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SubsequenceContentRequired: SPMATCHINGMODE = 5i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const OrderedSubsetContentRequired: SPMATCHINGMODE = 7i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPMMSYS_AUDIO_IN_TOKEN_ID: &str = "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\AudioInput\\TokenEnums\\MMAudioIn\\";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPMMSYS_AUDIO_OUT_TOKEN_ID: &str = "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\AudioOutput\\TokenEnums\\MMAudioOut\\";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPNORMALIZATIONLIST {
    pub ulSize: u32,
    pub ppszzNormalizedList: *mut *mut u16,
}
impl ::core::marker::Copy for SPNORMALIZATIONLIST {}
impl ::core::clone::Clone for SPNORMALIZATIONLIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SPNOTIFYCALLBACK = ::core::option::Option<unsafe extern "system" fn(wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM)>;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPPARTOFSPEECH = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPS_NotOverriden: SPPARTOFSPEECH = -1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPS_Unknown: SPPARTOFSPEECH = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPS_Noun: SPPARTOFSPEECH = 4096i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPS_Verb: SPPARTOFSPEECH = 8192i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPS_Modifier: SPPARTOFSPEECH = 12288i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPS_Function: SPPARTOFSPEECH = 16384i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPS_Interjection: SPPARTOFSPEECH = 20480i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPS_Noncontent: SPPARTOFSPEECH = 24576i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPS_LMA: SPPARTOFSPEECH = 28672i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPS_SuppressWord: SPPARTOFSPEECH = 61440i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct SPPHRASE {
    pub __AnonymousBase_sapi53_L5821_C34: SPPHRASE_50,
    pub pSML: ::windows_sys::core::PWSTR,
    pub pSemanticErrorInfo: *mut SPSEMANTICERRORINFO,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for SPPHRASE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for SPPHRASE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPPHRASEELEMENT {
    pub ulAudioTimeOffset: u32,
    pub ulAudioSizeTime: u32,
    pub ulAudioStreamOffset: u32,
    pub ulAudioSizeBytes: u32,
    pub ulRetainedStreamOffset: u32,
    pub ulRetainedSizeBytes: u32,
    pub pszDisplayText: ::windows_sys::core::PCWSTR,
    pub pszLexicalForm: ::windows_sys::core::PCWSTR,
    pub pszPronunciation: *const u16,
    pub bDisplayAttributes: u8,
    pub RequiredConfidence: i8,
    pub ActualConfidence: i8,
    pub Reserved: u8,
    pub SREngineConfidence: f32,
}
impl ::core::marker::Copy for SPPHRASEELEMENT {}
impl ::core::clone::Clone for SPPHRASEELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct SPPHRASEPROPERTY {
    pub pszName: ::windows_sys::core::PCWSTR,
    pub Anonymous: SPPHRASEPROPERTY_0,
    pub pszValue: ::windows_sys::core::PCWSTR,
    pub vValue: super::super::System::Com::VARIANT,
    pub ulFirstElement: u32,
    pub ulCountOfElements: u32,
    pub pNextSibling: *const SPPHRASEPROPERTY,
    pub pFirstChild: *const SPPHRASEPROPERTY,
    pub SREngineConfidence: f32,
    pub Confidence: i8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for SPPHRASEPROPERTY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for SPPHRASEPROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub union SPPHRASEPROPERTY_0 {
    pub ulId: u32,
    pub Anonymous: SPPHRASEPROPERTY_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for SPPHRASEPROPERTY_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for SPPHRASEPROPERTY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct SPPHRASEPROPERTY_0_0 {
    pub bType: u8,
    pub bReserved: u8,
    pub usArrayIndex: u16,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for SPPHRASEPROPERTY_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for SPPHRASEPROPERTY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPPHRASEPROPERTYUNIONTYPE = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPPUT_UNUSED: SPPHRASEPROPERTYUNIONTYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPPUT_ARRAY_INDEX: SPPHRASEPROPERTYUNIONTYPE = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPPHRASEREPLACEMENT {
    pub bDisplayAttributes: u8,
    pub pszReplacementText: ::windows_sys::core::PCWSTR,
    pub ulFirstElement: u32,
    pub ulCountOfElements: u32,
}
impl ::core::marker::Copy for SPPHRASEREPLACEMENT {}
impl ::core::clone::Clone for SPPHRASEREPLACEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPPHRASERNG = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPR_ALL_ELEMENTS: SPPHRASERNG = -1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPPHRASERULE {
    pub pszName: ::windows_sys::core::PCWSTR,
    pub ulId: u32,
    pub ulFirstElement: u32,
    pub ulCountOfElements: u32,
    pub pNextSibling: *const SPPHRASERULE,
    pub pFirstChild: *const SPPHRASERULE,
    pub SREngineConfidence: f32,
    pub Confidence: i8,
}
impl ::core::marker::Copy for SPPHRASERULE {}
impl ::core::clone::Clone for SPPHRASERULE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct SPPHRASE_50 {
    pub cbSize: u32,
    pub LangID: u16,
    pub wHomophoneGroupId: u16,
    pub ullGrammarID: u64,
    pub ftStartTime: u64,
    pub ullAudioStreamPosition: u64,
    pub ulAudioSizeBytes: u32,
    pub ulRetainedSizeBytes: u32,
    pub ulAudioSizeTime: u32,
    pub Rule: SPPHRASERULE,
    pub pProperties: *const SPPHRASEPROPERTY,
    pub pElements: *const SPPHRASEELEMENT,
    pub cReplacements: u32,
    pub pReplacements: *const SPPHRASEREPLACEMENT,
    pub SREngineID: ::windows_sys::core::GUID,
    pub ulSREnginePrivateDataSize: u32,
    pub pSREnginePrivateData: *const u8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for SPPHRASE_50 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for SPPHRASE_50 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPPRONUNCIATIONFLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const ePRONFLAG_USED: SPPRONUNCIATIONFLAGS = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct SPPROPERTYINFO {
    pub pszName: ::windows_sys::core::PCWSTR,
    pub ulId: u32,
    pub pszValue: ::windows_sys::core::PCWSTR,
    pub vValue: super::super::System::Com::VARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for SPPROPERTYINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for SPPROPERTYINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPROP_ADAPTATION_ON: &str = "AdaptationOn";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPROP_COMPLEX_RESPONSE_SPEED: &str = "ComplexResponseSpeed";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPROP_HIGH_CONFIDENCE_THRESHOLD: &str = "HighConfidenceThreshold";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPROP_LOW_CONFIDENCE_THRESHOLD: &str = "LowConfidenceThreshold";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPROP_NORMAL_CONFIDENCE_THRESHOLD: &str = "NormalConfidenceThreshold";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPROP_PERSISTED_BACKGROUND_ADAPTATION: &str = "PersistedBackgroundAdaptation";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPROP_PERSISTED_LANGUAGE_MODEL_ADAPTATION: &str = "PersistedLanguageModelAdaptation";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPROP_RESOURCE_USAGE: &str = "ResourceUsage";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPROP_RESPONSE_SPEED: &str = "ResponseSpeed";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPROP_UX_IS_LISTENING: &str = "UXIsListening";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPRECOCONTEXTSTATUS {
    pub eInterference: SPINTERFERENCE,
    pub szRequestTypeOfUI: [u16; 255],
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
impl ::core::marker::Copy for SPRECOCONTEXTSTATUS {}
impl ::core::clone::Clone for SPRECOCONTEXTSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPRECOEVENTFLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPREF_AutoPause: SPRECOEVENTFLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPREF_Emulated: SPRECOEVENTFLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPREF_SMLTimeout: SPRECOEVENTFLAGS = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPREF_ExtendableParse: SPRECOEVENTFLAGS = 8i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPREF_ReSent: SPRECOEVENTFLAGS = 16i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPREF_Hypothesis: SPRECOEVENTFLAGS = 32i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPREF_FalseRecognition: SPRECOEVENTFLAGS = 64i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRECOEXTENSION: &str = "RecoExtension";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPRECOGNIZERSTATUS {
    pub AudioStatus: SPAUDIOSTATUS,
    pub ullRecognitionStreamPos: u64,
    pub ulStreamNumber: u32,
    pub ulNumActive: u32,
    pub clsidEngine: ::windows_sys::core::GUID,
    pub cLangIDs: u32,
    pub aLangID: [u16; 20],
    pub ullRecognitionStreamTime: u64,
}
impl ::core::marker::Copy for SPRECOGNIZERSTATUS {}
impl ::core::clone::Clone for SPRECOGNIZERSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SPRECORESULTTIMES {
    pub ftStreamTime: super::super::Foundation::FILETIME,
    pub ullLength: u64,
    pub dwTickCount: u32,
    pub ullStart: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SPRECORESULTTIMES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SPRECORESULTTIMES {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPRECOSTATE = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRST_INACTIVE: SPRECOSTATE = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRST_ACTIVE: SPRECOSTATE = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRST_ACTIVE_ALWAYS: SPRECOSTATE = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRST_INACTIVE_WITH_PURGE: SPRECOSTATE = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRST_NUM_STATES: SPRECOSTATE = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPREG_LOCAL_MACHINE_ROOT: &str = "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPREG_SAFE_USER_TOKENS: &str = "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\UserTokens";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPREG_USER_ROOT: &str = "HKEY_CURRENT_USER\\SOFTWARE\\Microsoft\\Speech";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRP_NORMAL: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPRULE {
    pub pszRuleName: ::windows_sys::core::PCWSTR,
    pub ulRuleId: u32,
    pub dwAttributes: u32,
}
impl ::core::marker::Copy for SPRULE {}
impl ::core::clone::Clone for SPRULE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPRULESTATE = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRS_INACTIVE: SPRULESTATE = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRS_ACTIVE: SPRULESTATE = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRS_ACTIVE_WITH_AUTO_PAUSE: SPRULESTATE = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRS_ACTIVE_USER_DELIMITED: SPRULESTATE = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPRUNSTATE = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRS_DONE: SPRUNSTATE = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRS_IS_SPEAKING: SPRUNSTATE = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPSEMANTICERRORINFO {
    pub ulLineNumber: u32,
    pub pszScriptLine: ::windows_sys::core::PWSTR,
    pub pszSource: ::windows_sys::core::PWSTR,
    pub pszDescription: ::windows_sys::core::PWSTR,
    pub hrResultCode: ::windows_sys::core::HRESULT,
}
impl ::core::marker::Copy for SPSEMANTICERRORINFO {}
impl ::core::clone::Clone for SPSEMANTICERRORINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPSEMANTICFORMAT = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSMF_SAPI_PROPERTIES: SPSEMANTICFORMAT = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSMF_SRGS_SEMANTICINTERPRETATION_MS: SPSEMANTICFORMAT = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSMF_SRGS_SAPIPROPERTIES: SPSEMANTICFORMAT = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSMF_UPS: SPSEMANTICFORMAT = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSMF_SRGS_SEMANTICINTERPRETATION_W3C: SPSEMANTICFORMAT = 8i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPSERIALIZEDEVENT {
    pub _bitfield: i32,
    pub ulStreamNum: u32,
    pub ullAudioStreamOffset: u64,
    pub SerializedwParam: u32,
    pub SerializedlParam: i32,
}
impl ::core::marker::Copy for SPSERIALIZEDEVENT {}
impl ::core::clone::Clone for SPSERIALIZEDEVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPSERIALIZEDEVENT64 {
    pub _bitfield: i32,
    pub ulStreamNum: u32,
    pub ullAudioStreamOffset: u64,
    pub SerializedwParam: u64,
    pub SerializedlParam: i64,
}
impl ::core::marker::Copy for SPSERIALIZEDEVENT64 {}
impl ::core::clone::Clone for SPSERIALIZEDEVENT64 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPSERIALIZEDPHRASE {
    pub ulSerializedSize: u32,
}
impl ::core::marker::Copy for SPSERIALIZEDPHRASE {}
impl ::core::clone::Clone for SPSERIALIZEDPHRASE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPSERIALIZEDRESULT {
    pub ulSerializedSize: u32,
}
impl ::core::marker::Copy for SPSERIALIZEDRESULT {}
impl ::core::clone::Clone for SPSERIALIZEDRESULT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPSHORTCUTPAIR {
    pub pNextSHORTCUTPAIR: *mut SPSHORTCUTPAIR,
    pub LangID: u16,
    pub shType: SPSHORTCUTTYPE,
    pub pszDisplay: ::windows_sys::core::PWSTR,
    pub pszSpoken: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for SPSHORTCUTPAIR {}
impl ::core::clone::Clone for SPSHORTCUTPAIR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPSHORTCUTPAIRLIST {
    pub ulSize: u32,
    pub pvBuffer: *mut u8,
    pub pFirstShortcutPair: *mut SPSHORTCUTPAIR,
}
impl ::core::marker::Copy for SPSHORTCUTPAIRLIST {}
impl ::core::clone::Clone for SPSHORTCUTPAIRLIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPSHORTCUTTYPE = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSHT_NotOverriden: SPSHORTCUTTYPE = -1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSHT_Unknown: SPSHORTCUTTYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSHT_EMAIL: SPSHORTCUTTYPE = 4096i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSHT_OTHER: SPSHORTCUTTYPE = 8192i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPS_RESERVED1: SPSHORTCUTTYPE = 12288i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPS_RESERVED2: SPSHORTCUTTYPE = 16384i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPS_RESERVED3: SPSHORTCUTTYPE = 20480i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPS_RESERVED4: SPSHORTCUTTYPE = 61440i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPSTATEHANDLE__ {
    pub unused: i32,
}
impl ::core::marker::Copy for SPSTATEHANDLE__ {}
impl ::core::clone::Clone for SPSTATEHANDLE__ {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPSTREAMFORMAT = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_Default: SPSTREAMFORMAT = -1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_NoAssignedFormat: SPSTREAMFORMAT = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_Text: SPSTREAMFORMAT = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_NonStandardFormat: SPSTREAMFORMAT = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_ExtendedAudioFormat: SPSTREAMFORMAT = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_8kHz8BitMono: SPSTREAMFORMAT = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_8kHz8BitStereo: SPSTREAMFORMAT = 5i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_8kHz16BitMono: SPSTREAMFORMAT = 6i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_8kHz16BitStereo: SPSTREAMFORMAT = 7i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_11kHz8BitMono: SPSTREAMFORMAT = 8i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_11kHz8BitStereo: SPSTREAMFORMAT = 9i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_11kHz16BitMono: SPSTREAMFORMAT = 10i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_11kHz16BitStereo: SPSTREAMFORMAT = 11i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_12kHz8BitMono: SPSTREAMFORMAT = 12i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_12kHz8BitStereo: SPSTREAMFORMAT = 13i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_12kHz16BitMono: SPSTREAMFORMAT = 14i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_12kHz16BitStereo: SPSTREAMFORMAT = 15i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_16kHz8BitMono: SPSTREAMFORMAT = 16i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_16kHz8BitStereo: SPSTREAMFORMAT = 17i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_16kHz16BitMono: SPSTREAMFORMAT = 18i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_16kHz16BitStereo: SPSTREAMFORMAT = 19i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_22kHz8BitMono: SPSTREAMFORMAT = 20i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_22kHz8BitStereo: SPSTREAMFORMAT = 21i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_22kHz16BitMono: SPSTREAMFORMAT = 22i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_22kHz16BitStereo: SPSTREAMFORMAT = 23i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_24kHz8BitMono: SPSTREAMFORMAT = 24i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_24kHz8BitStereo: SPSTREAMFORMAT = 25i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_24kHz16BitMono: SPSTREAMFORMAT = 26i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_24kHz16BitStereo: SPSTREAMFORMAT = 27i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_32kHz8BitMono: SPSTREAMFORMAT = 28i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_32kHz8BitStereo: SPSTREAMFORMAT = 29i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_32kHz16BitMono: SPSTREAMFORMAT = 30i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_32kHz16BitStereo: SPSTREAMFORMAT = 31i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_44kHz8BitMono: SPSTREAMFORMAT = 32i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_44kHz8BitStereo: SPSTREAMFORMAT = 33i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_44kHz16BitMono: SPSTREAMFORMAT = 34i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_44kHz16BitStereo: SPSTREAMFORMAT = 35i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_48kHz8BitMono: SPSTREAMFORMAT = 36i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_48kHz8BitStereo: SPSTREAMFORMAT = 37i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_48kHz16BitMono: SPSTREAMFORMAT = 38i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_48kHz16BitStereo: SPSTREAMFORMAT = 39i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_TrueSpeech_8kHz1BitMono: SPSTREAMFORMAT = 40i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_CCITT_ALaw_8kHzMono: SPSTREAMFORMAT = 41i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_CCITT_ALaw_8kHzStereo: SPSTREAMFORMAT = 42i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_CCITT_ALaw_11kHzMono: SPSTREAMFORMAT = 43i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_CCITT_ALaw_11kHzStereo: SPSTREAMFORMAT = 44i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_CCITT_ALaw_22kHzMono: SPSTREAMFORMAT = 45i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_CCITT_ALaw_22kHzStereo: SPSTREAMFORMAT = 46i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_CCITT_ALaw_44kHzMono: SPSTREAMFORMAT = 47i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_CCITT_ALaw_44kHzStereo: SPSTREAMFORMAT = 48i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_CCITT_uLaw_8kHzMono: SPSTREAMFORMAT = 49i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_CCITT_uLaw_8kHzStereo: SPSTREAMFORMAT = 50i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_CCITT_uLaw_11kHzMono: SPSTREAMFORMAT = 51i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_CCITT_uLaw_11kHzStereo: SPSTREAMFORMAT = 52i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_CCITT_uLaw_22kHzMono: SPSTREAMFORMAT = 53i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_CCITT_uLaw_22kHzStereo: SPSTREAMFORMAT = 54i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_CCITT_uLaw_44kHzMono: SPSTREAMFORMAT = 55i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_CCITT_uLaw_44kHzStereo: SPSTREAMFORMAT = 56i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_ADPCM_8kHzMono: SPSTREAMFORMAT = 57i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_ADPCM_8kHzStereo: SPSTREAMFORMAT = 58i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_ADPCM_11kHzMono: SPSTREAMFORMAT = 59i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_ADPCM_11kHzStereo: SPSTREAMFORMAT = 60i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_ADPCM_22kHzMono: SPSTREAMFORMAT = 61i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_ADPCM_22kHzStereo: SPSTREAMFORMAT = 62i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_ADPCM_44kHzMono: SPSTREAMFORMAT = 63i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_ADPCM_44kHzStereo: SPSTREAMFORMAT = 64i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_GSM610_8kHzMono: SPSTREAMFORMAT = 65i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_GSM610_11kHzMono: SPSTREAMFORMAT = 66i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_GSM610_22kHzMono: SPSTREAMFORMAT = 67i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_GSM610_44kHzMono: SPSTREAMFORMAT = 68i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_NUM_FORMATS: SPSTREAMFORMAT = 69i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPTEXTSELECTIONINFO {
    pub ulStartActiveOffset: u32,
    pub cchActiveChars: u32,
    pub ulStartSelection: u32,
    pub cchSelection: u32,
}
impl ::core::marker::Copy for SPTEXTSELECTIONINFO {}
impl ::core::clone::Clone for SPTEXTSELECTIONINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPTOKENKEY_ATTRIBUTES: &str = "Attributes";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPTOKENKEY_AUDIO_LATENCY_TRUNCATE: &str = "LatencyTruncateThreshold";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPTOKENKEY_AUDIO_LATENCY_UPDATE_INTERVAL: &str = "LatencyUpdateInterval";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPTOKENKEY_AUDIO_LATENCY_WARNING: &str = "LatencyWarningThreshold";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPTOKENKEY_FILES: &str = "Files";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPTOKENKEY_RETAINEDAUDIO: &str = "SecondsPerRetainedAudioEvent";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPTOKENKEY_UI: &str = "UI";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPTOKENVALUE_CLSID: &str = "CLSID";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPTOPIC_SPELLING: &str = "Spelling";
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPVACTIONS = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPVA_Speak: SPVACTIONS = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPVA_Silence: SPVACTIONS = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPVA_Pronounce: SPVACTIONS = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPVA_Bookmark: SPVACTIONS = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPVA_SpellOut: SPVACTIONS = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPVA_Section: SPVACTIONS = 5i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPVA_ParseUnknownTag: SPVACTIONS = 6i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPVALUETYPE = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDF_PROPERTY: SPVALUETYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDF_REPLACEMENT: SPVALUETYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDF_RULE: SPVALUETYPE = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDF_DISPLAYTEXT: SPVALUETYPE = 8i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDF_LEXICALFORM: SPVALUETYPE = 16i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDF_PRONUNCIATION: SPVALUETYPE = 32i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDF_AUDIO: SPVALUETYPE = 64i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDF_ALTERNATES: SPVALUETYPE = 128i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDF_ALL: SPVALUETYPE = 255i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPVCONTEXT {
    pub pCategory: ::windows_sys::core::PCWSTR,
    pub pBefore: ::windows_sys::core::PCWSTR,
    pub pAfter: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for SPVCONTEXT {}
impl ::core::clone::Clone for SPVCONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPVFEATURE = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPVFEATURE_STRESSED: SPVFEATURE = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPVFEATURE_EMPHASIS: SPVFEATURE = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPVISEMES = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_0: SPVISEMES = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_1: SPVISEMES = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_2: SPVISEMES = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_3: SPVISEMES = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_4: SPVISEMES = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_5: SPVISEMES = 5i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_6: SPVISEMES = 6i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_7: SPVISEMES = 7i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_8: SPVISEMES = 8i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_9: SPVISEMES = 9i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_10: SPVISEMES = 10i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_11: SPVISEMES = 11i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_12: SPVISEMES = 12i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_13: SPVISEMES = 13i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_14: SPVISEMES = 14i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_15: SPVISEMES = 15i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_16: SPVISEMES = 16i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_17: SPVISEMES = 17i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_18: SPVISEMES = 18i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_19: SPVISEMES = 19i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_20: SPVISEMES = 20i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_21: SPVISEMES = 21i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPVLIMITS = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPMIN_VOLUME: SPVLIMITS = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPMAX_VOLUME: SPVLIMITS = 100i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPMIN_RATE: SPVLIMITS = -10i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPMAX_RATE: SPVLIMITS = 10i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPVOICECATEGORY_TTSRATE: &str = "DefaultTTSRate";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPVOICESTATUS {
    pub ulCurrentStream: u32,
    pub ulLastStreamQueued: u32,
    pub hrLastResult: ::windows_sys::core::HRESULT,
    pub dwRunningState: u32,
    pub ulInputWordPos: u32,
    pub ulInputWordLen: u32,
    pub ulInputSentPos: u32,
    pub ulInputSentLen: u32,
    pub lBookmarkId: i32,
    pub PhonemeId: u16,
    pub VisemeId: SPVISEMES,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
impl ::core::marker::Copy for SPVOICESTATUS {}
impl ::core::clone::Clone for SPVOICESTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPVPITCH {
    pub MiddleAdj: i32,
    pub RangeAdj: i32,
}
impl ::core::marker::Copy for SPVPITCH {}
impl ::core::clone::Clone for SPVPITCH {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPVPRIORITY = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPVPRI_NORMAL: SPVPRIORITY = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPVPRI_ALERT: SPVPRIORITY = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPVPRI_OVER: SPVPRIORITY = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPVSTATE {
    pub eAction: SPVACTIONS,
    pub LangID: u16,
    pub wReserved: u16,
    pub EmphAdj: i32,
    pub RateAdj: i32,
    pub Volume: u32,
    pub PitchAdj: SPVPITCH,
    pub SilenceMSecs: u32,
    pub pPhoneIds: *mut u16,
    pub ePartOfSpeech: SPPARTOFSPEECH,
    pub Context: SPVCONTEXT,
}
impl ::core::marker::Copy for SPVSTATE {}
impl ::core::clone::Clone for SPVSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPWAVEFORMATTYPE = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPWF_INPUT: SPWAVEFORMATTYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPWF_SRENGINE: SPWAVEFORMATTYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPWILDCARD: &str = "...";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPWORD {
    pub pNextWord: *mut SPWORD,
    pub LangID: u16,
    pub wReserved: u16,
    pub eWordType: SPWORDTYPE,
    pub pszWord: ::windows_sys::core::PWSTR,
    pub pFirstWordPronunciation: *mut SPWORDPRONUNCIATION,
}
impl ::core::marker::Copy for SPWORD {}
impl ::core::clone::Clone for SPWORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPWORDLIST {
    pub ulSize: u32,
    pub pvBuffer: *mut u8,
    pub pFirstWord: *mut SPWORD,
}
impl ::core::marker::Copy for SPWORDLIST {}
impl ::core::clone::Clone for SPWORDLIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPWORDPRONOUNCEABLE = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPWP_UNKNOWN_WORD_UNPRONOUNCEABLE: SPWORDPRONOUNCEABLE = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPWP_UNKNOWN_WORD_PRONOUNCEABLE: SPWORDPRONOUNCEABLE = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPWP_KNOWN_WORD_PRONOUNCEABLE: SPWORDPRONOUNCEABLE = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPWORDPRONUNCIATION {
    pub pNextWordPronunciation: *mut SPWORDPRONUNCIATION,
    pub eLexiconType: SPLEXICONTYPE,
    pub LangID: u16,
    pub wPronunciationFlags: u16,
    pub ePartOfSpeech: SPPARTOFSPEECH,
    pub szPronunciation: [u16; 1],
}
impl ::core::marker::Copy for SPWORDPRONUNCIATION {}
impl ::core::clone::Clone for SPWORDPRONUNCIATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPWORDPRONUNCIATIONLIST {
    pub ulSize: u32,
    pub pvBuffer: *mut u8,
    pub pFirstWordPronunciation: *mut SPWORDPRONUNCIATION,
}
impl ::core::marker::Copy for SPWORDPRONUNCIATIONLIST {}
impl ::core::clone::Clone for SPWORDPRONUNCIATIONLIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPWORDTYPE = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eWORDTYPE_ADDED: SPWORDTYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eWORDTYPE_DELETED: SPWORDTYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SPXMLRESULTOPTIONS = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPXRO_SML: SPXMLRESULTOPTIONS = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPXRO_Alternates_SML: SPXMLRESULTOPTIONS = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_EMULATE_RESULT: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_LOW_CONFIDENCE: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_MAX_LANGIDS: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_MAX_PRON_LENGTH: u32 = 384u32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_MAX_WORD_LENGTH: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_NORMAL_CONFIDENCE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_STREAMPOS_ASAP: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_STREAMPOS_REALTIME: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SR_LOCALIZED_DESCRIPTION: &str = "Description";
pub const SpAudioFormat: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2667145328, data2: 57696, data3: 18322, data4: [130, 13, 72, 207, 6, 73, 228, 236] };
pub const SpCompressedLexicon: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2425370390, data2: 12098, data3: 4563, data4: [156, 38, 0, 192, 79, 142, 248, 124] };
pub const SpCustomStream: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2378101055, data2: 6472, data3: 19112, data4: [140, 240, 4, 142, 235, 237, 149, 216] };
pub const SpFileStream: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2490897075, data2: 10977, data3: 17988, data4: [186, 134, 158, 144, 222, 215, 236, 145] };
pub const SpInProcRecoContext: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1940744258, data2: 44256, data3: 17896, data4: [164, 221, 135, 149, 136, 26, 44, 42] };
pub const SpInprocRecognizer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1102617451, data2: 37785, data3: 4562, data4: [150, 35, 0, 192, 79, 142, 230, 40] };
pub const SpLexicon: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 106292118, data2: 9680, data3: 4563, data4: [156, 38, 0, 192, 79, 142, 248, 124] };
pub const SpMMAudioEnum: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2870513824, data2: 59679, data3: 4562, data4: [187, 145, 0, 192, 79, 142, 230, 192] };
pub const SpMMAudioIn: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3476893264, data2: 21490, data3: 4562, data4: [150, 12, 0, 192, 79, 142, 230, 40] };
pub const SpMMAudioOut: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2831581419, data2: 15666, data3: 4562, data4: [158, 231, 0, 192, 79, 121, 115, 150] };
pub const SpMemoryStream: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1605889917, data2: 57332, data3: 18058, data4: [182, 183, 47, 203, 209, 136, 249, 148] };
pub const SpNotifyTranslator: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3803075442, data2: 23872, data3: 4562, data4: [150, 14, 0, 192, 79, 142, 230, 40] };
pub const SpNullPhoneConverter: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1163863273, data2: 29590, data3: 18966, data4: [151, 21, 124, 15, 219, 227, 239, 227] };
pub const SpObjectToken: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4014020434, data2: 14134, data3: 19636, data4: [156, 140, 142, 244, 204, 181, 142, 254] };
pub const SpObjectTokenCategory: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2836404351, data2: 3194, data3: 17836, data4: [146, 204, 89, 237, 175, 183, 123, 83] };
pub const SpPhoneConverter: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2441475907, data2: 4419, data3: 19496, data4: [134, 181, 191, 241, 79, 32, 229, 200] };
pub const SpPhoneticAlphabetConverter: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1329676582, data2: 57315, data3: 17961, data4: [153, 238, 121, 121, 120, 49, 126, 173] };
pub const SpPhraseInfoBuilder: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3258958477, data2: 50527, data3: 18208, data4: [139, 50, 145, 247, 60, 43, 213, 209] };
pub const SpResourceManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2524222323, data2: 13201, data3: 4562, data4: [158, 227, 0, 192, 79, 121, 115, 150] };
pub const SpSharedRecoContext: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1193304580, data2: 24266, data3: 4562, data4: [150, 15, 0, 192, 79, 142, 230, 40] };
pub const SpSharedRecognizer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1005471888, data2: 20457, data3: 18999, data4: [140, 30, 94, 126, 18, 121, 28, 31] };
pub const SpShortcut: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 225586970, data2: 40911, data3: 20066, data4: [150, 216, 109, 248, 240, 26, 38, 170] };
pub const SpStream: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1901960281, data2: 17474, data3: 4562, data4: [150, 5, 0, 192, 79, 142, 230, 40] };
pub const SpStreamFormatConverter: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1880331322, data2: 58092, data3: 4562, data4: [160, 134, 0, 192, 79, 142, 249, 181] };
pub const SpTextSelectionInformation: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 261227274, data2: 52221, data3: 19128, data4: [161, 100, 255, 89, 133, 84, 127, 246] };
pub const SpUnCompressedLexicon: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3387128853, data2: 57234, data3: 18215, data4: [133, 214, 114, 229, 238, 182, 153, 90] };
pub const SpVoice: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2524222327, data2: 13201, data3: 4562, data4: [158, 227, 0, 192, 79, 121, 115, 150] };
pub const SpWaveFormatEx: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3348780876, data2: 25534, data3: 17593, data4: [128, 31, 40, 63, 135, 248, 152, 190] };
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SpeechAllElements: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SpeechAudioFormatType = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTDefault: SpeechAudioFormatType = -1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTNoAssignedFormat: SpeechAudioFormatType = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTText: SpeechAudioFormatType = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTNonStandardFormat: SpeechAudioFormatType = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTExtendedAudioFormat: SpeechAudioFormatType = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT8kHz8BitMono: SpeechAudioFormatType = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT8kHz8BitStereo: SpeechAudioFormatType = 5i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT8kHz16BitMono: SpeechAudioFormatType = 6i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT8kHz16BitStereo: SpeechAudioFormatType = 7i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT11kHz8BitMono: SpeechAudioFormatType = 8i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT11kHz8BitStereo: SpeechAudioFormatType = 9i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT11kHz16BitMono: SpeechAudioFormatType = 10i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT11kHz16BitStereo: SpeechAudioFormatType = 11i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT12kHz8BitMono: SpeechAudioFormatType = 12i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT12kHz8BitStereo: SpeechAudioFormatType = 13i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT12kHz16BitMono: SpeechAudioFormatType = 14i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT12kHz16BitStereo: SpeechAudioFormatType = 15i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT16kHz8BitMono: SpeechAudioFormatType = 16i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT16kHz8BitStereo: SpeechAudioFormatType = 17i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT16kHz16BitMono: SpeechAudioFormatType = 18i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT16kHz16BitStereo: SpeechAudioFormatType = 19i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT22kHz8BitMono: SpeechAudioFormatType = 20i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT22kHz8BitStereo: SpeechAudioFormatType = 21i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT22kHz16BitMono: SpeechAudioFormatType = 22i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT22kHz16BitStereo: SpeechAudioFormatType = 23i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT24kHz8BitMono: SpeechAudioFormatType = 24i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT24kHz8BitStereo: SpeechAudioFormatType = 25i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT24kHz16BitMono: SpeechAudioFormatType = 26i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT24kHz16BitStereo: SpeechAudioFormatType = 27i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT32kHz8BitMono: SpeechAudioFormatType = 28i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT32kHz8BitStereo: SpeechAudioFormatType = 29i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT32kHz16BitMono: SpeechAudioFormatType = 30i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT32kHz16BitStereo: SpeechAudioFormatType = 31i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT44kHz8BitMono: SpeechAudioFormatType = 32i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT44kHz8BitStereo: SpeechAudioFormatType = 33i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT44kHz16BitMono: SpeechAudioFormatType = 34i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT44kHz16BitStereo: SpeechAudioFormatType = 35i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT48kHz8BitMono: SpeechAudioFormatType = 36i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT48kHz8BitStereo: SpeechAudioFormatType = 37i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT48kHz16BitMono: SpeechAudioFormatType = 38i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT48kHz16BitStereo: SpeechAudioFormatType = 39i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTTrueSpeech_8kHz1BitMono: SpeechAudioFormatType = 40i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTCCITT_ALaw_8kHzMono: SpeechAudioFormatType = 41i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTCCITT_ALaw_8kHzStereo: SpeechAudioFormatType = 42i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTCCITT_ALaw_11kHzMono: SpeechAudioFormatType = 43i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTCCITT_ALaw_11kHzStereo: SpeechAudioFormatType = 44i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTCCITT_ALaw_22kHzMono: SpeechAudioFormatType = 45i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTCCITT_ALaw_22kHzStereo: SpeechAudioFormatType = 46i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTCCITT_ALaw_44kHzMono: SpeechAudioFormatType = 47i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTCCITT_ALaw_44kHzStereo: SpeechAudioFormatType = 48i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTCCITT_uLaw_8kHzMono: SpeechAudioFormatType = 49i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTCCITT_uLaw_8kHzStereo: SpeechAudioFormatType = 50i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTCCITT_uLaw_11kHzMono: SpeechAudioFormatType = 51i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTCCITT_uLaw_11kHzStereo: SpeechAudioFormatType = 52i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTCCITT_uLaw_22kHzMono: SpeechAudioFormatType = 53i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTCCITT_uLaw_22kHzStereo: SpeechAudioFormatType = 54i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTCCITT_uLaw_44kHzMono: SpeechAudioFormatType = 55i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTCCITT_uLaw_44kHzStereo: SpeechAudioFormatType = 56i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTADPCM_8kHzMono: SpeechAudioFormatType = 57i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTADPCM_8kHzStereo: SpeechAudioFormatType = 58i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTADPCM_11kHzMono: SpeechAudioFormatType = 59i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTADPCM_11kHzStereo: SpeechAudioFormatType = 60i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTADPCM_22kHzMono: SpeechAudioFormatType = 61i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTADPCM_22kHzStereo: SpeechAudioFormatType = 62i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTADPCM_44kHzMono: SpeechAudioFormatType = 63i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTADPCM_44kHzStereo: SpeechAudioFormatType = 64i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTGSM610_8kHzMono: SpeechAudioFormatType = 65i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTGSM610_11kHzMono: SpeechAudioFormatType = 66i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTGSM610_22kHzMono: SpeechAudioFormatType = 67i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTGSM610_44kHzMono: SpeechAudioFormatType = 68i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SpeechAudioState = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SASClosed: SpeechAudioState = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SASStop: SpeechAudioState = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SASPause: SpeechAudioState = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SASRun: SpeechAudioState = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SpeechBookmarkOptions = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SBONone: SpeechBookmarkOptions = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SBOPause: SpeechBookmarkOptions = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SpeechDataKeyLocation = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SDKLDefaultLocation: SpeechDataKeyLocation = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SDKLCurrentUser: SpeechDataKeyLocation = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SDKLLocalMachine: SpeechDataKeyLocation = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SDKLCurrentConfig: SpeechDataKeyLocation = 5i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SpeechDiscardType = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SDTProperty: SpeechDiscardType = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SDTReplacement: SpeechDiscardType = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SDTRule: SpeechDiscardType = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SDTDisplayText: SpeechDiscardType = 8i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SDTLexicalForm: SpeechDiscardType = 16i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SDTPronunciation: SpeechDiscardType = 32i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SDTAudio: SpeechDiscardType = 64i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SDTAlternates: SpeechDiscardType = 128i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SDTAll: SpeechDiscardType = 255i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SpeechDisplayAttributes = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SDA_No_Trailing_Space: SpeechDisplayAttributes = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SDA_One_Trailing_Space: SpeechDisplayAttributes = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SDA_Two_Trailing_Spaces: SpeechDisplayAttributes = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SDA_Consume_Leading_Spaces: SpeechDisplayAttributes = 8i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SpeechEmulationCompareFlags = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SECFIgnoreCase: SpeechEmulationCompareFlags = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SECFIgnoreKanaType: SpeechEmulationCompareFlags = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SECFIgnoreWidth: SpeechEmulationCompareFlags = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SECFNoSpecialChars: SpeechEmulationCompareFlags = 536870912i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SECFEmulateResult: SpeechEmulationCompareFlags = 1073741824i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SECFDefault: SpeechEmulationCompareFlags = 196609i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SpeechEngineConfidence = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SECLowConfidence: SpeechEngineConfidence = -1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SECNormalConfidence: SpeechEngineConfidence = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SECHighConfidence: SpeechEngineConfidence = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SpeechFormatType = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SFTInput: SpeechFormatType = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SFTSREngine: SpeechFormatType = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SpeechGrammarRuleStateTransitionType = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SGRSTTEpsilon: SpeechGrammarRuleStateTransitionType = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SGRSTTWord: SpeechGrammarRuleStateTransitionType = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SGRSTTRule: SpeechGrammarRuleStateTransitionType = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SGRSTTDictation: SpeechGrammarRuleStateTransitionType = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SGRSTTWildcard: SpeechGrammarRuleStateTransitionType = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SGRSTTTextBuffer: SpeechGrammarRuleStateTransitionType = 5i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SpeechGrammarState = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SGSEnabled: SpeechGrammarState = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SGSDisabled: SpeechGrammarState = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SGSExclusive: SpeechGrammarState = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SpeechGrammarWordType = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SGDisplay: SpeechGrammarWordType = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SGLexical: SpeechGrammarWordType = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SGPronounciation: SpeechGrammarWordType = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SGLexicalNoSpecialChars: SpeechGrammarWordType = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SpeechInterference = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SINone: SpeechInterference = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SINoise: SpeechInterference = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SINoSignal: SpeechInterference = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SITooLoud: SpeechInterference = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SITooQuiet: SpeechInterference = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SITooFast: SpeechInterference = 5i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SITooSlow: SpeechInterference = 6i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SpeechLexiconType = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SLTUser: SpeechLexiconType = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SLTApp: SpeechLexiconType = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SpeechLoadOption = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SLOStatic: SpeechLoadOption = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SLODynamic: SpeechLoadOption = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SpeechPartOfSpeech = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSNotOverriden: SpeechPartOfSpeech = -1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSUnknown: SpeechPartOfSpeech = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSNoun: SpeechPartOfSpeech = 4096i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSVerb: SpeechPartOfSpeech = 8192i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSModifier: SpeechPartOfSpeech = 12288i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSFunction: SpeechPartOfSpeech = 16384i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSInterjection: SpeechPartOfSpeech = 20480i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSLMA: SpeechPartOfSpeech = 28672i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSSuppressWord: SpeechPartOfSpeech = 61440i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SpeechRecoContextState = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRCS_Disabled: SpeechRecoContextState = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRCS_Enabled: SpeechRecoContextState = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SpeechRecoEvents = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SREStreamEnd: SpeechRecoEvents = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRESoundStart: SpeechRecoEvents = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRESoundEnd: SpeechRecoEvents = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SREPhraseStart: SpeechRecoEvents = 8i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRERecognition: SpeechRecoEvents = 16i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SREHypothesis: SpeechRecoEvents = 32i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SREBookmark: SpeechRecoEvents = 64i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SREPropertyNumChange: SpeechRecoEvents = 128i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SREPropertyStringChange: SpeechRecoEvents = 256i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SREFalseRecognition: SpeechRecoEvents = 512i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SREInterference: SpeechRecoEvents = 1024i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRERequestUI: SpeechRecoEvents = 2048i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SREStateChange: SpeechRecoEvents = 4096i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SREAdaptation: SpeechRecoEvents = 8192i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SREStreamStart: SpeechRecoEvents = 16384i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRERecoOtherContext: SpeechRecoEvents = 32768i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SREAudioLevel: SpeechRecoEvents = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SREPrivate: SpeechRecoEvents = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SREAllEvents: SpeechRecoEvents = 393215i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SpeechRecognitionType = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRTStandard: SpeechRecognitionType = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRTAutopause: SpeechRecognitionType = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRTEmulated: SpeechRecognitionType = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRTSMLTimeout: SpeechRecognitionType = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRTExtendableParse: SpeechRecognitionType = 8i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRTReSent: SpeechRecognitionType = 16i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SpeechRecognizerState = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRSInactive: SpeechRecognizerState = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRSActive: SpeechRecognizerState = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRSActiveAlways: SpeechRecognizerState = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRSInactiveWithPurge: SpeechRecognizerState = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SpeechRetainedAudioOptions = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRAONone: SpeechRetainedAudioOptions = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRAORetainAudio: SpeechRetainedAudioOptions = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SpeechRuleAttributes = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRATopLevel: SpeechRuleAttributes = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRADefaultToActive: SpeechRuleAttributes = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRAExport: SpeechRuleAttributes = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRAImport: SpeechRuleAttributes = 8i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRAInterpreter: SpeechRuleAttributes = 16i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRADynamic: SpeechRuleAttributes = 32i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRARoot: SpeechRuleAttributes = 64i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SpeechRuleState = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SGDSInactive: SpeechRuleState = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SGDSActive: SpeechRuleState = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SGDSActiveWithAutoPause: SpeechRuleState = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SGDSActiveUserDelimited: SpeechRuleState = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SpeechRunState = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRSEDone: SpeechRunState = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRSEIsSpeaking: SpeechRunState = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SpeechSpecialTransitionType = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SSTTWildcard: SpeechSpecialTransitionType = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SSTTDictation: SpeechSpecialTransitionType = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SSTTTextBuffer: SpeechSpecialTransitionType = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SpeechStreamFileMode = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SSFMOpenForRead: SpeechStreamFileMode = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SSFMOpenReadWrite: SpeechStreamFileMode = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SSFMCreate: SpeechStreamFileMode = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SSFMCreateForWrite: SpeechStreamFileMode = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SpeechStreamSeekPositionType = u32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SSSPTRelativeToStart: SpeechStreamSeekPositionType = 0u32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SSSPTRelativeToCurrentPosition: SpeechStreamSeekPositionType = 1u32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SSSPTRelativeToEnd: SpeechStreamSeekPositionType = 2u32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SpeechTokenContext = u32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const STCInprocServer: SpeechTokenContext = 1u32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const STCInprocHandler: SpeechTokenContext = 2u32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const STCLocalServer: SpeechTokenContext = 4u32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const STCRemoteServer: SpeechTokenContext = 16u32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const STCAll: SpeechTokenContext = 23u32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SpeechTokenShellFolder = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const STSF_AppData: SpeechTokenShellFolder = 26i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const STSF_LocalAppData: SpeechTokenShellFolder = 28i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const STSF_CommonAppData: SpeechTokenShellFolder = 35i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const STSF_FlagCreate: SpeechTokenShellFolder = 32768i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SpeechVisemeFeature = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVF_None: SpeechVisemeFeature = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVF_Stressed: SpeechVisemeFeature = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVF_Emphasis: SpeechVisemeFeature = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SpeechVisemeType = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_0: SpeechVisemeType = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_1: SpeechVisemeType = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_2: SpeechVisemeType = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_3: SpeechVisemeType = 3i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_4: SpeechVisemeType = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_5: SpeechVisemeType = 5i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_6: SpeechVisemeType = 6i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_7: SpeechVisemeType = 7i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_8: SpeechVisemeType = 8i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_9: SpeechVisemeType = 9i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_10: SpeechVisemeType = 10i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_11: SpeechVisemeType = 11i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_12: SpeechVisemeType = 12i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_13: SpeechVisemeType = 13i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_14: SpeechVisemeType = 14i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_15: SpeechVisemeType = 15i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_16: SpeechVisemeType = 16i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_17: SpeechVisemeType = 17i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_18: SpeechVisemeType = 18i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_19: SpeechVisemeType = 19i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_20: SpeechVisemeType = 20i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_21: SpeechVisemeType = 21i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SpeechVoiceEvents = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVEStartInputStream: SpeechVoiceEvents = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVEEndInputStream: SpeechVoiceEvents = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVEVoiceChange: SpeechVoiceEvents = 8i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVEBookmark: SpeechVoiceEvents = 16i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVEWordBoundary: SpeechVoiceEvents = 32i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVEPhoneme: SpeechVoiceEvents = 64i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVESentenceBoundary: SpeechVoiceEvents = 128i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVEViseme: SpeechVoiceEvents = 256i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVEAudioLevel: SpeechVoiceEvents = 512i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVEPrivate: SpeechVoiceEvents = 32768i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVEAllEvents: SpeechVoiceEvents = 33790i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SpeechVoicePriority = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVPNormal: SpeechVoicePriority = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVPAlert: SpeechVoicePriority = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVPOver: SpeechVoicePriority = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SpeechVoiceSpeakFlags = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVSFDefault: SpeechVoiceSpeakFlags = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVSFlagsAsync: SpeechVoiceSpeakFlags = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVSFPurgeBeforeSpeak: SpeechVoiceSpeakFlags = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVSFIsFilename: SpeechVoiceSpeakFlags = 4i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVSFIsXML: SpeechVoiceSpeakFlags = 8i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVSFIsNotXML: SpeechVoiceSpeakFlags = 16i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVSFPersistXML: SpeechVoiceSpeakFlags = 32i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVSFNLPSpeakPunc: SpeechVoiceSpeakFlags = 64i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVSFParseSapi: SpeechVoiceSpeakFlags = 128i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVSFParseSsml: SpeechVoiceSpeakFlags = 256i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVSFParseAutodetect: SpeechVoiceSpeakFlags = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVSFNLPMask: SpeechVoiceSpeakFlags = 64i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVSFParseMask: SpeechVoiceSpeakFlags = 384i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVSFVoiceMask: SpeechVoiceSpeakFlags = 511i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVSFUnusedFlags: SpeechVoiceSpeakFlags = -512i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SpeechWordPronounceable = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SWPUnknownWordUnpronounceable: SpeechWordPronounceable = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SWPUnknownWordPronounceable: SpeechWordPronounceable = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SWPKnownWordPronounceable: SpeechWordPronounceable = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub type SpeechWordType = i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SWTAdded: SpeechWordType = 1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SWTDeleted: SpeechWordType = 2i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const Speech_Default_Weight: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const Speech_Max_Pron_Length: i32 = 384i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const Speech_Max_Word_Length: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const Speech_StreamPos_Asap: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const Speech_StreamPos_RealTime: i32 = -1i32;
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct _ISpeechRecoContextEvents {
    pub base__: super::super::System::Com::IDispatch,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct _ISpeechVoiceEvents {
    pub base__: super::super::System::Com::IDispatch,
}
