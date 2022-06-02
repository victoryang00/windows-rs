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
#[repr(C)]
pub struct IActivatedEventArgsWithUser {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[repr(C)]
pub struct IApplicationViewActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub CurrentlyShownApplicationViewId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppointmentsProviderActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Verb: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppointmentsProviderAddAppointmentActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub AddAppointmentOperation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments_AppointmentsProvider"))]
    AddAppointmentOperation: usize,
}
#[repr(C)]
pub struct IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub RemoveAppointmentOperation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments_AppointmentsProvider"))]
    RemoveAppointmentOperation: usize,
}
#[repr(C)]
pub struct IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub ReplaceAppointmentOperation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments_AppointmentsProvider"))]
    ReplaceAppointmentOperation: usize,
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
#[repr(C)]
pub struct IBackgroundActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_Background")]
    pub TaskInstance: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Background"))]
    TaskInstance: usize,
}
#[repr(C)]
pub struct IBarcodeScannerPreviewActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ConnectionId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICachedFileUpdaterActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Provider")]
    pub CachedFileUpdaterUI: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Provider"))]
    CachedFileUpdaterUI: usize,
}
#[repr(C)]
pub struct ICameraSettingsActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub VideoDeviceController: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VideoDeviceExtension: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICommandLineActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Operation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IContactActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Verb: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IContactPickerActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_Contacts_Provider")]
    pub ContactPickerUI: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts_Provider"))]
    ContactPickerUI: usize,
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
#[repr(C)]
pub struct IContactsProviderActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Verb: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IContinuationActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ContinuationData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ContinuationData: usize,
}
#[repr(C)]
pub struct IDeviceActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceInformationId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Verb: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDevicePairingActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Enumeration")]
    pub DeviceInformation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    DeviceInformation: usize,
}
#[repr(C)]
pub struct IDialReceiverActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub AppName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IFileActivatedEventArgsWithCallerPackageFamilyName {
    pub base__: ::windows_sys::core::IInspectable,
    pub CallerPackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFileActivatedEventArgsWithNeighboringFiles {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Search")]
    pub NeighboringFilesQuery: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Search"))]
    NeighboringFilesQuery: usize,
}
#[repr(C)]
pub struct IFileOpenPickerActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub FileOpenPickerUI: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Pickers_Provider"))]
    FileOpenPickerUI: usize,
}
#[repr(C)]
pub struct IFileOpenPickerActivatedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CallerPackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IFileSavePickerActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub FileSavePickerUI: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Pickers_Provider"))]
    FileSavePickerUI: usize,
}
#[repr(C)]
pub struct IFileSavePickerActivatedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CallerPackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub EnterpriseId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IFolderPickerContinuationEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub Folder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Storage", feature = "deprecated")))]
    Folder: usize,
}
#[repr(C)]
pub struct ILaunchActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Arguments: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TileId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILaunchActivatedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TileActivatedInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILockScreenActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Info: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILockScreenCallActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_Calls")]
    pub CallUI: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Calls"))]
    CallUI: usize,
}
#[repr(C)]
pub struct IPhoneCallActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub LineId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPickerReturnedActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub PickerOperationId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrelaunchActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub PrelaunchActivated: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrint3DWorkflowActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub Workflow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Printers_Extensions"))]
    Workflow: usize,
}
#[repr(C)]
pub struct IPrintTaskSettingsActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub Configuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Printers_Extensions"))]
    Configuration: usize,
}
#[repr(C)]
pub struct IProtocolActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
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
#[repr(C)]
pub struct IProtocolForResultsActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub ProtocolForResultsOperation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    ProtocolForResultsOperation: usize,
}
#[repr(C)]
pub struct IRestrictedLaunchActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub SharedContext: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISearchActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub QueryText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISearchActivatedEventArgsWithLinguisticDetails {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_Search")]
    pub LinguisticDetails: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Search"))]
    LinguisticDetails: usize,
}
#[repr(C)]
pub struct IShareTargetActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
    pub ShareOperation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_DataTransfer_ShareTarget"))]
    ShareOperation: usize,
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
#[repr(C)]
pub struct IStartupTaskActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub TaskId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITileActivatedInfo {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Notifications"))]
    pub RecentlyShownNotifications: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Notifications")))]
    RecentlyShownNotifications: usize,
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
#[repr(C)]
pub struct IUserDataAccountProviderActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
    pub Operation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_UserDataAccounts_Provider"))]
    Operation: usize,
}
#[repr(C)]
pub struct IViewSwitcherProvider {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_ViewManagement")]
    pub ViewSwitcher: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_ViewManagement"))]
    ViewSwitcher: usize,
}
#[repr(C)]
pub struct IVoiceCommandActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_SpeechRecognition")]
    pub Result: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_SpeechRecognition"))]
    Result: usize,
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
#[repr(C)]
pub struct IWebAccountProviderActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Authentication_Web_Provider")]
    pub Operation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web_Provider"))]
    Operation: usize,
}
#[repr(C)]
pub struct IWebAuthenticationBrokerContinuationEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Authentication_Web")]
    pub WebAuthenticationResult: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web"))]
    WebAuthenticationResult: usize,
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
