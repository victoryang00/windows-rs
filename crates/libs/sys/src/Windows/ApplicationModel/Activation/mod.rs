#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ActivationKind(pub i32);
impl ActivationKind {
    pub const Launch: Self = Self(0i32);
    pub const Search: Self = Self(1i32);
    pub const ShareTarget: Self = Self(2i32);
    pub const File: Self = Self(3i32);
    pub const Protocol: Self = Self(4i32);
    pub const FileOpenPicker: Self = Self(5i32);
    pub const FileSavePicker: Self = Self(6i32);
    pub const CachedFileUpdater: Self = Self(7i32);
    pub const ContactPicker: Self = Self(8i32);
    pub const Device: Self = Self(9i32);
    pub const PrintTaskSettings: Self = Self(10i32);
    pub const CameraSettings: Self = Self(11i32);
    pub const RestrictedLaunch: Self = Self(12i32);
    pub const AppointmentsProvider: Self = Self(13i32);
    pub const Contact: Self = Self(14i32);
    pub const LockScreenCall: Self = Self(15i32);
    pub const VoiceCommand: Self = Self(16i32);
    pub const LockScreen: Self = Self(17i32);
    pub const PickerReturned: Self = Self(1000i32);
    pub const WalletAction: Self = Self(1001i32);
    pub const PickFileContinuation: Self = Self(1002i32);
    pub const PickSaveFileContinuation: Self = Self(1003i32);
    pub const PickFolderContinuation: Self = Self(1004i32);
    pub const WebAuthenticationBrokerContinuation: Self = Self(1005i32);
    pub const WebAccountProvider: Self = Self(1006i32);
    pub const ComponentUI: Self = Self(1007i32);
    pub const ProtocolForResults: Self = Self(1009i32);
    pub const ToastNotification: Self = Self(1010i32);
    pub const Print3DWorkflow: Self = Self(1011i32);
    pub const DialReceiver: Self = Self(1012i32);
    pub const DevicePairing: Self = Self(1013i32);
    pub const UserDataAccountsProvider: Self = Self(1014i32);
    pub const FilePickerExperience: Self = Self(1015i32);
    pub const LockScreenComponent: Self = Self(1016i32);
    pub const ContactPanel: Self = Self(1017i32);
    pub const PrintWorkflowForegroundTask: Self = Self(1018i32);
    pub const GameUIProvider: Self = Self(1019i32);
    pub const StartupTask: Self = Self(1020i32);
    pub const CommandLineLaunch: Self = Self(1021i32);
    pub const BarcodeScannerProvider: Self = Self(1022i32);
    pub const PrintSupportJobUI: Self = Self(1023i32);
    pub const PrintSupportSettingsUI: Self = Self(1024i32);
    pub const PhoneCallActivation: Self = Self(1025i32);
    pub const VpnForeground: Self = Self(1026i32);
}
impl ::core::marker::Copy for ActivationKind {}
impl ::core::clone::Clone for ActivationKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ApplicationExecutionState(pub i32);
impl ApplicationExecutionState {
    pub const NotRunning: Self = Self(0i32);
    pub const Running: Self = Self(1i32);
    pub const Suspended: Self = Self(2i32);
    pub const Terminated: Self = Self(3i32);
    pub const ClosedByUser: Self = Self(4i32);
}
impl ::core::marker::Copy for ApplicationExecutionState {}
impl ::core::clone::Clone for ApplicationExecutionState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppointmentsProviderAddAppointmentActivatedEventArgs = *mut ::core::ffi::c_void;
pub type AppointmentsProviderRemoveAppointmentActivatedEventArgs = *mut ::core::ffi::c_void;
pub type AppointmentsProviderReplaceAppointmentActivatedEventArgs = *mut ::core::ffi::c_void;
pub type AppointmentsProviderShowAppointmentDetailsActivatedEventArgs = *mut ::core::ffi::c_void;
pub type AppointmentsProviderShowTimeFrameActivatedEventArgs = *mut ::core::ffi::c_void;
pub type BackgroundActivatedEventArgs = *mut ::core::ffi::c_void;
pub type BarcodeScannerPreviewActivatedEventArgs = *mut ::core::ffi::c_void;
pub type CachedFileUpdaterActivatedEventArgs = *mut ::core::ffi::c_void;
pub type CameraSettingsActivatedEventArgs = *mut ::core::ffi::c_void;
pub type CommandLineActivatedEventArgs = *mut ::core::ffi::c_void;
pub type CommandLineActivationOperation = *mut ::core::ffi::c_void;
pub type ContactCallActivatedEventArgs = *mut ::core::ffi::c_void;
pub type ContactMapActivatedEventArgs = *mut ::core::ffi::c_void;
pub type ContactMessageActivatedEventArgs = *mut ::core::ffi::c_void;
pub type ContactPanelActivatedEventArgs = *mut ::core::ffi::c_void;
pub type ContactPickerActivatedEventArgs = *mut ::core::ffi::c_void;
pub type ContactPostActivatedEventArgs = *mut ::core::ffi::c_void;
pub type ContactVideoCallActivatedEventArgs = *mut ::core::ffi::c_void;
pub type DeviceActivatedEventArgs = *mut ::core::ffi::c_void;
pub type DevicePairingActivatedEventArgs = *mut ::core::ffi::c_void;
pub type DialReceiverActivatedEventArgs = *mut ::core::ffi::c_void;
pub type FileActivatedEventArgs = *mut ::core::ffi::c_void;
pub type FileOpenPickerActivatedEventArgs = *mut ::core::ffi::c_void;
pub type FileOpenPickerContinuationEventArgs = *mut ::core::ffi::c_void;
pub type FileSavePickerActivatedEventArgs = *mut ::core::ffi::c_void;
pub type FileSavePickerContinuationEventArgs = *mut ::core::ffi::c_void;
pub type FolderPickerContinuationEventArgs = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ActivationKind) -> ::windows_sys::core::HRESULT,
    pub PreviousExecutionState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ApplicationExecutionState) -> ::windows_sys::core::HRESULT,
    pub SplashScreen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3479508755, data2: 52488, data3: 20440, data4: [182, 151, 162, 129, 182, 84, 78, 46] };
}
#[repr(C)]
pub struct IActivatedEventArgsWithUser {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
impl ::windows_sys::core::Interface for IActivatedEventArgsWithUser {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 485530526, data2: 39266, data3: 18742, data4: [128, 255, 175, 200, 232, 174, 92, 140] };
}
#[repr(C)]
pub struct IApplicationViewActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub CurrentlyShownApplicationViewId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IApplicationViewActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2467098443, data2: 47145, data3: 16636, data4: [136, 244, 133, 19, 232, 166, 71, 56] };
}
#[repr(C)]
pub struct IAppointmentsProviderActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Verb: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppointmentsProviderActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 862241797, data2: 37692, data3: 20093, data4: [160, 52, 80, 15, 184, 220, 217, 243] };
}
#[repr(C)]
pub struct IAppointmentsProviderAddAppointmentActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub AddAppointmentOperation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments_AppointmentsProvider"))]
    AddAppointmentOperation: usize,
}
impl ::windows_sys::core::Interface for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2726695783, data2: 52965, data3: 20045, data4: [158, 215, 65, 195, 78, 193, 139, 2] };
}
#[repr(C)]
pub struct IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub RemoveAppointmentOperation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments_AppointmentsProvider"))]
    RemoveAppointmentOperation: usize,
}
impl ::windows_sys::core::Interface for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1964980920, data2: 2958, data3: 17692, data4: [159, 21, 150, 110, 105, 155, 172, 37] };
}
#[repr(C)]
pub struct IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub ReplaceAppointmentOperation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments_AppointmentsProvider"))]
    ReplaceAppointmentOperation: usize,
}
impl ::windows_sys::core::Interface for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 357677012, data2: 43393, data3: 16487, data4: [138, 98, 5, 36, 228, 173, 225, 33] };
}
#[repr(C)]
pub struct IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub InstanceStartDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InstanceStartDate: usize,
    pub LocalId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RoamingId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 962130021, data2: 38977, data3: 19621, data4: [153, 155, 136, 81, 152, 185, 239, 42] };
}
#[repr(C)]
pub struct IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub TimeToShow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimeToShow: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
}
impl ::windows_sys::core::Interface for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2611915686, data2: 3595, data3: 18858, data4: [186, 188, 18, 177, 220, 119, 73, 134] };
}
#[repr(C)]
pub struct IBackgroundActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_Background")]
    pub TaskInstance: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Background"))]
    TaskInstance: usize,
}
impl ::windows_sys::core::Interface for IBackgroundActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2870263520, data2: 59232, data3: 17422, data4: [169, 28, 68, 121, 109, 227, 169, 45] };
}
#[repr(C)]
pub struct IBarcodeScannerPreviewActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ConnectionId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBarcodeScannerPreviewActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1735555452, data2: 39359, data3: 17225, data4: [175, 34, 228, 18, 53, 96, 55, 28] };
}
#[repr(C)]
pub struct ICachedFileUpdaterActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Provider")]
    pub CachedFileUpdaterUI: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Provider"))]
    CachedFileUpdaterUI: usize,
}
impl ::windows_sys::core::Interface for ICachedFileUpdaterActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3496915399, data2: 14341, data3: 20171, data4: [183, 87, 108, 241, 94, 38, 254, 243] };
}
#[repr(C)]
pub struct ICameraSettingsActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub VideoDeviceController: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VideoDeviceExtension: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICameraSettingsActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4217873672, data2: 11693, data3: 18698, data4: [145, 112, 220, 160, 54, 235, 17, 75] };
}
#[repr(C)]
pub struct ICommandLineActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Operation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICommandLineActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1158039340, data2: 106, data3: 18667, data4: [138, 251, 208, 122, 178, 94, 51, 102] };
}
#[repr(C)]
pub struct ICommandLineActivationOperation {
    pub base__: ::windows_sys::core::IInspectable,
    pub Arguments: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CurrentDirectoryPath: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetExitCode: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub ExitCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for ICommandLineActivationOperation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2571839553, data2: 50590, data3: 20329, data4: [188, 253, 182, 30, 212, 230, 34, 235] };
}
#[repr(C)]
pub struct IContactActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Verb: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3592921540, data2: 49189, data3: 19521, data4: [157, 239, 241, 234, 250, 208, 117, 231] };
}
#[repr(C)]
pub struct IContactCallActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ServiceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ServiceUserId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
}
impl ::windows_sys::core::Interface for IContactCallActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3269399751, data2: 12523, data3: 16838, data4: [179, 188, 91, 22, 148, 249, 218, 179] };
}
#[repr(C)]
pub struct IContactMapActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Address: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Address: usize,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
}
impl ::windows_sys::core::Interface for IContactMapActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3006003312, data2: 61159, data3: 19154, data4: [170, 241, 168, 126, 255, 207, 0, 164] };
}
#[repr(C)]
pub struct IContactMessageActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ServiceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ServiceUserId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
}
impl ::windows_sys::core::Interface for IContactMessageActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3730410930, data2: 3587, data3: 17328, data4: [191, 86, 188, 196, 11, 49, 98, 223] };
}
#[repr(C)]
pub struct IContactPanelActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub ContactPanel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    ContactPanel: usize,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
}
impl ::windows_sys::core::Interface for IContactPanelActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1388012516, data2: 54228, data3: 19299, data4: [128, 81, 74, 242, 8, 44, 171, 128] };
}
#[repr(C)]
pub struct IContactPickerActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_Contacts_Provider")]
    pub ContactPickerUI: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts_Provider"))]
    ContactPickerUI: usize,
}
impl ::windows_sys::core::Interface for IContactPickerActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3461851879, data2: 25673, data3: 17831, data4: [151, 31, 209, 19, 190, 122, 137, 54] };
}
#[repr(C)]
pub struct IContactPostActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ServiceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ServiceUserId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
}
impl ::windows_sys::core::Interface for IContactPostActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3009035367, data2: 61927, data3: 18005, data4: [173, 110, 72, 87, 88, 143, 85, 47] };
}
#[repr(C)]
pub struct IContactVideoCallActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ServiceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ServiceUserId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
}
impl ::windows_sys::core::Interface for IContactVideoCallActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1627889080, data2: 58343, data3: 19279, data4: [133, 141, 92, 99, 169, 110, 246, 132] };
}
#[repr(C)]
pub struct IContactsProviderActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Verb: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactsProviderActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1166073000, data2: 22352, data3: 18710, data4: [170, 82, 192, 130, 149, 33, 235, 148] };
}
#[repr(C)]
pub struct IContinuationActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ContinuationData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ContinuationData: usize,
}
impl ::windows_sys::core::Interface for IContinuationActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3850438325, data2: 5471, data3: 19092, data4: [167, 66, 199, 224, 143, 78, 24, 140] };
}
#[repr(C)]
pub struct IDeviceActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceInformationId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Verb: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDeviceActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3444619689, data2: 52752, data3: 17618, data4: [130, 52, 195, 85, 160, 115, 239, 51] };
}
#[repr(C)]
pub struct IDevicePairingActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Enumeration")]
    pub DeviceInformation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    DeviceInformation: usize,
}
impl ::windows_sys::core::Interface for IDevicePairingActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3953185252, data2: 60614, data3: 16712, data4: [148, 237, 244, 179, 126, 192, 91, 62] };
}
#[repr(C)]
pub struct IDialReceiverActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub AppName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDialReceiverActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4218912471, data2: 34286, data3: 17774, data4: [164, 77, 133, 215, 48, 231, 10, 237] };
}
#[repr(C)]
pub struct IFileActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub Files: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    Files: usize,
    pub Verb: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFileActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3140156467, data2: 37809, data3: 17133, data4: [139, 38, 35, 109, 217, 199, 132, 150] };
}
#[repr(C)]
pub struct IFileActivatedEventArgsWithCallerPackageFamilyName {
    pub base__: ::windows_sys::core::IInspectable,
    pub CallerPackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFileActivatedEventArgsWithCallerPackageFamilyName {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 761327723, data2: 53855, data3: 19749, data4: [134, 83, 225, 197, 225, 16, 131, 9] };
}
#[repr(C)]
pub struct IFileActivatedEventArgsWithNeighboringFiles {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Search")]
    pub NeighboringFilesQuery: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Search"))]
    NeighboringFilesQuery: usize,
}
impl ::windows_sys::core::Interface for IFileActivatedEventArgsWithNeighboringFiles {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1127981476, data2: 57826, data3: 18685, data4: [183, 252, 181, 214, 238, 230, 80, 51] };
}
#[repr(C)]
pub struct IFileOpenPickerActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub FileOpenPickerUI: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Pickers_Provider"))]
    FileOpenPickerUI: usize,
}
impl ::windows_sys::core::Interface for IFileOpenPickerActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921151106, data2: 21797, data3: 19442, data4: [188, 9, 31, 80, 149, 212, 150, 77] };
}
#[repr(C)]
pub struct IFileOpenPickerActivatedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CallerPackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFileOpenPickerActivatedEventArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1584602982, data2: 36127, data3: 17915, data4: [175, 29, 115, 32, 92, 143, 199, 161] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IFileOpenPickerContinuationEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
    pub Files: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated")))]
    Files: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IFileOpenPickerContinuationEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4042932026, data2: 54504, data3: 19155, data4: [156, 52, 35, 8, 243, 47, 206, 201] };
}
#[repr(C)]
pub struct IFileSavePickerActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub FileSavePickerUI: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Pickers_Provider"))]
    FileSavePickerUI: usize,
}
impl ::windows_sys::core::Interface for IFileSavePickerActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2176949489, data2: 29926, data3: 17287, data4: [130, 235, 187, 143, 214, 75, 67, 70] };
}
#[repr(C)]
pub struct IFileSavePickerActivatedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CallerPackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub EnterpriseId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFileSavePickerActivatedEventArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1802763795, data2: 11506, data3: 19784, data4: [140, 188, 175, 103, 210, 63, 28, 231] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IFileSavePickerContinuationEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub File: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Storage", feature = "deprecated")))]
    File: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IFileSavePickerContinuationEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 746876897, data2: 15277, data3: 20275, data4: [140, 139, 228, 111, 174, 130, 75, 75] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IFolderPickerContinuationEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub Folder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Storage", feature = "deprecated")))]
    Folder: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IFolderPickerContinuationEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1367876454, data2: 40779, data3: 18831, data4: [190, 176, 66, 104, 79, 110, 28, 41] };
}
#[repr(C)]
pub struct ILaunchActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Arguments: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TileId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILaunchActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4224269862, data2: 41290, data3: 19279, data4: [130, 176, 51, 190, 217, 32, 175, 82] };
}
#[repr(C)]
pub struct ILaunchActivatedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TileActivatedInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILaunchActivatedEventArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 265518780, data2: 40393, data3: 18101, data4: [154, 206, 189, 149, 212, 86, 83, 69] };
}
#[repr(C)]
pub struct ILockScreenActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Info: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILockScreenActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1017608550, data2: 24840, data3: 19009, data4: [130, 32, 238, 125, 19, 60, 133, 50] };
}
#[repr(C)]
pub struct ILockScreenCallActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_Calls")]
    pub CallUI: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Calls"))]
    CallUI: usize,
}
impl ::windows_sys::core::Interface for ILockScreenCallActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 116621246, data2: 46578, data3: 17547, data4: [177, 62, 227, 40, 172, 28, 81, 106] };
}
#[repr(C)]
pub struct IPhoneCallActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub LineId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhoneCallActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1415664161, data2: 41921, data3: 19693, data4: [182, 47, 140, 96, 82, 54, 25, 173] };
}
#[repr(C)]
pub struct IPickerReturnedActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub PickerOperationId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPickerReturnedActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 906883001, data2: 43475, data3: 18820, data4: [164, 237, 158, 199, 52, 96, 73, 33] };
}
#[repr(C)]
pub struct IPrelaunchActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub PrelaunchActivated: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrelaunchActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 205812091, data2: 6647, data3: 18646, data4: [176, 70, 207, 34, 130, 110, 170, 116] };
}
#[repr(C)]
pub struct IPrint3DWorkflowActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub Workflow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Printers_Extensions"))]
    Workflow: usize,
}
impl ::windows_sys::core::Interface for IPrint3DWorkflowActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1062725515, data2: 62124, data3: 17945, data4: [131, 2, 239, 133, 94, 28, 155, 144] };
}
#[repr(C)]
pub struct IPrintTaskSettingsActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub Configuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Printers_Extensions"))]
    Configuration: usize,
}
impl ::windows_sys::core::Interface for IPrintTaskSettingsActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3996164297, data2: 52822, data3: 18533, data4: [186, 142, 137, 84, 172, 39, 17, 7] };
}
#[repr(C)]
pub struct IProtocolActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
}
impl ::windows_sys::core::Interface for IProtocolActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1620440285, data2: 47040, data3: 18091, data4: [129, 254, 217, 15, 54, 208, 13, 36] };
}
#[repr(C)]
pub struct IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    pub base__: ::windows_sys::core::IInspectable,
    pub CallerPackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Data: usize,
}
impl ::windows_sys::core::Interface for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3628731410, data2: 23695, data3: 17292, data4: [131, 203, 194, 143, 204, 11, 47, 219] };
}
#[repr(C)]
pub struct IProtocolForResultsActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub ProtocolForResultsOperation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    ProtocolForResultsOperation: usize,
}
impl ::windows_sys::core::Interface for IProtocolForResultsActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3880858306, data2: 31463, data3: 17687, data4: [128, 172, 219, 232, 215, 204, 91, 156] };
}
#[repr(C)]
pub struct IRestrictedLaunchActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub SharedContext: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRestrictedLaunchActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3770133633, data2: 49091, data3: 17220, data4: [165, 218, 25, 253, 90, 39, 186, 174] };
}
#[repr(C)]
pub struct ISearchActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub QueryText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISearchActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2360568145, data2: 22728, data3: 17379, data4: [148, 188, 65, 211, 63, 139, 99, 14] };
}
#[repr(C)]
pub struct ISearchActivatedEventArgsWithLinguisticDetails {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_Search")]
    pub LinguisticDetails: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Search"))]
    LinguisticDetails: usize,
}
impl ::windows_sys::core::Interface for ISearchActivatedEventArgsWithLinguisticDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3231658970, data2: 2219, data3: 18737, data4: [155, 124, 69, 16, 37, 242, 31, 129] };
}
#[repr(C)]
pub struct IShareTargetActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
    pub ShareOperation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_DataTransfer_ShareTarget"))]
    ShareOperation: usize,
}
impl ::windows_sys::core::Interface for IShareTargetActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1272641992, data2: 52658, data3: 19147, data4: [191, 195, 102, 72, 86, 51, 120, 236] };
}
#[repr(C)]
pub struct ISplashScreen {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ImageLocation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ImageLocation: usize,
    #[cfg(feature = "Foundation")]
    pub Dismissed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Dismissed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDismissed: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDismissed: usize,
}
impl ::windows_sys::core::Interface for ISplashScreen {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3394082652, data2: 54486, data3: 17392, data4: [151, 192, 8, 51, 198, 57, 28, 36] };
}
#[repr(C)]
pub struct IStartupTaskActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub TaskId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStartupTaskActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 61938264, data2: 21110, data3: 19857, data4: [134, 33, 84, 97, 24, 100, 213, 250] };
}
#[repr(C)]
pub struct ITileActivatedInfo {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Notifications"))]
    pub RecentlyShownNotifications: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Notifications")))]
    RecentlyShownNotifications: usize,
}
impl ::windows_sys::core::Interface for ITileActivatedInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2162467761, data2: 14720, data3: 20247, data4: [183, 56, 137, 25, 78, 11, 143, 101] };
}
#[repr(C)]
pub struct IToastNotificationActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Argument: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub UserInput: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UserInput: usize,
}
impl ::windows_sys::core::Interface for IToastNotificationActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2460512130, data2: 21136, data3: 17181, data4: [190, 133, 196, 170, 238, 184, 104, 95] };
}
#[repr(C)]
pub struct IUserDataAccountProviderActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
    pub Operation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_UserDataAccounts_Provider"))]
    Operation: usize,
}
impl ::windows_sys::core::Interface for IUserDataAccountProviderActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 466220835, data2: 36593, data3: 19025, data4: [166, 58, 254, 113, 30, 234, 182, 7] };
}
#[repr(C)]
pub struct IViewSwitcherProvider {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_ViewManagement")]
    pub ViewSwitcher: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_ViewManagement"))]
    ViewSwitcher: usize,
}
impl ::windows_sys::core::Interface for IViewSwitcherProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 871532710, data2: 23596, data3: 19751, data4: [186, 199, 117, 54, 8, 143, 18, 25] };
}
#[repr(C)]
pub struct IVoiceCommandActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_SpeechRecognition")]
    pub Result: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_SpeechRecognition"))]
    Result: usize,
}
impl ::windows_sys::core::Interface for IVoiceCommandActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2878528765, data2: 36163, data3: 19942, data4: [151, 117, 32, 112, 75, 88, 27, 0] };
}
#[repr(C)]
pub struct IWalletActionActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ItemId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Wallet")]
    pub ActionKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Wallet::WalletActionKind) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Wallet"))]
    ActionKind: usize,
    pub ActionId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWalletActionActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4244374139, data2: 6682, data3: 19746, data4: [146, 63, 174, 111, 69, 250, 82, 217] };
}
#[repr(C)]
pub struct IWebAccountProviderActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Authentication_Web_Provider")]
    pub Operation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web_Provider"))]
    Operation: usize,
}
impl ::windows_sys::core::Interface for IWebAccountProviderActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1924601716, data2: 39146, data3: 19663, data4: [151, 82, 70, 217, 5, 16, 4, 241] };
}
#[repr(C)]
pub struct IWebAuthenticationBrokerContinuationEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Authentication_Web")]
    pub WebAuthenticationResult: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web"))]
    WebAuthenticationResult: usize,
}
impl ::windows_sys::core::Interface for IWebAuthenticationBrokerContinuationEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1977459668, data2: 30484, data3: 17725, data4: [183, 255, 185, 94, 58, 23, 9, 218] };
}
pub type LaunchActivatedEventArgs = *mut ::core::ffi::c_void;
pub type LockScreenActivatedEventArgs = *mut ::core::ffi::c_void;
pub type LockScreenCallActivatedEventArgs = *mut ::core::ffi::c_void;
pub type LockScreenComponentActivatedEventArgs = *mut ::core::ffi::c_void;
pub type PhoneCallActivatedEventArgs = *mut ::core::ffi::c_void;
pub type PickerReturnedActivatedEventArgs = *mut ::core::ffi::c_void;
pub type Print3DWorkflowActivatedEventArgs = *mut ::core::ffi::c_void;
pub type PrintTaskSettingsActivatedEventArgs = *mut ::core::ffi::c_void;
pub type ProtocolActivatedEventArgs = *mut ::core::ffi::c_void;
pub type ProtocolForResultsActivatedEventArgs = *mut ::core::ffi::c_void;
pub type RestrictedLaunchActivatedEventArgs = *mut ::core::ffi::c_void;
pub type SearchActivatedEventArgs = *mut ::core::ffi::c_void;
pub type ShareTargetActivatedEventArgs = *mut ::core::ffi::c_void;
pub type SplashScreen = *mut ::core::ffi::c_void;
pub type StartupTaskActivatedEventArgs = *mut ::core::ffi::c_void;
pub type TileActivatedInfo = *mut ::core::ffi::c_void;
pub type ToastNotificationActivatedEventArgs = *mut ::core::ffi::c_void;
pub type UserDataAccountProviderActivatedEventArgs = *mut ::core::ffi::c_void;
pub type VoiceCommandActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WalletActionActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebAccountProviderActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebAuthenticationBrokerContinuationEventArgs = *mut ::core::ffi::c_void;
