#[repr(C)]
pub struct IItemEnumerator {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Current: unsafe extern "system" fn(this: *mut *mut Self, item: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Current: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut *mut Self, itemvalid: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISettingsContext {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Serialize: unsafe extern "system" fn(this: *mut *mut Self, pstream: *mut ::core::ffi::c_void, ptarget: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Serialize: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Deserialize: unsafe extern "system" fn(this: *mut *mut Self, pstream: *mut ::core::ffi::c_void, ptarget: *mut ::core::ffi::c_void, pppresults: *mut *mut *mut ::core::ffi::c_void, pcresultcount: *mut usize) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Deserialize: usize,
    pub SetUserData: unsafe extern "system" fn(this: *mut *mut Self, puserdata: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetUserData: unsafe extern "system" fn(this: *mut *mut Self, puserdata: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetNamespaces: unsafe extern "system" fn(this: *mut *mut Self, ppnamespaceids: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStoredSettings: unsafe extern "system" fn(this: *mut *mut Self, pidentity: *mut ::core::ffi::c_void, ppaddedsettings: *mut *mut ::core::ffi::c_void, ppmodifiedsettings: *mut *mut ::core::ffi::c_void, ppdeletedsettings: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RevertSetting: unsafe extern "system" fn(this: *mut *mut Self, pidentity: *mut ::core::ffi::c_void, pwzsetting: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISettingsEngine {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetNamespaces: unsafe extern "system" fn(this: *mut *mut Self, flags: WcmNamespaceEnumerationFlags, reserved: *const ::core::ffi::c_void, namespaces: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetNamespace: unsafe extern "system" fn(this: *mut *mut Self, settingsid: *mut ::core::ffi::c_void, access: WcmNamespaceAccess, reserved: *const ::core::ffi::c_void, namespaceitem: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetErrorDescription: unsafe extern "system" fn(this: *mut *mut Self, hresult: i32, message: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetErrorDescription: usize,
    pub CreateSettingsIdentity: unsafe extern "system" fn(this: *mut *mut Self, settingsid: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStoreStatus: unsafe extern "system" fn(this: *mut *mut Self, reserved: *const ::core::ffi::c_void, status: *mut WcmUserStatus) -> ::windows_sys::core::HRESULT,
    pub LoadStore: unsafe extern "system" fn(this: *mut *mut Self, flags: u32) -> ::windows_sys::core::HRESULT,
    pub UnloadStore: unsafe extern "system" fn(this: *mut *mut Self, reserved: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RegisterNamespace: unsafe extern "system" fn(this: *mut *mut Self, settingsid: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, pushsettings: super::super::Foundation::BOOL, results: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RegisterNamespace: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UnregisterNamespace: unsafe extern "system" fn(this: *mut *mut Self, settingsid: *mut ::core::ffi::c_void, removesettings: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UnregisterNamespace: usize,
    pub CreateTargetInfo: unsafe extern "system" fn(this: *mut *mut Self, target: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetTargetInfo: unsafe extern "system" fn(this: *mut *mut Self, target: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTargetInfo: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSettingsContext: unsafe extern "system" fn(this: *mut *mut Self, flags: u32, reserved: *const ::core::ffi::c_void, settingscontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSettingsContext: unsafe extern "system" fn(this: *mut *mut Self, settingscontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ApplySettingsContext: unsafe extern "system" fn(this: *mut *mut Self, settingscontext: *mut ::core::ffi::c_void, pppwzidentities: *mut *mut ::windows_sys::core::PWSTR, pcidentities: *mut usize) -> ::windows_sys::core::HRESULT,
    pub GetSettingsContext: unsafe extern "system" fn(this: *mut *mut Self, settingscontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISettingsIdentity {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAttribute: unsafe extern "system" fn(this: *mut *mut Self, reserved: *const ::core::ffi::c_void, name: ::windows_sys::core::PCWSTR, value: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAttribute: usize,
    pub SetAttribute: unsafe extern "system" fn(this: *mut *mut Self, reserved: *const ::core::ffi::c_void, name: ::windows_sys::core::PCWSTR, value: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(this: *mut *mut Self, flags: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(this: *mut *mut Self, flags: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISettingsItem {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetName: unsafe extern "system" fn(this: *mut *mut Self, name: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetValue: usize,
    pub GetSettingType: unsafe extern "system" fn(this: *mut *mut Self, r#type: *mut WcmSettingType) -> ::windows_sys::core::HRESULT,
    pub GetDataType: unsafe extern "system" fn(this: *mut *mut Self, r#type: *mut WcmDataType) -> ::windows_sys::core::HRESULT,
    pub GetValueRaw: unsafe extern "system" fn(this: *mut *mut Self, data: *mut *mut u8, datasize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetValueRaw: unsafe extern "system" fn(this: *mut *mut Self, datatype: i32, data: *const u8, datasize: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub HasChild: unsafe extern "system" fn(this: *mut *mut Self, itemhaschild: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasChild: usize,
    pub Children: unsafe extern "system" fn(this: *mut *mut Self, children: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetChild: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PCWSTR, child: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetSettingByPath: unsafe extern "system" fn(this: *mut *mut Self, path: ::windows_sys::core::PCWSTR, setting: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSettingByPath: unsafe extern "system" fn(this: *mut *mut Self, path: ::windows_sys::core::PCWSTR, setting: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveSettingByPath: unsafe extern "system" fn(this: *mut *mut Self, path: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetListKeyInformation: unsafe extern "system" fn(this: *mut *mut Self, keyname: *mut super::super::Foundation::BSTR, datatype: *mut WcmDataType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetListKeyInformation: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateListElement: unsafe extern "system" fn(this: *mut *mut Self, keydata: *const super::Com::VARIANT, child: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateListElement: usize,
    pub RemoveListElement: unsafe extern "system" fn(this: *mut *mut Self, elementname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub Attributes: unsafe extern "system" fn(this: *mut *mut Self, attributes: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetAttribute: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PCWSTR, value: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetAttribute: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPath: unsafe extern "system" fn(this: *mut *mut Self, path: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPath: usize,
    pub GetRestrictionFacets: unsafe extern "system" fn(this: *mut *mut Self, restrictionfacets: *mut WcmRestrictionFacets) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetRestriction: unsafe extern "system" fn(this: *mut *mut Self, restrictionfacet: WcmRestrictionFacets, facetdata: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetRestriction: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetKeyValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetKeyValue: usize,
}
#[repr(C)]
pub struct ISettingsNamespace {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetIdentity: unsafe extern "system" fn(this: *mut *mut Self, settingsid: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Settings: unsafe extern "system" fn(this: *mut *mut Self, settings: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Save: unsafe extern "system" fn(this: *mut *mut Self, pushsettings: super::super::Foundation::BOOL, result: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Save: usize,
    pub GetSettingByPath: unsafe extern "system" fn(this: *mut *mut Self, path: ::windows_sys::core::PCWSTR, setting: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSettingByPath: unsafe extern "system" fn(this: *mut *mut Self, path: ::windows_sys::core::PCWSTR, setting: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveSettingByPath: unsafe extern "system" fn(this: *mut *mut Self, path: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetAttribute: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PCWSTR, value: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetAttribute: usize,
}
#[repr(C)]
pub struct ISettingsResult {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDescription: unsafe extern "system" fn(this: *mut *mut Self, description: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDescription: usize,
    pub GetErrorCode: unsafe extern "system" fn(this: *mut *mut Self, hrout: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetContextDescription: unsafe extern "system" fn(this: *mut *mut Self, description: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetContextDescription: usize,
    pub GetLine: unsafe extern "system" fn(this: *mut *mut Self, dwline: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetColumn: unsafe extern "system" fn(this: *mut *mut Self, dwcolumn: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSource: unsafe extern "system" fn(this: *mut *mut Self, file: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSource: usize,
}
#[repr(C)]
pub struct ITargetInfo {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetTargetMode: unsafe extern "system" fn(this: *mut *mut Self, targetmode: *mut WcmTargetMode) -> ::windows_sys::core::HRESULT,
    pub SetTargetMode: unsafe extern "system" fn(this: *mut *mut Self, targetmode: WcmTargetMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTemporaryStoreLocation: unsafe extern "system" fn(this: *mut *mut Self, temporarystorelocation: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTemporaryStoreLocation: usize,
    pub SetTemporaryStoreLocation: unsafe extern "system" fn(this: *mut *mut Self, temporarystorelocation: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTargetID: unsafe extern "system" fn(this: *mut *mut Self, targetid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTargetID: usize,
    pub SetTargetID: unsafe extern "system" fn(this: *mut *mut Self, targetid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTargetProcessorArchitecture: unsafe extern "system" fn(this: *mut *mut Self, processorarchitecture: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTargetProcessorArchitecture: usize,
    pub SetTargetProcessorArchitecture: unsafe extern "system" fn(this: *mut *mut Self, processorarchitecture: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetProperty: unsafe extern "system" fn(this: *mut *mut Self, offline: super::super::Foundation::BOOL, property: ::windows_sys::core::PCWSTR, value: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetProperty: unsafe extern "system" fn(this: *mut *mut Self, offline: super::super::Foundation::BOOL, property: ::windows_sys::core::PCWSTR, value: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetProperty: usize,
    pub GetEnumerator: unsafe extern "system" fn(this: *mut *mut Self, enumerator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ExpandTarget: unsafe extern "system" fn(this: *mut *mut Self, offline: super::super::Foundation::BOOL, location: ::windows_sys::core::PCWSTR, expandedlocation: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExpandTarget: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExpandTargetPath: unsafe extern "system" fn(this: *mut *mut Self, offline: super::super::Foundation::BOOL, location: ::windows_sys::core::PCWSTR, expandedlocation: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExpandTargetPath: usize,
    pub SetModulePath: unsafe extern "system" fn(this: *mut *mut Self, module: ::windows_sys::core::PCWSTR, path: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LoadModule: unsafe extern "system" fn(this: *mut *mut Self, module: ::windows_sys::core::PCWSTR, modulehandle: *mut super::super::Foundation::HINSTANCE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LoadModule: usize,
    pub SetWow64Context: unsafe extern "system" fn(this: *mut *mut Self, installermodule: ::windows_sys::core::PCWSTR, wow64context: *const u8) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TranslateWow64: unsafe extern "system" fn(this: *mut *mut Self, clientarchitecture: ::windows_sys::core::PCWSTR, value: ::windows_sys::core::PCWSTR, translatedvalue: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TranslateWow64: usize,
    pub SetSchemaHiveLocation: unsafe extern "system" fn(this: *mut *mut Self, pwzhivedir: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSchemaHiveLocation: unsafe extern "system" fn(this: *mut *mut Self, phivelocation: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSchemaHiveLocation: usize,
    pub SetSchemaHiveMountName: unsafe extern "system" fn(this: *mut *mut Self, pwzmountname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSchemaHiveMountName: unsafe extern "system" fn(this: *mut *mut Self, pmountname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSchemaHiveMountName: usize,
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const LIMITED_VALIDATION_MODE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const LINK_STORE_TO_ENGINE_INSTANCE: u32 = 1u32;
pub const SettingsEngine: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2675801013, data2: 8371, data3: 4570, data4: [129, 165, 0, 48, 241, 100, 46, 60] };
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_ABORTOPERATION: ::windows_sys::core::HRESULT = -2145255384i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_ASSERTIONFAILED: ::windows_sys::core::HRESULT = -2145255398i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_ATTRIBUTENOTALLOWED: ::windows_sys::core::HRESULT = -2145255420i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_ATTRIBUTENOTFOUND: ::windows_sys::core::HRESULT = -2145255421i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_CONFLICTINGASSERTION: ::windows_sys::core::HRESULT = -2145255399i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_CYCLICREFERENCE: ::windows_sys::core::HRESULT = -2145255389i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_DUPLICATENAME: ::windows_sys::core::HRESULT = -2145255397i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_EXPRESSIONNOTFOUND: ::windows_sys::core::HRESULT = -2145255408i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_HANDLERNOTFOUND: ::windows_sys::core::HRESULT = -2145255394i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INTERNALERROR: ::windows_sys::core::HRESULT = -2145255424i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDATTRIBUTECOMBINATION: ::windows_sys::core::HRESULT = -2145255385i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDDATATYPE: ::windows_sys::core::HRESULT = -2145255416i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDEXPRESSIONSYNTAX: ::windows_sys::core::HRESULT = -2145255401i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDHANDLERSYNTAX: ::windows_sys::core::HRESULT = -2145255393i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDKEY: ::windows_sys::core::HRESULT = -2145255396i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDLANGUAGEFORMAT: ::windows_sys::core::HRESULT = -2145255410i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDPATH: ::windows_sys::core::HRESULT = -2145255413i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDPROCESSORFORMAT: ::windows_sys::core::HRESULT = -2145255382i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDSTREAM: ::windows_sys::core::HRESULT = -2145255395i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDVALUE: ::windows_sys::core::HRESULT = -2145255419i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDVALUEFORMAT: ::windows_sys::core::HRESULT = -2145255418i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDVERSIONFORMAT: ::windows_sys::core::HRESULT = -2145255411i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_KEYNOTCHANGEABLE: ::windows_sys::core::HRESULT = -2145255409i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_MANIFESTCOMPILATIONFAILED: ::windows_sys::core::HRESULT = -2145255390i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_MISSINGCONFIGURATION: ::windows_sys::core::HRESULT = -2145255383i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_MIXTYPEASSERTION: ::windows_sys::core::HRESULT = -2145255388i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_NAMESPACEALREADYREGISTERED: ::windows_sys::core::HRESULT = -2145255403i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_NAMESPACENOTFOUND: ::windows_sys::core::HRESULT = -2145255404i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_NOTIFICATIONNOTFOUND: ::windows_sys::core::HRESULT = -2145255400i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_NOTPOSITIONED: ::windows_sys::core::HRESULT = -2145255415i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_NOTSUPPORTEDFUNCTION: ::windows_sys::core::HRESULT = -2145255387i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_READONLYITEM: ::windows_sys::core::HRESULT = -2145255414i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_RESTRICTIONFAILED: ::windows_sys::core::HRESULT = -2145255391i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_SOURCEMANEMPTYVALUE: ::windows_sys::core::HRESULT = -2145255381i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_STATENODENOTALLOWED: ::windows_sys::core::HRESULT = -2145255422i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_STATENODENOTFOUND: ::windows_sys::core::HRESULT = -2145255423i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_STORECORRUPTED: ::windows_sys::core::HRESULT = -2145255402i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_SUBSTITUTIONNOTFOUND: ::windows_sys::core::HRESULT = -2145255407i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_TYPENOTSPECIFIED: ::windows_sys::core::HRESULT = -2145255417i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_UNKNOWNRESULT: ::windows_sys::core::HRESULT = -2145251325i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_USERALREADYREGISTERED: ::windows_sys::core::HRESULT = -2145255406i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_USERNOTFOUND: ::windows_sys::core::HRESULT = -2145255405i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_VALIDATIONFAILED: ::windows_sys::core::HRESULT = -2145255392i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_VALUETOOBIG: ::windows_sys::core::HRESULT = -2145255386i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_WRONGESCAPESTRING: ::windows_sys::core::HRESULT = -2145255412i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_SETTINGS_ID_ARCHITECTURE: &str = "architecture";
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_SETTINGS_ID_FLAG_DEFINITION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_SETTINGS_ID_FLAG_REFERENCE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_SETTINGS_ID_LANGUAGE: &str = "language";
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_SETTINGS_ID_NAME: &str = "name";
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_SETTINGS_ID_TOKEN: &str = "token";
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_SETTINGS_ID_URI: &str = "uri";
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_SETTINGS_ID_VERSION: &str = "version";
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_SETTINGS_ID_VERSION_SCOPE: &str = "versionScope";
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_S_ATTRIBUTENOTALLOWED: ::windows_sys::core::HRESULT = 2232325i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_S_ATTRIBUTENOTFOUND: ::windows_sys::core::HRESULT = 2232321i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_S_INTERNALERROR: ::windows_sys::core::HRESULT = 2232320i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_S_INVALIDATTRIBUTECOMBINATION: ::windows_sys::core::HRESULT = 2232324i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_S_LEGACYSETTINGWARNING: ::windows_sys::core::HRESULT = 2232322i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_S_NAMESPACENOTFOUND: ::windows_sys::core::HRESULT = 2232326i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub type WcmDataType = i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeByte: WcmDataType = 1i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeSByte: WcmDataType = 2i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeUInt16: WcmDataType = 3i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeInt16: WcmDataType = 4i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeUInt32: WcmDataType = 5i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeInt32: WcmDataType = 6i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeUInt64: WcmDataType = 7i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeInt64: WcmDataType = 8i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeBoolean: WcmDataType = 11i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeString: WcmDataType = 12i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeFlagArray: WcmDataType = 32768i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub type WcmNamespaceAccess = i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const ReadOnlyAccess: WcmNamespaceAccess = 1i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const ReadWriteAccess: WcmNamespaceAccess = 2i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub type WcmNamespaceEnumerationFlags = i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const SharedEnumeration: WcmNamespaceEnumerationFlags = 1i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const UserEnumeration: WcmNamespaceEnumerationFlags = 2i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const AllEnumeration: WcmNamespaceEnumerationFlags = 3i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub type WcmRestrictionFacets = i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const restrictionFacetMaxLength: WcmRestrictionFacets = 1i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const restrictionFacetEnumeration: WcmRestrictionFacets = 2i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const restrictionFacetMaxInclusive: WcmRestrictionFacets = 4i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const restrictionFacetMinInclusive: WcmRestrictionFacets = 8i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub type WcmSettingType = i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const settingTypeScalar: WcmSettingType = 1i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const settingTypeComplex: WcmSettingType = 2i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const settingTypeList: WcmSettingType = 3i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub type WcmTargetMode = i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const OfflineMode: WcmTargetMode = 1i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const OnlineMode: WcmTargetMode = 2i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub type WcmUserStatus = i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const UnknownStatus: WcmUserStatus = 0i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const UserRegistered: WcmUserStatus = 1i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const UserUnregistered: WcmUserStatus = 2i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const UserLoaded: WcmUserStatus = 3i32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const UserUnloaded: WcmUserStatus = 4i32;
