pub const CLSID_CTask: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 344708384, data2: 41643, data3: 4558, data4: [177, 31, 0, 170, 0, 83, 5, 3] };
pub const CLSID_CTaskScheduler: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 344708394, data2: 41643, data3: 4558, data4: [177, 31, 0, 170, 0, 83, 5, 3] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub struct DAILY {
    pub DaysInterval: u16,
}
impl ::core::marker::Copy for DAILY {}
impl ::core::clone::Clone for DAILY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAction {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, pid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Id: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetId: unsafe extern "system" fn(this: *mut *mut Self, id: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetId: usize,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, ptype: *mut TASK_ACTION_TYPE) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IActionCollection {
    pub base__: super::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pcount: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, ppaction: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub XmlText: unsafe extern "system" fn(this: *mut *mut Self, ptext: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    XmlText: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetXmlText: unsafe extern "system" fn(this: *mut *mut Self, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetXmlText: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, r#type: TASK_ACTION_TYPE, ppaction: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Create: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Context: unsafe extern "system" fn(this: *mut *mut Self, pcontext: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Context: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetContext: unsafe extern "system" fn(this: *mut *mut Self, context: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetContext: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IBootTrigger {
    pub base__: ITrigger,
    #[cfg(feature = "Win32_Foundation")]
    pub Delay: unsafe extern "system" fn(this: *mut *mut Self, pdelay: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Delay: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDelay: unsafe extern "system" fn(this: *mut *mut Self, delay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDelay: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IComHandlerAction {
    pub base__: IAction,
    #[cfg(feature = "Win32_Foundation")]
    pub ClassId: unsafe extern "system" fn(this: *mut *mut Self, pclsid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClassId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClassId: unsafe extern "system" fn(this: *mut *mut Self, clsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClassId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, pdata: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Data: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetData: unsafe extern "system" fn(this: *mut *mut Self, data: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetData: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDailyTrigger {
    pub base__: ITrigger,
    pub DaysInterval: unsafe extern "system" fn(this: *mut *mut Self, pdays: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetDaysInterval: unsafe extern "system" fn(this: *mut *mut Self, days: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RandomDelay: unsafe extern "system" fn(this: *mut *mut Self, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RandomDelay: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRandomDelay: unsafe extern "system" fn(this: *mut *mut Self, randomdelay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRandomDelay: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IEmailAction {
    pub base__: IAction,
    #[cfg(feature = "Win32_Foundation")]
    pub Server: unsafe extern "system" fn(this: *mut *mut Self, pserver: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Server: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetServer: unsafe extern "system" fn(this: *mut *mut Self, server: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetServer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Subject: unsafe extern "system" fn(this: *mut *mut Self, psubject: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Subject: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSubject: unsafe extern "system" fn(this: *mut *mut Self, subject: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSubject: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub To: unsafe extern "system" fn(this: *mut *mut Self, pto: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    To: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTo: unsafe extern "system" fn(this: *mut *mut Self, to: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Cc: unsafe extern "system" fn(this: *mut *mut Self, pcc: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Cc: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCc: unsafe extern "system" fn(this: *mut *mut Self, cc: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCc: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Bcc: unsafe extern "system" fn(this: *mut *mut Self, pbcc: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Bcc: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBcc: unsafe extern "system" fn(this: *mut *mut Self, bcc: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBcc: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReplyTo: unsafe extern "system" fn(this: *mut *mut Self, preplyto: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReplyTo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetReplyTo: unsafe extern "system" fn(this: *mut *mut Self, replyto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetReplyTo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub From: unsafe extern "system" fn(this: *mut *mut Self, pfrom: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    From: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFrom: unsafe extern "system" fn(this: *mut *mut Self, from: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFrom: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub HeaderFields: unsafe extern "system" fn(this: *mut *mut Self, ppheaderfields: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    HeaderFields: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetHeaderFields: unsafe extern "system" fn(this: *mut *mut Self, pheaderfields: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetHeaderFields: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Body: unsafe extern "system" fn(this: *mut *mut Self, pbody: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Body: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBody: unsafe extern "system" fn(this: *mut *mut Self, body: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBody: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Attachments: unsafe extern "system" fn(this: *mut *mut Self, pattachements: *mut *mut super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Attachments: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetAttachments: unsafe extern "system" fn(this: *mut *mut Self, pattachements: *mut super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetAttachments: usize,
}
#[repr(C)]
pub struct IEnumWorkItems {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, rgpwsznames: *mut *mut ::windows_sys::core::PWSTR, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenumworkitems: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IEventTrigger {
    pub base__: ITrigger,
    #[cfg(feature = "Win32_Foundation")]
    pub Subscription: unsafe extern "system" fn(this: *mut *mut Self, pquery: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Subscription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSubscription: unsafe extern "system" fn(this: *mut *mut Self, query: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSubscription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Delay: unsafe extern "system" fn(this: *mut *mut Self, pdelay: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Delay: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDelay: unsafe extern "system" fn(this: *mut *mut Self, delay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDelay: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ValueQueries: unsafe extern "system" fn(this: *mut *mut Self, ppnamedxpaths: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ValueQueries: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetValueQueries: unsafe extern "system" fn(this: *mut *mut Self, pnamedxpaths: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetValueQueries: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IExecAction {
    pub base__: IAction,
    #[cfg(feature = "Win32_Foundation")]
    pub Path: unsafe extern "system" fn(this: *mut *mut Self, ppath: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Path: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPath: unsafe extern "system" fn(this: *mut *mut Self, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Arguments: unsafe extern "system" fn(this: *mut *mut Self, pargument: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Arguments: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetArguments: unsafe extern "system" fn(this: *mut *mut Self, argument: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetArguments: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub WorkingDirectory: unsafe extern "system" fn(this: *mut *mut Self, pworkingdirectory: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WorkingDirectory: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetWorkingDirectory: unsafe extern "system" fn(this: *mut *mut Self, workingdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetWorkingDirectory: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IExecAction2 {
    pub base__: IExecAction,
    pub HideAppWindow: unsafe extern "system" fn(this: *mut *mut Self, phideappwindow: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetHideAppWindow: unsafe extern "system" fn(this: *mut *mut Self, hideappwindow: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IIdleSettings {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub IdleDuration: unsafe extern "system" fn(this: *mut *mut Self, pdelay: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IdleDuration: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIdleDuration: unsafe extern "system" fn(this: *mut *mut Self, delay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIdleDuration: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub WaitTimeout: unsafe extern "system" fn(this: *mut *mut Self, ptimeout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WaitTimeout: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetWaitTimeout: unsafe extern "system" fn(this: *mut *mut Self, timeout: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetWaitTimeout: usize,
    pub StopOnIdleEnd: unsafe extern "system" fn(this: *mut *mut Self, pstop: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetStopOnIdleEnd: unsafe extern "system" fn(this: *mut *mut Self, stop: i16) -> ::windows_sys::core::HRESULT,
    pub RestartOnIdle: unsafe extern "system" fn(this: *mut *mut Self, prestart: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetRestartOnIdle: unsafe extern "system" fn(this: *mut *mut Self, restart: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IIdleTrigger {
    pub base__: ITrigger,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ILogonTrigger {
    pub base__: ITrigger,
    #[cfg(feature = "Win32_Foundation")]
    pub Delay: unsafe extern "system" fn(this: *mut *mut Self, pdelay: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Delay: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDelay: unsafe extern "system" fn(this: *mut *mut Self, delay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDelay: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UserId: unsafe extern "system" fn(this: *mut *mut Self, puser: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UserId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUserId: unsafe extern "system" fn(this: *mut *mut Self, user: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUserId: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMaintenanceSettings {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPeriod: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPeriod: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Period: unsafe extern "system" fn(this: *mut *mut Self, target: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Period: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDeadline: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDeadline: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Deadline: unsafe extern "system" fn(this: *mut *mut Self, target: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Deadline: usize,
    pub SetExclusive: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    pub Exclusive: unsafe extern "system" fn(this: *mut *mut Self, target: *mut i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMonthlyDOWTrigger {
    pub base__: ITrigger,
    pub DaysOfWeek: unsafe extern "system" fn(this: *mut *mut Self, pdays: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetDaysOfWeek: unsafe extern "system" fn(this: *mut *mut Self, days: i16) -> ::windows_sys::core::HRESULT,
    pub WeeksOfMonth: unsafe extern "system" fn(this: *mut *mut Self, pweeks: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetWeeksOfMonth: unsafe extern "system" fn(this: *mut *mut Self, weeks: i16) -> ::windows_sys::core::HRESULT,
    pub MonthsOfYear: unsafe extern "system" fn(this: *mut *mut Self, pmonths: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetMonthsOfYear: unsafe extern "system" fn(this: *mut *mut Self, months: i16) -> ::windows_sys::core::HRESULT,
    pub RunOnLastWeekOfMonth: unsafe extern "system" fn(this: *mut *mut Self, plastweek: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetRunOnLastWeekOfMonth: unsafe extern "system" fn(this: *mut *mut Self, lastweek: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RandomDelay: unsafe extern "system" fn(this: *mut *mut Self, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RandomDelay: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRandomDelay: unsafe extern "system" fn(this: *mut *mut Self, randomdelay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRandomDelay: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMonthlyTrigger {
    pub base__: ITrigger,
    pub DaysOfMonth: unsafe extern "system" fn(this: *mut *mut Self, pdays: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetDaysOfMonth: unsafe extern "system" fn(this: *mut *mut Self, days: i32) -> ::windows_sys::core::HRESULT,
    pub MonthsOfYear: unsafe extern "system" fn(this: *mut *mut Self, pmonths: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetMonthsOfYear: unsafe extern "system" fn(this: *mut *mut Self, months: i16) -> ::windows_sys::core::HRESULT,
    pub RunOnLastDayOfMonth: unsafe extern "system" fn(this: *mut *mut Self, plastday: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetRunOnLastDayOfMonth: unsafe extern "system" fn(this: *mut *mut Self, lastday: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RandomDelay: unsafe extern "system" fn(this: *mut *mut Self, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RandomDelay: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRandomDelay: unsafe extern "system" fn(this: *mut *mut Self, randomdelay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRandomDelay: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetworkSettings {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, pid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Id: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetId: unsafe extern "system" fn(this: *mut *mut Self, id: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetId: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IPrincipal {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, pid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Id: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetId: unsafe extern "system" fn(this: *mut *mut Self, id: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, pname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDisplayName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UserId: unsafe extern "system" fn(this: *mut *mut Self, puser: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UserId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUserId: unsafe extern "system" fn(this: *mut *mut Self, user: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUserId: usize,
    pub LogonType: unsafe extern "system" fn(this: *mut *mut Self, plogon: *mut TASK_LOGON_TYPE) -> ::windows_sys::core::HRESULT,
    pub SetLogonType: unsafe extern "system" fn(this: *mut *mut Self, logon: TASK_LOGON_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GroupId: unsafe extern "system" fn(this: *mut *mut Self, pgroup: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GroupId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetGroupId: unsafe extern "system" fn(this: *mut *mut Self, group: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetGroupId: usize,
    pub RunLevel: unsafe extern "system" fn(this: *mut *mut Self, prunlevel: *mut TASK_RUNLEVEL_TYPE) -> ::windows_sys::core::HRESULT,
    pub SetRunLevel: unsafe extern "system" fn(this: *mut *mut Self, runlevel: TASK_RUNLEVEL_TYPE) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IPrincipal2 {
    pub base__: super::Com::IDispatch,
    pub ProcessTokenSidType: unsafe extern "system" fn(this: *mut *mut Self, pprocesstokensidtype: *mut TASK_PROCESSTOKENSID_TYPE) -> ::windows_sys::core::HRESULT,
    pub SetProcessTokenSidType: unsafe extern "system" fn(this: *mut *mut Self, processtokensidtype: TASK_PROCESSTOKENSID_TYPE) -> ::windows_sys::core::HRESULT,
    pub RequiredPrivilegeCount: unsafe extern "system" fn(this: *mut *mut Self, pcount: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub get_RequiredPrivilege: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pprivilege: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_RequiredPrivilege: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AddRequiredPrivilege: unsafe extern "system" fn(this: *mut *mut Self, privilege: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddRequiredPrivilege: usize,
}
#[repr(C)]
pub struct IProvideTaskPage {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub GetPage: unsafe extern "system" fn(this: *mut *mut Self, tptype: TASKPAGE, fpersistchanges: super::super::Foundation::BOOL, phpage: *mut super::super::UI::Controls::HPROPSHEETPAGE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls")))]
    GetPage: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRegisteredTask {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Path: unsafe extern "system" fn(this: *mut *mut Self, ppath: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Path: usize,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, pstate: *mut TASK_STATE) -> ::windows_sys::core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, penabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, enabled: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Run: unsafe extern "system" fn(this: *mut *mut Self, params: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pprunningtask: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Run: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RunEx: unsafe extern "system" fn(this: *mut *mut Self, params: ::core::mem::ManuallyDrop<super::Com::VARIANT>, flags: i32, sessionid: i32, user: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pprunningtask: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RunEx: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetInstances: unsafe extern "system" fn(this: *mut *mut Self, flags: i32, pprunningtasks: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetInstances: usize,
    pub LastRunTime: unsafe extern "system" fn(this: *mut *mut Self, plastruntime: *mut f64) -> ::windows_sys::core::HRESULT,
    pub LastTaskResult: unsafe extern "system" fn(this: *mut *mut Self, plasttaskresult: *mut i32) -> ::windows_sys::core::HRESULT,
    pub NumberOfMissedRuns: unsafe extern "system" fn(this: *mut *mut Self, pnumberofmissedruns: *mut i32) -> ::windows_sys::core::HRESULT,
    pub NextRunTime: unsafe extern "system" fn(this: *mut *mut Self, pnextruntime: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Definition: unsafe extern "system" fn(this: *mut *mut Self, ppdefinition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Definition: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Xml: unsafe extern "system" fn(this: *mut *mut Self, pxml: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Xml: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSecurityDescriptor: unsafe extern "system" fn(this: *mut *mut Self, securityinformation: i32, psddl: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSecurityDescriptor: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSecurityDescriptor: unsafe extern "system" fn(this: *mut *mut Self, sddl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSecurityDescriptor: usize,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self, flags: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRunTimes: unsafe extern "system" fn(this: *mut *mut Self, pststart: *const super::super::Foundation::SYSTEMTIME, pstend: *const super::super::Foundation::SYSTEMTIME, pcount: *mut u32, pruntimes: *mut *mut super::super::Foundation::SYSTEMTIME) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRunTimes: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRegisteredTaskCollection {
    pub base__: super::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pcount: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppregisteredtask: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRegistrationInfo {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, pdescription: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Author: unsafe extern "system" fn(this: *mut *mut Self, pauthor: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Author: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAuthor: unsafe extern "system" fn(this: *mut *mut Self, author: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAuthor: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Version: unsafe extern "system" fn(this: *mut *mut Self, pversion: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Version: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetVersion: unsafe extern "system" fn(this: *mut *mut Self, version: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetVersion: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Date: unsafe extern "system" fn(this: *mut *mut Self, pdate: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Date: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDate: unsafe extern "system" fn(this: *mut *mut Self, date: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Documentation: unsafe extern "system" fn(this: *mut *mut Self, pdocumentation: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Documentation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDocumentation: unsafe extern "system" fn(this: *mut *mut Self, documentation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDocumentation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub XmlText: unsafe extern "system" fn(this: *mut *mut Self, ptext: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    XmlText: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetXmlText: unsafe extern "system" fn(this: *mut *mut Self, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetXmlText: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub URI: unsafe extern "system" fn(this: *mut *mut Self, puri: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    URI: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetURI: unsafe extern "system" fn(this: *mut *mut Self, uri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetURI: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SecurityDescriptor: unsafe extern "system" fn(this: *mut *mut Self, psddl: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SecurityDescriptor: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSecurityDescriptor: unsafe extern "system" fn(this: *mut *mut Self, sddl: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSecurityDescriptor: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, psource: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Source: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSource: unsafe extern "system" fn(this: *mut *mut Self, source: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSource: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRegistrationTrigger {
    pub base__: ITrigger,
    #[cfg(feature = "Win32_Foundation")]
    pub Delay: unsafe extern "system" fn(this: *mut *mut Self, pdelay: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Delay: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDelay: unsafe extern "system" fn(this: *mut *mut Self, delay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDelay: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRepetitionPattern {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Interval: unsafe extern "system" fn(this: *mut *mut Self, pinterval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Interval: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetInterval: unsafe extern "system" fn(this: *mut *mut Self, interval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetInterval: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, pduration: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Duration: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDuration: unsafe extern "system" fn(this: *mut *mut Self, duration: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDuration: usize,
    pub StopAtDurationEnd: unsafe extern "system" fn(this: *mut *mut Self, pstop: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetStopAtDurationEnd: unsafe extern "system" fn(this: *mut *mut Self, stop: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRunningTask {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InstanceGuid: unsafe extern "system" fn(this: *mut *mut Self, pguid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InstanceGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Path: unsafe extern "system" fn(this: *mut *mut Self, ppath: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Path: usize,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, pstate: *mut TASK_STATE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentAction: unsafe extern "system" fn(this: *mut *mut Self, pname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentAction: usize,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub EnginePID: unsafe extern "system" fn(this: *mut *mut Self, ppid: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRunningTaskCollection {
    pub base__: super::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pcount: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pprunningtask: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IScheduledWorkItem {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateTrigger: unsafe extern "system" fn(this: *mut *mut Self, pinewtrigger: *mut u16, pptrigger: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DeleteTrigger: unsafe extern "system" fn(this: *mut *mut Self, itrigger: u16) -> ::windows_sys::core::HRESULT,
    pub GetTriggerCount: unsafe extern "system" fn(this: *mut *mut Self, pwcount: *mut u16) -> ::windows_sys::core::HRESULT,
    pub GetTrigger: unsafe extern "system" fn(this: *mut *mut Self, itrigger: u16, pptrigger: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetTriggerString: unsafe extern "system" fn(this: *mut *mut Self, itrigger: u16, ppwsztrigger: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRunTimes: unsafe extern "system" fn(this: *mut *mut Self, pstbegin: *const super::super::Foundation::SYSTEMTIME, pstend: *const super::super::Foundation::SYSTEMTIME, pcount: *mut u16, rgsttasktimes: *mut *mut super::super::Foundation::SYSTEMTIME) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRunTimes: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetNextRunTime: unsafe extern "system" fn(this: *mut *mut Self, pstnextrun: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetNextRunTime: usize,
    pub SetIdleWait: unsafe extern "system" fn(this: *mut *mut Self, widleminutes: u16, wdeadlineminutes: u16) -> ::windows_sys::core::HRESULT,
    pub GetIdleWait: unsafe extern "system" fn(this: *mut *mut Self, pwidleminutes: *mut u16, pwdeadlineminutes: *mut u16) -> ::windows_sys::core::HRESULT,
    pub Run: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Terminate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EditWorkItem: unsafe extern "system" fn(this: *mut *mut Self, hparent: super::super::Foundation::HWND, dwreserved: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EditWorkItem: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMostRecentRunTime: unsafe extern "system" fn(this: *mut *mut Self, pstlastrun: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMostRecentRunTime: usize,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, phrstatus: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub GetExitCode: unsafe extern "system" fn(this: *mut *mut Self, pdwexitcode: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetComment: unsafe extern "system" fn(this: *mut *mut Self, pwszcomment: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetComment: unsafe extern "system" fn(this: *mut *mut Self, ppwszcomment: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetCreator: unsafe extern "system" fn(this: *mut *mut Self, pwszcreator: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetCreator: unsafe extern "system" fn(this: *mut *mut Self, ppwszcreator: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetWorkItemData: unsafe extern "system" fn(this: *mut *mut Self, cbdata: u16, rgbdata: *const u8) -> ::windows_sys::core::HRESULT,
    pub GetWorkItemData: unsafe extern "system" fn(this: *mut *mut Self, pcbdata: *mut u16, prgbdata: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetErrorRetryCount: unsafe extern "system" fn(this: *mut *mut Self, wretrycount: u16) -> ::windows_sys::core::HRESULT,
    pub GetErrorRetryCount: unsafe extern "system" fn(this: *mut *mut Self, pwretrycount: *mut u16) -> ::windows_sys::core::HRESULT,
    pub SetErrorRetryInterval: unsafe extern "system" fn(this: *mut *mut Self, wretryinterval: u16) -> ::windows_sys::core::HRESULT,
    pub GetErrorRetryInterval: unsafe extern "system" fn(this: *mut *mut Self, pwretryinterval: *mut u16) -> ::windows_sys::core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32) -> ::windows_sys::core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(this: *mut *mut Self, pdwflags: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetAccountInformation: unsafe extern "system" fn(this: *mut *mut Self, pwszaccountname: ::windows_sys::core::PCWSTR, pwszpassword: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetAccountInformation: unsafe extern "system" fn(this: *mut *mut Self, ppwszaccountname: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISessionStateChangeTrigger {
    pub base__: ITrigger,
    #[cfg(feature = "Win32_Foundation")]
    pub Delay: unsafe extern "system" fn(this: *mut *mut Self, pdelay: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Delay: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDelay: unsafe extern "system" fn(this: *mut *mut Self, delay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDelay: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UserId: unsafe extern "system" fn(this: *mut *mut Self, puser: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UserId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUserId: unsafe extern "system" fn(this: *mut *mut Self, user: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUserId: usize,
    pub StateChange: unsafe extern "system" fn(this: *mut *mut Self, ptype: *mut TASK_SESSION_STATE_CHANGE_TYPE) -> ::windows_sys::core::HRESULT,
    pub SetStateChange: unsafe extern "system" fn(this: *mut *mut Self, r#type: TASK_SESSION_STATE_CHANGE_TYPE) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IShowMessageAction {
    pub base__: IAction,
    #[cfg(feature = "Win32_Foundation")]
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, ptitle: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Title: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTitle: unsafe extern "system" fn(this: *mut *mut Self, title: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTitle: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MessageBody: unsafe extern "system" fn(this: *mut *mut Self, pmessagebody: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MessageBody: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMessageBody: unsafe extern "system" fn(this: *mut *mut Self, messagebody: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMessageBody: usize,
}
#[repr(C)]
pub struct ITask {
    pub base__: IScheduledWorkItem,
    pub SetApplicationName: unsafe extern "system" fn(this: *mut *mut Self, pwszapplicationname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetApplicationName: unsafe extern "system" fn(this: *mut *mut Self, ppwszapplicationname: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetParameters: unsafe extern "system" fn(this: *mut *mut Self, pwszparameters: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetParameters: unsafe extern "system" fn(this: *mut *mut Self, ppwszparameters: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetWorkingDirectory: unsafe extern "system" fn(this: *mut *mut Self, pwszworkingdirectory: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetWorkingDirectory: unsafe extern "system" fn(this: *mut *mut Self, ppwszworkingdirectory: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut *mut Self, dwpriority: u32) -> ::windows_sys::core::HRESULT,
    pub GetPriority: unsafe extern "system" fn(this: *mut *mut Self, pdwpriority: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetTaskFlags: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32) -> ::windows_sys::core::HRESULT,
    pub GetTaskFlags: unsafe extern "system" fn(this: *mut *mut Self, pdwflags: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetMaxRunTime: unsafe extern "system" fn(this: *mut *mut Self, dwmaxruntimems: u32) -> ::windows_sys::core::HRESULT,
    pub GetMaxRunTime: unsafe extern "system" fn(this: *mut *mut Self, pdwmaxruntimems: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITaskDefinition {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub RegistrationInfo: unsafe extern "system" fn(this: *mut *mut Self, ppregistrationinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RegistrationInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetRegistrationInfo: unsafe extern "system" fn(this: *mut *mut Self, pregistrationinfo: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetRegistrationInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Triggers: unsafe extern "system" fn(this: *mut *mut Self, pptriggers: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Triggers: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetTriggers: unsafe extern "system" fn(this: *mut *mut Self, ptriggers: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetTriggers: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Settings: unsafe extern "system" fn(this: *mut *mut Self, ppsettings: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Settings: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSettings: unsafe extern "system" fn(this: *mut *mut Self, psettings: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSettings: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, pdata: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Data: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetData: unsafe extern "system" fn(this: *mut *mut Self, data: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetData: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Principal: unsafe extern "system" fn(this: *mut *mut Self, ppprincipal: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Principal: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetPrincipal: unsafe extern "system" fn(this: *mut *mut Self, pprincipal: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetPrincipal: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Actions: unsafe extern "system" fn(this: *mut *mut Self, ppactions: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Actions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetActions: unsafe extern "system" fn(this: *mut *mut Self, pactions: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetActions: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub XmlText: unsafe extern "system" fn(this: *mut *mut Self, pxml: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    XmlText: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetXmlText: unsafe extern "system" fn(this: *mut *mut Self, xml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetXmlText: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITaskFolder {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Path: unsafe extern "system" fn(this: *mut *mut Self, ppath: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Path: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetFolder: unsafe extern "system" fn(this: *mut *mut Self, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfolder: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetFolder: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFolders: unsafe extern "system" fn(this: *mut *mut Self, flags: i32, ppfolders: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFolders: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateFolder: unsafe extern "system" fn(this: *mut *mut Self, subfoldername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sddl: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppfolder: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateFolder: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteFolder: unsafe extern "system" fn(this: *mut *mut Self, subfoldername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteFolder: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetTask: unsafe extern "system" fn(this: *mut *mut Self, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptask: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetTask: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetTasks: unsafe extern "system" fn(this: *mut *mut Self, flags: i32, pptasks: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetTasks: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteTask: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteTask: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RegisterTask: unsafe extern "system" fn(this: *mut *mut Self, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, xmltext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32, userid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, password: ::core::mem::ManuallyDrop<super::Com::VARIANT>, logontype: TASK_LOGON_TYPE, sddl: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pptask: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RegisterTask: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RegisterTaskDefinition: unsafe extern "system" fn(this: *mut *mut Self, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdefinition: *mut ::core::ffi::c_void, flags: i32, userid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, password: ::core::mem::ManuallyDrop<super::Com::VARIANT>, logontype: TASK_LOGON_TYPE, sddl: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pptask: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RegisterTaskDefinition: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSecurityDescriptor: unsafe extern "system" fn(this: *mut *mut Self, securityinformation: i32, psddl: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSecurityDescriptor: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSecurityDescriptor: unsafe extern "system" fn(this: *mut *mut Self, sddl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSecurityDescriptor: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITaskFolderCollection {
    pub base__: super::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pcount: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppfolder: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITaskHandler {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Start: unsafe extern "system" fn(this: *mut *mut Self, phandlerservices: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Start: usize,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self, pretcode: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITaskHandlerStatus {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub UpdateStatus: unsafe extern "system" fn(this: *mut *mut Self, percentcomplete: i16, statusmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UpdateStatus: usize,
    pub TaskCompleted: unsafe extern "system" fn(this: *mut *mut Self, taskerrcode: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITaskNamedValueCollection {
    pub base__: super::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pcount: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pppair: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pppair: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Create: usize,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, index: i32) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITaskNamedValuePair {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Value: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetValue: usize,
}
#[repr(C)]
pub struct ITaskScheduler {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetTargetComputer: unsafe extern "system" fn(this: *mut *mut Self, pwszcomputer: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetTargetComputer: unsafe extern "system" fn(this: *mut *mut Self, ppwszcomputer: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub Enum: unsafe extern "system" fn(this: *mut *mut Self, ppenumworkitems: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Activate: unsafe extern "system" fn(this: *mut *mut Self, pwszname: ::windows_sys::core::PCWSTR, riid: *const ::windows_sys::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self, pwszname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub NewWorkItem: unsafe extern "system" fn(this: *mut *mut Self, pwsztaskname: ::windows_sys::core::PCWSTR, rclsid: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AddWorkItem: unsafe extern "system" fn(this: *mut *mut Self, pwsztaskname: ::windows_sys::core::PCWSTR, pworkitem: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsOfType: unsafe extern "system" fn(this: *mut *mut Self, pwszname: ::windows_sys::core::PCWSTR, riid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITaskService {
    pub base__: super::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetFolder: unsafe extern "system" fn(this: *mut *mut Self, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfolder: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetFolder: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRunningTasks: unsafe extern "system" fn(this: *mut *mut Self, flags: i32, pprunningtasks: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRunningTasks: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub NewTask: unsafe extern "system" fn(this: *mut *mut Self, flags: u32, ppdefinition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NewTask: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Connect: unsafe extern "system" fn(this: *mut *mut Self, servername: ::core::mem::ManuallyDrop<super::Com::VARIANT>, user: ::core::mem::ManuallyDrop<super::Com::VARIANT>, domain: ::core::mem::ManuallyDrop<super::Com::VARIANT>, password: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Connect: usize,
    pub Connected: unsafe extern "system" fn(this: *mut *mut Self, pconnected: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TargetServer: unsafe extern "system" fn(this: *mut *mut Self, pserver: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TargetServer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ConnectedUser: unsafe extern "system" fn(this: *mut *mut Self, puser: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConnectedUser: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ConnectedDomain: unsafe extern "system" fn(this: *mut *mut Self, pdomain: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConnectedDomain: usize,
    pub HighestVersion: unsafe extern "system" fn(this: *mut *mut Self, pversion: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITaskSettings {
    pub base__: super::Com::IDispatch,
    pub AllowDemandStart: unsafe extern "system" fn(this: *mut *mut Self, pallowdemandstart: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAllowDemandStart: unsafe extern "system" fn(this: *mut *mut Self, allowdemandstart: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RestartInterval: unsafe extern "system" fn(this: *mut *mut Self, prestartinterval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RestartInterval: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRestartInterval: unsafe extern "system" fn(this: *mut *mut Self, restartinterval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRestartInterval: usize,
    pub RestartCount: unsafe extern "system" fn(this: *mut *mut Self, prestartcount: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetRestartCount: unsafe extern "system" fn(this: *mut *mut Self, restartcount: i32) -> ::windows_sys::core::HRESULT,
    pub MultipleInstances: unsafe extern "system" fn(this: *mut *mut Self, ppolicy: *mut TASK_INSTANCES_POLICY) -> ::windows_sys::core::HRESULT,
    pub SetMultipleInstances: unsafe extern "system" fn(this: *mut *mut Self, policy: TASK_INSTANCES_POLICY) -> ::windows_sys::core::HRESULT,
    pub StopIfGoingOnBatteries: unsafe extern "system" fn(this: *mut *mut Self, pstopifonbatteries: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetStopIfGoingOnBatteries: unsafe extern "system" fn(this: *mut *mut Self, stopifonbatteries: i16) -> ::windows_sys::core::HRESULT,
    pub DisallowStartIfOnBatteries: unsafe extern "system" fn(this: *mut *mut Self, pdisallowstart: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetDisallowStartIfOnBatteries: unsafe extern "system" fn(this: *mut *mut Self, disallowstart: i16) -> ::windows_sys::core::HRESULT,
    pub AllowHardTerminate: unsafe extern "system" fn(this: *mut *mut Self, pallowhardterminate: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAllowHardTerminate: unsafe extern "system" fn(this: *mut *mut Self, allowhardterminate: i16) -> ::windows_sys::core::HRESULT,
    pub StartWhenAvailable: unsafe extern "system" fn(this: *mut *mut Self, pstartwhenavailable: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetStartWhenAvailable: unsafe extern "system" fn(this: *mut *mut Self, startwhenavailable: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub XmlText: unsafe extern "system" fn(this: *mut *mut Self, ptext: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    XmlText: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetXmlText: unsafe extern "system" fn(this: *mut *mut Self, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetXmlText: usize,
    pub RunOnlyIfNetworkAvailable: unsafe extern "system" fn(this: *mut *mut Self, prunonlyifnetworkavailable: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetRunOnlyIfNetworkAvailable: unsafe extern "system" fn(this: *mut *mut Self, runonlyifnetworkavailable: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ExecutionTimeLimit: unsafe extern "system" fn(this: *mut *mut Self, pexecutiontimelimit: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExecutionTimeLimit: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetExecutionTimeLimit: unsafe extern "system" fn(this: *mut *mut Self, executiontimelimit: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetExecutionTimeLimit: usize,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, penabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, enabled: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteExpiredTaskAfter: unsafe extern "system" fn(this: *mut *mut Self, pexpirationdelay: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteExpiredTaskAfter: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDeleteExpiredTaskAfter: unsafe extern "system" fn(this: *mut *mut Self, expirationdelay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDeleteExpiredTaskAfter: usize,
    pub Priority: unsafe extern "system" fn(this: *mut *mut Self, ppriority: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut *mut Self, priority: i32) -> ::windows_sys::core::HRESULT,
    pub Compatibility: unsafe extern "system" fn(this: *mut *mut Self, pcompatlevel: *mut TASK_COMPATIBILITY) -> ::windows_sys::core::HRESULT,
    pub SetCompatibility: unsafe extern "system" fn(this: *mut *mut Self, compatlevel: TASK_COMPATIBILITY) -> ::windows_sys::core::HRESULT,
    pub Hidden: unsafe extern "system" fn(this: *mut *mut Self, phidden: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetHidden: unsafe extern "system" fn(this: *mut *mut Self, hidden: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub IdleSettings: unsafe extern "system" fn(this: *mut *mut Self, ppidlesettings: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IdleSettings: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetIdleSettings: unsafe extern "system" fn(this: *mut *mut Self, pidlesettings: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetIdleSettings: usize,
    pub RunOnlyIfIdle: unsafe extern "system" fn(this: *mut *mut Self, prunonlyifidle: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetRunOnlyIfIdle: unsafe extern "system" fn(this: *mut *mut Self, runonlyifidle: i16) -> ::windows_sys::core::HRESULT,
    pub WakeToRun: unsafe extern "system" fn(this: *mut *mut Self, pwake: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetWakeToRun: unsafe extern "system" fn(this: *mut *mut Self, wake: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub NetworkSettings: unsafe extern "system" fn(this: *mut *mut Self, ppnetworksettings: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NetworkSettings: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetNetworkSettings: unsafe extern "system" fn(this: *mut *mut Self, pnetworksettings: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetNetworkSettings: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITaskSettings2 {
    pub base__: super::Com::IDispatch,
    pub DisallowStartOnRemoteAppSession: unsafe extern "system" fn(this: *mut *mut Self, pdisallowstart: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetDisallowStartOnRemoteAppSession: unsafe extern "system" fn(this: *mut *mut Self, disallowstart: i16) -> ::windows_sys::core::HRESULT,
    pub UseUnifiedSchedulingEngine: unsafe extern "system" fn(this: *mut *mut Self, puseunifiedengine: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetUseUnifiedSchedulingEngine: unsafe extern "system" fn(this: *mut *mut Self, useunifiedengine: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITaskSettings3 {
    pub base__: ITaskSettings,
    pub DisallowStartOnRemoteAppSession: unsafe extern "system" fn(this: *mut *mut Self, pdisallowstart: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetDisallowStartOnRemoteAppSession: unsafe extern "system" fn(this: *mut *mut Self, disallowstart: i16) -> ::windows_sys::core::HRESULT,
    pub UseUnifiedSchedulingEngine: unsafe extern "system" fn(this: *mut *mut Self, puseunifiedengine: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetUseUnifiedSchedulingEngine: unsafe extern "system" fn(this: *mut *mut Self, useunifiedengine: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub MaintenanceSettings: unsafe extern "system" fn(this: *mut *mut Self, ppmaintenancesettings: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    MaintenanceSettings: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetMaintenanceSettings: unsafe extern "system" fn(this: *mut *mut Self, pmaintenancesettings: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetMaintenanceSettings: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateMaintenanceSettings: unsafe extern "system" fn(this: *mut *mut Self, ppmaintenancesettings: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateMaintenanceSettings: usize,
    pub Volatile: unsafe extern "system" fn(this: *mut *mut Self, pvolatile: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetVolatile: unsafe extern "system" fn(this: *mut *mut Self, volatile: i16) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITaskTrigger {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetTrigger: unsafe extern "system" fn(this: *mut *mut Self, ptrigger: *const TASK_TRIGGER) -> ::windows_sys::core::HRESULT,
    pub GetTrigger: unsafe extern "system" fn(this: *mut *mut Self, ptrigger: *mut TASK_TRIGGER) -> ::windows_sys::core::HRESULT,
    pub GetTriggerString: unsafe extern "system" fn(this: *mut *mut Self, ppwsztrigger: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITaskVariables {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetInput: unsafe extern "system" fn(this: *mut *mut Self, pinput: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetInput: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetOutput: unsafe extern "system" fn(this: *mut *mut Self, input: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetOutput: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetContext: unsafe extern "system" fn(this: *mut *mut Self, pcontext: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetContext: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITimeTrigger {
    pub base__: ITrigger,
    #[cfg(feature = "Win32_Foundation")]
    pub RandomDelay: unsafe extern "system" fn(this: *mut *mut Self, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RandomDelay: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRandomDelay: unsafe extern "system" fn(this: *mut *mut Self, randomdelay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRandomDelay: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITrigger {
    pub base__: super::Com::IDispatch,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, ptype: *mut TASK_TRIGGER_TYPE2) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, pid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Id: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetId: unsafe extern "system" fn(this: *mut *mut Self, id: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetId: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Repetition: unsafe extern "system" fn(this: *mut *mut Self, pprepeat: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Repetition: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetRepetition: unsafe extern "system" fn(this: *mut *mut Self, prepeat: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetRepetition: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExecutionTimeLimit: unsafe extern "system" fn(this: *mut *mut Self, ptimelimit: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExecutionTimeLimit: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetExecutionTimeLimit: unsafe extern "system" fn(this: *mut *mut Self, timelimit: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetExecutionTimeLimit: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub StartBoundary: unsafe extern "system" fn(this: *mut *mut Self, pstart: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StartBoundary: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetStartBoundary: unsafe extern "system" fn(this: *mut *mut Self, start: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetStartBoundary: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EndBoundary: unsafe extern "system" fn(this: *mut *mut Self, pend: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EndBoundary: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEndBoundary: unsafe extern "system" fn(this: *mut *mut Self, end: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEndBoundary: usize,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, penabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, enabled: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITriggerCollection {
    pub base__: super::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pcount: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pptrigger: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, r#type: TASK_TRIGGER_TYPE2, pptrigger: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Create: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWeeklyTrigger {
    pub base__: ITrigger,
    pub DaysOfWeek: unsafe extern "system" fn(this: *mut *mut Self, pdays: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetDaysOfWeek: unsafe extern "system" fn(this: *mut *mut Self, days: i16) -> ::windows_sys::core::HRESULT,
    pub WeeksInterval: unsafe extern "system" fn(this: *mut *mut Self, pweeks: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetWeeksInterval: unsafe extern "system" fn(this: *mut *mut Self, weeks: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RandomDelay: unsafe extern "system" fn(this: *mut *mut Self, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RandomDelay: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRandomDelay: unsafe extern "system" fn(this: *mut *mut Self, randomdelay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRandomDelay: usize,
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub struct MONTHLYDATE {
    pub rgfDays: u32,
    pub rgfMonths: u16,
}
impl ::core::marker::Copy for MONTHLYDATE {}
impl ::core::clone::Clone for MONTHLYDATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub struct MONTHLYDOW {
    pub wWhichWeek: u16,
    pub rgfDaysOfTheWeek: u16,
    pub rgfMonths: u16,
}
impl ::core::marker::Copy for MONTHLYDOW {}
impl ::core::clone::Clone for MONTHLYDOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub type TASKPAGE = i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASKPAGE_TASK: TASKPAGE = 0i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASKPAGE_SCHEDULE: TASKPAGE = 1i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASKPAGE_SETTINGS: TASKPAGE = 2i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub type TASK_ACTION_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_ACTION_EXEC: TASK_ACTION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_ACTION_COM_HANDLER: TASK_ACTION_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_ACTION_SEND_EMAIL: TASK_ACTION_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_ACTION_SHOW_MESSAGE: TASK_ACTION_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_APRIL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_AUGUST: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub type TASK_COMPATIBILITY = i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_COMPATIBILITY_AT: TASK_COMPATIBILITY = 0i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_COMPATIBILITY_V1: TASK_COMPATIBILITY = 1i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_COMPATIBILITY_V2: TASK_COMPATIBILITY = 2i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_COMPATIBILITY_V2_1: TASK_COMPATIBILITY = 3i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_COMPATIBILITY_V2_2: TASK_COMPATIBILITY = 4i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_COMPATIBILITY_V2_3: TASK_COMPATIBILITY = 5i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_COMPATIBILITY_V2_4: TASK_COMPATIBILITY = 6i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub type TASK_CREATION = i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_VALIDATE_ONLY: TASK_CREATION = 1i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_CREATE: TASK_CREATION = 2i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_UPDATE: TASK_CREATION = 4i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_CREATE_OR_UPDATE: TASK_CREATION = 6i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_DISABLE: TASK_CREATION = 8i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_DONT_ADD_PRINCIPAL_ACE: TASK_CREATION = 16i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_IGNORE_REGISTRATION_TRIGGERS: TASK_CREATION = 32i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_DECEMBER: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub type TASK_ENUM_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_ENUM_HIDDEN: TASK_ENUM_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FEBRUARY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FIRST_WEEK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_DELETE_WHEN_DONE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_DISABLED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_DONT_START_IF_ON_BATTERIES: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_HIDDEN: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_INTERACTIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_KILL_IF_GOING_ON_BATTERIES: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_KILL_ON_IDLE_END: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_RESTART_ON_IDLE_RESUME: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_RUN_IF_CONNECTED_TO_INTERNET: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_RUN_ONLY_IF_DOCKED: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_RUN_ONLY_IF_LOGGED_ON: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_START_ONLY_IF_IDLE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_SYSTEM_REQUIRED: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FOURTH_WEEK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FRIDAY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub type TASK_INSTANCES_POLICY = i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_INSTANCES_PARALLEL: TASK_INSTANCES_POLICY = 0i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_INSTANCES_QUEUE: TASK_INSTANCES_POLICY = 1i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_INSTANCES_IGNORE_NEW: TASK_INSTANCES_POLICY = 2i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_INSTANCES_STOP_EXISTING: TASK_INSTANCES_POLICY = 3i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_JANUARY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_JULY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_JUNE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_LAST_WEEK: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub type TASK_LOGON_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_LOGON_NONE: TASK_LOGON_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_LOGON_PASSWORD: TASK_LOGON_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_LOGON_S4U: TASK_LOGON_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_LOGON_INTERACTIVE_TOKEN: TASK_LOGON_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_LOGON_GROUP: TASK_LOGON_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_LOGON_SERVICE_ACCOUNT: TASK_LOGON_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_LOGON_INTERACTIVE_TOKEN_OR_PASSWORD: TASK_LOGON_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_MARCH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_MAX_RUN_TIMES: u32 = 1440u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_MAY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_MONDAY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_NOVEMBER: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_OCTOBER: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub type TASK_PROCESSTOKENSID_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_PROCESSTOKENSID_NONE: TASK_PROCESSTOKENSID_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_PROCESSTOKENSID_UNRESTRICTED: TASK_PROCESSTOKENSID_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_PROCESSTOKENSID_DEFAULT: TASK_PROCESSTOKENSID_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub type TASK_RUNLEVEL_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_RUNLEVEL_LUA: TASK_RUNLEVEL_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_RUNLEVEL_HIGHEST: TASK_RUNLEVEL_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub type TASK_RUN_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_RUN_NO_FLAGS: TASK_RUN_FLAGS = 0i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_RUN_AS_SELF: TASK_RUN_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_RUN_IGNORE_CONSTRAINTS: TASK_RUN_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_RUN_USE_SESSION_ID: TASK_RUN_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_RUN_USER_SID: TASK_RUN_FLAGS = 8i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_SATURDAY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_SECOND_WEEK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_SEPTEMBER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub type TASK_SESSION_STATE_CHANGE_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_CONSOLE_CONNECT: TASK_SESSION_STATE_CHANGE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_CONSOLE_DISCONNECT: TASK_SESSION_STATE_CHANGE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_REMOTE_CONNECT: TASK_SESSION_STATE_CHANGE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_REMOTE_DISCONNECT: TASK_SESSION_STATE_CHANGE_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_SESSION_LOCK: TASK_SESSION_STATE_CHANGE_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_SESSION_UNLOCK: TASK_SESSION_STATE_CHANGE_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub type TASK_STATE = i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_STATE_UNKNOWN: TASK_STATE = 0i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_STATE_DISABLED: TASK_STATE = 1i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_STATE_QUEUED: TASK_STATE = 2i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_STATE_READY: TASK_STATE = 3i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_STATE_RUNNING: TASK_STATE = 4i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_SUNDAY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_THIRD_WEEK: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_THURSDAY: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub struct TASK_TRIGGER {
    pub cbTriggerSize: u16,
    pub Reserved1: u16,
    pub wBeginYear: u16,
    pub wBeginMonth: u16,
    pub wBeginDay: u16,
    pub wEndYear: u16,
    pub wEndMonth: u16,
    pub wEndDay: u16,
    pub wStartHour: u16,
    pub wStartMinute: u16,
    pub MinutesDuration: u32,
    pub MinutesInterval: u32,
    pub rgFlags: u32,
    pub TriggerType: TASK_TRIGGER_TYPE,
    pub Type: TRIGGER_TYPE_UNION,
    pub Reserved2: u16,
    pub wRandomMinutesInterval: u16,
}
impl ::core::marker::Copy for TASK_TRIGGER {}
impl ::core::clone::Clone for TASK_TRIGGER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_FLAG_DISABLED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_FLAG_HAS_END_DATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_FLAG_KILL_AT_DURATION_END: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub type TASK_TRIGGER_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TIME_TRIGGER_ONCE: TASK_TRIGGER_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TIME_TRIGGER_DAILY: TASK_TRIGGER_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TIME_TRIGGER_WEEKLY: TASK_TRIGGER_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TIME_TRIGGER_MONTHLYDATE: TASK_TRIGGER_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TIME_TRIGGER_MONTHLYDOW: TASK_TRIGGER_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_EVENT_TRIGGER_ON_IDLE: TASK_TRIGGER_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_EVENT_TRIGGER_AT_SYSTEMSTART: TASK_TRIGGER_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_EVENT_TRIGGER_AT_LOGON: TASK_TRIGGER_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub type TASK_TRIGGER_TYPE2 = i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_EVENT: TASK_TRIGGER_TYPE2 = 0i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_TIME: TASK_TRIGGER_TYPE2 = 1i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_DAILY: TASK_TRIGGER_TYPE2 = 2i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_WEEKLY: TASK_TRIGGER_TYPE2 = 3i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_MONTHLY: TASK_TRIGGER_TYPE2 = 4i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_MONTHLYDOW: TASK_TRIGGER_TYPE2 = 5i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_IDLE: TASK_TRIGGER_TYPE2 = 6i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_REGISTRATION: TASK_TRIGGER_TYPE2 = 7i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_BOOT: TASK_TRIGGER_TYPE2 = 8i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_LOGON: TASK_TRIGGER_TYPE2 = 9i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_SESSION_STATE_CHANGE: TASK_TRIGGER_TYPE2 = 11i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_CUSTOM_TRIGGER_01: TASK_TRIGGER_TYPE2 = 12i32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TUESDAY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_WEDNESDAY: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub union TRIGGER_TYPE_UNION {
    pub Daily: DAILY,
    pub Weekly: WEEKLY,
    pub MonthlyDate: MONTHLYDATE,
    pub MonthlyDOW: MONTHLYDOW,
}
impl ::core::marker::Copy for TRIGGER_TYPE_UNION {}
impl ::core::clone::Clone for TRIGGER_TYPE_UNION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const TaskHandlerPS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4071005623, data2: 55852, data3: 17234, data4: [144, 102, 134, 254, 230, 218, 202, 201] };
pub const TaskHandlerStatusPS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2668963437, data2: 55226, data3: 18672, data4: [147, 193, 230, 137, 95, 111, 229, 172] };
pub const TaskScheduler: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 260519583, data2: 42213, data3: 19708, data4: [189, 62, 115, 230, 21, 69, 114, 221] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub struct WEEKLY {
    pub WeeksInterval: u16,
    pub rgfDaysOfTheWeek: u16,
}
impl ::core::marker::Copy for WEEKLY {}
impl ::core::clone::Clone for WEEKLY {
    fn clone(&self) -> Self {
        *self
    }
}
