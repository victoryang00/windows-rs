#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
    pub fn CloseIMsgSession(lpmsgsess: *mut _MSGSESS);
    #[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_AddressBook\"`*"]
    #[cfg(feature = "Win32_System_AddressBook")]
    pub fn GetAttribIMsgOnIStg(lpobject: *mut ::core::ffi::c_void, lpproptagarray: *mut super::super::System::AddressBook::SPropTagArray, lpppropattrarray: *mut *mut SPropAttrArray) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
    pub fn MapStorageSCode(stgscode: i32) -> i32;
    #[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_AddressBook\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OpenIMsgOnIStg(lpmsgsess: *mut _MSGSESS, lpallocatebuffer: super::super::System::AddressBook::LPALLOCATEBUFFER, lpallocatemore: super::super::System::AddressBook::LPALLOCATEMORE, lpfreebuffer: super::super::System::AddressBook::LPFREEBUFFER, lpmalloc: *mut *mut super::super::System::Com::IMalloc, lpmapisup: *mut ::core::ffi::c_void, lpstg: *mut *mut super::super::System::Com::StructuredStorage::IStorage, lpfmsgcallrelease: *mut MSGCALLRELEASE, ulcallerdata: u32, ulflags: u32, lppmsg: *mut *mut *mut super::super::System::AddressBook::IMessage) -> i32;
    #[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn OpenIMsgSession(lpmalloc: *mut *mut super::super::System::Com::IMalloc, ulflags: u32, lppmsgsess: *mut *mut _MSGSESS) -> i32;
    #[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_AddressBook\"`*"]
    #[cfg(feature = "Win32_System_AddressBook")]
    pub fn SetAttribIMsgOnIStg(lpobject: *mut ::core::ffi::c_void, lpproptags: *mut super::super::System::AddressBook::SPropTagArray, lppropattrs: *mut SPropAttrArray, lpppropproblems: *mut *mut super::super::System::AddressBook::SPropProblemArray) -> ::windows_sys::core::HRESULT;
}
pub const BlockRange: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3037186599, data2: 8708, data3: 4573, data4: [150, 106, 0, 26, 160, 27, 188, 88] };
pub const BlockRangeList: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3037186600, data2: 8708, data3: 4573, data4: [150, 106, 0, 26, 160, 27, 188, 88] };
pub const BootOptions: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 747904974, data2: 38747, data3: 22974, data4: [169, 96, 154, 42, 38, 40, 83, 165] };
pub const CATID_SMTP_DNSRESOLVERRECORDSINK: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3171631974, data2: 36355, data3: 4562, data4: [148, 246, 0, 192, 79, 121, 241, 214] };
pub const CATID_SMTP_DSN: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 582309681, data2: 62968, data3: 19747, data4: [189, 143, 135, 181, 35, 113, 167, 58] };
pub const CATID_SMTP_GET_AUX_DOMAIN_INFO_FLAGS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2231318154, data2: 64179, data3: 17367, data4: [188, 223, 105, 44, 91, 70, 230, 177] };
pub const CATID_SMTP_LOG: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2479924536, data2: 11294, data3: 19304, data4: [167, 201, 215, 58, 138, 166, 238, 151] };
pub const CATID_SMTP_MAXMSGSIZE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3958462942, data2: 42622, data3: 4562, data4: [148, 247, 0, 192, 79, 121, 241, 214] };
pub const CATID_SMTP_MSGTRACKLOG: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3336524458, data2: 32176, data3: 4562, data4: [148, 244, 0, 192, 79, 121, 241, 214] };
pub const CATID_SMTP_ON_BEFORE_DATA: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4133653650, data2: 3422, data3: 4562, data4: [170, 104, 0, 192, 79, 163, 91, 130] };
pub const CATID_SMTP_ON_INBOUND_COMMAND: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4133653645, data2: 3422, data3: 4562, data4: [170, 104, 0, 192, 79, 163, 91, 130] };
pub const CATID_SMTP_ON_MESSAGE_START: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4133653648, data2: 3422, data3: 4562, data4: [170, 104, 0, 192, 79, 163, 91, 130] };
pub const CATID_SMTP_ON_PER_RECIPIENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4133653649, data2: 3422, data3: 4562, data4: [170, 104, 0, 192, 79, 163, 91, 130] };
pub const CATID_SMTP_ON_SERVER_RESPONSE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4133653646, data2: 3422, data3: 4562, data4: [170, 104, 0, 192, 79, 163, 91, 130] };
pub const CATID_SMTP_ON_SESSION_END: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4133653651, data2: 3422, data3: 4562, data4: [170, 104, 0, 192, 79, 163, 91, 130] };
pub const CATID_SMTP_ON_SESSION_START: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4133653647, data2: 3422, data3: 4562, data4: [170, 104, 0, 192, 79, 163, 91, 130] };
pub const CATID_SMTP_STORE_DRIVER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1494702160, data2: 58675, data3: 4561, data4: [170, 103, 0, 192, 79, 163, 69, 246] };
pub const CATID_SMTP_TRANSPORT_CATEGORIZE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2516734627, data2: 2618, data3: 4562, data4: [158, 0, 0, 192, 79, 163, 34, 186] };
pub const CATID_SMTP_TRANSPORT_POSTCATEGORIZE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1987155540, data2: 1446, data3: 4562, data4: [157, 253, 0, 192, 79, 163, 34, 186] };
pub const CATID_SMTP_TRANSPORT_PRECATEGORIZE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2746022669, data2: 33791, data3: 4562, data4: [158, 20, 0, 192, 79, 163, 34, 186] };
pub const CATID_SMTP_TRANSPORT_ROUTER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 674509001, data2: 6224, data3: 4562, data4: [158, 3, 0, 192, 79, 163, 34, 186] };
pub const CATID_SMTP_TRANSPORT_SUBMISSION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4282165795, data2: 185, data3: 4562, data4: [157, 251, 0, 192, 79, 163, 34, 186] };
pub const CLSID_SmtpCat: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2990290359, data2: 37401, data3: 4562, data4: [158, 23, 0, 192, 79, 163, 34, 186] };
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct DDiscFormat2DataEvents {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Update: unsafe extern "system" fn(this: *mut *mut Self, object: *mut ::core::ffi::c_void, progress: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Update: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for DDiscFormat2DataEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801532, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct DDiscFormat2EraseEvents {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Update: unsafe extern "system" fn(this: *mut *mut Self, object: *mut ::core::ffi::c_void, elapsedseconds: i32, estimatedtotalseconds: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Update: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for DDiscFormat2EraseEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801530, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct DDiscFormat2RawCDEvents {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Update: unsafe extern "system" fn(this: *mut *mut Self, object: *mut ::core::ffi::c_void, progress: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Update: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for DDiscFormat2RawCDEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801538, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct DDiscFormat2TrackAtOnceEvents {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Update: unsafe extern "system" fn(this: *mut *mut Self, object: *mut ::core::ffi::c_void, progress: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Update: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for DDiscFormat2TrackAtOnceEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801535, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct DDiscMaster2Events {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub NotifyDeviceAdded: unsafe extern "system" fn(this: *mut *mut Self, object: *mut ::core::ffi::c_void, uniqueid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    NotifyDeviceAdded: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub NotifyDeviceRemoved: unsafe extern "system" fn(this: *mut *mut Self, object: *mut ::core::ffi::c_void, uniqueid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    NotifyDeviceRemoved: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for DDiscMaster2Events {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801521, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct DFileSystemImageEvents {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Update: unsafe extern "system" fn(this: *mut *mut Self, object: *mut ::core::ffi::c_void, currentfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, copiedsectors: i32, totalsectors: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Update: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for DFileSystemImageEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 747904991, data2: 38747, data3: 22974, data4: [169, 96, 154, 42, 38, 40, 83, 165] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct DFileSystemImageImportEvents {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub UpdateImport: unsafe extern "system" fn(this: *mut *mut Self, object: *mut ::core::ffi::c_void, filesystem: FsiFileSystems, currentitem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, importeddirectoryitems: i32, totaldirectoryitems: i32, importedfileitems: i32, totalfileitems: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    UpdateImport: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for DFileSystemImageImportEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3529257209, data2: 16519, data3: 17254, data4: [158, 36, 229, 91, 226, 134, 66, 75] };
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub type DISC_RECORDER_STATE_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RECORDER_BURNING: DISC_RECORDER_STATE_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RECORDER_DOING_NOTHING: DISC_RECORDER_STATE_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RECORDER_OPENED: DISC_RECORDER_STATE_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_DDISCFORMAT2DATAEVENTS_UPDATE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_DDISCFORMAT2RAWCDEVENTS_UPDATE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_DDISCFORMAT2TAOEVENTS_UPDATE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_DDISCMASTER2EVENTS_DEVICEADDED: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_DDISCMASTER2EVENTS_DEVICEREMOVED: u32 = 257u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_DFILESYSTEMIMAGEEVENTS_UPDATE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_DFILESYSTEMIMAGEIMPORTEVENTS_UPDATEIMPORT: u32 = 257u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_DWRITEENGINE2EVENTS_UPDATE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IBLOCKRANGELIST_BLOCKRANGES: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IBLOCKRANGE_ENDLBA: u32 = 257u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IBLOCKRANGE_STARTLBA: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_CURRENTACTION: u32 = 771u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_ELAPSEDTIME: u32 = 768u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_ESTIMATEDREMAININGTIME: u32 = 769u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_ESTIMATEDTOTALTIME: u32 = 770u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_BUFFERUNDERRUNFREEDISABLED: u32 = 257u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_CANCELWRITE: u32 = 513u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_CLIENTNAME: u32 = 272u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_CURRENTMEDIASTATUS: u32 = 262u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_CURRENTMEDIATYPE: u32 = 271u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_CURRENTROTATIONTYPEISPURECAV: u32 = 276u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_CURRENTWRITESPEED: u32 = 275u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_DISABLEDVDCOMPATIBILITYMODE: u32 = 270u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_FORCEMEDIATOBECLOSED: u32 = 269u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_FORCEOVERWRITE: u32 = 279u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_FREESECTORS: u32 = 265u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_LASTSECTOROFPREVIOUSSESSION: u32 = 268u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_MUTLISESSIONINTERFACES: u32 = 280u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_NEXTWRITABLEADDRESS: u32 = 266u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_POSTGAPALREADYINIMAGE: u32 = 260u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_RECORDER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_REQUESTEDROTATIONTYPEISPURECAV: u32 = 274u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_REQUESTEDWRITESPEED: u32 = 273u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_SETWRITESPEED: u32 = 514u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_STARTSECTOROFPREVIOUSSESSION: u32 = 267u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_SUPPORTEDWRITESPEEDDESCRIPTORS: u32 = 278u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_SUPPORTEDWRITESPEEDS: u32 = 277u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_TOTALSECTORS: u32 = 264u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_WRITE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_WRITEPROTECTSTATUS: u32 = 263u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2ERASEEVENTS_UPDATE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2ERASE_CLIENTNAME: u32 = 259u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2ERASE_ERASEMEDIA: u32 = 513u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2ERASE_FULLERASE: u32 = 257u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2ERASE_MEDIATYPE: u32 = 258u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2ERASE_RECORDER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_CURRENTACTION: u32 = 769u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_CURRENTTRACKNUMBER: u32 = 768u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_ELAPSEDTIME: u32 = 768u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_ESTIMATEDREMAININGTIME: u32 = 769u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_ESTIMATEDTOTALTIME: u32 = 770u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_BUFFERUNDERRUNFREEDISABLED: u32 = 258u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_CANCELWRITE: u32 = 515u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_CLIENTNAME: u32 = 266u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_CURRENTMEDIATYPE: u32 = 261u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_CURRENTROTATIONTYPEISPURECAV: u32 = 270u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_CURRENTWRITESPEED: u32 = 269u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_LASTPOSSIBLESTARTOFLEADOUT: u32 = 260u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_PREPAREMEDIA: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_RECORDER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_RELEASEMEDIA: u32 = 516u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_REQUESTEDDATASECTORTYPE: u32 = 265u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_REQUESTEDROTATIONTYPEISPURECAV: u32 = 268u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_REQUESTEDWRITESPEED: u32 = 267u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_SETWRITESPEED: u32 = 517u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_STARTOFNEXTSESSION: u32 = 259u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_SUPPORTEDDATASECTORTYPES: u32 = 264u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_SUPPORTEDWRITESPEEDDESCRIPTORS: u32 = 272u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_SUPPORTEDWRITESPEEDS: u32 = 271u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_WRITEMEDIA: u32 = 513u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_WRITEMEDIAWITHVALIDATION: u32 = 514u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_CURRENTACTION: u32 = 769u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_CURRENTTRACKNUMBER: u32 = 768u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_ELAPSEDTIME: u32 = 770u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_ESTIMATEDREMAININGTIME: u32 = 771u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_ESTIMATEDTOTALTIME: u32 = 772u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_ADDAUDIOTRACK: u32 = 513u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_BUFFERUNDERRUNFREEDISABLED: u32 = 258u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_CANCELADDTRACK: u32 = 514u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_CLIENTNAME: u32 = 270u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_CURRENTMEDIATYPE: u32 = 267u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_CURRENTROTATIONTYPEISPURECAV: u32 = 274u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_CURRENTWRITESPEED: u32 = 273u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_DONOTFINALIZEMEDIA: u32 = 263u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_EXPECTEDTABLEOFCONTENTS: u32 = 266u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_FINISHMEDIA: u32 = 515u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_FREESECTORSONMEDIA: u32 = 261u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_NUMBEROFEXISTINGTRACKS: u32 = 259u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_PREPAREMEDIA: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_RECORDER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_REQUESTEDROTATIONTYPEISPURECAV: u32 = 272u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_REQUESTEDWRITESPEED: u32 = 271u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_SETWRITESPEED: u32 = 516u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_SUPPORTEDWRITESPEEDDESCRIPTORS: u32 = 276u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_SUPPORTEDWRITESPEEDS: u32 = 275u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_TOTALSECTORSONMEDIA: u32 = 260u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_USEDSECTORSONMEDIA: u32 = 262u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2_MEDIAHEURISTICALLYBLANK: u32 = 1793u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2_MEDIAPHYSICALLYBLANK: u32 = 1792u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2_MEDIASUPPORTED: u32 = 2049u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2_RECORDERSUPPORTED: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2_SUPPORTEDMEDIATYPES: u32 = 1794u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_ACQUIREEXCLUSIVEACCESS: u32 = 258u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_ACTIVEDISCRECORDER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_CLOSETRAY: u32 = 257u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_CURRENTFEATUREPAGES: u32 = 521u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_CURRENTPROFILES: u32 = 523u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_DEVICECANLOADMEDIA: u32 = 518u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_DISABLEMCN: u32 = 260u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_EJECTMEDIA: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_ENABLEMCN: u32 = 261u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_EXCLUSIVEACCESSOWNER: u32 = 525u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_INITIALIZEDISCRECORDER: u32 = 262u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_LEGACYDEVICENUMBER: u32 = 519u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_PRODUCTID: u32 = 514u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_PRODUCTREVISION: u32 = 515u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_RELEASEEXCLUSIVEACCESS: u32 = 259u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_SUPPORTEDFEATUREPAGES: u32 = 520u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_SUPPORTEDMODEPAGES: u32 = 524u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_SUPPORTEDPROFILES: u32 = 522u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_VENDORID: u32 = 513u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_VOLUMENAME: u32 = 516u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_VOLUMEPATHNAMES: u32 = 517u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IMULTISESSION_FIRSTDATASESSION: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IMULTISESSION_FREESECTORS: u32 = 516u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IMULTISESSION_IMPORTRECORDER: u32 = 258u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IMULTISESSION_INUSE: u32 = 257u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IMULTISESSION_LASTSECTOROFPREVIOUSSESSION: u32 = 514u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IMULTISESSION_LASTWRITTENADDRESS: u32 = 518u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IMULTISESSION_NEXTWRITABLEADDRESS: u32 = 515u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IMULTISESSION_SECTORSONMEDIA: u32 = 519u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IMULTISESSION_STARTSECTOROFPREVIOUSSESSION: u32 = 513u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IMULTISESSION_SUPPORTEDONCURRENTMEDIA: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IMULTISESSION_WRITEUNITSIZE: u32 = 517u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDIMAGECREATOR_ADDSPECIALPREGAP: u32 = 514u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDIMAGECREATOR_ADDSUBCODERWGENERATOR: u32 = 515u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDIMAGECREATOR_ADDTRACK: u32 = 513u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDIMAGECREATOR_CREATERESULTIMAGE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDIMAGECREATOR_DISABLEGAPLESSAUDIO: u32 = 259u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDIMAGECREATOR_EXPECTEDTABLEOFCONTENTS: u32 = 265u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDIMAGECREATOR_MEDIACATALOGNUMBER: u32 = 260u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDIMAGECREATOR_NUMBEROFEXISTINGTRACKS: u32 = 263u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDIMAGECREATOR_RESULTINGIMAGETYPE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDIMAGECREATOR_STARTINGTRACKNUMBER: u32 = 261u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDIMAGECREATOR_STARTOFLEADOUT: u32 = 257u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDIMAGECREATOR_STARTOFLEADOUTLIMIT: u32 = 258u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDIMAGECREATOR_TRACKINFO: u32 = 262u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDIMAGECREATOR_USEDSECTORSONDISC: u32 = 264u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDTRACKINFO_AUDIOHASPREEMPHASIS: u32 = 262u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDTRACKINFO_DIGITALAUDIOCOPYSETTING: u32 = 261u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDTRACKINFO_ISRC: u32 = 260u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDTRACKINFO_SECTORCOUNT: u32 = 257u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDTRACKINFO_SECTORTYPE: u32 = 259u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDTRACKINFO_STARTINGLBA: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDTRACKINFO_TRACKNUMBER: u32 = 258u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IWRITEENGINE2EVENTARGS_FREESYSTEMBUFFER: u32 = 264u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IWRITEENGINE2EVENTARGS_LASTREADLBA: u32 = 258u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IWRITEENGINE2EVENTARGS_LASTWRITTENLBA: u32 = 259u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IWRITEENGINE2EVENTARGS_SECTORCOUNT: u32 = 257u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IWRITEENGINE2EVENTARGS_STARTLBA: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IWRITEENGINE2EVENTARGS_TOTALDEVICEBUFFER: u32 = 260u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IWRITEENGINE2EVENTARGS_TOTALSYSTEMBUFFER: u32 = 262u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IWRITEENGINE2EVENTARGS_USEDDEVICEBUFFER: u32 = 261u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IWRITEENGINE2EVENTARGS_USEDSYSTEMBUFFER: u32 = 263u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IWRITEENGINE2_BYTESPERSECTOR: u32 = 260u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IWRITEENGINE2_CANCELWRITE: u32 = 513u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IWRITEENGINE2_DISCRECORDER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IWRITEENGINE2_ENDINGSECTORSPERSECOND: u32 = 259u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IWRITEENGINE2_STARTINGSECTORSPERSECOND: u32 = 258u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IWRITEENGINE2_USESTREAMINGWRITE12: u32 = 257u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IWRITEENGINE2_WRITEINPROGRESS: u32 = 261u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IWRITEENGINE2_WRITESECTION: u32 = 512u32;
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct DWriteEngine2Events {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Update: unsafe extern "system" fn(this: *mut *mut Self, object: *mut ::core::ffi::c_void, progress: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Update: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for DWriteEngine2Events {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801527, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub type EmulationType = i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const EmulationNone: EmulationType = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const Emulation12MFloppy: EmulationType = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const Emulation144MFloppy: EmulationType = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const Emulation288MFloppy: EmulationType = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const EmulationHardDisk: EmulationType = 4i32;
pub const EnumFsiItems: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 747904966, data2: 38747, data3: 22974, data4: [169, 96, 154, 42, 38, 40, 83, 165] };
pub const EnumProgressItems: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 747904970, data2: 38747, data3: 22974, data4: [169, 96, 154, 42, 38, 40, 83, 165] };
pub const FileSystemImageResult: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 747904972, data2: 38747, data3: 22974, data4: [169, 96, 154, 42, 38, 40, 83, 165] };
pub const FsiDirectoryItem: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 747904968, data2: 38747, data3: 22974, data4: [169, 96, 154, 42, 38, 40, 83, 165] };
pub const FsiFileItem: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 747904967, data2: 38747, data3: 22974, data4: [169, 96, 154, 42, 38, 40, 83, 165] };
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub type FsiFileSystems = i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const FsiFileSystemNone: FsiFileSystems = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const FsiFileSystemISO9660: FsiFileSystems = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const FsiFileSystemJoliet: FsiFileSystems = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const FsiFileSystemUDF: FsiFileSystems = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const FsiFileSystemUnknown: FsiFileSystems = 1073741824i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub type FsiItemType = i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const FsiItemNotFound: FsiItemType = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const FsiItemDirectory: FsiItemType = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const FsiItemFile: FsiItemType = 2i32;
pub const FsiNamedStreams: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3333880045, data2: 27929, data3: 17588, data4: [181, 57, 177, 89, 183, 147, 163, 45] };
pub const FsiStream: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 747904973, data2: 38747, data3: 22974, data4: [169, 96, 154, 42, 38, 40, 83, 165] };
pub const GUID_SMTPSVC_SOURCE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 456918630, data2: 58480, data3: 4561, data4: [170, 103, 0, 192, 79, 163, 69, 246] };
pub const GUID_SMTP_SOURCE_TYPE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4217750748, data2: 58472, data3: 4561, data4: [170, 103, 0, 192, 79, 163, 69, 246] };
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IBlockRange {
    pub base__: super::super::System::Com::IDispatch,
    pub StartLba: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub EndLba: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IBlockRange {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3037186597, data2: 8708, data3: 4573, data4: [150, 106, 0, 26, 160, 27, 188, 88] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IBlockRangeList {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub BlockRanges: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BlockRanges: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IBlockRangeList {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3037186598, data2: 8708, data3: 4573, data4: [150, 106, 0, 26, 160, 27, 188, 88] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IBootOptions {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub BootImage: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BootImage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Manufacturer: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Manufacturer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetManufacturer: unsafe extern "system" fn(this: *mut *mut Self, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetManufacturer: usize,
    pub PlatformId: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut PlatformId) -> ::windows_sys::core::HRESULT,
    pub SetPlatformId: unsafe extern "system" fn(this: *mut *mut Self, newval: PlatformId) -> ::windows_sys::core::HRESULT,
    pub Emulation: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut EmulationType) -> ::windows_sys::core::HRESULT,
    pub SetEmulation: unsafe extern "system" fn(this: *mut *mut Self, newval: EmulationType) -> ::windows_sys::core::HRESULT,
    pub ImageSize: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AssignBootImage: unsafe extern "system" fn(this: *mut *mut Self, newval: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AssignBootImage: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IBootOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 747904980, data2: 38747, data3: 22974, data4: [169, 96, 154, 42, 38, 40, 83, 165] };
}
#[repr(C)]
pub struct IBurnVerification {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetBurnVerificationLevel: unsafe extern "system" fn(this: *mut *mut Self, value: IMAPI_BURN_VERIFICATION_LEVEL) -> ::windows_sys::core::HRESULT,
    pub BurnVerificationLevel: unsafe extern "system" fn(this: *mut *mut Self, value: *mut IMAPI_BURN_VERIFICATION_LEVEL) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBurnVerification {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3539982388, data2: 38283, data3: 17005, data4: [132, 112, 42, 19, 135, 156, 106, 145] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDiscFormat2 {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub IsRecorderSupported: unsafe extern "system" fn(this: *mut *mut Self, recorder: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IsRecorderSupported: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub IsCurrentMediaSupported: unsafe extern "system" fn(this: *mut *mut Self, recorder: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IsCurrentMediaSupported: usize,
    pub MediaPhysicallyBlank: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i16) -> ::windows_sys::core::HRESULT,
    pub MediaHeuristicallyBlank: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedMediaTypes: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedMediaTypes: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IDiscFormat2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801554, data2: 36708, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDiscFormat2Data {
    pub base__: IDiscFormat2,
    #[cfg(feature = "Win32_System_Com")]
    pub SetRecorder: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetRecorder: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Recorder: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Recorder: usize,
    pub SetBufferUnderrunFreeDisabled: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    pub BufferUnderrunFreeDisabled: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetPostgapAlreadyInImage: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    pub PostgapAlreadyInImage: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i16) -> ::windows_sys::core::HRESULT,
    pub CurrentMediaStatus: unsafe extern "system" fn(this: *mut *mut Self, value: *mut IMAPI_FORMAT2_DATA_MEDIA_STATE) -> ::windows_sys::core::HRESULT,
    pub WriteProtectStatus: unsafe extern "system" fn(this: *mut *mut Self, value: *mut IMAPI_MEDIA_WRITE_PROTECT_STATE) -> ::windows_sys::core::HRESULT,
    pub TotalSectorsOnMedia: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub FreeSectorsOnMedia: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub NextWritableAddress: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub StartAddressOfPreviousSession: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub LastWrittenAddressOfPreviousSession: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetForceMediaToBeClosed: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    pub ForceMediaToBeClosed: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetDisableConsumerDvdCompatibilityMode: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    pub DisableConsumerDvdCompatibilityMode: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i16) -> ::windows_sys::core::HRESULT,
    pub CurrentPhysicalMediaType: unsafe extern "system" fn(this: *mut *mut Self, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClientName: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClientName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ClientName: unsafe extern "system" fn(this: *mut *mut Self, value: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClientName: usize,
    pub RequestedWriteSpeed: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub RequestedRotationTypeIsPureCAV: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i16) -> ::windows_sys::core::HRESULT,
    pub CurrentWriteSpeed: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub CurrentRotationTypeIsPureCAV: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedWriteSpeeds: unsafe extern "system" fn(this: *mut *mut Self, supportedspeeds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedWriteSpeeds: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedWriteSpeedDescriptors: unsafe extern "system" fn(this: *mut *mut Self, supportedspeeddescriptors: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedWriteSpeedDescriptors: usize,
    pub SetForceOverwrite: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    pub ForceOverwrite: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub MultisessionInterfaces: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    MultisessionInterfaces: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Write: unsafe extern "system" fn(this: *mut *mut Self, data: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Write: usize,
    pub CancelWrite: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetWriteSpeed: unsafe extern "system" fn(this: *mut *mut Self, requestedsectorspersecond: i32, rotationtypeispurecav: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IDiscFormat2Data {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801555, data2: 40804, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDiscFormat2DataEventArgs {
    pub base__: IWriteEngine2EventArgs,
    pub ElapsedTime: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub RemainingTime: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub TotalTime: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub CurrentAction: unsafe extern "system" fn(this: *mut *mut Self, value: *mut IMAPI_FORMAT2_DATA_WRITE_ACTION) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IDiscFormat2DataEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801533, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDiscFormat2Erase {
    pub base__: IDiscFormat2,
    #[cfg(feature = "Win32_System_Com")]
    pub SetRecorder: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetRecorder: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Recorder: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Recorder: usize,
    pub SetFullErase: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    pub FullErase: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i16) -> ::windows_sys::core::HRESULT,
    pub CurrentPhysicalMediaType: unsafe extern "system" fn(this: *mut *mut Self, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClientName: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClientName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ClientName: unsafe extern "system" fn(this: *mut *mut Self, value: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClientName: usize,
    pub EraseMedia: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IDiscFormat2Erase {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801558, data2: 36708, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDiscFormat2RawCD {
    pub base__: IDiscFormat2,
    pub PrepareMedia: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub WriteMedia: unsafe extern "system" fn(this: *mut *mut Self, data: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    WriteMedia: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub WriteMedia2: unsafe extern "system" fn(this: *mut *mut Self, data: *mut ::core::ffi::c_void, streamleadinsectors: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    WriteMedia2: usize,
    pub CancelWrite: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ReleaseMedia: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetWriteSpeed: unsafe extern "system" fn(this: *mut *mut Self, requestedsectorspersecond: i32, rotationtypeispurecav: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetRecorder: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetRecorder: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Recorder: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Recorder: usize,
    pub SetBufferUnderrunFreeDisabled: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    pub BufferUnderrunFreeDisabled: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i16) -> ::windows_sys::core::HRESULT,
    pub StartOfNextSession: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub LastPossibleStartOfLeadout: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub CurrentPhysicalMediaType: unsafe extern "system" fn(this: *mut *mut Self, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedSectorTypes: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedSectorTypes: usize,
    pub SetRequestedSectorType: unsafe extern "system" fn(this: *mut *mut Self, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows_sys::core::HRESULT,
    pub RequestedSectorType: unsafe extern "system" fn(this: *mut *mut Self, value: *mut IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClientName: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClientName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ClientName: unsafe extern "system" fn(this: *mut *mut Self, value: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClientName: usize,
    pub RequestedWriteSpeed: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub RequestedRotationTypeIsPureCAV: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i16) -> ::windows_sys::core::HRESULT,
    pub CurrentWriteSpeed: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub CurrentRotationTypeIsPureCAV: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedWriteSpeeds: unsafe extern "system" fn(this: *mut *mut Self, supportedspeeds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedWriteSpeeds: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedWriteSpeedDescriptors: unsafe extern "system" fn(this: *mut *mut Self, supportedspeeddescriptors: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedWriteSpeedDescriptors: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IDiscFormat2RawCD {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801557, data2: 36708, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDiscFormat2RawCDEventArgs {
    pub base__: IWriteEngine2EventArgs,
    pub CurrentAction: unsafe extern "system" fn(this: *mut *mut Self, value: *mut IMAPI_FORMAT2_RAW_CD_WRITE_ACTION) -> ::windows_sys::core::HRESULT,
    pub ElapsedTime: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub RemainingTime: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IDiscFormat2RawCDEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801539, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDiscFormat2TrackAtOnce {
    pub base__: IDiscFormat2,
    pub PrepareMedia: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddAudioTrack: unsafe extern "system" fn(this: *mut *mut Self, data: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddAudioTrack: usize,
    pub CancelAddTrack: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ReleaseMedia: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetWriteSpeed: unsafe extern "system" fn(this: *mut *mut Self, requestedsectorspersecond: i32, rotationtypeispurecav: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetRecorder: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetRecorder: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Recorder: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Recorder: usize,
    pub SetBufferUnderrunFreeDisabled: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    pub BufferUnderrunFreeDisabled: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i16) -> ::windows_sys::core::HRESULT,
    pub NumberOfExistingTracks: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub TotalSectorsOnMedia: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub FreeSectorsOnMedia: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub UsedSectorsOnMedia: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetDoNotFinalizeMedia: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    pub DoNotFinalizeMedia: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ExpectedTableOfContents: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExpectedTableOfContents: usize,
    pub CurrentPhysicalMediaType: unsafe extern "system" fn(this: *mut *mut Self, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClientName: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClientName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ClientName: unsafe extern "system" fn(this: *mut *mut Self, value: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClientName: usize,
    pub RequestedWriteSpeed: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub RequestedRotationTypeIsPureCAV: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i16) -> ::windows_sys::core::HRESULT,
    pub CurrentWriteSpeed: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub CurrentRotationTypeIsPureCAV: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedWriteSpeeds: unsafe extern "system" fn(this: *mut *mut Self, supportedspeeds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedWriteSpeeds: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedWriteSpeedDescriptors: unsafe extern "system" fn(this: *mut *mut Self, supportedspeeddescriptors: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedWriteSpeedDescriptors: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IDiscFormat2TrackAtOnce {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801556, data2: 36708, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDiscFormat2TrackAtOnceEventArgs {
    pub base__: IWriteEngine2EventArgs,
    pub CurrentTrackNumber: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub CurrentAction: unsafe extern "system" fn(this: *mut *mut Self, value: *mut IMAPI_FORMAT2_TAO_WRITE_ACTION) -> ::windows_sys::core::HRESULT,
    pub ElapsedTime: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub RemainingTime: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IDiscFormat2TrackAtOnceEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801536, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
}
#[repr(C)]
pub struct IDiscMaster {
    pub base__: ::windows_sys::core::IUnknown,
    pub Open: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub EnumDiscMasterFormats: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetActiveDiscMasterFormat: unsafe extern "system" fn(this: *mut *mut Self, lpiid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SetActiveDiscMasterFormat: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EnumDiscRecorders: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetActiveDiscRecorder: unsafe extern "system" fn(this: *mut *mut Self, pprecorder: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetActiveDiscRecorder: unsafe extern "system" fn(this: *mut *mut Self, precorder: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ClearFormatContent: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ProgressAdvise: unsafe extern "system" fn(this: *mut *mut Self, pevents: *mut ::core::ffi::c_void, pvcookie: *mut usize) -> ::windows_sys::core::HRESULT,
    pub ProgressUnadvise: unsafe extern "system" fn(this: *mut *mut Self, vcookie: usize) -> ::windows_sys::core::HRESULT,
    pub RecordDisc: unsafe extern "system" fn(this: *mut *mut Self, bsimulate: u8, bejectafterburn: u8) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDiscMaster {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1376569954, data2: 20901, data3: 4563, data4: [145, 68, 0, 16, 75, 161, 28, 94] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDiscMaster2 {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, value: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub IsSupportedEnvironment: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IDiscMaster2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801520, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
}
#[repr(C)]
pub struct IDiscMasterProgressEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub QueryCancel: unsafe extern "system" fn(this: *mut *mut Self, pbcancel: *mut u8) -> ::windows_sys::core::HRESULT,
    pub NotifyPnPActivity: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub NotifyAddProgress: unsafe extern "system" fn(this: *mut *mut Self, ncompletedsteps: i32, ntotalsteps: i32) -> ::windows_sys::core::HRESULT,
    pub NotifyBlockProgress: unsafe extern "system" fn(this: *mut *mut Self, ncompleted: i32, ntotal: i32) -> ::windows_sys::core::HRESULT,
    pub NotifyTrackProgress: unsafe extern "system" fn(this: *mut *mut Self, ncurrenttrack: i32, ntotaltracks: i32) -> ::windows_sys::core::HRESULT,
    pub NotifyPreparingBurn: unsafe extern "system" fn(this: *mut *mut Self, nestimatedseconds: i32) -> ::windows_sys::core::HRESULT,
    pub NotifyClosingDisc: unsafe extern "system" fn(this: *mut *mut Self, nestimatedseconds: i32) -> ::windows_sys::core::HRESULT,
    pub NotifyBurnComplete: unsafe extern "system" fn(this: *mut *mut Self, status: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub NotifyEraseComplete: unsafe extern "system" fn(this: *mut *mut Self, status: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDiscMasterProgressEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3969798593, data2: 20061, data3: 4563, data4: [145, 68, 0, 16, 75, 161, 28, 94] };
}
#[repr(C)]
pub struct IDiscRecorder {
    pub base__: ::windows_sys::core::IUnknown,
    pub Init: unsafe extern "system" fn(this: *mut *mut Self, pbyuniqueid: *const u8, nulidsize: u32, nuldrivenumber: u32) -> ::windows_sys::core::HRESULT,
    pub GetRecorderGUID: unsafe extern "system" fn(this: *mut *mut Self, pbyuniqueid: *mut u8, ulbuffersize: u32, pulreturnsizerequired: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetRecorderType: unsafe extern "system" fn(this: *mut *mut Self, ftypecode: *mut RECORDER_TYPES) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDisplayNames: unsafe extern "system" fn(this: *mut *mut Self, pbstrvendorid: *mut super::super::Foundation::BSTR, pbstrproductid: *mut super::super::Foundation::BSTR, pbstrrevision: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDisplayNames: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBasePnPID: unsafe extern "system" fn(this: *mut *mut Self, pbstrbasepnpid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBasePnPID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPath: unsafe extern "system" fn(this: *mut *mut Self, pbstrpath: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPath: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub GetRecorderProperties: unsafe extern "system" fn(this: *mut *mut Self, pppropstg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    GetRecorderProperties: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub SetRecorderProperties: unsafe extern "system" fn(this: *mut *mut Self, ppropstg: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    SetRecorderProperties: usize,
    pub GetRecorderState: unsafe extern "system" fn(this: *mut *mut Self, puldevstateflags: *mut DISC_RECORDER_STATE_FLAGS) -> ::windows_sys::core::HRESULT,
    pub OpenExclusive: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub QueryMediaType: unsafe extern "system" fn(this: *mut *mut Self, fmediatype: *mut MEDIA_TYPES, fmediaflags: *mut MEDIA_FLAGS) -> ::windows_sys::core::HRESULT,
    pub QueryMediaInfo: unsafe extern "system" fn(this: *mut *mut Self, pbsessions: *mut u8, pblasttrack: *mut u8, ulstartaddress: *mut u32, ulnextwritable: *mut u32, ulfreeblocks: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Eject: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Erase: unsafe extern "system" fn(this: *mut *mut Self, bfullerase: u8) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDiscRecorder {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2242680694, data2: 51848, data3: 19698, data4: [137, 78, 9, 89, 140, 7, 138, 65] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDiscRecorder2 {
    pub base__: super::super::System::Com::IDispatch,
    pub EjectMedia: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub CloseTray: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AcquireExclusiveAccess: unsafe extern "system" fn(this: *mut *mut Self, force: i16, __midl__idiscrecorder20000: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AcquireExclusiveAccess: usize,
    pub ReleaseExclusiveAccess: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub DisableMcn: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub EnableMcn: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeDiscRecorder: unsafe extern "system" fn(this: *mut *mut Self, recorderuniqueid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeDiscRecorder: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ActiveDiscRecorder: unsafe extern "system" fn(this: *mut *mut Self, value: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ActiveDiscRecorder: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub VendorId: unsafe extern "system" fn(this: *mut *mut Self, value: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    VendorId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ProductId: unsafe extern "system" fn(this: *mut *mut Self, value: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ProductId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ProductRevision: unsafe extern "system" fn(this: *mut *mut Self, value: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ProductRevision: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub VolumeName: unsafe extern "system" fn(this: *mut *mut Self, value: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    VolumeName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub VolumePathNames: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    VolumePathNames: usize,
    pub DeviceCanLoadMedia: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i16) -> ::windows_sys::core::HRESULT,
    pub LegacyDeviceNumber: unsafe extern "system" fn(this: *mut *mut Self, legacydevicenumber: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedFeaturePages: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedFeaturePages: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CurrentFeaturePages: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CurrentFeaturePages: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedProfiles: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedProfiles: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CurrentProfiles: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CurrentProfiles: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedModePages: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedModePages: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExclusiveAccessOwner: unsafe extern "system" fn(this: *mut *mut Self, value: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExclusiveAccessOwner: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IDiscRecorder2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801523, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
}
#[repr(C)]
pub struct IDiscRecorder2Ex {
    pub base__: ::windows_sys::core::IUnknown,
    pub SendCommandNoData: unsafe extern "system" fn(this: *mut *mut Self, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32) -> ::windows_sys::core::HRESULT,
    pub SendCommandSendDataToDevice: unsafe extern "system" fn(this: *mut *mut Self, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *const u8, buffersize: u32) -> ::windows_sys::core::HRESULT,
    pub SendCommandGetDataFromDevice: unsafe extern "system" fn(this: *mut *mut Self, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *mut u8, buffersize: u32, bufferfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ReadDvdStructure: unsafe extern "system" fn(this: *mut *mut Self, format: u32, address: u32, layer: u32, agid: u32, data: *mut *mut u8, count: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SendDvdStructure: unsafe extern "system" fn(this: *mut *mut Self, format: u32, data: *const u8, count: u32) -> ::windows_sys::core::HRESULT,
    pub GetAdapterDescriptor: unsafe extern "system" fn(this: *mut *mut Self, data: *mut *mut u8, bytesize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetDeviceDescriptor: unsafe extern "system" fn(this: *mut *mut Self, data: *mut *mut u8, bytesize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetDiscInformation: unsafe extern "system" fn(this: *mut *mut Self, discinformation: *mut *mut u8, bytesize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetTrackInformation: unsafe extern "system" fn(this: *mut *mut Self, address: u32, addresstype: IMAPI_READ_TRACK_ADDRESS_TYPE, trackinformation: *mut *mut u8, bytesize: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFeaturePage: unsafe extern "system" fn(this: *mut *mut Self, requestedfeature: IMAPI_FEATURE_PAGE_TYPE, currentfeatureonly: super::super::Foundation::BOOLEAN, featuredata: *mut *mut u8, bytesize: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFeaturePage: usize,
    pub GetModePage: unsafe extern "system" fn(this: *mut *mut Self, requestedmodepage: IMAPI_MODE_PAGE_TYPE, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagedata: *mut *mut u8, bytesize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetModePage: unsafe extern "system" fn(this: *mut *mut Self, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, data: *const u8, bytesize: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSupportedFeaturePages: unsafe extern "system" fn(this: *mut *mut Self, currentfeatureonly: super::super::Foundation::BOOLEAN, featuredata: *mut *mut IMAPI_FEATURE_PAGE_TYPE, bytesize: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSupportedFeaturePages: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSupportedProfiles: unsafe extern "system" fn(this: *mut *mut Self, currentonly: super::super::Foundation::BOOLEAN, profiletypes: *mut *mut IMAPI_PROFILE_TYPE, validprofiles: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSupportedProfiles: usize,
    pub GetSupportedModePages: unsafe extern "system" fn(this: *mut *mut Self, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagetypes: *mut *mut IMAPI_MODE_PAGE_TYPE, validpages: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetByteAlignmentMask: unsafe extern "system" fn(this: *mut *mut Self, value: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetMaximumNonPageAlignedTransferSize: unsafe extern "system" fn(this: *mut *mut Self, value: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetMaximumPageAlignedTransferSize: unsafe extern "system" fn(this: *mut *mut Self, value: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDiscRecorder2Ex {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801522, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
}
#[repr(C)]
pub struct IEnumDiscMasterFormats {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, cformats: u32, lpiidformatid: *mut ::windows_sys::core::GUID, pcfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, cformats: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumDiscMasterFormats {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3723773409, data2: 21690, data3: 4563, data4: [145, 68, 0, 16, 75, 161, 28, 94] };
}
#[repr(C)]
pub struct IEnumDiscRecorders {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, crecorders: u32, pprecorder: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, crecorders: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumDiscRecorders {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2602115553, data2: 21676, data3: 4563, data4: [145, 68, 0, 16, 75, 161, 28, 94] };
}
#[repr(C)]
pub struct IEnumFsiItems {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumFsiItems {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 747904986, data2: 38747, data3: 22974, data4: [169, 96, 154, 42, 38, 40, 83, 165] };
}
#[repr(C)]
pub struct IEnumProgressItems {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumProgressItems {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 747904982, data2: 38747, data3: 22974, data4: [169, 96, 154, 42, 38, 40, 83, 165] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFileSystemImage {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Root: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Root: usize,
    pub SessionStartBlock: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetSessionStartBlock: unsafe extern "system" fn(this: *mut *mut Self, newval: i32) -> ::windows_sys::core::HRESULT,
    pub FreeMediaBlocks: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetFreeMediaBlocks: unsafe extern "system" fn(this: *mut *mut Self, newval: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetMaxMediaBlocksFromDevice: unsafe extern "system" fn(this: *mut *mut Self, discrecorder: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetMaxMediaBlocksFromDevice: usize,
    pub UsedBlocks: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub VolumeName: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    VolumeName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetVolumeName: unsafe extern "system" fn(this: *mut *mut Self, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetVolumeName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ImportedVolumeName: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ImportedVolumeName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub BootImageOptions: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BootImageOptions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetBootImageOptions: unsafe extern "system" fn(this: *mut *mut Self, newval: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetBootImageOptions: usize,
    pub FileCount: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub DirectoryCount: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub WorkingDirectory: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WorkingDirectory: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetWorkingDirectory: unsafe extern "system" fn(this: *mut *mut Self, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetWorkingDirectory: usize,
    pub ChangePoint: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub StrictFileSystemCompliance: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetStrictFileSystemCompliance: unsafe extern "system" fn(this: *mut *mut Self, newval: i16) -> ::windows_sys::core::HRESULT,
    pub UseRestrictedCharacterSet: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetUseRestrictedCharacterSet: unsafe extern "system" fn(this: *mut *mut Self, newval: i16) -> ::windows_sys::core::HRESULT,
    pub FileSystemsToCreate: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut FsiFileSystems) -> ::windows_sys::core::HRESULT,
    pub SetFileSystemsToCreate: unsafe extern "system" fn(this: *mut *mut Self, newval: FsiFileSystems) -> ::windows_sys::core::HRESULT,
    pub FileSystemsSupported: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut FsiFileSystems) -> ::windows_sys::core::HRESULT,
    pub SetUDFRevision: unsafe extern "system" fn(this: *mut *mut Self, newval: i32) -> ::windows_sys::core::HRESULT,
    pub UDFRevision: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub UDFRevisionsSupported: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UDFRevisionsSupported: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ChooseImageDefaults: unsafe extern "system" fn(this: *mut *mut Self, discrecorder: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ChooseImageDefaults: usize,
    pub ChooseImageDefaultsForMediaType: unsafe extern "system" fn(this: *mut *mut Self, value: IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows_sys::core::HRESULT,
    pub SetISO9660InterchangeLevel: unsafe extern "system" fn(this: *mut *mut Self, newval: i32) -> ::windows_sys::core::HRESULT,
    pub ISO9660InterchangeLevel: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ISO9660InterchangeLevelsSupported: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ISO9660InterchangeLevelsSupported: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateResultImage: unsafe extern "system" fn(this: *mut *mut Self, resultstream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateResultImage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Exists: unsafe extern "system" fn(this: *mut *mut Self, fullpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, itemtype: *mut FsiItemType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Exists: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CalculateDiscIdentifier: unsafe extern "system" fn(this: *mut *mut Self, discidentifier: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CalculateDiscIdentifier: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub IdentifyFileSystemsOnDisc: unsafe extern "system" fn(this: *mut *mut Self, discrecorder: *mut ::core::ffi::c_void, filesystems: *mut FsiFileSystems) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IdentifyFileSystemsOnDisc: usize,
    pub GetDefaultFileSystemForImport: unsafe extern "system" fn(this: *mut *mut Self, filesystems: FsiFileSystems, importdefault: *mut FsiFileSystems) -> ::windows_sys::core::HRESULT,
    pub ImportFileSystem: unsafe extern "system" fn(this: *mut *mut Self, importedfilesystem: *mut FsiFileSystems) -> ::windows_sys::core::HRESULT,
    pub ImportSpecificFileSystem: unsafe extern "system" fn(this: *mut *mut Self, filesystemtouse: FsiFileSystems) -> ::windows_sys::core::HRESULT,
    pub RollbackToChangePoint: unsafe extern "system" fn(this: *mut *mut Self, changepoint: i32) -> ::windows_sys::core::HRESULT,
    pub LockInChangePoint: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateDirectoryItem: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newitem: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateDirectoryItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateFileItem: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newitem: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateFileItem: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub VolumeNameUDF: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    VolumeNameUDF: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub VolumeNameJoliet: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    VolumeNameJoliet: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub VolumeNameISO9660: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    VolumeNameISO9660: usize,
    pub StageFiles: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetStageFiles: unsafe extern "system" fn(this: *mut *mut Self, newval: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub MultisessionInterfaces: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    MultisessionInterfaces: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetMultisessionInterfaces: unsafe extern "system" fn(this: *mut *mut Self, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetMultisessionInterfaces: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFileSystemImage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 747904993, data2: 38747, data3: 22974, data4: [169, 96, 154, 42, 38, 40, 83, 165] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFileSystemImage2 {
    pub base__: IFileSystemImage,
    #[cfg(feature = "Win32_System_Com")]
    pub BootImageOptionsArray: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BootImageOptionsArray: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetBootImageOptionsArray: unsafe extern "system" fn(this: *mut *mut Self, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetBootImageOptionsArray: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFileSystemImage2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3613674284, data2: 5431, data3: 18279, data4: [182, 47, 241, 56, 123, 2, 221, 253] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFileSystemImage3 {
    pub base__: IFileSystemImage2,
    pub CreateRedundantUdfMetadataFiles: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetCreateRedundantUdfMetadataFiles: unsafe extern "system" fn(this: *mut *mut Self, newval: i16) -> ::windows_sys::core::HRESULT,
    pub ProbeSpecificFileSystem: unsafe extern "system" fn(this: *mut *mut Self, filesystemtoprobe: FsiFileSystems, isappendable: *mut i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFileSystemImage3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2097120300, data2: 32407, data3: 18439, data4: [131, 4, 145, 13, 216, 247, 192, 81] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFileSystemImageResult {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub ImageStream: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ImageStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ProgressItems: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ProgressItems: usize,
    pub TotalBlocks: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub BlockSize: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DiscId: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DiscId: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFileSystemImageResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 747904984, data2: 38747, data3: 22974, data4: [169, 96, 154, 42, 38, 40, 83, 165] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFileSystemImageResult2 {
    pub base__: IFileSystemImageResult,
    #[cfg(feature = "Win32_System_Com")]
    pub ModifiedBlocks: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ModifiedBlocks: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFileSystemImageResult2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3037186601, data2: 8708, data3: 4573, data4: [150, 106, 0, 26, 160, 27, 188, 88] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsiDirectoryItem {
    pub base__: IFsiItem,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, item: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, count: *mut i32) -> ::windows_sys::core::HRESULT,
    pub EnumFsiItems: unsafe extern "system" fn(this: *mut *mut Self, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddDirectory: unsafe extern "system" fn(this: *mut *mut Self, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddDirectory: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AddFile: unsafe extern "system" fn(this: *mut *mut Self, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filedata: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AddFile: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AddTree: unsafe extern "system" fn(this: *mut *mut Self, sourcedirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, includebasedirectory: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddTree: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Remove: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoveTree: unsafe extern "system" fn(this: *mut *mut Self, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoveTree: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsiDirectoryItem {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 747904988, data2: 38747, data3: 22974, data4: [169, 96, 154, 42, 38, 40, 83, 165] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsiDirectoryItem2 {
    pub base__: IFsiDirectoryItem,
    #[cfg(feature = "Win32_Foundation")]
    pub AddTreeWithNamedStreams: unsafe extern "system" fn(this: *mut *mut Self, sourcedirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, includebasedirectory: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddTreeWithNamedStreams: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsiDirectoryItem2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4160441243, data2: 28054, data3: 19835, data4: [145, 21, 32, 27, 20, 72, 17, 239] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsiFileItem {
    pub base__: IFsiItem,
    pub DataSize: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i64) -> ::windows_sys::core::HRESULT,
    pub DataSize32BitLow: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub DataSize32BitHigh: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Data: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetData: unsafe extern "system" fn(this: *mut *mut Self, newval: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetData: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsiFileItem {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 747904987, data2: 38747, data3: 22974, data4: [169, 96, 154, 42, 38, 40, 83, 165] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsiFileItem2 {
    pub base__: IFsiFileItem,
    #[cfg(feature = "Win32_System_Com")]
    pub FsiNamedStreams: unsafe extern "system" fn(this: *mut *mut Self, streams: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FsiNamedStreams: usize,
    pub IsNamedStream: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AddStream: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, streamdata: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AddStream: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoveStream: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoveStream: usize,
    pub IsRealTime: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetIsRealTime: unsafe extern "system" fn(this: *mut *mut Self, newval: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsiFileItem2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 429722649, data2: 4577, data3: 16619, data4: [142, 194, 200, 200, 34, 160, 119, 146] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsiItem {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FullPath: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FullPath: usize,
    pub CreationTime: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetCreationTime: unsafe extern "system" fn(this: *mut *mut Self, newval: f64) -> ::windows_sys::core::HRESULT,
    pub LastAccessedTime: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetLastAccessedTime: unsafe extern "system" fn(this: *mut *mut Self, newval: f64) -> ::windows_sys::core::HRESULT,
    pub LastModifiedTime: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetLastModifiedTime: unsafe extern "system" fn(this: *mut *mut Self, newval: f64) -> ::windows_sys::core::HRESULT,
    pub IsHidden: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetIsHidden: unsafe extern "system" fn(this: *mut *mut Self, newval: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FileSystemName: unsafe extern "system" fn(this: *mut *mut Self, filesystem: FsiFileSystems, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FileSystemName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FileSystemPath: unsafe extern "system" fn(this: *mut *mut Self, filesystem: FsiFileSystems, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FileSystemPath: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsiItem {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 747904985, data2: 38747, data3: 22974, data4: [169, 96, 154, 42, 38, 40, 83, 165] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsiNamedStreams {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, item: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, count: *mut i32) -> ::windows_sys::core::HRESULT,
    pub EnumNamedStreams: unsafe extern "system" fn(this: *mut *mut Self, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsiNamedStreams {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3984177750, data2: 21140, data3: 16976, data4: [141, 70, 249, 174, 206, 226, 52, 89] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IIsoImageManager {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Path: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Path: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Stream: unsafe extern "system" fn(this: *mut *mut Self, data: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Stream: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPath: unsafe extern "system" fn(this: *mut *mut Self, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPath: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetStream: unsafe extern "system" fn(this: *mut *mut Self, data: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetStream: usize,
    pub Validate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IIsoImageManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1822657509, data2: 64443, data3: 18432, data4: [149, 161, 164, 56, 134, 94, 176, 212] };
}
#[repr(C)]
pub struct IJolietDiscMaster {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetTotalDataBlocks: unsafe extern "system" fn(this: *mut *mut Self, pnblocks: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetUsedDataBlocks: unsafe extern "system" fn(this: *mut *mut Self, pnblocks: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetDataBlockSize: unsafe extern "system" fn(this: *mut *mut Self, pnblockbytes: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub AddData: unsafe extern "system" fn(this: *mut *mut Self, pstorage: *mut ::core::ffi::c_void, lfileoverwrite: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    AddData: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub GetJolietProperties: unsafe extern "system" fn(this: *mut *mut Self, pppropstg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    GetJolietProperties: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub SetJolietProperties: unsafe extern "system" fn(this: *mut *mut Self, ppropstg: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    SetJolietProperties: usize,
}
impl ::windows_sys::core::Interface for IJolietDiscMaster {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3820765902, data2: 20060, data3: 4563, data4: [145, 68, 0, 16, 75, 161, 28, 94] };
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI2FS_BOOT_ENTRY_COUNT_MAX: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI2FS_FullVersion_STR: &str = "1.0";
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI2FS_FullVersion_WSTR: &str = "1.0";
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI2FS_MajorVersion: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI2FS_MinorVersion: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI2_DEFAULT_COMMAND_TIMEOUT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPILib2_MajorVersion: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPILib2_MinorVersion: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub type IMAPI_BURN_VERIFICATION_LEVEL = i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_BURN_VERIFICATION_NONE: IMAPI_BURN_VERIFICATION_LEVEL = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_BURN_VERIFICATION_QUICK: IMAPI_BURN_VERIFICATION_LEVEL = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_BURN_VERIFICATION_FULL: IMAPI_BURN_VERIFICATION_LEVEL = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub type IMAPI_CD_SECTOR_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_CD_SECTOR_AUDIO: IMAPI_CD_SECTOR_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_CD_SECTOR_MODE_ZERO: IMAPI_CD_SECTOR_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_CD_SECTOR_MODE1: IMAPI_CD_SECTOR_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_CD_SECTOR_MODE2FORM0: IMAPI_CD_SECTOR_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_CD_SECTOR_MODE2FORM1: IMAPI_CD_SECTOR_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_CD_SECTOR_MODE2FORM2: IMAPI_CD_SECTOR_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_CD_SECTOR_MODE1RAW: IMAPI_CD_SECTOR_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_CD_SECTOR_MODE2FORM0RAW: IMAPI_CD_SECTOR_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_CD_SECTOR_MODE2FORM1RAW: IMAPI_CD_SECTOR_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_CD_SECTOR_MODE2FORM2RAW: IMAPI_CD_SECTOR_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub type IMAPI_CD_TRACK_DIGITAL_COPY_SETTING = i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_CD_TRACK_DIGITAL_COPY_PERMITTED: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_CD_TRACK_DIGITAL_COPY_PROHIBITED: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_CD_TRACK_DIGITAL_COPY_SCMS: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_ALREADYOPEN: ::windows_sys::core::HRESULT = -2147220958i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_BADJOLIETNAME: ::windows_sys::core::HRESULT = -2147220963i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_BOOTIMAGE_AND_NONBLANK_DISC: ::windows_sys::core::HRESULT = -2147220946i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_CANNOT_WRITE_TO_MEDIA: ::windows_sys::core::HRESULT = -2147220948i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_COMPRESSEDSTASH: ::windows_sys::core::HRESULT = -2147220952i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_DEVICE_INVALIDTYPE: ::windows_sys::core::HRESULT = -2147220972i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_DEVICE_NOPROPERTIES: ::windows_sys::core::HRESULT = -2147220975i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_DEVICE_NOTACCESSIBLE: ::windows_sys::core::HRESULT = -2147220974i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_DEVICE_NOTPRESENT: ::windows_sys::core::HRESULT = -2147220973i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_DEVICE_STILL_IN_USE: ::windows_sys::core::HRESULT = -2147220954i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_DISCFULL: ::windows_sys::core::HRESULT = -2147220964i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_DISCINFO: ::windows_sys::core::HRESULT = -2147220967i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_ENCRYPTEDSTASH: ::windows_sys::core::HRESULT = -2147220951i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_FILEACCESS: ::windows_sys::core::HRESULT = -2147220968i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_FILEEXISTS: ::windows_sys::core::HRESULT = -2147220956i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_FILESYSTEM: ::windows_sys::core::HRESULT = -2147220969i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_GENERIC: ::windows_sys::core::HRESULT = -2147220978i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_INITIALIZE_ENDWRITE: ::windows_sys::core::HRESULT = -2147220970i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_INITIALIZE_WRITE: ::windows_sys::core::HRESULT = -2147220971i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_INVALIDIMAGE: ::windows_sys::core::HRESULT = -2147220962i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_LOSS_OF_STREAMING: ::windows_sys::core::HRESULT = -2147220953i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_MEDIUM_INVALIDTYPE: ::windows_sys::core::HRESULT = -2147220976i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_MEDIUM_NOTPRESENT: ::windows_sys::core::HRESULT = -2147220977i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_NOACTIVEFORMAT: ::windows_sys::core::HRESULT = -2147220961i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_NOACTIVERECORDER: ::windows_sys::core::HRESULT = -2147220960i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_NOTENOUGHDISKFORSTASH: ::windows_sys::core::HRESULT = -2147220950i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_NOTINITIALIZED: ::windows_sys::core::HRESULT = -2147220980i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_NOTOPENED: ::windows_sys::core::HRESULT = -2147220981i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_REMOVABLESTASH: ::windows_sys::core::HRESULT = -2147220949i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_STASHINUSE: ::windows_sys::core::HRESULT = -2147220955i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_TRACKNOTOPEN: ::windows_sys::core::HRESULT = -2147220966i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_TRACKOPEN: ::windows_sys::core::HRESULT = -2147220965i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_TRACK_NOT_BIG_ENOUGH: ::windows_sys::core::HRESULT = -2147220947i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_USERABORT: ::windows_sys::core::HRESULT = -2147220979i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_WRONGDISC: ::windows_sys::core::HRESULT = -2147220957i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_WRONGFORMAT: ::windows_sys::core::HRESULT = -2147220959i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub type IMAPI_FEATURE_PAGE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_PROFILE_LIST: IMAPI_FEATURE_PAGE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_CORE: IMAPI_FEATURE_PAGE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_MORPHING: IMAPI_FEATURE_PAGE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_REMOVABLE_MEDIUM: IMAPI_FEATURE_PAGE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_WRITE_PROTECT: IMAPI_FEATURE_PAGE_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_RANDOMLY_READABLE: IMAPI_FEATURE_PAGE_TYPE = 16i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_CD_MULTIREAD: IMAPI_FEATURE_PAGE_TYPE = 29i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_CD_READ: IMAPI_FEATURE_PAGE_TYPE = 30i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_READ: IMAPI_FEATURE_PAGE_TYPE = 31i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_RANDOMLY_WRITABLE: IMAPI_FEATURE_PAGE_TYPE = 32i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_INCREMENTAL_STREAMING_WRITABLE: IMAPI_FEATURE_PAGE_TYPE = 33i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_SECTOR_ERASABLE: IMAPI_FEATURE_PAGE_TYPE = 34i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_FORMATTABLE: IMAPI_FEATURE_PAGE_TYPE = 35i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_HARDWARE_DEFECT_MANAGEMENT: IMAPI_FEATURE_PAGE_TYPE = 36i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_WRITE_ONCE: IMAPI_FEATURE_PAGE_TYPE = 37i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_RESTRICTED_OVERWRITE: IMAPI_FEATURE_PAGE_TYPE = 38i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_CDRW_CAV_WRITE: IMAPI_FEATURE_PAGE_TYPE = 39i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_MRW: IMAPI_FEATURE_PAGE_TYPE = 40i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_ENHANCED_DEFECT_REPORTING: IMAPI_FEATURE_PAGE_TYPE = 41i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_PLUS_RW: IMAPI_FEATURE_PAGE_TYPE = 42i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_PLUS_R: IMAPI_FEATURE_PAGE_TYPE = 43i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_RIGID_RESTRICTED_OVERWRITE: IMAPI_FEATURE_PAGE_TYPE = 44i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_CD_TRACK_AT_ONCE: IMAPI_FEATURE_PAGE_TYPE = 45i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_CD_MASTERING: IMAPI_FEATURE_PAGE_TYPE = 46i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_DASH_WRITE: IMAPI_FEATURE_PAGE_TYPE = 47i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_DOUBLE_DENSITY_CD_READ: IMAPI_FEATURE_PAGE_TYPE = 48i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_DOUBLE_DENSITY_CD_R_WRITE: IMAPI_FEATURE_PAGE_TYPE = 49i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_DOUBLE_DENSITY_CD_RW_WRITE: IMAPI_FEATURE_PAGE_TYPE = 50i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_LAYER_JUMP_RECORDING: IMAPI_FEATURE_PAGE_TYPE = 51i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_CD_RW_MEDIA_WRITE_SUPPORT: IMAPI_FEATURE_PAGE_TYPE = 55i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_BD_PSEUDO_OVERWRITE: IMAPI_FEATURE_PAGE_TYPE = 56i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_PLUS_R_DUAL_LAYER: IMAPI_FEATURE_PAGE_TYPE = 59i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_BD_READ: IMAPI_FEATURE_PAGE_TYPE = 64i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_BD_WRITE: IMAPI_FEATURE_PAGE_TYPE = 65i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_HD_DVD_READ: IMAPI_FEATURE_PAGE_TYPE = 80i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_HD_DVD_WRITE: IMAPI_FEATURE_PAGE_TYPE = 81i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_POWER_MANAGEMENT: IMAPI_FEATURE_PAGE_TYPE = 256i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_SMART: IMAPI_FEATURE_PAGE_TYPE = 257i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_EMBEDDED_CHANGER: IMAPI_FEATURE_PAGE_TYPE = 258i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_CD_ANALOG_PLAY: IMAPI_FEATURE_PAGE_TYPE = 259i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_MICROCODE_UPDATE: IMAPI_FEATURE_PAGE_TYPE = 260i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_TIMEOUT: IMAPI_FEATURE_PAGE_TYPE = 261i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_CSS: IMAPI_FEATURE_PAGE_TYPE = 262i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_REAL_TIME_STREAMING: IMAPI_FEATURE_PAGE_TYPE = 263i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_LOGICAL_UNIT_SERIAL_NUMBER: IMAPI_FEATURE_PAGE_TYPE = 264i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_MEDIA_SERIAL_NUMBER: IMAPI_FEATURE_PAGE_TYPE = 265i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_DISC_CONTROL_BLOCKS: IMAPI_FEATURE_PAGE_TYPE = 266i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_CPRM: IMAPI_FEATURE_PAGE_TYPE = 267i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_FIRMWARE_INFORMATION: IMAPI_FEATURE_PAGE_TYPE = 268i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_AACS: IMAPI_FEATURE_PAGE_TYPE = 269i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_VCPS: IMAPI_FEATURE_PAGE_TYPE = 272i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub type IMAPI_FORMAT2_DATA_MEDIA_STATE = i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_UNKNOWN: IMAPI_FORMAT2_DATA_MEDIA_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_INFORMATIONAL_MASK: IMAPI_FORMAT2_DATA_MEDIA_STATE = 15i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_UNSUPPORTED_MASK: IMAPI_FORMAT2_DATA_MEDIA_STATE = 64512i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_OVERWRITE_ONLY: IMAPI_FORMAT2_DATA_MEDIA_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_RANDOMLY_WRITABLE: IMAPI_FORMAT2_DATA_MEDIA_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_BLANK: IMAPI_FORMAT2_DATA_MEDIA_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_APPENDABLE: IMAPI_FORMAT2_DATA_MEDIA_STATE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_FINAL_SESSION: IMAPI_FORMAT2_DATA_MEDIA_STATE = 8i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_DAMAGED: IMAPI_FORMAT2_DATA_MEDIA_STATE = 1024i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_ERASE_REQUIRED: IMAPI_FORMAT2_DATA_MEDIA_STATE = 2048i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_NON_EMPTY_SESSION: IMAPI_FORMAT2_DATA_MEDIA_STATE = 4096i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_WRITE_PROTECTED: IMAPI_FORMAT2_DATA_MEDIA_STATE = 8192i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_FINALIZED: IMAPI_FORMAT2_DATA_MEDIA_STATE = 16384i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_UNSUPPORTED_MEDIA: IMAPI_FORMAT2_DATA_MEDIA_STATE = 32768i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub type IMAPI_FORMAT2_DATA_WRITE_ACTION = i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_VALIDATING_MEDIA: IMAPI_FORMAT2_DATA_WRITE_ACTION = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_FORMATTING_MEDIA: IMAPI_FORMAT2_DATA_WRITE_ACTION = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_INITIALIZING_HARDWARE: IMAPI_FORMAT2_DATA_WRITE_ACTION = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_CALIBRATING_POWER: IMAPI_FORMAT2_DATA_WRITE_ACTION = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_WRITING_DATA: IMAPI_FORMAT2_DATA_WRITE_ACTION = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_FINALIZATION: IMAPI_FORMAT2_DATA_WRITE_ACTION = 5i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_COMPLETED: IMAPI_FORMAT2_DATA_WRITE_ACTION = 6i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_VERIFYING: IMAPI_FORMAT2_DATA_WRITE_ACTION = 7i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub type IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_RAW_CD_SUBCODE_PQ_ONLY: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_RAW_CD_SUBCODE_IS_COOKED: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_RAW_CD_SUBCODE_IS_RAW: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub type IMAPI_FORMAT2_RAW_CD_WRITE_ACTION = i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_RAW_CD_WRITE_ACTION_UNKNOWN: IMAPI_FORMAT2_RAW_CD_WRITE_ACTION = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_RAW_CD_WRITE_ACTION_PREPARING: IMAPI_FORMAT2_RAW_CD_WRITE_ACTION = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_RAW_CD_WRITE_ACTION_WRITING: IMAPI_FORMAT2_RAW_CD_WRITE_ACTION = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_RAW_CD_WRITE_ACTION_FINISHING: IMAPI_FORMAT2_RAW_CD_WRITE_ACTION = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub type IMAPI_FORMAT2_TAO_WRITE_ACTION = i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_UNKNOWN: IMAPI_FORMAT2_TAO_WRITE_ACTION = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_PREPARING: IMAPI_FORMAT2_TAO_WRITE_ACTION = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_WRITING: IMAPI_FORMAT2_TAO_WRITE_ACTION = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_FINISHING: IMAPI_FORMAT2_TAO_WRITE_ACTION = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_VERIFYING: IMAPI_FORMAT2_TAO_WRITE_ACTION = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub type IMAPI_MEDIA_PHYSICAL_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_UNKNOWN: IMAPI_MEDIA_PHYSICAL_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_CDROM: IMAPI_MEDIA_PHYSICAL_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_CDR: IMAPI_MEDIA_PHYSICAL_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_CDRW: IMAPI_MEDIA_PHYSICAL_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_DVDROM: IMAPI_MEDIA_PHYSICAL_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_DVDRAM: IMAPI_MEDIA_PHYSICAL_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_DVDPLUSR: IMAPI_MEDIA_PHYSICAL_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_DVDPLUSRW: IMAPI_MEDIA_PHYSICAL_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_DVDPLUSR_DUALLAYER: IMAPI_MEDIA_PHYSICAL_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_DVDDASHR: IMAPI_MEDIA_PHYSICAL_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_DVDDASHRW: IMAPI_MEDIA_PHYSICAL_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_DVDDASHR_DUALLAYER: IMAPI_MEDIA_PHYSICAL_TYPE = 11i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_DISK: IMAPI_MEDIA_PHYSICAL_TYPE = 12i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_DVDPLUSRW_DUALLAYER: IMAPI_MEDIA_PHYSICAL_TYPE = 13i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_HDDVDROM: IMAPI_MEDIA_PHYSICAL_TYPE = 14i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_HDDVDR: IMAPI_MEDIA_PHYSICAL_TYPE = 15i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_HDDVDRAM: IMAPI_MEDIA_PHYSICAL_TYPE = 16i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_BDROM: IMAPI_MEDIA_PHYSICAL_TYPE = 17i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_BDR: IMAPI_MEDIA_PHYSICAL_TYPE = 18i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_BDRE: IMAPI_MEDIA_PHYSICAL_TYPE = 19i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_MAX: IMAPI_MEDIA_PHYSICAL_TYPE = 19i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub type IMAPI_MEDIA_WRITE_PROTECT_STATE = i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_WRITEPROTECTED_UNTIL_POWERDOWN: IMAPI_MEDIA_WRITE_PROTECT_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_WRITEPROTECTED_BY_CARTRIDGE: IMAPI_MEDIA_WRITE_PROTECT_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_WRITEPROTECTED_BY_MEDIA_SPECIFIC_REASON: IMAPI_MEDIA_WRITE_PROTECT_STATE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_WRITEPROTECTED_BY_SOFTWARE_WRITE_PROTECT: IMAPI_MEDIA_WRITE_PROTECT_STATE = 8i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_WRITEPROTECTED_BY_DISC_CONTROL_BLOCK: IMAPI_MEDIA_WRITE_PROTECT_STATE = 16i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_WRITEPROTECTED_READ_ONLY_MEDIA: IMAPI_MEDIA_WRITE_PROTECT_STATE = 16384i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub type IMAPI_MODE_PAGE_REQUEST_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MODE_PAGE_REQUEST_TYPE_CURRENT_VALUES: IMAPI_MODE_PAGE_REQUEST_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MODE_PAGE_REQUEST_TYPE_CHANGEABLE_VALUES: IMAPI_MODE_PAGE_REQUEST_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MODE_PAGE_REQUEST_TYPE_DEFAULT_VALUES: IMAPI_MODE_PAGE_REQUEST_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MODE_PAGE_REQUEST_TYPE_SAVED_VALUES: IMAPI_MODE_PAGE_REQUEST_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub type IMAPI_MODE_PAGE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MODE_PAGE_TYPE_READ_WRITE_ERROR_RECOVERY: IMAPI_MODE_PAGE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MODE_PAGE_TYPE_MRW: IMAPI_MODE_PAGE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MODE_PAGE_TYPE_WRITE_PARAMETERS: IMAPI_MODE_PAGE_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MODE_PAGE_TYPE_CACHING: IMAPI_MODE_PAGE_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MODE_PAGE_TYPE_INFORMATIONAL_EXCEPTIONS: IMAPI_MODE_PAGE_TYPE = 28i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MODE_PAGE_TYPE_TIMEOUT_AND_PROTECT: IMAPI_MODE_PAGE_TYPE = 29i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MODE_PAGE_TYPE_POWER_CONDITION: IMAPI_MODE_PAGE_TYPE = 26i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MODE_PAGE_TYPE_LEGACY_CAPABILITIES: IMAPI_MODE_PAGE_TYPE = 42i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub type IMAPI_PROFILE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_INVALID: IMAPI_PROFILE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_NON_REMOVABLE_DISK: IMAPI_PROFILE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_REMOVABLE_DISK: IMAPI_PROFILE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_MO_ERASABLE: IMAPI_PROFILE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_MO_WRITE_ONCE: IMAPI_PROFILE_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_AS_MO: IMAPI_PROFILE_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_CDROM: IMAPI_PROFILE_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_CD_RECORDABLE: IMAPI_PROFILE_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_CD_REWRITABLE: IMAPI_PROFILE_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_DVDROM: IMAPI_PROFILE_TYPE = 16i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_DVD_DASH_RECORDABLE: IMAPI_PROFILE_TYPE = 17i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_DVD_RAM: IMAPI_PROFILE_TYPE = 18i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_DVD_DASH_REWRITABLE: IMAPI_PROFILE_TYPE = 19i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_DVD_DASH_RW_SEQUENTIAL: IMAPI_PROFILE_TYPE = 20i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_DVD_DASH_R_DUAL_SEQUENTIAL: IMAPI_PROFILE_TYPE = 21i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_DVD_DASH_R_DUAL_LAYER_JUMP: IMAPI_PROFILE_TYPE = 22i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_DVD_PLUS_RW: IMAPI_PROFILE_TYPE = 26i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_DVD_PLUS_R: IMAPI_PROFILE_TYPE = 27i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_DDCDROM: IMAPI_PROFILE_TYPE = 32i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_DDCD_RECORDABLE: IMAPI_PROFILE_TYPE = 33i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_DDCD_REWRITABLE: IMAPI_PROFILE_TYPE = 34i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_DVD_PLUS_RW_DUAL: IMAPI_PROFILE_TYPE = 42i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_DVD_PLUS_R_DUAL: IMAPI_PROFILE_TYPE = 43i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_BD_ROM: IMAPI_PROFILE_TYPE = 64i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_BD_R_SEQUENTIAL: IMAPI_PROFILE_TYPE = 65i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_BD_R_RANDOM_RECORDING: IMAPI_PROFILE_TYPE = 66i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_BD_REWRITABLE: IMAPI_PROFILE_TYPE = 67i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_HD_DVD_ROM: IMAPI_PROFILE_TYPE = 80i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_HD_DVD_RECORDABLE: IMAPI_PROFILE_TYPE = 81i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_HD_DVD_RAM: IMAPI_PROFILE_TYPE = 82i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_NON_STANDARD: IMAPI_PROFILE_TYPE = 65535i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub type IMAPI_READ_TRACK_ADDRESS_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_READ_TRACK_ADDRESS_TYPE_LBA: IMAPI_READ_TRACK_ADDRESS_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_READ_TRACK_ADDRESS_TYPE_TRACK: IMAPI_READ_TRACK_ADDRESS_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_READ_TRACK_ADDRESS_TYPE_SESSION: IMAPI_READ_TRACK_ADDRESS_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_BD: u32 = 2195u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_CD: u32 = 75u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_DVD: u32 = 680u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_HD_DVD: u32 = 4568u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_SECTOR_SIZE: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_S_BUFFER_TO_SMALL: ::windows_sys::core::HRESULT = 262657i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_S_PROPERTIESIGNORED: ::windows_sys::core::HRESULT = 262656i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub type IMMPID_CPV_ENUM = i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_CPV_BEFORE__: IMMPID_CPV_ENUM = 32767i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_CP_START: IMMPID_CPV_ENUM = 32768i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_CPV_AFTER__: IMMPID_CPV_ENUM = 32769i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub type IMMPID_MPV_ENUM = i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MPV_BEFORE__: IMMPID_MPV_ENUM = 12287i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MPV_STORE_DRIVER_HANDLE: IMMPID_MPV_ENUM = 12288i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MPV_MESSAGE_CREATION_FLAGS: IMMPID_MPV_ENUM = 12289i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MPV_MESSAGE_OPEN_HANDLES: IMMPID_MPV_ENUM = 12290i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MPV_TOTAL_OPEN_HANDLES: IMMPID_MPV_ENUM = 12291i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MPV_TOTAL_OPEN_PROPERTY_STREAM_HANDLES: IMMPID_MPV_ENUM = 12292i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MPV_TOTAL_OPEN_CONTENT_HANDLES: IMMPID_MPV_ENUM = 12293i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MPV_AFTER__: IMMPID_MPV_ENUM = 12294i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub type IMMPID_MP_ENUM = i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_BEFORE__: IMMPID_MP_ENUM = 4095i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_RECIPIENT_LIST: IMMPID_MP_ENUM = 4096i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_CONTENT_FILE_NAME: IMMPID_MP_ENUM = 4097i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_SENDER_ADDRESS_SMTP: IMMPID_MP_ENUM = 4098i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_SENDER_ADDRESS_X500: IMMPID_MP_ENUM = 4099i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_SENDER_ADDRESS_X400: IMMPID_MP_ENUM = 4100i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_SENDER_ADDRESS_LEGACY_EX_DN: IMMPID_MP_ENUM = 4101i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_DOMAIN_LIST: IMMPID_MP_ENUM = 4102i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_PICKUP_FILE_NAME: IMMPID_MP_ENUM = 4103i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_AUTHENTICATED_USER_NAME: IMMPID_MP_ENUM = 4104i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_CONNECTION_IP_ADDRESS: IMMPID_MP_ENUM = 4105i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_HELO_DOMAIN: IMMPID_MP_ENUM = 4106i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_EIGHTBIT_MIME_OPTION: IMMPID_MP_ENUM = 4107i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_CHUNKING_OPTION: IMMPID_MP_ENUM = 4108i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_BINARYMIME_OPTION: IMMPID_MP_ENUM = 4109i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_REMOTE_AUTHENTICATION_TYPE: IMMPID_MP_ENUM = 4110i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_ERROR_CODE: IMMPID_MP_ENUM = 4111i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_DSN_ENVID_VALUE: IMMPID_MP_ENUM = 4112i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_DSN_RET_VALUE: IMMPID_MP_ENUM = 4113i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_REMOTE_SERVER_DSN_CAPABLE: IMMPID_MP_ENUM = 4114i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_ARRIVAL_TIME: IMMPID_MP_ENUM = 4115i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_MESSAGE_STATUS: IMMPID_MP_ENUM = 4116i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_EXPIRE_DELAY: IMMPID_MP_ENUM = 4117i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_EXPIRE_NDR: IMMPID_MP_ENUM = 4118i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_LOCAL_EXPIRE_DELAY: IMMPID_MP_ENUM = 4119i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_LOCAL_EXPIRE_NDR: IMMPID_MP_ENUM = 4120i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_ARRIVAL_FILETIME: IMMPID_MP_ENUM = 4121i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_HR_CAT_STATUS: IMMPID_MP_ENUM = 4122i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_MSG_GUID: IMMPID_MP_ENUM = 4123i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_SUPERSEDES_MSG_GUID: IMMPID_MP_ENUM = 4124i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_SCANNED_FOR_CRLF_DOT_CRLF: IMMPID_MP_ENUM = 4125i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_FOUND_EMBEDDED_CRLF_DOT_CRLF: IMMPID_MP_ENUM = 4126i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_MSG_SIZE_HINT: IMMPID_MP_ENUM = 4127i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_RFC822_MSG_ID: IMMPID_MP_ENUM = 4128i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_RFC822_MSG_SUBJECT: IMMPID_MP_ENUM = 4129i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_RFC822_FROM_ADDRESS: IMMPID_MP_ENUM = 4130i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_RFC822_TO_ADDRESS: IMMPID_MP_ENUM = 4131i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_RFC822_CC_ADDRESS: IMMPID_MP_ENUM = 4132i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_RFC822_BCC_ADDRESS: IMMPID_MP_ENUM = 4133i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_CONNECTION_SERVER_IP_ADDRESS: IMMPID_MP_ENUM = 4134i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_SERVER_NAME: IMMPID_MP_ENUM = 4135i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_SERVER_VERSION: IMMPID_MP_ENUM = 4136i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_NUM_RECIPIENTS: IMMPID_MP_ENUM = 4137i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_X_PRIORITY: IMMPID_MP_ENUM = 4138i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_FROM_ADDRESS: IMMPID_MP_ENUM = 4139i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_SENDER_ADDRESS: IMMPID_MP_ENUM = 4140i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_DEFERRED_DELIVERY_FILETIME: IMMPID_MP_ENUM = 4141i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_SENDER_ADDRESS_OTHER: IMMPID_MP_ENUM = 4142i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_ORIGINAL_ARRIVAL_TIME: IMMPID_MP_ENUM = 4143i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_MSGCLASS: IMMPID_MP_ENUM = 4144i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_CONTENT_TYPE: IMMPID_MP_ENUM = 4145i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_ENCRYPTION_TYPE: IMMPID_MP_ENUM = 4146i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_CONNECTION_SERVER_PORT: IMMPID_MP_ENUM = 4147i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_CLIENT_AUTH_USER: IMMPID_MP_ENUM = 4148i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_CLIENT_AUTH_TYPE: IMMPID_MP_ENUM = 4149i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_CRC_GLOBAL: IMMPID_MP_ENUM = 4150i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_CRC_RECIPS: IMMPID_MP_ENUM = 4151i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_INBOUND_MAIL_FROM_AUTH: IMMPID_MP_ENUM = 4152i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_AFTER__: IMMPID_MP_ENUM = 4153i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub type IMMPID_NMP_ENUM = i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_NMP_BEFORE__: IMMPID_NMP_ENUM = 24575i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_NMP_SECONDARY_GROUPS: IMMPID_NMP_ENUM = 24576i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_NMP_SECONDARY_ARTNUM: IMMPID_NMP_ENUM = 24577i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_NMP_PRIMARY_GROUP: IMMPID_NMP_ENUM = 24578i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_NMP_PRIMARY_ARTID: IMMPID_NMP_ENUM = 24579i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_NMP_POST_TOKEN: IMMPID_NMP_ENUM = 24580i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_NMP_NEWSGROUP_LIST: IMMPID_NMP_ENUM = 24581i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_NMP_HEADERS: IMMPID_NMP_ENUM = 24582i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_NMP_NNTP_PROCESSING: IMMPID_NMP_ENUM = 24583i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_NMP_NNTP_APPROVED_HEADER: IMMPID_NMP_ENUM = 24584i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_NMP_AFTER__: IMMPID_NMP_ENUM = 24585i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub type IMMPID_RPV_ENUM = i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RPV_BEFORE__: IMMPID_RPV_ENUM = 16383i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RPV_DONT_DELIVER: IMMPID_RPV_ENUM = 16384i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RPV_NO_NAME_COLLISIONS: IMMPID_RPV_ENUM = 16385i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RPV_AFTER__: IMMPID_RPV_ENUM = 16386i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub type IMMPID_RP_ENUM = i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_BEFORE__: IMMPID_RP_ENUM = 8191i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_DSN_NOTIFY_SUCCESS: IMMPID_RP_ENUM = 8192i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_DSN_NOTIFY_INVALID: IMMPID_RP_ENUM = 8193i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_ADDRESS_TYPE: IMMPID_RP_ENUM = 8194i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_ADDRESS: IMMPID_RP_ENUM = 8195i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_ADDRESS_TYPE_SMTP: IMMPID_RP_ENUM = 8196i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_ERROR_CODE: IMMPID_RP_ENUM = 8197i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_ERROR_STRING: IMMPID_RP_ENUM = 8198i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_DSN_NOTIFY_VALUE: IMMPID_RP_ENUM = 8199i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_DSN_ORCPT_VALUE: IMMPID_RP_ENUM = 8200i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_ADDRESS_SMTP: IMMPID_RP_ENUM = 8201i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_ADDRESS_X400: IMMPID_RP_ENUM = 8202i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_ADDRESS_X500: IMMPID_RP_ENUM = 8203i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_LEGACY_EX_DN: IMMPID_RP_ENUM = 8204i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_RECIPIENT_FLAGS: IMMPID_RP_ENUM = 8205i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_SMTP_STATUS_STRING: IMMPID_RP_ENUM = 8206i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_DSN_PRE_CAT_ADDRESS: IMMPID_RP_ENUM = 8207i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_MDB_GUID: IMMPID_RP_ENUM = 8208i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_USER_GUID: IMMPID_RP_ENUM = 8209i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_DOMAIN: IMMPID_RP_ENUM = 8210i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_ADDRESS_OTHER: IMMPID_RP_ENUM = 8211i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_DISPLAY_NAME: IMMPID_RP_ENUM = 8212i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_AFTER__: IMMPID_RP_ENUM = 8213i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub struct IMMP_MPV_STORE_DRIVER_HANDLE {
    pub guidSignature: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for IMMP_MPV_STORE_DRIVER_HANDLE {}
impl ::core::clone::Clone for IMMP_MPV_STORE_DRIVER_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMultisession {
    pub base__: super::super::System::Com::IDispatch,
    pub IsSupportedOnCurrentMediaState: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetInUse: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    pub InUse: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ImportRecorder: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ImportRecorder: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IMultisession {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801552, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMultisessionRandomWrite {
    pub base__: IMultisession,
    pub WriteUnitSize: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub LastWrittenAddress: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub TotalSectorsOnMedia: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IMultisessionRandomWrite {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3037186595, data2: 8708, data3: 4573, data4: [150, 106, 0, 26, 160, 27, 188, 88] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMultisessionSequential {
    pub base__: IMultisession,
    pub IsFirstDataSession: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i16) -> ::windows_sys::core::HRESULT,
    pub StartAddressOfPreviousSession: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub LastWrittenAddressOfPreviousSession: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub NextWritableAddress: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub FreeSectorsOnMedia: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IMultisessionSequential {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801553, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMultisessionSequential2 {
    pub base__: IMultisessionSequential,
    pub WriteUnitSize: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IMultisessionSequential2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3037186594, data2: 8708, data3: 4573, data4: [150, 106, 0, 26, 160, 27, 188, 88] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IProgressItem {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, desc: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    pub FirstBlock: unsafe extern "system" fn(this: *mut *mut Self, block: *mut u32) -> ::windows_sys::core::HRESULT,
    pub LastBlock: unsafe extern "system" fn(this: *mut *mut Self, block: *mut u32) -> ::windows_sys::core::HRESULT,
    pub BlockCount: unsafe extern "system" fn(this: *mut *mut Self, blocks: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IProgressItem {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 747904981, data2: 38747, data3: 22974, data4: [169, 96, 154, 42, 38, 40, 83, 165] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IProgressItems {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, item: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, count: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ProgressItemFromBlock: unsafe extern "system" fn(this: *mut *mut Self, block: u32, item: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ProgressItemFromBlock: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub ProgressItemFromDescription: unsafe extern "system" fn(this: *mut *mut Self, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, item: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    ProgressItemFromDescription: usize,
    pub EnumProgressItems: unsafe extern "system" fn(this: *mut *mut Self, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IProgressItems {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 747904983, data2: 38747, data3: 22974, data4: [169, 96, 154, 42, 38, 40, 83, 165] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRawCDImageCreator {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateResultImage: unsafe extern "system" fn(this: *mut *mut Self, resultstream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateResultImage: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddTrack: unsafe extern "system" fn(this: *mut *mut Self, datatype: IMAPI_CD_SECTOR_TYPE, data: *mut ::core::ffi::c_void, trackindex: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddTrack: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddSpecialPregap: unsafe extern "system" fn(this: *mut *mut Self, data: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddSpecialPregap: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddSubcodeRWGenerator: unsafe extern "system" fn(this: *mut *mut Self, subcode: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddSubcodeRWGenerator: usize,
    pub SetResultingImageType: unsafe extern "system" fn(this: *mut *mut Self, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows_sys::core::HRESULT,
    pub ResultingImageType: unsafe extern "system" fn(this: *mut *mut Self, value: *mut IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows_sys::core::HRESULT,
    pub StartOfLeadout: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetStartOfLeadoutLimit: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub StartOfLeadoutLimit: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetDisableGaplessAudio: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    pub DisableGaplessAudio: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMediaCatalogNumber: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMediaCatalogNumber: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MediaCatalogNumber: unsafe extern "system" fn(this: *mut *mut Self, value: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MediaCatalogNumber: usize,
    pub SetStartingTrackNumber: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub StartingTrackNumber: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_TrackInfo: unsafe extern "system" fn(this: *mut *mut Self, trackindex: i32, value: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_TrackInfo: usize,
    pub NumberOfExistingTracks: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub LastUsedUserSectorInImage: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ExpectedTableOfContents: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExpectedTableOfContents: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IRawCDImageCreator {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 630732112, data2: 40293, data3: 18894, data4: [179, 53, 64, 99, 13, 144, 18, 39] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRawCDImageTrackInfo {
    pub base__: super::super::System::Com::IDispatch,
    pub StartingLba: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SectorCount: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub TrackNumber: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SectorType: unsafe extern "system" fn(this: *mut *mut Self, value: *mut IMAPI_CD_SECTOR_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ISRC: unsafe extern "system" fn(this: *mut *mut Self, value: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ISRC: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetISRC: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetISRC: usize,
    pub DigitalAudioCopySetting: unsafe extern "system" fn(this: *mut *mut Self, value: *mut IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> ::windows_sys::core::HRESULT,
    pub SetDigitalAudioCopySetting: unsafe extern "system" fn(this: *mut *mut Self, value: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> ::windows_sys::core::HRESULT,
    pub AudioHasPreemphasis: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAudioHasPreemphasis: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub TrackIndexes: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TrackIndexes: usize,
    pub AddTrackIndex: unsafe extern "system" fn(this: *mut *mut Self, lbaoffset: i32) -> ::windows_sys::core::HRESULT,
    pub ClearTrackIndex: unsafe extern "system" fn(this: *mut *mut Self, lbaoffset: i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IRawCDImageTrackInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 630732113, data2: 40293, data3: 18894, data4: [179, 53, 64, 99, 13, 144, 18, 39] };
}
#[repr(C)]
pub struct IRedbookDiscMaster {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetTotalAudioTracks: unsafe extern "system" fn(this: *mut *mut Self, pntracks: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetTotalAudioBlocks: unsafe extern "system" fn(this: *mut *mut Self, pnblocks: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetUsedAudioBlocks: unsafe extern "system" fn(this: *mut *mut Self, pnblocks: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetAvailableAudioTrackBlocks: unsafe extern "system" fn(this: *mut *mut Self, pnblocks: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetAudioBlockSize: unsafe extern "system" fn(this: *mut *mut Self, pnblockbytes: *mut i32) -> ::windows_sys::core::HRESULT,
    pub CreateAudioTrack: unsafe extern "system" fn(this: *mut *mut Self, nblocks: i32) -> ::windows_sys::core::HRESULT,
    pub AddAudioTrackBlocks: unsafe extern "system" fn(this: *mut *mut Self, pby: *const u8, cb: i32) -> ::windows_sys::core::HRESULT,
    pub CloseAudioTrack: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRedbookDiscMaster {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3820765901, data2: 20060, data3: 4563, data4: [145, 68, 0, 16, 75, 161, 28, 94] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IStreamConcatenate {
    pub base__: super::super::System::Com::IStream,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, stream1: *mut ::core::ffi::c_void, stream2: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize2: unsafe extern "system" fn(this: *mut *mut Self, streams: *const *mut ::core::ffi::c_void, streamcount: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Append: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Append: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Append2: unsafe extern "system" fn(this: *mut *mut Self, streams: *const *mut ::core::ffi::c_void, streamcount: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Append2: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IStreamConcatenate {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801542, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IStreamInterleave {
    pub base__: super::super::System::Com::IStream,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, streams: *const *mut ::core::ffi::c_void, interleavesizes: *const u32, streamcount: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IStreamInterleave {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801543, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IStreamPseudoRandomBased {
    pub base__: super::super::System::Com::IStream,
    pub SetSeed: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub Seed: unsafe extern "system" fn(this: *mut *mut Self, value: *mut u32) -> ::windows_sys::core::HRESULT,
    pub put_ExtendedSeed: unsafe extern "system" fn(this: *mut *mut Self, values: *const u32, ecount: u32) -> ::windows_sys::core::HRESULT,
    pub get_ExtendedSeed: unsafe extern "system" fn(this: *mut *mut Self, values: *mut *mut u32, ecount: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IStreamPseudoRandomBased {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801541, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWriteEngine2 {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub WriteSection: unsafe extern "system" fn(this: *mut *mut Self, data: *mut ::core::ffi::c_void, startingblockaddress: i32, numberofblocks: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    WriteSection: usize,
    pub CancelWrite: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetRecorder: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Recorder: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetUseStreamingWrite12: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    pub UseStreamingWrite12: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetStartingSectorsPerSecond: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub StartingSectorsPerSecond: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetEndingSectorsPerSecond: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub EndingSectorsPerSecond: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetBytesPerSector: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub BytesPerSector: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub WriteInProgress: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IWriteEngine2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801525, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWriteEngine2EventArgs {
    pub base__: super::super::System::Com::IDispatch,
    pub StartLba: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SectorCount: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub LastReadLba: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub LastWrittenLba: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub TotalSystemBuffer: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub UsedSystemBuffer: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
    pub FreeSystemBuffer: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IWriteEngine2EventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801526, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWriteSpeedDescriptor {
    pub base__: super::super::System::Com::IDispatch,
    pub MediaType: unsafe extern "system" fn(this: *mut *mut Self, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows_sys::core::HRESULT,
    pub RotationTypeIsPureCAV: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i16) -> ::windows_sys::core::HRESULT,
    pub WriteSpeed: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IWriteSpeedDescriptor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801540, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub type MEDIA_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MEDIA_BLANK: MEDIA_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MEDIA_RW: MEDIA_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MEDIA_WRITABLE: MEDIA_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MEDIA_FORMAT_UNUSABLE_BY_IMAPI: MEDIA_FLAGS = 8i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub type MEDIA_TYPES = i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MEDIA_CDDA_CDROM: MEDIA_TYPES = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MEDIA_CD_ROM_XA: MEDIA_TYPES = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MEDIA_CD_I: MEDIA_TYPES = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MEDIA_CD_EXTRA: MEDIA_TYPES = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MEDIA_CD_OTHER: MEDIA_TYPES = 5i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MEDIA_SPECIAL: MEDIA_TYPES = 6i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MPV_INBOUND_CUTOFF_EXCEEDED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MPV_WRITE_CONTENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MP_MSGCLASS_DELIVERY_REPORT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MP_MSGCLASS_NONDELIVERY_REPORT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MP_MSGCLASS_REPLICATION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MP_MSGCLASS_SYSTEM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MP_STATUS_ABANDON_DELIVERY: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MP_STATUS_ABORT_DELIVERY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MP_STATUS_BAD_MAIL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MP_STATUS_CATEGORIZED: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MP_STATUS_RETRY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MP_STATUS_SUBMITTED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MP_STATUS_SUCCESS: u32 = 0u32;
pub const MSDiscMasterObj: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1376569955, data2: 20901, data3: 4563, data4: [145, 68, 0, 16, 75, 161, 28, 94] };
pub const MSDiscRecorderObj: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1376569953, data2: 20901, data3: 4563, data4: [145, 68, 0, 16, 75, 161, 28, 94] };
pub const MSEnumDiscRecordersObj: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2315474554, data2: 25547, data3: 19368, data4: [186, 246, 82, 17, 152, 22, 209, 239] };
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_AddressBook\"`*"]
#[cfg(feature = "Win32_System_AddressBook")]
pub type MSGCALLRELEASE = ::core::option::Option<unsafe extern "system" fn(ulcallerdata: u32, lpmessage: *mut *mut super::super::System::AddressBook::IMessage)>;
pub const MsftDiscFormat2Data: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801514, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
pub const MsftDiscFormat2Erase: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801515, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
pub const MsftDiscFormat2RawCD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801512, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
pub const MsftDiscFormat2TrackAtOnce: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801513, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
pub const MsftDiscMaster2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801518, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
pub const MsftDiscRecorder2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801517, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
pub const MsftFileSystemImage: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 747904965, data2: 38747, data3: 22974, data4: [169, 96, 154, 42, 38, 40, 83, 165] };
pub const MsftIsoImageManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3471719266, data2: 36694, data3: 16470, data4: [134, 155, 239, 22, 145, 126, 62, 252] };
pub const MsftMultisessionRandomWrite: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3037186596, data2: 8708, data3: 4573, data4: [150, 106, 0, 26, 160, 27, 188, 88] };
pub const MsftMultisessionSequential: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801506, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
pub const MsftRawCDImageCreator: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 630732129, data2: 40293, data3: 18894, data4: [179, 53, 64, 99, 13, 144, 18, 39] };
pub const MsftStreamConcatenate: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801509, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
pub const MsftStreamInterleave: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801508, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
pub const MsftStreamPrng001: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801510, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
pub const MsftStreamZero: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801511, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
pub const MsftWriteEngine2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801516, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
pub const MsftWriteSpeedDescriptor: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801507, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const NMP_PROCESS_CONTROL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const NMP_PROCESS_MODERATOR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const NMP_PROCESS_POST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub type PlatformId = i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const PlatformX86: PlatformId = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const PlatformPowerPC: PlatformId = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const PlatformMac: PlatformId = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const PlatformEFI: PlatformId = 239i32;
pub const ProgressItem: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 747904971, data2: 38747, data3: 22974, data4: [169, 96, 154, 42, 38, 40, 83, 165] };
pub const ProgressItems: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 747904969, data2: 38747, data3: 22974, data4: [169, 96, 154, 42, 38, 40, 83, 165] };
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub type RECORDER_TYPES = i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RECORDER_CDR: RECORDER_TYPES = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RECORDER_CDRW: RECORDER_TYPES = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_DELIVERED: u32 = 272u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_DSN_HANDLED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_DSN_NOTIFY_DELAY: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_DSN_NOTIFY_FAILURE: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_DSN_NOTIFY_INVALID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_DSN_NOTIFY_MASK: u32 = 251658240u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_DSN_NOTIFY_NEVER: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_DSN_NOTIFY_SUCCESS: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_DSN_SENT_DELAYED: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_DSN_SENT_DELIVERED: u32 = 131136u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_DSN_SENT_EXPANDED: u32 = 32832u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_DSN_SENT_NDR: u32 = 1104u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_DSN_SENT_RELAYED: u32 = 65600u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_ENPANDED: u32 = 8208u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_ERROR_CONTEXT_CAT: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_ERROR_CONTEXT_MTA: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_ERROR_CONTEXT_STORE: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_EXPANDED: u32 = 8208u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_FAILED: u32 = 2096u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_GENERAL_FAILURE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_HANDLED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_RECIP_FLAGS_RESERVED: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_REMOTE_MTA_NO_DSN: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_UNRESOLVED: u32 = 4144u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_VOLATILE_FLAGS_MASK: u32 = 4026531840u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub struct SPropAttrArray {
    pub cValues: u32,
    pub aPropAttr: [u32; 1],
}
impl ::core::marker::Copy for SPropAttrArray {}
impl ::core::clone::Clone for SPropAttrArray {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const SZ_PROGID_SMTPCAT: &str = "Smtp.Cat";
#[repr(C)]
pub struct _MSGSESS(pub u8);
pub const tagIMMPID_CPV_STRUCT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2728880938, data2: 58669, data3: 4561, data4: [170, 100, 0, 192, 79, 163, 91, 130] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub struct tagIMMPID_GUIDLIST_ITEM {
    pub pguid: *const ::windows_sys::core::GUID,
    pub dwStart: u32,
    pub dwLast: u32,
}
impl ::core::marker::Copy for tagIMMPID_GUIDLIST_ITEM {}
impl ::core::clone::Clone for tagIMMPID_GUIDLIST_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
pub const tagIMMPID_MPV_STRUCT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3420886790, data2: 51645, data3: 4561, data4: [159, 242, 0, 192, 79, 163, 115, 72] };
pub const tagIMMPID_MP_STRUCT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 322456816, data2: 46020, data3: 4561, data4: [170, 146, 0, 170, 0, 107, 200, 11] };
pub const tagIMMPID_NMP_STRUCT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1949542826, data2: 8418, data3: 4562, data4: [148, 214, 0, 192, 79, 163, 121, 241] };
pub const tagIMMPID_RPV_STRUCT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2045255753, data2: 54048, data3: 4561, data4: [159, 244, 0, 192, 79, 163, 115, 72] };
pub const tagIMMPID_RP_STRUCT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2045255752, data2: 54048, data3: 4561, data4: [159, 244, 0, 192, 79, 163, 115, 72] };
