pub const CEventClass: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3451832768, data2: 31336, data3: 4561, data4: [136, 249, 0, 128, 199, 215, 113, 191] };
pub const CEventPublisher: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2878621216, data2: 31174, data3: 4561, data4: [136, 249, 0, 128, 199, 215, 113, 191] };
pub const CEventSubscription: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1967319392, data2: 31175, data3: 4561, data4: [136, 249, 0, 128, 199, 215, 113, 191] };
pub const CEventSystem: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1309997986, data2: 11810, data3: 4561, data4: [153, 100, 0, 192, 79, 187, 179, 69] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct COMEVENTSYSCHANGEINFO {
    pub cbSize: u32,
    pub changeType: EOC_ChangeType,
    pub objectId: super::super::super::Foundation::BSTR,
    pub partitionId: super::super::super::Foundation::BSTR,
    pub applicationId: super::super::super::Foundation::BSTR,
    pub reserved: [::windows_sys::core::GUID; 10],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COMEVENTSYSCHANGEINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COMEVENTSYSCHANGEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub type EOC_ChangeType = i32;
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub const EOC_NewObject: EOC_ChangeType = 0i32;
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub const EOC_ModifiedObject: EOC_ChangeType = 1i32;
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub const EOC_DeletedObject: EOC_ChangeType = 2i32;
pub const EventObjectChange: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3495317504, data2: 40436, data3: 4561, data4: [162, 129, 0, 192, 79, 202, 10, 167] };
pub const EventObjectChange2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3137845965, data2: 52566, data3: 20067, data4: [168, 255, 203, 240, 53, 95, 185, 244] };
#[repr(C)]
pub struct IDontSupportEventSubscription {
    pub base__: ::windows_sys::core::IUnknown,
}
impl ::windows_sys::core::Interface for IDontSupportEventSubscription {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2017534449, data2: 25254, data3: 19337, data4: [133, 95, 214, 95, 41, 109, 232, 58] };
}
#[repr(C)]
pub struct IEnumEventObject {
    pub base__: ::windows_sys::core::IUnknown,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, creqelem: u32, ppinterface: *mut *mut ::core::ffi::c_void, cretelem: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, cskipelem: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumEventObject {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4104158563, data2: 11813, data3: 4561, data4: [153, 100, 0, 192, 79, 187, 179, 69] };
}
#[repr(C)]
pub struct IEventClass {
    pub base__: super::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub EventClassID: unsafe extern "system" fn(this: *mut *mut Self, pbstreventclassid: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EventClassID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEventClassID: unsafe extern "system" fn(this: *mut *mut Self, bstreventclassid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEventClassID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EventClassName: unsafe extern "system" fn(this: *mut *mut Self, pbstreventclassname: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EventClassName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEventClassName: unsafe extern "system" fn(this: *mut *mut Self, bstreventclassname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEventClassName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OwnerSID: unsafe extern "system" fn(this: *mut *mut Self, pbstrownersid: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OwnerSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetOwnerSID: unsafe extern "system" fn(this: *mut *mut Self, bstrownersid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetOwnerSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FiringInterfaceID: unsafe extern "system" fn(this: *mut *mut Self, pbstrfiringinterfaceid: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FiringInterfaceID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFiringInterfaceID: unsafe extern "system" fn(this: *mut *mut Self, bstrfiringinterfaceid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFiringInterfaceID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, pbstrdescription: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, bstrdescription: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CustomConfigCLSID: unsafe extern "system" fn(this: *mut *mut Self, pbstrcustomconfigclsid: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CustomConfigCLSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCustomConfigCLSID: unsafe extern "system" fn(this: *mut *mut Self, bstrcustomconfigclsid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCustomConfigCLSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TypeLib: unsafe extern "system" fn(this: *mut *mut Self, pbstrtypelib: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TypeLib: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTypeLib: unsafe extern "system" fn(this: *mut *mut Self, bstrtypelib: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTypeLib: usize,
}
impl ::windows_sys::core::Interface for IEventClass {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4213928608, data2: 31336, data3: 4561, data4: [136, 249, 0, 128, 199, 215, 113, 191] };
}
#[repr(C)]
pub struct IEventClass2 {
    pub base__: IEventClass,
    #[cfg(feature = "Win32_Foundation")]
    pub PublisherID: unsafe extern "system" fn(this: *mut *mut Self, pbstrpublisherid: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PublisherID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPublisherID: unsafe extern "system" fn(this: *mut *mut Self, bstrpublisherid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPublisherID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MultiInterfacePublisherFilterCLSID: unsafe extern "system" fn(this: *mut *mut Self, pbstrpubfilclsid: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MultiInterfacePublisherFilterCLSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMultiInterfacePublisherFilterCLSID: unsafe extern "system" fn(this: *mut *mut Self, bstrpubfilclsid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMultiInterfacePublisherFilterCLSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AllowInprocActivation: unsafe extern "system" fn(this: *mut *mut Self, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AllowInprocActivation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllowInprocActivation: unsafe extern "system" fn(this: *mut *mut Self, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllowInprocActivation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FireInParallel: unsafe extern "system" fn(this: *mut *mut Self, pffireinparallel: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FireInParallel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFireInParallel: unsafe extern "system" fn(this: *mut *mut Self, ffireinparallel: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFireInParallel: usize,
}
impl ::windows_sys::core::Interface for IEventClass2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4213928609, data2: 31336, data3: 4561, data4: [136, 249, 0, 128, 199, 215, 113, 191] };
}
#[repr(C)]
pub struct IEventControl {
    pub base__: super::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPublisherFilter: unsafe extern "system" fn(this: *mut *mut Self, methodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppublisherfilter: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPublisherFilter: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AllowInprocActivation: unsafe extern "system" fn(this: *mut *mut Self, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AllowInprocActivation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllowInprocActivation: unsafe extern "system" fn(this: *mut *mut Self, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllowInprocActivation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSubscriptions: unsafe extern "system" fn(this: *mut *mut Self, methodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, optionalcriteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, optionalerrorindex: *const i32, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSubscriptions: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDefaultQuery: unsafe extern "system" fn(this: *mut *mut Self, methodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, criteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, errorindex: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDefaultQuery: usize,
}
impl ::windows_sys::core::Interface for IEventControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 54780660, data2: 34550, data3: 4561, data4: [183, 96, 0, 192, 79, 185, 38, 175] };
}
#[repr(C)]
pub struct IEventObjectChange {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub ChangedSubscription: unsafe extern "system" fn(this: *mut *mut Self, changetype: EOC_ChangeType, bstrsubscriptionid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ChangedSubscription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ChangedEventClass: unsafe extern "system" fn(this: *mut *mut Self, changetype: EOC_ChangeType, bstreventclassid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ChangedEventClass: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ChangedPublisher: unsafe extern "system" fn(this: *mut *mut Self, changetype: EOC_ChangeType, bstrpublisherid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ChangedPublisher: usize,
}
impl ::windows_sys::core::Interface for IEventObjectChange {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4104158576, data2: 11813, data3: 4561, data4: [153, 100, 0, 192, 79, 187, 179, 69] };
}
#[repr(C)]
pub struct IEventObjectChange2 {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub ChangedSubscription: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ChangedSubscription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ChangedEventClass: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ChangedEventClass: usize,
}
impl ::windows_sys::core::Interface for IEventObjectChange2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1996597699, data2: 48488, data3: 17295, data4: [131, 224, 103, 191, 79, 83, 164, 34] };
}
#[repr(C)]
pub struct IEventObjectCollection {
    pub base__: super::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppunkenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, objectid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pitem: *mut super::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pcount: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, item: *const super::VARIANT, objectid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    Add: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, objectid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Remove: usize,
}
impl ::windows_sys::core::Interface for IEventObjectCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4170891888, data2: 54507, data3: 4561, data4: [182, 130, 0, 128, 95, 199, 146, 22] };
}
#[repr(C)]
pub struct IEventProperty {
    pub base__: super::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, propertyname: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, propertyvalue: *mut super::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    Value: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, propertyvalue: *const super::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    SetValue: usize,
}
impl ::windows_sys::core::Interface for IEventProperty {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3662909154, data2: 62686, data3: 4561, data4: [182, 187, 0, 128, 95, 199, 146, 22] };
}
#[repr(C)]
pub struct IEventPublisher {
    pub base__: super::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub PublisherID: unsafe extern "system" fn(this: *mut *mut Self, pbstrpublisherid: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PublisherID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPublisherID: unsafe extern "system" fn(this: *mut *mut Self, bstrpublisherid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPublisherID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PublisherName: unsafe extern "system" fn(this: *mut *mut Self, pbstrpublishername: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PublisherName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPublisherName: unsafe extern "system" fn(this: *mut *mut Self, bstrpublishername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPublisherName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PublisherType: unsafe extern "system" fn(this: *mut *mut Self, pbstrpublishertype: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PublisherType: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPublisherType: unsafe extern "system" fn(this: *mut *mut Self, bstrpublishertype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPublisherType: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OwnerSID: unsafe extern "system" fn(this: *mut *mut Self, pbstrownersid: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OwnerSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetOwnerSID: unsafe extern "system" fn(this: *mut *mut Self, bstrownersid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetOwnerSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, pbstrdescription: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, bstrdescription: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetDefaultProperty: unsafe extern "system" fn(this: *mut *mut Self, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: *mut super::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetDefaultProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub PutDefaultProperty: unsafe extern "system" fn(this: *mut *mut Self, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: *const super::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    PutDefaultProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoveDefaultProperty: unsafe extern "system" fn(this: *mut *mut Self, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoveDefaultProperty: usize,
    pub GetDefaultPropertyCollection: unsafe extern "system" fn(this: *mut *mut Self, collection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEventPublisher {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3812708715, data2: 11826, data3: 4561, data4: [153, 100, 0, 192, 79, 187, 179, 69] };
}
#[repr(C)]
pub struct IEventSubscription {
    pub base__: super::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub SubscriptionID: unsafe extern "system" fn(this: *mut *mut Self, pbstrsubscriptionid: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SubscriptionID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSubscriptionID: unsafe extern "system" fn(this: *mut *mut Self, bstrsubscriptionid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSubscriptionID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SubscriptionName: unsafe extern "system" fn(this: *mut *mut Self, pbstrsubscriptionname: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SubscriptionName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSubscriptionName: unsafe extern "system" fn(this: *mut *mut Self, bstrsubscriptionname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSubscriptionName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PublisherID: unsafe extern "system" fn(this: *mut *mut Self, pbstrpublisherid: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PublisherID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPublisherID: unsafe extern "system" fn(this: *mut *mut Self, bstrpublisherid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPublisherID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EventClassID: unsafe extern "system" fn(this: *mut *mut Self, pbstreventclassid: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EventClassID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEventClassID: unsafe extern "system" fn(this: *mut *mut Self, bstreventclassid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEventClassID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MethodName: unsafe extern "system" fn(this: *mut *mut Self, pbstrmethodname: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MethodName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMethodName: unsafe extern "system" fn(this: *mut *mut Self, bstrmethodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMethodName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SubscriberCLSID: unsafe extern "system" fn(this: *mut *mut Self, pbstrsubscriberclsid: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SubscriberCLSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSubscriberCLSID: unsafe extern "system" fn(this: *mut *mut Self, bstrsubscriberclsid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSubscriberCLSID: usize,
    pub SubscriberInterface: unsafe extern "system" fn(this: *mut *mut Self, ppsubscriberinterface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSubscriberInterface: unsafe extern "system" fn(this: *mut *mut Self, psubscriberinterface: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub PerUser: unsafe extern "system" fn(this: *mut *mut Self, pfperuser: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PerUser: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPerUser: unsafe extern "system" fn(this: *mut *mut Self, fperuser: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPerUser: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OwnerSID: unsafe extern "system" fn(this: *mut *mut Self, pbstrownersid: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OwnerSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetOwnerSID: unsafe extern "system" fn(this: *mut *mut Self, bstrownersid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetOwnerSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, fenabled: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, pbstrdescription: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, bstrdescription: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MachineName: unsafe extern "system" fn(this: *mut *mut Self, pbstrmachinename: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MachineName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMachineName: unsafe extern "system" fn(this: *mut *mut Self, bstrmachinename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMachineName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetPublisherProperty: unsafe extern "system" fn(this: *mut *mut Self, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: *mut super::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetPublisherProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub PutPublisherProperty: unsafe extern "system" fn(this: *mut *mut Self, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: *const super::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    PutPublisherProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RemovePublisherProperty: unsafe extern "system" fn(this: *mut *mut Self, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemovePublisherProperty: usize,
    pub GetPublisherPropertyCollection: unsafe extern "system" fn(this: *mut *mut Self, collection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetSubscriberProperty: unsafe extern "system" fn(this: *mut *mut Self, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: *mut super::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetSubscriberProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub PutSubscriberProperty: unsafe extern "system" fn(this: *mut *mut Self, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: *const super::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    PutSubscriberProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoveSubscriberProperty: unsafe extern "system" fn(this: *mut *mut Self, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoveSubscriberProperty: usize,
    pub GetSubscriberPropertyCollection: unsafe extern "system" fn(this: *mut *mut Self, collection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub InterfaceID: unsafe extern "system" fn(this: *mut *mut Self, pbstrinterfaceid: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InterfaceID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetInterfaceID: unsafe extern "system" fn(this: *mut *mut Self, bstrinterfaceid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetInterfaceID: usize,
}
impl ::windows_sys::core::Interface for IEventSubscription {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1248529941, data2: 11832, data3: 4561, data4: [153, 101, 0, 192, 79, 187, 179, 69] };
}
#[repr(C)]
pub struct IEventSystem {
    pub base__: super::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Query: unsafe extern "system" fn(this: *mut *mut Self, progid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, querycriteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, errorindex: *mut i32, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Query: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Store: unsafe extern "system" fn(this: *mut *mut Self, progid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pinterface: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Store: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, progid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, querycriteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, errorindex: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Remove: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EventObjectChangeEventClassID: unsafe extern "system" fn(this: *mut *mut Self, pbstreventclassid: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EventObjectChangeEventClassID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryS: unsafe extern "system" fn(this: *mut *mut Self, progid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, querycriteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryS: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoveS: unsafe extern "system" fn(this: *mut *mut Self, progid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, querycriteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoveS: usize,
}
impl ::windows_sys::core::Interface for IEventSystem {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1309997983, data2: 11810, data3: 4561, data4: [153, 100, 0, 192, 79, 187, 179, 69] };
}
#[repr(C)]
pub struct IFiringControl {
    pub base__: super::IDispatch,
    pub FireSubscription: unsafe extern "system" fn(this: *mut *mut Self, subscription: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFiringControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3762916499, data2: 20222, data3: 4561, data4: [153, 113, 0, 192, 79, 187, 179, 69] };
}
#[repr(C)]
pub struct IMultiInterfaceEventControl {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetMultiInterfacePublisherFilter: unsafe extern "system" fn(this: *mut *mut Self, classfilter: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSubscriptions: unsafe extern "system" fn(this: *mut *mut Self, eventiid: *const ::windows_sys::core::GUID, bstrmethodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, optionalcriteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, optionalerrorindex: *const i32, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSubscriptions: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDefaultQuery: unsafe extern "system" fn(this: *mut *mut Self, eventiid: *const ::windows_sys::core::GUID, bstrmethodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrcriteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, errorindex: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDefaultQuery: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AllowInprocActivation: unsafe extern "system" fn(this: *mut *mut Self, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AllowInprocActivation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllowInprocActivation: unsafe extern "system" fn(this: *mut *mut Self, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllowInprocActivation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FireInParallel: unsafe extern "system" fn(this: *mut *mut Self, pffireinparallel: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FireInParallel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFireInParallel: unsafe extern "system" fn(this: *mut *mut Self, ffireinparallel: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFireInParallel: usize,
}
impl ::windows_sys::core::Interface for IMultiInterfaceEventControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 54780661, data2: 34550, data3: 4561, data4: [183, 96, 0, 192, 79, 185, 38, 175] };
}
#[repr(C)]
pub struct IMultiInterfacePublisherFilter {
    pub base__: ::windows_sys::core::IUnknown,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, peic: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub PrepareToFire: unsafe extern "system" fn(this: *mut *mut Self, iid: *const ::windows_sys::core::GUID, methodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, firingcontrol: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PrepareToFire: usize,
}
impl ::windows_sys::core::Interface for IMultiInterfacePublisherFilter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1180589249, data2: 31526, data3: 4561, data4: [136, 251, 0, 128, 199, 215, 113, 191] };
}
#[repr(C)]
pub struct IPublisherFilter {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, methodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, dispuserdefined: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PrepareToFire: unsafe extern "system" fn(this: *mut *mut Self, methodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, firingcontrol: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PrepareToFire: usize,
}
impl ::windows_sys::core::Interface for IPublisherFilter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1180589248, data2: 31526, data3: 4561, data4: [136, 251, 0, 128, 199, 215, 113, 191] };
}
