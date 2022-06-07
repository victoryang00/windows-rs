pub type HostMessageReceivedCallback = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IIsolatedWindowsEnvironment {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartProcessSilentlyAsync: unsafe extern "system" fn(this: *mut *mut Self, hostexepath: ::windows_sys::core::HSTRING, arguments: ::windows_sys::core::HSTRING, activator: IsolatedWindowsEnvironmentActivator, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartProcessSilentlyAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StartProcessSilentlyWithTelemetryAsync: unsafe extern "system" fn(this: *mut *mut Self, hostexepath: ::windows_sys::core::HSTRING, arguments: ::windows_sys::core::HSTRING, activator: IsolatedWindowsEnvironmentActivator, telemetryparameters: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartProcessSilentlyWithTelemetryAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShareFolderAsync: unsafe extern "system" fn(this: *mut *mut Self, hostfolder: ::windows_sys::core::HSTRING, requestoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShareFolderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShareFolderWithTelemetryAsync: unsafe extern "system" fn(this: *mut *mut Self, hostfolder: ::windows_sys::core::HSTRING, requestoptions: *mut ::core::ffi::c_void, telemetryparameters: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShareFolderWithTelemetryAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchFileWithUIAsync: unsafe extern "system" fn(this: *mut *mut Self, appexepath: ::windows_sys::core::HSTRING, argumentstemplate: ::windows_sys::core::HSTRING, filepath: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFileWithUIAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchFileWithUIAndTelemetryAsync: unsafe extern "system" fn(this: *mut *mut Self, appexepath: ::windows_sys::core::HSTRING, argumentstemplate: ::windows_sys::core::HSTRING, filepath: ::windows_sys::core::HSTRING, telemetryparameters: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFileWithUIAndTelemetryAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TerminateAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TerminateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TerminateWithTelemetryAsync: unsafe extern "system" fn(this: *mut *mut Self, telemetryparameters: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TerminateWithTelemetryAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterMessageReceiver: unsafe extern "system" fn(this: *mut *mut Self, receiverid: ::windows_sys::core::GUID, messagereceivedcallback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterMessageReceiver: usize,
    pub UnregisterMessageReceiver: unsafe extern "system" fn(this: *mut *mut Self, receiverid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IIsolatedWindowsEnvironment {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1104299415, data2: 49960, data3: 17511, data4: [179, 127, 77, 252, 111, 96, 182, 188] };
}
#[repr(C)]
pub struct IIsolatedWindowsEnvironment2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub PostMessageToReceiverAsync: unsafe extern "system" fn(this: *mut *mut Self, receiverid: ::windows_sys::core::GUID, message: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PostMessageToReceiverAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub PostMessageToReceiverWithTelemetryAsync: unsafe extern "system" fn(this: *mut *mut Self, receiverid: ::windows_sys::core::GUID, message: *mut ::core::ffi::c_void, telemetryparameters: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PostMessageToReceiverWithTelemetryAsync: usize,
}
impl ::windows_sys::core::Interface for IIsolatedWindowsEnvironment2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 758538041, data2: 35005, data3: 19124, data4: [147, 207, 126, 43, 206, 243, 55, 192] };
}
#[repr(C)]
pub struct IIsolatedWindowsEnvironment3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetUserInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ShareFileAsync: unsafe extern "system" fn(this: *mut *mut Self, filepath: ::windows_sys::core::HSTRING, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShareFileAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShareFileWithTelemetryAsync: unsafe extern "system" fn(this: *mut *mut Self, filepath: ::windows_sys::core::HSTRING, options: *mut ::core::ffi::c_void, telemetryparameters: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShareFileWithTelemetryAsync: usize,
}
impl ::windows_sys::core::Interface for IIsolatedWindowsEnvironment3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3414149074, data2: 53358, data3: 19494, data4: [138, 218, 218, 205, 170, 173, 3, 245] };
}
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentCreateResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut IsolatedWindowsEnvironmentCreateStatus) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub Environment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IIsolatedWindowsEnvironmentCreateResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4019871320, data2: 56535, data3: 17858, data4: [156, 133, 171, 100, 42, 113, 94, 142] };
}
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateAsync: unsafe extern "system" fn(this: *mut *mut Self, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateWithTelemetryAsync: unsafe extern "system" fn(this: *mut *mut Self, options: *mut ::core::ffi::c_void, telemetryparameters: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWithTelemetryAsync: usize,
    pub GetById: unsafe extern "system" fn(this: *mut *mut Self, environmentid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindByOwnerId: unsafe extern "system" fn(this: *mut *mut Self, environmentownerid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindByOwnerId: usize,
}
impl ::windows_sys::core::Interface for IIsolatedWindowsEnvironmentFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 449483751, data2: 59396, data3: 17741, data4: [132, 102, 249, 137, 124, 32, 176, 246] };
}
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentFile {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub HostPath: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IIsolatedWindowsEnvironmentFile {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1297801711, data2: 671, data3: 16641, data4: [140, 53, 254, 145, 191, 156, 213, 240] };
}
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentFile2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GuestPath: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsReadOnly: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IIsolatedWindowsEnvironmentFile2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1324060140, data2: 44381, data3: 19210, data4: [183, 84, 243, 108, 61, 70, 214, 132] };
}
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentHostStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsReady: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub HostErrors: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    HostErrors: usize,
}
impl ::windows_sys::core::Interface for IIsolatedWindowsEnvironmentHostStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 739123911, data2: 1440, data3: 20858, data4: [184, 28, 110, 232, 121, 12, 56, 31] };
}
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentLaunchFileResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut IsolatedWindowsEnvironmentLaunchFileStatus) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub File: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IIsolatedWindowsEnvironmentLaunchFileResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1750942070, data2: 63200, data3: 17769, data4: [177, 170, 33, 92, 15, 245, 178, 87] };
}
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub EnvironmentOwnerId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetEnvironmentOwnerId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AllowedClipboardFormats: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut IsolatedWindowsEnvironmentAllowedClipboardFormats) -> ::windows_sys::core::HRESULT,
    pub SetAllowedClipboardFormats: unsafe extern "system" fn(this: *mut *mut Self, value: IsolatedWindowsEnvironmentAllowedClipboardFormats) -> ::windows_sys::core::HRESULT,
    pub ClipboardCopyPasteDirections: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut IsolatedWindowsEnvironmentClipboardCopyPasteDirections) -> ::windows_sys::core::HRESULT,
    pub SetClipboardCopyPasteDirections: unsafe extern "system" fn(this: *mut *mut Self, value: IsolatedWindowsEnvironmentClipboardCopyPasteDirections) -> ::windows_sys::core::HRESULT,
    pub AvailablePrinters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut IsolatedWindowsEnvironmentAvailablePrinters) -> ::windows_sys::core::HRESULT,
    pub SetAvailablePrinters: unsafe extern "system" fn(this: *mut *mut Self, value: IsolatedWindowsEnvironmentAvailablePrinters) -> ::windows_sys::core::HRESULT,
    pub SharedHostFolderPath: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SharedFolderNameInEnvironment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ShareHostFolderForUntrustedItems: unsafe extern "system" fn(this: *mut *mut Self, sharedhostfolderpath: ::windows_sys::core::HSTRING, sharefoldernameinenvironment: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PersistUserProfile: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetPersistUserProfile: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AllowGraphicsHardwareAcceleration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowGraphicsHardwareAcceleration: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AllowCameraAndMicrophoneAccess: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowCameraAndMicrophoneAccess: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IIsolatedWindowsEnvironmentOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3072170231, data2: 25072, data3: 16392, data4: [178, 7, 11, 249, 235, 45, 118, 242] };
}
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentOptions2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub WindowAnnotationOverride: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetWindowAnnotationOverride: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IIsolatedWindowsEnvironmentOptions2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 282577969, data2: 35727, data3: 19357, data4: [178, 44, 97, 113, 3, 181, 91, 8] };
}
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationData {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ShareableFolders: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ShareableFolders: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ProcessesRunnableAsSystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProcessesRunnableAsSystem: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ProcessesRunnableAsUser: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProcessesRunnableAsUser: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ActivationFileExtensions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ActivationFileExtensions: usize,
}
impl ::windows_sys::core::Interface for IIsolatedWindowsEnvironmentOwnerRegistrationData {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4169722914, data2: 59599, data3: 22208, data4: [177, 223, 144, 175, 74, 216, 14, 132] };
}
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut IsolatedWindowsEnvironmentOwnerRegistrationStatus) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IIsolatedWindowsEnvironmentOwnerRegistrationResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1839961169, data2: 24937, data3: 21983, data4: [143, 81, 121, 14, 153, 215, 39, 125] };
}
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Register: unsafe extern "system" fn(this: *mut *mut Self, ownername: ::windows_sys::core::HSTRING, ownerregistrationdata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Unregister: unsafe extern "system" fn(this: *mut *mut Self, ownername: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IIsolatedWindowsEnvironmentOwnerRegistrationStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 278206292, data2: 8267, data3: 24265, data4: [157, 227, 223, 121, 45, 7, 74, 97] };
}
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentPostMessageResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut IsolatedWindowsEnvironmentPostMessageStatus) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IIsolatedWindowsEnvironmentPostMessageResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 234498298, data2: 12016, data3: 19855, data4: [179, 65, 49, 113, 178, 223, 147, 177] };
}
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentProcess {
    pub base__: ::windows_sys::core::IInspectable,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut IsolatedWindowsEnvironmentProcessState) -> ::windows_sys::core::HRESULT,
    pub ExitCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub WaitForExit: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub WaitForExitWithTimeout: unsafe extern "system" fn(this: *mut *mut Self, timeoutmilliseconds: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub WaitForExitAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WaitForExitAsync: usize,
}
impl ::windows_sys::core::Interface for IIsolatedWindowsEnvironmentProcess {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2824389615, data2: 33138, data3: 20240, data4: [175, 147, 203, 230, 10, 248, 141, 9] };
}
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentShareFileRequestOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub AllowWrite: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowWrite: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IIsolatedWindowsEnvironmentShareFileRequestOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3373862616, data2: 4048, data3: 18758, data4: [187, 136, 17, 122, 96, 115, 123, 97] };
}
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentShareFileResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut IsolatedWindowsEnvironmentShareFileStatus) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub File: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IIsolatedWindowsEnvironmentShareFileResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2932329127, data2: 39622, data3: 19445, data4: [139, 145, 92, 26, 223, 13, 125, 0] };
}
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentShareFolderRequestOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub AllowWrite: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowWrite: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IIsolatedWindowsEnvironmentShareFolderRequestOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3288722301, data2: 28755, data3: 20330, data4: [155, 135, 116, 104, 70, 237, 25, 178] };
}
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentShareFolderResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut IsolatedWindowsEnvironmentShareFolderStatus) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IIsolatedWindowsEnvironmentShareFolderResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1433118510, data2: 51869, data3: 16913, data4: [177, 67, 28, 237, 200, 110, 178, 254] };
}
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentStartProcessResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut IsolatedWindowsEnvironmentStartProcessStatus) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub Process: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IIsolatedWindowsEnvironmentStartProcessResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2409749551, data2: 22490, data3: 19381, data4: [156, 6, 250, 7, 45, 32, 50, 226] };
}
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentTelemetryParameters {
    pub base__: ::windows_sys::core::IInspectable,
    pub CorrelationId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SetCorrelationId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IIsolatedWindowsEnvironmentTelemetryParameters {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3957013675, data2: 31290, data3: 17700, data4: [160, 244, 249, 110, 40, 77, 51, 205] };
}
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentUserInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub EnvironmentUserSid: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub EnvironmentUserName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryWaitForSignInAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryWaitForSignInAsync: usize,
}
impl ::windows_sys::core::Interface for IIsolatedWindowsEnvironmentUserInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2325509550, data2: 27066, data3: 16385, data4: [150, 252, 25, 160, 39, 3, 179, 64] };
}
#[repr(C)]
pub struct IIsolatedWindowsHostMessengerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub PostMessageToReceiver: unsafe extern "system" fn(this: *mut *mut Self, receiverid: ::windows_sys::core::GUID, message: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PostMessageToReceiver: usize,
    pub GetFileId: unsafe extern "system" fn(this: *mut *mut Self, filepath: ::windows_sys::core::HSTRING, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IIsolatedWindowsHostMessengerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 115623099, data2: 21440, data3: 18569, data4: [143, 163, 83, 89, 46, 55, 207, 33] };
}
#[repr(C)]
pub struct IIsolatedWindowsHostMessengerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterHostMessageReceiver: unsafe extern "system" fn(this: *mut *mut Self, receiverid: ::windows_sys::core::GUID, hostmessagereceivedcallback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterHostMessageReceiver: usize,
    pub UnregisterHostMessageReceiver: unsafe extern "system" fn(this: *mut *mut Self, receiverid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IIsolatedWindowsHostMessengerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1441767100, data2: 1092, data3: 17069, data4: [131, 45, 27, 137, 192, 137, 209, 202] };
}
pub type IsolatedWindowsEnvironment = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentActivator(pub i32);
impl IsolatedWindowsEnvironmentActivator {
    pub const System: Self = Self(0i32);
    pub const User: Self = Self(1i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentActivator {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentActivator {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentAllowedClipboardFormats(pub u32);
impl IsolatedWindowsEnvironmentAllowedClipboardFormats {
    pub const None: Self = Self(0u32);
    pub const Text: Self = Self(1u32);
    pub const Image: Self = Self(2u32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentAllowedClipboardFormats {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentAvailablePrinters(pub u32);
impl IsolatedWindowsEnvironmentAvailablePrinters {
    pub const None: Self = Self(0u32);
    pub const Local: Self = Self(1u32);
    pub const Network: Self = Self(2u32);
    pub const SystemPrintToPdf: Self = Self(4u32);
    pub const SystemPrintToXps: Self = Self(8u32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentAvailablePrinters {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentAvailablePrinters {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentClipboardCopyPasteDirections(pub u32);
impl IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    pub const None: Self = Self(0u32);
    pub const HostToIsolatedWindowsEnvironment: Self = Self(1u32);
    pub const IsolatedWindowsEnvironmentToHost: Self = Self(2u32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Security_Isolation\"`*"]
pub struct IsolatedWindowsEnvironmentCreateProgress {
    pub State: IsolatedWindowsEnvironmentProgressState,
    pub PercentComplete: u32,
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentCreateProgress {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentCreateProgress {
    fn clone(&self) -> Self {
        *self
    }
}
pub type IsolatedWindowsEnvironmentCreateResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentCreateStatus(pub i32);
impl IsolatedWindowsEnvironmentCreateStatus {
    pub const Success: Self = Self(0i32);
    pub const FailureByPolicy: Self = Self(1i32);
    pub const UnknownFailure: Self = Self(2i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentCreateStatus {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentCreateStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type IsolatedWindowsEnvironmentFile = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentHostError(pub i32);
impl IsolatedWindowsEnvironmentHostError {
    pub const AdminPolicyIsDisabledOrNotPresent: Self = Self(0i32);
    pub const FeatureNotInstalled: Self = Self(1i32);
    pub const HardwareRequirementsNotMet: Self = Self(2i32);
    pub const RebootRequired: Self = Self(3i32);
    pub const UnknownError: Self = Self(4i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentHostError {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentHostError {
    fn clone(&self) -> Self {
        *self
    }
}
pub type IsolatedWindowsEnvironmentLaunchFileResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentLaunchFileStatus(pub i32);
impl IsolatedWindowsEnvironmentLaunchFileStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const EnvironmentUnavailable: Self = Self(2i32);
    pub const FileNotFound: Self = Self(3i32);
    pub const TimedOut: Self = Self(4i32);
    pub const AlreadySharedWithConflictingOptions: Self = Self(5i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentLaunchFileStatus {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentLaunchFileStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type IsolatedWindowsEnvironmentOptions = *mut ::core::ffi::c_void;
pub type IsolatedWindowsEnvironmentOwnerRegistrationData = *mut ::core::ffi::c_void;
pub type IsolatedWindowsEnvironmentOwnerRegistrationResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentOwnerRegistrationStatus(pub i32);
impl IsolatedWindowsEnvironmentOwnerRegistrationStatus {
    pub const Success: Self = Self(0i32);
    pub const InvalidArgument: Self = Self(1i32);
    pub const AccessDenied: Self = Self(2i32);
    pub const InsufficientMemory: Self = Self(3i32);
    pub const UnknownFailure: Self = Self(4i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentOwnerRegistrationStatus {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentOwnerRegistrationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type IsolatedWindowsEnvironmentPostMessageResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentPostMessageStatus(pub i32);
impl IsolatedWindowsEnvironmentPostMessageStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const EnvironmentUnavailable: Self = Self(2i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentPostMessageStatus {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentPostMessageStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type IsolatedWindowsEnvironmentProcess = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentProcessState(pub i32);
impl IsolatedWindowsEnvironmentProcessState {
    pub const Running: Self = Self(1i32);
    pub const Aborted: Self = Self(2i32);
    pub const Completed: Self = Self(3i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentProcessState {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentProcessState {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentProgressState(pub i32);
impl IsolatedWindowsEnvironmentProgressState {
    pub const Queued: Self = Self(0i32);
    pub const Processing: Self = Self(1i32);
    pub const Completed: Self = Self(2i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentProgressState {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentProgressState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type IsolatedWindowsEnvironmentShareFileRequestOptions = *mut ::core::ffi::c_void;
pub type IsolatedWindowsEnvironmentShareFileResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentShareFileStatus(pub i32);
impl IsolatedWindowsEnvironmentShareFileStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const EnvironmentUnavailable: Self = Self(2i32);
    pub const AlreadySharedWithConflictingOptions: Self = Self(3i32);
    pub const FileNotFound: Self = Self(4i32);
    pub const AccessDenied: Self = Self(5i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentShareFileStatus {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentShareFileStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type IsolatedWindowsEnvironmentShareFolderRequestOptions = *mut ::core::ffi::c_void;
pub type IsolatedWindowsEnvironmentShareFolderResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentShareFolderStatus(pub i32);
impl IsolatedWindowsEnvironmentShareFolderStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const EnvironmentUnavailable: Self = Self(2i32);
    pub const FolderNotFound: Self = Self(3i32);
    pub const AccessDenied: Self = Self(4i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentShareFolderStatus {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentShareFolderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type IsolatedWindowsEnvironmentStartProcessResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentStartProcessStatus(pub i32);
impl IsolatedWindowsEnvironmentStartProcessStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const EnvironmentUnavailable: Self = Self(2i32);
    pub const FileNotFound: Self = Self(3i32);
    pub const AppNotRegistered: Self = Self(4i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentStartProcessStatus {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentStartProcessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type IsolatedWindowsEnvironmentTelemetryParameters = *mut ::core::ffi::c_void;
pub type IsolatedWindowsEnvironmentUserInfo = *mut ::core::ffi::c_void;
pub type MessageReceivedCallback = *mut ::core::ffi::c_void;
