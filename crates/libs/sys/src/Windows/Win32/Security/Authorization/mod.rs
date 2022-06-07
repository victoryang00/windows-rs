#[cfg(feature = "Win32_Security_Authorization_UI")]
pub mod UI;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzAccessCheck(flags: AUTHZ_ACCESS_CHECK_FLAGS, hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, prequest: *const AUTHZ_ACCESS_REQUEST, hauditevent: AUTHZ_AUDIT_EVENT_HANDLE, psecuritydescriptor: super::PSECURITY_DESCRIPTOR, optionalsecuritydescriptorarray: *const super::PSECURITY_DESCRIPTOR, optionalsecuritydescriptorcount: u32, preply: *mut AUTHZ_ACCESS_REPLY, phaccesscheckresults: *mut isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzAddSidsToContext(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, sids: *const super::SID_AND_ATTRIBUTES, sidcount: u32, restrictedsids: *const super::SID_AND_ATTRIBUTES, restrictedsidcount: u32, phnewauthzclientcontext: *mut isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzCachedAccessCheck(flags: u32, haccesscheckresults: AUTHZ_ACCESS_CHECK_RESULTS_HANDLE, prequest: *const AUTHZ_ACCESS_REQUEST, hauditevent: AUTHZ_AUDIT_EVENT_HANDLE, preply: *mut AUTHZ_ACCESS_REPLY) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzEnumerateSecurityEventSources(dwflags: u32, buffer: *mut AUTHZ_SOURCE_SCHEMA_REGISTRATION, pdwcount: *mut u32, pdwlength: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzEvaluateSacl(authzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, prequest: *const AUTHZ_ACCESS_REQUEST, sacl: *const super::ACL, grantedaccess: u32, accessgranted: super::super::Foundation::BOOL, pbgenerateaudit: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzFreeAuditEvent(hauditevent: AUTHZ_AUDIT_EVENT_HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzFreeCentralAccessPolicyCache() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzFreeContext(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzFreeHandle(haccesscheckresults: AUTHZ_ACCESS_CHECK_RESULTS_HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzFreeResourceManager(hauthzresourcemanager: AUTHZ_RESOURCE_MANAGER_HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzGetInformationFromContext(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, infoclass: AUTHZ_CONTEXT_INFORMATION_CLASS, buffersize: u32, psizerequired: *mut u32, buffer: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzInitializeCompoundContext(usercontext: AUTHZ_CLIENT_CONTEXT_HANDLE, devicecontext: AUTHZ_CLIENT_CONTEXT_HANDLE, phcompoundcontext: *mut isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzInitializeContextFromAuthzContext(flags: u32, hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, pexpirationtime: *const i64, identifier: super::super::Foundation::LUID, dynamicgroupargs: *const ::core::ffi::c_void, phnewauthzclientcontext: *mut isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzInitializeContextFromSid(flags: u32, usersid: super::super::Foundation::PSID, hauthzresourcemanager: AUTHZ_RESOURCE_MANAGER_HANDLE, pexpirationtime: *const i64, identifier: super::super::Foundation::LUID, dynamicgroupargs: *const ::core::ffi::c_void, phauthzclientcontext: *mut isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzInitializeContextFromToken(flags: u32, tokenhandle: super::super::Foundation::HANDLE, hauthzresourcemanager: AUTHZ_RESOURCE_MANAGER_HANDLE, pexpirationtime: *const i64, identifier: super::super::Foundation::LUID, dynamicgroupargs: *const ::core::ffi::c_void, phauthzclientcontext: *mut isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzInitializeObjectAccessAuditEvent(flags: AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS, hauditeventtype: AUTHZ_AUDIT_EVENT_TYPE_HANDLE, szoperationtype: ::windows_sys::core::PCWSTR, szobjecttype: ::windows_sys::core::PCWSTR, szobjectname: ::windows_sys::core::PCWSTR, szadditionalinfo: ::windows_sys::core::PCWSTR, phauditevent: *mut isize, dwadditionalparametercount: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzInitializeObjectAccessAuditEvent2(flags: u32, hauditeventtype: AUTHZ_AUDIT_EVENT_TYPE_HANDLE, szoperationtype: ::windows_sys::core::PCWSTR, szobjecttype: ::windows_sys::core::PCWSTR, szobjectname: ::windows_sys::core::PCWSTR, szadditionalinfo: ::windows_sys::core::PCWSTR, szadditionalinfo2: ::windows_sys::core::PCWSTR, phauditevent: *mut isize, dwadditionalparametercount: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzInitializeRemoteResourceManager(prpcinitinfo: *const AUTHZ_RPC_INIT_INFO_CLIENT, phauthzresourcemanager: *mut AUTHZ_RESOURCE_MANAGER_HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzInitializeResourceManager(flags: u32, pfndynamicaccesscheck: PFN_AUTHZ_DYNAMIC_ACCESS_CHECK, pfncomputedynamicgroups: PFN_AUTHZ_COMPUTE_DYNAMIC_GROUPS, pfnfreedynamicgroups: PFN_AUTHZ_FREE_DYNAMIC_GROUPS, szresourcemanagername: ::windows_sys::core::PCWSTR, phauthzresourcemanager: *mut AUTHZ_RESOURCE_MANAGER_HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzInitializeResourceManagerEx(flags: AUTHZ_RESOURCE_MANAGER_FLAGS, pauthzinitinfo: *const AUTHZ_INIT_INFO, phauthzresourcemanager: *mut AUTHZ_RESOURCE_MANAGER_HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzInstallSecurityEventSource(dwflags: u32, pregistration: *const AUTHZ_SOURCE_SCHEMA_REGISTRATION) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzModifyClaims(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, claimclass: AUTHZ_CONTEXT_INFORMATION_CLASS, pclaimoperations: *const AUTHZ_SECURITY_ATTRIBUTE_OPERATION, pclaims: *const AUTHZ_SECURITY_ATTRIBUTES_INFORMATION) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzModifySecurityAttributes(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, poperations: *const AUTHZ_SECURITY_ATTRIBUTE_OPERATION, pattributes: *const AUTHZ_SECURITY_ATTRIBUTES_INFORMATION) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzModifySids(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, sidclass: AUTHZ_CONTEXT_INFORMATION_CLASS, psidoperations: *const AUTHZ_SID_OPERATION, psids: *const super::TOKEN_GROUPS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzOpenObjectAudit(flags: u32, hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, prequest: *const AUTHZ_ACCESS_REQUEST, hauditevent: AUTHZ_AUDIT_EVENT_HANDLE, psecuritydescriptor: super::PSECURITY_DESCRIPTOR, optionalsecuritydescriptorarray: *const super::PSECURITY_DESCRIPTOR, optionalsecuritydescriptorcount: u32, preply: *const AUTHZ_ACCESS_REPLY) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`, `\"Win32_System_Threading\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
    pub fn AuthzRegisterCapChangeNotification(phcapchangesubscription: *mut *mut AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE__, pfncapchangecallback: super::super::System::Threading::LPTHREAD_START_ROUTINE, pcallbackcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzRegisterSecurityEventSource(dwflags: u32, szeventsourcename: ::windows_sys::core::PCWSTR, pheventprovider: *mut isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzReportSecurityEvent(dwflags: u32, heventprovider: AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE, dwauditid: u32, pusersid: super::super::Foundation::PSID, dwcount: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzReportSecurityEventFromParams(dwflags: u32, heventprovider: AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE, dwauditid: u32, pusersid: super::super::Foundation::PSID, pparams: *const AUDIT_PARAMS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzSetAppContainerInformation(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, pappcontainersid: super::super::Foundation::PSID, capabilitycount: u32, pcapabilitysids: *const super::SID_AND_ATTRIBUTES) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzUninstallSecurityEventSource(dwflags: u32, szeventsourcename: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzUnregisterCapChangeNotification(hcapchangesubscription: *const AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE__) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzUnregisterSecurityEventSource(dwflags: u32, pheventprovider: *mut isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
    pub fn BuildExplicitAccessWithNameA(pexplicitaccess: *mut EXPLICIT_ACCESS_A, ptrusteename: ::windows_sys::core::PCSTR, accesspermissions: u32, accessmode: ACCESS_MODE, inheritance: super::ACE_FLAGS);
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
    pub fn BuildExplicitAccessWithNameW(pexplicitaccess: *mut EXPLICIT_ACCESS_W, ptrusteename: ::windows_sys::core::PCWSTR, accesspermissions: u32, accessmode: ACCESS_MODE, inheritance: super::ACE_FLAGS);
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
    pub fn BuildImpersonateExplicitAccessWithNameA(pexplicitaccess: *mut EXPLICIT_ACCESS_A, ptrusteename: ::windows_sys::core::PCSTR, ptrustee: *const TRUSTEE_A, accesspermissions: u32, accessmode: ACCESS_MODE, inheritance: u32);
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
    pub fn BuildImpersonateExplicitAccessWithNameW(pexplicitaccess: *mut EXPLICIT_ACCESS_W, ptrusteename: ::windows_sys::core::PCWSTR, ptrustee: *const TRUSTEE_W, accesspermissions: u32, accessmode: ACCESS_MODE, inheritance: u32);
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
    pub fn BuildImpersonateTrusteeA(ptrustee: *mut TRUSTEE_A, pimpersonatetrustee: *const TRUSTEE_A);
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
    pub fn BuildImpersonateTrusteeW(ptrustee: *mut TRUSTEE_W, pimpersonatetrustee: *const TRUSTEE_W);
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
    pub fn BuildSecurityDescriptorA(powner: *const TRUSTEE_A, pgroup: *const TRUSTEE_A, ccountofaccessentries: u32, plistofaccessentries: *const EXPLICIT_ACCESS_A, ccountofauditentries: u32, plistofauditentries: *const EXPLICIT_ACCESS_A, poldsd: super::PSECURITY_DESCRIPTOR, psizenewsd: *mut u32, pnewsd: *mut super::PSECURITY_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
    pub fn BuildSecurityDescriptorW(powner: *const TRUSTEE_W, pgroup: *const TRUSTEE_W, ccountofaccessentries: u32, plistofaccessentries: *const EXPLICIT_ACCESS_W, ccountofauditentries: u32, plistofauditentries: *const EXPLICIT_ACCESS_W, poldsd: super::PSECURITY_DESCRIPTOR, psizenewsd: *mut u32, pnewsd: *mut super::PSECURITY_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
    pub fn BuildTrusteeWithNameA(ptrustee: *mut TRUSTEE_A, pname: ::windows_sys::core::PCSTR);
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
    pub fn BuildTrusteeWithNameW(ptrustee: *mut TRUSTEE_W, pname: ::windows_sys::core::PCWSTR);
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
    pub fn BuildTrusteeWithObjectsAndNameA(ptrustee: *mut TRUSTEE_A, pobjname: *const OBJECTS_AND_NAME_A, objecttype: SE_OBJECT_TYPE, objecttypename: ::windows_sys::core::PCSTR, inheritedobjecttypename: ::windows_sys::core::PCSTR, name: ::windows_sys::core::PCSTR);
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
    pub fn BuildTrusteeWithObjectsAndNameW(ptrustee: *mut TRUSTEE_W, pobjname: *const OBJECTS_AND_NAME_W, objecttype: SE_OBJECT_TYPE, objecttypename: ::windows_sys::core::PCWSTR, inheritedobjecttypename: ::windows_sys::core::PCWSTR, name: ::windows_sys::core::PCWSTR);
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildTrusteeWithObjectsAndSidA(ptrustee: *mut TRUSTEE_A, pobjsid: *const OBJECTS_AND_SID, pobjectguid: *const ::windows_sys::core::GUID, pinheritedobjectguid: *const ::windows_sys::core::GUID, psid: super::super::Foundation::PSID);
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildTrusteeWithObjectsAndSidW(ptrustee: *mut TRUSTEE_W, pobjsid: *const OBJECTS_AND_SID, pobjectguid: *const ::windows_sys::core::GUID, pinheritedobjectguid: *const ::windows_sys::core::GUID, psid: super::super::Foundation::PSID);
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildTrusteeWithSidA(ptrustee: *mut TRUSTEE_A, psid: super::super::Foundation::PSID);
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildTrusteeWithSidW(ptrustee: *mut TRUSTEE_W, psid: super::super::Foundation::PSID);
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertSecurityDescriptorToStringSecurityDescriptorA(securitydescriptor: super::PSECURITY_DESCRIPTOR, requestedstringsdrevision: u32, securityinformation: u32, stringsecuritydescriptor: *mut ::windows_sys::core::PSTR, stringsecuritydescriptorlen: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertSecurityDescriptorToStringSecurityDescriptorW(securitydescriptor: super::PSECURITY_DESCRIPTOR, requestedstringsdrevision: u32, securityinformation: u32, stringsecuritydescriptor: *mut ::windows_sys::core::PWSTR, stringsecuritydescriptorlen: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertSidToStringSidA(sid: super::super::Foundation::PSID, stringsid: *mut ::windows_sys::core::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertSidToStringSidW(sid: super::super::Foundation::PSID, stringsid: *mut ::windows_sys::core::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertStringSecurityDescriptorToSecurityDescriptorA(stringsecuritydescriptor: ::windows_sys::core::PCSTR, stringsdrevision: u32, securitydescriptor: *mut super::PSECURITY_DESCRIPTOR, securitydescriptorsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertStringSecurityDescriptorToSecurityDescriptorW(stringsecuritydescriptor: ::windows_sys::core::PCWSTR, stringsdrevision: u32, securitydescriptor: *mut super::PSECURITY_DESCRIPTOR, securitydescriptorsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertStringSidToSidA(stringsid: ::windows_sys::core::PCSTR, sid: *mut super::super::Foundation::PSID) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertStringSidToSidW(stringsid: ::windows_sys::core::PCWSTR, sid: *mut super::super::Foundation::PSID) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
    pub fn FreeInheritedFromArray(pinheritarray: *const INHERITED_FROMW, acecnt: u16, pfnarray: *const FN_OBJECT_MGR_FUNCTIONS) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
    pub fn GetAuditedPermissionsFromAclA(pacl: *const super::ACL, ptrustee: *const TRUSTEE_A, psuccessfulauditedrights: *mut u32, pfailedauditrights: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
    pub fn GetAuditedPermissionsFromAclW(pacl: *const super::ACL, ptrustee: *const TRUSTEE_W, psuccessfulauditedrights: *mut u32, pfailedauditrights: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
    pub fn GetEffectiveRightsFromAclA(pacl: *const super::ACL, ptrustee: *const TRUSTEE_A, paccessrights: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
    pub fn GetEffectiveRightsFromAclW(pacl: *const super::ACL, ptrustee: *const TRUSTEE_W, paccessrights: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
    pub fn GetExplicitEntriesFromAclA(pacl: *const super::ACL, pccountofexplicitentries: *mut u32, plistofexplicitentries: *mut *mut EXPLICIT_ACCESS_A) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
    pub fn GetExplicitEntriesFromAclW(pacl: *const super::ACL, pccountofexplicitentries: *mut u32, plistofexplicitentries: *mut *mut EXPLICIT_ACCESS_W) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetInheritanceSourceA(pobjectname: ::windows_sys::core::PCSTR, objecttype: SE_OBJECT_TYPE, securityinfo: u32, container: super::super::Foundation::BOOL, pobjectclassguids: *const *const ::windows_sys::core::GUID, guidcount: u32, pacl: *const super::ACL, pfnarray: *const FN_OBJECT_MGR_FUNCTIONS, pgenericmapping: *const super::GENERIC_MAPPING, pinheritarray: *mut INHERITED_FROMA) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetInheritanceSourceW(pobjectname: ::windows_sys::core::PCWSTR, objecttype: SE_OBJECT_TYPE, securityinfo: u32, container: super::super::Foundation::BOOL, pobjectclassguids: *const *const ::windows_sys::core::GUID, guidcount: u32, pacl: *const super::ACL, pfnarray: *const FN_OBJECT_MGR_FUNCTIONS, pgenericmapping: *const super::GENERIC_MAPPING, pinheritarray: *mut INHERITED_FROMW) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
    pub fn GetMultipleTrusteeA(ptrustee: *const TRUSTEE_A) -> *mut TRUSTEE_A;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
    pub fn GetMultipleTrusteeOperationA(ptrustee: *const TRUSTEE_A) -> MULTIPLE_TRUSTEE_OPERATION;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
    pub fn GetMultipleTrusteeOperationW(ptrustee: *const TRUSTEE_W) -> MULTIPLE_TRUSTEE_OPERATION;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
    pub fn GetMultipleTrusteeW(ptrustee: *const TRUSTEE_W) -> *mut TRUSTEE_W;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNamedSecurityInfoA(pobjectname: ::windows_sys::core::PCSTR, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, ppsidowner: *mut super::super::Foundation::PSID, ppsidgroup: *mut super::super::Foundation::PSID, ppdacl: *mut *mut super::ACL, ppsacl: *mut *mut super::ACL, ppsecuritydescriptor: *mut super::PSECURITY_DESCRIPTOR) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNamedSecurityInfoW(pobjectname: ::windows_sys::core::PCWSTR, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, ppsidowner: *mut super::super::Foundation::PSID, ppsidgroup: *mut super::super::Foundation::PSID, ppdacl: *mut *mut super::ACL, ppsacl: *mut *mut super::ACL, ppsecuritydescriptor: *mut super::PSECURITY_DESCRIPTOR) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSecurityInfo(handle: super::super::Foundation::HANDLE, objecttype: SE_OBJECT_TYPE, securityinfo: u32, ppsidowner: *mut super::super::Foundation::PSID, ppsidgroup: *mut super::super::Foundation::PSID, ppdacl: *mut *mut super::ACL, ppsacl: *mut *mut super::ACL, ppsecuritydescriptor: *mut super::PSECURITY_DESCRIPTOR) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
    pub fn GetTrusteeFormA(ptrustee: *const TRUSTEE_A) -> TRUSTEE_FORM;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
    pub fn GetTrusteeFormW(ptrustee: *const TRUSTEE_W) -> TRUSTEE_FORM;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
    pub fn GetTrusteeNameA(ptrustee: *const TRUSTEE_A) -> ::windows_sys::core::PSTR;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
    pub fn GetTrusteeNameW(ptrustee: *const TRUSTEE_W) -> ::windows_sys::core::PWSTR;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
    pub fn GetTrusteeTypeA(ptrustee: *const TRUSTEE_A) -> TRUSTEE_TYPE;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
    pub fn GetTrusteeTypeW(ptrustee: *const TRUSTEE_W) -> TRUSTEE_TYPE;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
    pub fn LookupSecurityDescriptorPartsA(ppowner: *mut *mut TRUSTEE_A, ppgroup: *mut *mut TRUSTEE_A, pccountofaccessentries: *mut u32, pplistofaccessentries: *mut *mut EXPLICIT_ACCESS_A, pccountofauditentries: *mut u32, pplistofauditentries: *mut *mut EXPLICIT_ACCESS_A, psd: super::PSECURITY_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
    pub fn LookupSecurityDescriptorPartsW(ppowner: *mut *mut TRUSTEE_W, ppgroup: *mut *mut TRUSTEE_W, pccountofaccessentries: *mut u32, pplistofaccessentries: *mut *mut EXPLICIT_ACCESS_W, pccountofauditentries: *mut u32, pplistofauditentries: *mut *mut EXPLICIT_ACCESS_W, psd: super::PSECURITY_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
    pub fn SetEntriesInAclA(ccountofexplicitentries: u32, plistofexplicitentries: *const EXPLICIT_ACCESS_A, oldacl: *const super::ACL, newacl: *mut *mut super::ACL) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
    pub fn SetEntriesInAclW(ccountofexplicitentries: u32, plistofexplicitentries: *const EXPLICIT_ACCESS_W, oldacl: *const super::ACL, newacl: *mut *mut super::ACL) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetNamedSecurityInfoA(pobjectname: ::windows_sys::core::PCSTR, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, psidowner: super::super::Foundation::PSID, psidgroup: super::super::Foundation::PSID, pdacl: *const super::ACL, psacl: *const super::ACL) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetNamedSecurityInfoW(pobjectname: ::windows_sys::core::PCWSTR, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, psidowner: super::super::Foundation::PSID, psidgroup: super::super::Foundation::PSID, pdacl: *const super::ACL, psacl: *const super::ACL) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSecurityInfo(handle: super::super::Foundation::HANDLE, objecttype: SE_OBJECT_TYPE, securityinfo: u32, psidowner: super::super::Foundation::PSID, psidgroup: super::super::Foundation::PSID, pdacl: *const super::ACL, psacl: *const super::ACL) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TreeResetNamedSecurityInfoA(pobjectname: ::windows_sys::core::PCSTR, objecttype: SE_OBJECT_TYPE, securityinfo: u32, powner: super::super::Foundation::PSID, pgroup: super::super::Foundation::PSID, pdacl: *const super::ACL, psacl: *const super::ACL, keepexplicit: super::super::Foundation::BOOL, fnprogress: FN_PROGRESS, progressinvokesetting: PROG_INVOKE_SETTING, args: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TreeResetNamedSecurityInfoW(pobjectname: ::windows_sys::core::PCWSTR, objecttype: SE_OBJECT_TYPE, securityinfo: u32, powner: super::super::Foundation::PSID, pgroup: super::super::Foundation::PSID, pdacl: *const super::ACL, psacl: *const super::ACL, keepexplicit: super::super::Foundation::BOOL, fnprogress: FN_PROGRESS, progressinvokesetting: PROG_INVOKE_SETTING, args: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TreeSetNamedSecurityInfoA(pobjectname: ::windows_sys::core::PCSTR, objecttype: SE_OBJECT_TYPE, securityinfo: u32, powner: super::super::Foundation::PSID, pgroup: super::super::Foundation::PSID, pdacl: *const super::ACL, psacl: *const super::ACL, dwaction: TREE_SEC_INFO, fnprogress: FN_PROGRESS, progressinvokesetting: PROG_INVOKE_SETTING, args: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TreeSetNamedSecurityInfoW(pobjectname: ::windows_sys::core::PCWSTR, objecttype: SE_OBJECT_TYPE, securityinfo: u32, powner: super::super::Foundation::PSID, pgroup: super::super::Foundation::PSID, pdacl: *const super::ACL, psacl: *const super::ACL, dwaction: TREE_SEC_INFO, fnprogress: FN_PROGRESS, progressinvokesetting: PROG_INVOKE_SETTING, args: *const ::core::ffi::c_void) -> u32;
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACCCTRL_DEFAULT_PROVIDER: &str = "Windows NT Access Provider";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACCCTRL_DEFAULT_PROVIDERA: &str = "Windows NT Access Provider";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACCCTRL_DEFAULT_PROVIDERW: &str = "Windows NT Access Provider";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub type ACCESS_MODE = i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const NOT_USED_ACCESS: ACCESS_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const GRANT_ACCESS: ACCESS_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SET_ACCESS: ACCESS_MODE = 2i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const DENY_ACCESS: ACCESS_MODE = 3i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const REVOKE_ACCESS: ACCESS_MODE = 4i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SET_AUDIT_SUCCESS: ACCESS_MODE = 5i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SET_AUDIT_FAILURE: ACCESS_MODE = 6i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct ACTRL_ACCESSA {
    pub cEntries: u32,
    pub pPropertyAccessList: *mut ACTRL_PROPERTY_ENTRYA,
}
impl ::core::marker::Copy for ACTRL_ACCESSA {}
impl ::core::clone::Clone for ACTRL_ACCESSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct ACTRL_ACCESSW {
    pub cEntries: u32,
    pub pPropertyAccessList: *mut ACTRL_PROPERTY_ENTRYW,
}
impl ::core::marker::Copy for ACTRL_ACCESSW {}
impl ::core::clone::Clone for ACTRL_ACCESSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct ACTRL_ACCESS_ENTRYA {
    pub Trustee: TRUSTEE_A,
    pub fAccessFlags: ACTRL_ACCESS_ENTRY_ACCESS_FLAGS,
    pub Access: u32,
    pub ProvSpecificAccess: u32,
    pub Inheritance: super::ACE_FLAGS,
    pub lpInheritProperty: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for ACTRL_ACCESS_ENTRYA {}
impl ::core::clone::Clone for ACTRL_ACCESS_ENTRYA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct ACTRL_ACCESS_ENTRYW {
    pub Trustee: TRUSTEE_W,
    pub fAccessFlags: ACTRL_ACCESS_ENTRY_ACCESS_FLAGS,
    pub Access: u32,
    pub ProvSpecificAccess: u32,
    pub Inheritance: super::ACE_FLAGS,
    pub lpInheritProperty: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for ACTRL_ACCESS_ENTRYW {}
impl ::core::clone::Clone for ACTRL_ACCESS_ENTRYW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub type ACTRL_ACCESS_ENTRY_ACCESS_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_ACCESS_ALLOWED: ACTRL_ACCESS_ENTRY_ACCESS_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_ACCESS_DENIED: ACTRL_ACCESS_ENTRY_ACCESS_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_AUDIT_SUCCESS: ACTRL_ACCESS_ENTRY_ACCESS_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_AUDIT_FAILURE: ACTRL_ACCESS_ENTRY_ACCESS_FLAGS = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct ACTRL_ACCESS_ENTRY_LISTA {
    pub cEntries: u32,
    pub pAccessList: *mut ACTRL_ACCESS_ENTRYA,
}
impl ::core::marker::Copy for ACTRL_ACCESS_ENTRY_LISTA {}
impl ::core::clone::Clone for ACTRL_ACCESS_ENTRY_LISTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct ACTRL_ACCESS_ENTRY_LISTW {
    pub cEntries: u32,
    pub pAccessList: *mut ACTRL_ACCESS_ENTRYW,
}
impl ::core::marker::Copy for ACTRL_ACCESS_ENTRY_LISTW {}
impl ::core::clone::Clone for ACTRL_ACCESS_ENTRY_LISTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct ACTRL_ACCESS_INFOA {
    pub fAccessPermission: u32,
    pub lpAccessPermissionName: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for ACTRL_ACCESS_INFOA {}
impl ::core::clone::Clone for ACTRL_ACCESS_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct ACTRL_ACCESS_INFOW {
    pub fAccessPermission: u32,
    pub lpAccessPermissionName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for ACTRL_ACCESS_INFOW {}
impl ::core::clone::Clone for ACTRL_ACCESS_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_ACCESS_NO_OPTIONS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_ACCESS_PROTECTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_ACCESS_SUPPORTS_OBJECT_ENTRIES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_CHANGE_ACCESS: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_CHANGE_OWNER: u32 = 1073741824u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct ACTRL_CONTROL_INFOA {
    pub lpControlId: ::windows_sys::core::PSTR,
    pub lpControlName: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for ACTRL_CONTROL_INFOA {}
impl ::core::clone::Clone for ACTRL_CONTROL_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct ACTRL_CONTROL_INFOW {
    pub lpControlId: ::windows_sys::core::PWSTR,
    pub lpControlName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for ACTRL_CONTROL_INFOW {}
impl ::core::clone::Clone for ACTRL_CONTROL_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_DELETE: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_DIR_CREATE_CHILD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_DIR_CREATE_OBJECT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_DIR_DELETE_CHILD: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_DIR_LIST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_DIR_TRAVERSE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_FILE_APPEND: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_FILE_CREATE_PIPE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_FILE_EXECUTE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_FILE_READ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_FILE_READ_ATTRIB: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_FILE_READ_PROP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_FILE_WRITE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_FILE_WRITE_ATTRIB: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_FILE_WRITE_PROP: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_KERNEL_ALERT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_KERNEL_CONTROL: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_KERNEL_DIMPERSONATE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_KERNEL_DUP_HANDLE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_KERNEL_GET_CONTEXT: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_KERNEL_GET_INFO: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_KERNEL_IMPERSONATE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_KERNEL_PROCESS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_KERNEL_SET_CONTEXT: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_KERNEL_SET_INFO: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_KERNEL_TERMINATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_KERNEL_THREAD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_KERNEL_TOKEN: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_KERNEL_VM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_KERNEL_VM_READ: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_KERNEL_VM_WRITE: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ACTRL_OVERLAPPED {
    pub Anonymous: ACTRL_OVERLAPPED_0,
    pub Reserved2: u32,
    pub hEvent: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACTRL_OVERLAPPED {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACTRL_OVERLAPPED {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union ACTRL_OVERLAPPED_0 {
    pub Provider: *mut ::core::ffi::c_void,
    pub Reserved1: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACTRL_OVERLAPPED_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACTRL_OVERLAPPED_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_10: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_11: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_12: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_13: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_14: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_15: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_16: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_17: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_18: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_19: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_20: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_3: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_4: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_5: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_6: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_7: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_8: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_9: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PRINT_JADMIN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PRINT_PADMIN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PRINT_PUSE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PRINT_SADMIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PRINT_SLIST: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct ACTRL_PROPERTY_ENTRYA {
    pub lpProperty: ::windows_sys::core::PSTR,
    pub pAccessEntryList: *mut ACTRL_ACCESS_ENTRY_LISTA,
    pub fListFlags: u32,
}
impl ::core::marker::Copy for ACTRL_PROPERTY_ENTRYA {}
impl ::core::clone::Clone for ACTRL_PROPERTY_ENTRYA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct ACTRL_PROPERTY_ENTRYW {
    pub lpProperty: ::windows_sys::core::PWSTR,
    pub pAccessEntryList: *mut ACTRL_ACCESS_ENTRY_LISTW,
    pub fListFlags: u32,
}
impl ::core::marker::Copy for ACTRL_PROPERTY_ENTRYW {}
impl ::core::clone::Clone for ACTRL_PROPERTY_ENTRYW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_READ_CONTROL: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_REG_CREATE_CHILD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_REG_LINK: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_REG_LIST: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_REG_NOTIFY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_REG_QUERY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_REG_SET: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_RESERVED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_STD_RIGHTS_ALL: u32 = 4160749568u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_SVC_GET_INFO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_SVC_INTERROGATE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_SVC_LIST: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_SVC_PAUSE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_SVC_SET_INFO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_SVC_START: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_SVC_STATUS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_SVC_STOP: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_SVC_UCONTROL: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_SYNCHRONIZE: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_SYSTEM_ACCESS: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_WIN_CLIPBRD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_WIN_CREATE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_WIN_EXIT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_WIN_GLOBAL_ATOMS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_WIN_LIST: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_WIN_LIST_DESK: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_WIN_READ_ATTRIBS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_WIN_SCREEN: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_WIN_WRITE_ATTRIBS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const APF_AuditFailure: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const APF_AuditSuccess: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const APF_ValidFlags: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AP_ParamTypeBits: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AP_ParamTypeMask: i32 = 255i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct AUDIT_IP_ADDRESS {
    pub pIpAddress: [u8; 128],
}
impl ::core::marker::Copy for AUDIT_IP_ADDRESS {}
impl ::core::clone::Clone for AUDIT_IP_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct AUDIT_OBJECT_TYPE {
    pub ObjectType: ::windows_sys::core::GUID,
    pub Flags: u16,
    pub Level: u16,
    pub AccessMask: u32,
}
impl ::core::marker::Copy for AUDIT_OBJECT_TYPE {}
impl ::core::clone::Clone for AUDIT_OBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct AUDIT_OBJECT_TYPES {
    pub Count: u16,
    pub Flags: u16,
    pub pObjectTypes: *mut AUDIT_OBJECT_TYPE,
}
impl ::core::marker::Copy for AUDIT_OBJECT_TYPES {}
impl ::core::clone::Clone for AUDIT_OBJECT_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct AUDIT_PARAM {
    pub Type: AUDIT_PARAM_TYPE,
    pub Length: u32,
    pub Flags: u32,
    pub Anonymous1: AUDIT_PARAM_0,
    pub Anonymous2: AUDIT_PARAM_1,
}
impl ::core::marker::Copy for AUDIT_PARAM {}
impl ::core::clone::Clone for AUDIT_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub union AUDIT_PARAM_0 {
    pub Data0: usize,
    pub String: ::windows_sys::core::PWSTR,
    pub u: usize,
    pub psid: *mut super::SID,
    pub pguid: *mut ::windows_sys::core::GUID,
    pub LogonId_LowPart: u32,
    pub pObjectTypes: *mut AUDIT_OBJECT_TYPES,
    pub pIpAddress: *mut AUDIT_IP_ADDRESS,
}
impl ::core::marker::Copy for AUDIT_PARAM_0 {}
impl ::core::clone::Clone for AUDIT_PARAM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub union AUDIT_PARAM_1 {
    pub Data1: usize,
    pub LogonId_HighPart: i32,
}
impl ::core::marker::Copy for AUDIT_PARAM_1 {}
impl ::core::clone::Clone for AUDIT_PARAM_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct AUDIT_PARAMS {
    pub Length: u32,
    pub Flags: u32,
    pub Count: u16,
    pub Parameters: *mut AUDIT_PARAM,
}
impl ::core::marker::Copy for AUDIT_PARAMS {}
impl ::core::clone::Clone for AUDIT_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub type AUDIT_PARAM_TYPE = i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const APT_None: AUDIT_PARAM_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const APT_String: AUDIT_PARAM_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const APT_Ulong: AUDIT_PARAM_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const APT_Pointer: AUDIT_PARAM_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const APT_Sid: AUDIT_PARAM_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const APT_LogonId: AUDIT_PARAM_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const APT_ObjectTypeList: AUDIT_PARAM_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const APT_Luid: AUDIT_PARAM_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const APT_Guid: AUDIT_PARAM_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const APT_Time: AUDIT_PARAM_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const APT_Int64: AUDIT_PARAM_TYPE = 11i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const APT_IpAddress: AUDIT_PARAM_TYPE = 12i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const APT_LogonIdWithSid: AUDIT_PARAM_TYPE = 13i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUDIT_TYPE_LEGACY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUDIT_TYPE_WMI: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZP_WPD_EVENT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub type AUTHZ_ACCESS_CHECK_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_ACCESS_CHECK_NO_DEEP_COPY_SD: AUTHZ_ACCESS_CHECK_FLAGS = 1u32;
pub type AUTHZ_ACCESS_CHECK_RESULTS_HANDLE = isize;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct AUTHZ_ACCESS_REPLY {
    pub ResultListLength: u32,
    pub GrantedAccessMask: *mut u32,
    pub SaclEvaluationResults: *mut AUTHZ_GENERATE_RESULTS,
    pub Error: *mut u32,
}
impl ::core::marker::Copy for AUTHZ_ACCESS_REPLY {}
impl ::core::clone::Clone for AUTHZ_ACCESS_REPLY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AUTHZ_ACCESS_REQUEST {
    pub DesiredAccess: u32,
    pub PrincipalSelfSid: super::super::Foundation::PSID,
    pub ObjectTypeList: *mut super::OBJECT_TYPE_LIST,
    pub ObjectTypeListLength: u32,
    pub OptionalArguments: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AUTHZ_ACCESS_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AUTHZ_ACCESS_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_ALLOW_MULTIPLE_SOURCE_INSTANCES: u32 = 1u32;
pub type AUTHZ_AUDIT_EVENT_HANDLE = isize;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub type AUTHZ_AUDIT_EVENT_INFORMATION_CLASS = i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzAuditEventInfoFlags: AUTHZ_AUDIT_EVENT_INFORMATION_CLASS = 1i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzAuditEventInfoOperationType: AUTHZ_AUDIT_EVENT_INFORMATION_CLASS = 2i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzAuditEventInfoObjectType: AUTHZ_AUDIT_EVENT_INFORMATION_CLASS = 3i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzAuditEventInfoObjectName: AUTHZ_AUDIT_EVENT_INFORMATION_CLASS = 4i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzAuditEventInfoAdditionalInfo: AUTHZ_AUDIT_EVENT_INFORMATION_CLASS = 5i32;
pub type AUTHZ_AUDIT_EVENT_TYPE_HANDLE = isize;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct AUTHZ_AUDIT_EVENT_TYPE_LEGACY {
    pub CategoryId: u16,
    pub AuditId: u16,
    pub ParameterCount: u16,
}
impl ::core::marker::Copy for AUTHZ_AUDIT_EVENT_TYPE_LEGACY {}
impl ::core::clone::Clone for AUTHZ_AUDIT_EVENT_TYPE_LEGACY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AUTHZ_AUDIT_EVENT_TYPE_OLD {
    pub Version: u32,
    pub dwFlags: u32,
    pub RefCount: i32,
    pub hAudit: usize,
    pub LinkId: super::super::Foundation::LUID,
    pub u: AUTHZ_AUDIT_EVENT_TYPE_UNION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AUTHZ_AUDIT_EVENT_TYPE_OLD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AUTHZ_AUDIT_EVENT_TYPE_OLD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub union AUTHZ_AUDIT_EVENT_TYPE_UNION {
    pub Legacy: AUTHZ_AUDIT_EVENT_TYPE_LEGACY,
}
impl ::core::marker::Copy for AUTHZ_AUDIT_EVENT_TYPE_UNION {}
impl ::core::clone::Clone for AUTHZ_AUDIT_EVENT_TYPE_UNION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_AUDIT_INSTANCE_INFORMATION: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE__ {
    pub unused: i32,
}
impl ::core::marker::Copy for AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE__ {}
impl ::core::clone::Clone for AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE__ {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AUTHZ_CLIENT_CONTEXT_HANDLE = isize;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_COMPUTE_PRIVILEGES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub type AUTHZ_CONTEXT_INFORMATION_CLASS = i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzContextInfoUserSid: AUTHZ_CONTEXT_INFORMATION_CLASS = 1i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzContextInfoGroupsSids: AUTHZ_CONTEXT_INFORMATION_CLASS = 2i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzContextInfoRestrictedSids: AUTHZ_CONTEXT_INFORMATION_CLASS = 3i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzContextInfoPrivileges: AUTHZ_CONTEXT_INFORMATION_CLASS = 4i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzContextInfoExpirationTime: AUTHZ_CONTEXT_INFORMATION_CLASS = 5i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzContextInfoServerContext: AUTHZ_CONTEXT_INFORMATION_CLASS = 6i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzContextInfoIdentifier: AUTHZ_CONTEXT_INFORMATION_CLASS = 7i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzContextInfoSource: AUTHZ_CONTEXT_INFORMATION_CLASS = 8i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzContextInfoAll: AUTHZ_CONTEXT_INFORMATION_CLASS = 9i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzContextInfoAuthenticationId: AUTHZ_CONTEXT_INFORMATION_CLASS = 10i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzContextInfoSecurityAttributes: AUTHZ_CONTEXT_INFORMATION_CLASS = 11i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzContextInfoDeviceSids: AUTHZ_CONTEXT_INFORMATION_CLASS = 12i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzContextInfoUserClaims: AUTHZ_CONTEXT_INFORMATION_CLASS = 13i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzContextInfoDeviceClaims: AUTHZ_CONTEXT_INFORMATION_CLASS = 14i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzContextInfoAppContainerSid: AUTHZ_CONTEXT_INFORMATION_CLASS = 15i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzContextInfoCapabilitySids: AUTHZ_CONTEXT_INFORMATION_CLASS = 16i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_FLAG_ALLOW_MULTIPLE_SOURCE_INSTANCES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub type AUTHZ_GENERATE_RESULTS = u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_GENERATE_SUCCESS_AUDIT: AUTHZ_GENERATE_RESULTS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_GENERATE_FAILURE_AUDIT: AUTHZ_GENERATE_RESULTS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub type AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_NO_SUCCESS_AUDIT: AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_NO_FAILURE_AUDIT: AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_NO_ALLOC_STRINGS: AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AUTHZ_INIT_INFO {
    pub version: u16,
    pub szResourceManagerName: ::windows_sys::core::PCWSTR,
    pub pfnDynamicAccessCheck: PFN_AUTHZ_DYNAMIC_ACCESS_CHECK,
    pub pfnComputeDynamicGroups: PFN_AUTHZ_COMPUTE_DYNAMIC_GROUPS,
    pub pfnFreeDynamicGroups: PFN_AUTHZ_FREE_DYNAMIC_GROUPS,
    pub pfnGetCentralAccessPolicy: PFN_AUTHZ_GET_CENTRAL_ACCESS_POLICY,
    pub pfnFreeCentralAccessPolicy: PFN_AUTHZ_FREE_CENTRAL_ACCESS_POLICY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AUTHZ_INIT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AUTHZ_INIT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_INIT_INFO_VERSION_V1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_MIGRATED_LEGACY_PUBLISHER: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET {
    pub szObjectTypeName: ::windows_sys::core::PWSTR,
    pub dwOffset: u32,
}
impl ::core::marker::Copy for AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET {}
impl ::core::clone::Clone for AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_REQUIRE_S4U_LOGON: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub type AUTHZ_RESOURCE_MANAGER_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_RM_FLAG_NO_AUDIT: AUTHZ_RESOURCE_MANAGER_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_RM_FLAG_INITIALIZE_UNDER_IMPERSONATION: AUTHZ_RESOURCE_MANAGER_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_RM_FLAG_NO_CENTRAL_ACCESS_POLICIES: AUTHZ_RESOURCE_MANAGER_FLAGS = 4u32;
pub type AUTHZ_RESOURCE_MANAGER_HANDLE = isize;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct AUTHZ_RPC_INIT_INFO_CLIENT {
    pub version: u16,
    pub ObjectUuid: ::windows_sys::core::PWSTR,
    pub ProtSeq: ::windows_sys::core::PWSTR,
    pub NetworkAddr: ::windows_sys::core::PWSTR,
    pub Endpoint: ::windows_sys::core::PWSTR,
    pub Options: ::windows_sys::core::PWSTR,
    pub ServerSpn: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for AUTHZ_RPC_INIT_INFO_CLIENT {}
impl ::core::clone::Clone for AUTHZ_RPC_INIT_INFO_CLIENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_RPC_INIT_INFO_CLIENT_VERSION_V1: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct AUTHZ_SECURITY_ATTRIBUTES_INFORMATION {
    pub Version: u16,
    pub Reserved: u16,
    pub AttributeCount: u32,
    pub Attribute: AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_0,
}
impl ::core::marker::Copy for AUTHZ_SECURITY_ATTRIBUTES_INFORMATION {}
impl ::core::clone::Clone for AUTHZ_SECURITY_ATTRIBUTES_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub union AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_0 {
    pub pAttributeV1: *mut AUTHZ_SECURITY_ATTRIBUTE_V1,
}
impl ::core::marker::Copy for AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_0 {}
impl ::core::clone::Clone for AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_VERSION_V1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub type AUTHZ_SECURITY_ATTRIBUTE_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SECURITY_ATTRIBUTE_NON_INHERITABLE: AUTHZ_SECURITY_ATTRIBUTE_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SECURITY_ATTRIBUTE_VALUE_CASE_SENSITIVE: AUTHZ_SECURITY_ATTRIBUTE_FLAGS = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct AUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE {
    pub Version: u64,
    pub pName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for AUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE {}
impl ::core::clone::Clone for AUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    pub pValue: *mut ::core::ffi::c_void,
    pub ValueLength: u32,
}
impl ::core::marker::Copy for AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {}
impl ::core::clone::Clone for AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub type AUTHZ_SECURITY_ATTRIBUTE_OPERATION = i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SECURITY_ATTRIBUTE_OPERATION_NONE: AUTHZ_SECURITY_ATTRIBUTE_OPERATION = 0i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SECURITY_ATTRIBUTE_OPERATION_REPLACE_ALL: AUTHZ_SECURITY_ATTRIBUTE_OPERATION = 1i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SECURITY_ATTRIBUTE_OPERATION_ADD: AUTHZ_SECURITY_ATTRIBUTE_OPERATION = 2i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SECURITY_ATTRIBUTE_OPERATION_DELETE: AUTHZ_SECURITY_ATTRIBUTE_OPERATION = 3i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SECURITY_ATTRIBUTE_OPERATION_REPLACE: AUTHZ_SECURITY_ATTRIBUTE_OPERATION = 4i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_BOOLEAN: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_FQBN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_INT64: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_INVALID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_OCTET_STRING: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_SID: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_STRING: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_UINT64: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct AUTHZ_SECURITY_ATTRIBUTE_V1 {
    pub pName: ::windows_sys::core::PWSTR,
    pub ValueType: u16,
    pub Reserved: u16,
    pub Flags: AUTHZ_SECURITY_ATTRIBUTE_FLAGS,
    pub ValueCount: u32,
    pub Values: AUTHZ_SECURITY_ATTRIBUTE_V1_0,
}
impl ::core::marker::Copy for AUTHZ_SECURITY_ATTRIBUTE_V1 {}
impl ::core::clone::Clone for AUTHZ_SECURITY_ATTRIBUTE_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub union AUTHZ_SECURITY_ATTRIBUTE_V1_0 {
    pub pInt64: *mut i64,
    pub pUint64: *mut u64,
    pub ppString: *mut ::windows_sys::core::PWSTR,
    pub pFqbn: *mut AUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE,
    pub pOctetString: *mut AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE,
}
impl ::core::marker::Copy for AUTHZ_SECURITY_ATTRIBUTE_V1_0 {}
impl ::core::clone::Clone for AUTHZ_SECURITY_ATTRIBUTE_V1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE = isize;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub type AUTHZ_SID_OPERATION = i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SID_OPERATION_NONE: AUTHZ_SID_OPERATION = 0i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SID_OPERATION_REPLACE_ALL: AUTHZ_SID_OPERATION = 1i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SID_OPERATION_ADD: AUTHZ_SID_OPERATION = 2i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SID_OPERATION_DELETE: AUTHZ_SID_OPERATION = 3i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SID_OPERATION_REPLACE: AUTHZ_SID_OPERATION = 4i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SKIP_TOKEN_GROUPS: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct AUTHZ_SOURCE_SCHEMA_REGISTRATION {
    pub dwFlags: u32,
    pub szEventSourceName: ::windows_sys::core::PWSTR,
    pub szEventMessageFile: ::windows_sys::core::PWSTR,
    pub szEventSourceXmlSchemaFile: ::windows_sys::core::PWSTR,
    pub szEventAccessStringsFile: ::windows_sys::core::PWSTR,
    pub szExecutableImagePath: ::windows_sys::core::PWSTR,
    pub Anonymous: AUTHZ_SOURCE_SCHEMA_REGISTRATION_0,
    pub dwObjectTypeNameCount: u32,
    pub ObjectTypeNames: [AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET; 1],
}
impl ::core::marker::Copy for AUTHZ_SOURCE_SCHEMA_REGISTRATION {}
impl ::core::clone::Clone for AUTHZ_SOURCE_SCHEMA_REGISTRATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub union AUTHZ_SOURCE_SCHEMA_REGISTRATION_0 {
    pub pReserved: *mut ::core::ffi::c_void,
    pub pProviderGuid: *mut ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for AUTHZ_SOURCE_SCHEMA_REGISTRATION_0 {}
impl ::core::clone::Clone for AUTHZ_SOURCE_SCHEMA_REGISTRATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_WPD_CATEGORY_FLAG: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub type AZ_PROP_CONSTANTS = i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_NAME: AZ_PROP_CONSTANTS = 1i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_DESCRIPTION: AZ_PROP_CONSTANTS = 2i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_WRITABLE: AZ_PROP_CONSTANTS = 3i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_APPLICATION_DATA: AZ_PROP_CONSTANTS = 4i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_CHILD_CREATE: AZ_PROP_CONSTANTS = 5i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_MAX_APPLICATION_NAME_LENGTH: AZ_PROP_CONSTANTS = 512i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_MAX_OPERATION_NAME_LENGTH: AZ_PROP_CONSTANTS = 64i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_MAX_TASK_NAME_LENGTH: AZ_PROP_CONSTANTS = 64i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_MAX_SCOPE_NAME_LENGTH: AZ_PROP_CONSTANTS = 65536i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_MAX_GROUP_NAME_LENGTH: AZ_PROP_CONSTANTS = 64i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_MAX_ROLE_NAME_LENGTH: AZ_PROP_CONSTANTS = 64i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_MAX_NAME_LENGTH: AZ_PROP_CONSTANTS = 65536i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_MAX_DESCRIPTION_LENGTH: AZ_PROP_CONSTANTS = 1024i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_MAX_APPLICATION_DATA_LENGTH: AZ_PROP_CONSTANTS = 4096i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_SUBMIT_FLAG_ABORT: AZ_PROP_CONSTANTS = 1i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_SUBMIT_FLAG_FLUSH: AZ_PROP_CONSTANTS = 2i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_MAX_POLICY_URL_LENGTH: AZ_PROP_CONSTANTS = 65536i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_AZSTORE_FLAG_CREATE: AZ_PROP_CONSTANTS = 1i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_AZSTORE_FLAG_MANAGE_STORE_ONLY: AZ_PROP_CONSTANTS = 2i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_AZSTORE_FLAG_BATCH_UPDATE: AZ_PROP_CONSTANTS = 4i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_AZSTORE_FLAG_AUDIT_IS_CRITICAL: AZ_PROP_CONSTANTS = 8i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_AZSTORE_FORCE_APPLICATION_CLOSE: AZ_PROP_CONSTANTS = 16i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_AZSTORE_NT6_FUNCTION_LEVEL: AZ_PROP_CONSTANTS = 32i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_AZSTORE_FLAG_MANAGE_ONLY_PASSIVE_SUBMIT: AZ_PROP_CONSTANTS = 32768i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_AZSTORE_DOMAIN_TIMEOUT: AZ_PROP_CONSTANTS = 100i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_AZSTORE_DEFAULT_DOMAIN_TIMEOUT: AZ_PROP_CONSTANTS = 15000i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_AZSTORE_SCRIPT_ENGINE_TIMEOUT: AZ_PROP_CONSTANTS = 101i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_AZSTORE_MIN_DOMAIN_TIMEOUT: AZ_PROP_CONSTANTS = 500i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_AZSTORE_MIN_SCRIPT_ENGINE_TIMEOUT: AZ_PROP_CONSTANTS = 5000i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_AZSTORE_DEFAULT_SCRIPT_ENGINE_TIMEOUT: AZ_PROP_CONSTANTS = 45000i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_AZSTORE_MAX_SCRIPT_ENGINES: AZ_PROP_CONSTANTS = 102i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_AZSTORE_DEFAULT_MAX_SCRIPT_ENGINES: AZ_PROP_CONSTANTS = 120i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_AZSTORE_MAJOR_VERSION: AZ_PROP_CONSTANTS = 103i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_AZSTORE_MINOR_VERSION: AZ_PROP_CONSTANTS = 104i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_AZSTORE_TARGET_MACHINE: AZ_PROP_CONSTANTS = 105i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_AZTORE_IS_ADAM_INSTANCE: AZ_PROP_CONSTANTS = 106i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_OPERATION_ID: AZ_PROP_CONSTANTS = 200i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_TASK_OPERATIONS: AZ_PROP_CONSTANTS = 300i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_TASK_BIZRULE: AZ_PROP_CONSTANTS = 301i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_TASK_BIZRULE_LANGUAGE: AZ_PROP_CONSTANTS = 302i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_TASK_TASKS: AZ_PROP_CONSTANTS = 303i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_TASK_BIZRULE_IMPORTED_PATH: AZ_PROP_CONSTANTS = 304i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_TASK_IS_ROLE_DEFINITION: AZ_PROP_CONSTANTS = 305i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_MAX_TASK_BIZRULE_LENGTH: AZ_PROP_CONSTANTS = 65536i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_MAX_TASK_BIZRULE_LANGUAGE_LENGTH: AZ_PROP_CONSTANTS = 64i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_MAX_TASK_BIZRULE_IMPORTED_PATH_LENGTH: AZ_PROP_CONSTANTS = 512i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_MAX_BIZRULE_STRING: AZ_PROP_CONSTANTS = 65536i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_GROUP_TYPE: AZ_PROP_CONSTANTS = 400i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_GROUPTYPE_LDAP_QUERY: AZ_PROP_CONSTANTS = 1i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_GROUPTYPE_BASIC: AZ_PROP_CONSTANTS = 2i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_GROUPTYPE_BIZRULE: AZ_PROP_CONSTANTS = 3i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_GROUP_APP_MEMBERS: AZ_PROP_CONSTANTS = 401i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_GROUP_APP_NON_MEMBERS: AZ_PROP_CONSTANTS = 402i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_GROUP_LDAP_QUERY: AZ_PROP_CONSTANTS = 403i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_MAX_GROUP_LDAP_QUERY_LENGTH: AZ_PROP_CONSTANTS = 4096i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_GROUP_MEMBERS: AZ_PROP_CONSTANTS = 404i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_GROUP_NON_MEMBERS: AZ_PROP_CONSTANTS = 405i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_GROUP_MEMBERS_NAME: AZ_PROP_CONSTANTS = 406i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_GROUP_NON_MEMBERS_NAME: AZ_PROP_CONSTANTS = 407i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_GROUP_BIZRULE: AZ_PROP_CONSTANTS = 408i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_GROUP_BIZRULE_LANGUAGE: AZ_PROP_CONSTANTS = 409i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_GROUP_BIZRULE_IMPORTED_PATH: AZ_PROP_CONSTANTS = 410i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_MAX_GROUP_BIZRULE_LENGTH: AZ_PROP_CONSTANTS = 65536i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_MAX_GROUP_BIZRULE_LANGUAGE_LENGTH: AZ_PROP_CONSTANTS = 64i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_MAX_GROUP_BIZRULE_IMPORTED_PATH_LENGTH: AZ_PROP_CONSTANTS = 512i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_ROLE_APP_MEMBERS: AZ_PROP_CONSTANTS = 500i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_ROLE_MEMBERS: AZ_PROP_CONSTANTS = 501i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_ROLE_OPERATIONS: AZ_PROP_CONSTANTS = 502i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_ROLE_TASKS: AZ_PROP_CONSTANTS = 504i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_ROLE_MEMBERS_NAME: AZ_PROP_CONSTANTS = 505i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_SCOPE_BIZRULES_WRITABLE: AZ_PROP_CONSTANTS = 600i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_SCOPE_CAN_BE_DELEGATED: AZ_PROP_CONSTANTS = 601i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_CLIENT_CONTEXT_USER_DN: AZ_PROP_CONSTANTS = 700i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_CLIENT_CONTEXT_USER_SAM_COMPAT: AZ_PROP_CONSTANTS = 701i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_CLIENT_CONTEXT_USER_DISPLAY: AZ_PROP_CONSTANTS = 702i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_CLIENT_CONTEXT_USER_GUID: AZ_PROP_CONSTANTS = 703i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_CLIENT_CONTEXT_USER_CANONICAL: AZ_PROP_CONSTANTS = 704i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_CLIENT_CONTEXT_USER_UPN: AZ_PROP_CONSTANTS = 705i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_CLIENT_CONTEXT_USER_DNS_SAM_COMPAT: AZ_PROP_CONSTANTS = 707i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_CLIENT_CONTEXT_ROLE_FOR_ACCESS_CHECK: AZ_PROP_CONSTANTS = 708i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_CLIENT_CONTEXT_LDAP_QUERY_DN: AZ_PROP_CONSTANTS = 709i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_APPLICATION_AUTHZ_INTERFACE_CLSID: AZ_PROP_CONSTANTS = 800i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_APPLICATION_VERSION: AZ_PROP_CONSTANTS = 801i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_MAX_APPLICATION_VERSION_LENGTH: AZ_PROP_CONSTANTS = 512i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_APPLICATION_NAME: AZ_PROP_CONSTANTS = 802i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_APPLICATION_BIZRULE_ENABLED: AZ_PROP_CONSTANTS = 803i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_APPLY_STORE_SACL: AZ_PROP_CONSTANTS = 900i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_GENERATE_AUDITS: AZ_PROP_CONSTANTS = 901i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_POLICY_ADMINS: AZ_PROP_CONSTANTS = 902i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_POLICY_READERS: AZ_PROP_CONSTANTS = 903i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_DELEGATED_POLICY_USERS: AZ_PROP_CONSTANTS = 904i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_POLICY_ADMINS_NAME: AZ_PROP_CONSTANTS = 905i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_POLICY_READERS_NAME: AZ_PROP_CONSTANTS = 906i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_DELEGATED_POLICY_USERS_NAME: AZ_PROP_CONSTANTS = 907i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_CLIENT_CONTEXT_SKIP_GROUP: AZ_PROP_CONSTANTS = 1i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_CLIENT_CONTEXT_SKIP_LDAP_QUERY: AZ_PROP_CONSTANTS = 1i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_CLIENT_CONTEXT_GET_GROUP_RECURSIVE: AZ_PROP_CONSTANTS = 2i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_CLIENT_CONTEXT_GET_GROUPS_STORE_LEVEL_ONLY: AZ_PROP_CONSTANTS = 2i32;
pub const AzAuthorizationStore: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2998730585, data2: 42839, data3: 19211, data4: [161, 188, 234, 105, 152, 29, 166, 158] };
pub const AzBizRuleContext: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1546504559, data2: 36177, data3: 17227, data4: [179, 60, 55, 155, 204, 174, 119, 195] };
pub const AzPrincipalLocator: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1211824989, data2: 28895, data3: 19990, data4: [171, 220, 161, 222, 77, 1, 90, 62] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct EXPLICIT_ACCESS_A {
    pub grfAccessPermissions: u32,
    pub grfAccessMode: ACCESS_MODE,
    pub grfInheritance: super::ACE_FLAGS,
    pub Trustee: TRUSTEE_A,
}
impl ::core::marker::Copy for EXPLICIT_ACCESS_A {}
impl ::core::clone::Clone for EXPLICIT_ACCESS_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct EXPLICIT_ACCESS_W {
    pub grfAccessPermissions: u32,
    pub grfAccessMode: ACCESS_MODE,
    pub grfInheritance: super::ACE_FLAGS,
    pub Trustee: TRUSTEE_W,
}
impl ::core::marker::Copy for EXPLICIT_ACCESS_W {}
impl ::core::clone::Clone for EXPLICIT_ACCESS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct FN_OBJECT_MGR_FUNCTIONS {
    pub Placeholder: u32,
}
impl ::core::marker::Copy for FN_OBJECT_MGR_FUNCTIONS {}
impl ::core::clone::Clone for FN_OBJECT_MGR_FUNCTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type FN_PROGRESS = ::core::option::Option<unsafe extern "system" fn(pobjectname: ::windows_sys::core::PCWSTR, status: u32, pinvokesetting: *mut PROG_INVOKE_SETTING, args: *const ::core::ffi::c_void, securityset: super::super::Foundation::BOOL)>;
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzApplication {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ApplicationData: unsafe extern "system" fn(this: *mut *mut Self, pbstrapplicationdata: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ApplicationData: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetApplicationData: unsafe extern "system" fn(this: *mut *mut Self, bstrapplicationdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetApplicationData: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AuthzInterfaceClsid: unsafe extern "system" fn(this: *mut *mut Self, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AuthzInterfaceClsid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAuthzInterfaceClsid: unsafe extern "system" fn(this: *mut *mut Self, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAuthzInterfaceClsid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Version: unsafe extern "system" fn(this: *mut *mut Self, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Version: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetVersion: unsafe extern "system" fn(this: *mut *mut Self, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetVersion: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GenerateAudits: unsafe extern "system" fn(this: *mut *mut Self, pbprop: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GenerateAudits: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetGenerateAudits: unsafe extern "system" fn(this: *mut *mut Self, bprop: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetGenerateAudits: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ApplyStoreSacl: unsafe extern "system" fn(this: *mut *mut Self, pbprop: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ApplyStoreSacl: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetApplyStoreSacl: unsafe extern "system" fn(this: *mut *mut Self, bprop: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetApplyStoreSacl: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Writable: unsafe extern "system" fn(this: *mut *mut Self, pfprop: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Writable: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut *mut Self, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetProperty: unsafe extern "system" fn(this: *mut *mut Self, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PolicyAdministrators: unsafe extern "system" fn(this: *mut *mut Self, pvaradmins: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PolicyAdministrators: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PolicyReaders: unsafe extern "system" fn(this: *mut *mut Self, pvarreaders: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PolicyReaders: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPolicyAdministrator: unsafe extern "system" fn(this: *mut *mut Self, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPolicyAdministrator: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePolicyAdministrator: unsafe extern "system" fn(this: *mut *mut Self, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePolicyAdministrator: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPolicyReader: unsafe extern "system" fn(this: *mut *mut Self, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPolicyReader: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePolicyReader: unsafe extern "system" fn(this: *mut *mut Self, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePolicyReader: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Scopes: unsafe extern "system" fn(this: *mut *mut Self, ppscopecollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Scopes: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OpenScope: unsafe extern "system" fn(this: *mut *mut Self, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppscope: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OpenScope: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateScope: unsafe extern "system" fn(this: *mut *mut Self, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppscope: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateScope: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteScope: unsafe extern "system" fn(this: *mut *mut Self, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteScope: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Operations: unsafe extern "system" fn(this: *mut *mut Self, ppoperationcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Operations: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OpenOperation: unsafe extern "system" fn(this: *mut *mut Self, bstroperationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppoperation: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OpenOperation: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateOperation: unsafe extern "system" fn(this: *mut *mut Self, bstroperationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppoperation: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateOperation: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteOperation: unsafe extern "system" fn(this: *mut *mut Self, bstroperationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteOperation: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Tasks: unsafe extern "system" fn(this: *mut *mut Self, pptaskcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Tasks: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OpenTask: unsafe extern "system" fn(this: *mut *mut Self, bstrtaskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pptask: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OpenTask: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateTask: unsafe extern "system" fn(this: *mut *mut Self, bstrtaskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pptask: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateTask: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteTask: unsafe extern "system" fn(this: *mut *mut Self, bstrtaskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteTask: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ApplicationGroups: unsafe extern "system" fn(this: *mut *mut Self, ppgroupcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ApplicationGroups: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OpenApplicationGroup: unsafe extern "system" fn(this: *mut *mut Self, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OpenApplicationGroup: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateApplicationGroup: unsafe extern "system" fn(this: *mut *mut Self, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateApplicationGroup: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteApplicationGroup: unsafe extern "system" fn(this: *mut *mut Self, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteApplicationGroup: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Roles: unsafe extern "system" fn(this: *mut *mut Self, pprolecollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Roles: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OpenRole: unsafe extern "system" fn(this: *mut *mut Self, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pprole: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OpenRole: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateRole: unsafe extern "system" fn(this: *mut *mut Self, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pprole: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateRole: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteRole: unsafe extern "system" fn(this: *mut *mut Self, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteRole: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub InitializeClientContextFromToken: unsafe extern "system" fn(this: *mut *mut Self, ulltokenhandle: u64, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclientcontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    InitializeClientContextFromToken: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPropertyItem: unsafe extern "system" fn(this: *mut *mut Self, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPropertyItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePropertyItem: unsafe extern "system" fn(this: *mut *mut Self, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePropertyItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Submit: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Submit: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub InitializeClientContextFromName: unsafe extern "system" fn(this: *mut *mut Self, clientname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, domainname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclientcontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    InitializeClientContextFromName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DelegatedPolicyUsers: unsafe extern "system" fn(this: *mut *mut Self, pvardelegatedpolicyusers: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DelegatedPolicyUsers: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddDelegatedPolicyUser: unsafe extern "system" fn(this: *mut *mut Self, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddDelegatedPolicyUser: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteDelegatedPolicyUser: unsafe extern "system" fn(this: *mut *mut Self, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteDelegatedPolicyUser: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub InitializeClientContextFromStringSid: unsafe extern "system" fn(this: *mut *mut Self, sidstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclientcontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    InitializeClientContextFromStringSid: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PolicyAdministratorsName: unsafe extern "system" fn(this: *mut *mut Self, pvaradmins: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PolicyAdministratorsName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PolicyReadersName: unsafe extern "system" fn(this: *mut *mut Self, pvarreaders: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PolicyReadersName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPolicyAdministratorName: unsafe extern "system" fn(this: *mut *mut Self, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPolicyAdministratorName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePolicyAdministratorName: unsafe extern "system" fn(this: *mut *mut Self, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePolicyAdministratorName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPolicyReaderName: unsafe extern "system" fn(this: *mut *mut Self, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPolicyReaderName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePolicyReaderName: unsafe extern "system" fn(this: *mut *mut Self, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePolicyReaderName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DelegatedPolicyUsersName: unsafe extern "system" fn(this: *mut *mut Self, pvardelegatedpolicyusers: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DelegatedPolicyUsersName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddDelegatedPolicyUserName: unsafe extern "system" fn(this: *mut *mut Self, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddDelegatedPolicyUserName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteDelegatedPolicyUserName: unsafe extern "system" fn(this: *mut *mut Self, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteDelegatedPolicyUserName: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAzApplication {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2558248903, data2: 47123, data3: 19751, data4: [190, 222, 107, 165, 174, 134, 126, 149] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzApplication2 {
    pub base__: IAzApplication,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub InitializeClientContextFromToken2: unsafe extern "system" fn(this: *mut *mut Self, ultokenhandlelowpart: u32, ultokenhandlehighpart: u32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclientcontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    InitializeClientContextFromToken2: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub InitializeClientContext2: unsafe extern "system" fn(this: *mut *mut Self, identifyingstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclientcontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    InitializeClientContext2: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAzApplication2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 141191343, data2: 41545, data3: 17276, data4: [177, 141, 212, 216, 109, 106, 150, 96] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzApplication3 {
    pub base__: IAzApplication2,
    #[cfg(feature = "Win32_Foundation")]
    pub ScopeExists: unsafe extern "system" fn(this: *mut *mut Self, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbexist: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ScopeExists: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OpenScope2: unsafe extern "system" fn(this: *mut *mut Self, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppscope2: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OpenScope2: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateScope2: unsafe extern "system" fn(this: *mut *mut Self, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppscope2: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateScope2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteScope2: unsafe extern "system" fn(this: *mut *mut Self, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteScope2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RoleDefinitions: unsafe extern "system" fn(this: *mut *mut Self, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RoleDefinitions: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateRoleDefinition: unsafe extern "system" fn(this: *mut *mut Self, bstrroledefinitionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateRoleDefinition: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OpenRoleDefinition: unsafe extern "system" fn(this: *mut *mut Self, bstrroledefinitionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OpenRoleDefinition: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteRoleDefinition: unsafe extern "system" fn(this: *mut *mut Self, bstrroledefinitionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteRoleDefinition: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RoleAssignments: unsafe extern "system" fn(this: *mut *mut Self, pproleassignments: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RoleAssignments: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateRoleAssignment: unsafe extern "system" fn(this: *mut *mut Self, bstrroleassignmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproleassignment: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateRoleAssignment: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OpenRoleAssignment: unsafe extern "system" fn(this: *mut *mut Self, bstrroleassignmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproleassignment: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OpenRoleAssignment: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteRoleAssignment: unsafe extern "system" fn(this: *mut *mut Self, bstrroleassignmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteRoleAssignment: usize,
    pub BizRulesEnabled: unsafe extern "system" fn(this: *mut *mut Self, pbenabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetBizRulesEnabled: unsafe extern "system" fn(this: *mut *mut Self, benabled: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAzApplication3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 404522078, data2: 29078, data3: 19069, data4: [172, 46, 2, 12, 11, 183, 163, 3] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzApplicationGroup {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, plprop: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut *mut Self, lprop: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LdapQuery: unsafe extern "system" fn(this: *mut *mut Self, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LdapQuery: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLdapQuery: unsafe extern "system" fn(this: *mut *mut Self, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLdapQuery: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AppMembers: unsafe extern "system" fn(this: *mut *mut Self, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AppMembers: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AppNonMembers: unsafe extern "system" fn(this: *mut *mut Self, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AppNonMembers: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Members: unsafe extern "system" fn(this: *mut *mut Self, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Members: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub NonMembers: unsafe extern "system" fn(this: *mut *mut Self, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    NonMembers: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddAppMember: unsafe extern "system" fn(this: *mut *mut Self, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddAppMember: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteAppMember: unsafe extern "system" fn(this: *mut *mut Self, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteAppMember: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddAppNonMember: unsafe extern "system" fn(this: *mut *mut Self, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddAppNonMember: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteAppNonMember: unsafe extern "system" fn(this: *mut *mut Self, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteAppNonMember: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddMember: unsafe extern "system" fn(this: *mut *mut Self, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddMember: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteMember: unsafe extern "system" fn(this: *mut *mut Self, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteMember: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddNonMember: unsafe extern "system" fn(this: *mut *mut Self, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddNonMember: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteNonMember: unsafe extern "system" fn(this: *mut *mut Self, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteNonMember: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Writable: unsafe extern "system" fn(this: *mut *mut Self, pfprop: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Writable: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut *mut Self, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetProperty: unsafe extern "system" fn(this: *mut *mut Self, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPropertyItem: unsafe extern "system" fn(this: *mut *mut Self, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPropertyItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePropertyItem: unsafe extern "system" fn(this: *mut *mut Self, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePropertyItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Submit: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Submit: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddMemberName: unsafe extern "system" fn(this: *mut *mut Self, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddMemberName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteMemberName: unsafe extern "system" fn(this: *mut *mut Self, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteMemberName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddNonMemberName: unsafe extern "system" fn(this: *mut *mut Self, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddNonMemberName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteNonMemberName: unsafe extern "system" fn(this: *mut *mut Self, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteNonMemberName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub MembersName: unsafe extern "system" fn(this: *mut *mut Self, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    MembersName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub NonMembersName: unsafe extern "system" fn(this: *mut *mut Self, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    NonMembersName: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAzApplicationGroup {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4055319757, data2: 22694, data3: 19974, data4: [159, 191, 54, 246, 215, 121, 226, 30] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzApplicationGroup2 {
    pub base__: IAzApplicationGroup,
    #[cfg(feature = "Win32_Foundation")]
    pub BizRule: unsafe extern "system" fn(this: *mut *mut Self, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BizRule: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBizRule: unsafe extern "system" fn(this: *mut *mut Self, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBizRule: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BizRuleLanguage: unsafe extern "system" fn(this: *mut *mut Self, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BizRuleLanguage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBizRuleLanguage: unsafe extern "system" fn(this: *mut *mut Self, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBizRuleLanguage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BizRuleImportedPath: unsafe extern "system" fn(this: *mut *mut Self, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BizRuleImportedPath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBizRuleImportedPath: unsafe extern "system" fn(this: *mut *mut Self, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBizRuleImportedPath: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub RoleAssignments: unsafe extern "system" fn(this: *mut *mut Self, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, brecursive: i16, pproleassignments: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    RoleAssignments: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAzApplicationGroup2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1057362940, data2: 46874, data3: 17998, data4: [161, 29, 91, 136, 26, 86, 206, 250] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzApplicationGroups {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, plcount: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAzApplicationGroups {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1290169045, data2: 40764, data3: 18077, data4: [169, 17, 185, 152, 135, 167, 230, 133] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzApplications {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, plcount: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAzApplications {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2459636137, data2: 38341, data3: 19076, data4: [162, 154, 32, 173, 66, 194, 241, 108] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzAuthorizationStore {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ApplicationData: unsafe extern "system" fn(this: *mut *mut Self, pbstrapplicationdata: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ApplicationData: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetApplicationData: unsafe extern "system" fn(this: *mut *mut Self, bstrapplicationdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetApplicationData: usize,
    pub DomainTimeout: unsafe extern "system" fn(this: *mut *mut Self, plprop: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetDomainTimeout: unsafe extern "system" fn(this: *mut *mut Self, lprop: i32) -> ::windows_sys::core::HRESULT,
    pub ScriptEngineTimeout: unsafe extern "system" fn(this: *mut *mut Self, plprop: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetScriptEngineTimeout: unsafe extern "system" fn(this: *mut *mut Self, lprop: i32) -> ::windows_sys::core::HRESULT,
    pub MaxScriptEngines: unsafe extern "system" fn(this: *mut *mut Self, plprop: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMaxScriptEngines: unsafe extern "system" fn(this: *mut *mut Self, lprop: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GenerateAudits: unsafe extern "system" fn(this: *mut *mut Self, pbprop: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GenerateAudits: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetGenerateAudits: unsafe extern "system" fn(this: *mut *mut Self, bprop: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetGenerateAudits: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Writable: unsafe extern "system" fn(this: *mut *mut Self, pfprop: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Writable: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut *mut Self, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetProperty: unsafe extern "system" fn(this: *mut *mut Self, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPropertyItem: unsafe extern "system" fn(this: *mut *mut Self, lpropid: AZ_PROP_CONSTANTS, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPropertyItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePropertyItem: unsafe extern "system" fn(this: *mut *mut Self, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePropertyItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PolicyAdministrators: unsafe extern "system" fn(this: *mut *mut Self, pvaradmins: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PolicyAdministrators: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PolicyReaders: unsafe extern "system" fn(this: *mut *mut Self, pvarreaders: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PolicyReaders: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPolicyAdministrator: unsafe extern "system" fn(this: *mut *mut Self, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPolicyAdministrator: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePolicyAdministrator: unsafe extern "system" fn(this: *mut *mut Self, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePolicyAdministrator: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPolicyReader: unsafe extern "system" fn(this: *mut *mut Self, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPolicyReader: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePolicyReader: unsafe extern "system" fn(this: *mut *mut Self, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePolicyReader: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, lflags: AZ_PROP_CONSTANTS, bstrpolicyurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Initialize: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub UpdateCache: unsafe extern "system" fn(this: *mut *mut Self, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    UpdateCache: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Delete: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Applications: unsafe extern "system" fn(this: *mut *mut Self, ppappcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Applications: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OpenApplication: unsafe extern "system" fn(this: *mut *mut Self, bstrapplicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppapplication: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OpenApplication: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateApplication: unsafe extern "system" fn(this: *mut *mut Self, bstrapplicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppapplication: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateApplication: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteApplication: unsafe extern "system" fn(this: *mut *mut Self, bstrapplicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteApplication: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ApplicationGroups: unsafe extern "system" fn(this: *mut *mut Self, ppgroupcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ApplicationGroups: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateApplicationGroup: unsafe extern "system" fn(this: *mut *mut Self, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateApplicationGroup: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OpenApplicationGroup: unsafe extern "system" fn(this: *mut *mut Self, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OpenApplicationGroup: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteApplicationGroup: unsafe extern "system" fn(this: *mut *mut Self, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteApplicationGroup: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Submit: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Submit: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DelegatedPolicyUsers: unsafe extern "system" fn(this: *mut *mut Self, pvardelegatedpolicyusers: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DelegatedPolicyUsers: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddDelegatedPolicyUser: unsafe extern "system" fn(this: *mut *mut Self, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddDelegatedPolicyUser: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteDelegatedPolicyUser: unsafe extern "system" fn(this: *mut *mut Self, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteDelegatedPolicyUser: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TargetMachine: unsafe extern "system" fn(this: *mut *mut Self, pbstrtargetmachine: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TargetMachine: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ApplyStoreSacl: unsafe extern "system" fn(this: *mut *mut Self, pbapplystoresacl: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ApplyStoreSacl: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetApplyStoreSacl: unsafe extern "system" fn(this: *mut *mut Self, bapplystoresacl: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetApplyStoreSacl: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PolicyAdministratorsName: unsafe extern "system" fn(this: *mut *mut Self, pvaradmins: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PolicyAdministratorsName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PolicyReadersName: unsafe extern "system" fn(this: *mut *mut Self, pvarreaders: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PolicyReadersName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPolicyAdministratorName: unsafe extern "system" fn(this: *mut *mut Self, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPolicyAdministratorName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePolicyAdministratorName: unsafe extern "system" fn(this: *mut *mut Self, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePolicyAdministratorName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPolicyReaderName: unsafe extern "system" fn(this: *mut *mut Self, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPolicyReaderName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePolicyReaderName: unsafe extern "system" fn(this: *mut *mut Self, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePolicyReaderName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DelegatedPolicyUsersName: unsafe extern "system" fn(this: *mut *mut Self, pvardelegatedpolicyusers: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DelegatedPolicyUsersName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddDelegatedPolicyUserName: unsafe extern "system" fn(this: *mut *mut Self, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddDelegatedPolicyUserName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteDelegatedPolicyUserName: unsafe extern "system" fn(this: *mut *mut Self, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteDelegatedPolicyUserName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CloseApplication: unsafe extern "system" fn(this: *mut *mut Self, bstrapplicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflag: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CloseApplication: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAzAuthorizationStore {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3988626601, data2: 39810, data3: 20330, data4: [158, 139, 152, 48, 30, 69, 15, 20] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzAuthorizationStore2 {
    pub base__: IAzAuthorizationStore,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OpenApplication2: unsafe extern "system" fn(this: *mut *mut Self, bstrapplicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppapplication: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OpenApplication2: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateApplication2: unsafe extern "system" fn(this: *mut *mut Self, bstrapplicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppapplication: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateApplication2: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAzAuthorizationStore2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2971555204, data2: 54647, data3: 17011, data4: [182, 197, 9, 115, 224, 248, 232, 13] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzAuthorizationStore3 {
    pub base__: IAzAuthorizationStore2,
    pub IsUpdateNeeded: unsafe extern "system" fn(this: *mut *mut Self, pbisupdateneeded: *mut i16) -> ::windows_sys::core::HRESULT,
    pub BizruleGroupSupported: unsafe extern "system" fn(this: *mut *mut Self, pbsupported: *mut i16) -> ::windows_sys::core::HRESULT,
    pub UpgradeStoresFunctionalLevel: unsafe extern "system" fn(this: *mut *mut Self, lfunctionallevel: i32) -> ::windows_sys::core::HRESULT,
    pub IsFunctionalLevelUpgradeSupported: unsafe extern "system" fn(this: *mut *mut Self, lfunctionallevel: i32, pbsupported: *mut i16) -> ::windows_sys::core::HRESULT,
    pub GetSchemaVersion: unsafe extern "system" fn(this: *mut *mut Self, plmajorversion: *mut i32, plminorversion: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAzAuthorizationStore3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2881520677, data2: 3206, data3: 20384, data4: [155, 227, 113, 137, 149, 108, 146, 110] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzBizRuleContext {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBusinessRuleResult: unsafe extern "system" fn(this: *mut *mut Self, bresult: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBusinessRuleResult: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBusinessRuleString: unsafe extern "system" fn(this: *mut *mut Self, bstrbusinessrulestring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBusinessRuleString: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BusinessRuleString: unsafe extern "system" fn(this: *mut *mut Self, pbstrbusinessrulestring: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BusinessRuleString: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetParameter: unsafe extern "system" fn(this: *mut *mut Self, bstrparametername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarparametervalue: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetParameter: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAzBizRuleContext {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3784503677, data2: 54687, data3: 17758, data4: [161, 82, 148, 3, 22, 205, 119, 178] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzBizRuleInterfaces {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddInterface: unsafe extern "system" fn(this: *mut *mut Self, bstrinterfacename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linterfaceflag: i32, varinterface: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddInterface: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddInterfaces: unsafe extern "system" fn(this: *mut *mut Self, varinterfacenames: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varinterfaceflags: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varinterfaces: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddInterfaces: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetInterfaceValue: unsafe extern "system" fn(this: *mut *mut Self, bstrinterfacename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linterfaceflag: *mut i32, varinterface: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetInterfaceValue: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, bstrinterfacename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Remove: usize,
    pub RemoveAll: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, plcount: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAzBizRuleInterfaces {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3913361607, data2: 59866, data3: 17612, data4: [176, 189, 83, 3, 111, 58, 171, 61] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzBizRuleParameters {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddParameter: unsafe extern "system" fn(this: *mut *mut Self, bstrparametername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varparametervalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddParameter: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddParameters: unsafe extern "system" fn(this: *mut *mut Self, varparameternames: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varparametervalues: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddParameters: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetParameterValue: unsafe extern "system" fn(this: *mut *mut Self, bstrparametername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarparametervalue: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetParameterValue: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, varparametername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Remove: usize,
    pub RemoveAll: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, plcount: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAzBizRuleParameters {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4229392479, data2: 57949, data3: 19917, data4: [186, 225, 39, 110, 201, 83, 60, 181] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzClientContext {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AccessCheck: unsafe extern "system" fn(this: *mut *mut Self, bstrobjectname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varscopenames: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varoperations: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varparameternames: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varparametervalues: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varinterfacenames: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varinterfaceflags: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varinterfaces: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarresults: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AccessCheck: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBusinessRuleString: unsafe extern "system" fn(this: *mut *mut Self, pbstrbusinessrulestring: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBusinessRuleString: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UserDn: unsafe extern "system" fn(this: *mut *mut Self, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UserDn: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UserSamCompat: unsafe extern "system" fn(this: *mut *mut Self, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UserSamCompat: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UserDisplay: unsafe extern "system" fn(this: *mut *mut Self, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UserDisplay: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UserGuid: unsafe extern "system" fn(this: *mut *mut Self, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UserGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UserCanonical: unsafe extern "system" fn(this: *mut *mut Self, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UserCanonical: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UserUpn: unsafe extern "system" fn(this: *mut *mut Self, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UserUpn: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UserDnsSamCompat: unsafe extern "system" fn(this: *mut *mut Self, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UserDnsSamCompat: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut *mut Self, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetRoles: unsafe extern "system" fn(this: *mut *mut Self, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarrolenames: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetRoles: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RoleForAccessCheck: unsafe extern "system" fn(this: *mut *mut Self, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RoleForAccessCheck: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRoleForAccessCheck: unsafe extern "system" fn(this: *mut *mut Self, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRoleForAccessCheck: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAzClientContext {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4025610251, data2: 18570, data3: 18029, data4: [175, 217, 164, 1, 197, 249, 238, 245] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzClientContext2 {
    pub base__: IAzClientContext,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetAssignedScopesPage: unsafe extern "system" fn(this: *mut *mut Self, loptions: i32, pagesize: i32, pvarcursor: *mut super::super::System::Com::VARIANT, pvarscopenames: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetAssignedScopesPage: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddRoles: unsafe extern "system" fn(this: *mut *mut Self, varroles: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddRoles: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddApplicationGroups: unsafe extern "system" fn(this: *mut *mut Self, varapplicationgroups: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddApplicationGroups: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddStringSids: unsafe extern "system" fn(this: *mut *mut Self, varstringsids: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddStringSids: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLDAPQueryDN: unsafe extern "system" fn(this: *mut *mut Self, bstrldapquerydn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLDAPQueryDN: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LDAPQueryDN: unsafe extern "system" fn(this: *mut *mut Self, pbstrldapquerydn: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LDAPQueryDN: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAzClientContext2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 722244280, data2: 8330, data3: 18570, data4: [143, 129, 228, 237, 178, 33, 17, 205] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzClientContext3 {
    pub base__: IAzClientContext2,
    #[cfg(feature = "Win32_Foundation")]
    pub AccessCheck2: unsafe extern "system" fn(this: *mut *mut Self, bstrobjectname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loperation: i32, plresult: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AccessCheck2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsInRoleAssignment: unsafe extern "system" fn(this: *mut *mut Self, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbisinrole: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsInRoleAssignment: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetOperations: unsafe extern "system" fn(this: *mut *mut Self, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppoperationcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetOperations: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetTasks: unsafe extern "system" fn(this: *mut *mut Self, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptaskcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetTasks: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub BizRuleParameters: unsafe extern "system" fn(this: *mut *mut Self, ppbizruleparam: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BizRuleParameters: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub BizRuleInterfaces: unsafe extern "system" fn(this: *mut *mut Self, ppbizruleinterfaces: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BizRuleInterfaces: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetGroups: unsafe extern "system" fn(this: *mut *mut Self, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, uloptions: AZ_PROP_CONSTANTS, pgrouparray: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetGroups: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Sids: unsafe extern "system" fn(this: *mut *mut Self, pstringsidarray: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Sids: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAzClientContext3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 294211550, data2: 7659, data3: 19275, data4: [137, 7, 109, 28, 218, 31, 93, 79] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzNameResolver {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub NameFromSid: unsafe extern "system" fn(this: *mut *mut Self, bstrsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psidtype: *mut i32, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    NameFromSid: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub NamesFromSids: unsafe extern "system" fn(this: *mut *mut Self, vsids: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvsidtypes: *mut super::super::System::Com::VARIANT, pvnames: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    NamesFromSids: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAzNameResolver {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1347227413, data2: 29666, data3: 17375, data4: [168, 112, 166, 79, 64, 113, 79, 83] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzObjectPicker {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetPrincipals: unsafe extern "system" fn(this: *mut *mut Self, hparentwnd: super::super::Foundation::HWND, bstrtitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvsidtypes: *mut super::super::System::Com::VARIANT, pvnames: *mut super::super::System::Com::VARIANT, pvsids: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetPrincipals: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAzObjectPicker {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1662192200, data2: 27034, data3: 17112, data4: [191, 1, 198, 42, 195, 251, 121, 249] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzOperation {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ApplicationData: unsafe extern "system" fn(this: *mut *mut Self, pbstrapplicationdata: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ApplicationData: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetApplicationData: unsafe extern "system" fn(this: *mut *mut Self, bstrapplicationdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetApplicationData: usize,
    pub OperationID: unsafe extern "system" fn(this: *mut *mut Self, plprop: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetOperationID: unsafe extern "system" fn(this: *mut *mut Self, lprop: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Writable: unsafe extern "system" fn(this: *mut *mut Self, pfprop: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Writable: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut *mut Self, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetProperty: unsafe extern "system" fn(this: *mut *mut Self, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Submit: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Submit: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAzOperation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1582740047, data2: 59905, data3: 19809, data4: [190, 68, 196, 155, 94, 78, 175, 116] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzOperation2 {
    pub base__: IAzOperation,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub RoleAssignments: unsafe extern "system" fn(this: *mut *mut Self, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, brecursive: i16, pproleassignments: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    RoleAssignments: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAzOperation2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 526295071, data2: 17570, data3: 16772, data4: [156, 72, 167, 91, 77, 204, 140, 204] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzOperations {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, plcount: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAzOperations {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2431622151, data2: 38662, data3: 18905, data4: [175, 128, 4, 56, 165, 243, 236, 53] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzPrincipalLocator {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub NameResolver: unsafe extern "system" fn(this: *mut *mut Self, ppnameresolver: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NameResolver: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ObjectPicker: unsafe extern "system" fn(this: *mut *mut Self, ppobjectpicker: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ObjectPicker: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAzPrincipalLocator {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3854782589, data2: 44394, data3: 18834, data4: [156, 127, 116, 171, 72, 11, 68, 204] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzRole {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ApplicationData: unsafe extern "system" fn(this: *mut *mut Self, pbstrapplicationdata: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ApplicationData: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetApplicationData: unsafe extern "system" fn(this: *mut *mut Self, bstrapplicationdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetApplicationData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddAppMember: unsafe extern "system" fn(this: *mut *mut Self, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddAppMember: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteAppMember: unsafe extern "system" fn(this: *mut *mut Self, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteAppMember: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddTask: unsafe extern "system" fn(this: *mut *mut Self, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddTask: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteTask: unsafe extern "system" fn(this: *mut *mut Self, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteTask: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddOperation: unsafe extern "system" fn(this: *mut *mut Self, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddOperation: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteOperation: unsafe extern "system" fn(this: *mut *mut Self, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteOperation: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddMember: unsafe extern "system" fn(this: *mut *mut Self, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddMember: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteMember: unsafe extern "system" fn(this: *mut *mut Self, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteMember: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Writable: unsafe extern "system" fn(this: *mut *mut Self, pfprop: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Writable: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut *mut Self, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetProperty: unsafe extern "system" fn(this: *mut *mut Self, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AppMembers: unsafe extern "system" fn(this: *mut *mut Self, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AppMembers: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Members: unsafe extern "system" fn(this: *mut *mut Self, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Members: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Operations: unsafe extern "system" fn(this: *mut *mut Self, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Operations: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Tasks: unsafe extern "system" fn(this: *mut *mut Self, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Tasks: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPropertyItem: unsafe extern "system" fn(this: *mut *mut Self, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPropertyItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePropertyItem: unsafe extern "system" fn(this: *mut *mut Self, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePropertyItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Submit: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Submit: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddMemberName: unsafe extern "system" fn(this: *mut *mut Self, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddMemberName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteMemberName: unsafe extern "system" fn(this: *mut *mut Self, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteMemberName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub MembersName: unsafe extern "system" fn(this: *mut *mut Self, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    MembersName: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAzRole {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2241727885, data2: 25303, data3: 16856, data4: [160, 52, 192, 205, 93, 67, 253, 250] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzRoleAssignment {
    pub base__: IAzRole,
    #[cfg(feature = "Win32_Foundation")]
    pub AddRoleDefinition: unsafe extern "system" fn(this: *mut *mut Self, bstrroledefinition: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddRoleDefinition: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteRoleDefinition: unsafe extern "system" fn(this: *mut *mut Self, bstrroledefinition: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteRoleDefinition: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RoleDefinitions: unsafe extern "system" fn(this: *mut *mut Self, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RoleDefinitions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Scope: unsafe extern "system" fn(this: *mut *mut Self, ppscope: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Scope: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAzRoleAssignment {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1432649009, data2: 3418, data3: 20387, data4: [180, 172, 43, 95, 154, 213, 171, 118] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzRoleAssignments {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, plcount: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAzRoleAssignments {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2625681664, data2: 64747, data3: 19827, data4: [160, 244, 200, 59, 11, 191, 36, 129] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzRoleDefinition {
    pub base__: IAzTask,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub RoleAssignments: unsafe extern "system" fn(this: *mut *mut Self, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, brecursive: i16, pproleassignments: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    RoleAssignments: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AddRoleDefinition: unsafe extern "system" fn(this: *mut *mut Self, bstrroledefinition: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddRoleDefinition: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteRoleDefinition: unsafe extern "system" fn(this: *mut *mut Self, bstrroledefinition: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteRoleDefinition: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RoleDefinitions: unsafe extern "system" fn(this: *mut *mut Self, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RoleDefinitions: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAzRoleDefinition {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3649031841, data2: 9625, data3: 17649, data4: [159, 195, 88, 233, 251, 224, 148, 102] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzRoleDefinitions {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, plcount: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAzRoleDefinitions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2283742629, data2: 55125, data3: 17744, data4: [149, 122, 213, 3, 163, 179, 64, 1] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzRoles {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, plcount: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAzRoles {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2514546969, data2: 5044, data3: 19886, data4: [182, 95, 47, 125, 96, 216, 34, 228] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzScope {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ApplicationData: unsafe extern "system" fn(this: *mut *mut Self, pbstrapplicationdata: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ApplicationData: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetApplicationData: unsafe extern "system" fn(this: *mut *mut Self, bstrapplicationdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetApplicationData: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Writable: unsafe extern "system" fn(this: *mut *mut Self, pfprop: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Writable: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut *mut Self, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetProperty: unsafe extern "system" fn(this: *mut *mut Self, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPropertyItem: unsafe extern "system" fn(this: *mut *mut Self, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPropertyItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePropertyItem: unsafe extern "system" fn(this: *mut *mut Self, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePropertyItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PolicyAdministrators: unsafe extern "system" fn(this: *mut *mut Self, pvaradmins: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PolicyAdministrators: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PolicyReaders: unsafe extern "system" fn(this: *mut *mut Self, pvarreaders: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PolicyReaders: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPolicyAdministrator: unsafe extern "system" fn(this: *mut *mut Self, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPolicyAdministrator: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePolicyAdministrator: unsafe extern "system" fn(this: *mut *mut Self, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePolicyAdministrator: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPolicyReader: unsafe extern "system" fn(this: *mut *mut Self, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPolicyReader: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePolicyReader: unsafe extern "system" fn(this: *mut *mut Self, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePolicyReader: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ApplicationGroups: unsafe extern "system" fn(this: *mut *mut Self, ppgroupcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ApplicationGroups: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OpenApplicationGroup: unsafe extern "system" fn(this: *mut *mut Self, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OpenApplicationGroup: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateApplicationGroup: unsafe extern "system" fn(this: *mut *mut Self, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateApplicationGroup: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteApplicationGroup: unsafe extern "system" fn(this: *mut *mut Self, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteApplicationGroup: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Roles: unsafe extern "system" fn(this: *mut *mut Self, pprolecollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Roles: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OpenRole: unsafe extern "system" fn(this: *mut *mut Self, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pprole: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OpenRole: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateRole: unsafe extern "system" fn(this: *mut *mut Self, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pprole: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateRole: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteRole: unsafe extern "system" fn(this: *mut *mut Self, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteRole: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Tasks: unsafe extern "system" fn(this: *mut *mut Self, pptaskcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Tasks: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OpenTask: unsafe extern "system" fn(this: *mut *mut Self, bstrtaskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pptask: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OpenTask: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateTask: unsafe extern "system" fn(this: *mut *mut Self, bstrtaskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pptask: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateTask: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteTask: unsafe extern "system" fn(this: *mut *mut Self, bstrtaskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteTask: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Submit: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Submit: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CanBeDelegated: unsafe extern "system" fn(this: *mut *mut Self, pfprop: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CanBeDelegated: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BizrulesWritable: unsafe extern "system" fn(this: *mut *mut Self, pfprop: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BizrulesWritable: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PolicyAdministratorsName: unsafe extern "system" fn(this: *mut *mut Self, pvaradmins: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PolicyAdministratorsName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PolicyReadersName: unsafe extern "system" fn(this: *mut *mut Self, pvarreaders: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PolicyReadersName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPolicyAdministratorName: unsafe extern "system" fn(this: *mut *mut Self, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPolicyAdministratorName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePolicyAdministratorName: unsafe extern "system" fn(this: *mut *mut Self, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePolicyAdministratorName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPolicyReaderName: unsafe extern "system" fn(this: *mut *mut Self, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPolicyReaderName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePolicyReaderName: unsafe extern "system" fn(this: *mut *mut Self, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePolicyReaderName: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAzScope {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 15017095, data2: 57485, data3: 17684, data4: [182, 46, 135, 125, 86, 69, 245, 171] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzScope2 {
    pub base__: IAzScope,
    #[cfg(feature = "Win32_System_Com")]
    pub RoleDefinitions: unsafe extern "system" fn(this: *mut *mut Self, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RoleDefinitions: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateRoleDefinition: unsafe extern "system" fn(this: *mut *mut Self, bstrroledefinitionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateRoleDefinition: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OpenRoleDefinition: unsafe extern "system" fn(this: *mut *mut Self, bstrroledefinitionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OpenRoleDefinition: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteRoleDefinition: unsafe extern "system" fn(this: *mut *mut Self, bstrroledefinitionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteRoleDefinition: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RoleAssignments: unsafe extern "system" fn(this: *mut *mut Self, pproleassignments: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RoleAssignments: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateRoleAssignment: unsafe extern "system" fn(this: *mut *mut Self, bstrroleassignmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproleassignment: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateRoleAssignment: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OpenRoleAssignment: unsafe extern "system" fn(this: *mut *mut Self, bstrroleassignmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproleassignment: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OpenRoleAssignment: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteRoleAssignment: unsafe extern "system" fn(this: *mut *mut Self, bstrroleassignmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteRoleAssignment: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAzScope2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4003457225, data2: 51699, data3: 16610, data4: [170, 18, 209, 216, 89, 151, 39, 253] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzScopes {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, plcount: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAzScopes {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2028030035, data2: 40798, data3: 16493, data4: [155, 145, 107, 219, 166, 151, 53, 16] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzTask {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ApplicationData: unsafe extern "system" fn(this: *mut *mut Self, pbstrapplicationdata: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ApplicationData: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetApplicationData: unsafe extern "system" fn(this: *mut *mut Self, bstrapplicationdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetApplicationData: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BizRule: unsafe extern "system" fn(this: *mut *mut Self, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BizRule: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBizRule: unsafe extern "system" fn(this: *mut *mut Self, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBizRule: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BizRuleLanguage: unsafe extern "system" fn(this: *mut *mut Self, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BizRuleLanguage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBizRuleLanguage: unsafe extern "system" fn(this: *mut *mut Self, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBizRuleLanguage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BizRuleImportedPath: unsafe extern "system" fn(this: *mut *mut Self, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BizRuleImportedPath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBizRuleImportedPath: unsafe extern "system" fn(this: *mut *mut Self, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBizRuleImportedPath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsRoleDefinition: unsafe extern "system" fn(this: *mut *mut Self, pfprop: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsRoleDefinition: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIsRoleDefinition: unsafe extern "system" fn(this: *mut *mut Self, fprop: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIsRoleDefinition: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Operations: unsafe extern "system" fn(this: *mut *mut Self, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Operations: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Tasks: unsafe extern "system" fn(this: *mut *mut Self, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Tasks: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddOperation: unsafe extern "system" fn(this: *mut *mut Self, bstrop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddOperation: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteOperation: unsafe extern "system" fn(this: *mut *mut Self, bstrop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteOperation: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddTask: unsafe extern "system" fn(this: *mut *mut Self, bstrtask: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddTask: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteTask: unsafe extern "system" fn(this: *mut *mut Self, bstrtask: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteTask: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Writable: unsafe extern "system" fn(this: *mut *mut Self, pfprop: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Writable: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut *mut Self, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetProperty: unsafe extern "system" fn(this: *mut *mut Self, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPropertyItem: unsafe extern "system" fn(this: *mut *mut Self, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPropertyItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePropertyItem: unsafe extern "system" fn(this: *mut *mut Self, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePropertyItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Submit: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Submit: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAzTask {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3415532946, data2: 11790, data3: 19052, data4: [163, 54, 184, 154, 109, 193, 227, 136] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzTask2 {
    pub base__: IAzTask,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub RoleAssignments: unsafe extern "system" fn(this: *mut *mut Self, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, brecursive: i16, pproleassignments: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    RoleAssignments: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAzTask2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 61449710, data2: 18632, data3: 18482, data4: [144, 37, 170, 213, 3, 196, 101, 38] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzTasks {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, plcount: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAzTasks {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3006844075, data2: 19589, data3: 17288, data4: [140, 10, 197, 133, 146, 186, 211, 152] };
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const INHERITED_ACCESS_ENTRY: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct INHERITED_FROMA {
    pub GenerationGap: i32,
    pub AncestorName: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for INHERITED_FROMA {}
impl ::core::clone::Clone for INHERITED_FROMA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct INHERITED_FROMW {
    pub GenerationGap: i32,
    pub AncestorName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for INHERITED_FROMW {}
impl ::core::clone::Clone for INHERITED_FROMW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const INHERITED_GRANDPARENT: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const INHERITED_PARENT: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub type MULTIPLE_TRUSTEE_OPERATION = i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const NO_MULTIPLE_TRUSTEE: MULTIPLE_TRUSTEE_OPERATION = 0i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_IS_IMPERSONATE: MULTIPLE_TRUSTEE_OPERATION = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct OBJECTS_AND_NAME_A {
    pub ObjectsPresent: super::SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: SE_OBJECT_TYPE,
    pub ObjectTypeName: ::windows_sys::core::PSTR,
    pub InheritedObjectTypeName: ::windows_sys::core::PSTR,
    pub ptstrName: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for OBJECTS_AND_NAME_A {}
impl ::core::clone::Clone for OBJECTS_AND_NAME_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct OBJECTS_AND_NAME_W {
    pub ObjectsPresent: super::SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: SE_OBJECT_TYPE,
    pub ObjectTypeName: ::windows_sys::core::PWSTR,
    pub InheritedObjectTypeName: ::windows_sys::core::PWSTR,
    pub ptstrName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for OBJECTS_AND_NAME_W {}
impl ::core::clone::Clone for OBJECTS_AND_NAME_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct OBJECTS_AND_SID {
    pub ObjectsPresent: super::SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectTypeGuid: ::windows_sys::core::GUID,
    pub InheritedObjectTypeGuid: ::windows_sys::core::GUID,
    pub pSid: *mut super::SID,
}
impl ::core::marker::Copy for OBJECTS_AND_SID {}
impl ::core::clone::Clone for OBJECTS_AND_SID {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const OLESCRIPT_E_SYNTAX: ::windows_sys::core::HRESULT = -2147352319i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_AUTHZ_COMPUTE_DYNAMIC_GROUPS = ::core::option::Option<unsafe extern "system" fn(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, args: *const ::core::ffi::c_void, psidattrarray: *mut *mut super::SID_AND_ATTRIBUTES, psidcount: *mut u32, prestrictedsidattrarray: *mut *mut super::SID_AND_ATTRIBUTES, prestrictedsidcount: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_AUTHZ_DYNAMIC_ACCESS_CHECK = ::core::option::Option<unsafe extern "system" fn(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, pace: *const super::ACE_HEADER, pargs: *const ::core::ffi::c_void, pbaceapplicable: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub type PFN_AUTHZ_FREE_CENTRAL_ACCESS_POLICY = ::core::option::Option<unsafe extern "system" fn(pcentralaccesspolicy: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_AUTHZ_FREE_DYNAMIC_GROUPS = ::core::option::Option<unsafe extern "system" fn(psidattrarray: *const super::SID_AND_ATTRIBUTES)>;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_AUTHZ_GET_CENTRAL_ACCESS_POLICY = ::core::option::Option<unsafe extern "system" fn(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, capid: super::super::Foundation::PSID, pargs: *const ::core::ffi::c_void, pcentralaccesspolicyapplicable: *mut super::super::Foundation::BOOL, ppcentralaccesspolicy: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub type PROG_INVOKE_SETTING = i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ProgressInvokeNever: PROG_INVOKE_SETTING = 1i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ProgressInvokeEveryObject: PROG_INVOKE_SETTING = 2i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ProgressInvokeOnError: PROG_INVOKE_SETTING = 3i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ProgressCancelOperation: PROG_INVOKE_SETTING = 4i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ProgressRetryOperation: PROG_INVOKE_SETTING = 5i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ProgressInvokePrePostError: PROG_INVOKE_SETTING = 6i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ACCESS_ALLOWED: &str = "A";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ACCESS_CONTROL_ASSISTANCE_OPS: &str = "AA";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ACCESS_DENIED: &str = "D";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ACCESS_FILTER: &str = "FL";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ACCOUNT_OPERATORS: &str = "AO";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ACE_BEGIN: &str = "(";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ACE_COND_ATTRIBUTE_PREFIX: &str = "@";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ACE_COND_BEGIN: &str = "(";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ACE_COND_BLOB_PREFIX: &str = "#";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ACE_COND_DEVICE_ATTRIBUTE_PREFIX: &str = "@DEVICE.";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ACE_COND_END: &str = ")";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ACE_COND_RESOURCE_ATTRIBUTE_PREFIX: &str = "@RESOURCE.";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ACE_COND_SID_PREFIX: &str = "SID";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ACE_COND_TOKEN_ATTRIBUTE_PREFIX: &str = "@TOKEN.";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ACE_COND_USER_ATTRIBUTE_PREFIX: &str = "@USER.";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ACE_END: &str = ")";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ALARM: &str = "AL";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ALIAS_PREW2KCOMPACC: &str = "RU";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ALIAS_SIZE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ALL_APP_PACKAGES: &str = "AC";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ANONYMOUS: &str = "AN";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_AUDIT: &str = "AU";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_AUDIT_FAILURE: &str = "FA";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_AUDIT_SUCCESS: &str = "SA";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_AUTHENTICATED_USERS: &str = "AU";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_AUTHORITY_ASSERTED: &str = "AS";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_AUTO_INHERITED: &str = "AI";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_AUTO_INHERIT_REQ: &str = "AR";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_BACKUP_OPERATORS: &str = "BO";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_BLOB: &str = "TX";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_BOOLEAN: &str = "TB";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_BUILTIN_ADMINISTRATORS: &str = "BA";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_BUILTIN_GUESTS: &str = "BG";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_BUILTIN_USERS: &str = "BU";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_CALLBACK_ACCESS_ALLOWED: &str = "XA";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_CALLBACK_ACCESS_DENIED: &str = "XD";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_CALLBACK_AUDIT: &str = "XU";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_CALLBACK_OBJECT_ACCESS_ALLOWED: &str = "ZA";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_CERTSVC_DCOM_ACCESS: &str = "CD";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_CERT_SERV_ADMINISTRATORS: &str = "CA";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_CLONEABLE_CONTROLLERS: &str = "CN";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_CONTAINER_INHERIT: &str = "CI";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_CONTROL_ACCESS: &str = "CR";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_CREATE_CHILD: &str = "CC";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_CREATOR_GROUP: &str = "CG";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_CREATOR_OWNER: &str = "CO";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_CRITICAL: &str = "CR";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_CRYPTO_OPERATORS: &str = "CY";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_DACL: &str = "D";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_DELETE_CHILD: &str = "DC";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_DELETE_TREE: &str = "DT";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_DELIMINATOR: &str = ":";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_DOMAIN_ADMINISTRATORS: &str = "DA";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_DOMAIN_COMPUTERS: &str = "DC";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_DOMAIN_DOMAIN_CONTROLLERS: &str = "DD";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_DOMAIN_GUESTS: &str = "DG";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_DOMAIN_USERS: &str = "DU";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ENTERPRISE_ADMINS: &str = "EA";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ENTERPRISE_DOMAIN_CONTROLLERS: &str = "ED";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ENTERPRISE_KEY_ADMINS: &str = "EK";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ENTERPRISE_RO_DCs: &str = "RO";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_EVENT_LOG_READERS: &str = "ER";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_EVERYONE: &str = "WD";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_FILE_ALL: &str = "FA";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_FILE_EXECUTE: &str = "FX";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_FILE_READ: &str = "FR";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_FILE_WRITE: &str = "FW";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_GENERIC_ALL: &str = "GA";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_GENERIC_EXECUTE: &str = "GX";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_GENERIC_READ: &str = "GR";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_GENERIC_WRITE: &str = "GW";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_GROUP: &str = "G";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_GROUP_POLICY_ADMINS: &str = "PA";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_HYPER_V_ADMINS: &str = "HA";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_IIS_USERS: &str = "IS";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_INHERITED: &str = "ID";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_INHERIT_ONLY: &str = "IO";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_INT: &str = "TI";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_INTERACTIVE: &str = "IU";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_KEY_ADMINS: &str = "KA";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_KEY_ALL: &str = "KA";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_KEY_EXECUTE: &str = "KX";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_KEY_READ: &str = "KR";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_KEY_WRITE: &str = "KW";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_LIST_CHILDREN: &str = "LC";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_LIST_OBJECT: &str = "LO";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_LOCAL_ADMIN: &str = "LA";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_LOCAL_GUEST: &str = "LG";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_LOCAL_SERVICE: &str = "LS";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_LOCAL_SYSTEM: &str = "SY";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_MANDATORY_LABEL: &str = "ML";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ML_HIGH: &str = "HI";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ML_LOW: &str = "LW";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ML_MEDIUM: &str = "ME";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ML_MEDIUM_PLUS: &str = "MP";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ML_SYSTEM: &str = "SI";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_NETWORK: &str = "NU";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_NETWORK_CONFIGURATION_OPS: &str = "NO";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_NETWORK_SERVICE: &str = "NS";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_NO_EXECUTE_UP: &str = "NX";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_NO_PROPAGATE: &str = "NP";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_NO_READ_UP: &str = "NR";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_NO_WRITE_UP: &str = "NW";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_NULL_ACL: &str = "NO_ACCESS_CONTROL";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_OBJECT_ACCESS_ALLOWED: &str = "OA";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_OBJECT_ACCESS_DENIED: &str = "OD";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_OBJECT_ALARM: &str = "OL";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_OBJECT_AUDIT: &str = "OU";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_OBJECT_INHERIT: &str = "OI";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_OWNER: &str = "O";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_OWNER_RIGHTS: &str = "OW";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_PERFLOG_USERS: &str = "LU";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_PERFMON_USERS: &str = "MU";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_PERSONAL_SELF: &str = "PS";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_POWER_USERS: &str = "PU";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_PRINTER_OPERATORS: &str = "PO";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_PROCESS_TRUST_LABEL: &str = "TL";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_PROTECTED: &str = "P";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_PROTECTED_USERS: &str = "AP";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_RAS_SERVERS: &str = "RS";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_RDS_ENDPOINT_SERVERS: &str = "ES";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_RDS_MANAGEMENT_SERVERS: &str = "MS";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_RDS_REMOTE_ACCESS_SERVERS: &str = "RA";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_READ_CONTROL: &str = "RC";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_READ_PROPERTY: &str = "RP";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_REMOTE_DESKTOP: &str = "RD";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_REMOTE_MANAGEMENT_USERS: &str = "RM";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_REPLICATOR: &str = "RE";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_RESOURCE_ATTRIBUTE: &str = "RA";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_RESTRICTED_CODE: &str = "RC";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_REVISION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_SACL: &str = "S";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_SCHEMA_ADMINISTRATORS: &str = "SA";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_SCOPED_POLICY_ID: &str = "SP";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_SELF_WRITE: &str = "SW";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_SEPERATOR: &str = ";";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_SERVER_OPERATORS: &str = "SO";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_SERVICE: &str = "SU";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_SERVICE_ASSERTED: &str = "SS";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_SID: &str = "TD";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_SPACE: &str = " ";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_STANDARD_DELETE: &str = "SD";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_TRUST_PROTECTED_FILTER: &str = "TP";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_UINT: &str = "TU";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_USER_MODE_DRIVERS: &str = "UD";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_WRITE_DAC: &str = "WD";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_WRITE_OWNER: &str = "WO";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_WRITE_PROPERTY: &str = "WP";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_WRITE_RESTRICTED_CODE: &str = "WR";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_WSTRING: &str = "TS";
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub type SE_OBJECT_TYPE = i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SE_UNKNOWN_OBJECT_TYPE: SE_OBJECT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SE_FILE_OBJECT: SE_OBJECT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SE_SERVICE: SE_OBJECT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SE_PRINTER: SE_OBJECT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SE_REGISTRY_KEY: SE_OBJECT_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SE_LMSHARE: SE_OBJECT_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SE_KERNEL_OBJECT: SE_OBJECT_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SE_WINDOW_OBJECT: SE_OBJECT_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SE_DS_OBJECT: SE_OBJECT_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SE_DS_OBJECT_ALL: SE_OBJECT_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SE_PROVIDER_DEFINED_OBJECT: SE_OBJECT_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SE_WMIGUID_OBJECT: SE_OBJECT_TYPE = 11i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SE_REGISTRY_WOW64_32KEY: SE_OBJECT_TYPE = 12i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SE_REGISTRY_WOW64_64KEY: SE_OBJECT_TYPE = 13i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub type TREE_SEC_INFO = u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TREE_SEC_INFO_SET: TREE_SEC_INFO = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TREE_SEC_INFO_RESET: TREE_SEC_INFO = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TREE_SEC_INFO_RESET_KEEP_EXPLICIT: TREE_SEC_INFO = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct TRUSTEE_A {
    pub pMultipleTrustee: *mut TRUSTEE_A,
    pub MultipleTrusteeOperation: MULTIPLE_TRUSTEE_OPERATION,
    pub TrusteeForm: TRUSTEE_FORM,
    pub TrusteeType: TRUSTEE_TYPE,
    pub ptstrName: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for TRUSTEE_A {}
impl ::core::clone::Clone for TRUSTEE_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct TRUSTEE_ACCESSA {
    pub lpProperty: ::windows_sys::core::PSTR,
    pub Access: u32,
    pub fAccessFlags: u32,
    pub fReturnedAccess: u32,
}
impl ::core::marker::Copy for TRUSTEE_ACCESSA {}
impl ::core::clone::Clone for TRUSTEE_ACCESSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct TRUSTEE_ACCESSW {
    pub lpProperty: ::windows_sys::core::PWSTR,
    pub Access: u32,
    pub fAccessFlags: u32,
    pub fReturnedAccess: u32,
}
impl ::core::marker::Copy for TRUSTEE_ACCESSW {}
impl ::core::clone::Clone for TRUSTEE_ACCESSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_ACCESS_ALL: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_ACCESS_ALLOWED: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_ACCESS_EXPLICIT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_ACCESS_READ: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_ACCESS_WRITE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub type TRUSTEE_FORM = i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_IS_SID: TRUSTEE_FORM = 0i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_IS_NAME: TRUSTEE_FORM = 1i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_BAD_FORM: TRUSTEE_FORM = 2i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_IS_OBJECTS_AND_SID: TRUSTEE_FORM = 3i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_IS_OBJECTS_AND_NAME: TRUSTEE_FORM = 4i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub type TRUSTEE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_IS_UNKNOWN: TRUSTEE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_IS_USER: TRUSTEE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_IS_GROUP: TRUSTEE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_IS_DOMAIN: TRUSTEE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_IS_ALIAS: TRUSTEE_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_IS_WELL_KNOWN_GROUP: TRUSTEE_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_IS_DELETED: TRUSTEE_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_IS_INVALID: TRUSTEE_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_IS_COMPUTER: TRUSTEE_TYPE = 8i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct TRUSTEE_W {
    pub pMultipleTrustee: *mut TRUSTEE_W,
    pub MultipleTrusteeOperation: MULTIPLE_TRUSTEE_OPERATION,
    pub TrusteeForm: TRUSTEE_FORM,
    pub TrusteeType: TRUSTEE_TYPE,
    pub ptstrName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for TRUSTEE_W {}
impl ::core::clone::Clone for TRUSTEE_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const _AUTHZ_SS_MAXSIZE: u32 = 128u32;
