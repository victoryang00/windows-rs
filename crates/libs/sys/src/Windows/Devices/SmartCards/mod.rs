pub type CardAddedEventArgs = *mut ::core::ffi::c_void;
pub type CardRemovedEventArgs = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct ICardAddedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub SmartCard: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICardRemovedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub SmartCard: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct ISmartCardAppletIdGroupFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, displayname: ::windows_sys::core::HSTRING, appletids: *mut ::core::ffi::c_void, emulationcategory: SmartCardEmulationCategory, emulationtype: SmartCardEmulationType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    Create: usize,
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
#[repr(C)]
pub struct ISmartCardAppletIdGroupRegistration2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SmartCardReaderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SetPropertiesAsync: unsafe extern "system" fn(this: *mut *mut Self, props: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetPropertiesAsync: usize,
}
#[repr(C)]
pub struct ISmartCardAppletIdGroupStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxAppletIds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct ISmartCardAutomaticResponseApdu3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AllowWhenCryptogramGeneratorNotPrepared: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowWhenCryptogramGeneratorNotPrepared: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISmartCardAutomaticResponseApduFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, commandapdu: *mut ::core::ffi::c_void, responseapdu: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Create: usize,
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
#[repr(C)]
pub struct ISmartCardConnect {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ConnectAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectAsync: usize,
}
#[repr(C)]
pub struct ISmartCardConnection {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub TransmitAsync: unsafe extern "system" fn(this: *mut *mut Self, command: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    TransmitAsync: usize,
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
#[repr(C)]
pub struct ISmartCardCryptogramGeneratorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetSmartCardCryptogramGeneratorAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetSmartCardCryptogramGeneratorAsync: usize,
}
#[repr(C)]
pub struct ISmartCardCryptogramGeneratorStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub OperationStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmartCardCryptogramGeneratorOperationStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Characteristics: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Characteristics: usize,
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
#[repr(C)]
pub struct ISmartCardCryptogramMaterialPossessionProof {
    pub base__: ::windows_sys::core::IInspectable,
    pub OperationStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmartCardCryptogramGeneratorOperationStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Proof: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Proof: usize,
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
#[repr(C)]
pub struct ISmartCardCryptogramStorageKeyInfo2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub OperationalRequirements: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISmartCardEmulator {
    pub base__: ::windows_sys::core::IInspectable,
    pub EnablementPolicy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmartCardEmulatorEnablementPolicy) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct ISmartCardEmulatorApduReceivedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub TryRespondWithStateAsync: unsafe extern "system" fn(this: *mut *mut Self, responseapdu: *mut ::core::ffi::c_void, nextstate: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    TryRespondWithStateAsync: usize,
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
#[repr(C)]
pub struct ISmartCardEmulatorConnectionDeactivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ConnectionProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Reason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmartCardEmulatorConnectionDeactivatedReason) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISmartCardEmulatorConnectionProperties {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmartCardEmulatorConnectionSource) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISmartCardEmulatorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
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
#[repr(C)]
pub struct ISmartCardEmulatorStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct ISmartCardPinResetDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct ISmartCardProvisioning2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetAuthorityKeyContainerNameAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAuthorityKeyContainerNameAsync: usize,
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
#[repr(C)]
pub struct ISmartCardTriggerDetails3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SmartCard: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
