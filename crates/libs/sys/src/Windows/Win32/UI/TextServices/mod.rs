#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DoMsCtfMonitor(dwflags: u32, heventforservicestop: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
    pub fn InitLocalMsCtfMonitor(dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
    pub fn UninitLocalMsCtfMonitor() -> ::windows_sys::core::HRESULT;
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub type ANCHOR_CHANGE_HISTORY_FLAGS = u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_CH_PRECEDING_DEL: ANCHOR_CHANGE_HISTORY_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_CH_FOLLOWING_DEL: ANCHOR_CHANGE_HISTORY_FLAGS = 2u32;
pub const AccClientDocMgr: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4232629296, data2: 20286, data3: 20385, data4: [128, 59, 173, 14, 25, 106, 131, 177] };
pub const AccDictionary: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1702030870, data2: 24549, data3: 17201, data4: [187, 109, 118, 164, 156, 86, 228, 35] };
pub const AccServerDocMgr: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1619633022, data2: 60298, data3: 18477, data4: [189, 111, 249, 244, 105, 4, 209, 109] };
pub const AccStore: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1413514111, data2: 19455, data3: 19173, data4: [161, 177, 119, 34, 236, 198, 51, 42] };
pub const CLSID_TF_CategoryMgr: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2763343009, data2: 17293, data3: 19265, data4: [147, 37, 134, 149, 35, 226, 214, 199] };
pub const CLSID_TF_ClassicLangBar: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 857224716, data2: 6908, data3: 19721, data4: [168, 107, 159, 156, 182, 220, 235, 156] };
pub const CLSID_TF_DisplayAttributeMgr: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1021791716, data2: 21459, data3: 19828, data4: [139, 131, 67, 27, 56, 40, 186, 83] };
pub const CLSID_TF_InputProcessorProfiles: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 868563536, data2: 62550, data3: 18564, data4: [176, 73, 133, 253, 100, 62, 207, 237] };
pub const CLSID_TF_LangBarItemMgr: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3113424530, data2: 41651, data3: 20395, data4: [191, 51, 158, 198, 249, 251, 150, 172] };
pub const CLSID_TF_LangBarMgr: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3954216005, data2: 27722, data3: 20444, data4: [174, 83, 78, 184, 196, 199, 219, 142] };
pub const CLSID_TF_ThreadMgr: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1385864811, data2: 25991, data3: 20259, data4: [171, 158, 156, 125, 104, 62, 60, 80] };
pub const CLSID_TF_TransitoryExtensionUIEntry: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2926305288, data2: 2043, data3: 16397, data4: [139, 235, 51, 122, 100, 247, 5, 31] };
pub const CLSID_TsfServices: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 967760896, data2: 27488, data3: 18139, data4: [141, 49, 54, 66, 190, 14, 67, 115] };
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const DCM_FLAGS_CTFMON: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const DCM_FLAGS_LOCALTHREADTSF: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const DCM_FLAGS_TASKENG: u32 = 1u32;
pub const DocWrap: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3208802174, data2: 31326, data3: 17622, data4: [131, 12, 163, 144, 234, 148, 98, 163] };
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub type GET_TEXT_AND_PROPERTY_UPDATES_FLAGS = u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_GTP_NONE: GET_TEXT_AND_PROPERTY_UPDATES_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_GTP_INCL_TEXT: GET_TEXT_AND_PROPERTY_UPDATES_FLAGS = 1u32;
pub const GUID_APP_FUNCTIONPROVIDER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1286533150, data2: 4783, data3: 19214, data4: [157, 177, 166, 236, 91, 136, 18, 8] };
pub const GUID_COMPARTMENT_CONVERSIONMODEBIAS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1419244822, data2: 61073, data3: 17262, data4: [185, 70, 170, 44, 5, 241, 172, 91] };
pub const GUID_COMPARTMENT_EMPTYCONTEXT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3611852223, data2: 32846, data3: 16837, data4: [137, 77, 173, 150, 253, 78, 234, 19] };
pub const GUID_COMPARTMENT_ENABLED_PROFILES_UPDATED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2462186824, data2: 43438, data3: 19068, data4: [190, 8, 67, 41, 228, 114, 56, 23] };
pub const GUID_COMPARTMENT_HANDWRITING_OPENCLOSE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4188941419, data2: 6246, data3: 17249, data4: [175, 114, 122, 163, 9, 72, 137, 14] };
pub const GUID_COMPARTMENT_KEYBOARD_DISABLED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1906684499, data2: 6481, data3: 18027, data4: [159, 188, 156, 136, 8, 250, 132, 242] };
pub const GUID_COMPARTMENT_KEYBOARD_INPUTMODE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3059295505, data2: 48366, data3: 16674, data4: [167, 196, 9, 244, 179, 250, 67, 150] };
pub const GUID_COMPARTMENT_KEYBOARD_INPUTMODE_CONVERSION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3438304728, data2: 19079, data3: 4567, data4: [166, 226, 0, 6, 91, 132, 67, 92] };
pub const GUID_COMPARTMENT_KEYBOARD_INPUTMODE_SENTENCE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3438304729, data2: 19079, data3: 4567, data4: [166, 226, 0, 6, 91, 132, 67, 92] };
pub const GUID_COMPARTMENT_KEYBOARD_OPENCLOSE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1478965933, data2: 443, data3: 16740, data4: [149, 198, 117, 91, 160, 181, 22, 45] };
pub const GUID_COMPARTMENT_SAPI_AUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1370431622, data2: 52331, data3: 17789, data4: [181, 170, 139, 25, 220, 41, 10, 180] };
pub const GUID_COMPARTMENT_SPEECH_CFGMENU: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4218182701, data2: 20099, data3: 19382, data4: [145, 162, 224, 25, 191, 246, 118, 45] };
pub const GUID_COMPARTMENT_SPEECH_DISABLED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1455801863, data2: 1795, data3: 20057, data4: [142, 82, 203, 200, 78, 139, 190, 53] };
pub const GUID_COMPARTMENT_SPEECH_GLOBALSTATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 710213262, data2: 3336, data3: 17932, data4: [167, 93, 135, 3, 95, 244, 54, 197] };
pub const GUID_COMPARTMENT_SPEECH_OPENCLOSE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1414359651, data2: 58088, data3: 18258, data4: [187, 209, 0, 9, 96, 188, 160, 131] };
pub const GUID_COMPARTMENT_SPEECH_UI_STATUS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3642758896, data2: 37735, data3: 20455, data4: [154, 191, 188, 89, 218, 203, 224, 227] };
pub const GUID_COMPARTMENT_TIPUISTATUS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 344761324, data2: 870, data3: 16412, data4: [141, 117, 237, 151, 141, 133, 251, 201] };
pub const GUID_COMPARTMENT_TRANSITORYEXTENSION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2346928117, data2: 51104, data3: 4567, data4: [180, 8, 0, 6, 91, 132, 67, 92] };
pub const GUID_COMPARTMENT_TRANSITORYEXTENSION_DOCUMENTMANAGER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2346928119, data2: 51104, data3: 4567, data4: [180, 8, 0, 6, 91, 132, 67, 92] };
pub const GUID_COMPARTMENT_TRANSITORYEXTENSION_PARENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2346928120, data2: 51104, data3: 4567, data4: [180, 8, 0, 6, 91, 132, 67, 92] };
pub const GUID_INTEGRATIONSTYLE_SEARCHBOX: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3872505105, data2: 33527, data3: 18691, data4: [174, 33, 26, 99, 151, 205, 226, 235] };
pub const GUID_LBI_INPUTMODE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 746039326, data2: 16844, data3: 16760, data4: [163, 167, 95, 138, 152, 117, 104, 230] };
pub const GUID_LBI_SAPILAYR_CFGMENUBUTTON: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3492750497, data2: 37933, data3: 16942, data4: [141, 153, 180, 242, 173, 222, 233, 153] };
pub const GUID_MODEBIAS_CHINESE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2061313758, data2: 17192, data3: 18587, data4: [131, 174, 100, 147, 117, 12, 173, 92] };
pub const GUID_MODEBIAS_CONVERSATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 256819460, data2: 6032, data3: 17467, data4: [149, 241, 225, 15, 147, 157, 101, 70] };
pub const GUID_MODEBIAS_DATETIME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4072518514, data2: 32609, data3: 16441, data4: [146, 239, 28, 53, 89, 159, 2, 34] };
pub const GUID_MODEBIAS_FILENAME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3623290878, data2: 17606, data3: 20426, data4: [142, 118, 134, 171, 80, 199, 147, 27] };
pub const GUID_MODEBIAS_FULLWIDTHALPHANUMERIC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2169020344, data2: 45930, data3: 18237, data4: [129, 70, 228, 162, 37, 139, 36, 174] };
pub const GUID_MODEBIAS_FULLWIDTHHANGUL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3222988489, data2: 17845, data3: 20432, data4: [156, 177, 159, 76, 235, 195, 159, 234] };
pub const GUID_MODEBIAS_HALFWIDTHKATAKANA: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 6253411, data2: 30932, data3: 16844, data4: [136, 89, 72, 92, 168, 33, 167, 149] };
pub const GUID_MODEBIAS_HANGUL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1995375937, data2: 9139, data3: 19831, data4: [160, 116, 105, 24, 1, 204, 234, 23] };
pub const GUID_MODEBIAS_HIRAGANA: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3611111790, data2: 39825, data3: 18161, data4: [162, 128, 49, 89, 127, 82, 198, 148] };
pub const GUID_MODEBIAS_KATAKANA: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 772730333, data2: 14874, data3: 18846, data4: [133, 67, 60, 126, 231, 148, 152, 17] };
pub const GUID_MODEBIAS_NAME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4259057904, data2: 53817, data3: 18879, data4: [184, 252, 84, 16, 202, 170, 66, 126] };
pub const GUID_MODEBIAS_NONE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 0, data2: 0, data3: 0, data4: [0, 0, 0, 0, 0, 0, 0, 0] };
pub const GUID_MODEBIAS_NUMERIC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1075934828, data2: 59506, data3: 18685, data4: [156, 238, 78, 197, 199, 94, 22, 195] };
pub const GUID_MODEBIAS_READING: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3809887139, data2: 25702, data3: 19647, data4: [141, 139, 11, 212, 216, 84, 84, 97] };
pub const GUID_MODEBIAS_URLHISTORY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2332972249, data2: 25586, data3: 19560, data4: [132, 212, 121, 174, 231, 165, 159, 9] };
pub const GUID_PROP_ATTRIBUTE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 884233840, data2: 29990, data3: 4562, data4: [161, 71, 0, 16, 90, 39, 153, 181] };
pub const GUID_PROP_COMPOSING: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3777675360, data2: 44821, data3: 4562, data4: [175, 197, 0, 16, 90, 39, 153, 181] };
pub const GUID_PROP_INPUTSCOPE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 387177818, data2: 26855, data3: 19035, data4: [154, 246, 89, 42, 89, 92, 119, 141] };
pub const GUID_PROP_LANGID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 847302176, data2: 32818, data3: 4562, data4: [182, 3, 0, 16, 90, 39, 153, 181] };
pub const GUID_PROP_MODEBIAS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 925763350, data2: 38735, data3: 16556, data4: [160, 136, 8, 205, 201, 46, 191, 188] };
pub const GUID_PROP_READING: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1415837632, data2: 36401, data3: 4562, data4: [191, 70, 0, 16, 90, 39, 153, 181] };
pub const GUID_PROP_TEXTOWNER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4058174752, data2: 2409, data3: 4563, data4: [141, 240, 0, 16, 90, 39, 153, 181] };
pub const GUID_PROP_TKB_ALTERNATES: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1890756611, data2: 38541, data3: 17966, data4: [185, 59, 33, 100, 201, 21, 23, 247] };
pub const GUID_SYSTEM_FUNCTIONPROVIDER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2590608304, data2: 3873, data3: 4563, data4: [141, 241, 0, 16, 90, 39, 153, 181] };
pub const GUID_TFCAT_CATEGORY_OF_TIP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1397508289, data2: 1543, data3: 16536, data4: [165, 33, 79, 200, 153, 199, 62, 144] };
pub const GUID_TFCAT_DISPLAYATTRIBUTEPROPERTY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3110017051, data2: 59980, data3: 19185, data4: [128, 86, 124, 50, 26, 187, 176, 145] };
pub const GUID_TFCAT_DISPLAYATTRIBUTEPROVIDER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 74157184, data2: 5703, data3: 16631, data4: [155, 33, 185, 59, 129, 170, 188, 27] };
pub const GUID_TFCAT_PROPSTYLE_STATIC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1449113816, data2: 27604, data3: 19617, data4: [178, 35, 15, 44, 203, 143, 79, 150] };
pub const GUID_TFCAT_PROP_AUDIODATA: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2608587689, data2: 59563, data3: 19783, data4: [168, 254, 37, 79, 164, 35, 67, 109] };
pub const GUID_TFCAT_PROP_INKDATA: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2087355054, data2: 45271, data3: 20244, data4: [167, 69, 20, 242, 139, 0, 157, 97] };
pub const GUID_TFCAT_TIPCAP_COMLESS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 910300633, data2: 30140, data3: 4567, data4: [166, 239, 0, 6, 91, 132, 67, 92] };
pub const GUID_TFCAT_TIPCAP_DUALMODE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 989009058, data2: 55199, data3: 19227, data4: [153, 146, 21, 8, 109, 51, 155, 5] };
pub const GUID_TFCAT_TIPCAP_IMMERSIVEONLY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 977426860, data2: 25613, data3: 19156, data4: [137, 247, 30, 182, 126, 124, 78, 232] };
pub const GUID_TFCAT_TIPCAP_IMMERSIVESUPPORT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 329258719, data2: 22027, data3: 18125, data4: [148, 122, 76, 58, 241, 224, 227, 93] };
pub const GUID_TFCAT_TIPCAP_INPUTMODECOMPARTMENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3438304727, data2: 19079, data3: 4567, data4: [166, 226, 0, 6, 91, 132, 67, 92] };
pub const GUID_TFCAT_TIPCAP_LOCALSERVER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1953930985, data2: 19046, data3: 20381, data4: [144, 214, 191, 139, 124, 62, 180, 97] };
pub const GUID_TFCAT_TIPCAP_SECUREMODE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1238563278, data2: 8030, data3: 4567, data4: [166, 211, 0, 6, 91, 132, 67, 92] };
pub const GUID_TFCAT_TIPCAP_SYSTRAYSUPPORT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 626020276, data2: 31659, data3: 19393, data4: [156, 105, 207, 129, 137, 15, 14, 245] };
pub const GUID_TFCAT_TIPCAP_TSF3: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 131904687, data2: 39134, data3: 17736, data4: [190, 247, 37, 189, 69, 151, 154, 31] };
pub const GUID_TFCAT_TIPCAP_UIELEMENTENABLED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1238563279, data2: 8030, data3: 4567, data4: [166, 211, 0, 6, 91, 132, 67, 92] };
pub const GUID_TFCAT_TIPCAP_WOW16: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 910300634, data2: 30140, data3: 4567, data4: [166, 239, 0, 6, 91, 132, 67, 92] };
pub const GUID_TFCAT_TIP_HANDWRITING: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 611240839, data2: 49906, data3: 19134, data4: [144, 91, 200, 179, 138, 221, 44, 67] };
pub const GUID_TFCAT_TIP_KEYBOARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 880041059, data2: 45808, data3: 18308, data4: [139, 103, 94, 18, 200, 112, 26, 49] };
pub const GUID_TFCAT_TIP_SPEECH: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3047636177, data2: 33621, data3: 17003, data4: [161, 97, 37, 152, 8, 242, 107, 20] };
pub const GUID_TFCAT_TRANSITORYEXTENSIONUI: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1661132322, data2: 42447, data3: 19202, data4: [191, 232, 77, 114, 178, 190, 211, 198] };
pub const GUID_TS_SERVICE_ACCESSIBLE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4185416192, data2: 42431, data3: 18959, data4: [140, 36, 251, 22, 245, 209, 170, 187] };
pub const GUID_TS_SERVICE_ACTIVEX: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3935533648, data2: 51622, data3: 19325, data4: [137, 74, 73, 217, 155, 120, 72, 52] };
pub const GUID_TS_SERVICE_DATAOBJECT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1619458997, data2: 57893, data3: 18126, data4: [167, 112, 193, 187, 211, 224, 93, 123] };
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GXFPF_NEAREST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GXFPF_ROUND_NEAREST: u32 = 1u32;
pub type HKL = isize;
#[repr(C)]
pub struct IAccClientDocMgr {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDocuments: unsafe extern "system" fn(this: *mut *mut Self, enumunknown: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDocuments: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LookupByHWND: unsafe extern "system" fn(this: *mut *mut Self, hwnd: super::super::Foundation::HWND, riid: *const ::windows_sys::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LookupByHWND: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LookupByPoint: unsafe extern "system" fn(this: *mut *mut Self, pt: super::super::Foundation::POINT, riid: *const ::windows_sys::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LookupByPoint: usize,
    pub GetFocused: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAccClientDocMgr {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1284071481, data2: 31597, data3: 18918, data4: [168, 193, 69, 17, 106, 152, 41, 43] };
}
#[repr(C)]
pub struct IAccDictionary {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLocalizedString: unsafe extern "system" fn(this: *mut *mut Self, term: *const ::windows_sys::core::GUID, lcid: u32, presult: *mut super::super::Foundation::BSTR, plcid: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLocalizedString: usize,
    pub GetParentTerm: unsafe extern "system" fn(this: *mut *mut Self, term: *const ::windows_sys::core::GUID, pparentterm: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMnemonicString: unsafe extern "system" fn(this: *mut *mut Self, term: *const ::windows_sys::core::GUID, presult: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMnemonicString: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LookupMnemonicTerm: unsafe extern "system" fn(this: *mut *mut Self, bstrmnemonic: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pterm: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LookupMnemonicTerm: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ConvertValueToString: unsafe extern "system" fn(this: *mut *mut Self, term: *const ::windows_sys::core::GUID, lcid: u32, varvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pbstrresult: *mut super::super::Foundation::BSTR, plcid: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ConvertValueToString: usize,
}
impl ::windows_sys::core::Interface for IAccDictionary {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 499436383, data2: 55095, data3: 18253, data4: [173, 233, 92, 207, 201, 188, 28, 201] };
}
#[repr(C)]
pub struct IAccServerDocMgr {
    pub base__: ::windows_sys::core::IUnknown,
    pub NewDocument: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RevokeDocument: unsafe extern "system" fn(this: *mut *mut Self, punk: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnDocumentFocus: unsafe extern "system" fn(this: *mut *mut Self, punk: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAccServerDocMgr {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2910614479, data2: 28117, data3: 18517, data4: [171, 194, 176, 75, 173, 91, 145, 83] };
}
#[repr(C)]
pub struct IAccStore {
    pub base__: ::windows_sys::core::IUnknown,
    pub Register: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Unregister: unsafe extern "system" fn(this: *mut *mut Self, punk: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDocuments: unsafe extern "system" fn(this: *mut *mut Self, enumunknown: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDocuments: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LookupByHWND: unsafe extern "system" fn(this: *mut *mut Self, hwnd: super::super::Foundation::HWND, riid: *const ::windows_sys::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LookupByHWND: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LookupByPoint: unsafe extern "system" fn(this: *mut *mut Self, pt: super::super::Foundation::POINT, riid: *const ::windows_sys::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LookupByPoint: usize,
    pub OnDocumentFocus: unsafe extern "system" fn(this: *mut *mut Self, punk: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFocused: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAccStore {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3805104739, data2: 11122, data3: 19784, data4: [183, 57, 149, 228, 118, 81, 149, 186] };
}
#[repr(C)]
pub struct IAnchor {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetGravity: unsafe extern "system" fn(this: *mut *mut Self, gravity: TsGravity) -> ::windows_sys::core::HRESULT,
    pub GetGravity: unsafe extern "system" fn(this: *mut *mut Self, pgravity: *mut TsGravity) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsEqual: unsafe extern "system" fn(this: *mut *mut Self, pawith: *mut ::core::ffi::c_void, pfequal: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsEqual: usize,
    pub Compare: unsafe extern "system" fn(this: *mut *mut Self, pawith: *mut ::core::ffi::c_void, plresult: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Shift: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, cchreq: i32, pcch: *mut i32, pahaltanchor: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ShiftTo: unsafe extern "system" fn(this: *mut *mut Self, pasite: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ShiftRegion: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, dir: TsShiftDir, pfnoregion: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShiftRegion: usize,
    pub SetChangeHistoryMask: unsafe extern "system" fn(this: *mut *mut Self, dwmask: u32) -> ::windows_sys::core::HRESULT,
    pub GetChangeHistory: unsafe extern "system" fn(this: *mut *mut Self, pdwhistory: *mut ANCHOR_CHANGE_HISTORY_FLAGS) -> ::windows_sys::core::HRESULT,
    pub ClearChangeHistory: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppaclone: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAnchor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 267091508, data2: 23136, data3: 17238, data4: [142, 247, 171, 222, 194, 255, 124, 248] };
}
#[repr(C)]
pub struct IClonableWrapper {
    pub base__: ::windows_sys::core::IUnknown,
    pub CloneNewWrapper: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IClonableWrapper {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3007215103, data2: 59468, data3: 19914, data4: [162, 92, 51, 184, 220, 0, 51, 116] };
}
#[repr(C)]
pub struct ICoCreateLocally {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CoCreateLocally: unsafe extern "system" fn(this: *mut *mut Self, rclsid: *const ::windows_sys::core::GUID, dwclscontext: u32, riid: *const ::windows_sys::core::GUID, punk: *mut *mut ::core::ffi::c_void, riidparam: *const ::windows_sys::core::GUID, punkparam: *mut ::core::ffi::c_void, varparam: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CoCreateLocally: usize,
}
impl ::windows_sys::core::Interface for ICoCreateLocally {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 64880810, data2: 62066, data3: 16867, data4: [153, 203, 3, 197, 232, 17, 78, 160] };
}
#[repr(C)]
pub struct ICoCreatedLocally {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LocalInit: unsafe extern "system" fn(this: *mut *mut Self, punklocalobject: *mut ::core::ffi::c_void, riidparam: *const ::windows_sys::core::GUID, punkparam: *mut ::core::ffi::c_void, varparam: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LocalInit: usize,
}
impl ::windows_sys::core::Interface for ICoCreatedLocally {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 173271916, data2: 6408, data3: 18242, data4: [140, 255, 44, 238, 46, 147, 249, 76] };
}
#[repr(C)]
pub struct IDocWrap {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetDoc: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetWrappedDoc: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDocWrap {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3704784382, data2: 3040, data3: 17341, data4: [153, 201, 170, 174, 197, 19, 197, 85] };
}
#[repr(C)]
pub struct IEnumITfCompositionView {
    pub base__: ::windows_sys::core::IUnknown,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32, rgcompositionview: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumITfCompositionView {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1593647802, data2: 30776, data3: 18123, data4: [136, 226, 202, 219, 20, 18, 79, 143] };
}
#[repr(C)]
pub struct IEnumSpeechCommands {
    pub base__: ::windows_sys::core::IUnknown,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32, pspcmds: *mut *mut u16, pcfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumSpeechCommands {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2354949199, data2: 2108, data3: 19333, data4: [164, 201, 113, 116, 96, 72, 173, 202] };
}
#[repr(C)]
pub struct IEnumTfCandidates {
    pub base__: ::windows_sys::core::IUnknown,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32, ppcand: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumTfCandidates {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3740997926, data2: 27776, data3: 19688, data4: [135, 212, 214, 183, 43, 129, 43, 222] };
}
#[repr(C)]
pub struct IEnumTfContextViews {
    pub base__: ::windows_sys::core::IUnknown,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32, rgviews: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumTfContextViews {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4039178461, data2: 53048, data3: 17633, data4: [187, 15, 104, 207, 13, 85, 28, 120] };
}
#[repr(C)]
pub struct IEnumTfContexts {
    pub base__: ::windows_sys::core::IUnknown,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32, rgcontext: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumTfContexts {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2400878246, data2: 5716, data3: 17666, data4: [168, 110, 178, 144, 35, 68, 213, 7] };
}
#[repr(C)]
pub struct IEnumTfDisplayAttributeInfo {
    pub base__: ::windows_sys::core::IUnknown,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32, rginfo: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumTfDisplayAttributeInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2096039127, data2: 52085, data3: 20096, data4: [167, 171, 95, 91, 199, 211, 50, 222] };
}
#[repr(C)]
pub struct IEnumTfDocumentMgrs {
    pub base__: ::windows_sys::core::IUnknown,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32, rgdocumentmgr: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumTfDocumentMgrs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2860574728, data2: 8225, data3: 4562, data4: [147, 224, 0, 96, 176, 103, 184, 110] };
}
#[repr(C)]
pub struct IEnumTfFunctionProviders {
    pub base__: ::windows_sys::core::IUnknown,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32, ppcmdobj: *mut *mut ::core::ffi::c_void, pcfetch: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumTfFunctionProviders {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3836890544, data2: 2448, data3: 4563, data4: [141, 240, 0, 16, 90, 39, 153, 181] };
}
#[repr(C)]
pub struct IEnumTfInputProcessorProfiles {
    pub base__: ::windows_sys::core::IUnknown,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32, pprofile: *mut TF_INPUTPROCESSORPROFILE, pcfetch: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumTfInputProcessorProfiles {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1908860749, data2: 3880, data3: 4568, data4: [168, 42, 0, 6, 91, 132, 67, 92] };
}
#[repr(C)]
pub struct IEnumTfLangBarItems {
    pub base__: ::windows_sys::core::IUnknown,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32, ppitem: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumTfLangBarItems {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1480537296, data2: 56869, data3: 4562, data4: [175, 221, 0, 16, 90, 39, 153, 181] };
}
#[repr(C)]
pub struct IEnumTfLanguageProfiles {
    pub base__: ::windows_sys::core::IUnknown,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32, pprofile: *mut TF_LANGUAGEPROFILE, pcfetch: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumTfLanguageProfiles {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1029816081, data2: 44127, data3: 17096, data4: [164, 203, 147, 27, 204, 40, 199, 68] };
}
#[repr(C)]
pub struct IEnumTfLatticeElements {
    pub base__: ::windows_sys::core::IUnknown,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32, rgselements: *mut TF_LMLATTELEMENT, pcfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumTfLatticeElements {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1452834898, data2: 18394, data3: 18949, data4: [145, 26, 227, 217, 65, 241, 113, 69] };
}
#[repr(C)]
pub struct IEnumTfProperties {
    pub base__: ::windows_sys::core::IUnknown,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32, ppprop: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumTfProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 421039280, data2: 44201, data3: 4562, data4: [175, 197, 0, 16, 90, 39, 153, 181] };
}
#[repr(C)]
pub struct IEnumTfPropertyValue {
    pub base__: ::windows_sys::core::IUnknown,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32, rgvalues: *mut TF_PROPERTYVAL, pcfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumTfPropertyValue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2396559387, data2: 31760, data3: 19837, data4: [159, 179, 171, 114, 233, 199, 95, 114] };
}
#[repr(C)]
pub struct IEnumTfRanges {
    pub base__: ::windows_sys::core::IUnknown,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32, pprange: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumTfRanges {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4187832128, data2: 36402, data3: 4562, data4: [191, 70, 0, 16, 90, 39, 153, 181] };
}
#[repr(C)]
pub struct IEnumTfUIElements {
    pub base__: ::windows_sys::core::IUnknown,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32, ppelement: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumTfUIElements {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2289740062, data2: 44218, data3: 18737, data4: [132, 218, 60, 82, 8, 207, 84, 63] };
}
#[repr(C)]
pub struct IInternalDocWrap {
    pub base__: ::windows_sys::core::IUnknown,
    pub NotifyRevoke: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInternalDocWrap {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3786040422, data2: 40372, data3: 16570, data4: [190, 3, 119, 195, 142, 142, 96, 178] };
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const ILMCM_CHECKLAYOUTANDTIPENABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const ILMCM_LANGUAGEBAROFF: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub type INSERT_TEXT_AT_SELECTION_FLAGS = u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IAS_NOQUERY: INSERT_TEXT_AT_SELECTION_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IAS_QUERYONLY: INSERT_TEXT_AT_SELECTION_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IAS_NO_DEFAULT_COMPOSITION: INSERT_TEXT_AT_SELECTION_FLAGS = 2147483648u32;
#[repr(C)]
pub struct ISpeechCommandProvider {
    pub base__: ::windows_sys::core::IUnknown,
    pub EnumSpeechCommands: unsafe extern "system" fn(this: *mut *mut Self, langid: u16, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ProcessCommand: unsafe extern "system" fn(this: *mut *mut Self, pszcommand: ::windows_sys::core::PCWSTR, cch: u32, langid: u16) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpeechCommandProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 954244428, data2: 22637, data3: 17242, data4: [181, 146, 200, 168, 102, 145, 222, 198] };
}
#[repr(C)]
pub struct ITextStoreACP {
    pub base__: ::windows_sys::core::IUnknown,
    pub AdviseSink: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, punk: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows_sys::core::HRESULT,
    pub UnadviseSink: unsafe extern "system" fn(this: *mut *mut Self, punk: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RequestLock: unsafe extern "system" fn(this: *mut *mut Self, dwlockflags: u32, phrsession: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, pdcs: *mut TS_STATUS) -> ::windows_sys::core::HRESULT,
    pub QueryInsert: unsafe extern "system" fn(this: *mut *mut Self, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSelection: unsafe extern "system" fn(this: *mut *mut Self, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSelection: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSelection: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSelection: usize,
    pub GetText: unsafe extern "system" fn(this: *mut *mut Self, acpstart: i32, acpend: i32, pchplain: ::windows_sys::core::PWSTR, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, acpstart: i32, acpend: i32, pchtext: ::windows_sys::core::PCWSTR, cch: u32, pchange: *mut TS_TEXTCHANGE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFormattedText: unsafe extern "system" fn(this: *mut *mut Self, acpstart: i32, acpend: i32, ppdataobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFormattedText: usize,
    pub GetEmbedded: unsafe extern "system" fn(this: *mut *mut Self, acppos: i32, rguidservice: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub QueryInsertEmbedded: unsafe extern "system" fn(this: *mut *mut Self, pguidservice: *const ::windows_sys::core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    QueryInsertEmbedded: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InsertEmbedded: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: *mut ::core::ffi::c_void, pchange: *mut TS_TEXTCHANGE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InsertEmbedded: usize,
    pub InsertTextAtSelection: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, pchtext: ::windows_sys::core::PCWSTR, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InsertEmbeddedAtSelection: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, pdataobject: *mut ::core::ffi::c_void, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InsertEmbeddedAtSelection: usize,
    pub RequestSupportedAttrs: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub RequestAttrsAtPosition: unsafe extern "system" fn(this: *mut *mut Self, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows_sys::core::GUID, dwflags: u32) -> ::windows_sys::core::HRESULT,
    pub RequestAttrsTransitioningAtPosition: unsafe extern "system" fn(this: *mut *mut Self, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows_sys::core::GUID, dwflags: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FindNextAttrTransition: unsafe extern "system" fn(this: *mut *mut Self, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const ::windows_sys::core::GUID, dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FindNextAttrTransition: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RetrieveRequestedAttrs: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RetrieveRequestedAttrs: usize,
    pub GetEndACP: unsafe extern "system" fn(this: *mut *mut Self, pacp: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetActiveView: unsafe extern "system" fn(this: *mut *mut Self, pvcview: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetACPFromPoint: unsafe extern "system" fn(this: *mut *mut Self, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, pacp: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetACPFromPoint: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTextExt: unsafe extern "system" fn(this: *mut *mut Self, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTextExt: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetScreenExt: unsafe extern "system" fn(this: *mut *mut Self, vcview: u32, prc: *mut super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetScreenExt: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetWnd: unsafe extern "system" fn(this: *mut *mut Self, vcview: u32, phwnd: *mut super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetWnd: usize,
}
impl ::windows_sys::core::Interface for ITextStoreACP {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 680038371, data2: 49824, data3: 18490, data4: [163, 234, 140, 177, 206, 81, 255, 61] };
}
#[repr(C)]
pub struct ITextStoreACP2 {
    pub base__: ::windows_sys::core::IUnknown,
    pub AdviseSink: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, punk: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows_sys::core::HRESULT,
    pub UnadviseSink: unsafe extern "system" fn(this: *mut *mut Self, punk: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RequestLock: unsafe extern "system" fn(this: *mut *mut Self, dwlockflags: u32, phrsession: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, pdcs: *mut TS_STATUS) -> ::windows_sys::core::HRESULT,
    pub QueryInsert: unsafe extern "system" fn(this: *mut *mut Self, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSelection: unsafe extern "system" fn(this: *mut *mut Self, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSelection: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSelection: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSelection: usize,
    pub GetText: unsafe extern "system" fn(this: *mut *mut Self, acpstart: i32, acpend: i32, pchplain: ::windows_sys::core::PWSTR, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, acpstart: i32, acpend: i32, pchtext: ::windows_sys::core::PCWSTR, cch: u32, pchange: *mut TS_TEXTCHANGE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFormattedText: unsafe extern "system" fn(this: *mut *mut Self, acpstart: i32, acpend: i32, ppdataobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFormattedText: usize,
    pub GetEmbedded: unsafe extern "system" fn(this: *mut *mut Self, acppos: i32, rguidservice: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub QueryInsertEmbedded: unsafe extern "system" fn(this: *mut *mut Self, pguidservice: *const ::windows_sys::core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    QueryInsertEmbedded: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InsertEmbedded: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: *mut ::core::ffi::c_void, pchange: *mut TS_TEXTCHANGE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InsertEmbedded: usize,
    pub InsertTextAtSelection: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, pchtext: ::windows_sys::core::PCWSTR, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InsertEmbeddedAtSelection: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, pdataobject: *mut ::core::ffi::c_void, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InsertEmbeddedAtSelection: usize,
    pub RequestSupportedAttrs: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub RequestAttrsAtPosition: unsafe extern "system" fn(this: *mut *mut Self, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows_sys::core::GUID, dwflags: u32) -> ::windows_sys::core::HRESULT,
    pub RequestAttrsTransitioningAtPosition: unsafe extern "system" fn(this: *mut *mut Self, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows_sys::core::GUID, dwflags: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FindNextAttrTransition: unsafe extern "system" fn(this: *mut *mut Self, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const ::windows_sys::core::GUID, dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FindNextAttrTransition: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RetrieveRequestedAttrs: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RetrieveRequestedAttrs: usize,
    pub GetEndACP: unsafe extern "system" fn(this: *mut *mut Self, pacp: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetActiveView: unsafe extern "system" fn(this: *mut *mut Self, pvcview: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetACPFromPoint: unsafe extern "system" fn(this: *mut *mut Self, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, pacp: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetACPFromPoint: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTextExt: unsafe extern "system" fn(this: *mut *mut Self, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTextExt: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetScreenExt: unsafe extern "system" fn(this: *mut *mut Self, vcview: u32, prc: *mut super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetScreenExt: usize,
}
impl ::windows_sys::core::Interface for ITextStoreACP2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4167751839, data2: 24548, data3: 19341, data4: [187, 159, 239, 55, 151, 168, 79, 31] };
}
#[repr(C)]
pub struct ITextStoreACPEx {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub ScrollToRect: unsafe extern "system" fn(this: *mut *mut Self, acpstart: i32, acpend: i32, rc: super::super::Foundation::RECT, dwposition: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ScrollToRect: usize,
}
impl ::windows_sys::core::Interface for ITextStoreACPEx {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2732473282, data2: 15758, data3: 4563, data4: [129, 169, 247, 83, 251, 230, 26, 0] };
}
#[repr(C)]
pub struct ITextStoreACPServices {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Serialize: unsafe extern "system" fn(this: *mut *mut Self, pprop: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Serialize: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Unserialize: unsafe extern "system" fn(this: *mut *mut Self, pprop: *mut ::core::ffi::c_void, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: *mut ::core::ffi::c_void, ploader: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Unserialize: usize,
    pub ForceLoadProperty: unsafe extern "system" fn(this: *mut *mut Self, pprop: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateRange: unsafe extern "system" fn(this: *mut *mut Self, acpstart: i32, acpend: i32, pprange: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITextStoreACPServices {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2860574977, data2: 8225, data3: 4562, data4: [147, 224, 0, 96, 176, 103, 184, 110] };
}
#[repr(C)]
pub struct ITextStoreACPSink {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnTextChange: unsafe extern "system" fn(this: *mut *mut Self, dwflags: TEXT_STORE_TEXT_CHANGE_FLAGS, pchange: *const TS_TEXTCHANGE) -> ::windows_sys::core::HRESULT,
    pub OnSelectionChange: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub OnLayoutChange: unsafe extern "system" fn(this: *mut *mut Self, lcode: TsLayoutCode, vcview: u32) -> ::windows_sys::core::HRESULT,
    pub OnStatusChange: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32) -> ::windows_sys::core::HRESULT,
    pub OnAttrsChange: unsafe extern "system" fn(this: *mut *mut Self, acpstart: i32, acpend: i32, cattrs: u32, paattrs: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub OnLockGranted: unsafe extern "system" fn(this: *mut *mut Self, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows_sys::core::HRESULT,
    pub OnStartEditTransaction: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub OnEndEditTransaction: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITextStoreACPSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 584338580, data2: 42009, data3: 17730, data4: [162, 114, 174, 38, 9, 62, 206, 207] };
}
#[repr(C)]
pub struct ITextStoreACPSinkEx {
    pub base__: ITextStoreACPSink,
    pub OnDisconnect: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITextStoreACPSinkEx {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 736072804, data2: 16866, data3: 17379, data4: [149, 12, 166, 134, 91, 162, 92, 212] };
}
#[repr(C)]
pub struct ITextStoreAnchor {
    pub base__: ::windows_sys::core::IUnknown,
    pub AdviseSink: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, punk: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows_sys::core::HRESULT,
    pub UnadviseSink: unsafe extern "system" fn(this: *mut *mut Self, punk: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RequestLock: unsafe extern "system" fn(this: *mut *mut Self, dwlockflags: u32, phrsession: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, pdcs: *mut TS_STATUS) -> ::windows_sys::core::HRESULT,
    pub QueryInsert: unsafe extern "system" fn(this: *mut *mut Self, pateststart: *mut ::core::ffi::c_void, patestend: *mut ::core::ffi::c_void, cch: u32, pparesultstart: *mut *mut ::core::ffi::c_void, pparesultend: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSelection: unsafe extern "system" fn(this: *mut *mut Self, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ANCHOR, pcfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSelection: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSelection: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32, pselection: *const TS_SELECTION_ANCHOR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSelection: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetText: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, pastart: *mut ::core::ffi::c_void, paend: *mut ::core::ffi::c_void, pchtext: ::windows_sys::core::PWSTR, cchreq: u32, pcch: *mut u32, fupdateanchor: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetText: usize,
    pub SetText: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, pastart: *mut ::core::ffi::c_void, paend: *mut ::core::ffi::c_void, pchtext: ::windows_sys::core::PCWSTR, cch: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFormattedText: unsafe extern "system" fn(this: *mut *mut Self, pastart: *mut ::core::ffi::c_void, paend: *mut ::core::ffi::c_void, ppdataobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFormattedText: usize,
    pub GetEmbedded: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, papos: *mut ::core::ffi::c_void, rguidservice: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InsertEmbedded: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, pastart: *mut ::core::ffi::c_void, paend: *mut ::core::ffi::c_void, pdataobject: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InsertEmbedded: usize,
    pub RequestSupportedAttrs: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub RequestAttrsAtPosition: unsafe extern "system" fn(this: *mut *mut Self, papos: *mut ::core::ffi::c_void, cfilterattrs: u32, pafilterattrs: *const ::windows_sys::core::GUID, dwflags: u32) -> ::windows_sys::core::HRESULT,
    pub RequestAttrsTransitioningAtPosition: unsafe extern "system" fn(this: *mut *mut Self, papos: *mut ::core::ffi::c_void, cfilterattrs: u32, pafilterattrs: *const ::windows_sys::core::GUID, dwflags: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FindNextAttrTransition: unsafe extern "system" fn(this: *mut *mut Self, pastart: *mut ::core::ffi::c_void, pahalt: *mut ::core::ffi::c_void, cfilterattrs: u32, pafilterattrs: *const ::windows_sys::core::GUID, dwflags: u32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FindNextAttrTransition: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RetrieveRequestedAttrs: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RetrieveRequestedAttrs: usize,
    pub GetStart: unsafe extern "system" fn(this: *mut *mut Self, ppastart: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetEnd: unsafe extern "system" fn(this: *mut *mut Self, ppaend: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetActiveView: unsafe extern "system" fn(this: *mut *mut Self, pvcview: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAnchorFromPoint: unsafe extern "system" fn(this: *mut *mut Self, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, ppasite: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAnchorFromPoint: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTextExt: unsafe extern "system" fn(this: *mut *mut Self, vcview: u32, pastart: *mut ::core::ffi::c_void, paend: *mut ::core::ffi::c_void, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTextExt: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetScreenExt: unsafe extern "system" fn(this: *mut *mut Self, vcview: u32, prc: *mut super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetScreenExt: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetWnd: unsafe extern "system" fn(this: *mut *mut Self, vcview: u32, phwnd: *mut super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetWnd: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub QueryInsertEmbedded: unsafe extern "system" fn(this: *mut *mut Self, pguidservice: *const ::windows_sys::core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    QueryInsertEmbedded: usize,
    pub InsertTextAtSelection: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, pchtext: ::windows_sys::core::PCWSTR, cch: u32, ppastart: *mut *mut ::core::ffi::c_void, ppaend: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InsertEmbeddedAtSelection: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, pdataobject: *mut ::core::ffi::c_void, ppastart: *mut *mut ::core::ffi::c_void, ppaend: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InsertEmbeddedAtSelection: usize,
}
impl ::windows_sys::core::Interface for ITextStoreAnchor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2602596272, data2: 24344, data3: 19948, data4: [190, 233, 60, 199, 34, 245, 223, 224] };
}
#[repr(C)]
pub struct ITextStoreAnchorEx {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub ScrollToRect: unsafe extern "system" fn(this: *mut *mut Self, pstart: *mut ::core::ffi::c_void, pend: *mut ::core::ffi::c_void, rc: super::super::Foundation::RECT, dwposition: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ScrollToRect: usize,
}
impl ::windows_sys::core::Interface for ITextStoreAnchorEx {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2732473281, data2: 15758, data3: 4563, data4: [129, 169, 247, 83, 251, 230, 26, 0] };
}
#[repr(C)]
pub struct ITextStoreAnchorSink {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnTextChange: unsafe extern "system" fn(this: *mut *mut Self, dwflags: TEXT_STORE_CHANGE_FLAGS, pastart: *mut ::core::ffi::c_void, paend: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnSelectionChange: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub OnLayoutChange: unsafe extern "system" fn(this: *mut *mut Self, lcode: TsLayoutCode, vcview: u32) -> ::windows_sys::core::HRESULT,
    pub OnStatusChange: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32) -> ::windows_sys::core::HRESULT,
    pub OnAttrsChange: unsafe extern "system" fn(this: *mut *mut Self, pastart: *mut ::core::ffi::c_void, paend: *mut ::core::ffi::c_void, cattrs: u32, paattrs: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub OnLockGranted: unsafe extern "system" fn(this: *mut *mut Self, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows_sys::core::HRESULT,
    pub OnStartEditTransaction: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub OnEndEditTransaction: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITextStoreAnchorSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2860574981, data2: 8225, data3: 4562, data4: [147, 224, 0, 96, 176, 103, 184, 110] };
}
#[repr(C)]
pub struct ITextStoreSinkAnchorEx {
    pub base__: ITextStoreAnchorSink,
    pub OnDisconnect: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITextStoreSinkAnchorEx {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 627319846, data2: 653, data3: 17524, data4: [151, 123, 17, 27, 177, 20, 254, 62] };
}
#[repr(C)]
pub struct ITfActiveLanguageProfileNotifySink {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub OnActivated: unsafe extern "system" fn(this: *mut *mut Self, clsid: *const ::windows_sys::core::GUID, guidprofile: *const ::windows_sys::core::GUID, factivated: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnActivated: usize,
}
impl ::windows_sys::core::Interface for ITfActiveLanguageProfileNotifySink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2990984053, data2: 43326, data3: 18002, data4: [191, 140, 179, 254, 12, 253, 126, 87] };
}
#[repr(C)]
pub struct ITfCandidateList {
    pub base__: ::windows_sys::core::IUnknown,
    pub EnumCandidates: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetCandidate: unsafe extern "system" fn(this: *mut *mut Self, nindex: u32, ppcand: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetCandidateNum: unsafe extern "system" fn(this: *mut *mut Self, pncnt: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetResult: unsafe extern "system" fn(this: *mut *mut Self, nindex: u32, imcr: TfCandidateResult) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfCandidateList {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2746044667, data2: 39899, data3: 18915, data4: [168, 67, 108, 118, 82, 15, 191, 93] };
}
#[repr(C)]
pub struct ITfCandidateListUIElement {
    pub base__: ITfUIElement,
    pub GetUpdatedFlags: unsafe extern "system" fn(this: *mut *mut Self, pdwflags: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetDocumentMgr: unsafe extern "system" fn(this: *mut *mut Self, ppdim: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self, pucount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetSelection: unsafe extern "system" fn(this: *mut *mut Self, puindex: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetString: unsafe extern "system" fn(this: *mut *mut Self, uindex: u32, pstr: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetString: usize,
    pub GetPageIndex: unsafe extern "system" fn(this: *mut *mut Self, pindex: *mut u32, usize: u32, pupagecnt: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetPageIndex: unsafe extern "system" fn(this: *mut *mut Self, pindex: *const u32, upagecnt: u32) -> ::windows_sys::core::HRESULT,
    pub GetCurrentPage: unsafe extern "system" fn(this: *mut *mut Self, pupage: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfCandidateListUIElement {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3927875896, data2: 6623, data3: 4567, data4: [166, 210, 0, 6, 91, 132, 67, 92] };
}
#[repr(C)]
pub struct ITfCandidateListUIElementBehavior {
    pub base__: ITfCandidateListUIElement,
    pub SetSelection: unsafe extern "system" fn(this: *mut *mut Self, nindex: u32) -> ::windows_sys::core::HRESULT,
    pub Finalize: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Abort: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfCandidateListUIElementBehavior {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2247807365, data2: 22734, data3: 18810, data4: [148, 96, 53, 83, 102, 182, 75, 154] };
}
#[repr(C)]
pub struct ITfCandidateString {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetString: unsafe extern "system" fn(this: *mut *mut Self, pbstr: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetString: usize,
    pub GetIndex: unsafe extern "system" fn(this: *mut *mut Self, pnindex: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfCandidateString {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1478439294, data2: 64925, data3: 17471, data4: [185, 114, 237, 0, 70, 124, 93, 64] };
}
#[repr(C)]
pub struct ITfCategoryMgr {
    pub base__: ::windows_sys::core::IUnknown,
    pub RegisterCategory: unsafe extern "system" fn(this: *mut *mut Self, rclsid: *const ::windows_sys::core::GUID, rcatid: *const ::windows_sys::core::GUID, rguid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub UnregisterCategory: unsafe extern "system" fn(this: *mut *mut Self, rclsid: *const ::windows_sys::core::GUID, rcatid: *const ::windows_sys::core::GUID, rguid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumCategoriesInItem: unsafe extern "system" fn(this: *mut *mut Self, rguid: *const ::windows_sys::core::GUID, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumCategoriesInItem: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumItemsInCategory: unsafe extern "system" fn(this: *mut *mut Self, rcatid: *const ::windows_sys::core::GUID, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumItemsInCategory: usize,
    pub FindClosestCategory: unsafe extern "system" fn(this: *mut *mut Self, rguid: *const ::windows_sys::core::GUID, pcatid: *mut ::windows_sys::core::GUID, ppcatidlist: *const *const ::windows_sys::core::GUID, ulcount: u32) -> ::windows_sys::core::HRESULT,
    pub RegisterGUIDDescription: unsafe extern "system" fn(this: *mut *mut Self, rclsid: *const ::windows_sys::core::GUID, rguid: *const ::windows_sys::core::GUID, pchdesc: ::windows_sys::core::PCWSTR, cch: u32) -> ::windows_sys::core::HRESULT,
    pub UnregisterGUIDDescription: unsafe extern "system" fn(this: *mut *mut Self, rclsid: *const ::windows_sys::core::GUID, rguid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetGUIDDescription: unsafe extern "system" fn(this: *mut *mut Self, rguid: *const ::windows_sys::core::GUID, pbstrdesc: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetGUIDDescription: usize,
    pub RegisterGUIDDWORD: unsafe extern "system" fn(this: *mut *mut Self, rclsid: *const ::windows_sys::core::GUID, rguid: *const ::windows_sys::core::GUID, dw: u32) -> ::windows_sys::core::HRESULT,
    pub UnregisterGUIDDWORD: unsafe extern "system" fn(this: *mut *mut Self, rclsid: *const ::windows_sys::core::GUID, rguid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetGUIDDWORD: unsafe extern "system" fn(this: *mut *mut Self, rguid: *const ::windows_sys::core::GUID, pdw: *mut u32) -> ::windows_sys::core::HRESULT,
    pub RegisterGUID: unsafe extern "system" fn(this: *mut *mut Self, rguid: *const ::windows_sys::core::GUID, pguidatom: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetGUID: unsafe extern "system" fn(this: *mut *mut Self, guidatom: u32, pguid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsEqualTfGuidAtom: unsafe extern "system" fn(this: *mut *mut Self, guidatom: u32, rguid: *const ::windows_sys::core::GUID, pfequal: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsEqualTfGuidAtom: usize,
}
impl ::windows_sys::core::Interface for ITfCategoryMgr {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3282890677, data2: 63133, data3: 18693, data4: [147, 143, 252, 173, 207, 75, 232, 48] };
}
#[repr(C)]
pub struct ITfCleanupContextDurationSink {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnStartCleanupContext: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub OnEndCleanupContext: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfCleanupContextDurationSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1170428228, data2: 5454, data3: 18327, data4: [190, 216, 211, 58, 231, 191, 135, 148] };
}
#[repr(C)]
pub struct ITfCleanupContextSink {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnCleanupContext: unsafe extern "system" fn(this: *mut *mut Self, ecwrite: u32, pic: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfCleanupContextSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 23631497, data2: 31435, data3: 20123, data4: [171, 124, 126, 164, 107, 18, 181, 34] };
}
#[repr(C)]
pub struct ITfClientId {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetClientId: unsafe extern "system" fn(this: *mut *mut Self, rclsid: *const ::windows_sys::core::GUID, ptid: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfClientId {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3591011145, data2: 7071, data3: 19426, data4: [183, 2, 71, 233, 220, 5, 222, 195] };
}
#[repr(C)]
pub struct ITfCompartment {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, tid: u32, pvarvalue: *const super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetValue: unsafe extern "system" fn(this: *mut *mut Self, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetValue: usize,
}
impl ::windows_sys::core::Interface for ITfCompartment {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3137927081, data2: 24698, data3: 17284, data4: [134, 35, 5, 104, 146, 182, 67, 113] };
}
#[repr(C)]
pub struct ITfCompartmentEventSink {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnChange: unsafe extern "system" fn(this: *mut *mut Self, rguid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfCompartmentEventSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1950006623, data2: 62061, data3: 18655, data4: [140, 197, 35, 132, 146, 65, 155, 100] };
}
#[repr(C)]
pub struct ITfCompartmentMgr {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCompartment: unsafe extern "system" fn(this: *mut *mut Self, rguid: *const ::windows_sys::core::GUID, ppcomp: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ClearCompartment: unsafe extern "system" fn(this: *mut *mut Self, tid: u32, rguid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumCompartments: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumCompartments: usize,
}
impl ::windows_sys::core::Interface for ITfCompartmentMgr {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2110740396, data2: 6317, data3: 17291, data4: [130, 77, 151, 155, 255, 183, 75, 124] };
}
#[repr(C)]
pub struct ITfComposition {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetRange: unsafe extern "system" fn(this: *mut *mut Self, pprange: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ShiftStart: unsafe extern "system" fn(this: *mut *mut Self, ecwrite: u32, pnewstart: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ShiftEnd: unsafe extern "system" fn(this: *mut *mut Self, ecwrite: u32, pnewend: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EndComposition: unsafe extern "system" fn(this: *mut *mut Self, ecwrite: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfComposition {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 538348900, data2: 23183, data3: 19034, data4: [183, 189, 207, 162, 159, 77, 15, 217] };
}
#[repr(C)]
pub struct ITfCompositionSink {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnCompositionTerminated: unsafe extern "system" fn(this: *mut *mut Self, ecwrite: u32, pcomposition: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfCompositionSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2810278284, data2: 22426, data3: 19221, data4: [162, 128, 50, 184, 87, 122, 204, 94] };
}
#[repr(C)]
pub struct ITfCompositionView {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetOwnerClsid: unsafe extern "system" fn(this: *mut *mut Self, pclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetRange: unsafe extern "system" fn(this: *mut *mut Self, pprange: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfCompositionView {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3612607041, data2: 63905, data3: 17252, data4: [190, 252, 219, 205, 44, 67, 149, 183] };
}
#[repr(C)]
pub struct ITfConfigureSystemKeystrokeFeed {
    pub base__: ::windows_sys::core::IUnknown,
    pub DisableSystemKeystrokeFeed: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub EnableSystemKeystrokeFeed: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfConfigureSystemKeystrokeFeed {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 221025946, data2: 48284, data3: 17276, data4: [132, 238, 149, 28, 73, 177, 167, 100] };
}
#[repr(C)]
pub struct ITfContext {
    pub base__: ::windows_sys::core::IUnknown,
    pub RequestEditSession: unsafe extern "system" fn(this: *mut *mut Self, tid: u32, pes: *mut ::core::ffi::c_void, dwflags: TF_CONTEXT_EDIT_CONTEXT_FLAGS, phrsession: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub InWriteSession: unsafe extern "system" fn(this: *mut *mut Self, tid: u32, pfwritesession: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InWriteSession: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSelection: unsafe extern "system" fn(this: *mut *mut Self, ec: u32, ulindex: u32, ulcount: u32, pselection: *mut TF_SELECTION, pcfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSelection: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSelection: unsafe extern "system" fn(this: *mut *mut Self, ec: u32, ulcount: u32, pselection: *const TF_SELECTION) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSelection: usize,
    pub GetStart: unsafe extern "system" fn(this: *mut *mut Self, ec: u32, ppstart: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetEnd: unsafe extern "system" fn(this: *mut *mut Self, ec: u32, ppend: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetActiveView: unsafe extern "system" fn(this: *mut *mut Self, ppview: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EnumViews: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, pdcs: *mut TS_STATUS) -> ::windows_sys::core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(this: *mut *mut Self, guidprop: *const ::windows_sys::core::GUID, ppprop: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetAppProperty: unsafe extern "system" fn(this: *mut *mut Self, guidprop: *const ::windows_sys::core::GUID, ppprop: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TrackProperties: unsafe extern "system" fn(this: *mut *mut Self, prgprop: *const *const ::windows_sys::core::GUID, cprop: u32, prgappprop: *const *const ::windows_sys::core::GUID, cappprop: u32, ppproperty: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EnumProperties: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDocumentMgr: unsafe extern "system" fn(this: *mut *mut Self, ppdm: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateRangeBackup: unsafe extern "system" fn(this: *mut *mut Self, ec: u32, prange: *mut ::core::ffi::c_void, ppbackup: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfContext {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2860574717, data2: 8225, data3: 4562, data4: [147, 224, 0, 96, 176, 103, 184, 110] };
}
#[repr(C)]
pub struct ITfContextComposition {
    pub base__: ::windows_sys::core::IUnknown,
    pub StartComposition: unsafe extern "system" fn(this: *mut *mut Self, ecwrite: u32, pcompositionrange: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void, ppcomposition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EnumCompositions: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FindComposition: unsafe extern "system" fn(this: *mut *mut Self, ecread: u32, ptestrange: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TakeOwnership: unsafe extern "system" fn(this: *mut *mut Self, ecwrite: u32, pcomposition: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void, ppcomposition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfContextComposition {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3557591726, data2: 44178, data3: 20423, data4: [154, 17, 14, 224, 226, 58, 163, 155] };
}
#[repr(C)]
pub struct ITfContextKeyEventSink {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub OnKeyDown: unsafe extern "system" fn(this: *mut *mut Self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnKeyDown: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnKeyUp: unsafe extern "system" fn(this: *mut *mut Self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnKeyUp: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnTestKeyDown: unsafe extern "system" fn(this: *mut *mut Self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnTestKeyDown: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnTestKeyUp: unsafe extern "system" fn(this: *mut *mut Self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnTestKeyUp: usize,
}
impl ::windows_sys::core::Interface for ITfContextKeyEventSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 89307741, data2: 51253, data3: 18740, data4: [191, 80, 132, 106, 170, 103, 67, 47] };
}
#[repr(C)]
pub struct ITfContextOwner {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetACPFromPoint: unsafe extern "system" fn(this: *mut *mut Self, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, pacp: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetACPFromPoint: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTextExt: unsafe extern "system" fn(this: *mut *mut Self, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTextExt: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetScreenExt: unsafe extern "system" fn(this: *mut *mut Self, prc: *mut super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetScreenExt: usize,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, pdcs: *mut TS_STATUS) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetWnd: unsafe extern "system" fn(this: *mut *mut Self, phwnd: *mut super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetWnd: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetAttribute: unsafe extern "system" fn(this: *mut *mut Self, rguidattribute: *const ::windows_sys::core::GUID, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetAttribute: usize,
}
impl ::windows_sys::core::Interface for ITfContextOwner {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2860574732, data2: 8225, data3: 4562, data4: [147, 224, 0, 96, 176, 103, 184, 110] };
}
#[repr(C)]
pub struct ITfContextOwnerCompositionServices {
    pub base__: ITfContextComposition,
    pub TerminateComposition: unsafe extern "system" fn(this: *mut *mut Self, pcomposition: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfContextOwnerCompositionServices {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2252744720, data2: 22843, data3: 18710, data4: [151, 100, 25, 192, 142, 156, 225, 16] };
}
#[repr(C)]
pub struct ITfContextOwnerCompositionSink {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub OnStartComposition: unsafe extern "system" fn(this: *mut *mut Self, pcomposition: *mut ::core::ffi::c_void, pfok: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnStartComposition: usize,
    pub OnUpdateComposition: unsafe extern "system" fn(this: *mut *mut Self, pcomposition: *mut ::core::ffi::c_void, prangenew: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnEndComposition: unsafe extern "system" fn(this: *mut *mut Self, pcomposition: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfContextOwnerCompositionSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1595976256, data2: 46458, data3: 20276, data4: [150, 171, 53, 118, 243, 119, 204, 121] };
}
#[repr(C)]
pub struct ITfContextOwnerServices {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnLayoutChange: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub OnStatusChange: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32) -> ::windows_sys::core::HRESULT,
    pub OnAttributeChange: unsafe extern "system" fn(this: *mut *mut Self, rguidattribute: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Serialize: unsafe extern "system" fn(this: *mut *mut Self, pprop: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Serialize: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Unserialize: unsafe extern "system" fn(this: *mut *mut Self, pprop: *mut ::core::ffi::c_void, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: *mut ::core::ffi::c_void, ploader: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Unserialize: usize,
    pub ForceLoadProperty: unsafe extern "system" fn(this: *mut *mut Self, pprop: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateRange: unsafe extern "system" fn(this: *mut *mut Self, acpstart: i32, acpend: i32, pprange: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfContextOwnerServices {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2990454320, data2: 15900, data3: 4563, data4: [167, 69, 0, 80, 4, 10, 180, 7] };
}
#[repr(C)]
pub struct ITfContextView {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRangeFromPoint: unsafe extern "system" fn(this: *mut *mut Self, ec: u32, ppt: *const super::super::Foundation::POINT, dwflags: u32, pprange: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRangeFromPoint: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTextExt: unsafe extern "system" fn(this: *mut *mut Self, ec: u32, prange: *mut ::core::ffi::c_void, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTextExt: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetScreenExt: unsafe extern "system" fn(this: *mut *mut Self, prc: *mut super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetScreenExt: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetWnd: unsafe extern "system" fn(this: *mut *mut Self, phwnd: *mut super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetWnd: usize,
}
impl ::windows_sys::core::Interface for ITfContextView {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 607371150, data2: 3995, data3: 17244, data4: [186, 44, 24, 6, 17, 151, 140, 48] };
}
#[repr(C)]
pub struct ITfCreatePropertyStore {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub IsStoreSerializable: unsafe extern "system" fn(this: *mut *mut Self, guidprop: *const ::windows_sys::core::GUID, prange: *mut ::core::ffi::c_void, ppropstore: *mut ::core::ffi::c_void, pfserializable: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsStoreSerializable: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePropertyStore: unsafe extern "system" fn(this: *mut *mut Self, guidprop: *const ::windows_sys::core::GUID, prange: *mut ::core::ffi::c_void, cb: u32, pstream: *mut ::core::ffi::c_void, ppstore: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePropertyStore: usize,
}
impl ::windows_sys::core::Interface for ITfCreatePropertyStore {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 610532336, data2: 45231, data3: 4562, data4: [175, 197, 0, 16, 90, 39, 153, 181] };
}
#[repr(C)]
pub struct ITfDisplayAttributeInfo {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetGUID: unsafe extern "system" fn(this: *mut *mut Self, pguid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDescription: unsafe extern "system" fn(this: *mut *mut Self, pbstrdesc: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDescription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAttributeInfo: unsafe extern "system" fn(this: *mut *mut Self, pda: *mut TF_DISPLAYATTRIBUTE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAttributeInfo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAttributeInfo: unsafe extern "system" fn(this: *mut *mut Self, pda: *const TF_DISPLAYATTRIBUTE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAttributeInfo: usize,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfDisplayAttributeInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1884457042, data2: 12070, data3: 19178, data4: [140, 150, 33, 81, 80, 87, 137, 50] };
}
#[repr(C)]
pub struct ITfDisplayAttributeMgr {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnUpdateInfo: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub EnumDisplayAttributeInfo: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDisplayAttributeInfo: unsafe extern "system" fn(this: *mut *mut Self, guid: *const ::windows_sys::core::GUID, ppinfo: *mut *mut ::core::ffi::c_void, pclsidowner: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfDisplayAttributeMgr {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2381149075, data2: 23985, data3: 18268, data4: [158, 113, 163, 145, 17, 176, 255, 103] };
}
#[repr(C)]
pub struct ITfDisplayAttributeNotifySink {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnUpdateInfo: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfDisplayAttributeNotifySink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2908156930, data2: 57698, data3: 20261, data4: [144, 143, 125, 87, 124, 249, 189, 169] };
}
#[repr(C)]
pub struct ITfDisplayAttributeProvider {
    pub base__: ::windows_sys::core::IUnknown,
    pub EnumDisplayAttributeInfo: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDisplayAttributeInfo: unsafe extern "system" fn(this: *mut *mut Self, guid: *const ::windows_sys::core::GUID, ppinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfDisplayAttributeProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4276385655, data2: 5692, data3: 18281, data4: [153, 106, 110, 156, 80, 173, 143, 84] };
}
#[repr(C)]
pub struct ITfDocumentMgr {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateContext: unsafe extern "system" fn(this: *mut *mut Self, tidowner: u32, dwflags: u32, punk: *mut ::core::ffi::c_void, ppic: *mut *mut ::core::ffi::c_void, pectextstore: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Push: unsafe extern "system" fn(this: *mut *mut Self, pic: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Pop: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32) -> ::windows_sys::core::HRESULT,
    pub GetTop: unsafe extern "system" fn(this: *mut *mut Self, ppic: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetBase: unsafe extern "system" fn(this: *mut *mut Self, ppic: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EnumContexts: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfDocumentMgr {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2860574708, data2: 8225, data3: 4562, data4: [147, 224, 0, 96, 176, 103, 184, 110] };
}
#[repr(C)]
pub struct ITfEditRecord {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSelectionStatus: unsafe extern "system" fn(this: *mut *mut Self, pfchanged: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSelectionStatus: usize,
    pub GetTextAndPropertyUpdates: unsafe extern "system" fn(this: *mut *mut Self, dwflags: GET_TEXT_AND_PROPERTY_UPDATES_FLAGS, prgproperties: *const *const ::windows_sys::core::GUID, cproperties: u32, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfEditRecord {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1121243289, data2: 31770, data3: 19081, data4: [184, 54, 108, 111, 34, 22, 13, 240] };
}
#[repr(C)]
pub struct ITfEditSession {
    pub base__: ::windows_sys::core::IUnknown,
    pub DoEditSession: unsafe extern "system" fn(this: *mut *mut Self, ec: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfEditSession {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2860574723, data2: 8225, data3: 4562, data4: [147, 224, 0, 96, 176, 103, 184, 110] };
}
#[repr(C)]
pub struct ITfEditTransactionSink {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnStartEditTransaction: unsafe extern "system" fn(this: *mut *mut Self, pic: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnEndEditTransaction: unsafe extern "system" fn(this: *mut *mut Self, pic: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfEditTransactionSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1888468848, data2: 46368, data3: 16747, data4: [176, 108, 44, 65, 171, 68, 248, 186] };
}
#[repr(C)]
pub struct ITfFnAdviseText {
    pub base__: ITfFunction,
    pub OnTextUpdate: unsafe extern "system" fn(this: *mut *mut Self, prange: *mut ::core::ffi::c_void, pchtext: ::windows_sys::core::PCWSTR, cch: i32) -> ::windows_sys::core::HRESULT,
    pub OnLatticeUpdate: unsafe extern "system" fn(this: *mut *mut Self, prange: *mut ::core::ffi::c_void, plattice: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfFnAdviseText {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 891758219, data2: 32083, data3: 19929, data4: [146, 183, 114, 150, 174, 70, 18, 73] };
}
#[repr(C)]
pub struct ITfFnBalloon {
    pub base__: ::windows_sys::core::IUnknown,
    pub UpdateBalloon: unsafe extern "system" fn(this: *mut *mut Self, style: TfLBBalloonStyle, pch: ::windows_sys::core::PCWSTR, cch: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfFnBalloon {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1001097700, data2: 24510, data3: 17908, data4: [165, 188, 220, 163, 106, 210, 37, 168] };
}
#[repr(C)]
pub struct ITfFnConfigure {
    pub base__: ITfFunction,
    #[cfg(feature = "Win32_Foundation")]
    pub Show: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Show: usize,
}
impl ::windows_sys::core::Interface for ITfFnConfigure {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2297784262, data2: 5975, data3: 18936, data4: [161, 178, 137, 35, 76, 30, 239, 249] };
}
#[repr(C)]
pub struct ITfFnConfigureRegisterEudc {
    pub base__: ITfFunction,
    #[cfg(feature = "Win32_Foundation")]
    pub Show: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows_sys::core::GUID, bstrregistered: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Show: usize,
}
impl ::windows_sys::core::Interface for ITfFnConfigureRegisterEudc {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3051515893, data2: 55213, data3: 17156, data4: [145, 63, 33, 162, 237, 149, 161, 176] };
}
#[repr(C)]
pub struct ITfFnConfigureRegisterWord {
    pub base__: ITfFunction,
    #[cfg(feature = "Win32_Foundation")]
    pub Show: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows_sys::core::GUID, bstrregistered: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Show: usize,
}
impl ::windows_sys::core::Interface for ITfFnConfigureRegisterWord {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3147137162, data2: 28047, data3: 19402, data4: [132, 0, 83, 144, 181, 134, 174, 223] };
}
#[repr(C)]
pub struct ITfFnCustomSpeechCommand {
    pub base__: ITfFunction,
    pub SetSpeechCommandProvider: unsafe extern "system" fn(this: *mut *mut Self, pspcmdprovider: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfFnCustomSpeechCommand {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4238787401, data2: 41263, data3: 17315, data4: [141, 214, 90, 90, 66, 130, 87, 123] };
}
#[repr(C)]
pub struct ITfFnGetLinguisticAlternates {
    pub base__: ITfFunction,
    pub GetAlternates: unsafe extern "system" fn(this: *mut *mut Self, prange: *mut ::core::ffi::c_void, ppcandidatelist: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfFnGetLinguisticAlternates {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3927325922, data2: 31333, data3: 17670, data4: [130, 163, 197, 40, 33, 93, 166, 78] };
}
#[repr(C)]
pub struct ITfFnGetPreferredTouchKeyboardLayout {
    pub base__: ITfFunction,
    pub GetLayout: unsafe extern "system" fn(this: *mut *mut Self, ptkblayouttype: *mut TKBLayoutType, pwpreferredlayoutid: *const u16) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfFnGetPreferredTouchKeyboardLayout {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1597020737, data2: 22794, data3: 19148, data4: [169, 127, 216, 239, 255, 19, 253, 252] };
}
#[repr(C)]
pub struct ITfFnGetSAPIObject {
    pub base__: ITfFunction,
    pub Get: unsafe extern "system" fn(this: *mut *mut Self, sobj: TfSapiObject, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfFnGetSAPIObject {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1544206314, data2: 5757, data3: 20313, data4: [191, 181, 70, 147, 117, 94, 144, 202] };
}
#[repr(C)]
pub struct ITfFnLMInternal {
    pub base__: ITfFnLMProcessor,
    pub ProcessLattice: unsafe extern "system" fn(this: *mut *mut Self, prange: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfFnLMInternal {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 79177137, data2: 44186, data3: 20347, data4: [181, 173, 199, 22, 143, 30, 228, 69] };
}
#[repr(C)]
pub struct ITfFnLMProcessor {
    pub base__: ITfFunction,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryRange: unsafe extern "system" fn(this: *mut *mut Self, prange: *mut ::core::ffi::c_void, ppnewrange: *mut *mut ::core::ffi::c_void, pfaccepted: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryRange: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryLangID: unsafe extern "system" fn(this: *mut *mut Self, langid: u16, pfaccepted: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryLangID: usize,
    pub GetReconversion: unsafe extern "system" fn(this: *mut *mut Self, prange: *mut ::core::ffi::c_void, ppcandlist: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Reconvert: unsafe extern "system" fn(this: *mut *mut Self, prange: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryKey: unsafe extern "system" fn(this: *mut *mut Self, fup: super::super::Foundation::BOOL, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM, pfinterested: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryKey: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InvokeKey: unsafe extern "system" fn(this: *mut *mut Self, fup: super::super::Foundation::BOOL, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InvokeKey: usize,
    pub InvokeFunc: unsafe extern "system" fn(this: *mut *mut Self, pic: *mut ::core::ffi::c_void, refguidfunc: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfFnLMProcessor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2063333607, data2: 44107, data3: 16514, data4: [176, 88, 137, 8, 153, 211, 160, 16] };
}
#[repr(C)]
pub struct ITfFnLangProfileUtil {
    pub base__: ITfFunction,
    pub RegisterActiveProfiles: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsProfileAvailableForLang: unsafe extern "system" fn(this: *mut *mut Self, langid: u16, pfavailable: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsProfileAvailableForLang: usize,
}
impl ::windows_sys::core::Interface for ITfFnLangProfileUtil {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2826601844, data2: 42689, data3: 19989, data4: [153, 240, 61, 57, 101, 245, 72, 235] };
}
#[repr(C)]
pub struct ITfFnPlayBack {
    pub base__: ITfFunction,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryRange: unsafe extern "system" fn(this: *mut *mut Self, prange: *mut ::core::ffi::c_void, ppnewrange: *mut *mut ::core::ffi::c_void, pfplayable: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryRange: usize,
    pub Play: unsafe extern "system" fn(this: *mut *mut Self, prange: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfFnPlayBack {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2745439908, data2: 3940, data3: 4563, data4: [181, 183, 0, 192, 79, 195, 36, 161] };
}
#[repr(C)]
pub struct ITfFnPropertyUIStatus {
    pub base__: ITfFunction,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, refguidprop: *const ::windows_sys::core::GUID, pdw: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(this: *mut *mut Self, refguidprop: *const ::windows_sys::core::GUID, dw: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfFnPropertyUIStatus {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 590916718, data2: 11165, data3: 17600, data4: [167, 94, 238, 100, 242, 86, 179, 189] };
}
#[repr(C)]
pub struct ITfFnReconversion {
    pub base__: ITfFunction,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryRange: unsafe extern "system" fn(this: *mut *mut Self, prange: *mut ::core::ffi::c_void, ppnewrange: *mut *mut ::core::ffi::c_void, pfconvertable: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryRange: usize,
    pub GetReconversion: unsafe extern "system" fn(this: *mut *mut Self, prange: *mut ::core::ffi::c_void, ppcandlist: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Reconvert: unsafe extern "system" fn(this: *mut *mut Self, prange: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfFnReconversion {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1290441664, data2: 2648, data3: 4563, data4: [141, 240, 0, 16, 90, 39, 153, 181] };
}
#[repr(C)]
pub struct ITfFnSearchCandidateProvider {
    pub base__: ITfFunction,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSearchCandidates: unsafe extern "system" fn(this: *mut *mut Self, bstrquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrapplicationid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pplist: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSearchCandidates: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetResult: unsafe extern "system" fn(this: *mut *mut Self, bstrquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrapplicationid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrresult: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetResult: usize,
}
impl ::windows_sys::core::Interface for ITfFnSearchCandidateProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2275585423, data2: 62075, data3: 18720, data4: [133, 1, 103, 96, 34, 128, 23, 93] };
}
#[repr(C)]
pub struct ITfFnShowHelp {
    pub base__: ITfFunction,
    #[cfg(feature = "Win32_Foundation")]
    pub Show: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Show: usize,
}
impl ::windows_sys::core::Interface for ITfFnShowHelp {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1521603340, data2: 2381, data3: 19497, data4: [142, 165, 11, 245, 155, 232, 123, 243] };
}
#[repr(C)]
pub struct ITfFunction {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDisplayName: usize,
}
impl ::windows_sys::core::Interface for ITfFunction {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3680056464, data2: 2447, data3: 4563, data4: [141, 240, 0, 16, 90, 39, 153, 181] };
}
#[repr(C)]
pub struct ITfFunctionProvider {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetType: unsafe extern "system" fn(this: *mut *mut Self, pguid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDescription: unsafe extern "system" fn(this: *mut *mut Self, pbstrdesc: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDescription: usize,
    pub GetFunction: unsafe extern "system" fn(this: *mut *mut Self, rguid: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfFunctionProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 270362128, data2: 2448, data3: 4563, data4: [141, 240, 0, 16, 90, 39, 153, 181] };
}
#[repr(C)]
pub struct ITfInputProcessorProfileActivationSink {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnActivated: unsafe extern "system" fn(this: *mut *mut Self, dwprofiletype: u32, langid: u16, clsid: *const ::windows_sys::core::GUID, catid: *const ::windows_sys::core::GUID, guidprofile: *const ::windows_sys::core::GUID, hkl: HKL, dwflags: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfInputProcessorProfileActivationSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1908860750, data2: 3880, data3: 4568, data4: [168, 42, 0, 6, 91, 132, 67, 92] };
}
#[repr(C)]
pub struct ITfInputProcessorProfileMgr {
    pub base__: ::windows_sys::core::IUnknown,
    pub ActivateProfile: unsafe extern "system" fn(this: *mut *mut Self, dwprofiletype: u32, langid: u16, clsid: *const ::windows_sys::core::GUID, guidprofile: *const ::windows_sys::core::GUID, hkl: HKL, dwflags: u32) -> ::windows_sys::core::HRESULT,
    pub DeactivateProfile: unsafe extern "system" fn(this: *mut *mut Self, dwprofiletype: u32, langid: u16, clsid: *const ::windows_sys::core::GUID, guidprofile: *const ::windows_sys::core::GUID, hkl: HKL, dwflags: u32) -> ::windows_sys::core::HRESULT,
    pub GetProfile: unsafe extern "system" fn(this: *mut *mut Self, dwprofiletype: u32, langid: u16, clsid: *const ::windows_sys::core::GUID, guidprofile: *const ::windows_sys::core::GUID, hkl: HKL, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> ::windows_sys::core::HRESULT,
    pub EnumProfiles: unsafe extern "system" fn(this: *mut *mut Self, langid: u16, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ReleaseInputProcessor: unsafe extern "system" fn(this: *mut *mut Self, rclsid: *const ::windows_sys::core::GUID, dwflags: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterProfile: unsafe extern "system" fn(this: *mut *mut Self, rclsid: *const ::windows_sys::core::GUID, langid: u16, guidprofile: *const ::windows_sys::core::GUID, pchdesc: ::windows_sys::core::PCWSTR, cchdesc: u32, pchiconfile: ::windows_sys::core::PCWSTR, cchfile: u32, uiconindex: u32, hklsubstitute: HKL, dwpreferredlayout: u32, benabledbydefault: super::super::Foundation::BOOL, dwflags: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterProfile: usize,
    pub UnregisterProfile: unsafe extern "system" fn(this: *mut *mut Self, rclsid: *const ::windows_sys::core::GUID, langid: u16, guidprofile: *const ::windows_sys::core::GUID, dwflags: u32) -> ::windows_sys::core::HRESULT,
    pub GetActiveProfile: unsafe extern "system" fn(this: *mut *mut Self, catid: *const ::windows_sys::core::GUID, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfInputProcessorProfileMgr {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1908860748, data2: 3880, data3: 4568, data4: [168, 42, 0, 6, 91, 132, 67, 92] };
}
#[repr(C)]
pub struct ITfInputProcessorProfileSubstituteLayout {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetSubstituteKeyboardLayout: unsafe extern "system" fn(this: *mut *mut Self, rclsid: *const ::windows_sys::core::GUID, langid: u16, guidprofile: *const ::windows_sys::core::GUID, phkl: *mut HKL) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfInputProcessorProfileSubstituteLayout {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1339453844, data2: 4098, data3: 17683, data4: [191, 242, 192, 221, 246, 37, 133, 82] };
}
#[repr(C)]
pub struct ITfInputProcessorProfiles {
    pub base__: ::windows_sys::core::IUnknown,
    pub Register: unsafe extern "system" fn(this: *mut *mut Self, rclsid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Unregister: unsafe extern "system" fn(this: *mut *mut Self, rclsid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub AddLanguageProfile: unsafe extern "system" fn(this: *mut *mut Self, rclsid: *const ::windows_sys::core::GUID, langid: u16, guidprofile: *const ::windows_sys::core::GUID, pchdesc: ::windows_sys::core::PCWSTR, cchdesc: u32, pchiconfile: ::windows_sys::core::PCWSTR, cchfile: u32, uiconindex: u32) -> ::windows_sys::core::HRESULT,
    pub RemoveLanguageProfile: unsafe extern "system" fn(this: *mut *mut Self, rclsid: *const ::windows_sys::core::GUID, langid: u16, guidprofile: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumInputProcessorInfo: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumInputProcessorInfo: usize,
    pub GetDefaultLanguageProfile: unsafe extern "system" fn(this: *mut *mut Self, langid: u16, catid: *const ::windows_sys::core::GUID, pclsid: *mut ::windows_sys::core::GUID, pguidprofile: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SetDefaultLanguageProfile: unsafe extern "system" fn(this: *mut *mut Self, langid: u16, rclsid: *const ::windows_sys::core::GUID, guidprofiles: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub ActivateLanguageProfile: unsafe extern "system" fn(this: *mut *mut Self, rclsid: *const ::windows_sys::core::GUID, langid: u16, guidprofiles: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetActiveLanguageProfile: unsafe extern "system" fn(this: *mut *mut Self, rclsid: *const ::windows_sys::core::GUID, plangid: *mut u16, pguidprofile: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLanguageProfileDescription: unsafe extern "system" fn(this: *mut *mut Self, rclsid: *const ::windows_sys::core::GUID, langid: u16, guidprofile: *const ::windows_sys::core::GUID, pbstrprofile: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLanguageProfileDescription: usize,
    pub GetCurrentLanguage: unsafe extern "system" fn(this: *mut *mut Self, plangid: *mut u16) -> ::windows_sys::core::HRESULT,
    pub ChangeCurrentLanguage: unsafe extern "system" fn(this: *mut *mut Self, langid: u16) -> ::windows_sys::core::HRESULT,
    pub GetLanguageList: unsafe extern "system" fn(this: *mut *mut Self, pplangid: *mut *mut u16, pulcount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub EnumLanguageProfiles: unsafe extern "system" fn(this: *mut *mut Self, langid: u16, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnableLanguageProfile: unsafe extern "system" fn(this: *mut *mut Self, rclsid: *const ::windows_sys::core::GUID, langid: u16, guidprofile: *const ::windows_sys::core::GUID, fenable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnableLanguageProfile: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsEnabledLanguageProfile: unsafe extern "system" fn(this: *mut *mut Self, rclsid: *const ::windows_sys::core::GUID, langid: u16, guidprofile: *const ::windows_sys::core::GUID, pfenable: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsEnabledLanguageProfile: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnableLanguageProfileByDefault: unsafe extern "system" fn(this: *mut *mut Self, rclsid: *const ::windows_sys::core::GUID, langid: u16, guidprofile: *const ::windows_sys::core::GUID, fenable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnableLanguageProfileByDefault: usize,
    pub SubstituteKeyboardLayout: unsafe extern "system" fn(this: *mut *mut Self, rclsid: *const ::windows_sys::core::GUID, langid: u16, guidprofile: *const ::windows_sys::core::GUID, hkl: HKL) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfInputProcessorProfiles {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 520271557, data2: 30786, data3: 20198, data4: [138, 11, 154, 36, 24, 58, 149, 202] };
}
#[repr(C)]
pub struct ITfInputProcessorProfilesEx {
    pub base__: ITfInputProcessorProfiles,
    pub SetLanguageProfileDisplayName: unsafe extern "system" fn(this: *mut *mut Self, rclsid: *const ::windows_sys::core::GUID, langid: u16, guidprofile: *const ::windows_sys::core::GUID, pchfile: ::windows_sys::core::PCWSTR, cchfile: u32, uresid: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfInputProcessorProfilesEx {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2301567759, data2: 65024, data3: 19009, data4: [169, 142, 252, 214, 222, 13, 53, 239] };
}
#[repr(C)]
pub struct ITfInputScope {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetInputScopes: unsafe extern "system" fn(this: *mut *mut Self, pprginputscopes: *mut *mut InputScope, pccount: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPhrase: unsafe extern "system" fn(this: *mut *mut Self, ppbstrphrases: *mut *mut super::super::Foundation::BSTR, pccount: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPhrase: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRegularExpression: unsafe extern "system" fn(this: *mut *mut Self, pbstrregexp: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRegularExpression: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSRGS: unsafe extern "system" fn(this: *mut *mut Self, pbstrsrgs: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSRGS: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetXML: unsafe extern "system" fn(this: *mut *mut Self, pbstrxml: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetXML: usize,
}
impl ::windows_sys::core::Interface for ITfInputScope {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4259441390, data2: 26916, data3: 19679, data4: [145, 231, 218, 56, 207, 245, 85, 157] };
}
#[repr(C)]
pub struct ITfInputScope2 {
    pub base__: ITfInputScope,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumWordList: unsafe extern "system" fn(this: *mut *mut Self, ppenumstring: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumWordList: usize,
}
impl ::windows_sys::core::Interface for ITfInputScope2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1462889120, data2: 27586, data3: 18049, data4: [165, 50, 146, 251, 183, 77, 124, 65] };
}
#[repr(C)]
pub struct ITfInsertAtSelection {
    pub base__: ::windows_sys::core::IUnknown,
    pub InsertTextAtSelection: unsafe extern "system" fn(this: *mut *mut Self, ec: u32, dwflags: INSERT_TEXT_AT_SELECTION_FLAGS, pchtext: ::windows_sys::core::PCWSTR, cch: i32, pprange: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InsertEmbeddedAtSelection: unsafe extern "system" fn(this: *mut *mut Self, ec: u32, dwflags: u32, pdataobject: *mut ::core::ffi::c_void, pprange: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InsertEmbeddedAtSelection: usize,
}
impl ::windows_sys::core::Interface for ITfInsertAtSelection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1439569594, data2: 12308, data3: 16833, data4: [156, 235, 250, 222, 20, 70, 172, 108] };
}
#[repr(C)]
pub struct ITfIntegratableCandidateListUIElement {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetIntegrationStyle: unsafe extern "system" fn(this: *mut *mut Self, guidintegrationstyle: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetSelectionStyle: unsafe extern "system" fn(this: *mut *mut Self, ptfselectionstyle: *mut TfIntegratableCandidateListSelectionStyle) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnKeyDown: unsafe extern "system" fn(this: *mut *mut Self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnKeyDown: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowCandidateNumbers: unsafe extern "system" fn(this: *mut *mut Self, pfshow: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowCandidateNumbers: usize,
    pub FinalizeExactCompositionString: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfIntegratableCandidateListUIElement {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3349607759, data2: 45440, data3: 16751, data4: [178, 191, 123, 242, 228, 104, 61, 123] };
}
#[repr(C)]
pub struct ITfKeyEventSink {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub OnSetFocus: unsafe extern "system" fn(this: *mut *mut Self, fforeground: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnSetFocus: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnTestKeyDown: unsafe extern "system" fn(this: *mut *mut Self, pic: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnTestKeyDown: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnTestKeyUp: unsafe extern "system" fn(this: *mut *mut Self, pic: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnTestKeyUp: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnKeyDown: unsafe extern "system" fn(this: *mut *mut Self, pic: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnKeyDown: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnKeyUp: unsafe extern "system" fn(this: *mut *mut Self, pic: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnKeyUp: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnPreservedKey: unsafe extern "system" fn(this: *mut *mut Self, pic: *mut ::core::ffi::c_void, rguid: *const ::windows_sys::core::GUID, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnPreservedKey: usize,
}
impl ::windows_sys::core::Interface for ITfKeyEventSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2860574709, data2: 8225, data3: 4562, data4: [147, 224, 0, 96, 176, 103, 184, 110] };
}
#[repr(C)]
pub struct ITfKeyTraceEventSink {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub OnKeyTraceDown: unsafe extern "system" fn(this: *mut *mut Self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnKeyTraceDown: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnKeyTraceUp: unsafe extern "system" fn(this: *mut *mut Self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnKeyTraceUp: usize,
}
impl ::windows_sys::core::Interface for ITfKeyTraceEventSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 483705147, data2: 7222, data3: 16785, data4: [167, 10, 127, 62, 97, 31, 54, 125] };
}
#[repr(C)]
pub struct ITfKeystrokeMgr {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub AdviseKeyEventSink: unsafe extern "system" fn(this: *mut *mut Self, tid: u32, psink: *mut ::core::ffi::c_void, fforeground: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AdviseKeyEventSink: usize,
    pub UnadviseKeyEventSink: unsafe extern "system" fn(this: *mut *mut Self, tid: u32) -> ::windows_sys::core::HRESULT,
    pub GetForeground: unsafe extern "system" fn(this: *mut *mut Self, pclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TestKeyDown: unsafe extern "system" fn(this: *mut *mut Self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TestKeyDown: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TestKeyUp: unsafe extern "system" fn(this: *mut *mut Self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TestKeyUp: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub KeyDown: unsafe extern "system" fn(this: *mut *mut Self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    KeyDown: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub KeyUp: unsafe extern "system" fn(this: *mut *mut Self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    KeyUp: usize,
    pub GetPreservedKey: unsafe extern "system" fn(this: *mut *mut Self, pic: *mut ::core::ffi::c_void, pprekey: *const TF_PRESERVEDKEY, pguid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPreservedKey: unsafe extern "system" fn(this: *mut *mut Self, rguid: *const ::windows_sys::core::GUID, pprekey: *const TF_PRESERVEDKEY, pfregistered: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPreservedKey: usize,
    pub PreserveKey: unsafe extern "system" fn(this: *mut *mut Self, tid: u32, rguid: *const ::windows_sys::core::GUID, prekey: *const TF_PRESERVEDKEY, pchdesc: ::windows_sys::core::PCWSTR, cchdesc: u32) -> ::windows_sys::core::HRESULT,
    pub UnpreserveKey: unsafe extern "system" fn(this: *mut *mut Self, rguid: *const ::windows_sys::core::GUID, pprekey: *const TF_PRESERVEDKEY) -> ::windows_sys::core::HRESULT,
    pub SetPreservedKeyDescription: unsafe extern "system" fn(this: *mut *mut Self, rguid: *const ::windows_sys::core::GUID, pchdesc: ::windows_sys::core::PCWSTR, cchdesc: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPreservedKeyDescription: unsafe extern "system" fn(this: *mut *mut Self, rguid: *const ::windows_sys::core::GUID, pbstrdesc: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPreservedKeyDescription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SimulatePreservedKey: unsafe extern "system" fn(this: *mut *mut Self, pic: *mut ::core::ffi::c_void, rguid: *const ::windows_sys::core::GUID, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SimulatePreservedKey: usize,
}
impl ::windows_sys::core::Interface for ITfKeystrokeMgr {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2860574704, data2: 8225, data3: 4562, data4: [147, 224, 0, 96, 176, 103, 184, 110] };
}
#[repr(C)]
pub struct ITfLMLattice {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryType: unsafe extern "system" fn(this: *mut *mut Self, rguidtype: *const ::windows_sys::core::GUID, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryType: usize,
    pub EnumLatticeElements: unsafe extern "system" fn(this: *mut *mut Self, dwframestart: u32, rguidtype: *const ::windows_sys::core::GUID, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfLMLattice {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3559089781, data2: 42431, data3: 17776, data4: [157, 66, 93, 109, 123, 2, 213, 155] };
}
#[repr(C)]
pub struct ITfLangBarEventSink {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnSetFocus: unsafe extern "system" fn(this: *mut *mut Self, dwthreadid: u32) -> ::windows_sys::core::HRESULT,
    pub OnThreadTerminate: unsafe extern "system" fn(this: *mut *mut Self, dwthreadid: u32) -> ::windows_sys::core::HRESULT,
    pub OnThreadItemChange: unsafe extern "system" fn(this: *mut *mut Self, dwthreadid: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnModalInput: unsafe extern "system" fn(this: *mut *mut Self, dwthreadid: u32, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnModalInput: usize,
    pub ShowFloating: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetItemFloatingRect: unsafe extern "system" fn(this: *mut *mut Self, dwthreadid: u32, rguid: *const ::windows_sys::core::GUID, prc: *mut super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetItemFloatingRect: usize,
}
impl ::windows_sys::core::Interface for ITfLangBarEventSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 413460736, data2: 57518, data3: 4562, data4: [175, 221, 0, 16, 90, 39, 153, 181] };
}
#[repr(C)]
pub struct ITfLangBarItem {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetInfo: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *mut TF_LANGBARITEMINFO) -> ::windows_sys::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, pdwstatus: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Show: unsafe extern "system" fn(this: *mut *mut Self, fshow: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Show: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTooltipString: unsafe extern "system" fn(this: *mut *mut Self, pbstrtooltip: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTooltipString: usize,
}
impl ::windows_sys::core::Interface for ITfLangBarItem {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1934888297, data2: 60907, data3: 20201, data4: [150, 201, 35, 170, 48, 178, 89, 22] };
}
#[repr(C)]
pub struct ITfLangBarItemBalloon {
    pub base__: ITfLangBarItem,
    #[cfg(feature = "Win32_Foundation")]
    pub OnClick: unsafe extern "system" fn(this: *mut *mut Self, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnClick: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPreferredSize: unsafe extern "system" fn(this: *mut *mut Self, pszdefault: *const super::super::Foundation::SIZE, psz: *mut super::super::Foundation::SIZE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPreferredSize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBalloonInfo: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *mut TF_LBBALLOONINFO) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBalloonInfo: usize,
}
impl ::windows_sys::core::Interface for ITfLangBarItemBalloon {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 29545093, data2: 54215, data3: 19323, data4: [181, 181, 217, 116, 17, 208, 194, 131] };
}
#[repr(C)]
pub struct ITfLangBarItemBitmap {
    pub base__: ITfLangBarItem,
    #[cfg(feature = "Win32_Foundation")]
    pub OnClick: unsafe extern "system" fn(this: *mut *mut Self, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnClick: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPreferredSize: unsafe extern "system" fn(this: *mut *mut Self, pszdefault: *const super::super::Foundation::SIZE, psz: *mut super::super::Foundation::SIZE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPreferredSize: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub DrawBitmap: unsafe extern "system" fn(this: *mut *mut Self, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    DrawBitmap: usize,
}
impl ::windows_sys::core::Interface for ITfLangBarItemBitmap {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1937965906, data2: 55074, data3: 16761, data4: [173, 165, 240, 69, 201, 141, 243, 85] };
}
#[repr(C)]
pub struct ITfLangBarItemBitmapButton {
    pub base__: ITfLangBarItem,
    #[cfg(feature = "Win32_Foundation")]
    pub OnClick: unsafe extern "system" fn(this: *mut *mut Self, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnClick: usize,
    pub InitMenu: unsafe extern "system" fn(this: *mut *mut Self, pmenu: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnMenuSelect: unsafe extern "system" fn(this: *mut *mut Self, wid: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPreferredSize: unsafe extern "system" fn(this: *mut *mut Self, pszdefault: *const super::super::Foundation::SIZE, psz: *mut super::super::Foundation::SIZE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPreferredSize: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub DrawBitmap: unsafe extern "system" fn(this: *mut *mut Self, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    DrawBitmap: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetText: unsafe extern "system" fn(this: *mut *mut Self, pbstrtext: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetText: usize,
}
impl ::windows_sys::core::Interface for ITfLangBarItemBitmapButton {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2724857125, data2: 16302, data3: 20384, data4: [137, 238, 136, 169, 100, 249, 241, 181] };
}
#[repr(C)]
pub struct ITfLangBarItemButton {
    pub base__: ITfLangBarItem,
    #[cfg(feature = "Win32_Foundation")]
    pub OnClick: unsafe extern "system" fn(this: *mut *mut Self, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnClick: usize,
    pub InitMenu: unsafe extern "system" fn(this: *mut *mut Self, pmenu: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnMenuSelect: unsafe extern "system" fn(this: *mut *mut Self, wid: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub GetIcon: unsafe extern "system" fn(this: *mut *mut Self, phicon: *mut super::WindowsAndMessaging::HICON) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    GetIcon: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetText: unsafe extern "system" fn(this: *mut *mut Self, pbstrtext: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetText: usize,
}
impl ::windows_sys::core::Interface for ITfLangBarItemButton {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 684192208, data2: 56869, data3: 4562, data4: [175, 221, 0, 16, 90, 39, 153, 181] };
}
#[repr(C)]
pub struct ITfLangBarItemMgr {
    pub base__: ::windows_sys::core::IUnknown,
    pub EnumItems: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetItem: unsafe extern "system" fn(this: *mut *mut Self, rguid: *const ::windows_sys::core::GUID, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AddItem: unsafe extern "system" fn(this: *mut *mut Self, punk: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveItem: unsafe extern "system" fn(this: *mut *mut Self, punk: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AdviseItemSink: unsafe extern "system" fn(this: *mut *mut Self, punk: *mut ::core::ffi::c_void, pdwcookie: *mut u32, rguiditem: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub UnadviseItemSink: unsafe extern "system" fn(this: *mut *mut Self, dwcookie: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetItemFloatingRect: unsafe extern "system" fn(this: *mut *mut Self, dwthreadid: u32, rguid: *const ::windows_sys::core::GUID, prc: *mut super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetItemFloatingRect: usize,
    pub GetItemsStatus: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32, prgguid: *const ::windows_sys::core::GUID, pdwstatus: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetItemNum: unsafe extern "system" fn(this: *mut *mut Self, pulcount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetItems: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32, ppitem: *mut *mut ::core::ffi::c_void, pinfo: *mut TF_LANGBARITEMINFO, pdwstatus: *mut u32, pcfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub AdviseItemsSink: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32, ppunk: *const *mut ::core::ffi::c_void, pguiditem: *const ::windows_sys::core::GUID, pdwcookie: *mut u32) -> ::windows_sys::core::HRESULT,
    pub UnadviseItemsSink: unsafe extern "system" fn(this: *mut *mut Self, ulcount: u32, pdwcookie: *const u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfLangBarItemMgr {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3125185621, data2: 39254, data3: 20401, data4: [165, 157, 82, 167, 221, 124, 198, 170] };
}
#[repr(C)]
pub struct ITfLangBarItemSink {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnUpdate: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfLangBarItemSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1474027936, data2: 56869, data3: 4562, data4: [175, 221, 0, 16, 90, 39, 153, 181] };
}
#[repr(C)]
pub struct ITfLangBarMgr {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub AdviseEventSink: unsafe extern "system" fn(this: *mut *mut Self, psink: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, dwflags: u32, pdwcookie: *const u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AdviseEventSink: usize,
    pub UnadviseEventSink: unsafe extern "system" fn(this: *mut *mut Self, dwcookie: u32) -> ::windows_sys::core::HRESULT,
    pub GetThreadMarshalInterface: unsafe extern "system" fn(this: *mut *mut Self, dwthreadid: u32, dwtype: u32, riid: *const ::windows_sys::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetThreadLangBarItemMgr: unsafe extern "system" fn(this: *mut *mut Self, dwthreadid: u32, pplbi: *mut *mut ::core::ffi::c_void, pdwthreadid: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetInputProcessorProfiles: unsafe extern "system" fn(this: *mut *mut Self, dwthreadid: u32, ppaip: *mut *mut ::core::ffi::c_void, pdwthreadid: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RestoreLastFocus: unsafe extern "system" fn(this: *mut *mut Self, pdwthreadid: *mut u32, fprev: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RestoreLastFocus: usize,
    pub SetModalInput: unsafe extern "system" fn(this: *mut *mut Self, psink: *mut ::core::ffi::c_void, dwthreadid: u32, dwflags: u32) -> ::windows_sys::core::HRESULT,
    pub ShowFloating: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32) -> ::windows_sys::core::HRESULT,
    pub GetShowFloatingStatus: unsafe extern "system" fn(this: *mut *mut Self, pdwflags: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfLangBarMgr {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2274711184, data2: 58919, data3: 4562, data4: [141, 219, 0, 16, 90, 39, 153, 181] };
}
#[repr(C)]
pub struct ITfLanguageProfileNotifySink {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub OnLanguageChange: unsafe extern "system" fn(this: *mut *mut Self, langid: u16, pfaccept: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnLanguageChange: usize,
    pub OnLanguageChanged: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfLanguageProfileNotifySink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1137311253, data2: 62612, data3: 19479, data4: [157, 226, 184, 164, 172, 53, 10, 168] };
}
#[repr(C)]
pub struct ITfMSAAControl {
    pub base__: ::windows_sys::core::IUnknown,
    pub SystemEnableMSAA: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SystemDisableMSAA: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfMSAAControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3052993339, data2: 14655, data3: 20348, data4: [132, 203, 80, 73, 36, 194, 112, 90] };
}
#[repr(C)]
pub struct ITfMenu {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub AddMenuItem: unsafe extern "system" fn(this: *mut *mut Self, uid: u32, dwflags: u32, hbmp: super::super::Graphics::Gdi::HBITMAP, hbmpmask: super::super::Graphics::Gdi::HBITMAP, pch: ::windows_sys::core::PCWSTR, cch: u32, ppmenu: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    AddMenuItem: usize,
}
impl ::windows_sys::core::Interface for ITfMenu {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1871354084, data2: 43680, data3: 20245, data4: [140, 91, 7, 224, 223, 10, 61, 216] };
}
#[repr(C)]
pub struct ITfMessagePump {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub PeekMessageA: unsafe extern "system" fn(this: *mut *mut Self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    PeekMessageA: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub GetMessageA: unsafe extern "system" fn(this: *mut *mut Self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    GetMessageA: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub PeekMessageW: unsafe extern "system" fn(this: *mut *mut Self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    PeekMessageW: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub GetMessageW: unsafe extern "system" fn(this: *mut *mut Self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    GetMessageW: usize,
}
impl ::windows_sys::core::Interface for ITfMessagePump {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2400946904, data2: 2923, data3: 18548, data4: [144, 197, 189, 118, 1, 30, 143, 124] };
}
#[repr(C)]
pub struct ITfMouseSink {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub OnMouseEvent: unsafe extern "system" fn(this: *mut *mut Self, uedge: u32, uquadrant: u32, dwbtnstatus: u32, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnMouseEvent: usize,
}
impl ::windows_sys::core::Interface for ITfMouseSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2712513186, data2: 14884, data3: 17565, data4: [172, 150, 81, 131, 231, 245, 194, 23] };
}
#[repr(C)]
pub struct ITfMouseTracker {
    pub base__: ::windows_sys::core::IUnknown,
    pub AdviseMouseSink: unsafe extern "system" fn(this: *mut *mut Self, range: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows_sys::core::HRESULT,
    pub UnadviseMouseSink: unsafe extern "system" fn(this: *mut *mut Self, dwcookie: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfMouseTracker {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 164710093, data2: 42308, data3: 16690, data4: [146, 91, 122, 250, 142, 243, 34, 208] };
}
#[repr(C)]
pub struct ITfMouseTrackerACP {
    pub base__: ::windows_sys::core::IUnknown,
    pub AdviseMouseSink: unsafe extern "system" fn(this: *mut *mut Self, range: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows_sys::core::HRESULT,
    pub UnadviseMouseSink: unsafe extern "system" fn(this: *mut *mut Self, dwcookie: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfMouseTrackerACP {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1004370146, data2: 49518, data3: 18429, data4: [184, 131, 206, 111, 172, 193, 162, 8] };
}
#[repr(C)]
pub struct ITfPersistentPropertyLoaderACP {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub LoadProperty: unsafe extern "system" fn(this: *mut *mut Self, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LoadProperty: usize,
}
impl ::windows_sys::core::Interface for ITfPersistentPropertyLoaderACP {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1324912976, data2: 2055, data3: 4563, data4: [141, 240, 0, 16, 90, 39, 153, 181] };
}
#[repr(C)]
pub struct ITfPreservedKeyNotifySink {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnUpdated: unsafe extern "system" fn(this: *mut *mut Self, pprekey: *const TF_PRESERVEDKEY) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfPreservedKeyNotifySink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1870121363, data2: 53937, data3: 17518, data4: [133, 62, 89, 18, 239, 200, 162, 134] };
}
#[repr(C)]
pub struct ITfProperty {
    pub base__: ITfReadOnlyProperty,
    pub FindRange: unsafe extern "system" fn(this: *mut *mut Self, ec: u32, prange: *mut ::core::ffi::c_void, pprange: *mut *mut ::core::ffi::c_void, apos: TfAnchor) -> ::windows_sys::core::HRESULT,
    pub SetValueStore: unsafe extern "system" fn(this: *mut *mut Self, ec: u32, prange: *mut ::core::ffi::c_void, ppropstore: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, ec: u32, prange: *mut ::core::ffi::c_void, pvarvalue: *const super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetValue: usize,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self, ec: u32, prange: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfProperty {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3796145760, data2: 38210, data3: 4562, data4: [191, 70, 0, 16, 90, 39, 153, 181] };
}
#[repr(C)]
pub struct ITfPropertyStore {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetType: unsafe extern "system" fn(this: *mut *mut Self, pguid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetDataType: unsafe extern "system" fn(this: *mut *mut Self, pdwreserved: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetData: unsafe extern "system" fn(this: *mut *mut Self, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetData: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnTextUpdated: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, prangenew: *mut ::core::ffi::c_void, pfaccept: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnTextUpdated: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Shrink: unsafe extern "system" fn(this: *mut *mut Self, prangenew: *mut ::core::ffi::c_void, pffree: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Shrink: usize,
    pub Divide: unsafe extern "system" fn(this: *mut *mut Self, prangethis: *mut ::core::ffi::c_void, prangenew: *mut ::core::ffi::c_void, pppropstore: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppropstore: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetPropertyRangeCreator: unsafe extern "system" fn(this: *mut *mut Self, pclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Serialize: unsafe extern "system" fn(this: *mut *mut Self, pstream: *mut ::core::ffi::c_void, pcb: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Serialize: usize,
}
impl ::windows_sys::core::Interface for ITfPropertyStore {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1748283680, data2: 35019, data3: 4562, data4: [191, 69, 0, 16, 90, 39, 153, 181] };
}
#[repr(C)]
pub struct ITfQueryEmbedded {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub QueryInsertEmbedded: unsafe extern "system" fn(this: *mut *mut Self, pguidservice: *const ::windows_sys::core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    QueryInsertEmbedded: usize,
}
impl ::windows_sys::core::Interface for ITfQueryEmbedded {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 262904795, data2: 53840, data3: 16745, data4: [132, 229, 107, 225, 24, 253, 215, 168] };
}
#[repr(C)]
pub struct ITfRange {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetText: unsafe extern "system" fn(this: *mut *mut Self, ec: u32, dwflags: u32, pchtext: ::windows_sys::core::PWSTR, cchmax: u32, pcch: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut *mut Self, ec: u32, dwflags: u32, pchtext: ::windows_sys::core::PCWSTR, cch: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFormattedText: unsafe extern "system" fn(this: *mut *mut Self, ec: u32, ppdataobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFormattedText: usize,
    pub GetEmbedded: unsafe extern "system" fn(this: *mut *mut Self, ec: u32, rguidservice: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InsertEmbedded: unsafe extern "system" fn(this: *mut *mut Self, ec: u32, dwflags: u32, pdataobject: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InsertEmbedded: usize,
    pub ShiftStart: unsafe extern "system" fn(this: *mut *mut Self, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> ::windows_sys::core::HRESULT,
    pub ShiftEnd: unsafe extern "system" fn(this: *mut *mut Self, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> ::windows_sys::core::HRESULT,
    pub ShiftStartToRange: unsafe extern "system" fn(this: *mut *mut Self, ec: u32, prange: *mut ::core::ffi::c_void, apos: TfAnchor) -> ::windows_sys::core::HRESULT,
    pub ShiftEndToRange: unsafe extern "system" fn(this: *mut *mut Self, ec: u32, prange: *mut ::core::ffi::c_void, apos: TfAnchor) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ShiftStartRegion: unsafe extern "system" fn(this: *mut *mut Self, ec: u32, dir: TfShiftDir, pfnoregion: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShiftStartRegion: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ShiftEndRegion: unsafe extern "system" fn(this: *mut *mut Self, ec: u32, dir: TfShiftDir, pfnoregion: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShiftEndRegion: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsEmpty: unsafe extern "system" fn(this: *mut *mut Self, ec: u32, pfempty: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsEmpty: usize,
    pub Collapse: unsafe extern "system" fn(this: *mut *mut Self, ec: u32, apos: TfAnchor) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsEqualStart: unsafe extern "system" fn(this: *mut *mut Self, ec: u32, pwith: *mut ::core::ffi::c_void, apos: TfAnchor, pfequal: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsEqualStart: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsEqualEnd: unsafe extern "system" fn(this: *mut *mut Self, ec: u32, pwith: *mut ::core::ffi::c_void, apos: TfAnchor, pfequal: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsEqualEnd: usize,
    pub CompareStart: unsafe extern "system" fn(this: *mut *mut Self, ec: u32, pwith: *mut ::core::ffi::c_void, apos: TfAnchor, plresult: *mut i32) -> ::windows_sys::core::HRESULT,
    pub CompareEnd: unsafe extern "system" fn(this: *mut *mut Self, ec: u32, pwith: *mut ::core::ffi::c_void, apos: TfAnchor, plresult: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AdjustForInsert: unsafe extern "system" fn(this: *mut *mut Self, ec: u32, cchinsert: u32, pfinsertok: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AdjustForInsert: usize,
    pub GetGravity: unsafe extern "system" fn(this: *mut *mut Self, pgstart: *mut TfGravity, pgend: *mut TfGravity) -> ::windows_sys::core::HRESULT,
    pub SetGravity: unsafe extern "system" fn(this: *mut *mut Self, ec: u32, gstart: TfGravity, gend: TfGravity) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppclone: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetContext: unsafe extern "system" fn(this: *mut *mut Self, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfRange {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2860574719, data2: 8225, data3: 4562, data4: [147, 224, 0, 96, 176, 103, 184, 110] };
}
#[repr(C)]
pub struct ITfRangeACP {
    pub base__: ITfRange,
    pub GetExtent: unsafe extern "system" fn(this: *mut *mut Self, pacpanchor: *mut i32, pcch: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetExtent: unsafe extern "system" fn(this: *mut *mut Self, acpanchor: i32, cch: i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfRangeACP {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 91906710, data2: 667, data3: 16724, data4: [183, 154, 13, 70, 29, 78, 169, 76] };
}
#[repr(C)]
pub struct ITfRangeBackup {
    pub base__: ::windows_sys::core::IUnknown,
    pub Restore: unsafe extern "system" fn(this: *mut *mut Self, ec: u32, prange: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfRangeBackup {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1178226797, data2: 27026, data3: 18898, data4: [155, 136, 147, 213, 94, 112, 187, 22] };
}
#[repr(C)]
pub struct ITfReadOnlyProperty {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetType: unsafe extern "system" fn(this: *mut *mut Self, pguid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub EnumRanges: unsafe extern "system" fn(this: *mut *mut Self, ec: u32, ppenum: *mut *mut ::core::ffi::c_void, ptargetrange: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetValue: unsafe extern "system" fn(this: *mut *mut Self, ec: u32, prange: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetValue: usize,
    pub GetContext: unsafe extern "system" fn(this: *mut *mut Self, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfReadOnlyProperty {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 399809085, data2: 63672, data3: 19247, data4: [178, 84, 82, 49, 157, 214, 76, 83] };
}
#[repr(C)]
pub struct ITfReadingInformationUIElement {
    pub base__: ITfUIElement,
    pub GetUpdatedFlags: unsafe extern "system" fn(this: *mut *mut Self, pdwflags: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetContext: unsafe extern "system" fn(this: *mut *mut Self, ppic: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetString: unsafe extern "system" fn(this: *mut *mut Self, pstr: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetString: usize,
    pub GetMaxReadingStringLength: unsafe extern "system" fn(this: *mut *mut Self, pcchmax: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetErrorIndex: unsafe extern "system" fn(this: *mut *mut Self, perrorindex: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsVerticalOrderPreferred: unsafe extern "system" fn(this: *mut *mut Self, pfvertical: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsVerticalOrderPreferred: usize,
}
impl ::windows_sys::core::Interface for ITfReadingInformationUIElement {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3927875897, data2: 6623, data3: 4567, data4: [166, 210, 0, 6, 91, 132, 67, 92] };
}
#[repr(C)]
pub struct ITfReverseConversion {
    pub base__: ::windows_sys::core::IUnknown,
    pub DoReverseConversion: unsafe extern "system" fn(this: *mut *mut Self, lpstr: ::windows_sys::core::PCWSTR, pplist: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfReverseConversion {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2752897378, data2: 5501, data3: 16765, data4: [138, 140, 10, 178, 108, 125, 39, 129] };
}
#[repr(C)]
pub struct ITfReverseConversionList {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetLength: unsafe extern "system" fn(this: *mut *mut Self, puindex: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetString: unsafe extern "system" fn(this: *mut *mut Self, uindex: u32, pbstr: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetString: usize,
}
impl ::windows_sys::core::Interface for ITfReverseConversionList {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 354249200, data2: 34548, data3: 18036, data4: [183, 33, 86, 145, 30, 121, 127, 71] };
}
#[repr(C)]
pub struct ITfReverseConversionMgr {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetReverseConversion: unsafe extern "system" fn(this: *mut *mut Self, langid: u16, guidprofile: *const ::windows_sys::core::GUID, dwflag: u32, ppreverseconversion: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfReverseConversionMgr {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3057893942, data2: 50323, data3: 16822, data4: [171, 179, 105, 36, 18, 119, 92, 196] };
}
#[repr(C)]
pub struct ITfSource {
    pub base__: ::windows_sys::core::IUnknown,
    pub AdviseSink: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, punk: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows_sys::core::HRESULT,
    pub UnadviseSink: unsafe extern "system" fn(this: *mut *mut Self, dwcookie: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1319406133, data2: 24750, data3: 17519, data4: [143, 214, 230, 168, 216, 36, 89, 247] };
}
#[repr(C)]
pub struct ITfSourceSingle {
    pub base__: ::windows_sys::core::IUnknown,
    pub AdviseSingleSink: unsafe extern "system" fn(this: *mut *mut Self, tid: u32, riid: *const ::windows_sys::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UnadviseSingleSink: unsafe extern "system" fn(this: *mut *mut Self, tid: u32, riid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfSourceSingle {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1930633116, data2: 22185, data3: 18909, data4: [176, 238, 208, 70, 99, 63, 117, 40] };
}
#[repr(C)]
pub struct ITfSpeechUIServer {
    pub base__: ::windows_sys::core::IUnknown,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowUI: unsafe extern "system" fn(this: *mut *mut Self, fshow: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowUI: usize,
    pub UpdateBalloon: unsafe extern "system" fn(this: *mut *mut Self, style: TfLBBalloonStyle, pch: ::windows_sys::core::PCWSTR, cch: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfSpeechUIServer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2431232324, data2: 37444, data3: 18591, data4: [167, 143, 222, 103, 175, 192, 19, 167] };
}
#[repr(C)]
pub struct ITfStatusSink {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnStatusChange: unsafe extern "system" fn(this: *mut *mut Self, pic: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfStatusSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1803390323, data2: 45671, data3: 20329, data4: [179, 46, 28, 163, 33, 206, 79, 69] };
}
#[repr(C)]
pub struct ITfSystemDeviceTypeLangBarItem {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetIconMode: unsafe extern "system" fn(this: *mut *mut Self, dwflags: LANG_BAR_ITEM_ICON_MODE_FLAGS) -> ::windows_sys::core::HRESULT,
    pub GetIconMode: unsafe extern "system" fn(this: *mut *mut Self, pdwflags: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfSystemDeviceTypeLangBarItem {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1164390073, data2: 36953, data3: 18082, data4: [131, 141, 69, 48, 53, 95, 106, 119] };
}
#[repr(C)]
pub struct ITfSystemLangBarItem {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub SetIcon: unsafe extern "system" fn(this: *mut *mut Self, hicon: super::WindowsAndMessaging::HICON) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    SetIcon: usize,
    pub SetTooltipString: unsafe extern "system" fn(this: *mut *mut Self, pchtooltip: ::windows_sys::core::PCWSTR, cch: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfSystemLangBarItem {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 504621548, data2: 27443, data3: 19786, data4: [181, 235, 138, 146, 240, 41, 243, 86] };
}
#[repr(C)]
pub struct ITfSystemLangBarItemSink {
    pub base__: ::windows_sys::core::IUnknown,
    pub InitMenu: unsafe extern "system" fn(this: *mut *mut Self, pmenu: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnMenuSelect: unsafe extern "system" fn(this: *mut *mut Self, wid: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfSystemLangBarItemSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 340384171, data2: 5071, data3: 18055, data4: [170, 62, 141, 139, 24, 87, 67, 150] };
}
#[repr(C)]
pub struct ITfSystemLangBarItemText {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetItemText: unsafe extern "system" fn(this: *mut *mut Self, pch: ::windows_sys::core::PCWSTR, cch: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetItemText: unsafe extern "system" fn(this: *mut *mut Self, pbstrtext: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetItemText: usize,
}
impl ::windows_sys::core::Interface for ITfSystemLangBarItemText {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1548542181, data2: 47689, data3: 19282, data4: [172, 107, 59, 57, 123, 79, 112, 31] };
}
#[repr(C)]
pub struct ITfTextEditSink {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnEndEdit: unsafe extern "system" fn(this: *mut *mut Self, pic: *mut ::core::ffi::c_void, ecreadonly: u32, peditrecord: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfTextEditSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2166871049, data2: 52435, data3: 18051, data4: [150, 122, 180, 61, 91, 72, 43, 247] };
}
#[repr(C)]
pub struct ITfTextInputProcessor {
    pub base__: ::windows_sys::core::IUnknown,
    pub Activate: unsafe extern "system" fn(this: *mut *mut Self, ptim: *mut ::core::ffi::c_void, tid: u32) -> ::windows_sys::core::HRESULT,
    pub Deactivate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfTextInputProcessor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2860574711, data2: 8225, data3: 4562, data4: [147, 224, 0, 96, 176, 103, 184, 110] };
}
#[repr(C)]
pub struct ITfTextInputProcessorEx {
    pub base__: ITfTextInputProcessor,
    pub ActivateEx: unsafe extern "system" fn(this: *mut *mut Self, ptim: *mut ::core::ffi::c_void, tid: u32, dwflags: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfTextInputProcessorEx {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1850614018, data2: 63949, data3: 17213, data4: [180, 150, 48, 60, 224, 58, 101, 7] };
}
#[repr(C)]
pub struct ITfTextLayoutSink {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnLayoutChange: unsafe extern "system" fn(this: *mut *mut Self, pic: *mut ::core::ffi::c_void, lcode: TfLayoutCode, pview: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfTextLayoutSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 720556138, data2: 56667, data3: 18727, data4: [160, 180, 84, 241, 156, 145, 250, 222] };
}
#[repr(C)]
pub struct ITfThreadFocusSink {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnSetThreadFocus: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub OnKillThreadFocus: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfThreadFocusSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3237075724, data2: 14880, data3: 16476, data4: [163, 3, 150, 182, 1, 10, 136, 95] };
}
#[repr(C)]
pub struct ITfThreadMgr {
    pub base__: ::windows_sys::core::IUnknown,
    pub Activate: unsafe extern "system" fn(this: *mut *mut Self, ptid: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Deactivate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub CreateDocumentMgr: unsafe extern "system" fn(this: *mut *mut Self, ppdim: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EnumDocumentMgrs: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFocus: unsafe extern "system" fn(this: *mut *mut Self, ppdimfocus: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetFocus: unsafe extern "system" fn(this: *mut *mut Self, pdimfocus: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AssociateFocus: unsafe extern "system" fn(this: *mut *mut Self, hwnd: super::super::Foundation::HWND, pdimnew: *mut ::core::ffi::c_void, ppdimprev: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AssociateFocus: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsThreadFocus: unsafe extern "system" fn(this: *mut *mut Self, pfthreadfocus: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsThreadFocus: usize,
    pub GetFunctionProvider: unsafe extern "system" fn(this: *mut *mut Self, clsid: *const ::windows_sys::core::GUID, ppfuncprov: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EnumFunctionProviders: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetGlobalCompartment: unsafe extern "system" fn(this: *mut *mut Self, ppcompmgr: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfThreadMgr {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2860574721, data2: 8225, data3: 4562, data4: [147, 224, 0, 96, 176, 103, 184, 110] };
}
#[repr(C)]
pub struct ITfThreadMgr2 {
    pub base__: ::windows_sys::core::IUnknown,
    pub Activate: unsafe extern "system" fn(this: *mut *mut Self, ptid: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Deactivate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub CreateDocumentMgr: unsafe extern "system" fn(this: *mut *mut Self, ppdim: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EnumDocumentMgrs: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFocus: unsafe extern "system" fn(this: *mut *mut Self, ppdimfocus: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetFocus: unsafe extern "system" fn(this: *mut *mut Self, pdimfocus: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsThreadFocus: unsafe extern "system" fn(this: *mut *mut Self, pfthreadfocus: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsThreadFocus: usize,
    pub GetFunctionProvider: unsafe extern "system" fn(this: *mut *mut Self, clsid: *const ::windows_sys::core::GUID, ppfuncprov: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EnumFunctionProviders: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetGlobalCompartment: unsafe extern "system" fn(this: *mut *mut Self, ppcompmgr: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ActivateEx: unsafe extern "system" fn(this: *mut *mut Self, ptid: *mut u32, dwflags: u32) -> ::windows_sys::core::HRESULT,
    pub GetActiveFlags: unsafe extern "system" fn(this: *mut *mut Self, lpdwflags: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SuspendKeystrokeHandling: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ResumeKeystrokeHandling: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfThreadMgr2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 179411183, data2: 25719, data3: 20200, data4: [136, 18, 103, 128, 237, 184, 45, 94] };
}
#[repr(C)]
pub struct ITfThreadMgrEventSink {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnInitDocumentMgr: unsafe extern "system" fn(this: *mut *mut Self, pdim: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnUninitDocumentMgr: unsafe extern "system" fn(this: *mut *mut Self, pdim: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnSetFocus: unsafe extern "system" fn(this: *mut *mut Self, pdimfocus: *mut ::core::ffi::c_void, pdimprevfocus: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnPushContext: unsafe extern "system" fn(this: *mut *mut Self, pic: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnPopContext: unsafe extern "system" fn(this: *mut *mut Self, pic: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfThreadMgrEventSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2860574734, data2: 8225, data3: 4562, data4: [147, 224, 0, 96, 176, 103, 184, 110] };
}
#[repr(C)]
pub struct ITfThreadMgrEx {
    pub base__: ITfThreadMgr,
    pub ActivateEx: unsafe extern "system" fn(this: *mut *mut Self, ptid: *mut u32, dwflags: u32) -> ::windows_sys::core::HRESULT,
    pub GetActiveFlags: unsafe extern "system" fn(this: *mut *mut Self, lpdwflags: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfThreadMgrEx {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1049669091, data2: 30100, data3: 19632, data4: [187, 88, 105, 98, 143, 95, 69, 140] };
}
#[repr(C)]
pub struct ITfToolTipUIElement {
    pub base__: ITfUIElement,
    #[cfg(feature = "Win32_Foundation")]
    pub GetString: unsafe extern "system" fn(this: *mut *mut Self, pstr: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetString: usize,
}
impl ::windows_sys::core::Interface for ITfToolTipUIElement {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1387367260, data2: 21853, data3: 18098, data4: [176, 10, 250, 104, 1, 68, 251, 219] };
}
#[repr(C)]
pub struct ITfTransitoryExtensionSink {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub OnTransitoryExtensionUpdated: unsafe extern "system" fn(this: *mut *mut Self, pic: *mut ::core::ffi::c_void, ecreadonly: u32, presultrange: *mut ::core::ffi::c_void, pcompositionrange: *mut ::core::ffi::c_void, pfdeleteresultrange: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnTransitoryExtensionUpdated: usize,
}
impl ::windows_sys::core::Interface for ITfTransitoryExtensionSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2786396527, data2: 7255, data3: 18451, data4: [138, 21, 85, 238, 110, 90, 131, 156] };
}
#[repr(C)]
pub struct ITfTransitoryExtensionUIElement {
    pub base__: ITfUIElement,
    pub GetDocumentMgr: unsafe extern "system" fn(this: *mut *mut Self, ppdim: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfTransitoryExtensionUIElement {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2240779626, data2: 38703, data3: 17058, data4: [162, 242, 3, 33, 225, 171, 226, 9] };
}
#[repr(C)]
pub struct ITfUIElement {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDescription: unsafe extern "system" fn(this: *mut *mut Self, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDescription: usize,
    pub GetGUID: unsafe extern "system" fn(this: *mut *mut Self, pguid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Show: unsafe extern "system" fn(this: *mut *mut Self, bshow: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Show: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsShown: unsafe extern "system" fn(this: *mut *mut Self, pbshow: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsShown: usize,
}
impl ::windows_sys::core::Interface for ITfUIElement {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3927875895, data2: 6623, data3: 4567, data4: [166, 210, 0, 6, 91, 132, 67, 92] };
}
#[repr(C)]
pub struct ITfUIElementMgr {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub BeginUIElement: unsafe extern "system" fn(this: *mut *mut Self, pelement: *mut ::core::ffi::c_void, pbshow: *mut super::super::Foundation::BOOL, pdwuielementid: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BeginUIElement: usize,
    pub UpdateUIElement: unsafe extern "system" fn(this: *mut *mut Self, dwuielementid: u32) -> ::windows_sys::core::HRESULT,
    pub EndUIElement: unsafe extern "system" fn(this: *mut *mut Self, dwuielementid: u32) -> ::windows_sys::core::HRESULT,
    pub GetUIElement: unsafe extern "system" fn(this: *mut *mut Self, dwuielementid: u32, ppelement: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EnumUIElements: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfUIElementMgr {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3927875893, data2: 6623, data3: 4567, data4: [166, 210, 0, 6, 91, 132, 67, 92] };
}
#[repr(C)]
pub struct ITfUIElementSink {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub BeginUIElement: unsafe extern "system" fn(this: *mut *mut Self, dwuielementid: u32, pbshow: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BeginUIElement: usize,
    pub UpdateUIElement: unsafe extern "system" fn(this: *mut *mut Self, dwuielementid: u32) -> ::windows_sys::core::HRESULT,
    pub EndUIElement: unsafe extern "system" fn(this: *mut *mut Self, dwuielementid: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITfUIElementSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3927875894, data2: 6623, data3: 4567, data4: [166, 210, 0, 6, 91, 132, 67, 92] };
}
#[repr(C)]
pub struct IUIManagerEventSink {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub OnWindowOpening: unsafe extern "system" fn(this: *mut *mut Self, prcbounds: *const super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnWindowOpening: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnWindowOpened: unsafe extern "system" fn(this: *mut *mut Self, prcbounds: *const super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnWindowOpened: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnWindowUpdating: unsafe extern "system" fn(this: *mut *mut Self, prcupdatedbounds: *const super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnWindowUpdating: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnWindowUpdated: unsafe extern "system" fn(this: *mut *mut Self, prcupdatedbounds: *const super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnWindowUpdated: usize,
    pub OnWindowClosing: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub OnWindowClosed: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUIManagerEventSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3448886928, data2: 42984, data3: 16997, data4: [155, 56, 139, 179, 187, 171, 167, 222] };
}
#[repr(C)]
pub struct IVersionInfo {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetSubcomponentCount: unsafe extern "system" fn(this: *mut *mut Self, ulsub: u32, ulcount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetImplementationID: unsafe extern "system" fn(this: *mut *mut Self, ulsub: u32, implid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetBuildVersion: unsafe extern "system" fn(this: *mut *mut Self, ulsub: u32, pdwmajor: *mut u32, pdwminor: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetComponentDescription: unsafe extern "system" fn(this: *mut *mut Self, ulsub: u32, pimplstr: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetComponentDescription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetInstanceDescription: unsafe extern "system" fn(this: *mut *mut Self, ulsub: u32, pimplstr: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetInstanceDescription: usize,
}
impl ::windows_sys::core::Interface for IVersionInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1075124460, data2: 56064, data3: 17937, data4: [155, 41, 42, 14, 75, 154, 250, 133] };
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub type InputScope = i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_DEFAULT: InputScope = 0i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_URL: InputScope = 1i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_FILE_FULLFILEPATH: InputScope = 2i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_FILE_FILENAME: InputScope = 3i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_EMAIL_USERNAME: InputScope = 4i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_EMAIL_SMTPEMAILADDRESS: InputScope = 5i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_LOGINNAME: InputScope = 6i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_PERSONALNAME_FULLNAME: InputScope = 7i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_PERSONALNAME_PREFIX: InputScope = 8i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_PERSONALNAME_GIVENNAME: InputScope = 9i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_PERSONALNAME_MIDDLENAME: InputScope = 10i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_PERSONALNAME_SURNAME: InputScope = 11i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_PERSONALNAME_SUFFIX: InputScope = 12i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_ADDRESS_FULLPOSTALADDRESS: InputScope = 13i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_ADDRESS_POSTALCODE: InputScope = 14i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_ADDRESS_STREET: InputScope = 15i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_ADDRESS_STATEORPROVINCE: InputScope = 16i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_ADDRESS_CITY: InputScope = 17i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_ADDRESS_COUNTRYNAME: InputScope = 18i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_ADDRESS_COUNTRYSHORTNAME: InputScope = 19i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_CURRENCY_AMOUNTANDSYMBOL: InputScope = 20i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_CURRENCY_AMOUNT: InputScope = 21i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_DATE_FULLDATE: InputScope = 22i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_DATE_MONTH: InputScope = 23i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_DATE_DAY: InputScope = 24i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_DATE_YEAR: InputScope = 25i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_DATE_MONTHNAME: InputScope = 26i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_DATE_DAYNAME: InputScope = 27i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_DIGITS: InputScope = 28i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_NUMBER: InputScope = 29i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_ONECHAR: InputScope = 30i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_PASSWORD: InputScope = 31i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_TELEPHONE_FULLTELEPHONENUMBER: InputScope = 32i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_TELEPHONE_COUNTRYCODE: InputScope = 33i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_TELEPHONE_AREACODE: InputScope = 34i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_TELEPHONE_LOCALNUMBER: InputScope = 35i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_TIME_FULLTIME: InputScope = 36i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_TIME_HOUR: InputScope = 37i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_TIME_MINORSEC: InputScope = 38i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_NUMBER_FULLWIDTH: InputScope = 39i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_ALPHANUMERIC_HALFWIDTH: InputScope = 40i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_ALPHANUMERIC_FULLWIDTH: InputScope = 41i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_CURRENCY_CHINESE: InputScope = 42i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_BOPOMOFO: InputScope = 43i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_HIRAGANA: InputScope = 44i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_KATAKANA_HALFWIDTH: InputScope = 45i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_KATAKANA_FULLWIDTH: InputScope = 46i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_HANJA: InputScope = 47i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_HANGUL_HALFWIDTH: InputScope = 48i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_HANGUL_FULLWIDTH: InputScope = 49i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_SEARCH: InputScope = 50i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_FORMULA: InputScope = 51i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_SEARCH_INCREMENTAL: InputScope = 52i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_CHINESE_HALFWIDTH: InputScope = 53i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_CHINESE_FULLWIDTH: InputScope = 54i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_NATIVE_SCRIPT: InputScope = 55i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_YOMI: InputScope = 56i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_TEXT: InputScope = 57i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_CHAT: InputScope = 58i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_NAME_OR_PHONENUMBER: InputScope = 59i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_EMAILNAME_OR_ADDRESS: InputScope = 60i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_PRIVATE: InputScope = 61i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_MAPS: InputScope = 62i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_NUMERIC_PASSWORD: InputScope = 63i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_NUMERIC_PIN: InputScope = 64i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_ALPHANUMERIC_PIN: InputScope = 65i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_ALPHANUMERIC_PIN_SET: InputScope = 66i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_FORMULA_NUMBER: InputScope = 67i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_CHAT_WITHOUT_EMOJI: InputScope = 68i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_PHRASELIST: InputScope = -1i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_REGULAREXPRESSION: InputScope = -2i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_SRGS: InputScope = -3i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_XML: InputScope = -4i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_ENUMSTRING: InputScope = -5i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub type LANG_BAR_ITEM_ICON_MODE_FLAGS = u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_DTLBI_NONE: LANG_BAR_ITEM_ICON_MODE_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_DTLBI_USEPROFILEICON: LANG_BAR_ITEM_ICON_MODE_FLAGS = 1u32;
pub const LIBID_MSAATEXTLib: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 353250682, data2: 56001, data3: 17794, data4: [148, 125, 42, 143, 215, 139, 130, 205] };
pub const MSAAControl: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 147691071, data2: 31294, data3: 20316, data4: [155, 216, 214, 146, 187, 4, 60, 91] };
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub type TEXT_STORE_CHANGE_FLAGS = u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_TC_NONE: TEXT_STORE_CHANGE_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_TC_CORRECTION: TEXT_STORE_CHANGE_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub type TEXT_STORE_LOCK_FLAGS = u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_LF_READ: TEXT_STORE_LOCK_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_LF_READWRITE: TEXT_STORE_LOCK_FLAGS = 6u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub type TEXT_STORE_TEXT_CHANGE_FLAGS = u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_ST_NONE: TEXT_STORE_TEXT_CHANGE_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_ST_CORRECTION: TEXT_STORE_TEXT_CHANGE_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CHAR_EMBEDDED: u32 = 65532u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CLUIE_COUNT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CLUIE_CURRENTPAGE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CLUIE_DOCUMENTMGR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CLUIE_PAGEINDEX: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CLUIE_SELECTION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CLUIE_STRING: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_COMMANDING_ENABLED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_COMMANDING_ON: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub type TF_CONTEXT_EDIT_CONTEXT_FLAGS = u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_ES_ASYNCDONTCARE: TF_CONTEXT_EDIT_CONTEXT_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_ES_SYNC: TF_CONTEXT_EDIT_CONTEXT_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_ES_READ: TF_CONTEXT_EDIT_CONTEXT_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_ES_READWRITE: TF_CONTEXT_EDIT_CONTEXT_FLAGS = 6u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_ES_ASYNC: TF_CONTEXT_EDIT_CONTEXT_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CONVERSIONMODE_ALPHANUMERIC: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CONVERSIONMODE_CHARCODE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CONVERSIONMODE_EUDC: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CONVERSIONMODE_FIXED: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CONVERSIONMODE_FULLSHAPE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CONVERSIONMODE_KATAKANA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CONVERSIONMODE_NATIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CONVERSIONMODE_NOCONVERSION: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CONVERSIONMODE_ROMAN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CONVERSIONMODE_SOFTKEYBOARD: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CONVERSIONMODE_SYMBOL: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub type TF_DA_ATTR_INFO = i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_ATTR_INPUT: TF_DA_ATTR_INFO = 0i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_ATTR_TARGET_CONVERTED: TF_DA_ATTR_INFO = 1i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_ATTR_CONVERTED: TF_DA_ATTR_INFO = 2i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_ATTR_TARGET_NOTCONVERTED: TF_DA_ATTR_INFO = 3i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_ATTR_INPUT_ERROR: TF_DA_ATTR_INFO = 4i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_ATTR_FIXEDCONVERTED: TF_DA_ATTR_INFO = 5i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_ATTR_OTHER: TF_DA_ATTR_INFO = -1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub struct TF_DA_COLOR {
    pub r#type: TF_DA_COLORTYPE,
    pub Anonymous: TF_DA_COLOR_0,
}
impl ::core::marker::Copy for TF_DA_COLOR {}
impl ::core::clone::Clone for TF_DA_COLOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub union TF_DA_COLOR_0 {
    pub nIndex: i32,
    pub cr: u32,
}
impl ::core::marker::Copy for TF_DA_COLOR_0 {}
impl ::core::clone::Clone for TF_DA_COLOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub type TF_DA_COLORTYPE = i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CT_NONE: TF_DA_COLORTYPE = 0i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CT_SYSCOLOR: TF_DA_COLORTYPE = 1i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CT_COLORREF: TF_DA_COLORTYPE = 2i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub type TF_DA_LINESTYLE = i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LS_NONE: TF_DA_LINESTYLE = 0i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LS_SOLID: TF_DA_LINESTYLE = 1i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LS_DOT: TF_DA_LINESTYLE = 2i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LS_DASH: TF_DA_LINESTYLE = 3i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LS_SQUIGGLE: TF_DA_LINESTYLE = 4i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_DICTATION_ENABLED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_DICTATION_ON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_DISABLE_BALLOON: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_DISABLE_COMMANDING: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_DISABLE_DICTATION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_DISABLE_SPEECH: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TF_DISPLAYATTRIBUTE {
    pub crText: TF_DA_COLOR,
    pub crBk: TF_DA_COLOR,
    pub lsStyle: TF_DA_LINESTYLE,
    pub fBoldLine: super::super::Foundation::BOOL,
    pub crLine: TF_DA_COLOR,
    pub bAttr: TF_DA_ATTR_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TF_DISPLAYATTRIBUTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TF_DISPLAYATTRIBUTE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_ENABLE_PROCESS_ATOM: &str = "_CTF_ENABLE_PROCESS_ATOM_";
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_ALREADY_EXISTS: ::windows_sys::core::HRESULT = -2147220218i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_COMPOSITION_REJECTED: ::windows_sys::core::HRESULT = -2147220216i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_DISCONNECTED: ::windows_sys::core::HRESULT = -2147220220i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_EMPTYCONTEXT: ::windows_sys::core::HRESULT = -2147220215i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_FORMAT: ::windows_sys::core::HRESULT = -2147220982i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_INVALIDPOINT: ::windows_sys::core::HRESULT = -2147220985i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_INVALIDPOS: ::windows_sys::core::HRESULT = -2147220992i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_INVALIDVIEW: ::windows_sys::core::HRESULT = -2147220219i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_LOCKED: ::windows_sys::core::HRESULT = -2147220224i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_NOCONVERSION: ::windows_sys::core::HRESULT = -2147219968i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_NOINTERFACE: ::windows_sys::core::HRESULT = -2147220988i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_NOLAYOUT: ::windows_sys::core::HRESULT = -2147220986i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_NOLOCK: ::windows_sys::core::HRESULT = -2147220991i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_NOOBJECT: ::windows_sys::core::HRESULT = -2147220990i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_NOPROVIDER: ::windows_sys::core::HRESULT = -2147220221i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_NOSELECTION: ::windows_sys::core::HRESULT = -2147220987i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_NOSERVICE: ::windows_sys::core::HRESULT = -2147220989i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_NOTOWNEDRANGE: ::windows_sys::core::HRESULT = -2147220222i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_RANGE_NOT_COVERED: ::windows_sys::core::HRESULT = -2147220217i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_READONLY: ::windows_sys::core::HRESULT = -2147220983i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_STACKFULL: ::windows_sys::core::HRESULT = -2147220223i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_SYNCHRONOUS: ::windows_sys::core::HRESULT = -2147220984i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_FLOATINGLANGBAR_WNDTITLE: &str = "TF_FloatingLangBar_WndTitle";
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_FLOATINGLANGBAR_WNDTITLEA: &str = "TF_FloatingLangBar_WndTitle";
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_FLOATINGLANGBAR_WNDTITLEW: &str = "TF_FloatingLangBar_WndTitle";
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub struct TF_HALTCOND {
    pub pHaltRange: *mut *mut *mut *mut ITfRange,
    pub aHaltPos: TfAnchor,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for TF_HALTCOND {}
impl ::core::clone::Clone for TF_HALTCOND {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_HF_OBJECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IE_CORRECTION: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub struct TF_INPUTPROCESSORPROFILE {
    pub dwProfileType: u32,
    pub langid: u16,
    pub clsid: ::windows_sys::core::GUID,
    pub guidProfile: ::windows_sys::core::GUID,
    pub catid: ::windows_sys::core::GUID,
    pub hklSubstitute: HKL,
    pub dwCaps: u32,
    pub hkl: HKL,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for TF_INPUTPROCESSORPROFILE {}
impl ::core::clone::Clone for TF_INPUTPROCESSORPROFILE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_INVALID_COOKIE: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_INVALID_EDIT_COOKIE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IPPMF_DISABLEPROFILE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IPPMF_DONTCARECURRENTINPUTLANGUAGE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IPPMF_ENABLEPROFILE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IPPMF_FORPROCESS: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IPPMF_FORSESSION: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IPPMF_FORSYSTEMALL: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IPP_CAPS_COMLESSSUPPORT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IPP_CAPS_DISABLEONTRANSITORY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IPP_CAPS_IMMERSIVESUPPORT: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IPP_CAPS_SECUREMODESUPPORT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IPP_CAPS_SYSTRAYSUPPORT: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IPP_CAPS_UIELEMENTENABLED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IPP_CAPS_WOW16SUPPORT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IPP_FLAG_ACTIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IPP_FLAG_ENABLED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IPP_FLAG_SUBSTITUTEDBYINPUTPROCESSOR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IPSINK_FLAG_ACTIVE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub struct TF_LANGBARITEMINFO {
    pub clsidService: ::windows_sys::core::GUID,
    pub guidItem: ::windows_sys::core::GUID,
    pub dwStyle: u32,
    pub ulSort: u32,
    pub szDescription: [u16; 32],
}
impl ::core::marker::Copy for TF_LANGBARITEMINFO {}
impl ::core::clone::Clone for TF_LANGBARITEMINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TF_LANGUAGEPROFILE {
    pub clsid: ::windows_sys::core::GUID,
    pub langid: u16,
    pub catid: ::windows_sys::core::GUID,
    pub fActive: super::super::Foundation::BOOL,
    pub guidProfile: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TF_LANGUAGEPROFILE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TF_LANGUAGEPROFILE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TF_LBBALLOONINFO {
    pub style: TfLBBalloonStyle,
    pub bstrText: super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TF_LBBALLOONINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TF_LBBALLOONINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_BALLOON: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_BITMAP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_BMPF_VERTICAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_CUSTOMUI: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_DESC_MAXLEN: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_ICON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_STATUS: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_STATUS_BTN_TOGGLED: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_STATUS_DISABLED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_STATUS_HIDDEN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_STYLE_BTN_BUTTON: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_STYLE_BTN_MENU: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_STYLE_BTN_TOGGLE: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_STYLE_HIDDENBYDEFAULT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_STYLE_HIDDENSTATUSCONTROL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_STYLE_HIDEONNOOTHERITEMS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_STYLE_SHOWNINTRAY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_STYLE_SHOWNINTRAYONLY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_STYLE_TEXTCOLORICON: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_TEXT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_TOOLTIP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBMENUF_CHECKED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBMENUF_GRAYED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBMENUF_RADIOCHECKED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBMENUF_SEPARATOR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBMENUF_SUBMENU: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TF_LMLATTELEMENT {
    pub dwFrameStart: u32,
    pub dwFrameLen: u32,
    pub dwFlags: u32,
    pub Anonymous: TF_LMLATTELEMENT_0,
    pub bstrText: super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TF_LMLATTELEMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TF_LMLATTELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union TF_LMLATTELEMENT_0 {
    pub iCost: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TF_LMLATTELEMENT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TF_LMLATTELEMENT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_MENUREADY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_MOD_ALT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_MOD_CONTROL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_MOD_IGNORE_ALL_MODIFIER: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_MOD_LALT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_MOD_LCONTROL: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_MOD_LSHIFT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_MOD_ON_KEYUP: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_MOD_RALT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_MOD_RCONTROL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_MOD_RSHIFT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_MOD_SHIFT: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub struct TF_PERSISTENT_PROPERTY_HEADER_ACP {
    pub guidType: ::windows_sys::core::GUID,
    pub ichStart: i32,
    pub cch: i32,
    pub cb: u32,
    pub dwPrivate: u32,
    pub clsidTIP: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for TF_PERSISTENT_PROPERTY_HEADER_ACP {}
impl ::core::clone::Clone for TF_PERSISTENT_PROPERTY_HEADER_ACP {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_POPF_ALL: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub struct TF_PRESERVEDKEY {
    pub uVKey: u32,
    pub uModifiers: u32,
}
impl ::core::marker::Copy for TF_PRESERVEDKEY {}
impl ::core::clone::Clone for TF_PRESERVEDKEY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_PROCESS_ATOM: &str = "_CTF_PROCESS_ATOM_";
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_PROFILETYPE_INPUTPROCESSOR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_PROFILETYPE_KEYBOARDLAYOUT: u32 = 2u32;
pub const TF_PROFILE_ARRAY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3549364069, data2: 43590, data3: 20437, data4: [145, 167, 103, 132, 95, 176, 47, 91] };
pub const TF_PROFILE_CANTONESE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 183242908, data2: 32406, data3: 4564, data4: [178, 239, 0, 128, 200, 130, 104, 126] };
pub const TF_PROFILE_CHANGJIE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1272946435, data2: 51155, data3: 4564, data4: [178, 171, 0, 128, 200, 130, 104, 126] };
pub const TF_PROFILE_DAYI: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 58403877, data2: 18444, data3: 19839, data4: [176, 39, 214, 202, 107, 105, 120, 138] };
pub const TF_PROFILE_NEWCHANGJIE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4089090170, data2: 27774, data3: 4564, data4: [151, 250, 0, 128, 200, 130, 104, 126] };
pub const TF_PROFILE_NEWPHONETIC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3002713346, data2: 5954, data3: 4564, data4: [151, 144, 0, 128, 200, 130, 104, 126] };
pub const TF_PROFILE_NEWQUICK: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 193477536, data2: 49607, data3: 4564, data4: [135, 249, 0, 128, 200, 130, 104, 126] };
pub const TF_PROFILE_PHONETIC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1980959198, data2: 12666, data3: 4564, data4: [155, 93, 0, 128, 200, 130, 104, 126] };
pub const TF_PROFILE_PINYIN: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4089090167, data2: 27774, data3: 4564, data4: [151, 250, 0, 128, 200, 130, 104, 126] };
pub const TF_PROFILE_QUICK: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1613018207, data2: 23636, data3: 4564, data4: [185, 33, 0, 128, 200, 130, 104, 126] };
pub const TF_PROFILE_SIMPLEFAST: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4199877380, data2: 23255, data3: 16671, data4: [165, 172, 202, 3, 142, 197, 21, 215] };
pub const TF_PROFILE_TIGRINYA: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1017874615, data2: 52286, data3: 18086, data4: [151, 101, 183, 114, 173, 119, 97, 255] };
pub const TF_PROFILE_WUBI: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2186873875, data2: 62685, data3: 17652, data4: [186, 29, 134, 103, 36, 111, 223, 142] };
pub const TF_PROFILE_YI: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1083999094, data2: 123, data3: 17239, data4: [174, 142, 38, 49, 110, 227, 251, 13] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct TF_PROPERTYVAL {
    pub guidId: ::windows_sys::core::GUID,
    pub varValue: super::super::System::Com::VARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for TF_PROPERTYVAL {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for TF_PROPERTYVAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_PROPUI_STATUS_SAVETOFILE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_RCM_COMLESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_RCM_HINT_COLLISION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_RCM_HINT_READING_LENGTH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_RCM_VKEY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_RIP_FLAG_FREEUNUSEDLIBRARIES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_RIUIE_CONTEXT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_RIUIE_ERRORINDEX: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_RIUIE_MAXREADINGSTRINGLENGTH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_RIUIE_STRING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_RIUIE_VERTICALORDER: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_RP_HIDDENINSETTINGUI: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_RP_LOCALPROCESS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_RP_LOCALTHREAD: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_RP_SUBITEMINSETTINGUI: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SD_LOADING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SD_READONLY: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TF_SELECTION {
    pub range: *mut *mut *mut *mut ITfRange,
    pub style: TF_SELECTIONSTYLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TF_SELECTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TF_SELECTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TF_SELECTIONSTYLE {
    pub ase: TfActiveSelEnd,
    pub fInterimChar: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TF_SELECTIONSTYLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TF_SELECTIONSTYLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SENTENCEMODE_AUTOMATIC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SENTENCEMODE_CONVERSATION: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SENTENCEMODE_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SENTENCEMODE_PHRASEPREDICT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SENTENCEMODE_PLAURALCLAUSE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SENTENCEMODE_SINGLECONVERT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SFT_DESKBAND: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SFT_DOCK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SFT_EXTRAICONSONMINIMIZED: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SFT_HIDDEN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SFT_HIGHTRANSPARENCY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SFT_LABELS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SFT_LOWTRANSPARENCY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SFT_MINIMIZED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SFT_NOEXTRAICONSONMINIMIZED: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SFT_NOLABELS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SFT_NOTRANSPARENCY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SFT_SHOWNORMAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SHOW_BALLOON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SPEECHUI_SHOWN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SS_DISJOINTSEL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SS_REGIONS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SS_TKBAUTOCORRECTENABLE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SS_TKBPREDICTIONENABLE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SS_TRANSITORY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_ST_CORRECTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_S_ASYNC: ::windows_sys::core::HRESULT = 262912i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TF_IGNOREEND: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TF_MOVESTART: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TMAE_COMLESS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TMAE_CONSOLE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TMAE_NOACTIVATEKEYBOARDLAYOUT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TMAE_NOACTIVATETIP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TMAE_SECUREMODE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TMAE_UIELEMENTENABLEDONLY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TMAE_WOW16: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TMF_ACTIVATED: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TMF_COMLESS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TMF_CONSOLE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TMF_IMMERSIVEMODE: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TMF_NOACTIVATETIP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TMF_SECUREMODE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TMF_UIELEMENTENABLEDONLY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TMF_WOW16: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TRANSITORYEXTENSION_ATSELECTION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TRANSITORYEXTENSION_FLOATING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TRANSITORYEXTENSION_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TU_CORRECTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_URP_ALLPROFILES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_URP_LOCALPROCESS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_URP_LOCALTHREAD: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_US_HIDETIPUI: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TKBL_CLASSIC_TRADITIONAL_CHINESE_CHANGJIE: u32 = 61506u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TKBL_CLASSIC_TRADITIONAL_CHINESE_DAYI: u32 = 61507u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TKBL_CLASSIC_TRADITIONAL_CHINESE_PHONETIC: u32 = 1028u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TKBL_OPT_JAPANESE_ABC: u32 = 1041u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TKBL_OPT_KOREAN_HANGUL_2_BULSIK: u32 = 1042u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TKBL_OPT_SIMPLIFIED_CHINESE_PINYIN: u32 = 2052u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TKBL_OPT_TRADITIONAL_CHINESE_PHONETIC: u32 = 1028u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TKBL_UNDEFINED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub type TKBLayoutType = i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TKBLT_UNDEFINED: TKBLayoutType = 0i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TKBLT_CLASSIC: TKBLayoutType = 1i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TKBLT_OPTIMIZED: TKBLayoutType = 2i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TKB_ALTERNATES_AUTOCORRECTION_APPLIED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TKB_ALTERNATES_FOR_AUTOCORRECTION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TKB_ALTERNATES_FOR_PREDICTION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TKB_ALTERNATES_STANDARD: u32 = 1u32;
pub const TSATTRID_App: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2819586015, data2: 16951, data3: 16613, data4: [132, 156, 181, 250, 81, 193, 58, 199] };
pub const TSATTRID_App_IncorrectGrammar: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3176457112, data2: 44291, data3: 19316, data4: [182, 179, 94, 219, 25, 153, 99, 136] };
pub const TSATTRID_App_IncorrectSpelling: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4096648252, data2: 61202, data3: 17165, data4: [148, 76, 154, 8, 151, 10, 37, 210] };
pub const TSATTRID_Font: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1463724069, data2: 29851, data3: 20362, data4: [156, 253, 33, 195, 96, 92, 168, 40] };
pub const TSATTRID_Font_FaceName: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3040259766, data2: 1339, data3: 20152, data4: [182, 90, 80, 218, 30, 129, 231, 46] };
pub const TSATTRID_Font_SizePts: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3360240386, data2: 42473, data3: 17773, data4: [175, 4, 128, 5, 228, 19, 15, 3] };
pub const TSATTRID_Font_Style: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1756538751, data2: 27406, data3: 20264, data4: [129, 119, 87, 28, 47, 58, 66, 177] };
pub const TSATTRID_Font_Style_Animation: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3707190562, data2: 57385, data3: 18359, data4: [187, 54, 242, 99, 163, 208, 4, 204] };
pub const TSATTRID_Font_Style_Animation_BlinkingBackground: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2263200004, data2: 260, data3: 19216, data4: [181, 133, 0, 242, 82, 117, 34, 181] };
pub const TSATTRID_Font_Style_Animation_LasVegasLights: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4093912021, data2: 3975, data3: 20367, data4: [186, 218, 230, 214, 12, 37, 225, 82] };
pub const TSATTRID_Font_Style_Animation_MarchingBlackAnts: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1984225383, data2: 61830, data3: 18690, data4: [191, 198, 236, 129, 90, 162, 14, 157] };
pub const TSATTRID_Font_Style_Animation_MarchingRedAnts: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2016841133, data2: 20731, data3: 19567, data4: [132, 11, 212, 134, 187, 108, 247, 129] };
pub const TSATTRID_Font_Style_Animation_Shimmer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 753081176, data2: 21139, data3: 19510, data4: [136, 9, 191, 139, 181, 26, 39, 179] };
pub const TSATTRID_Font_Style_Animation_SparkleText: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1396354336, data2: 38444, data3: 20127, data4: [140, 9, 180, 46, 164, 116, 151, 17] };
pub const TSATTRID_Font_Style_Animation_WipeDown: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1483925620, data2: 13947, data3: 18435, data4: [177, 96, 201, 15, 246, 37, 105, 208] };
pub const TSATTRID_Font_Style_Animation_WipeRight: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3092630499, data2: 15660, data3: 17920, data4: [177, 233, 225, 201, 206, 2, 248, 66] };
pub const TSATTRID_Font_Style_BackgroundColor: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3037637198, data2: 12433, data3: 17512, data4: [129, 219, 215, 158, 161, 144, 199, 199] };
pub const TSATTRID_Font_Style_Blink: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3216162870, data2: 31439, data3: 17714, data4: [183, 32, 180, 22, 221, 119, 101, 168] };
pub const TSATTRID_Font_Style_Bold: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1216428611, data2: 35360, data3: 18752, data4: [142, 88, 151, 130, 63, 123, 38, 138] };
pub const TSATTRID_Font_Style_Capitalize: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2105910202, data2: 46333, data3: 17331, data4: [190, 252, 107, 152, 92, 132, 49, 65] };
pub const TSATTRID_Font_Style_Color: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2239396407, data2: 47279, data3: 20122, data4: [129, 180, 172, 247, 0, 200, 65, 27] };
pub const TSATTRID_Font_Style_Emboss: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3180255042, data2: 13470, data3: 20023, data4: [130, 251, 67, 121, 121, 203, 83, 167] };
pub const TSATTRID_Font_Style_Engrave: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2620617182, data2: 33586, data3: 18583, data4: [190, 93, 137, 35, 50, 35, 23, 154] };
pub const TSATTRID_Font_Style_Height: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2123592823, data2: 4838, data3: 17803, data4: [146, 106, 31, 164, 78, 232, 243, 145] };
pub const TSATTRID_Font_Style_Hidden: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2984413040, data2: 34844, data3: 18271, data4: [134, 63, 136, 122, 100, 123, 16, 144] };
pub const TSATTRID_Font_Style_Italic: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2269145130, data2: 42853, data3: 18657, data4: [172, 252, 210, 34, 34, 178, 248, 16] };
pub const TSATTRID_Font_Style_Kerning: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3425100212, data2: 12186, data3: 18376, data4: [139, 255, 191, 30, 183, 204, 224, 221] };
pub const TSATTRID_Font_Style_Lowercase: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1993919669, data2: 51835, data3: 17560, data4: [142, 233, 213, 196, 246, 247, 76, 96] };
pub const TSATTRID_Font_Style_Outlined: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 283564849, data2: 56077, data3: 19142, data4: [167, 245, 156, 156, 255, 111, 42, 180] };
pub const TSATTRID_Font_Style_Overline: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3818430282, data2: 39211, data3: 17153, data4: [140, 225, 165, 183, 198, 209, 243, 200] };
pub const TSATTRID_Font_Style_Overline_Double: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3695576634, data2: 57621, data3: 18147, data4: [188, 216, 202, 103, 114, 170, 149, 180] };
pub const TSATTRID_Font_Style_Overline_Single: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2218842444, data2: 20942, data3: 18354, data4: [141, 76, 21, 117, 30, 95, 114, 27] };
pub const TSATTRID_Font_Style_Position: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 365766315, data2: 62203, data3: 16482, data4: [181, 166, 154, 73, 225, 165, 204, 11] };
pub const TSATTRID_Font_Style_Protected: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 475364530, data2: 5327, data3: 17748, data4: [165, 116, 236, 178, 247, 231, 239, 212] };
pub const TSATTRID_Font_Style_Shadow: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1600679215, data2: 50893, data3: 19542, data4: [138, 26, 153, 74, 75, 151, 102, 190] };
pub const TSATTRID_Font_Style_SmallCaps: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4207635398, data2: 37120, data3: 19654, data4: [185, 105, 17, 238, 164, 90, 134, 180] };
pub const TSATTRID_Font_Style_Spacing: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2562793485, data2: 36614, data3: 16538, data4: [142, 73, 106, 85, 75, 247, 193, 83] };
pub const TSATTRID_Font_Style_Strikethrough: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 206971283, data2: 11528, data3: 18024, data4: [150, 1, 206, 212, 19, 9, 215, 175] };
pub const TSATTRID_Font_Style_Strikethrough_Double: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1648925489, data2: 41959, data3: 20372, data4: [172, 67, 235, 175, 143, 204, 122, 159] };
pub const TSATTRID_Font_Style_Strikethrough_Single: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1977038518, data2: 15503, data3: 19351, data4: [171, 120, 24, 119, 203, 153, 13, 49] };
pub const TSATTRID_Font_Style_Subscript: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1467284356, data2: 14491, data3: 17340, data4: [167, 75, 21, 104, 52, 124, 240, 244] };
pub const TSATTRID_Font_Style_Superscript: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 782539068, data2: 22076, data3: 18858, data4: [147, 114, 11, 239, 9, 169, 37, 91] };
pub const TSATTRID_Font_Style_Underline: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3284781555, data2: 30978, data3: 17483, data4: [154, 123, 72, 231, 15, 75, 80, 247] };
pub const TSATTRID_Font_Style_Underline_Double: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1959938726, data2: 7603, data3: 19561, data4: [161, 118, 49, 18, 14, 117, 134, 213] };
pub const TSATTRID_Font_Style_Underline_Single: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 459743461, data2: 3955, data3: 18769, data4: [166, 179, 111, 25, 228, 60, 148, 97] };
pub const TSATTRID_Font_Style_Uppercase: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 866320616, data2: 58176, data3: 18743, data4: [182, 151, 143, 35, 64, 69, 205, 154] };
pub const TSATTRID_Font_Style_Weight: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 317921436, data2: 35760, data3: 17947, data4: [177, 250, 234, 249, 7, 4, 127, 224] };
pub const TSATTRID_List: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1131243323, data2: 9969, data3: 19182, data4: [158, 101, 143, 131, 164, 237, 72, 132] };
pub const TSATTRID_List_LevelIndel: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2138884249, data2: 12575, data3: 18555, data4: [173, 93, 226, 164, 89, 225, 45, 66] };
pub const TSATTRID_List_Type: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2923325022, data2: 19406, data3: 18915, data4: [160, 254, 45, 180, 125, 58, 23, 174] };
pub const TSATTRID_List_Type_Arabic: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 322487766, data2: 39075, data3: 20387, data4: [155, 209, 122, 96, 238, 248, 233, 224] };
pub const TSATTRID_List_Type_Bullet: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3167582149, data2: 19533, data3: 19682, data4: [177, 2, 85, 159, 59, 43, 252, 234] };
pub const TSATTRID_List_Type_LowerLetter: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2520195717, data2: 62415, data3: 18718, data4: [169, 37, 56, 50, 52, 127, 210, 55] };
pub const TSATTRID_List_Type_LowerRoman: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2420531810, data2: 14720, data3: 19342, data4: [147, 104, 145, 139, 209, 33, 138, 65] };
pub const TSATTRID_List_Type_UpperLetter: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2038937549, data2: 52818, data3: 17035, data4: [155, 149, 163, 87, 246, 241, 12, 69] };
pub const TSATTRID_List_Type_UpperRoman: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 258651474, data2: 19072, data3: 18047, data4: [178, 241, 18, 126, 42, 163, 186, 158] };
pub const TSATTRID_OTHERS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3015912185, data2: 22480, data3: 18089, data4: [188, 168, 218, 194, 56, 161, 48, 87] };
pub const TSATTRID_Text: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2128318056, data2: 33273, data3: 17565, data4: [161, 90, 135, 168, 56, 143, 170, 192] };
pub const TSATTRID_Text_Alignment: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 328810982, data2: 5991, data3: 17773, data4: [147, 142, 53, 186, 86, 139, 92, 212] };
pub const TSATTRID_Text_Alignment_Center: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2762562582, data2: 21439, data3: 19797, data4: [139, 135, 75, 221, 141, 66, 117, 252] };
pub const TSATTRID_Text_Alignment_Justify: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3979675456, data2: 41207, data3: 17107, data4: [142, 168, 248, 27, 100, 136, 250, 240] };
pub const TSATTRID_Text_Alignment_Left: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 380540371, data2: 25441, data3: 17314, data4: [132, 149, 208, 15, 57, 127, 22, 147] };
pub const TSATTRID_Text_Alignment_Right: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3010400152, data2: 7070, data3: 17248, data4: [134, 22, 3, 251, 8, 167, 132, 86] };
pub const TSATTRID_Text_EmbeddedObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2128318056, data2: 33273, data3: 17565, data4: [161, 90, 135, 168, 56, 143, 170, 192] };
pub const TSATTRID_Text_Hyphenation: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3672065317, data2: 24974, data3: 18923, data4: [177, 168, 59, 104, 189, 118, 72, 227] };
pub const TSATTRID_Text_Language: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3636481777, data2: 22355, data3: 19493, data4: [136, 135, 133, 68, 63, 229, 248, 25] };
pub const TSATTRID_Text_Link: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1204654161, data2: 14114, data3: 19672, data4: [183, 200, 78, 23, 202, 23, 89, 245] };
pub const TSATTRID_Text_Orientation: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1806397567, data2: 34693, data3: 19513, data4: [139, 82, 150, 248, 120, 48, 63, 251] };
pub const TSATTRID_Text_Para: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1591498786, data2: 39388, data3: 19926, data4: [174, 195, 182, 43, 170, 91, 46, 124] };
pub const TSATTRID_Text_Para_FirstLineIndent: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 130644499, data2: 29810, data3: 19928, data4: [144, 169, 145, 227, 215, 228, 242, 156] };
pub const TSATTRID_Text_Para_LeftIndent: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4213721321, data2: 29809, data3: 16841, data4: [182, 179, 138, 20, 80, 224, 24, 151] };
pub const TSATTRID_Text_Para_LineSpacing: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1771780109, data2: 32652, data3: 18134, data4: [167, 59, 223, 227, 209, 83, 141, 243] };
pub const TSATTRID_Text_Para_LineSpacing_AtLeast: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2919161649, data2: 11588, data3: 17460, data4: [165, 255, 127, 76, 73, 144, 169, 5] };
pub const TSATTRID_Text_Para_LineSpacing_Double: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2197493765, data2: 42692, data3: 16945, data4: [172, 18, 98, 96, 175, 42, 186, 40] };
pub const TSATTRID_Text_Para_LineSpacing_Exactly: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1027976512, data2: 9182, data3: 18647, data4: [166, 179, 118, 84, 32, 198, 32, 204] };
pub const TSATTRID_Text_Para_LineSpacing_Multiple: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2433687100, data2: 54992, data3: 20325, data4: [138, 60, 66, 180, 179, 24, 104, 197] };
pub const TSATTRID_Text_Para_LineSpacing_OnePtFive: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 69771297, data2: 919, data3: 19287, data4: [154, 23, 7, 149, 153, 76, 211, 197] };
pub const TSATTRID_Text_Para_LineSpacing_Single: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3979675456, data2: 41207, data3: 17107, data4: [142, 168, 248, 27, 100, 136, 250, 240] };
pub const TSATTRID_Text_Para_RightIndent: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 746530553, data2: 42466, data3: 18650, data4: [185, 138, 82, 12, 177, 101, 19, 191] };
pub const TSATTRID_Text_Para_SpaceAfter: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2064269141, data2: 8924, data3: 16991, data4: [164, 17, 147, 218, 29, 143, 155, 170] };
pub const TSATTRID_Text_Para_SpaceBefore: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2381940105, data2: 6474, data3: 17921, data4: [178, 81, 152, 101, 163, 233, 6, 221] };
pub const TSATTRID_Text_ReadOnly: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2239981079, data2: 56882, data3: 19197, data4: [165, 15, 162, 219, 17, 14, 110, 77] };
pub const TSATTRID_Text_RightToLeft: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3395710577, data2: 6920, data3: 17725, data4: [191, 221, 40, 224, 140, 138, 175, 122] };
pub const TSATTRID_Text_VerticalWriting: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1807384981, data2: 1135, data3: 20137, data4: [179, 17, 151, 253, 102, 196, 39, 75] };
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_AS_ATTR_CHANGE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_AS_LAYOUT_CHANGE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_AS_SEL_CHANGE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_AS_STATUS_CHANGE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_AS_TEXT_CHANGE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct TS_ATTRVAL {
    pub idAttr: ::windows_sys::core::GUID,
    pub dwOverlapId: u32,
    pub varValue: super::super::System::Com::VARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for TS_ATTRVAL {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for TS_ATTRVAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_ATTR_FIND_BACKWARDS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_ATTR_FIND_HIDDEN: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_ATTR_FIND_UPDATESTART: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_ATTR_FIND_WANT_END: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_ATTR_FIND_WANT_OFFSET: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_ATTR_FIND_WANT_VALUE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_CHAR_EMBEDDED: u32 = 65532u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_CHAR_REGION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_CHAR_REPLACEMENT: u32 = 65533u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_E_FORMAT: ::windows_sys::core::HRESULT = -2147220982i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_E_INVALIDPOINT: ::windows_sys::core::HRESULT = -2147220985i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_E_INVALIDPOS: ::windows_sys::core::HRESULT = -2147220992i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_E_NOINTERFACE: ::windows_sys::core::HRESULT = -2147220988i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_E_NOLAYOUT: ::windows_sys::core::HRESULT = -2147220986i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_E_NOLOCK: ::windows_sys::core::HRESULT = -2147220991i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_E_NOOBJECT: ::windows_sys::core::HRESULT = -2147220990i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_E_NOSELECTION: ::windows_sys::core::HRESULT = -2147220987i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_E_NOSERVICE: ::windows_sys::core::HRESULT = -2147220989i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_E_READONLY: ::windows_sys::core::HRESULT = -2147220983i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_E_SYNCHRONOUS: ::windows_sys::core::HRESULT = -2147220984i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_GEA_HIDDEN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_GTA_HIDDEN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_IAS_NOQUERY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_IAS_QUERYONLY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_IE_COMPOSITION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_IE_CORRECTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_LF_SYNC: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub struct TS_RUNINFO {
    pub uCount: u32,
    pub r#type: TsRunType,
}
impl ::core::marker::Copy for TS_RUNINFO {}
impl ::core::clone::Clone for TS_RUNINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SD_EMBEDDEDHANDWRITINGVIEW_ENABLED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SD_EMBEDDEDHANDWRITINGVIEW_VISIBLE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SD_INPUTPANEMANUALDISPLAYENABLE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SD_LOADING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SD_READONLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SD_RESERVED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SD_TKBAUTOCORRECTENABLE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SD_TKBPREDICTIONENABLE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SD_UIINTEGRATIONENABLE: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TS_SELECTIONSTYLE {
    pub ase: TsActiveSelEnd,
    pub fInterimChar: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TS_SELECTIONSTYLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TS_SELECTIONSTYLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TS_SELECTION_ACP {
    pub acpStart: i32,
    pub acpEnd: i32,
    pub style: TS_SELECTIONSTYLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TS_SELECTION_ACP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TS_SELECTION_ACP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TS_SELECTION_ANCHOR {
    pub paStart: *mut *mut *mut *mut IAnchor,
    pub paEnd: *mut *mut *mut *mut IAnchor,
    pub style: TS_SELECTIONSTYLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TS_SELECTION_ANCHOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TS_SELECTION_ANCHOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SHIFT_COUNT_HIDDEN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SHIFT_COUNT_ONLY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SHIFT_HALT_HIDDEN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SHIFT_HALT_VISIBLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SS_DISJOINTSEL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SS_NOHIDDENTEXT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SS_REGIONS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SS_TKBAUTOCORRECTENABLE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SS_TKBPREDICTIONENABLE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SS_TRANSITORY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SS_UWPCONTROL: u32 = 64u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub struct TS_STATUS {
    pub dwDynamicFlags: u32,
    pub dwStaticFlags: u32,
}
impl ::core::marker::Copy for TS_STATUS {}
impl ::core::clone::Clone for TS_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_STRF_END: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_STRF_MID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_STRF_START: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_S_ASYNC: ::windows_sys::core::HRESULT = 262912i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub struct TS_TEXTCHANGE {
    pub acpStart: i32,
    pub acpOldEnd: i32,
    pub acpNewEnd: i32,
}
impl ::core::marker::Copy for TS_TEXTCHANGE {}
impl ::core::clone::Clone for TS_TEXTCHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_VCOOKIE_NUL: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub type TfActiveSelEnd = i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_AE_NONE: TfActiveSelEnd = 0i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_AE_START: TfActiveSelEnd = 1i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_AE_END: TfActiveSelEnd = 2i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub type TfAnchor = i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_ANCHOR_START: TfAnchor = 0i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_ANCHOR_END: TfAnchor = 1i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub type TfCandidateResult = i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const CAND_FINALIZED: TfCandidateResult = 0i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const CAND_SELECTED: TfCandidateResult = 1i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const CAND_CANCELED: TfCandidateResult = 2i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub type TfGravity = i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_GRAVITY_BACKWARD: TfGravity = 0i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_GRAVITY_FORWARD: TfGravity = 1i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub type TfIntegratableCandidateListSelectionStyle = i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const STYLE_ACTIVE_SELECTION: TfIntegratableCandidateListSelectionStyle = 0i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const STYLE_IMPLIED_SELECTION: TfIntegratableCandidateListSelectionStyle = 1i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub type TfLBBalloonStyle = i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LB_BALLOON_RECO: TfLBBalloonStyle = 0i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LB_BALLOON_SHOW: TfLBBalloonStyle = 1i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LB_BALLOON_MISS: TfLBBalloonStyle = 2i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub type TfLBIClick = i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_CLK_RIGHT: TfLBIClick = 1i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_CLK_LEFT: TfLBIClick = 2i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub type TfLayoutCode = i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LC_CREATE: TfLayoutCode = 0i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LC_CHANGE: TfLayoutCode = 1i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LC_DESTROY: TfLayoutCode = 2i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub type TfSapiObject = i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GETIF_RESMGR: TfSapiObject = 0i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GETIF_RECOCONTEXT: TfSapiObject = 1i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GETIF_RECOGNIZER: TfSapiObject = 2i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GETIF_VOICE: TfSapiObject = 3i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GETIF_DICTGRAM: TfSapiObject = 4i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GETIF_RECOGNIZERNOINIT: TfSapiObject = 5i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub type TfShiftDir = i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SD_BACKWARD: TfShiftDir = 0i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SD_FORWARD: TfShiftDir = 1i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub type TsActiveSelEnd = i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_AE_NONE: TsActiveSelEnd = 0i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_AE_START: TsActiveSelEnd = 1i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_AE_END: TsActiveSelEnd = 2i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub type TsGravity = i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_GR_BACKWARD: TsGravity = 0i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_GR_FORWARD: TsGravity = 1i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub type TsLayoutCode = i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_LC_CREATE: TsLayoutCode = 0i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_LC_CHANGE: TsLayoutCode = 1i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_LC_DESTROY: TsLayoutCode = 2i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub type TsRunType = i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_RT_PLAIN: TsRunType = 0i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_RT_HIDDEN: TsRunType = 1i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_RT_OPAQUE: TsRunType = 2i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub type TsShiftDir = i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SD_BACKWARD: TsShiftDir = 0i32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SD_FORWARD: TsShiftDir = 1i32;
