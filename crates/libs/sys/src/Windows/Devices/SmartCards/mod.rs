pub type CardAddedEventArgs = *mut ::core::ffi::c_void;
pub type CardRemovedEventArgs = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct ICardAddedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub SmartCard: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICardAddedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 414969752, data2: 61835, data3: 19923, data4: [177, 24, 223, 178, 200, 226, 60, 198] };
}
#[repr(C)]
pub struct ICardRemovedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub SmartCard: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICardRemovedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 355670703, data2: 8919, data3: 18757, data4: [175, 201, 3, 180, 111, 66, 166, 205] };
}
#[repr(C)]
pub struct IKnownSmartCardAppletIds {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub PaymentSystemEnvironment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PaymentSystemEnvironment: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ProximityPaymentSystemEnvironment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ProximityPaymentSystemEnvironment: usize,
}
impl ::windows_sys::core::Interface for IKnownSmartCardAppletIds {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2063915224, data2: 38324, data3: 19592, data4: [140, 234, 65, 30, 85, 81, 30, 252] };
}
#[repr(C)]
pub struct ISmartCard {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetStatusAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetStatusAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetAnswerToResetAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetAnswerToResetAsync: usize,
}
impl ::windows_sys::core::Interface for ISmartCard {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 460425329, data2: 25652, data3: 17396, data4: [181, 90, 106, 41, 98, 56, 112, 170] };
}
#[repr(C)]
pub struct ISmartCardAppletIdGroup {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub AppletIds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    AppletIds: usize,
    pub SmartCardEmulationCategory: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmartCardEmulationCategory) -> ::windows_sys::core::HRESULT,
    pub SetSmartCardEmulationCategory: unsafe extern "system" fn(this: *mut *mut Self, value: SmartCardEmulationCategory) -> ::windows_sys::core::HRESULT,
    pub SmartCardEmulationType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmartCardEmulationType) -> ::windows_sys::core::HRESULT,
    pub SetSmartCardEmulationType: unsafe extern "system" fn(this: *mut *mut Self, value: SmartCardEmulationType) -> ::windows_sys::core::HRESULT,
    pub AutomaticEnablement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAutomaticEnablement: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISmartCardAppletIdGroup {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2108777958, data2: 25188, data3: 22260, data4: [94, 3, 200, 99, 133, 57, 94, 177] };
}
#[repr(C)]
pub struct ISmartCardAppletIdGroup2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub Logo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Logo: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetLogo: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetLogo: usize,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    pub SecureUserAuthenticationRequired: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetSecureUserAuthenticationRequired: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISmartCardAppletIdGroup2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1796143580, data2: 39254, data3: 19042, data4: [141, 78, 211, 122, 104, 235, 195, 166] };
}
#[repr(C)]
pub struct ISmartCardAppletIdGroupFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, displayname: ::windows_sys::core::HSTRING, appletids: *mut ::core::ffi::c_void, emulationcategory: SmartCardEmulationCategory, emulationtype: SmartCardEmulationType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    Create: usize,
}
impl ::windows_sys::core::Interface for ISmartCardAppletIdGroupFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2433084237, data2: 19045, data3: 20033, data4: [128, 97, 203, 232, 63, 54, 149, 229] };
}
#[repr(C)]
pub struct ISmartCardAppletIdGroupRegistration {
    pub base__: ::windows_sys::core::IInspectable,
    pub ActivationPolicy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmartCardAppletIdGroupActivationPolicy) -> ::windows_sys::core::HRESULT,
    pub AppletIdGroup: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestActivationPolicyChangeAsync: unsafe extern "system" fn(this: *mut *mut Self, policy: SmartCardAppletIdGroupActivationPolicy, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestActivationPolicyChangeAsync: usize,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SetAutomaticResponseApdusAsync: unsafe extern "system" fn(this: *mut *mut Self, apdus: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetAutomaticResponseApdusAsync: usize,
}
impl ::windows_sys::core::Interface for ISmartCardAppletIdGroupRegistration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3742501073, data2: 12731, data3: 21910, data4: [67, 177, 109, 105, 160, 37, 123, 58] };
}
#[repr(C)]
pub struct ISmartCardAppletIdGroupRegistration2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SmartCardReaderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SetPropertiesAsync: unsafe extern "system" fn(this: *mut *mut Self, props: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetPropertiesAsync: usize,
}
impl ::windows_sys::core::Interface for ISmartCardAppletIdGroupRegistration2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1599408344, data2: 39079, data3: 20270, data4: [145, 217, 108, 252, 206, 218, 64, 127] };
}
#[repr(C)]
pub struct ISmartCardAppletIdGroupStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxAppletIds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISmartCardAppletIdGroupStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2871564713, data2: 59244, data3: 17871, data4: [191, 29, 144, 234, 166, 32, 89, 39] };
}
#[repr(C)]
pub struct ISmartCardAutomaticResponseApdu {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub CommandApdu: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CommandApdu: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetCommandApdu: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetCommandApdu: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CommandApduBitMask: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CommandApduBitMask: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetCommandApduBitMask: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetCommandApduBitMask: usize,
    pub ShouldMatchLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetShouldMatchLength: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub AppletId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AppletId: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetAppletId: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetAppletId: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ResponseApdu: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ResponseApdu: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetResponseApdu: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetResponseApdu: usize,
}
impl ::windows_sys::core::Interface for ISmartCardAutomaticResponseApdu {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1377119147, data2: 50750, data3: 17713, data4: [168, 87, 215, 86, 217, 155, 152, 106] };
}
#[repr(C)]
pub struct ISmartCardAutomaticResponseApdu2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub InputState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InputState: usize,
    #[cfg(feature = "Foundation")]
    pub SetInputState: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetInputState: usize,
    #[cfg(feature = "Foundation")]
    pub OutputState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OutputState: usize,
    #[cfg(feature = "Foundation")]
    pub SetOutputState: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetOutputState: usize,
}
impl ::windows_sys::core::Interface for ISmartCardAutomaticResponseApdu2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1152301844, data2: 21917, data3: 17713, data4: [78, 81, 137, 219, 111, 168, 165, 122] };
}
#[repr(C)]
pub struct ISmartCardAutomaticResponseApdu3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AllowWhenCryptogramGeneratorNotPrepared: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowWhenCryptogramGeneratorNotPrepared: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISmartCardAutomaticResponseApdu3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3208895092, data2: 25974, data3: 17298, data4: [147, 103, 254, 59, 201, 226, 212, 150] };
}
#[repr(C)]
pub struct ISmartCardAutomaticResponseApduFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, commandapdu: *mut ::core::ffi::c_void, responseapdu: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for ISmartCardAutomaticResponseApduFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3917390586, data2: 53292, data3: 19541, data4: [176, 42, 140, 255, 127, 169, 240, 91] };
}
#[repr(C)]
pub struct ISmartCardChallengeContext {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub Challenge: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Challenge: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub VerifyResponseAsync: unsafe extern "system" fn(this: *mut *mut Self, response: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    VerifyResponseAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ProvisionAsync: unsafe extern "system" fn(this: *mut *mut Self, response: *mut ::core::ffi::c_void, formatcard: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ProvisionAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ProvisionAsyncWithNewCardId: unsafe extern "system" fn(this: *mut *mut Self, response: *mut ::core::ffi::c_void, formatcard: bool, newcardid: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ProvisionAsyncWithNewCardId: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ChangeAdministrativeKeyAsync: unsafe extern "system" fn(this: *mut *mut Self, response: *mut ::core::ffi::c_void, newadministrativekey: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ChangeAdministrativeKeyAsync: usize,
}
impl ::windows_sys::core::Interface for ISmartCardChallengeContext {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 422204185, data2: 51652, data3: 18759, data4: [129, 204, 68, 121, 74, 97, 239, 145] };
}
#[repr(C)]
pub struct ISmartCardConnect {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ConnectAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectAsync: usize,
}
impl ::windows_sys::core::Interface for ISmartCardConnect {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 803178469, data2: 653, data3: 18718, data4: [160, 88, 51, 130, 195, 152, 111, 64] };
}
#[repr(C)]
pub struct ISmartCardConnection {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub TransmitAsync: unsafe extern "system" fn(this: *mut *mut Self, command: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    TransmitAsync: usize,
}
impl ::windows_sys::core::Interface for ISmartCardConnection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2128320794, data2: 43034, data3: 18364, data4: [166, 73, 21, 107, 230, 183, 242, 49] };
}
#[repr(C)]
pub struct ISmartCardCryptogramGenerator {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCryptogramMaterialTypes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCryptogramMaterialTypes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCryptogramAlgorithms: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCryptogramAlgorithms: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCryptogramMaterialPackageFormats: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCryptogramMaterialPackageFormats: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCryptogramMaterialPackageConfirmationResponseFormats: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCryptogramMaterialPackageConfirmationResponseFormats: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedSmartCardCryptogramStorageKeyCapabilities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedSmartCardCryptogramStorageKeyCapabilities: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteCryptogramMaterialStorageKeyAsync: unsafe extern "system" fn(this: *mut *mut Self, storagekeyname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteCryptogramMaterialStorageKeyAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateCryptogramMaterialStorageKeyAsync: unsafe extern "system" fn(this: *mut *mut Self, promptingbehavior: SmartCardUnlockPromptingBehavior, storagekeyname: ::windows_sys::core::HSTRING, algorithm: SmartCardCryptogramStorageKeyAlgorithm, capabilities: SmartCardCryptogramStorageKeyCapabilities, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateCryptogramMaterialStorageKeyAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Core"))]
    pub RequestCryptogramMaterialStorageKeyInfoAsync: unsafe extern "system" fn(this: *mut *mut Self, promptingbehavior: SmartCardUnlockPromptingBehavior, storagekeyname: ::windows_sys::core::HSTRING, format: super::super::Security::Cryptography::Core::CryptographicPublicKeyBlobType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Cryptography_Core")))]
    RequestCryptogramMaterialStorageKeyInfoAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ImportCryptogramMaterialPackageAsync: unsafe extern "system" fn(this: *mut *mut Self, format: SmartCardCryptogramMaterialPackageFormat, storagekeyname: ::windows_sys::core::HSTRING, materialpackagename: ::windows_sys::core::HSTRING, cryptogrammaterialpackage: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ImportCryptogramMaterialPackageAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub TryProvePossessionOfCryptogramMaterialPackageAsync: unsafe extern "system" fn(this: *mut *mut Self, promptingbehavior: SmartCardUnlockPromptingBehavior, responseformat: SmartCardCryptogramMaterialPackageConfirmationResponseFormat, materialpackagename: ::windows_sys::core::HSTRING, materialname: ::windows_sys::core::HSTRING, challenge: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    TryProvePossessionOfCryptogramMaterialPackageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestUnlockCryptogramMaterialForUseAsync: unsafe extern "system" fn(this: *mut *mut Self, promptingbehavior: SmartCardUnlockPromptingBehavior, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestUnlockCryptogramMaterialForUseAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteCryptogramMaterialPackageAsync: unsafe extern "system" fn(this: *mut *mut Self, materialpackagename: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteCryptogramMaterialPackageAsync: usize,
}
impl ::windows_sys::core::Interface for ISmartCardCryptogramGenerator {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3818870907, data2: 60883, data3: 20041, data4: [181, 148, 15, 245, 228, 208, 199, 111] };
}
#[repr(C)]
pub struct ISmartCardCryptogramGenerator2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub ValidateRequestApduAsync: unsafe extern "system" fn(this: *mut *mut Self, promptingbehavior: SmartCardUnlockPromptingBehavior, apdutovalidate: *mut ::core::ffi::c_void, cryptogramplacementsteps: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    ValidateRequestApduAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAllCryptogramStorageKeyCharacteristicsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAllCryptogramStorageKeyCharacteristicsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAllCryptogramMaterialPackageCharacteristicsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAllCryptogramMaterialPackageCharacteristicsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAllCryptogramMaterialPackageCharacteristicsWithStorageKeyAsync: unsafe extern "system" fn(this: *mut *mut Self, storagekeyname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAllCryptogramMaterialPackageCharacteristicsWithStorageKeyAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAllCryptogramMaterialCharacteristicsAsync: unsafe extern "system" fn(this: *mut *mut Self, promptingbehavior: SmartCardUnlockPromptingBehavior, materialpackagename: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAllCryptogramMaterialCharacteristicsAsync: usize,
}
impl ::windows_sys::core::Interface for ISmartCardCryptogramGenerator2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1897310772, data2: 23917, data3: 19274, data4: [150, 163, 239, 164, 125, 42, 126, 37] };
}
#[repr(C)]
pub struct ISmartCardCryptogramGeneratorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetSmartCardCryptogramGeneratorAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetSmartCardCryptogramGeneratorAsync: usize,
}
impl ::windows_sys::core::Interface for ISmartCardCryptogramGeneratorStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 160643344, data2: 52124, data3: 16405, data4: [150, 125, 82, 52, 243, 176, 41, 0] };
}
#[repr(C)]
pub struct ISmartCardCryptogramGeneratorStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISmartCardCryptogramGeneratorStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 163444197, data2: 46269, data3: 20003, data4: [165, 136, 116, 70, 146, 4, 193, 40] };
}
#[repr(C)]
pub struct ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub OperationStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmartCardCryptogramGeneratorOperationStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Characteristics: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Characteristics: usize,
}
impl ::windows_sys::core::Interface for ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 664330281, data2: 54919, data3: 19602, data4: [134, 198, 57, 158, 154, 14, 203, 9] };
}
#[repr(C)]
pub struct ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub OperationStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmartCardCryptogramGeneratorOperationStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Characteristics: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Characteristics: usize,
}
impl ::windows_sys::core::Interface for ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1315605084, data2: 38771, data3: 18116, data4: [163, 47, 177, 229, 67, 21, 158, 4] };
}
#[repr(C)]
pub struct ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub OperationStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmartCardCryptogramGeneratorOperationStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Characteristics: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Characteristics: usize,
}
impl ::windows_sys::core::Interface for ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2356996183, data2: 42983, data3: 18589, data4: [185, 214, 54, 128, 97, 81, 80, 18] };
}
#[repr(C)]
pub struct ISmartCardCryptogramMaterialCharacteristics {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaterialName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AllowedAlgorithms: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllowedAlgorithms: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub AllowedProofOfPossessionAlgorithms: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllowedProofOfPossessionAlgorithms: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub AllowedValidations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllowedValidations: usize,
    pub MaterialType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmartCardCryptogramMaterialType) -> ::windows_sys::core::HRESULT,
    pub ProtectionMethod: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmartCardCryptogramMaterialProtectionMethod) -> ::windows_sys::core::HRESULT,
    pub ProtectionVersion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MaterialLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISmartCardCryptogramMaterialCharacteristics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4238001612, data2: 49623, data3: 16723, data4: [146, 59, 162, 212, 60, 108, 141, 73] };
}
#[repr(C)]
pub struct ISmartCardCryptogramMaterialPackageCharacteristics {
    pub base__: ::windows_sys::core::IInspectable,
    pub PackageName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub StorageKeyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DateImported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DateImported: usize,
    pub PackageFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmartCardCryptogramMaterialPackageFormat) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISmartCardCryptogramMaterialPackageCharacteristics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4290088479, data2: 1682, data3: 19527, data4: [147, 207, 52, 217, 31, 157, 205, 0] };
}
#[repr(C)]
pub struct ISmartCardCryptogramMaterialPossessionProof {
    pub base__: ::windows_sys::core::IInspectable,
    pub OperationStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmartCardCryptogramGeneratorOperationStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Proof: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Proof: usize,
}
impl ::windows_sys::core::Interface for ISmartCardCryptogramMaterialPossessionProof {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3854150540, data2: 41281, data3: 16693, data4: [154, 221, 176, 210, 227, 170, 31, 201] };
}
#[repr(C)]
pub struct ISmartCardCryptogramPlacementStep {
    pub base__: ::windows_sys::core::IInspectable,
    pub Algorithm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmartCardCryptogramAlgorithm) -> ::windows_sys::core::HRESULT,
    pub SetAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, value: SmartCardCryptogramAlgorithm) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SourceData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SourceData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetSourceData: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetSourceData: usize,
    pub CryptogramMaterialPackageName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCryptogramMaterialPackageName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CryptogramMaterialName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCryptogramMaterialName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TemplateOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetTemplateOffset: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub CryptogramOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetCryptogramOffset: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub CryptogramLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetCryptogramLength: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub CryptogramPlacementOptions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmartCardCryptogramPlacementOptions) -> ::windows_sys::core::HRESULT,
    pub SetCryptogramPlacementOptions: unsafe extern "system" fn(this: *mut *mut Self, value: SmartCardCryptogramPlacementOptions) -> ::windows_sys::core::HRESULT,
    pub ChainedOutputStep: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetChainedOutputStep: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISmartCardCryptogramPlacementStep {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2491089899, data2: 33602, data3: 18322, data4: [162, 229, 146, 86, 54, 55, 138, 83] };
}
#[repr(C)]
pub struct ISmartCardCryptogramStorageKeyCharacteristics {
    pub base__: ::windows_sys::core::IInspectable,
    pub StorageKeyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DateCreated: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DateCreated: usize,
    pub Algorithm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmartCardCryptogramStorageKeyAlgorithm) -> ::windows_sys::core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmartCardCryptogramStorageKeyCapabilities) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISmartCardCryptogramStorageKeyCharacteristics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2236765294, data2: 17495, data3: 18469, data4: [180, 100, 99, 84, 113, 163, 159, 92] };
}
#[repr(C)]
pub struct ISmartCardCryptogramStorageKeyInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub OperationStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmartCardCryptogramGeneratorOperationStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Security_Cryptography_Core")]
    pub PublicKeyBlobType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Security::Cryptography::Core::CryptographicPublicKeyBlobType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Core"))]
    PublicKeyBlobType: usize,
    #[cfg(feature = "Storage_Streams")]
    pub PublicKey: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PublicKey: usize,
    pub AttestationStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmartCardCryptographicKeyAttestationStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Attestation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Attestation: usize,
    #[cfg(feature = "Storage_Streams")]
    pub AttestationCertificateChain: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AttestationCertificateChain: usize,
    pub Capabilities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmartCardCryptogramStorageKeyCapabilities) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISmartCardCryptogramStorageKeyInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2008084493, data2: 45207, data3: 20321, data4: [162, 106, 149, 97, 99, 156, 156, 58] };
}
#[repr(C)]
pub struct ISmartCardCryptogramStorageKeyInfo2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub OperationalRequirements: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISmartCardCryptogramStorageKeyInfo2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 278777, data2: 63485, data3: 16765, data4: [137, 225, 251, 176, 56, 42, 220, 77] };
}
#[repr(C)]
pub struct ISmartCardEmulator {
    pub base__: ::windows_sys::core::IInspectable,
    pub EnablementPolicy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmartCardEmulatorEnablementPolicy) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISmartCardEmulator {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3753445042, data2: 34654, data3: 18405, data4: [128, 119, 232, 191, 241, 177, 198, 251] };
}
#[repr(C)]
pub struct ISmartCardEmulator2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ApduReceived: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ApduReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveApduReceived: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveApduReceived: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectionDeactivated: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionDeactivated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveConnectionDeactivated: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConnectionDeactivated: usize,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub IsHostCardEmulationSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISmartCardEmulator2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4265590968, data2: 34089, data3: 16666, data4: [128, 123, 72, 237, 194, 160, 171, 68] };
}
#[repr(C)]
pub struct ISmartCardEmulatorApduReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub CommandApdu: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CommandApdu: usize,
    pub ConnectionProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub TryRespondAsync: unsafe extern "system" fn(this: *mut *mut Self, responseapdu: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    TryRespondAsync: usize,
    pub AutomaticResponseStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmartCardAutomaticResponseStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISmartCardEmulatorApduReceivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3579647350, data2: 27090, data3: 21299, data4: [91, 95, 248, 192, 214, 233, 240, 159] };
}
#[repr(C)]
pub struct ISmartCardEmulatorApduReceivedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub TryRespondWithStateAsync: unsafe extern "system" fn(this: *mut *mut Self, responseapdu: *mut ::core::ffi::c_void, nextstate: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    TryRespondWithStateAsync: usize,
}
impl ::windows_sys::core::Interface for ISmartCardEmulatorApduReceivedEventArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2348367344, data2: 8929, data3: 16952, data4: [134, 16, 148, 206, 74, 150, 84, 37] };
}
#[repr(C)]
pub struct ISmartCardEmulatorApduReceivedEventArgsWithCryptograms {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub TryRespondWithCryptogramsAsync: unsafe extern "system" fn(this: *mut *mut Self, responsetemplate: *mut ::core::ffi::c_void, cryptogramplacementsteps: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    TryRespondWithCryptogramsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub TryRespondWithCryptogramsAndStateAsync: unsafe extern "system" fn(this: *mut *mut Self, responsetemplate: *mut ::core::ffi::c_void, cryptogramplacementsteps: *mut ::core::ffi::c_void, nextstate: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    TryRespondWithCryptogramsAndStateAsync: usize,
}
impl ::windows_sys::core::Interface for ISmartCardEmulatorApduReceivedEventArgsWithCryptograms {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3578837703, data2: 47039, data3: 20009, data4: [146, 148, 12, 74, 195, 201, 65, 189] };
}
#[repr(C)]
pub struct ISmartCardEmulatorConnectionDeactivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ConnectionProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Reason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmartCardEmulatorConnectionDeactivatedReason) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISmartCardEmulatorConnectionDeactivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 562485459, data2: 50667, data3: 21090, data4: [67, 223, 98, 160, 161, 181, 85, 87] };
}
#[repr(C)]
pub struct ISmartCardEmulatorConnectionProperties {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmartCardEmulatorConnectionSource) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISmartCardEmulatorConnectionProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1311548910, data2: 63849, data3: 20605, data4: [108, 249, 52, 226, 209, 141, 243, 17] };
}
#[repr(C)]
pub struct ISmartCardEmulatorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
}
impl ::windows_sys::core::Interface for ISmartCardEmulatorStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2057043019, data2: 50387, data3: 18767, data4: [184, 162, 98, 21, 216, 30, 133, 178] };
}
#[repr(C)]
pub struct ISmartCardEmulatorStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAppletIdGroupRegistrationsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAppletIdGroupRegistrationsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RegisterAppletIdGroupAsync: unsafe extern "system" fn(this: *mut *mut Self, appletidgroup: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RegisterAppletIdGroupAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UnregisterAppletIdGroupAsync: unsafe extern "system" fn(this: *mut *mut Self, registration: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnregisterAppletIdGroupAsync: usize,
    pub MaxAppletIdGroupRegistrations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISmartCardEmulatorStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1773051786, data2: 46965, data3: 18571, data4: [132, 54, 108, 30, 40, 237, 115, 31] };
}
#[repr(C)]
pub struct ISmartCardEmulatorStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISmartCardEmulatorStatics3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1508512810, data2: 40713, data3: 17397, data4: [133, 101, 207, 168, 20, 142, 76, 178] };
}
#[repr(C)]
pub struct ISmartCardPinPolicy {
    pub base__: ::windows_sys::core::IInspectable,
    pub MinLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetMinLength: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub MaxLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetMaxLength: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub UppercaseLetters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmartCardPinCharacterPolicyOption) -> ::windows_sys::core::HRESULT,
    pub SetUppercaseLetters: unsafe extern "system" fn(this: *mut *mut Self, value: SmartCardPinCharacterPolicyOption) -> ::windows_sys::core::HRESULT,
    pub LowercaseLetters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmartCardPinCharacterPolicyOption) -> ::windows_sys::core::HRESULT,
    pub SetLowercaseLetters: unsafe extern "system" fn(this: *mut *mut Self, value: SmartCardPinCharacterPolicyOption) -> ::windows_sys::core::HRESULT,
    pub Digits: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmartCardPinCharacterPolicyOption) -> ::windows_sys::core::HRESULT,
    pub SetDigits: unsafe extern "system" fn(this: *mut *mut Self, value: SmartCardPinCharacterPolicyOption) -> ::windows_sys::core::HRESULT,
    pub SpecialCharacters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmartCardPinCharacterPolicyOption) -> ::windows_sys::core::HRESULT,
    pub SetSpecialCharacters: unsafe extern "system" fn(this: *mut *mut Self, value: SmartCardPinCharacterPolicyOption) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISmartCardPinPolicy {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 406643076, data2: 19894, data3: 18497, data4: [172, 158, 42, 193, 243, 155, 115, 4] };
}
#[repr(C)]
pub struct ISmartCardPinResetDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISmartCardPinResetDeferral {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 415845036, data2: 30725, data3: 16388, data4: [133, 228, 187, 239, 172, 143, 104, 132] };
}
#[repr(C)]
pub struct ISmartCardPinResetRequest {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub Challenge: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Challenge: usize,
    #[cfg(feature = "Foundation")]
    pub Deadline: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Deadline: usize,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SetResponse: unsafe extern "system" fn(this: *mut *mut Self, response: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetResponse: usize,
}
impl ::windows_sys::core::Interface for ISmartCardPinResetRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 318651469, data2: 24505, data3: 20110, data4: [159, 246, 97, 244, 117, 18, 79, 239] };
}
#[repr(C)]
pub struct ISmartCardProvisioning {
    pub base__: ::windows_sys::core::IInspectable,
    pub SmartCard: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetIdAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetNameAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetNameAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetChallengeContextAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetChallengeContextAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestPinChangeAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestPinChangeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestPinResetAsync: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestPinResetAsync: usize,
}
impl ::windows_sys::core::Interface for ISmartCardProvisioning {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 435088829, data2: 8107, data3: 18300, data4: [183, 18, 26, 44, 90, 241, 253, 110] };
}
#[repr(C)]
pub struct ISmartCardProvisioning2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetAuthorityKeyContainerNameAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAuthorityKeyContainerNameAsync: usize,
}
impl ::windows_sys::core::Interface for ISmartCardProvisioning2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 285026539, data2: 16249, data3: 19302, data4: [155, 124, 17, 193, 73, 183, 208, 188] };
}
#[repr(C)]
pub struct ISmartCardProvisioningStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FromSmartCardAsync: unsafe extern "system" fn(this: *mut *mut Self, card: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromSmartCardAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub RequestVirtualSmartCardCreationAsync: unsafe extern "system" fn(this: *mut *mut Self, friendlyname: ::windows_sys::core::HSTRING, administrativekey: *mut ::core::ffi::c_void, pinpolicy: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    RequestVirtualSmartCardCreationAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub RequestVirtualSmartCardCreationAsyncWithCardId: unsafe extern "system" fn(this: *mut *mut Self, friendlyname: ::windows_sys::core::HSTRING, administrativekey: *mut ::core::ffi::c_void, pinpolicy: *mut ::core::ffi::c_void, cardid: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    RequestVirtualSmartCardCreationAsyncWithCardId: usize,
    #[cfg(feature = "Foundation")]
    pub RequestVirtualSmartCardDeletionAsync: unsafe extern "system" fn(this: *mut *mut Self, card: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestVirtualSmartCardDeletionAsync: usize,
}
impl ::windows_sys::core::Interface for ISmartCardProvisioningStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 327690312, data2: 3347, data3: 20080, data4: [151, 53, 81, 218, 236, 165, 37, 79] };
}
#[repr(C)]
pub struct ISmartCardProvisioningStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub RequestAttestedVirtualSmartCardCreationAsync: unsafe extern "system" fn(this: *mut *mut Self, friendlyname: ::windows_sys::core::HSTRING, administrativekey: *mut ::core::ffi::c_void, pinpolicy: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    RequestAttestedVirtualSmartCardCreationAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub RequestAttestedVirtualSmartCardCreationAsyncWithCardId: unsafe extern "system" fn(this: *mut *mut Self, friendlyname: ::windows_sys::core::HSTRING, administrativekey: *mut ::core::ffi::c_void, pinpolicy: *mut ::core::ffi::c_void, cardid: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    RequestAttestedVirtualSmartCardCreationAsyncWithCardId: usize,
}
impl ::windows_sys::core::Interface for ISmartCardProvisioningStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 877119144, data2: 51616, data3: 19414, data4: [181, 13, 37, 31, 78, 141, 58, 98] };
}
#[repr(C)]
pub struct ISmartCardReader {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmartCardReaderKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetStatusAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetStatusAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllCardsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllCardsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CardAdded: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CardAdded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCardAdded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCardAdded: usize,
    #[cfg(feature = "Foundation")]
    pub CardRemoved: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CardRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCardRemoved: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCardRemoved: usize,
}
impl ::windows_sys::core::Interface for ISmartCardReader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 276083936, data2: 21698, data3: 19952, data4: [129, 122, 20, 193, 67, 120, 240, 108] };
}
#[repr(C)]
pub struct ISmartCardReaderStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDeviceSelectorWithKind: unsafe extern "system" fn(this: *mut *mut Self, kind: SmartCardReaderKind, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
impl ::windows_sys::core::Interface for ISmartCardReaderStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 272368865, data2: 41418, data3: 18674, data4: [162, 129, 91, 111, 102, 154, 241, 7] };
}
#[repr(C)]
pub struct ISmartCardTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub TriggerType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmartCardTriggerType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SourceAppletId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SourceAppletId: usize,
    #[cfg(feature = "Storage_Streams")]
    pub TriggerData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    TriggerData: usize,
}
impl ::windows_sys::core::Interface for ISmartCardTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1604055326, data2: 14831, data3: 20267, data4: [180, 79, 10, 145, 85, 177, 119, 188] };
}
#[repr(C)]
pub struct ISmartCardTriggerDetails2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Emulator: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryLaunchCurrentAppAsync: unsafe extern "system" fn(this: *mut *mut Self, arguments: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryLaunchCurrentAppAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryLaunchCurrentAppWithBehaviorAsync: unsafe extern "system" fn(this: *mut *mut Self, arguments: ::windows_sys::core::HSTRING, behavior: SmartCardLaunchBehavior, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryLaunchCurrentAppWithBehaviorAsync: usize,
}
impl ::windows_sys::core::Interface for ISmartCardTriggerDetails2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 692438377, data2: 35189, data3: 19025, data4: [158, 26, 95, 138, 118, 238, 81, 175] };
}
#[repr(C)]
pub struct ISmartCardTriggerDetails3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SmartCard: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISmartCardTriggerDetails3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3017982589, data2: 6342, data3: 19368, data4: [131, 118, 239, 3, 212, 145, 38, 102] };
}
pub type SmartCard = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardActivationPolicyChangeResult(pub i32);
impl SmartCardActivationPolicyChangeResult {
    pub const Denied: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
}
impl ::core::marker::Copy for SmartCardActivationPolicyChangeResult {}
impl ::core::clone::Clone for SmartCardActivationPolicyChangeResult {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SmartCardAppletIdGroup = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardAppletIdGroupActivationPolicy(pub i32);
impl SmartCardAppletIdGroupActivationPolicy {
    pub const Disabled: Self = Self(0i32);
    pub const ForegroundOverride: Self = Self(1i32);
    pub const Enabled: Self = Self(2i32);
}
impl ::core::marker::Copy for SmartCardAppletIdGroupActivationPolicy {}
impl ::core::clone::Clone for SmartCardAppletIdGroupActivationPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SmartCardAppletIdGroupRegistration = *mut ::core::ffi::c_void;
pub type SmartCardAutomaticResponseApdu = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardAutomaticResponseStatus(pub i32);
impl SmartCardAutomaticResponseStatus {
    pub const None: Self = Self(0i32);
    pub const Success: Self = Self(1i32);
    pub const UnknownError: Self = Self(2i32);
}
impl ::core::marker::Copy for SmartCardAutomaticResponseStatus {}
impl ::core::clone::Clone for SmartCardAutomaticResponseStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SmartCardChallengeContext = *mut ::core::ffi::c_void;
pub type SmartCardConnection = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardCryptogramAlgorithm(pub i32);
impl SmartCardCryptogramAlgorithm {
    pub const None: Self = Self(0i32);
    pub const CbcMac: Self = Self(1i32);
    pub const Cvc3Umd: Self = Self(2i32);
    pub const DecimalizedMsd: Self = Self(3i32);
    pub const Cvc3MD: Self = Self(4i32);
    pub const Sha1: Self = Self(5i32);
    pub const SignedDynamicApplicationData: Self = Self(6i32);
    pub const RsaPkcs1: Self = Self(7i32);
    pub const Sha256Hmac: Self = Self(8i32);
}
impl ::core::marker::Copy for SmartCardCryptogramAlgorithm {}
impl ::core::clone::Clone for SmartCardCryptogramAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SmartCardCryptogramGenerator = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardCryptogramGeneratorOperationStatus(pub i32);
impl SmartCardCryptogramGeneratorOperationStatus {
    pub const Success: Self = Self(0i32);
    pub const AuthorizationFailed: Self = Self(1i32);
    pub const AuthorizationCanceled: Self = Self(2i32);
    pub const AuthorizationRequired: Self = Self(3i32);
    pub const CryptogramMaterialPackageStorageKeyExists: Self = Self(4i32);
    pub const NoCryptogramMaterialPackageStorageKey: Self = Self(5i32);
    pub const NoCryptogramMaterialPackage: Self = Self(6i32);
    pub const UnsupportedCryptogramMaterialPackage: Self = Self(7i32);
    pub const UnknownCryptogramMaterialName: Self = Self(8i32);
    pub const InvalidCryptogramMaterialUsage: Self = Self(9i32);
    pub const ApduResponseNotSent: Self = Self(10i32);
    pub const OtherError: Self = Self(11i32);
    pub const ValidationFailed: Self = Self(12i32);
    pub const NotSupported: Self = Self(13i32);
}
impl ::core::marker::Copy for SmartCardCryptogramGeneratorOperationStatus {}
impl ::core::clone::Clone for SmartCardCryptogramGeneratorOperationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult = *mut ::core::ffi::c_void;
pub type SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult = *mut ::core::ffi::c_void;
pub type SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult = *mut ::core::ffi::c_void;
pub type SmartCardCryptogramMaterialCharacteristics = *mut ::core::ffi::c_void;
pub type SmartCardCryptogramMaterialPackageCharacteristics = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardCryptogramMaterialPackageConfirmationResponseFormat(pub i32);
impl SmartCardCryptogramMaterialPackageConfirmationResponseFormat {
    pub const None: Self = Self(0i32);
    pub const VisaHmac: Self = Self(1i32);
}
impl ::core::marker::Copy for SmartCardCryptogramMaterialPackageConfirmationResponseFormat {}
impl ::core::clone::Clone for SmartCardCryptogramMaterialPackageConfirmationResponseFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardCryptogramMaterialPackageFormat(pub i32);
impl SmartCardCryptogramMaterialPackageFormat {
    pub const None: Self = Self(0i32);
    pub const JweRsaPki: Self = Self(1i32);
}
impl ::core::marker::Copy for SmartCardCryptogramMaterialPackageFormat {}
impl ::core::clone::Clone for SmartCardCryptogramMaterialPackageFormat {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SmartCardCryptogramMaterialPossessionProof = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardCryptogramMaterialProtectionMethod(pub i32);
impl SmartCardCryptogramMaterialProtectionMethod {
    pub const None: Self = Self(0i32);
    pub const WhiteBoxing: Self = Self(1i32);
}
impl ::core::marker::Copy for SmartCardCryptogramMaterialProtectionMethod {}
impl ::core::clone::Clone for SmartCardCryptogramMaterialProtectionMethod {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardCryptogramMaterialType(pub i32);
impl SmartCardCryptogramMaterialType {
    pub const None: Self = Self(0i32);
    pub const StaticDataAuthentication: Self = Self(1i32);
    pub const TripleDes112: Self = Self(2i32);
    pub const Aes: Self = Self(3i32);
    pub const RsaPkcs1: Self = Self(4i32);
}
impl ::core::marker::Copy for SmartCardCryptogramMaterialType {}
impl ::core::clone::Clone for SmartCardCryptogramMaterialType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardCryptogramPlacementOptions(pub u32);
impl SmartCardCryptogramPlacementOptions {
    pub const None: Self = Self(0u32);
    pub const UnitsAreInNibbles: Self = Self(1u32);
    pub const ChainOutput: Self = Self(2u32);
}
impl ::core::marker::Copy for SmartCardCryptogramPlacementOptions {}
impl ::core::clone::Clone for SmartCardCryptogramPlacementOptions {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SmartCardCryptogramPlacementStep = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardCryptogramStorageKeyAlgorithm(pub i32);
impl SmartCardCryptogramStorageKeyAlgorithm {
    pub const None: Self = Self(0i32);
    pub const Rsa2048: Self = Self(1i32);
}
impl ::core::marker::Copy for SmartCardCryptogramStorageKeyAlgorithm {}
impl ::core::clone::Clone for SmartCardCryptogramStorageKeyAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardCryptogramStorageKeyCapabilities(pub u32);
impl SmartCardCryptogramStorageKeyCapabilities {
    pub const None: Self = Self(0u32);
    pub const HardwareProtection: Self = Self(1u32);
    pub const UnlockPrompt: Self = Self(2u32);
}
impl ::core::marker::Copy for SmartCardCryptogramStorageKeyCapabilities {}
impl ::core::clone::Clone for SmartCardCryptogramStorageKeyCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SmartCardCryptogramStorageKeyCharacteristics = *mut ::core::ffi::c_void;
pub type SmartCardCryptogramStorageKeyInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardCryptographicKeyAttestationStatus(pub i32);
impl SmartCardCryptographicKeyAttestationStatus {
    pub const NoAttestation: Self = Self(0i32);
    pub const SoftwareKeyWithoutTpm: Self = Self(1i32);
    pub const SoftwareKeyWithTpm: Self = Self(2i32);
    pub const TpmKeyUnknownAttestationStatus: Self = Self(3i32);
    pub const TpmKeyWithoutAttestationCapability: Self = Self(4i32);
    pub const TpmKeyWithTemporaryAttestationFailure: Self = Self(5i32);
    pub const TpmKeyWithLongTermAttestationFailure: Self = Self(6i32);
    pub const TpmKeyWithAttestation: Self = Self(7i32);
}
impl ::core::marker::Copy for SmartCardCryptographicKeyAttestationStatus {}
impl ::core::clone::Clone for SmartCardCryptographicKeyAttestationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardEmulationCategory(pub i32);
impl SmartCardEmulationCategory {
    pub const Other: Self = Self(0i32);
    pub const Payment: Self = Self(1i32);
}
impl ::core::marker::Copy for SmartCardEmulationCategory {}
impl ::core::clone::Clone for SmartCardEmulationCategory {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardEmulationType(pub i32);
impl SmartCardEmulationType {
    pub const Host: Self = Self(0i32);
    pub const Uicc: Self = Self(1i32);
    pub const EmbeddedSE: Self = Self(2i32);
}
impl ::core::marker::Copy for SmartCardEmulationType {}
impl ::core::clone::Clone for SmartCardEmulationType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SmartCardEmulator = *mut ::core::ffi::c_void;
pub type SmartCardEmulatorApduReceivedEventArgs = *mut ::core::ffi::c_void;
pub type SmartCardEmulatorConnectionDeactivatedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardEmulatorConnectionDeactivatedReason(pub i32);
impl SmartCardEmulatorConnectionDeactivatedReason {
    pub const ConnectionLost: Self = Self(0i32);
    pub const ConnectionRedirected: Self = Self(1i32);
}
impl ::core::marker::Copy for SmartCardEmulatorConnectionDeactivatedReason {}
impl ::core::clone::Clone for SmartCardEmulatorConnectionDeactivatedReason {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SmartCardEmulatorConnectionProperties = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardEmulatorConnectionSource(pub i32);
impl SmartCardEmulatorConnectionSource {
    pub const Unknown: Self = Self(0i32);
    pub const NfcReader: Self = Self(1i32);
}
impl ::core::marker::Copy for SmartCardEmulatorConnectionSource {}
impl ::core::clone::Clone for SmartCardEmulatorConnectionSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardEmulatorEnablementPolicy(pub i32);
impl SmartCardEmulatorEnablementPolicy {
    pub const Never: Self = Self(0i32);
    pub const Always: Self = Self(1i32);
    pub const ScreenOn: Self = Self(2i32);
    pub const ScreenUnlocked: Self = Self(3i32);
}
impl ::core::marker::Copy for SmartCardEmulatorEnablementPolicy {}
impl ::core::clone::Clone for SmartCardEmulatorEnablementPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardLaunchBehavior(pub i32);
impl SmartCardLaunchBehavior {
    pub const Default: Self = Self(0i32);
    pub const AboveLock: Self = Self(1i32);
}
impl ::core::marker::Copy for SmartCardLaunchBehavior {}
impl ::core::clone::Clone for SmartCardLaunchBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardPinCharacterPolicyOption(pub i32);
impl SmartCardPinCharacterPolicyOption {
    pub const Allow: Self = Self(0i32);
    pub const RequireAtLeastOne: Self = Self(1i32);
    pub const Disallow: Self = Self(2i32);
}
impl ::core::marker::Copy for SmartCardPinCharacterPolicyOption {}
impl ::core::clone::Clone for SmartCardPinCharacterPolicyOption {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SmartCardPinPolicy = *mut ::core::ffi::c_void;
pub type SmartCardPinResetDeferral = *mut ::core::ffi::c_void;
pub type SmartCardPinResetHandler = *mut ::core::ffi::c_void;
pub type SmartCardPinResetRequest = *mut ::core::ffi::c_void;
pub type SmartCardProvisioning = *mut ::core::ffi::c_void;
pub type SmartCardReader = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardReaderKind(pub i32);
impl SmartCardReaderKind {
    pub const Any: Self = Self(0i32);
    pub const Generic: Self = Self(1i32);
    pub const Tpm: Self = Self(2i32);
    pub const Nfc: Self = Self(3i32);
    pub const Uicc: Self = Self(4i32);
    pub const EmbeddedSE: Self = Self(5i32);
}
impl ::core::marker::Copy for SmartCardReaderKind {}
impl ::core::clone::Clone for SmartCardReaderKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardReaderStatus(pub i32);
impl SmartCardReaderStatus {
    pub const Disconnected: Self = Self(0i32);
    pub const Ready: Self = Self(1i32);
    pub const Exclusive: Self = Self(2i32);
}
impl ::core::marker::Copy for SmartCardReaderStatus {}
impl ::core::clone::Clone for SmartCardReaderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardStatus(pub i32);
impl SmartCardStatus {
    pub const Disconnected: Self = Self(0i32);
    pub const Ready: Self = Self(1i32);
    pub const Shared: Self = Self(2i32);
    pub const Exclusive: Self = Self(3i32);
    pub const Unresponsive: Self = Self(4i32);
}
impl ::core::marker::Copy for SmartCardStatus {}
impl ::core::clone::Clone for SmartCardStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SmartCardTriggerDetails = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardTriggerType(pub i32);
impl SmartCardTriggerType {
    pub const EmulatorTransaction: Self = Self(0i32);
    pub const EmulatorNearFieldEntry: Self = Self(1i32);
    pub const EmulatorNearFieldExit: Self = Self(2i32);
    pub const EmulatorHostApplicationActivated: Self = Self(3i32);
    pub const EmulatorAppletIdGroupRegistrationChanged: Self = Self(4i32);
    pub const ReaderCardAdded: Self = Self(5i32);
}
impl ::core::marker::Copy for SmartCardTriggerType {}
impl ::core::clone::Clone for SmartCardTriggerType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardUnlockPromptingBehavior(pub i32);
impl SmartCardUnlockPromptingBehavior {
    pub const AllowUnlockPrompt: Self = Self(0i32);
    pub const RequireUnlockPrompt: Self = Self(1i32);
    pub const PreventUnlockPrompt: Self = Self(2i32);
}
impl ::core::marker::Copy for SmartCardUnlockPromptingBehavior {}
impl ::core::clone::Clone for SmartCardUnlockPromptingBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
