#[cfg(feature = "Win32_System_Search_Common")]
pub mod Common;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn ODBCGetTryWaitValue() -> u32;
    #[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ODBCSetTryWaitValue(dwvalue: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLAllocConnect(environmenthandle: *mut ::core::ffi::c_void, connectionhandle: *mut *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLAllocEnv(environmenthandle: *mut *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLAllocHandle(handletype: i16, inputhandle: *mut ::core::ffi::c_void, outputhandle: *mut *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLAllocHandleStd(fhandletype: i16, hinput: *mut ::core::ffi::c_void, phoutput: *mut *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLAllocStmt(connectionhandle: *mut ::core::ffi::c_void, statementhandle: *mut *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn SQLBindCol(statementhandle: *mut ::core::ffi::c_void, columnnumber: u16, targettype: i16, targetvalue: *mut ::core::ffi::c_void, bufferlength: i64, strlen_or_ind: *mut i64) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(target_arch = "x86")]
    pub fn SQLBindCol(statementhandle: *mut ::core::ffi::c_void, columnnumber: u16, targettype: i16, targetvalue: *mut ::core::ffi::c_void, bufferlength: i32, strlen_or_ind: *mut i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn SQLBindParam(statementhandle: *mut ::core::ffi::c_void, parameternumber: u16, valuetype: i16, parametertype: i16, lengthprecision: u64, parameterscale: i16, parametervalue: *mut ::core::ffi::c_void, strlen_or_ind: *mut i64) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(target_arch = "x86")]
    pub fn SQLBindParam(statementhandle: *mut ::core::ffi::c_void, parameternumber: u16, valuetype: i16, parametertype: i16, lengthprecision: u32, parameterscale: i16, parametervalue: *mut ::core::ffi::c_void, strlen_or_ind: *mut i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn SQLBindParameter(hstmt: *mut ::core::ffi::c_void, ipar: u16, fparamtype: i16, fctype: i16, fsqltype: i16, cbcoldef: u64, ibscale: i16, rgbvalue: *mut ::core::ffi::c_void, cbvaluemax: i64, pcbvalue: *mut i64) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(target_arch = "x86")]
    pub fn SQLBindParameter(hstmt: *mut ::core::ffi::c_void, ipar: u16, fparamtype: i16, fctype: i16, fsqltype: i16, cbcoldef: u32, ibscale: i16, rgbvalue: *mut ::core::ffi::c_void, cbvaluemax: i32, pcbvalue: *mut i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLBrowseConnect(hdbc: *mut ::core::ffi::c_void, szconnstrin: *const u8, cchconnstrin: i16, szconnstrout: *mut u8, cchconnstroutmax: i16, pcchconnstrout: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLBrowseConnectA(hdbc: *mut ::core::ffi::c_void, szconnstrin: *const u8, cbconnstrin: i16, szconnstrout: *mut u8, cbconnstroutmax: i16, pcbconnstrout: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLBrowseConnectW(hdbc: *mut ::core::ffi::c_void, szconnstrin: *const u16, cchconnstrin: i16, szconnstrout: *mut u16, cchconnstroutmax: i16, pcchconnstrout: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLBulkOperations(statementhandle: *mut ::core::ffi::c_void, operation: i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLCancel(statementhandle: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLCancelHandle(handletype: i16, inputhandle: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLCloseCursor(statementhandle: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SQLCloseEnumServers(henumhandle: super::super::Foundation::HANDLE) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn SQLColAttribute(statementhandle: *mut ::core::ffi::c_void, columnnumber: u16, fieldidentifier: u16, characterattribute: *mut ::core::ffi::c_void, bufferlength: i16, stringlength: *mut i16, numericattribute: *mut i64) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(target_arch = "x86")]
    pub fn SQLColAttribute(statementhandle: *mut ::core::ffi::c_void, columnnumber: u16, fieldidentifier: u16, characterattribute: *mut ::core::ffi::c_void, bufferlength: i16, stringlength: *mut i16, numericattribute: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn SQLColAttributeA(hstmt: *mut ::core::ffi::c_void, icol: i16, ifield: i16, pcharattr: *mut ::core::ffi::c_void, cbcharattrmax: i16, pcbcharattr: *mut i16, pnumattr: *mut i64) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(target_arch = "x86")]
    pub fn SQLColAttributeA(hstmt: *mut ::core::ffi::c_void, icol: i16, ifield: i16, pcharattr: *mut ::core::ffi::c_void, cbcharattrmax: i16, pcbcharattr: *mut i16, pnumattr: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn SQLColAttributeW(hstmt: *mut ::core::ffi::c_void, icol: u16, ifield: u16, pcharattr: *mut ::core::ffi::c_void, cbdescmax: i16, pcbcharattr: *mut i16, pnumattr: *mut i64) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(target_arch = "x86")]
    pub fn SQLColAttributeW(hstmt: *mut ::core::ffi::c_void, icol: u16, ifield: u16, pcharattr: *mut ::core::ffi::c_void, cbdescmax: i16, pcbcharattr: *mut i16, pnumattr: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn SQLColAttributes(hstmt: *mut ::core::ffi::c_void, icol: u16, fdesctype: u16, rgbdesc: *mut ::core::ffi::c_void, cbdescmax: i16, pcbdesc: *mut i16, pfdesc: *mut i64) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(target_arch = "x86")]
    pub fn SQLColAttributes(hstmt: *mut ::core::ffi::c_void, icol: u16, fdesctype: u16, rgbdesc: *mut ::core::ffi::c_void, cbdescmax: i16, pcbdesc: *mut i16, pfdesc: *mut i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn SQLColAttributesA(hstmt: *mut ::core::ffi::c_void, icol: u16, fdesctype: u16, rgbdesc: *mut ::core::ffi::c_void, cbdescmax: i16, pcbdesc: *mut i16, pfdesc: *mut i64) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(target_arch = "x86")]
    pub fn SQLColAttributesA(hstmt: *mut ::core::ffi::c_void, icol: u16, fdesctype: u16, rgbdesc: *mut ::core::ffi::c_void, cbdescmax: i16, pcbdesc: *mut i16, pfdesc: *mut i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn SQLColAttributesW(hstmt: *mut ::core::ffi::c_void, icol: u16, fdesctype: u16, rgbdesc: *mut ::core::ffi::c_void, cbdescmax: i16, pcbdesc: *mut i16, pfdesc: *mut i64) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(target_arch = "x86")]
    pub fn SQLColAttributesW(hstmt: *mut ::core::ffi::c_void, icol: u16, fdesctype: u16, rgbdesc: *mut ::core::ffi::c_void, cbdescmax: i16, pcbdesc: *mut i16, pfdesc: *mut i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLColumnPrivileges(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cchcatalogname: i16, szschemaname: *const u8, cchschemaname: i16, sztablename: *const u8, cchtablename: i16, szcolumnname: *const u8, cchcolumnname: i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLColumnPrivilegesA(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cbcatalogname: i16, szschemaname: *const u8, cbschemaname: i16, sztablename: *const u8, cbtablename: i16, szcolumnname: *const u8, cbcolumnname: i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLColumnPrivilegesW(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u16, cchcatalogname: i16, szschemaname: *const u16, cchschemaname: i16, sztablename: *const u16, cchtablename: i16, szcolumnname: *const u16, cchcolumnname: i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLColumns(statementhandle: *mut ::core::ffi::c_void, catalogname: *const u8, namelength1: i16, schemaname: *const u8, namelength2: i16, tablename: *const u8, namelength3: i16, columnname: *const u8, namelength4: i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLColumnsA(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cbcatalogname: i16, szschemaname: *const u8, cbschemaname: i16, sztablename: *const u8, cbtablename: i16, szcolumnname: *const u8, cbcolumnname: i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLColumnsW(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u16, cchcatalogname: i16, szschemaname: *const u16, cchschemaname: i16, sztablename: *const u16, cchtablename: i16, szcolumnname: *const u16, cchcolumnname: i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLCompleteAsync(handletype: i16, handle: *mut ::core::ffi::c_void, asyncretcodeptr: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLConnect(connectionhandle: *mut ::core::ffi::c_void, servername: *const u8, namelength1: i16, username: *const u8, namelength2: i16, authentication: *const u8, namelength3: i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLConnectA(hdbc: *mut ::core::ffi::c_void, szdsn: *const u8, cbdsn: i16, szuid: *const u8, cbuid: i16, szauthstr: *const u8, cbauthstr: i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLConnectW(hdbc: *mut ::core::ffi::c_void, szdsn: *const u16, cchdsn: i16, szuid: *const u16, cchuid: i16, szauthstr: *const u16, cchauthstr: i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLCopyDesc(sourcedeschandle: *mut ::core::ffi::c_void, targetdeschandle: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLDataSources(environmenthandle: *mut ::core::ffi::c_void, direction: u16, servername: *mut u8, bufferlength1: i16, namelength1ptr: *mut i16, description: *mut u8, bufferlength2: i16, namelength2ptr: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLDataSourcesA(henv: *mut ::core::ffi::c_void, fdirection: u16, szdsn: *mut u8, cbdsnmax: i16, pcbdsn: *mut i16, szdescription: *mut u8, cbdescriptionmax: i16, pcbdescription: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLDataSourcesW(henv: *mut ::core::ffi::c_void, fdirection: u16, szdsn: *mut u16, cchdsnmax: i16, pcchdsn: *mut i16, wszdescription: *mut u16, cchdescriptionmax: i16, pcchdescription: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn SQLDescribeCol(statementhandle: *mut ::core::ffi::c_void, columnnumber: u16, columnname: *mut u8, bufferlength: i16, namelength: *mut i16, datatype: *mut i16, columnsize: *mut u64, decimaldigits: *mut i16, nullable: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(target_arch = "x86")]
    pub fn SQLDescribeCol(statementhandle: *mut ::core::ffi::c_void, columnnumber: u16, columnname: *mut u8, bufferlength: i16, namelength: *mut i16, datatype: *mut i16, columnsize: *mut u32, decimaldigits: *mut i16, nullable: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn SQLDescribeColA(hstmt: *mut ::core::ffi::c_void, icol: u16, szcolname: *mut u8, cbcolnamemax: i16, pcbcolname: *mut i16, pfsqltype: *mut i16, pcbcoldef: *mut u64, pibscale: *mut i16, pfnullable: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(target_arch = "x86")]
    pub fn SQLDescribeColA(hstmt: *mut ::core::ffi::c_void, icol: u16, szcolname: *mut u8, cbcolnamemax: i16, pcbcolname: *mut i16, pfsqltype: *mut i16, pcbcoldef: *mut u32, pibscale: *mut i16, pfnullable: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn SQLDescribeColW(hstmt: *mut ::core::ffi::c_void, icol: u16, szcolname: *mut u16, cchcolnamemax: i16, pcchcolname: *mut i16, pfsqltype: *mut i16, pcbcoldef: *mut u64, pibscale: *mut i16, pfnullable: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(target_arch = "x86")]
    pub fn SQLDescribeColW(hstmt: *mut ::core::ffi::c_void, icol: u16, szcolname: *mut u16, cchcolnamemax: i16, pcchcolname: *mut i16, pfsqltype: *mut i16, pcbcoldef: *mut u32, pibscale: *mut i16, pfnullable: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn SQLDescribeParam(hstmt: *mut ::core::ffi::c_void, ipar: u16, pfsqltype: *mut i16, pcbparamdef: *mut u64, pibscale: *mut i16, pfnullable: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(target_arch = "x86")]
    pub fn SQLDescribeParam(hstmt: *mut ::core::ffi::c_void, ipar: u16, pfsqltype: *mut i16, pcbparamdef: *mut u32, pibscale: *mut i16, pfnullable: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLDisconnect(connectionhandle: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLDriverConnect(hdbc: *mut ::core::ffi::c_void, hwnd: isize, szconnstrin: *const u8, cchconnstrin: i16, szconnstrout: *mut u8, cchconnstroutmax: i16, pcchconnstrout: *mut i16, fdrivercompletion: u16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLDriverConnectA(hdbc: *mut ::core::ffi::c_void, hwnd: isize, szconnstrin: *const u8, cbconnstrin: i16, szconnstrout: *mut u8, cbconnstroutmax: i16, pcbconnstrout: *mut i16, fdrivercompletion: u16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLDriverConnectW(hdbc: *mut ::core::ffi::c_void, hwnd: isize, szconnstrin: *const u16, cchconnstrin: i16, szconnstrout: *mut u16, cchconnstroutmax: i16, pcchconnstrout: *mut i16, fdrivercompletion: u16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLDrivers(henv: *mut ::core::ffi::c_void, fdirection: u16, szdriverdesc: *mut u8, cchdriverdescmax: i16, pcchdriverdesc: *mut i16, szdriverattributes: *mut u8, cchdrvrattrmax: i16, pcchdrvrattr: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLDriversA(henv: *mut ::core::ffi::c_void, fdirection: u16, szdriverdesc: *mut u8, cbdriverdescmax: i16, pcbdriverdesc: *mut i16, szdriverattributes: *mut u8, cbdrvrattrmax: i16, pcbdrvrattr: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLDriversW(henv: *mut ::core::ffi::c_void, fdirection: u16, szdriverdesc: *mut u16, cchdriverdescmax: i16, pcchdriverdesc: *mut i16, szdriverattributes: *mut u16, cchdrvrattrmax: i16, pcchdrvrattr: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLEndTran(handletype: i16, handle: *mut ::core::ffi::c_void, completiontype: i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLError(environmenthandle: *mut ::core::ffi::c_void, connectionhandle: *mut ::core::ffi::c_void, statementhandle: *mut ::core::ffi::c_void, sqlstate: *mut u8, nativeerror: *mut i32, messagetext: *mut u8, bufferlength: i16, textlength: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLErrorA(henv: *mut ::core::ffi::c_void, hdbc: *mut ::core::ffi::c_void, hstmt: *mut ::core::ffi::c_void, szsqlstate: *mut u8, pfnativeerror: *mut i32, szerrormsg: *mut u8, cberrormsgmax: i16, pcberrormsg: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLErrorW(henv: *mut ::core::ffi::c_void, hdbc: *mut ::core::ffi::c_void, hstmt: *mut ::core::ffi::c_void, wszsqlstate: *mut u16, pfnativeerror: *mut i32, wszerrormsg: *mut u16, ccherrormsgmax: i16, pccherrormsg: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLExecDirect(statementhandle: *mut ::core::ffi::c_void, statementtext: *const u8, textlength: i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLExecDirectA(hstmt: *mut ::core::ffi::c_void, szsqlstr: *const u8, cbsqlstr: i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLExecDirectW(hstmt: *mut ::core::ffi::c_void, szsqlstr: *const u16, textlength: i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLExecute(statementhandle: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn SQLExtendedFetch(hstmt: *mut ::core::ffi::c_void, ffetchtype: u16, irow: i64, pcrow: *mut u64, rgfrowstatus: *mut u16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(target_arch = "x86")]
    pub fn SQLExtendedFetch(hstmt: *mut ::core::ffi::c_void, ffetchtype: u16, irow: i32, pcrow: *mut u32, rgfrowstatus: *mut u16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLFetch(statementhandle: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn SQLFetchScroll(statementhandle: *mut ::core::ffi::c_void, fetchorientation: i16, fetchoffset: i64) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(target_arch = "x86")]
    pub fn SQLFetchScroll(statementhandle: *mut ::core::ffi::c_void, fetchorientation: i16, fetchoffset: i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLForeignKeys(hstmt: *mut ::core::ffi::c_void, szpkcatalogname: *const u8, cchpkcatalogname: i16, szpkschemaname: *const u8, cchpkschemaname: i16, szpktablename: *const u8, cchpktablename: i16, szfkcatalogname: *const u8, cchfkcatalogname: i16, szfkschemaname: *const u8, cchfkschemaname: i16, szfktablename: *const u8, cchfktablename: i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLForeignKeysA(hstmt: *mut ::core::ffi::c_void, szpkcatalogname: *const u8, cbpkcatalogname: i16, szpkschemaname: *const u8, cbpkschemaname: i16, szpktablename: *const u8, cbpktablename: i16, szfkcatalogname: *const u8, cbfkcatalogname: i16, szfkschemaname: *const u8, cbfkschemaname: i16, szfktablename: *const u8, cbfktablename: i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLForeignKeysW(hstmt: *mut ::core::ffi::c_void, szpkcatalogname: *const u16, cchpkcatalogname: i16, szpkschemaname: *const u16, cchpkschemaname: i16, szpktablename: *const u16, cchpktablename: i16, szfkcatalogname: *const u16, cchfkcatalogname: i16, szfkschemaname: *const u16, cchfkschemaname: i16, szfktablename: *const u16, cchfktablename: i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLFreeConnect(connectionhandle: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLFreeEnv(environmenthandle: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLFreeHandle(handletype: i16, handle: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLFreeStmt(statementhandle: *mut ::core::ffi::c_void, option: u16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLGetConnectAttr(connectionhandle: *mut ::core::ffi::c_void, attribute: i32, value: *mut ::core::ffi::c_void, bufferlength: i32, stringlengthptr: *mut i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLGetConnectAttrA(hdbc: *mut ::core::ffi::c_void, fattribute: i32, rgbvalue: *mut ::core::ffi::c_void, cbvaluemax: i32, pcbvalue: *mut i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLGetConnectAttrW(hdbc: *mut ::core::ffi::c_void, fattribute: i32, rgbvalue: *mut ::core::ffi::c_void, cbvaluemax: i32, pcbvalue: *mut i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLGetConnectOption(connectionhandle: *mut ::core::ffi::c_void, option: u16, value: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLGetConnectOptionA(hdbc: *mut ::core::ffi::c_void, foption: u16, pvparam: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLGetConnectOptionW(hdbc: *mut ::core::ffi::c_void, foption: u16, pvparam: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLGetCursorName(statementhandle: *mut ::core::ffi::c_void, cursorname: *mut u8, bufferlength: i16, namelengthptr: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLGetCursorNameA(hstmt: *mut ::core::ffi::c_void, szcursor: *mut u8, cbcursormax: i16, pcbcursor: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLGetCursorNameW(hstmt: *mut ::core::ffi::c_void, szcursor: *mut u16, cchcursormax: i16, pcchcursor: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn SQLGetData(statementhandle: *mut ::core::ffi::c_void, columnnumber: u16, targettype: i16, targetvalue: *mut ::core::ffi::c_void, bufferlength: i64, strlen_or_indptr: *mut i64) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(target_arch = "x86")]
    pub fn SQLGetData(statementhandle: *mut ::core::ffi::c_void, columnnumber: u16, targettype: i16, targetvalue: *mut ::core::ffi::c_void, bufferlength: i32, strlen_or_indptr: *mut i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLGetDescField(descriptorhandle: *mut ::core::ffi::c_void, recnumber: i16, fieldidentifier: i16, value: *mut ::core::ffi::c_void, bufferlength: i32, stringlength: *mut i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLGetDescFieldA(hdesc: *mut ::core::ffi::c_void, irecord: i16, ifield: i16, rgbvalue: *mut ::core::ffi::c_void, cbbufferlength: i32, stringlength: *mut i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLGetDescFieldW(hdesc: *mut ::core::ffi::c_void, irecord: i16, ifield: i16, rgbvalue: *mut ::core::ffi::c_void, cbbufferlength: i32, stringlength: *mut i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn SQLGetDescRec(descriptorhandle: *mut ::core::ffi::c_void, recnumber: i16, name: *mut u8, bufferlength: i16, stringlengthptr: *mut i16, typeptr: *mut i16, subtypeptr: *mut i16, lengthptr: *mut i64, precisionptr: *mut i16, scaleptr: *mut i16, nullableptr: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(target_arch = "x86")]
    pub fn SQLGetDescRec(descriptorhandle: *mut ::core::ffi::c_void, recnumber: i16, name: *mut u8, bufferlength: i16, stringlengthptr: *mut i16, typeptr: *mut i16, subtypeptr: *mut i16, lengthptr: *mut i32, precisionptr: *mut i16, scaleptr: *mut i16, nullableptr: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn SQLGetDescRecA(hdesc: *mut ::core::ffi::c_void, irecord: i16, szname: *mut u8, cbnamemax: i16, pcbname: *mut i16, pftype: *mut i16, pfsubtype: *mut i16, plength: *mut i64, pprecision: *mut i16, pscale: *mut i16, pnullable: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(target_arch = "x86")]
    pub fn SQLGetDescRecA(hdesc: *mut ::core::ffi::c_void, irecord: i16, szname: *mut u8, cbnamemax: i16, pcbname: *mut i16, pftype: *mut i16, pfsubtype: *mut i16, plength: *mut i32, pprecision: *mut i16, pscale: *mut i16, pnullable: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn SQLGetDescRecW(hdesc: *mut ::core::ffi::c_void, irecord: i16, szname: *mut u16, cchnamemax: i16, pcchname: *mut i16, pftype: *mut i16, pfsubtype: *mut i16, plength: *mut i64, pprecision: *mut i16, pscale: *mut i16, pnullable: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(target_arch = "x86")]
    pub fn SQLGetDescRecW(hdesc: *mut ::core::ffi::c_void, irecord: i16, szname: *mut u16, cchnamemax: i16, pcchname: *mut i16, pftype: *mut i16, pfsubtype: *mut i16, plength: *mut i32, pprecision: *mut i16, pscale: *mut i16, pnullable: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLGetDiagField(handletype: i16, handle: *mut ::core::ffi::c_void, recnumber: i16, diagidentifier: i16, diaginfo: *mut ::core::ffi::c_void, bufferlength: i16, stringlength: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLGetDiagFieldA(fhandletype: i16, handle: *mut ::core::ffi::c_void, irecord: i16, fdiagfield: i16, rgbdiaginfo: *mut ::core::ffi::c_void, cbdiaginfomax: i16, pcbdiaginfo: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLGetDiagFieldW(fhandletype: i16, handle: *mut ::core::ffi::c_void, irecord: i16, fdiagfield: i16, rgbdiaginfo: *mut ::core::ffi::c_void, cbbufferlength: i16, pcbstringlength: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLGetDiagRec(handletype: i16, handle: *mut ::core::ffi::c_void, recnumber: i16, sqlstate: *mut u8, nativeerror: *mut i32, messagetext: *mut u8, bufferlength: i16, textlength: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLGetDiagRecA(fhandletype: i16, handle: *mut ::core::ffi::c_void, irecord: i16, szsqlstate: *mut u8, pfnativeerror: *mut i32, szerrormsg: *mut u8, cberrormsgmax: i16, pcberrormsg: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLGetDiagRecW(fhandletype: i16, handle: *mut ::core::ffi::c_void, irecord: i16, szsqlstate: *mut u16, pfnativeerror: *mut i32, szerrormsg: *mut u16, ccherrormsgmax: i16, pccherrormsg: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLGetEnvAttr(environmenthandle: *mut ::core::ffi::c_void, attribute: i32, value: *mut ::core::ffi::c_void, bufferlength: i32, stringlength: *mut i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLGetFunctions(connectionhandle: *mut ::core::ffi::c_void, functionid: u16, supported: *mut u16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLGetInfo(connectionhandle: *mut ::core::ffi::c_void, infotype: u16, infovalue: *mut ::core::ffi::c_void, bufferlength: i16, stringlengthptr: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLGetInfoA(hdbc: *mut ::core::ffi::c_void, finfotype: u16, rgbinfovalue: *mut ::core::ffi::c_void, cbinfovaluemax: i16, pcbinfovalue: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLGetInfoW(hdbc: *mut ::core::ffi::c_void, finfotype: u16, rgbinfovalue: *mut ::core::ffi::c_void, cbinfovaluemax: i16, pcbinfovalue: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SQLGetNextEnumeration(henumhandle: super::super::Foundation::HANDLE, prgenumdata: *mut u8, pienumlength: *mut i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLGetStmtAttr(statementhandle: *mut ::core::ffi::c_void, attribute: i32, value: *mut ::core::ffi::c_void, bufferlength: i32, stringlength: *mut i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLGetStmtAttrA(hstmt: *mut ::core::ffi::c_void, fattribute: i32, rgbvalue: *mut ::core::ffi::c_void, cbvaluemax: i32, pcbvalue: *mut i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLGetStmtAttrW(hstmt: *mut ::core::ffi::c_void, fattribute: i32, rgbvalue: *mut ::core::ffi::c_void, cbvaluemax: i32, pcbvalue: *mut i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLGetStmtOption(statementhandle: *mut ::core::ffi::c_void, option: u16, value: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLGetTypeInfo(statementhandle: *mut ::core::ffi::c_void, datatype: i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLGetTypeInfoA(statementhandle: *mut ::core::ffi::c_void, datatype: i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLGetTypeInfoW(statementhandle: *mut ::core::ffi::c_void, datatype: i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SQLInitEnumServers(pwchservername: ::windows_sys::core::PCWSTR, pwchinstancename: ::windows_sys::core::PCWSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLLinkedCatalogsA(param0: *mut ::core::ffi::c_void, param1: ::windows_sys::core::PCSTR, param2: i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLLinkedCatalogsW(param0: *mut ::core::ffi::c_void, param1: ::windows_sys::core::PCWSTR, param2: i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLLinkedServers(param0: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLMoreResults(hstmt: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLNativeSql(hdbc: *mut ::core::ffi::c_void, szsqlstrin: *const u8, cchsqlstrin: i32, szsqlstr: *mut u8, cchsqlstrmax: i32, pcbsqlstr: *mut i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLNativeSqlA(hdbc: *mut ::core::ffi::c_void, szsqlstrin: *const u8, cbsqlstrin: i32, szsqlstr: *mut u8, cbsqlstrmax: i32, pcbsqlstr: *mut i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLNativeSqlW(hdbc: *mut ::core::ffi::c_void, szsqlstrin: *const u16, cchsqlstrin: i32, szsqlstr: *mut u16, cchsqlstrmax: i32, pcchsqlstr: *mut i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLNumParams(hstmt: *mut ::core::ffi::c_void, pcpar: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLNumResultCols(statementhandle: *mut ::core::ffi::c_void, columncount: *mut i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLParamData(statementhandle: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn SQLParamOptions(hstmt: *mut ::core::ffi::c_void, crow: u64, pirow: *mut u64) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(target_arch = "x86")]
    pub fn SQLParamOptions(hstmt: *mut ::core::ffi::c_void, crow: u32, pirow: *mut u32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLPrepare(statementhandle: *mut ::core::ffi::c_void, statementtext: *const u8, textlength: i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLPrepareA(hstmt: *mut ::core::ffi::c_void, szsqlstr: *const u8, cbsqlstr: i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLPrepareW(hstmt: *mut ::core::ffi::c_void, szsqlstr: *const u16, cchsqlstr: i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLPrimaryKeys(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cchcatalogname: i16, szschemaname: *const u8, cchschemaname: i16, sztablename: *const u8, cchtablename: i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLPrimaryKeysA(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cbcatalogname: i16, szschemaname: *const u8, cbschemaname: i16, sztablename: *const u8, cbtablename: i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLPrimaryKeysW(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u16, cchcatalogname: i16, szschemaname: *const u16, cchschemaname: i16, sztablename: *const u16, cchtablename: i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLProcedureColumns(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cchcatalogname: i16, szschemaname: *const u8, cchschemaname: i16, szprocname: *const u8, cchprocname: i16, szcolumnname: *const u8, cchcolumnname: i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLProcedureColumnsA(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cbcatalogname: i16, szschemaname: *const u8, cbschemaname: i16, szprocname: *const u8, cbprocname: i16, szcolumnname: *const u8, cbcolumnname: i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLProcedureColumnsW(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u16, cchcatalogname: i16, szschemaname: *const u16, cchschemaname: i16, szprocname: *const u16, cchprocname: i16, szcolumnname: *const u16, cchcolumnname: i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLProcedures(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cchcatalogname: i16, szschemaname: *const u8, cchschemaname: i16, szprocname: *const u8, cchprocname: i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLProceduresA(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cbcatalogname: i16, szschemaname: *const u8, cbschemaname: i16, szprocname: *const u8, cbprocname: i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLProceduresW(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u16, cchcatalogname: i16, szschemaname: *const u16, cchschemaname: i16, szprocname: *const u16, cchprocname: i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn SQLPutData(statementhandle: *mut ::core::ffi::c_void, data: *const ::core::ffi::c_void, strlen_or_ind: i64) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(target_arch = "x86")]
    pub fn SQLPutData(statementhandle: *mut ::core::ffi::c_void, data: *const ::core::ffi::c_void, strlen_or_ind: i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn SQLRowCount(statementhandle: *const ::core::ffi::c_void, rowcount: *mut i64) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(target_arch = "x86")]
    pub fn SQLRowCount(statementhandle: *const ::core::ffi::c_void, rowcount: *mut i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLSetConnectAttr(connectionhandle: *mut ::core::ffi::c_void, attribute: i32, value: *const ::core::ffi::c_void, stringlength: i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLSetConnectAttrA(hdbc: *mut ::core::ffi::c_void, fattribute: i32, rgbvalue: *const ::core::ffi::c_void, cbvalue: i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLSetConnectAttrW(hdbc: *mut ::core::ffi::c_void, fattribute: i32, rgbvalue: *const ::core::ffi::c_void, cbvalue: i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn SQLSetConnectOption(connectionhandle: *mut ::core::ffi::c_void, option: u16, value: u64) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(target_arch = "x86")]
    pub fn SQLSetConnectOption(connectionhandle: *mut ::core::ffi::c_void, option: u16, value: u32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn SQLSetConnectOptionA(hdbc: *mut ::core::ffi::c_void, foption: u16, vparam: u64) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(target_arch = "x86")]
    pub fn SQLSetConnectOptionA(hdbc: *mut ::core::ffi::c_void, foption: u16, vparam: u32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn SQLSetConnectOptionW(hdbc: *mut ::core::ffi::c_void, foption: u16, vparam: u64) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(target_arch = "x86")]
    pub fn SQLSetConnectOptionW(hdbc: *mut ::core::ffi::c_void, foption: u16, vparam: u32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLSetCursorName(statementhandle: *mut ::core::ffi::c_void, cursorname: *const u8, namelength: i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLSetCursorNameA(hstmt: *mut ::core::ffi::c_void, szcursor: *const u8, cbcursor: i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLSetCursorNameW(hstmt: *mut ::core::ffi::c_void, szcursor: *const u16, cchcursor: i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLSetDescField(descriptorhandle: *mut ::core::ffi::c_void, recnumber: i16, fieldidentifier: i16, value: *const ::core::ffi::c_void, bufferlength: i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLSetDescFieldW(descriptorhandle: *mut ::core::ffi::c_void, recnumber: i16, fieldidentifier: i16, value: *mut ::core::ffi::c_void, bufferlength: i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn SQLSetDescRec(descriptorhandle: *mut ::core::ffi::c_void, recnumber: i16, r#type: i16, subtype: i16, length: i64, precision: i16, scale: i16, data: *mut ::core::ffi::c_void, stringlength: *mut i64, indicator: *mut i64) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(target_arch = "x86")]
    pub fn SQLSetDescRec(descriptorhandle: *mut ::core::ffi::c_void, recnumber: i16, r#type: i16, subtype: i16, length: i32, precision: i16, scale: i16, data: *mut ::core::ffi::c_void, stringlength: *mut i32, indicator: *mut i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLSetEnvAttr(environmenthandle: *mut ::core::ffi::c_void, attribute: i32, value: *const ::core::ffi::c_void, stringlength: i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn SQLSetParam(statementhandle: *mut ::core::ffi::c_void, parameternumber: u16, valuetype: i16, parametertype: i16, lengthprecision: u64, parameterscale: i16, parametervalue: *const ::core::ffi::c_void, strlen_or_ind: *mut i64) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(target_arch = "x86")]
    pub fn SQLSetParam(statementhandle: *mut ::core::ffi::c_void, parameternumber: u16, valuetype: i16, parametertype: i16, lengthprecision: u32, parameterscale: i16, parametervalue: *const ::core::ffi::c_void, strlen_or_ind: *mut i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn SQLSetPos(hstmt: *mut ::core::ffi::c_void, irow: u64, foption: u16, flock: u16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(target_arch = "x86")]
    pub fn SQLSetPos(hstmt: *mut ::core::ffi::c_void, irow: u16, foption: u16, flock: u16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn SQLSetScrollOptions(hstmt: *mut ::core::ffi::c_void, fconcurrency: u16, crowkeyset: i64, crowrowset: u16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(target_arch = "x86")]
    pub fn SQLSetScrollOptions(hstmt: *mut ::core::ffi::c_void, fconcurrency: u16, crowkeyset: i32, crowrowset: u16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLSetStmtAttr(statementhandle: *mut ::core::ffi::c_void, attribute: i32, value: *const ::core::ffi::c_void, stringlength: i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLSetStmtAttrW(hstmt: *mut ::core::ffi::c_void, fattribute: i32, rgbvalue: *mut ::core::ffi::c_void, cbvaluemax: i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn SQLSetStmtOption(statementhandle: *mut ::core::ffi::c_void, option: u16, value: u64) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    #[cfg(target_arch = "x86")]
    pub fn SQLSetStmtOption(statementhandle: *mut ::core::ffi::c_void, option: u16, value: u32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLSpecialColumns(statementhandle: *mut ::core::ffi::c_void, identifiertype: u16, catalogname: *const u8, namelength1: i16, schemaname: *const u8, namelength2: i16, tablename: *const u8, namelength3: i16, scope: u16, nullable: u16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLSpecialColumnsA(hstmt: *mut ::core::ffi::c_void, fcoltype: u16, szcatalogname: *const u8, cbcatalogname: i16, szschemaname: *const u8, cbschemaname: i16, sztablename: *const u8, cbtablename: i16, fscope: u16, fnullable: u16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLSpecialColumnsW(hstmt: *mut ::core::ffi::c_void, fcoltype: u16, szcatalogname: *const u16, cchcatalogname: i16, szschemaname: *const u16, cchschemaname: i16, sztablename: *const u16, cchtablename: i16, fscope: u16, fnullable: u16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLStatistics(statementhandle: *mut ::core::ffi::c_void, catalogname: *const u8, namelength1: i16, schemaname: *const u8, namelength2: i16, tablename: *const u8, namelength3: i16, unique: u16, reserved: u16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLStatisticsA(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cbcatalogname: i16, szschemaname: *const u8, cbschemaname: i16, sztablename: *const u8, cbtablename: i16, funique: u16, faccuracy: u16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLStatisticsW(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u16, cchcatalogname: i16, szschemaname: *const u16, cchschemaname: i16, sztablename: *const u16, cchtablename: i16, funique: u16, faccuracy: u16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLTablePrivileges(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cchcatalogname: i16, szschemaname: *const u8, cchschemaname: i16, sztablename: *const u8, cchtablename: i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLTablePrivilegesA(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cbcatalogname: i16, szschemaname: *const u8, cbschemaname: i16, sztablename: *const u8, cbtablename: i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLTablePrivilegesW(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u16, cchcatalogname: i16, szschemaname: *const u16, cchschemaname: i16, sztablename: *const u16, cchtablename: i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLTables(statementhandle: *mut ::core::ffi::c_void, catalogname: *const u8, namelength1: i16, schemaname: *const u8, namelength2: i16, tablename: *const u8, namelength3: i16, tabletype: *const u8, namelength4: i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLTablesA(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cbcatalogname: i16, szschemaname: *const u8, cbschemaname: i16, sztablename: *const u8, cbtablename: i16, sztabletype: *const u8, cbtabletype: i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLTablesW(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u16, cchcatalogname: i16, szschemaname: *const u16, cchschemaname: i16, sztablename: *const u16, cchtablename: i16, sztabletype: *const u16, cchtabletype: i16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn SQLTransact(environmenthandle: *mut ::core::ffi::c_void, connectionhandle: *mut ::core::ffi::c_void, completiontype: u16) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn bcp_batch(param0: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn bcp_bind(param0: *mut ::core::ffi::c_void, param1: *mut u8, param2: i32, param3: i32, param4: *mut u8, param5: i32, param6: i32, param7: i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn bcp_colfmt(param0: *mut ::core::ffi::c_void, param1: i32, param2: u8, param3: i32, param4: i32, param5: *mut u8, param6: i32, param7: i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn bcp_collen(param0: *mut ::core::ffi::c_void, param1: i32, param2: i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn bcp_colptr(param0: *mut ::core::ffi::c_void, param1: *mut u8, param2: i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn bcp_columns(param0: *mut ::core::ffi::c_void, param1: i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn bcp_control(param0: *mut ::core::ffi::c_void, param1: i32, param2: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn bcp_done(param0: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn bcp_exec(param0: *mut ::core::ffi::c_void, param1: *mut i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn bcp_getcolfmt(param0: *mut ::core::ffi::c_void, param1: i32, param2: i32, param3: *mut ::core::ffi::c_void, param4: i32, param5: *mut i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn bcp_initA(param0: *mut ::core::ffi::c_void, param1: ::windows_sys::core::PCSTR, param2: ::windows_sys::core::PCSTR, param3: ::windows_sys::core::PCSTR, param4: i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn bcp_initW(param0: *mut ::core::ffi::c_void, param1: ::windows_sys::core::PCWSTR, param2: ::windows_sys::core::PCWSTR, param3: ::windows_sys::core::PCWSTR, param4: i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn bcp_moretext(param0: *mut ::core::ffi::c_void, param1: i32, param2: *mut u8) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn bcp_readfmtA(param0: *mut ::core::ffi::c_void, param1: ::windows_sys::core::PCSTR) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn bcp_readfmtW(param0: *mut ::core::ffi::c_void, param1: ::windows_sys::core::PCWSTR) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn bcp_sendrow(param0: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn bcp_setcolfmt(param0: *mut ::core::ffi::c_void, param1: i32, param2: i32, param3: *mut ::core::ffi::c_void, param4: i32) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn bcp_writefmtA(param0: *mut ::core::ffi::c_void, param1: ::windows_sys::core::PCSTR) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn bcp_writefmtW(param0: *mut ::core::ffi::c_void, param1: ::windows_sys::core::PCWSTR) -> i16;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn dbprtypeA(param0: i32) -> ::windows_sys::core::PSTR;
    #[doc = "*Required features: `\"Win32_System_Search\"`*"]
    pub fn dbprtypeW(param0: i32) -> ::windows_sys::core::PWSTR;
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type ACCESS_MASKENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PERM_EXCLUSIVE: ACCESS_MASKENUM = 512i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PERM_READDESIGN: ACCESS_MASKENUM = 1024i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PERM_WRITEDESIGN: ACCESS_MASKENUM = 2048i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PERM_WITHGRANT: ACCESS_MASKENUM = 4096i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PERM_REFERENCE: ACCESS_MASKENUM = 8192i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PERM_CREATE: ACCESS_MASKENUM = 16384i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PERM_INSERT: ACCESS_MASKENUM = 32768i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PERM_DELETE: ACCESS_MASKENUM = 65536i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PERM_READCONTROL: ACCESS_MASKENUM = 131072i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PERM_WRITEPERMISSIONS: ACCESS_MASKENUM = 262144i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PERM_WRITEOWNER: ACCESS_MASKENUM = 524288i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PERM_MAXIMUM_ALLOWED: ACCESS_MASKENUM = 33554432i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PERM_ALL: ACCESS_MASKENUM = 268435456i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PERM_EXECUTE: ACCESS_MASKENUM = 536870912i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PERM_READ: ACCESS_MASKENUM = -2147483648i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PERM_UPDATE: ACCESS_MASKENUM = 1073741824i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PERM_DROP: ACCESS_MASKENUM = 256i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub struct AUTHENTICATION_INFO {
    pub dwSize: u32,
    pub atAuthenticationType: AUTH_TYPE,
    pub pcwszUser: ::windows_sys::core::PCWSTR,
    pub pcwszPassword: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for AUTHENTICATION_INFO {}
impl ::core::clone::Clone for AUTHENTICATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type AUTH_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const eAUTH_TYPE_ANONYMOUS: AUTH_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const eAUTH_TYPE_NTLM: AUTH_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const eAUTH_TYPE_BASIC: AUTH_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const BCP6xFILEFMT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const BCPABORT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const BCPBATCH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const BCPFILECP: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const BCPFILECP_ACP: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const BCPFILECP_OEMCP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const BCPFILECP_RAW: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const BCPFILEFMT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const BCPFIRST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const BCPHINTS: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const BCPHINTSA: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const BCPHINTSW: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const BCPKEEPIDENTITY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const BCPKEEPNULLS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const BCPLAST: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const BCPMAXERRS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const BCPODBC: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const BCPTEXTFILE: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const BCPUNICODEFILE: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const BCP_FMT_COLLATION: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const BCP_FMT_COLLATION_ID: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const BCP_FMT_DATA_LEN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const BCP_FMT_INDICATOR_LEN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const BCP_FMT_SERVER_COL: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const BCP_FMT_TERMINATOR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const BCP_FMT_TYPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const BMK_DURABILITY_INTRANSACTION: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const BMK_DURABILITY_REORGANIZATION: i32 = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const BMK_DURABILITY_ROWSET: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const BMK_DURABILITY_XTRANSACTION: i32 = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub struct BUCKETCATEGORIZE {
    pub cBuckets: u32,
    pub Distribution: u32,
}
impl ::core::marker::Copy for BUCKETCATEGORIZE {}
impl ::core::clone::Clone for BUCKETCATEGORIZE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const BUCKET_EXPONENTIAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const BUCKET_LINEAR: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type CASE_REQUIREMENT = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CASE_REQUIREMENT_ANY: CASE_REQUIREMENT = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CASE_REQUIREMENT_UPPER_IF_AQS: CASE_REQUIREMENT = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
pub struct CATEGORIZATION {
    pub ulCatType: u32,
    pub Anonymous: CATEGORIZATION_0,
    pub csColumns: COLUMNSET,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for CATEGORIZATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for CATEGORIZATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
pub union CATEGORIZATION_0 {
    pub cClusters: u32,
    pub bucket: BUCKETCATEGORIZE,
    pub range: RANGECATEGORIZE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for CATEGORIZATION_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for CATEGORIZATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
pub struct CATEGORIZATIONSET {
    pub cCat: u32,
    pub aCat: *mut CATEGORIZATION,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for CATEGORIZATIONSET {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for CATEGORIZATIONSET {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CATEGORIZE_BUCKETS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CATEGORIZE_CLUSTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CATEGORIZE_RANGE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CATEGORIZE_UNIQUE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CATEGORY_COLLATOR: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CATEGORY_GATHERER: i32 = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CATEGORY_INDEXER: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CATEGORY_SEARCH: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CDBBMKDISPIDS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CDBCOLDISPIDS: u32 = 28u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CDBSELFDISPIDS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CERT_E_NOT_FOUND_OR_NO_PERMISSSION: i32 = -2147211263i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type CHANNEL_AGENT_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CHANNEL_AGENT_DYNAMIC_SCHEDULE: CHANNEL_AGENT_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CHANNEL_AGENT_PRECACHE_SOME: CHANNEL_AGENT_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CHANNEL_AGENT_PRECACHE_ALL: CHANNEL_AGENT_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CHANNEL_AGENT_PRECACHE_SCRNSAVER: CHANNEL_AGENT_FLAGS = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CI_E_CORRUPT_FWIDX: ::windows_sys::core::HRESULT = -1073473491i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CI_E_DIACRITIC_SETTINGS_DIFFER: ::windows_sys::core::HRESULT = -1073473490i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CI_E_INCONSISTENT_TRANSACTION: ::windows_sys::core::HRESULT = -1073473486i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CI_E_INVALID_CATALOG_LIST_VERSION: ::windows_sys::core::HRESULT = -2147215313i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CI_E_MULTIPLE_PROTECTED_USERS_UNSUPPORTED: ::windows_sys::core::HRESULT = -1073473483i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CI_E_NO_AUXMETADATA: ::windows_sys::core::HRESULT = -2147215318i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CI_E_NO_CATALOG_MANAGER: ::windows_sys::core::HRESULT = -1073473487i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CI_E_NO_PROTECTED_USER: ::windows_sys::core::HRESULT = -1073473484i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CI_E_PROTECTED_CATALOG_NON_INTERACTIVE_USER: ::windows_sys::core::HRESULT = -1073473481i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CI_E_PROTECTED_CATALOG_NOT_AVAILABLE: ::windows_sys::core::HRESULT = -1073473485i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CI_E_PROTECTED_CATALOG_SID_MISMATCH: ::windows_sys::core::HRESULT = -1073473482i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CI_S_CATALOG_RESET: ::windows_sys::core::HRESULT = 268336i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CI_S_CLIENT_REQUESTED_ABORT: ::windows_sys::core::HRESULT = 268331i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CI_S_NEW_AUXMETADATA: ::windows_sys::core::HRESULT = 268329i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CI_S_RETRY_DOCUMENT: ::windows_sys::core::HRESULT = 268332i32;
pub const CLSID_DataShapeProvider: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 877240776, data2: 50540, data3: 4560, data4: [173, 114, 0, 192, 79, 194, 152, 99] };
pub const CLSID_MSDASQL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3367314123, data2: 23795, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
pub const CLSID_MSDASQL_ENUMERATOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3367314125, data2: 23795, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
pub const CLSID_MSPersist: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2080891088, data2: 17432, data3: 4562, data4: [146, 18, 0, 192, 79, 187, 191, 179] };
pub const CLSID_SQLOLEDB: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 209711468, data2: 14563, data3: 4560, data4: [151, 171, 0, 192, 79, 194, 173, 152] };
pub const CLSID_SQLOLEDB_ENUMERATOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3751947150, data2: 59021, data3: 4560, data4: [151, 228, 0, 192, 79, 194, 173, 152] };
pub const CLSID_SQLOLEDB_ERROR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3230870626, data2: 14565, data3: 4560, data4: [151, 171, 0, 192, 79, 194, 173, 152] };
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type CLUSION_REASON = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CLUSIONREASON_UNKNOWNSCOPE: CLUSION_REASON = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CLUSIONREASON_DEFAULT: CLUSION_REASON = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CLUSIONREASON_USER: CLUSION_REASON = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CLUSIONREASON_GROUPPOLICY: CLUSION_REASON = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CMDLINE_E_ALREADY_INIT: i32 = -2147216123i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CMDLINE_E_NOT_INIT: i32 = -2147216124i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CMDLINE_E_NUM_PARAMS: i32 = -2147216122i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CMDLINE_E_PARAM_SIZE: i32 = -2147216125i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CMDLINE_E_PAREN: i32 = -2147216126i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CMDLINE_E_UNEXPECTED: i32 = -2147216127i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CM_E_CONNECTIONTIMEOUT: i32 = -2147219963i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CM_E_DATASOURCENOTAVAILABLE: i32 = -2147219964i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CM_E_INSUFFICIENTBUFFER: i32 = -2147219957i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CM_E_INVALIDDATASOURCE: i32 = -2147219959i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CM_E_NOQUERYCONNECTIONS: i32 = -2147219965i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CM_E_REGISTRY: i32 = -2147219960i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CM_E_SERVERNOTFOUND: i32 = -2147219962i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CM_E_TIMEOUT: i32 = -2147219958i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CM_E_TOOMANYDATASERVERS: i32 = -2147219967i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CM_E_TOOMANYDATASOURCES: i32 = -2147219966i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CM_S_NODATASERVERS: i32 = 263687i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const COLL_E_BADRESULT: i32 = -2147220218i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const COLL_E_BADSEQUENCE: i32 = -2147220223i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const COLL_E_BUFFERTOOSMALL: i32 = -2147220220i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const COLL_E_DUPLICATEDBID: i32 = -2147220216i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const COLL_E_INCOMPATIBLECOLUMNS: i32 = -2147220221i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const COLL_E_MAXCONNEXCEEDED: i32 = -2147220213i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const COLL_E_NODEFAULTCATALOG: i32 = -2147220214i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const COLL_E_NOMOREDATA: i32 = -2147220222i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const COLL_E_NOSORTCOLUMN: i32 = -2147220217i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const COLL_E_TOOMANYMERGECOLUMNS: i32 = -2147220215i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
pub struct COLUMNSET {
    pub cCol: u32,
    pub aCol: *mut super::super::Storage::IndexServer::FULLPROPSPEC,
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for COLUMNSET {}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for COLUMNSET {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type CONDITION_CREATION_OPTIONS = u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CONDITION_CREATION_DEFAULT: CONDITION_CREATION_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CONDITION_CREATION_NONE: CONDITION_CREATION_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CONDITION_CREATION_SIMPLIFY: CONDITION_CREATION_OPTIONS = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CONDITION_CREATION_VECTOR_AND: CONDITION_CREATION_OPTIONS = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CONDITION_CREATION_VECTOR_OR: CONDITION_CREATION_OPTIONS = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CONDITION_CREATION_VECTOR_LEAF: CONDITION_CREATION_OPTIONS = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CONDITION_CREATION_USE_CONTENT_LOCALE: CONDITION_CREATION_OPTIONS = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
pub struct CONTENTRESTRICTION {
    pub prop: super::super::Storage::IndexServer::FULLPROPSPEC,
    pub pwcsPhrase: ::windows_sys::core::PWSTR,
    pub lcid: u32,
    pub ulGenerateMethod: u32,
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for CONTENTRESTRICTION {}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for CONTENTRESTRICTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CONTENT_SOURCE_E_CONTENT_CLASS_READ: i32 = -2147208188i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CONTENT_SOURCE_E_CONTENT_SOURCE_COLUMN_TYPE: i32 = -2147208185i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CONTENT_SOURCE_E_NULL_CONTENT_CLASS_BSTR: i32 = -2147208186i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CONTENT_SOURCE_E_NULL_URI: i32 = -2147208183i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CONTENT_SOURCE_E_OUT_OF_RANGE: i32 = -2147208184i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CONTENT_SOURCE_E_PROPERTY_MAPPING_BAD_VECTOR_SIZE: i32 = -2147208189i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CONTENT_SOURCE_E_PROPERTY_MAPPING_READ: i32 = -2147208191i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CONTENT_SOURCE_E_UNEXPECTED_EXCEPTION: i32 = -2147208187i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CONTENT_SOURCE_E_UNEXPECTED_NULL_POINTER: i32 = -2147208190i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CQUERYDISPIDS: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CQUERYMETADISPIDS: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CQUERYPROPERTY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type CREATESUBSCRIPTIONFLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CREATESUBS_ADDTOFAVORITES: CREATESUBSCRIPTIONFLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CREATESUBS_FROMFAVORITES: CREATESUBSCRIPTIONFLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CREATESUBS_NOUI: CREATESUBSCRIPTIONFLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CREATESUBS_NOSAVE: CREATESUBSCRIPTIONFLAGS = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CREATESUBS_SOFTWAREUPDATE: CREATESUBSCRIPTIONFLAGS = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_DBSCHEMA_ASSERTIONS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_DBSCHEMA_CATALOGS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_DBSCHEMA_CHARACTER_SETS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_DBSCHEMA_CHECK_CONSTRAINTS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_DBSCHEMA_CHECK_CONSTRAINTS_BY_TABLE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_DBSCHEMA_COLLATIONS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_DBSCHEMA_COLUMNS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_DBSCHEMA_COLUMN_DOMAIN_USAGE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_DBSCHEMA_COLUMN_PRIVILEGES: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_DBSCHEMA_CONSTRAINT_COLUMN_USAGE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_DBSCHEMA_CONSTRAINT_TABLE_USAGE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_DBSCHEMA_FOREIGN_KEYS: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_DBSCHEMA_INDEXES: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_DBSCHEMA_KEY_COLUMN_USAGE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_DBSCHEMA_LINKEDSERVERS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_DBSCHEMA_OBJECTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_DBSCHEMA_OBJECT_ACTIONS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_DBSCHEMA_PRIMARY_KEYS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_DBSCHEMA_PROCEDURES: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_DBSCHEMA_PROCEDURE_COLUMNS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_DBSCHEMA_PROCEDURE_PARAMETERS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_DBSCHEMA_PROVIDER_TYPES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_DBSCHEMA_REFERENTIAL_CONSTRAINTS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_DBSCHEMA_SCHEMATA: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_DBSCHEMA_SQL_LANGUAGES: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_DBSCHEMA_STATISTICS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_DBSCHEMA_TABLES: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_DBSCHEMA_TABLES_INFO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_DBSCHEMA_TABLE_CONSTRAINTS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_DBSCHEMA_TABLE_PRIVILEGES: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_DBSCHEMA_TABLE_STATISTICS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_DBSCHEMA_TRANSLATIONS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_DBSCHEMA_TRUSTEE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_DBSCHEMA_USAGE_PRIVILEGES: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_DBSCHEMA_VIEWS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_DBSCHEMA_VIEW_COLUMN_USAGE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_DBSCHEMA_VIEW_TABLE_USAGE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_MDSCHEMA_ACTIONS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_MDSCHEMA_COMMANDS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_MDSCHEMA_CUBES: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_MDSCHEMA_DIMENSIONS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_MDSCHEMA_FUNCTIONS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_MDSCHEMA_HIERARCHIES: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_MDSCHEMA_LEVELS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_MDSCHEMA_MEASURES: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_MDSCHEMA_MEMBERS: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_MDSCHEMA_PROPERTIES: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CRESTRICTIONS_MDSCHEMA_SETS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CSTORAGEPROPERTY: u32 = 23u32;
pub const CSearchLanguageSupport: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1785252992, data2: 17207, data3: 19900, data4: [189, 39, 251, 251, 16, 83, 130, 11] };
pub const CSearchManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2097769567, data2: 44040, data3: 20255, data4: [190, 183, 92, 34, 197, 23, 206, 57] };
pub const CSearchRoot: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 813067218, data2: 59932, data3: 20264, data4: [191, 39, 11, 68, 226, 246, 141, 183] };
pub const CSearchScopeRule: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3862816592, data2: 15319, data3: 19429, data4: [156, 132, 107, 66, 129, 152, 140, 68] };
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type CatalogPausedReason = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CATALOG_PAUSED_REASON_NONE: CatalogPausedReason = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CATALOG_PAUSED_REASON_HIGH_IO: CatalogPausedReason = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CATALOG_PAUSED_REASON_HIGH_CPU: CatalogPausedReason = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CATALOG_PAUSED_REASON_HIGH_NTF_RATE: CatalogPausedReason = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CATALOG_PAUSED_REASON_LOW_BATTERY: CatalogPausedReason = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CATALOG_PAUSED_REASON_LOW_MEMORY: CatalogPausedReason = 5i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CATALOG_PAUSED_REASON_LOW_DISK: CatalogPausedReason = 6i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CATALOG_PAUSED_REASON_DELAYED_RECOVERY: CatalogPausedReason = 7i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CATALOG_PAUSED_REASON_USER_ACTIVE: CatalogPausedReason = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CATALOG_PAUSED_REASON_EXTERNAL: CatalogPausedReason = 9i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CATALOG_PAUSED_REASON_UPGRADING: CatalogPausedReason = 10i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type CatalogStatus = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CATALOG_STATUS_IDLE: CatalogStatus = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CATALOG_STATUS_PAUSED: CatalogStatus = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CATALOG_STATUS_RECOVERING: CatalogStatus = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CATALOG_STATUS_FULL_CRAWL: CatalogStatus = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CATALOG_STATUS_INCREMENTAL_CRAWL: CatalogStatus = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CATALOG_STATUS_PROCESSING_NOTIFICATIONS: CatalogStatus = 5i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const CATALOG_STATUS_SHUTTING_DOWN: CatalogStatus = 6i32;
pub const CompoundCondition: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 292523283, data2: 4126, data3: 20389, data4: [132, 212, 255, 130, 121, 56, 25, 53] };
pub const ConditionFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3762193840, data2: 31715, data3: 16384, data4: [186, 152, 108, 19, 222, 159, 164, 134] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub struct DATE_STRUCT {
    pub year: i16,
    pub month: u16,
    pub day: u16,
}
impl ::core::marker::Copy for DATE_STRUCT {}
impl ::core::clone::Clone for DATE_STRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBACCESSORFLAGSENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBACCESSOR_INVALID: DBACCESSORFLAGSENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBACCESSOR_PASSBYREF: DBACCESSORFLAGSENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBACCESSOR_ROWDATA: DBACCESSORFLAGSENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBACCESSOR_PARAMETERDATA: DBACCESSORFLAGSENUM = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBACCESSOR_OPTIMIZED: DBACCESSORFLAGSENUM = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBACCESSOR_INHERITED: DBACCESSORFLAGSENUM = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBASYNCHOPENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBASYNCHOP_OPEN: DBASYNCHOPENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBASYNCHPHASEENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBASYNCHPHASE_INITIALIZATION: DBASYNCHPHASEENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBASYNCHPHASE_POPULATION: DBASYNCHPHASEENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBASYNCHPHASE_COMPLETE: DBASYNCHPHASEENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBASYNCHPHASE_CANCELED: DBASYNCHPHASEENUM = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct DBBINDEXT {
    pub pExtension: *mut u8,
    pub ulExtension: usize,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for DBBINDEXT {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for DBBINDEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
#[cfg(target_arch = "x86")]
pub struct DBBINDEXT {
    pub pExtension: *mut u8,
    pub ulExtension: usize,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for DBBINDEXT {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for DBBINDEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBBINDFLAGENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBBINDFLAG_HTML: DBBINDFLAGENUM = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_System_Com\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Com")]
pub struct DBBINDING {
    pub iOrdinal: usize,
    pub obValue: usize,
    pub obLength: usize,
    pub obStatus: usize,
    pub pTypeInfo: *mut *mut *mut *mut super::Com::ITypeInfo,
    pub pObject: *mut DBOBJECT,
    pub pBindExt: *mut DBBINDEXT,
    pub dwPart: u32,
    pub dwMemOwner: u32,
    pub eParamIO: u32,
    pub cbMaxLen: usize,
    pub dwFlags: u32,
    pub wType: u16,
    pub bPrecision: u8,
    pub bScale: u8,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for DBBINDING {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for DBBINDING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_System_Com\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Com")]
pub struct DBBINDING {
    pub iOrdinal: usize,
    pub obValue: usize,
    pub obLength: usize,
    pub obStatus: usize,
    pub pTypeInfo: *mut *mut *mut *mut super::Com::ITypeInfo,
    pub pObject: *mut DBOBJECT,
    pub pBindExt: *mut DBBINDEXT,
    pub dwPart: u32,
    pub dwMemOwner: u32,
    pub eParamIO: u32,
    pub cbMaxLen: usize,
    pub dwFlags: u32,
    pub wType: u16,
    pub bPrecision: u8,
    pub bScale: u8,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for DBBINDING {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for DBBINDING {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBBINDSTATUSENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBBINDSTATUS_OK: DBBINDSTATUSENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBBINDSTATUS_BADORDINAL: DBBINDSTATUSENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBBINDSTATUS_UNSUPPORTEDCONVERSION: DBBINDSTATUSENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBBINDSTATUS_BADBINDINFO: DBBINDSTATUSENUM = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBBINDSTATUS_BADSTORAGEFLAGS: DBBINDSTATUSENUM = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBBINDSTATUS_NOINTERFACE: DBBINDSTATUSENUM = 5i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBBINDSTATUS_MULTIPLESTORAGE: DBBINDSTATUSENUM = 6i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBBINDURLFLAGENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBBINDURLFLAG_READ: DBBINDURLFLAGENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBBINDURLFLAG_WRITE: DBBINDURLFLAGENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBBINDURLFLAG_READWRITE: DBBINDURLFLAGENUM = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBBINDURLFLAG_SHARE_DENY_READ: DBBINDURLFLAGENUM = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBBINDURLFLAG_SHARE_DENY_WRITE: DBBINDURLFLAGENUM = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBBINDURLFLAG_SHARE_EXCLUSIVE: DBBINDURLFLAGENUM = 12i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBBINDURLFLAG_SHARE_DENY_NONE: DBBINDURLFLAGENUM = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBBINDURLFLAG_ASYNCHRONOUS: DBBINDURLFLAGENUM = 4096i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBBINDURLFLAG_COLLECTION: DBBINDURLFLAGENUM = 8192i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBBINDURLFLAG_DELAYFETCHSTREAM: DBBINDURLFLAGENUM = 16384i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBBINDURLFLAG_DELAYFETCHCOLUMNS: DBBINDURLFLAGENUM = 32768i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBBINDURLFLAG_RECURSIVE: DBBINDURLFLAGENUM = 4194304i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBBINDURLFLAG_OUTPUT: DBBINDURLFLAGENUM = 8388608i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBBINDURLFLAG_WAITFORINIT: DBBINDURLFLAGENUM = 16777216i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBBINDURLFLAG_OPENIFEXISTS: DBBINDURLFLAGENUM = 33554432i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBBINDURLFLAG_OVERWRITE: DBBINDURLFLAGENUM = 67108864i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBBINDURLFLAG_ISSTRUCTUREDDOCUMENT: DBBINDURLFLAGENUM = 134217728i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBBINDURLSTATUSENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBBINDURLSTATUS_S_OK: DBBINDURLSTATUSENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBBINDURLSTATUS_S_DENYNOTSUPPORTED: DBBINDURLSTATUSENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBBINDURLSTATUS_S_DENYTYPENOTSUPPORTED: DBBINDURLSTATUSENUM = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBBINDURLSTATUS_S_REDIRECTED: DBBINDURLSTATUSENUM = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBBOOKMARK = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBBMK_INVALID: DBBOOKMARK = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBBMK_FIRST: DBBOOKMARK = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBBMK_LAST: DBBOOKMARK = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Storage_IndexServer\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Storage_IndexServer")]
pub struct DBCOLUMNACCESS {
    pub pData: *mut ::core::ffi::c_void,
    pub columnid: super::super::Storage::IndexServer::DBID,
    pub cbDataLen: usize,
    pub dwStatus: u32,
    pub cbMaxLen: usize,
    pub dwReserved: usize,
    pub wType: u16,
    pub bPrecision: u8,
    pub bScale: u8,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ::core::marker::Copy for DBCOLUMNACCESS {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ::core::clone::Clone for DBCOLUMNACCESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Storage_IndexServer\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Storage_IndexServer")]
pub struct DBCOLUMNACCESS {
    pub pData: *mut ::core::ffi::c_void,
    pub columnid: super::super::Storage::IndexServer::DBID,
    pub cbDataLen: usize,
    pub dwStatus: u32,
    pub cbMaxLen: usize,
    pub dwReserved: usize,
    pub wType: u16,
    pub bPrecision: u8,
    pub bScale: u8,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ::core::marker::Copy for DBCOLUMNACCESS {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ::core::clone::Clone for DBCOLUMNACCESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct DBCOLUMNDESC {
    pub pwszTypeName: ::windows_sys::core::PWSTR,
    pub pTypeInfo: *mut *mut *mut *mut super::Com::ITypeInfo,
    pub rgPropertySets: *mut DBPROPSET,
    pub pclsid: *mut ::windows_sys::core::GUID,
    pub cPropertySets: u32,
    pub ulColumnSize: usize,
    pub dbcid: super::super::Storage::IndexServer::DBID,
    pub wType: u16,
    pub bPrecision: u8,
    pub bScale: u8,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for DBCOLUMNDESC {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for DBCOLUMNDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct DBCOLUMNDESC {
    pub pwszTypeName: ::windows_sys::core::PWSTR,
    pub pTypeInfo: *mut *mut *mut *mut super::Com::ITypeInfo,
    pub rgPropertySets: *mut DBPROPSET,
    pub pclsid: *mut ::windows_sys::core::GUID,
    pub cPropertySets: u32,
    pub ulColumnSize: usize,
    pub dbcid: super::super::Storage::IndexServer::DBID,
    pub wType: u16,
    pub bPrecision: u8,
    pub bScale: u8,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for DBCOLUMNDESC {}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for DBCOLUMNDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBCOLUMNDESCFLAGSENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOLUMNDESCFLAGS_TYPENAME: DBCOLUMNDESCFLAGSENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOLUMNDESCFLAGS_ITYPEINFO: DBCOLUMNDESCFLAGSENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOLUMNDESCFLAGS_PROPERTIES: DBCOLUMNDESCFLAGSENUM = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOLUMNDESCFLAGS_CLSID: DBCOLUMNDESCFLAGSENUM = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOLUMNDESCFLAGS_COLSIZE: DBCOLUMNDESCFLAGSENUM = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOLUMNDESCFLAGS_DBCID: DBCOLUMNDESCFLAGSENUM = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOLUMNDESCFLAGS_WTYPE: DBCOLUMNDESCFLAGSENUM = 64i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOLUMNDESCFLAGS_PRECISION: DBCOLUMNDESCFLAGSENUM = 128i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOLUMNDESCFLAGS_SCALE: DBCOLUMNDESCFLAGSENUM = 256i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBCOLUMNFLAGS15ENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOLUMNFLAGS_ISCHAPTER: DBCOLUMNFLAGS15ENUM = 8192i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBCOLUMNFLAGSENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOLUMNFLAGS_ISBOOKMARK: DBCOLUMNFLAGSENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOLUMNFLAGS_MAYDEFER: DBCOLUMNFLAGSENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOLUMNFLAGS_WRITE: DBCOLUMNFLAGSENUM = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOLUMNFLAGS_WRITEUNKNOWN: DBCOLUMNFLAGSENUM = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOLUMNFLAGS_ISFIXEDLENGTH: DBCOLUMNFLAGSENUM = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOLUMNFLAGS_ISNULLABLE: DBCOLUMNFLAGSENUM = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOLUMNFLAGS_MAYBENULL: DBCOLUMNFLAGSENUM = 64i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOLUMNFLAGS_ISLONG: DBCOLUMNFLAGSENUM = 128i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOLUMNFLAGS_ISROWID: DBCOLUMNFLAGSENUM = 256i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOLUMNFLAGS_ISROWVER: DBCOLUMNFLAGSENUM = 512i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOLUMNFLAGS_CACHEDEFERRED: DBCOLUMNFLAGSENUM = 4096i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBCOLUMNFLAGSENUM20 = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOLUMNFLAGS_SCALEISNEGATIVE: DBCOLUMNFLAGSENUM20 = 16384i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOLUMNFLAGS_RESERVED: DBCOLUMNFLAGSENUM20 = 32768i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBCOLUMNFLAGSENUM21 = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOLUMNFLAGS_ISROWURL: DBCOLUMNFLAGSENUM21 = 65536i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOLUMNFLAGS_ISDEFAULTSTREAM: DBCOLUMNFLAGSENUM21 = 131072i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOLUMNFLAGS_ISCOLLECTION: DBCOLUMNFLAGSENUM21 = 262144i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBCOLUMNFLAGSENUM26 = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOLUMNFLAGS_ISSTREAM: DBCOLUMNFLAGSENUM26 = 524288i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOLUMNFLAGS_ISROWSET: DBCOLUMNFLAGSENUM26 = 1048576i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOLUMNFLAGS_ISROW: DBCOLUMNFLAGSENUM26 = 2097152i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOLUMNFLAGS_ROWSPECIFICCOLUMN: DBCOLUMNFLAGSENUM26 = 4194304i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
pub struct DBCOLUMNINFO {
    pub pwszName: ::windows_sys::core::PWSTR,
    pub pTypeInfo: *mut *mut *mut *mut super::Com::ITypeInfo,
    pub iOrdinal: usize,
    pub dwFlags: u32,
    pub ulColumnSize: usize,
    pub wType: u16,
    pub bPrecision: u8,
    pub bScale: u8,
    pub columnid: super::super::Storage::IndexServer::DBID,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for DBCOLUMNINFO {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for DBCOLUMNINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
pub struct DBCOLUMNINFO {
    pub pwszName: ::windows_sys::core::PWSTR,
    pub pTypeInfo: *mut *mut *mut *mut super::Com::ITypeInfo,
    pub iOrdinal: usize,
    pub dwFlags: u32,
    pub ulColumnSize: usize,
    pub wType: u16,
    pub bPrecision: u8,
    pub bScale: u8,
    pub columnid: super::super::Storage::IndexServer::DBID,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for DBCOLUMNINFO {}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for DBCOLUMNINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBCOMMANDPERSISTFLAGENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOMMANDPERSISTFLAG_NOSAVE: DBCOMMANDPERSISTFLAGENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBCOMMANDPERSISTFLAGENUM21 = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOMMANDPERSISTFLAG_DEFAULT: DBCOMMANDPERSISTFLAGENUM21 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOMMANDPERSISTFLAG_PERSISTVIEW: DBCOMMANDPERSISTFLAGENUM21 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOMMANDPERSISTFLAG_PERSISTPROCEDURE: DBCOMMANDPERSISTFLAGENUM21 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBCOMPAREENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOMPARE_LT: DBCOMPAREENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOMPARE_EQ: DBCOMPAREENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOMPARE_GT: DBCOMPAREENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOMPARE_NE: DBCOMPAREENUM = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOMPARE_NOTCOMPARABLE: DBCOMPAREENUM = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBCOMPAREOPSENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOMPAREOPS_LT: DBCOMPAREOPSENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOMPAREOPS_LE: DBCOMPAREOPSENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOMPAREOPS_EQ: DBCOMPAREOPSENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOMPAREOPS_GE: DBCOMPAREOPSENUM = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOMPAREOPS_GT: DBCOMPAREOPSENUM = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOMPAREOPS_BEGINSWITH: DBCOMPAREOPSENUM = 5i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOMPAREOPS_CONTAINS: DBCOMPAREOPSENUM = 6i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOMPAREOPS_NE: DBCOMPAREOPSENUM = 7i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOMPAREOPS_IGNORE: DBCOMPAREOPSENUM = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOMPAREOPS_CASESENSITIVE: DBCOMPAREOPSENUM = 4096i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOMPAREOPS_CASEINSENSITIVE: DBCOMPAREOPSENUM = 8192i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBCOMPAREOPSENUM20 = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOMPAREOPS_NOTBEGINSWITH: DBCOMPAREOPSENUM20 = 9i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOMPAREOPS_NOTCONTAINS: DBCOMPAREOPSENUM20 = 10i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOMPUTEMODE_COMPUTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOMPUTEMODE_DYNAMIC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOMPUTEMODE_NOTCOMPUTED: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct DBCONSTRAINTDESC {
    pub pConstraintID: *mut super::super::Storage::IndexServer::DBID,
    pub ConstraintType: u32,
    pub cColumns: usize,
    pub rgColumnList: *mut super::super::Storage::IndexServer::DBID,
    pub pReferencedTableID: *mut super::super::Storage::IndexServer::DBID,
    pub cForeignKeyColumns: usize,
    pub rgForeignKeyColumnList: *mut super::super::Storage::IndexServer::DBID,
    pub pwszConstraintText: ::windows_sys::core::PWSTR,
    pub UpdateRule: u32,
    pub DeleteRule: u32,
    pub MatchType: u32,
    pub Deferrability: u32,
    pub cReserved: usize,
    pub rgReserved: *mut DBPROPSET,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for DBCONSTRAINTDESC {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for DBCONSTRAINTDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct DBCONSTRAINTDESC {
    pub pConstraintID: *mut super::super::Storage::IndexServer::DBID,
    pub ConstraintType: u32,
    pub cColumns: usize,
    pub rgColumnList: *mut super::super::Storage::IndexServer::DBID,
    pub pReferencedTableID: *mut super::super::Storage::IndexServer::DBID,
    pub cForeignKeyColumns: usize,
    pub rgForeignKeyColumnList: *mut super::super::Storage::IndexServer::DBID,
    pub pwszConstraintText: ::windows_sys::core::PWSTR,
    pub UpdateRule: u32,
    pub DeleteRule: u32,
    pub MatchType: u32,
    pub Deferrability: u32,
    pub cReserved: usize,
    pub rgReserved: *mut DBPROPSET,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for DBCONSTRAINTDESC {}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for DBCONSTRAINTDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBCONSTRAINTTYPEENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCONSTRAINTTYPE_UNIQUE: DBCONSTRAINTTYPEENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCONSTRAINTTYPE_FOREIGNKEY: DBCONSTRAINTTYPEENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCONSTRAINTTYPE_PRIMARYKEY: DBCONSTRAINTTYPEENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCONSTRAINTTYPE_CHECK: DBCONSTRAINTTYPEENUM = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBCONVERTFLAGSENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCONVERTFLAGS_COLUMN: DBCONVERTFLAGSENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCONVERTFLAGS_PARAMETER: DBCONVERTFLAGSENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBCONVERTFLAGSENUM20 = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCONVERTFLAGS_ISLONG: DBCONVERTFLAGSENUM20 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCONVERTFLAGS_ISFIXEDLENGTH: DBCONVERTFLAGSENUM20 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCONVERTFLAGS_FROMVARIANT: DBCONVERTFLAGSENUM20 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBCOPYFLAGSENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOPY_ASYNC: DBCOPYFLAGSENUM = 256i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOPY_REPLACE_EXISTING: DBCOPYFLAGSENUM = 512i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOPY_ALLOW_EMULATION: DBCOPYFLAGSENUM = 1024i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOPY_NON_RECURSIVE: DBCOPYFLAGSENUM = 2048i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBCOPY_ATOMIC: DBCOPYFLAGSENUM = 4096i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct DBCOST {
    pub eKind: u32,
    pub dwUnits: u32,
    pub lValue: i32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for DBCOST {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for DBCOST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
#[cfg(target_arch = "x86")]
pub struct DBCOST {
    pub eKind: u32,
    pub dwUnits: u32,
    pub lValue: i32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for DBCOST {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for DBCOST {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBCOSTUNITENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBUNIT_INVALID: DBCOSTUNITENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBUNIT_WEIGHT: DBCOSTUNITENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBUNIT_PERCENT: DBCOSTUNITENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBUNIT_MAXIMUM: DBCOSTUNITENUM = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBUNIT_MINIMUM: DBCOSTUNITENUM = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBUNIT_MICRO_SECOND: DBCOSTUNITENUM = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBUNIT_MILLI_SECOND: DBCOSTUNITENUM = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBUNIT_SECOND: DBCOSTUNITENUM = 64i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBUNIT_MINUTE: DBCOSTUNITENUM = 128i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBUNIT_HOUR: DBCOSTUNITENUM = 256i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBUNIT_BYTE: DBCOSTUNITENUM = 512i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBUNIT_KILO_BYTE: DBCOSTUNITENUM = 1024i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBUNIT_MEGA_BYTE: DBCOSTUNITENUM = 2048i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBUNIT_GIGA_BYTE: DBCOSTUNITENUM = 4096i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBUNIT_NUM_MSGS: DBCOSTUNITENUM = 8192i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBUNIT_NUM_LOCKS: DBCOSTUNITENUM = 16384i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBUNIT_NUM_ROWS: DBCOSTUNITENUM = 32768i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBUNIT_OTHER: DBCOSTUNITENUM = 65536i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBDATACONVERTENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBDATACONVERT_DEFAULT: DBDATACONVERTENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBDATACONVERT_SETDATABEHAVIOR: DBDATACONVERTENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBDATACONVERT_LENGTHFROMNTS: DBDATACONVERTENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBDATACONVERT_DSTISFIXEDLENGTH: DBDATACONVERTENUM = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBDATACONVERT_DECIMALSCALE: DBDATACONVERTENUM = 8i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub struct DBDATE {
    pub year: i16,
    pub month: u16,
    pub day: u16,
}
impl ::core::marker::Copy for DBDATE {}
impl ::core::clone::Clone for DBDATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBDEFERRABILITYENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBDEFERRABILITY_DEFERRED: DBDEFERRABILITYENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBDEFERRABILITY_DEFERRABLE: DBDEFERRABILITYENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBDELETEFLAGSENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBDELETE_ASYNC: DBDELETEFLAGSENUM = 256i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBDELETE_ATOMIC: DBDELETEFLAGSENUM = 4096i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBEVENTPHASEENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBEVENTPHASE_OKTODO: DBEVENTPHASEENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBEVENTPHASE_ABOUTTODO: DBEVENTPHASEENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBEVENTPHASE_SYNCHAFTER: DBEVENTPHASEENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBEVENTPHASE_FAILEDTODO: DBEVENTPHASEENUM = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBEVENTPHASE_DIDEVENT: DBEVENTPHASEENUM = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBEXECLIMITSENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBEXECLIMITS_ABORT: DBEXECLIMITSENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBEXECLIMITS_STOP: DBEXECLIMITSENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBEXECLIMITS_SUSPEND: DBEXECLIMITSENUM = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct DBFAILUREINFO {
    pub hRow: usize,
    pub iColumn: usize,
    pub failure: ::windows_sys::core::HRESULT,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for DBFAILUREINFO {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for DBFAILUREINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
#[cfg(target_arch = "x86")]
pub struct DBFAILUREINFO {
    pub hRow: usize,
    pub iColumn: usize,
    pub failure: ::windows_sys::core::HRESULT,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for DBFAILUREINFO {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for DBFAILUREINFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DBGUID_MSSQLXML: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1565727922, data2: 59117, data3: 4562, data4: [178, 82, 0, 192, 79, 104, 27, 113] };
pub const DBGUID_XPATH: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3962192531, data2: 59544, data3: 4562, data4: [177, 183, 0, 192, 79, 104, 12, 86] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct DBIMPLICITSESSION {
    pub pUnkOuter: *mut *mut *mut *mut ::windows_sys::core::IUnknown,
    pub piid: *mut ::windows_sys::core::GUID,
    pub pSession: *mut *mut *mut *mut ::windows_sys::core::IUnknown,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for DBIMPLICITSESSION {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for DBIMPLICITSESSION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
#[cfg(target_arch = "x86")]
pub struct DBIMPLICITSESSION {
    pub pUnkOuter: *mut *mut *mut *mut ::windows_sys::core::IUnknown,
    pub piid: *mut ::windows_sys::core::GUID,
    pub pSession: *mut *mut *mut *mut ::windows_sys::core::IUnknown,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for DBIMPLICITSESSION {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for DBIMPLICITSESSION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Storage_IndexServer\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Storage_IndexServer")]
pub struct DBINDEXCOLUMNDESC {
    pub pColumnID: *mut super::super::Storage::IndexServer::DBID,
    pub eIndexColOrder: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ::core::marker::Copy for DBINDEXCOLUMNDESC {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ::core::clone::Clone for DBINDEXCOLUMNDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Storage_IndexServer\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Storage_IndexServer")]
pub struct DBINDEXCOLUMNDESC {
    pub pColumnID: *mut super::super::Storage::IndexServer::DBID,
    pub eIndexColOrder: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ::core::marker::Copy for DBINDEXCOLUMNDESC {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ::core::clone::Clone for DBINDEXCOLUMNDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBINDEX_COL_ORDERENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBINDEX_COL_ORDER_ASC: DBINDEX_COL_ORDERENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBINDEX_COL_ORDER_DESC: DBINDEX_COL_ORDERENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBLITERALENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBLITERAL_INVALID: DBLITERALENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBLITERAL_BINARY_LITERAL: DBLITERALENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBLITERAL_CATALOG_NAME: DBLITERALENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBLITERAL_CATALOG_SEPARATOR: DBLITERALENUM = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBLITERAL_CHAR_LITERAL: DBLITERALENUM = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBLITERAL_COLUMN_ALIAS: DBLITERALENUM = 5i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBLITERAL_COLUMN_NAME: DBLITERALENUM = 6i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBLITERAL_CORRELATION_NAME: DBLITERALENUM = 7i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBLITERAL_CURSOR_NAME: DBLITERALENUM = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBLITERAL_ESCAPE_PERCENT: DBLITERALENUM = 9i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBLITERAL_ESCAPE_UNDERSCORE: DBLITERALENUM = 10i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBLITERAL_INDEX_NAME: DBLITERALENUM = 11i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBLITERAL_LIKE_PERCENT: DBLITERALENUM = 12i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBLITERAL_LIKE_UNDERSCORE: DBLITERALENUM = 13i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBLITERAL_PROCEDURE_NAME: DBLITERALENUM = 14i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBLITERAL_QUOTE: DBLITERALENUM = 15i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBLITERAL_SCHEMA_NAME: DBLITERALENUM = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBLITERAL_TABLE_NAME: DBLITERALENUM = 17i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBLITERAL_TEXT_COMMAND: DBLITERALENUM = 18i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBLITERAL_USER_NAME: DBLITERALENUM = 19i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBLITERAL_VIEW_NAME: DBLITERALENUM = 20i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBLITERALENUM20 = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBLITERAL_CUBE_NAME: DBLITERALENUM20 = 21i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBLITERAL_DIMENSION_NAME: DBLITERALENUM20 = 22i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBLITERAL_HIERARCHY_NAME: DBLITERALENUM20 = 23i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBLITERAL_LEVEL_NAME: DBLITERALENUM20 = 24i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBLITERAL_MEMBER_NAME: DBLITERALENUM20 = 25i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBLITERAL_PROPERTY_NAME: DBLITERALENUM20 = 26i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBLITERAL_SCHEMA_SEPARATOR: DBLITERALENUM20 = 27i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBLITERAL_QUOTE_SUFFIX: DBLITERALENUM20 = 28i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBLITERALENUM21 = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBLITERAL_ESCAPE_PERCENT_SUFFIX: DBLITERALENUM21 = 29i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBLITERAL_ESCAPE_UNDERSCORE_SUFFIX: DBLITERALENUM21 = 30i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct DBLITERALINFO {
    pub pwszLiteralValue: ::windows_sys::core::PWSTR,
    pub pwszInvalidChars: ::windows_sys::core::PWSTR,
    pub pwszInvalidStartingChars: ::windows_sys::core::PWSTR,
    pub lt: u32,
    pub fSupported: super::super::Foundation::BOOL,
    pub cchMaxLen: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DBLITERALINFO {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DBLITERALINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct DBLITERALINFO {
    pub pwszLiteralValue: ::windows_sys::core::PWSTR,
    pub pwszInvalidChars: ::windows_sys::core::PWSTR,
    pub pwszInvalidStartingChars: ::windows_sys::core::PWSTR,
    pub lt: u32,
    pub fSupported: super::super::Foundation::BOOL,
    pub cchMaxLen: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DBLITERALINFO {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DBLITERALINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBMATCHTYPEENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBMATCHTYPE_FULL: DBMATCHTYPEENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBMATCHTYPE_NONE: DBMATCHTYPEENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBMATCHTYPE_PARTIAL: DBMATCHTYPEENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBMAXCHAR: u32 = 8001u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBMEMOWNERENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBMEMOWNER_CLIENTOWNED: DBMEMOWNERENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBMEMOWNER_PROVIDEROWNED: DBMEMOWNERENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBMOVEFLAGSENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBMOVE_REPLACE_EXISTING: DBMOVEFLAGSENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBMOVE_ASYNC: DBMOVEFLAGSENUM = 256i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBMOVE_DONT_UPDATE_LINKS: DBMOVEFLAGSENUM = 512i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBMOVE_ALLOW_EMULATION: DBMOVEFLAGSENUM = 1024i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBMOVE_ATOMIC: DBMOVEFLAGSENUM = 4096i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct DBOBJECT {
    pub dwFlags: u32,
    pub iid: ::windows_sys::core::GUID,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for DBOBJECT {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for DBOBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
#[cfg(target_arch = "x86")]
pub struct DBOBJECT {
    pub dwFlags: u32,
    pub iid: ::windows_sys::core::GUID,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for DBOBJECT {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for DBOBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct DBPARAMBINDINFO {
    pub pwszDataSourceType: ::windows_sys::core::PWSTR,
    pub pwszName: ::windows_sys::core::PWSTR,
    pub ulParamSize: usize,
    pub dwFlags: u32,
    pub bPrecision: u8,
    pub bScale: u8,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for DBPARAMBINDINFO {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for DBPARAMBINDINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
#[cfg(target_arch = "x86")]
pub struct DBPARAMBINDINFO {
    pub pwszDataSourceType: ::windows_sys::core::PWSTR,
    pub pwszName: ::windows_sys::core::PWSTR,
    pub ulParamSize: usize,
    pub dwFlags: u32,
    pub bPrecision: u8,
    pub bScale: u8,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for DBPARAMBINDINFO {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for DBPARAMBINDINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBPARAMFLAGSENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPARAMFLAGS_ISINPUT: DBPARAMFLAGSENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPARAMFLAGS_ISOUTPUT: DBPARAMFLAGSENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPARAMFLAGS_ISSIGNED: DBPARAMFLAGSENUM = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPARAMFLAGS_ISNULLABLE: DBPARAMFLAGSENUM = 64i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPARAMFLAGS_ISLONG: DBPARAMFLAGSENUM = 128i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBPARAMFLAGSENUM20 = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPARAMFLAGS_SCALEISNEGATIVE: DBPARAMFLAGSENUM20 = 256i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_System_Com\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Com")]
pub struct DBPARAMINFO {
    pub dwFlags: u32,
    pub iOrdinal: usize,
    pub pwszName: ::windows_sys::core::PWSTR,
    pub pTypeInfo: *mut *mut *mut *mut super::Com::ITypeInfo,
    pub ulParamSize: usize,
    pub wType: u16,
    pub bPrecision: u8,
    pub bScale: u8,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for DBPARAMINFO {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for DBPARAMINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_System_Com\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Com")]
pub struct DBPARAMINFO {
    pub dwFlags: u32,
    pub iOrdinal: usize,
    pub pwszName: ::windows_sys::core::PWSTR,
    pub pTypeInfo: *mut *mut *mut *mut super::Com::ITypeInfo,
    pub ulParamSize: usize,
    pub wType: u16,
    pub bPrecision: u8,
    pub bScale: u8,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for DBPARAMINFO {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for DBPARAMINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBPARAMIOENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPARAMIO_NOTPARAM: DBPARAMIOENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPARAMIO_INPUT: DBPARAMIOENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPARAMIO_OUTPUT: DBPARAMIOENUM = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct DBPARAMS {
    pub pData: *mut ::core::ffi::c_void,
    pub cParamSets: usize,
    pub hAccessor: usize,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for DBPARAMS {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for DBPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
#[cfg(target_arch = "x86")]
pub struct DBPARAMS {
    pub pData: *mut ::core::ffi::c_void,
    pub cParamSets: usize,
    pub hAccessor: usize,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for DBPARAMS {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for DBPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPARAMTYPE_INPUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPARAMTYPE_INPUTOUTPUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPARAMTYPE_OUTPUT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPARAMTYPE_RETURNVALUE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBPARTENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPART_INVALID: DBPARTENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPART_VALUE: DBPARTENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPART_LENGTH: DBPARTENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPART_STATUS: DBPARTENUM = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBPENDINGSTATUSENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPENDINGSTATUS_NEW: DBPENDINGSTATUSENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPENDINGSTATUS_CHANGED: DBPENDINGSTATUSENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPENDINGSTATUS_DELETED: DBPENDINGSTATUSENUM = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPENDINGSTATUS_UNCHANGED: DBPENDINGSTATUSENUM = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPENDINGSTATUS_INVALIDROW: DBPENDINGSTATUSENUM = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBPOSITIONFLAGSENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPOSITION_OK: DBPOSITIONFLAGSENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPOSITION_NOROW: DBPOSITIONFLAGSENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPOSITION_BOF: DBPOSITIONFLAGSENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPOSITION_EOF: DBPOSITIONFLAGSENUM = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBPROMPTOPTIONSENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROMPTOPTIONS_NONE: DBPROMPTOPTIONSENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROMPTOPTIONS_WIZARDSHEET: DBPROMPTOPTIONSENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROMPTOPTIONS_PROPERTYSHEET: DBPROMPTOPTIONSENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROMPTOPTIONS_BROWSEONLY: DBPROMPTOPTIONSENUM = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROMPTOPTIONS_DISABLE_PROVIDER_SELECTION: DBPROMPTOPTIONSENUM = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROMPTOPTIONS_DISABLESAVEPASSWORD: DBPROMPTOPTIONSENUM = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROMPT_COMPLETE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROMPT_COMPLETEREQUIRED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROMPT_NOPROMPT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROMPT_PROMPT: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct DBPROP {
    pub dwPropertyID: u32,
    pub dwOptions: u32,
    pub dwStatus: u32,
    pub colid: super::super::Storage::IndexServer::DBID,
    pub vValue: super::Com::VARIANT,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for DBPROP {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for DBPROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct DBPROP {
    pub dwPropertyID: u32,
    pub dwOptions: u32,
    pub dwStatus: u32,
    pub colid: super::super::Storage::IndexServer::DBID,
    pub vValue: super::Com::VARIANT,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for DBPROP {}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for DBPROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBPROPENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_ABORTPRESERVE: DBPROPENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_ACTIVESESSIONS: DBPROPENUM = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_APPENDONLY: DBPROPENUM = 187i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_ASYNCTXNABORT: DBPROPENUM = 168i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_ASYNCTXNCOMMIT: DBPROPENUM = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_AUTH_CACHE_AUTHINFO: DBPROPENUM = 5i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_AUTH_ENCRYPT_PASSWORD: DBPROPENUM = 6i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_AUTH_INTEGRATED: DBPROPENUM = 7i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_AUTH_MASK_PASSWORD: DBPROPENUM = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_AUTH_PASSWORD: DBPROPENUM = 9i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_AUTH_PERSIST_ENCRYPTED: DBPROPENUM = 10i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_AUTH_PERSIST_SENSITIVE_AUTHINFO: DBPROPENUM = 11i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_AUTH_USERID: DBPROPENUM = 12i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_BLOCKINGSTORAGEOBJECTS: DBPROPENUM = 13i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_BOOKMARKS: DBPROPENUM = 14i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_BOOKMARKSKIPPED: DBPROPENUM = 15i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_BOOKMARKTYPE: DBPROPENUM = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_BYREFACCESSORS: DBPROPENUM = 120i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_CACHEDEFERRED: DBPROPENUM = 17i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_CANFETCHBACKWARDS: DBPROPENUM = 18i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_CANHOLDROWS: DBPROPENUM = 19i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_CANSCROLLBACKWARDS: DBPROPENUM = 21i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_CATALOGLOCATION: DBPROPENUM = 22i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_CATALOGTERM: DBPROPENUM = 23i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_CATALOGUSAGE: DBPROPENUM = 24i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_CHANGEINSERTEDROWS: DBPROPENUM = 188i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_COL_AUTOINCREMENT: DBPROPENUM = 26i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_COL_DEFAULT: DBPROPENUM = 27i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_COL_DESCRIPTION: DBPROPENUM = 28i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_COL_FIXEDLENGTH: DBPROPENUM = 167i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_COL_NULLABLE: DBPROPENUM = 29i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_COL_PRIMARYKEY: DBPROPENUM = 30i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_COL_UNIQUE: DBPROPENUM = 31i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_COLUMNDEFINITION: DBPROPENUM = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_COLUMNRESTRICT: DBPROPENUM = 33i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_COMMANDTIMEOUT: DBPROPENUM = 34i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_COMMITPRESERVE: DBPROPENUM = 35i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_CONCATNULLBEHAVIOR: DBPROPENUM = 36i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_CURRENTCATALOG: DBPROPENUM = 37i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_DATASOURCENAME: DBPROPENUM = 38i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_DATASOURCEREADONLY: DBPROPENUM = 39i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_DBMSNAME: DBPROPENUM = 40i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_DBMSVER: DBPROPENUM = 41i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_DEFERRED: DBPROPENUM = 42i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_DELAYSTORAGEOBJECTS: DBPROPENUM = 43i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_DSOTHREADMODEL: DBPROPENUM = 169i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_GROUPBY: DBPROPENUM = 44i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_HETEROGENEOUSTABLES: DBPROPENUM = 45i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IAccessor: DBPROPENUM = 121i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IColumnsInfo: DBPROPENUM = 122i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IColumnsRowset: DBPROPENUM = 123i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IConnectionPointContainer: DBPROPENUM = 124i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IConvertType: DBPROPENUM = 194i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IRowset: DBPROPENUM = 126i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IRowsetChange: DBPROPENUM = 127i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IRowsetIdentity: DBPROPENUM = 128i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IRowsetIndex: DBPROPENUM = 159i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IRowsetInfo: DBPROPENUM = 129i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IRowsetLocate: DBPROPENUM = 130i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IRowsetResynch: DBPROPENUM = 132i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IRowsetScroll: DBPROPENUM = 133i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IRowsetUpdate: DBPROPENUM = 134i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_ISupportErrorInfo: DBPROPENUM = 135i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_ILockBytes: DBPROPENUM = 136i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_ISequentialStream: DBPROPENUM = 137i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IStorage: DBPROPENUM = 138i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IStream: DBPROPENUM = 139i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IDENTIFIERCASE: DBPROPENUM = 46i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IMMOBILEROWS: DBPROPENUM = 47i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_INDEX_AUTOUPDATE: DBPROPENUM = 48i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_INDEX_CLUSTERED: DBPROPENUM = 49i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_INDEX_FILLFACTOR: DBPROPENUM = 50i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_INDEX_INITIALSIZE: DBPROPENUM = 51i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_INDEX_NULLCOLLATION: DBPROPENUM = 52i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_INDEX_NULLS: DBPROPENUM = 53i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_INDEX_PRIMARYKEY: DBPROPENUM = 54i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_INDEX_SORTBOOKMARKS: DBPROPENUM = 55i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_INDEX_TEMPINDEX: DBPROPENUM = 163i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_INDEX_TYPE: DBPROPENUM = 56i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_INDEX_UNIQUE: DBPROPENUM = 57i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_INIT_DATASOURCE: DBPROPENUM = 59i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_INIT_HWND: DBPROPENUM = 60i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_INIT_IMPERSONATION_LEVEL: DBPROPENUM = 61i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_INIT_LCID: DBPROPENUM = 186i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_INIT_LOCATION: DBPROPENUM = 62i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_INIT_MODE: DBPROPENUM = 63i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_INIT_PROMPT: DBPROPENUM = 64i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_INIT_PROTECTION_LEVEL: DBPROPENUM = 65i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_INIT_PROVIDERSTRING: DBPROPENUM = 160i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_INIT_TIMEOUT: DBPROPENUM = 66i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_LITERALBOOKMARKS: DBPROPENUM = 67i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_LITERALIDENTITY: DBPROPENUM = 68i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_MAXINDEXSIZE: DBPROPENUM = 70i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_MAXOPENROWS: DBPROPENUM = 71i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_MAXPENDINGROWS: DBPROPENUM = 72i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_MAXROWS: DBPROPENUM = 73i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_MAXROWSIZE: DBPROPENUM = 74i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_MAXROWSIZEINCLUDESBLOB: DBPROPENUM = 75i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_MAXTABLESINSELECT: DBPROPENUM = 76i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_MAYWRITECOLUMN: DBPROPENUM = 77i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_MEMORYUSAGE: DBPROPENUM = 78i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_MULTIPLEPARAMSETS: DBPROPENUM = 191i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_MULTIPLERESULTS: DBPROPENUM = 196i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_MULTIPLESTORAGEOBJECTS: DBPROPENUM = 80i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_MULTITABLEUPDATE: DBPROPENUM = 81i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_NOTIFICATIONGRANULARITY: DBPROPENUM = 198i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_NOTIFICATIONPHASES: DBPROPENUM = 82i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_NOTIFYCOLUMNSET: DBPROPENUM = 171i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_NOTIFYROWDELETE: DBPROPENUM = 173i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_NOTIFYROWFIRSTCHANGE: DBPROPENUM = 174i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_NOTIFYROWINSERT: DBPROPENUM = 175i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_NOTIFYROWRESYNCH: DBPROPENUM = 177i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_NOTIFYROWSETCHANGED: DBPROPENUM = 211i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_NOTIFYROWSETRELEASE: DBPROPENUM = 178i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_NOTIFYROWSETFETCHPOSITIONCHANGE: DBPROPENUM = 179i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_NOTIFYROWUNDOCHANGE: DBPROPENUM = 180i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_NOTIFYROWUNDODELETE: DBPROPENUM = 181i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_NOTIFYROWUNDOINSERT: DBPROPENUM = 182i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_NOTIFYROWUPDATE: DBPROPENUM = 183i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_NULLCOLLATION: DBPROPENUM = 83i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_OLEOBJECTS: DBPROPENUM = 84i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_ORDERBYCOLUMNSINSELECT: DBPROPENUM = 85i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_ORDEREDBOOKMARKS: DBPROPENUM = 86i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_OTHERINSERT: DBPROPENUM = 87i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_OTHERUPDATEDELETE: DBPROPENUM = 88i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_OUTPUTPARAMETERAVAILABILITY: DBPROPENUM = 184i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_OWNINSERT: DBPROPENUM = 89i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_OWNUPDATEDELETE: DBPROPENUM = 90i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_PERSISTENTIDTYPE: DBPROPENUM = 185i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_PREPAREABORTBEHAVIOR: DBPROPENUM = 91i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_PREPARECOMMITBEHAVIOR: DBPROPENUM = 92i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_PROCEDURETERM: DBPROPENUM = 93i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_PROVIDERNAME: DBPROPENUM = 96i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_PROVIDEROLEDBVER: DBPROPENUM = 97i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_PROVIDERVER: DBPROPENUM = 98i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_QUICKRESTART: DBPROPENUM = 99i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_QUOTEDIDENTIFIERCASE: DBPROPENUM = 100i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_REENTRANTEVENTS: DBPROPENUM = 101i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_REMOVEDELETED: DBPROPENUM = 102i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_REPORTMULTIPLECHANGES: DBPROPENUM = 103i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_RETURNPENDINGINSERTS: DBPROPENUM = 189i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_ROWRESTRICT: DBPROPENUM = 104i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_ROWSETCONVERSIONSONCOMMAND: DBPROPENUM = 192i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_ROWTHREADMODEL: DBPROPENUM = 105i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_SCHEMATERM: DBPROPENUM = 106i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_SCHEMAUSAGE: DBPROPENUM = 107i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_SERVERCURSOR: DBPROPENUM = 108i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_SESS_AUTOCOMMITISOLEVELS: DBPROPENUM = 190i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_SQLSUPPORT: DBPROPENUM = 109i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_STRONGIDENTITY: DBPROPENUM = 119i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_STRUCTUREDSTORAGE: DBPROPENUM = 111i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_SUBQUERIES: DBPROPENUM = 112i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_SUPPORTEDTXNDDL: DBPROPENUM = 161i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_SUPPORTEDTXNISOLEVELS: DBPROPENUM = 113i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_SUPPORTEDTXNISORETAIN: DBPROPENUM = 114i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_TABLETERM: DBPROPENUM = 115i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_TBL_TEMPTABLE: DBPROPENUM = 140i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_TRANSACTEDOBJECT: DBPROPENUM = 116i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_UPDATABILITY: DBPROPENUM = 117i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_USERNAME: DBPROPENUM = 118i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBPROPENUM15 = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_FILTERCOMPAREOPS: DBPROPENUM15 = 209i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_FINDCOMPAREOPS: DBPROPENUM15 = 210i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IChapteredRowset: DBPROPENUM15 = 202i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IDBAsynchStatus: DBPROPENUM15 = 203i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IRowsetFind: DBPROPENUM15 = 204i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IRowsetView: DBPROPENUM15 = 212i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IViewChapter: DBPROPENUM15 = 213i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IViewFilter: DBPROPENUM15 = 214i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IViewRowset: DBPROPENUM15 = 215i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IViewSort: DBPROPENUM15 = 216i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_INIT_ASYNCH: DBPROPENUM15 = 200i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_MAXOPENCHAPTERS: DBPROPENUM15 = 199i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_MAXORSINFILTER: DBPROPENUM15 = 205i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_MAXSORTCOLUMNS: DBPROPENUM15 = 206i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_ROWSET_ASYNCH: DBPROPENUM15 = 201i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_SORTONINDEX: DBPROPENUM15 = 207i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBPROPENUM20 = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IMultipleResults: DBPROPENUM20 = 217i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_DATASOURCE_TYPE: DBPROPENUM20 = 251i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROP_AXES: DBPROPENUM20 = 252i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROP_FLATTENING_SUPPORT: DBPROPENUM20 = 253i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROP_MDX_JOINCUBES: DBPROPENUM20 = 254i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROP_NAMED_LEVELS: DBPROPENUM20 = 255i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROP_RANGEROWSET: DBPROPENUM20 = 256i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROP_MDX_SLICER: DBPROPENUM20 = 218i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROP_MDX_CUBEQUALIFICATION: DBPROPENUM20 = 219i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROP_MDX_OUTERREFERENCE: DBPROPENUM20 = 220i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROP_MDX_QUERYBYPROPERTY: DBPROPENUM20 = 221i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROP_MDX_CASESUPPORT: DBPROPENUM20 = 222i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROP_MDX_STRING_COMPOP: DBPROPENUM20 = 224i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROP_MDX_DESCFLAGS: DBPROPENUM20 = 225i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROP_MDX_SET_FUNCTIONS: DBPROPENUM20 = 226i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROP_MDX_MEMBER_FUNCTIONS: DBPROPENUM20 = 227i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROP_MDX_NUMERIC_FUNCTIONS: DBPROPENUM20 = 228i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROP_MDX_FORMULAS: DBPROPENUM20 = 229i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROP_AGGREGATECELL_UPDATE: DBPROPENUM20 = 230i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROP_MDX_AGGREGATECELL_UPDATE: DBPROPENUM20 = 230i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROP_MDX_OBJQUALIFICATION: DBPROPENUM20 = 261i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROP_MDX_NONMEASURE_EXPRESSIONS: DBPROPENUM20 = 262i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_ACCESSORDER: DBPROPENUM20 = 231i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_BOOKMARKINFO: DBPROPENUM20 = 232i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_INIT_CATALOG: DBPROPENUM20 = 233i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_ROW_BULKOPS: DBPROPENUM20 = 234i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_PROVIDERFRIENDLYNAME: DBPROPENUM20 = 235i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_LOCKMODE: DBPROPENUM20 = 236i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_MULTIPLECONNECTIONS: DBPROPENUM20 = 237i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_UNIQUEROWS: DBPROPENUM20 = 238i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_SERVERDATAONINSERT: DBPROPENUM20 = 239i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_STORAGEFLAGS: DBPROPENUM20 = 240i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_CONNECTIONSTATUS: DBPROPENUM20 = 244i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_ALTERCOLUMN: DBPROPENUM20 = 245i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_COLUMNLCID: DBPROPENUM20 = 246i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_RESETDATASOURCE: DBPROPENUM20 = 247i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_INIT_OLEDBSERVICES: DBPROPENUM20 = 248i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IRowsetRefresh: DBPROPENUM20 = 249i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_SERVERNAME: DBPROPENUM20 = 250i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IParentRowset: DBPROPENUM20 = 257i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_HIDDENCOLUMNS: DBPROPENUM20 = 258i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_PROVIDERMEMORY: DBPROPENUM20 = 259i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_CLIENTCURSOR: DBPROPENUM20 = 260i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBPROPENUM21 = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_TRUSTEE_USERNAME: DBPROPENUM21 = 241i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_TRUSTEE_AUTHENTICATION: DBPROPENUM21 = 242i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_TRUSTEE_NEWAUTHENTICATION: DBPROPENUM21 = 243i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IRow: DBPROPENUM21 = 263i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IRowChange: DBPROPENUM21 = 264i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IRowSchemaChange: DBPROPENUM21 = 265i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IGetRow: DBPROPENUM21 = 266i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IScopedOperations: DBPROPENUM21 = 267i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IBindResource: DBPROPENUM21 = 268i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_ICreateRow: DBPROPENUM21 = 269i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_INIT_BINDFLAGS: DBPROPENUM21 = 270i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_INIT_LOCKOWNER: DBPROPENUM21 = 271i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_GENERATEURL: DBPROPENUM21 = 273i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IDBBinderProperties: DBPROPENUM21 = 274i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IColumnsInfo2: DBPROPENUM21 = 275i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IRegisterProvider: DBPROPENUM21 = 276i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IGetSession: DBPROPENUM21 = 277i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IGetSourceRow: DBPROPENUM21 = 278i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IRowsetCurrentIndex: DBPROPENUM21 = 279i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_OPENROWSETSUPPORT: DBPROPENUM21 = 280i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_COL_ISLONG: DBPROPENUM21 = 281i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBPROPENUM25 = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_COL_SEED: DBPROPENUM25 = 282i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_COL_INCREMENT: DBPROPENUM25 = 283i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_INIT_GENERALTIMEOUT: DBPROPENUM25 = 284i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_COMSERVICES: DBPROPENUM25 = 285i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBPROPENUM25_DEPRECATED = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_ICommandCost: DBPROPENUM25_DEPRECATED = 141i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_ICommandTree: DBPROPENUM25_DEPRECATED = 142i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_ICommandValidate: DBPROPENUM25_DEPRECATED = 143i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IDBSchemaCommand: DBPROPENUM25_DEPRECATED = 144i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IProvideMoniker: DBPROPENUM25_DEPRECATED = 125i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IQuery: DBPROPENUM25_DEPRECATED = 146i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IReadData: DBPROPENUM25_DEPRECATED = 147i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IRowsetAsynch: DBPROPENUM25_DEPRECATED = 148i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IRowsetCopyRows: DBPROPENUM25_DEPRECATED = 149i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IRowsetKeys: DBPROPENUM25_DEPRECATED = 151i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IRowsetNewRowAfter: DBPROPENUM25_DEPRECATED = 152i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IRowsetNextRowset: DBPROPENUM25_DEPRECATED = 153i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IRowsetWatchAll: DBPROPENUM25_DEPRECATED = 155i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IRowsetWatchNotify: DBPROPENUM25_DEPRECATED = 156i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IRowsetWatchRegion: DBPROPENUM25_DEPRECATED = 157i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IRowsetWithParameters: DBPROPENUM25_DEPRECATED = 158i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBPROPENUM26 = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_OUTPUTSTREAM: DBPROPENUM26 = 286i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_OUTPUTENCODING: DBPROPENUM26 = 287i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_TABLESTATISTICS: DBPROPENUM26 = 288i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_SKIPROWCOUNTRESULTS: DBPROPENUM26 = 291i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_IRowsetBookmark: DBPROPENUM26 = 292i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROP_VISUALMODE: DBPROPENUM26 = 293i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBPROPFLAGSENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPFLAGS_NOTSUPPORTED: DBPROPFLAGSENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPFLAGS_COLUMN: DBPROPFLAGSENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPFLAGS_DATASOURCE: DBPROPFLAGSENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPFLAGS_DATASOURCECREATE: DBPROPFLAGSENUM = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPFLAGS_DATASOURCEINFO: DBPROPFLAGSENUM = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPFLAGS_DBINIT: DBPROPFLAGSENUM = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPFLAGS_INDEX: DBPROPFLAGSENUM = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPFLAGS_ROWSET: DBPROPFLAGSENUM = 64i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPFLAGS_TABLE: DBPROPFLAGSENUM = 128i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPFLAGS_COLUMNOK: DBPROPFLAGSENUM = 256i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPFLAGS_READ: DBPROPFLAGSENUM = 512i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPFLAGS_WRITE: DBPROPFLAGSENUM = 1024i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPFLAGS_REQUIRED: DBPROPFLAGSENUM = 2048i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPFLAGS_SESSION: DBPROPFLAGSENUM = 4096i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBPROPFLAGSENUM21 = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPFLAGS_TRUSTEE: DBPROPFLAGSENUM21 = 8192i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBPROPFLAGSENUM25 = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPFLAGS_VIEW: DBPROPFLAGSENUM25 = 16384i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBPROPFLAGSENUM26 = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPFLAGS_STREAM: DBPROPFLAGSENUM26 = 32768i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPFLAGS_PERSIST: u32 = 8192u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct DBPROPIDSET {
    pub rgPropertyIDs: *mut u32,
    pub cPropertyIDs: u32,
    pub guidPropertySet: ::windows_sys::core::GUID,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for DBPROPIDSET {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for DBPROPIDSET {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
#[cfg(target_arch = "x86")]
pub struct DBPROPIDSET {
    pub rgPropertyIDs: *mut u32,
    pub cPropertyIDs: u32,
    pub guidPropertySet: ::windows_sys::core::GUID,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for DBPROPIDSET {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for DBPROPIDSET {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct DBPROPINFO {
    pub pwszDescription: ::windows_sys::core::PWSTR,
    pub dwPropertyID: u32,
    pub dwFlags: u32,
    pub vtType: u16,
    pub vValues: super::Com::VARIANT,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for DBPROPINFO {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for DBPROPINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct DBPROPINFO {
    pub pwszDescription: ::windows_sys::core::PWSTR,
    pub dwPropertyID: u32,
    pub dwFlags: u32,
    pub vtType: u16,
    pub vValues: super::Com::VARIANT,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for DBPROPINFO {}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for DBPROPINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct DBPROPINFOSET {
    pub rgPropertyInfos: *mut DBPROPINFO,
    pub cPropertyInfos: u32,
    pub guidPropertySet: ::windows_sys::core::GUID,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for DBPROPINFOSET {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for DBPROPINFOSET {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct DBPROPINFOSET {
    pub rgPropertyInfos: *mut DBPROPINFO,
    pub cPropertyInfos: u32,
    pub guidPropertySet: ::windows_sys::core::GUID,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for DBPROPINFOSET {}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for DBPROPINFOSET {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBPROPOPTIONSENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPOPTIONS_REQUIRED: DBPROPOPTIONSENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPOPTIONS_SETIFCHEAP: DBPROPOPTIONSENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPOPTIONS_OPTIONAL: DBPROPOPTIONSENUM = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct DBPROPSET {
    pub rgProperties: *mut DBPROP,
    pub cProperties: u32,
    pub guidPropertySet: ::windows_sys::core::GUID,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for DBPROPSET {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for DBPROPSET {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct DBPROPSET {
    pub rgProperties: *mut DBPROP,
    pub cProperties: u32,
    pub guidPropertySet: ::windows_sys::core::GUID,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for DBPROPSET {}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for DBPROPSET {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DBPROPSET_MSDAORA8_ROWSET: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2131141493, data2: 56682, data3: 17371, data4: [180, 224, 31, 193, 33, 229, 230, 43] };
pub const DBPROPSET_MSDAORA_ROWSET: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3905703101, data2: 65023, data3: 4560, data4: [184, 101, 0, 160, 201, 8, 28, 29] };
pub const DBPROPSET_MSDSDBINIT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1439404456, data2: 23674, data3: 4561, data4: [173, 173, 0, 192, 79, 194, 152, 99] };
pub const DBPROPSET_MSDSSESSION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3992024374, data2: 44991, data3: 4561, data4: [136, 71, 0, 0, 248, 121, 249, 140] };
pub const DBPROPSET_PERSIST: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1299724704, data2: 23438, data3: 4561, data4: [166, 179, 0, 160, 201, 19, 140, 102] };
pub const DBPROPSET_PROVIDERCONNATTR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1232888036, data2: 28963, data3: 4559, data4: [177, 113, 0, 170, 0, 87, 89, 158] };
pub const DBPROPSET_PROVIDERDATASOURCEINFO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1232888032, data2: 28963, data3: 4559, data4: [177, 113, 0, 170, 0, 87, 89, 158] };
pub const DBPROPSET_PROVIDERDBINIT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1232888034, data2: 28963, data3: 4559, data4: [177, 113, 0, 170, 0, 87, 89, 158] };
pub const DBPROPSET_PROVIDERROWSET: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1232888033, data2: 28963, data3: 4559, data4: [177, 113, 0, 170, 0, 87, 89, 158] };
pub const DBPROPSET_PROVIDERSTMTATTR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1232888035, data2: 28963, data3: 4559, data4: [177, 113, 0, 170, 0, 87, 89, 158] };
pub const DBPROPSET_SQLSERVERCOLUMN: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 996408158, data2: 16315, data3: 4563, data4: [159, 41, 0, 192, 79, 142, 233, 220] };
pub const DBPROPSET_SQLSERVERDATASOURCE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 686796516, data2: 11564, data3: 4561, data4: [152, 7, 0, 192, 79, 194, 173, 152] };
pub const DBPROPSET_SQLSERVERDATASOURCEINFO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3742419860, data2: 13814, data3: 4562, data4: [156, 84, 0, 192, 79, 121, 113, 211] };
pub const DBPROPSET_SQLSERVERDBINIT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1559546384, data2: 61217, data3: 4560, data4: [151, 231, 0, 192, 79, 194, 173, 152] };
pub const DBPROPSET_SQLSERVERROWSET: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1559546385, data2: 61217, data3: 4560, data4: [151, 231, 0, 192, 79, 194, 173, 152] };
pub const DBPROPSET_SQLSERVERSESSION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 686796517, data2: 11564, data3: 4561, data4: [152, 7, 0, 192, 79, 194, 173, 152] };
pub const DBPROPSET_SQLSERVERSTREAM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2675556467, data2: 35437, data3: 19402, data4: [168, 168, 201, 183, 154, 155, 150, 45] };
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBPROPSTATUSENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPSTATUS_OK: DBPROPSTATUSENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPSTATUS_NOTSUPPORTED: DBPROPSTATUSENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPSTATUS_BADVALUE: DBPROPSTATUSENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPSTATUS_BADOPTION: DBPROPSTATUSENUM = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPSTATUS_BADCOLUMN: DBPROPSTATUSENUM = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPSTATUS_NOTALLSETTABLE: DBPROPSTATUSENUM = 5i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPSTATUS_NOTSETTABLE: DBPROPSTATUSENUM = 6i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPSTATUS_NOTSET: DBPROPSTATUSENUM = 7i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPSTATUS_CONFLICTING: DBPROPSTATUSENUM = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBPROPSTATUSENUM21 = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPSTATUS_NOTAVAILABLE: DBPROPSTATUSENUM21 = 9i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_AO_RANDOM: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_AO_SEQUENTIAL: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_AO_SEQUENTIALSTORAGEOBJECTS: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_ASYNCH_BACKGROUNDPOPULATION: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_ASYNCH_INITIALIZE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_ASYNCH_POPULATEONDEMAND: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_ASYNCH_PREPOPULATE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_ASYNCH_RANDOMPOPULATION: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_ASYNCH_SEQUENTIALPOPULATION: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_BD_INTRANSACTION: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_BD_REORGANIZATION: i32 = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_BD_ROWSET: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_BD_XTRANSACTION: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_BI_CROSSROWSET: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_BMK_KEY: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_BMK_NUMERIC: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_BO_NOINDEXUPDATE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_BO_NOLOG: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_BO_REFINTEGRITY: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_CB_DELETE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_CB_NON_NULL: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_CB_NULL: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_CB_PRESERVE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_CD_NOTNULL: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_CL_END: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_CL_START: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_CM_TRANSACTIONS: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_CO_BEGINSWITH: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_CO_CASEINSENSITIVE: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_CO_CASESENSITIVE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_CO_CONTAINS: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_CO_EQUALITY: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_CO_STRING: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_CS_COMMUNICATIONFAILURE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_CS_INITIALIZED: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_CS_UNINITIALIZED: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_CU_DML_STATEMENTS: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_CU_INDEX_DEFINITION: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_CU_PRIVILEGE_DEFINITION: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_CU_TABLE_DEFINITION: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_DF_INITIALLY_DEFERRED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_DF_INITIALLY_IMMEDIATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_DF_NOT_DEFERRABLE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_DST_DOCSOURCE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_DST_MDP: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_DST_TDP: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_DST_TDPANDMDP: i32 = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_FU_CATALOG: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_FU_COLUMN: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_FU_NOT_SUPPORTED: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_FU_TABLE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_GB_COLLATE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_GB_CONTAINS_SELECT: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_GB_EQUALS_SELECT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_GB_NOT_SUPPORTED: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_GB_NO_RELATION: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_GU_NOTSUPPORTED: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_GU_SUFFIX: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_HT_DIFFERENT_CATALOGS: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_HT_DIFFERENT_PROVIDERS: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_IC_LOWER: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_IC_MIXED: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_IC_SENSITIVE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_IC_UPPER: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_IN_ALLOWNULL: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_IN_DISALLOWNULL: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_IN_IGNOREANYNULL: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_IN_IGNORENULL: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_IT_BTREE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_IT_CONTENT: i32 = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_IT_HASH: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_IT_OTHER: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_LM_INTENT: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_LM_NONE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_LM_READ: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_LM_RITE: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_LM_SINGLEROW: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_MR_CONCURRENT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_MR_NOTSUPPORTED: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_MR_SUPPORTED: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_NC_END: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_NC_HIGH: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_NC_LOW: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_NC_START: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_NP_ABOUTTODO: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_NP_DIDEVENT: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_NP_FAILEDTODO: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_NP_OKTODO: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_NP_SYNCHAFTER: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_NT_MULTIPLEROWS: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_NT_SINGLEROW: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_OA_ATEXECUTE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_OA_ATROWRELEASE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_OA_NOTSUPPORTED: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_OO_BLOB: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_OO_DIRECTBIND: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_OO_IPERSIST: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_OO_ROWOBJECT: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_OO_SCOPED: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_OO_SINGLETON: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_OP_EQUAL: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_OP_RELATIVE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_OP_STRING: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_ORS_HISTOGRAM: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_ORS_INDEX: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_ORS_INTEGRATEDINDEX: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_ORS_STOREDPROC: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_ORS_TABLE: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_OS_AGR_AFTERSESSION: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_OS_CLIENTCURSOR: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_OS_DISABLEALL: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_OS_ENABLEALL: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_OS_RESOURCEPOOLING: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_OS_TXNENLISTMENT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_PERSIST_ADTG: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_PERSIST_XML: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_PT_GUID: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_PT_GUID_NAME: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_PT_GUID_PROPID: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_PT_NAME: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_PT_PGUID_NAME: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_PT_PGUID_PROPID: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_PT_PROPID: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_RD_RESETALL: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_RT_APTMTTHREAD: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_RT_FREETHREAD: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_RT_SINGLETHREAD: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_SQL_ANSI89_IEF: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_SQL_ANSI92_ENTRY: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_SQL_ANSI92_FULL: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_SQL_ANSI92_INTERMEDIATE: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_SQL_ESCAPECLAUSES: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_SQL_FIPS_TRANSITIONAL: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_SQL_NONE: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_SQL_ODBC_CORE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_SQL_ODBC_EXTENDED: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_SQL_ODBC_MINIMUM: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_SQL_SUBMINIMUM: i32 = 512i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_SQ_COMPARISON: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_SQ_CORRELATEDSUBQUERIES: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_SQ_EXISTS: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_SQ_IN: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_SQ_QUANTIFIED: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_SQ_TABLE: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_SS_ILOCKBYTES: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_SS_ISEQUENTIALSTREAM: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_SS_ISTORAGE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_SS_ISTREAM: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_STGM_CONVERT: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_STGM_DELETEONRELEASE: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_STGM_DIRECT: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_STGM_FAILIFTHERE: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_STGM_PRIORITY: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_STGM_TRANSACTED: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_SU_DML_STATEMENTS: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_SU_INDEX_DEFINITION: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_SU_PRIVILEGE_DEFINITION: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_SU_TABLE_DEFINITION: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_TC_ALL: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_TC_DDL_COMMIT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_TC_DDL_IGNORE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_TC_DDL_LOCK: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_TC_DML: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_TC_NONE: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_TI_BROWSE: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_TI_CHAOS: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_TI_CURSORSTABILITY: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_TI_ISOLATED: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_TI_READCOMMITTED: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_TI_READUNCOMMITTED: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_TI_REPEATABLEREAD: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_TI_SERIALIZABLE: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_TR_ABORT: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_TR_ABORT_DC: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_TR_ABORT_NO: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_TR_BOTH: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_TR_COMMIT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_TR_COMMIT_DC: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_TR_COMMIT_NO: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_TR_DONTCARE: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_TR_NONE: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_TR_OPTIMISTIC: i32 = 512i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_TS_CARDINALITY: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_TS_HISTOGRAM: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_UP_CHANGE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_UP_DELETE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROPVAL_UP_INSERT: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_HCHAPTER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_INTERLEAVEDROWS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_MAINTAINPROPS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_MSDAORA8_DETERMINEKEYCOLUMNS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_MSDAORA_DETERMINEKEYCOLUMNS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_PersistFormat: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_PersistSchema: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_Unicode: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBRANGEENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBRANGE_INCLUSIVESTART: DBRANGEENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBRANGE_INCLUSIVEEND: DBRANGEENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBRANGE_EXCLUSIVESTART: DBRANGEENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBRANGE_EXCLUSIVEEND: DBRANGEENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBRANGE_EXCLUDENULLS: DBRANGEENUM = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBRANGE_PREFIX: DBRANGEENUM = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBRANGE_MATCH: DBRANGEENUM = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBRANGEENUM20 = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBRANGE_MATCH_N_SHIFT: DBRANGEENUM20 = 24i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBRANGE_MATCH_N_MASK: DBRANGEENUM20 = 255i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBREASONENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBREASON_ROWSET_FETCHPOSITIONCHANGE: DBREASONENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBREASON_ROWSET_RELEASE: DBREASONENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBREASON_COLUMN_SET: DBREASONENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBREASON_COLUMN_RECALCULATED: DBREASONENUM = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBREASON_ROW_ACTIVATE: DBREASONENUM = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBREASON_ROW_RELEASE: DBREASONENUM = 5i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBREASON_ROW_DELETE: DBREASONENUM = 6i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBREASON_ROW_FIRSTCHANGE: DBREASONENUM = 7i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBREASON_ROW_INSERT: DBREASONENUM = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBREASON_ROW_RESYNCH: DBREASONENUM = 9i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBREASON_ROW_UNDOCHANGE: DBREASONENUM = 10i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBREASON_ROW_UNDOINSERT: DBREASONENUM = 11i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBREASON_ROW_UNDODELETE: DBREASONENUM = 12i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBREASON_ROW_UPDATE: DBREASONENUM = 13i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBREASON_ROWSET_CHANGED: DBREASONENUM = 14i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBREASONENUM15 = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBREASON_ROWPOSITION_CHANGED: DBREASONENUM15 = 15i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBREASON_ROWPOSITION_CHAPTERCHANGED: DBREASONENUM15 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBREASON_ROWPOSITION_CLEARED: DBREASONENUM15 = 17i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBREASON_ROW_ASYNCHINSERT: DBREASONENUM15 = 18i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBREASONENUM25 = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBREASON_ROWSET_ROWSADDED: DBREASONENUM25 = 19i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBREASON_ROWSET_POPULATIONCOMPLETE: DBREASONENUM25 = 20i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBREASON_ROWSET_POPULATIONSTOPPED: DBREASONENUM25 = 21i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBRESOURCEKINDENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBRESOURCE_INVALID: DBRESOURCEKINDENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBRESOURCE_TOTAL: DBRESOURCEKINDENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBRESOURCE_CPU: DBRESOURCEKINDENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBRESOURCE_MEMORY: DBRESOURCEKINDENUM = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBRESOURCE_DISK: DBRESOURCEKINDENUM = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBRESOURCE_NETWORK: DBRESOURCEKINDENUM = 5i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBRESOURCE_RESPONSE: DBRESOURCEKINDENUM = 6i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBRESOURCE_ROWS: DBRESOURCEKINDENUM = 7i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBRESOURCE_OTHER: DBRESOURCEKINDENUM = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBRESULTFLAGENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBRESULTFLAG_DEFAULT: DBRESULTFLAGENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBRESULTFLAG_ROWSET: DBRESULTFLAGENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBRESULTFLAG_ROW: DBRESULTFLAGENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBROWCHANGEKINDENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBROWCHANGEKIND_INSERT: DBROWCHANGEKINDENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBROWCHANGEKIND_DELETE: DBROWCHANGEKINDENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBROWCHANGEKIND_UPDATE: DBROWCHANGEKINDENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBROWCHANGEKIND_COUNT: DBROWCHANGEKINDENUM = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBROWSTATUSENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBROWSTATUS_S_OK: DBROWSTATUSENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBROWSTATUS_S_MULTIPLECHANGES: DBROWSTATUSENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBROWSTATUS_S_PENDINGCHANGES: DBROWSTATUSENUM = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBROWSTATUS_E_CANCELED: DBROWSTATUSENUM = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBROWSTATUS_E_CANTRELEASE: DBROWSTATUSENUM = 6i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBROWSTATUS_E_CONCURRENCYVIOLATION: DBROWSTATUSENUM = 7i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBROWSTATUS_E_DELETED: DBROWSTATUSENUM = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBROWSTATUS_E_PENDINGINSERT: DBROWSTATUSENUM = 9i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBROWSTATUS_E_NEWLYINSERTED: DBROWSTATUSENUM = 10i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBROWSTATUS_E_INTEGRITYVIOLATION: DBROWSTATUSENUM = 11i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBROWSTATUS_E_INVALID: DBROWSTATUSENUM = 12i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBROWSTATUS_E_MAXPENDCHANGESEXCEEDED: DBROWSTATUSENUM = 13i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBROWSTATUS_E_OBJECTOPEN: DBROWSTATUSENUM = 14i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBROWSTATUS_E_OUTOFMEMORY: DBROWSTATUSENUM = 15i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBROWSTATUS_E_PERMISSIONDENIED: DBROWSTATUSENUM = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBROWSTATUS_E_LIMITREACHED: DBROWSTATUSENUM = 17i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBROWSTATUS_E_SCHEMAVIOLATION: DBROWSTATUSENUM = 18i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBROWSTATUS_E_FAIL: DBROWSTATUSENUM = 19i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBROWSTATUSENUM20 = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBROWSTATUS_S_NOCHANGE: DBROWSTATUSENUM20 = 20i32;
pub const DBSCHEMA_LINKEDSERVERS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2425604852, data2: 11948, data3: 4561, data4: [152, 9, 0, 192, 79, 194, 173, 152] };
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBSEEKENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSEEK_INVALID: DBSEEKENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSEEK_FIRSTEQ: DBSEEKENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSEEK_LASTEQ: DBSEEKENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSEEK_AFTEREQ: DBSEEKENUM = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSEEK_AFTER: DBSEEKENUM = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSEEK_BEFOREEQ: DBSEEKENUM = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSEEK_BEFORE: DBSEEKENUM = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBSORTENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSORT_ASCENDING: DBSORTENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSORT_DESCENDING: DBSORTENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBSOURCETYPEENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSOURCETYPE_DATASOURCE: DBSOURCETYPEENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSOURCETYPE_ENUMERATOR: DBSOURCETYPEENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBSOURCETYPEENUM20 = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSOURCETYPE_DATASOURCE_TDP: DBSOURCETYPEENUM20 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSOURCETYPE_DATASOURCE_MDP: DBSOURCETYPEENUM20 = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBSOURCETYPEENUM25 = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSOURCETYPE_BINDER: DBSOURCETYPEENUM25 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBSTATUSENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSTATUS_S_OK: DBSTATUSENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSTATUS_E_BADACCESSOR: DBSTATUSENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSTATUS_E_CANTCONVERTVALUE: DBSTATUSENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSTATUS_S_ISNULL: DBSTATUSENUM = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSTATUS_S_TRUNCATED: DBSTATUSENUM = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSTATUS_E_SIGNMISMATCH: DBSTATUSENUM = 5i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSTATUS_E_DATAOVERFLOW: DBSTATUSENUM = 6i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSTATUS_E_CANTCREATE: DBSTATUSENUM = 7i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSTATUS_E_UNAVAILABLE: DBSTATUSENUM = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSTATUS_E_PERMISSIONDENIED: DBSTATUSENUM = 9i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSTATUS_E_INTEGRITYVIOLATION: DBSTATUSENUM = 10i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSTATUS_E_SCHEMAVIOLATION: DBSTATUSENUM = 11i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSTATUS_E_BADSTATUS: DBSTATUSENUM = 12i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSTATUS_S_DEFAULT: DBSTATUSENUM = 13i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBSTATUSENUM20 = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDSTATUS_S_CELLEMPTY: DBSTATUSENUM20 = 14i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSTATUS_S_IGNORE: DBSTATUSENUM20 = 15i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBSTATUSENUM21 = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSTATUS_E_DOESNOTEXIST: DBSTATUSENUM21 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSTATUS_E_INVALIDURL: DBSTATUSENUM21 = 17i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSTATUS_E_RESOURCELOCKED: DBSTATUSENUM21 = 18i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSTATUS_E_RESOURCEEXISTS: DBSTATUSENUM21 = 19i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSTATUS_E_CANNOTCOMPLETE: DBSTATUSENUM21 = 20i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSTATUS_E_VOLUMENOTFOUND: DBSTATUSENUM21 = 21i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSTATUS_E_OUTOFSPACE: DBSTATUSENUM21 = 22i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSTATUS_S_CANNOTDELETESOURCE: DBSTATUSENUM21 = 23i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSTATUS_E_READONLY: DBSTATUSENUM21 = 24i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSTATUS_E_RESOURCEOUTOFSCOPE: DBSTATUSENUM21 = 25i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSTATUS_S_ALREADYEXISTS: DBSTATUSENUM21 = 26i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBSTATUSENUM25 = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSTATUS_E_CANCELED: DBSTATUSENUM25 = 27i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSTATUS_E_NOTCOLLECTION: DBSTATUSENUM25 = 28i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBSTATUSENUM26 = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSTATUS_S_ROWSETCOLUMN: DBSTATUSENUM26 = 29i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBTABLESTATISTICSTYPE26 = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSTAT_HISTOGRAM: DBTABLESTATISTICSTYPE26 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSTAT_COLUMN_CARDINALITY: DBTABLESTATISTICSTYPE26 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBSTAT_TUPLE_CARDINALITY: DBTABLESTATISTICSTYPE26 = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub struct DBTIME {
    pub hour: u16,
    pub minute: u16,
    pub second: u16,
}
impl ::core::marker::Copy for DBTIME {}
impl ::core::clone::Clone for DBTIME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct DBTIMESTAMP {
    pub year: i16,
    pub month: u16,
    pub day: u16,
    pub hour: u16,
    pub minute: u16,
    pub second: u16,
    pub fraction: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for DBTIMESTAMP {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for DBTIMESTAMP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
#[cfg(target_arch = "x86")]
pub struct DBTIMESTAMP {
    pub year: i16,
    pub month: u16,
    pub day: u16,
    pub hour: u16,
    pub minute: u16,
    pub second: u16,
    pub fraction: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for DBTIMESTAMP {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for DBTIMESTAMP {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBTYPEENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_EMPTY: DBTYPEENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_NULL: DBTYPEENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_I2: DBTYPEENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_I4: DBTYPEENUM = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_R4: DBTYPEENUM = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_R8: DBTYPEENUM = 5i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_CY: DBTYPEENUM = 6i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_DATE: DBTYPEENUM = 7i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_BSTR: DBTYPEENUM = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_IDISPATCH: DBTYPEENUM = 9i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_ERROR: DBTYPEENUM = 10i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_BOOL: DBTYPEENUM = 11i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_VARIANT: DBTYPEENUM = 12i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_IUNKNOWN: DBTYPEENUM = 13i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_DECIMAL: DBTYPEENUM = 14i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_UI1: DBTYPEENUM = 17i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_ARRAY: DBTYPEENUM = 8192i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_BYREF: DBTYPEENUM = 16384i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_I1: DBTYPEENUM = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_UI2: DBTYPEENUM = 18i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_UI4: DBTYPEENUM = 19i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_I8: DBTYPEENUM = 20i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_UI8: DBTYPEENUM = 21i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_GUID: DBTYPEENUM = 72i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_VECTOR: DBTYPEENUM = 4096i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_RESERVED: DBTYPEENUM = 32768i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_BYTES: DBTYPEENUM = 128i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_STR: DBTYPEENUM = 129i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_WSTR: DBTYPEENUM = 130i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_NUMERIC: DBTYPEENUM = 131i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_UDT: DBTYPEENUM = 132i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_DBDATE: DBTYPEENUM = 133i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_DBTIME: DBTYPEENUM = 134i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_DBTIMESTAMP: DBTYPEENUM = 135i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBTYPEENUM15 = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_HCHAPTER: DBTYPEENUM15 = 136i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBTYPEENUM20 = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_FILETIME: DBTYPEENUM20 = 64i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_PROPVARIANT: DBTYPEENUM20 = 138i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_VARNUMERIC: DBTYPEENUM20 = 139i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBTYPE_SQLVARIANT: u32 = 144u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBUPDELRULEENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBUPDELRULE_NOACTION: DBUPDELRULEENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBUPDELRULE_CASCADE: DBUPDELRULEENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBUPDELRULE_SETNULL: DBUPDELRULEENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBUPDELRULE_SETDEFAULT: DBUPDELRULEENUM = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct DBVECTOR {
    pub size: usize,
    pub ptr: *mut ::core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for DBVECTOR {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for DBVECTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
#[cfg(target_arch = "x86")]
pub struct DBVECTOR {
    pub size: usize,
    pub ptr: *mut ::core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for DBVECTOR {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for DBVECTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBWATCHMODEENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBWATCHMODE_ALL: DBWATCHMODEENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBWATCHMODE_EXTEND: DBWATCHMODEENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBWATCHMODE_MOVE: DBWATCHMODEENUM = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBWATCHMODE_COUNT: DBWATCHMODEENUM = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DBWATCHNOTIFYENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBWATCHNOTIFY_ROWSCHANGED: DBWATCHNOTIFYENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBWATCHNOTIFY_QUERYDONE: DBWATCHNOTIFYENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBWATCHNOTIFY_QUERYREEXECUTED: DBWATCHNOTIFYENUM = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_ALL_EXCEPT_LIKE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_BINDFLAGS_COLLECTION: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_BINDFLAGS_DELAYFETCHCOLUMNS: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_BINDFLAGS_DELAYFETCHSTREAM: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_BINDFLAGS_ISSTRUCTUREDDOCUMENT: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_BINDFLAGS_OPENIFEXISTS: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_BINDFLAGS_OUTPUT: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_BINDFLAGS_OVERWRITE: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_BINDFLAGS_RECURSIVE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_COLLATION_ASC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_COLLATION_DESC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_COUNTUNAVAILABLE: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_ABORTLIMITREACHED: ::windows_sys::core::HRESULT = -2147217871i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_ALREADYINITIALIZED: ::windows_sys::core::HRESULT = -2147217838i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_ALTERRESTRICTED: ::windows_sys::core::HRESULT = -2147217763i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_ASYNCNOTSUPPORTED: ::windows_sys::core::HRESULT = -2147217771i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADACCESSORFLAGS: ::windows_sys::core::HRESULT = -2147217850i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADACCESSORHANDLE: ::windows_sys::core::HRESULT = -2147217920i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADACCESSORTYPE: ::windows_sys::core::HRESULT = -2147217845i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADBINDINFO: ::windows_sys::core::HRESULT = -2147217912i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADBOOKMARK: ::windows_sys::core::HRESULT = -2147217906i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADCHAPTER: ::windows_sys::core::HRESULT = -2147217914i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADCOLUMNID: ::windows_sys::core::HRESULT = -2147217903i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADCOMMANDFLAGS: ::windows_sys::core::HRESULT = -2147217780i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADCOMMANDID: ::windows_sys::core::HRESULT = -2147217802i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADCOMPAREOP: ::windows_sys::core::HRESULT = -2147217881i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADCONSTRAINTFORM: ::windows_sys::core::HRESULT = -2147217800i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADCONSTRAINTID: ::windows_sys::core::HRESULT = -2147217781i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADCONSTRAINTTYPE: ::windows_sys::core::HRESULT = -2147217801i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADCONVERTFLAG: ::windows_sys::core::HRESULT = -2147217828i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADCOPY: ::windows_sys::core::HRESULT = -2147217863i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADDEFERRABILITY: ::windows_sys::core::HRESULT = -2147217799i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADDYNAMICERRORID: ::windows_sys::core::HRESULT = -2147217830i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADHRESULT: ::windows_sys::core::HRESULT = -2147217832i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADID: i32 = -2147217860i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADINDEXID: ::windows_sys::core::HRESULT = -2147217806i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADINITSTRING: ::windows_sys::core::HRESULT = -2147217805i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADLOCKMODE: ::windows_sys::core::HRESULT = -2147217905i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADLOOKUPID: ::windows_sys::core::HRESULT = -2147217831i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADMATCHTYPE: ::windows_sys::core::HRESULT = -2147217792i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADORDINAL: ::windows_sys::core::HRESULT = -2147217835i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADPARAMETERNAME: ::windows_sys::core::HRESULT = -2147217827i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADPRECISION: ::windows_sys::core::HRESULT = -2147217862i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADPROPERTYVALUE: ::windows_sys::core::HRESULT = -2147217852i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADRATIO: ::windows_sys::core::HRESULT = -2147217902i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADRECORDNUM: ::windows_sys::core::HRESULT = -2147217854i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADREGIONHANDLE: ::windows_sys::core::HRESULT = -2147217878i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADROWHANDLE: ::windows_sys::core::HRESULT = -2147217916i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADSCALE: ::windows_sys::core::HRESULT = -2147217861i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADSOURCEHANDLE: ::windows_sys::core::HRESULT = -2147217840i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADSTARTPOSITION: ::windows_sys::core::HRESULT = -2147217890i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADSTATUSVALUE: ::windows_sys::core::HRESULT = -2147217880i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADSTORAGEFLAG: ::windows_sys::core::HRESULT = -2147217882i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADSTORAGEFLAGS: ::windows_sys::core::HRESULT = -2147217849i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADTABLEID: ::windows_sys::core::HRESULT = -2147217860i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADTYPE: ::windows_sys::core::HRESULT = -2147217859i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADTYPENAME: ::windows_sys::core::HRESULT = -2147217872i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADUPDATEDELETERULE: ::windows_sys::core::HRESULT = -2147217782i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BADVALUES: ::windows_sys::core::HRESULT = -2147217901i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BOGUS: ::windows_sys::core::HRESULT = -2147217665i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BOOKMARKSKIPPED: ::windows_sys::core::HRESULT = -2147217853i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_BYREFACCESSORNOTSUPPORTED: ::windows_sys::core::HRESULT = -2147217848i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_CANCELED: ::windows_sys::core::HRESULT = -2147217842i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_CANNOTCONNECT: ::windows_sys::core::HRESULT = -2147217770i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_CANNOTFREE: ::windows_sys::core::HRESULT = -2147217894i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_CANNOTRESTART: ::windows_sys::core::HRESULT = -2147217896i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_CANTCANCEL: ::windows_sys::core::HRESULT = -2147217899i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_CANTCONVERTVALUE: ::windows_sys::core::HRESULT = -2147217913i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_CANTFETCHBACKWARDS: ::windows_sys::core::HRESULT = -2147217884i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_CANTFILTER: ::windows_sys::core::HRESULT = -2147217825i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_CANTORDER: ::windows_sys::core::HRESULT = -2147217824i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_CANTSCROLLBACKWARDS: ::windows_sys::core::HRESULT = -2147217879i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_CANTTRANSLATE: ::windows_sys::core::HRESULT = -2147217869i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_CHAPTERNOTRELEASED: ::windows_sys::core::HRESULT = -2147217841i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_COLUMNUNAVAILABLE: ::windows_sys::core::HRESULT = -2147217760i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_COMMANDNOTPERSISTED: ::windows_sys::core::HRESULT = -2147217817i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_CONCURRENCYVIOLATION: ::windows_sys::core::HRESULT = -2147217864i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_COSTLIMIT: ::windows_sys::core::HRESULT = -2147217907i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_DATAOVERFLOW: ::windows_sys::core::HRESULT = -2147217833i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_DELETEDROW: ::windows_sys::core::HRESULT = -2147217885i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_DIALECTNOTSUPPORTED: ::windows_sys::core::HRESULT = -2147217898i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_DROPRESTRICTED: ::windows_sys::core::HRESULT = -2147217776i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_DUPLICATECOLUMNID: ::windows_sys::core::HRESULT = -2147217858i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_DUPLICATECONSTRAINTID: ::windows_sys::core::HRESULT = -2147217767i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_DUPLICATEDATASOURCE: ::windows_sys::core::HRESULT = -2147217897i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_DUPLICATEID: ::windows_sys::core::HRESULT = -2147217816i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_DUPLICATEINDEXID: ::windows_sys::core::HRESULT = -2147217868i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_DUPLICATETABLEID: ::windows_sys::core::HRESULT = -2147217857i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_ERRORSINCOMMAND: ::windows_sys::core::HRESULT = -2147217900i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_ERRORSOCCURRED: ::windows_sys::core::HRESULT = -2147217887i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_GOALREJECTED: ::windows_sys::core::HRESULT = -2147217892i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_INDEXINUSE: ::windows_sys::core::HRESULT = -2147217866i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_INTEGRITYVIOLATION: ::windows_sys::core::HRESULT = -2147217873i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_INVALID: ::windows_sys::core::HRESULT = -2147217851i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_INVALIDTRANSITION: ::windows_sys::core::HRESULT = -2147217876i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_LIMITREJECTED: ::windows_sys::core::HRESULT = -2147217909i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_MAXPENDCHANGESEXCEEDED: ::windows_sys::core::HRESULT = -2147217836i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_MISMATCHEDPROVIDER: ::windows_sys::core::HRESULT = -2147217803i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_MULTIPLESTATEMENTS: ::windows_sys::core::HRESULT = -2147217874i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_MULTIPLESTORAGE: ::windows_sys::core::HRESULT = -2147217826i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_NEWLYINSERTED: ::windows_sys::core::HRESULT = -2147217893i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_NOAGGREGATION: ::windows_sys::core::HRESULT = -2147217886i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_NOCOLUMN: ::windows_sys::core::HRESULT = -2147217819i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_NOCOMMAND: ::windows_sys::core::HRESULT = -2147217908i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_NOCONSTRAINT: ::windows_sys::core::HRESULT = -2147217761i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_NOINDEX: ::windows_sys::core::HRESULT = -2147217867i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_NOLOCALE: ::windows_sys::core::HRESULT = -2147217855i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_NONCONTIGUOUSRANGE: ::windows_sys::core::HRESULT = -2147217877i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_NOPROVIDERSREGISTERED: ::windows_sys::core::HRESULT = -2147217804i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_NOQUERY: ::windows_sys::core::HRESULT = -2147217889i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_NOSOURCEOBJECT: ::windows_sys::core::HRESULT = -2147217775i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_NOSTATISTIC: ::windows_sys::core::HRESULT = -2147217764i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_NOTABLE: ::windows_sys::core::HRESULT = -2147217865i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_NOTAREFERENCECOLUMN: ::windows_sys::core::HRESULT = -2147217910i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_NOTASUBREGION: ::windows_sys::core::HRESULT = -2147217875i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_NOTCOLLECTION: ::windows_sys::core::HRESULT = -2147217773i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_NOTFOUND: ::windows_sys::core::HRESULT = -2147217895i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_NOTPREPARED: ::windows_sys::core::HRESULT = -2147217846i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_NOTREENTRANT: ::windows_sys::core::HRESULT = -2147217888i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_NOTSUPPORTED: ::windows_sys::core::HRESULT = -2147217837i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_NULLACCESSORNOTSUPPORTED: ::windows_sys::core::HRESULT = -2147217847i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_OBJECTCREATIONLIMITREACHED: ::windows_sys::core::HRESULT = -2147217815i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_OBJECTMISMATCH: ::windows_sys::core::HRESULT = -2147217779i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_OBJECTOPEN: ::windows_sys::core::HRESULT = -2147217915i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_OUTOFSPACE: ::windows_sys::core::HRESULT = -2147217766i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_PARAMNOTOPTIONAL: ::windows_sys::core::HRESULT = -2147217904i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_PARAMUNAVAILABLE: ::windows_sys::core::HRESULT = -2147217839i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_PENDINGCHANGES: ::windows_sys::core::HRESULT = -2147217834i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_PENDINGINSERT: ::windows_sys::core::HRESULT = -2147217829i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_READONLY: ::windows_sys::core::HRESULT = -2147217772i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_READONLYACCESSOR: ::windows_sys::core::HRESULT = -2147217918i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_RESOURCEEXISTS: ::windows_sys::core::HRESULT = -2147217768i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_RESOURCELOCKED: ::windows_sys::core::HRESULT = -2147217774i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_RESOURCENOTSUPPORTED: ::windows_sys::core::HRESULT = -2147217762i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_RESOURCEOUTOFSCOPE: ::windows_sys::core::HRESULT = -2147217778i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_ROWLIMITEXCEEDED: ::windows_sys::core::HRESULT = -2147217919i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_ROWSETINCOMMAND: ::windows_sys::core::HRESULT = -2147217870i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_ROWSNOTRELEASED: ::windows_sys::core::HRESULT = -2147217883i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_SCHEMAVIOLATION: ::windows_sys::core::HRESULT = -2147217917i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_TABLEINUSE: ::windows_sys::core::HRESULT = -2147217856i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_TIMEOUT: ::windows_sys::core::HRESULT = -2147217769i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_UNSUPPORTEDCONVERSION: ::windows_sys::core::HRESULT = -2147217891i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_E_WRITEONLYACCESSOR: ::windows_sys::core::HRESULT = -2147217844i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_IMP_LEVEL_ANONYMOUS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_IMP_LEVEL_DELEGATE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_IMP_LEVEL_IDENTIFY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_IMP_LEVEL_IMPERSONATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_IN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_INVALID_HACCESSOR: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_INVALID_HCHAPTER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_LIKE_ONLY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_LOCAL_EXCLUSIVE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_LOCAL_SHARED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_MODE_READ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_MODE_READWRITE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_MODE_SHARE_DENY_NONE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_MODE_SHARE_DENY_READ: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_MODE_SHARE_DENY_WRITE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_MODE_SHARE_EXCLUSIVE: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_MODE_WRITE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_NULL_HACCESSOR: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_NULL_HCHAPTER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_NULL_HROW: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub struct DB_NUMERIC {
    pub precision: u8,
    pub scale: u8,
    pub sign: u8,
    pub val: [u8; 16],
}
impl ::core::marker::Copy for DB_NUMERIC {}
impl ::core::clone::Clone for DB_NUMERIC {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_OUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_PROT_LEVEL_CALL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_PROT_LEVEL_CONNECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_PROT_LEVEL_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_PROT_LEVEL_PKT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_PROT_LEVEL_PKT_INTEGRITY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_PROT_LEVEL_PKT_PRIVACY: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_PT_FUNCTION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_PT_PROCEDURE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_PT_UNKNOWN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_REMOTE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_SEARCHABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_SEC_E_AUTH_FAILED: ::windows_sys::core::HRESULT = -2147217843i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_SEC_E_PERMISSIONDENIED: ::windows_sys::core::HRESULT = -2147217911i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_SEC_E_SAFEMODE_DENIED: ::windows_sys::core::HRESULT = -2147217765i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_S_ASYNCHRONOUS: ::windows_sys::core::HRESULT = 265936i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_S_BADROWHANDLE: ::windows_sys::core::HRESULT = 265939i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_S_BOOKMARKSKIPPED: ::windows_sys::core::HRESULT = 265923i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_S_BUFFERFULL: ::windows_sys::core::HRESULT = 265928i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_S_CANTRELEASE: ::windows_sys::core::HRESULT = 265930i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_S_COLUMNSCHANGED: ::windows_sys::core::HRESULT = 265937i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_S_COLUMNTYPEMISMATCH: ::windows_sys::core::HRESULT = 265921i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_S_COMMANDREEXECUTED: ::windows_sys::core::HRESULT = 265927i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_S_DELETEDROW: ::windows_sys::core::HRESULT = 265940i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_S_DIALECTIGNORED: ::windows_sys::core::HRESULT = 265933i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_S_ENDOFROWSET: ::windows_sys::core::HRESULT = 265926i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_S_ERRORSOCCURRED: ::windows_sys::core::HRESULT = 265946i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_S_ERRORSRETURNED: ::windows_sys::core::HRESULT = 265938i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_S_GOALCHANGED: ::windows_sys::core::HRESULT = 265931i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_S_LOCKUPGRADED: ::windows_sys::core::HRESULT = 265944i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_S_MULTIPLECHANGES: ::windows_sys::core::HRESULT = 265948i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_S_NONEXTROWSET: ::windows_sys::core::HRESULT = 265925i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_S_NORESULT: ::windows_sys::core::HRESULT = 265929i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_S_NOROWSPECIFICCOLUMNS: ::windows_sys::core::HRESULT = 265949i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_S_NOTSINGLETON: ::windows_sys::core::HRESULT = 265943i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_S_PARAMUNAVAILABLE: ::windows_sys::core::HRESULT = 265947i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_S_PROPERTIESCHANGED: ::windows_sys::core::HRESULT = 265945i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_S_ROWLIMITEXCEEDED: ::windows_sys::core::HRESULT = 265920i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_S_STOPLIMITREACHED: ::windows_sys::core::HRESULT = 265942i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_S_TOOMANYCHANGES: ::windows_sys::core::HRESULT = 265941i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_S_TYPEINFOOVERRIDDEN: ::windows_sys::core::HRESULT = 265922i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_S_UNWANTEDOPERATION: ::windows_sys::core::HRESULT = 265932i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_S_UNWANTEDPHASE: ::windows_sys::core::HRESULT = 265934i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_S_UNWANTEDREASON: ::windows_sys::core::HRESULT = 265935i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DB_UNSEARCHABLE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub struct DB_VARNUMERIC {
    pub precision: u8,
    pub scale: i8,
    pub sign: u8,
    pub val: [u8; 1],
}
impl ::core::marker::Copy for DB_VARNUMERIC {}
impl ::core::clone::Clone for DB_VARNUMERIC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct DCINFO {
    pub eInfoType: u32,
    pub vData: super::Com::VARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for DCINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for DCINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DCINFOTYPEENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DCINFOTYPE_VERSION: DCINFOTYPEENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type DELIVERY_AGENT_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DELIVERY_AGENT_FLAG_NO_BROADCAST: DELIVERY_AGENT_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DELIVERY_AGENT_FLAG_NO_RESTRICTIONS: DELIVERY_AGENT_FLAGS = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DELIVERY_AGENT_FLAG_SILENT_DIAL: DELIVERY_AGENT_FLAGS = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DISPID_QUERY_ALL: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DISPID_QUERY_HITCOUNT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DISPID_QUERY_LASTSEENTIME: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DISPID_QUERY_METADATA_PROPDISPID: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DISPID_QUERY_METADATA_PROPGUID: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DISPID_QUERY_METADATA_PROPMODIFIABLE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DISPID_QUERY_METADATA_PROPNAME: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DISPID_QUERY_METADATA_STORELEVEL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DISPID_QUERY_METADATA_VROOTAUTOMATIC: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DISPID_QUERY_METADATA_VROOTMANUAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DISPID_QUERY_METADATA_VROOTUSED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DISPID_QUERY_RANK: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DISPID_QUERY_RANKVECTOR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DISPID_QUERY_REVNAME: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DISPID_QUERY_UNFILTERED: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DISPID_QUERY_VIRTUALPATH: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DISPID_QUERY_WORKID: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DS_E_ALREADYDISABLED: i32 = -2147220447i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DS_E_ALREADYENABLED: i32 = -2147220454i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DS_E_BADREQUEST: i32 = -2147220475i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DS_E_BADRESULT: i32 = -2147220445i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DS_E_BADSEQUENCE: i32 = -2147220473i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DS_E_BUFFERTOOSMALL: i32 = -2147220449i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DS_E_CANNOTREMOVECONCURRENT: i32 = -2147220443i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DS_E_CANNOTWRITEREGISTRY: i32 = -2147220444i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DS_E_CONFIGBAD: i32 = -2147220470i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DS_E_CONFIGNOTRIGHTTYPE: i32 = -2147220456i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DS_E_DATANOTPRESENT: i32 = -2147220464i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DS_E_DATASOURCENOTAVAILABLE: i32 = -2147220478i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DS_E_DATASOURCENOTDISABLED: i32 = -2147220459i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DS_E_DUPLICATEID: i32 = -2147220462i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DS_E_INDEXDIRECTORY: i32 = -2147220452i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DS_E_INVALIDCATALOGNAME: i32 = -2147220457i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DS_E_INVALIDDATASOURCE: i32 = -2147220479i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DS_E_INVALIDTAGDB: i32 = -2147220458i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DS_E_MESSAGETOOLONG: i32 = -2147220472i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DS_E_MISSINGCATALOG: i32 = -2147220440i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DS_E_NOMOREDATA: i32 = -2147220480i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DS_E_PARAMOUTOFRANGE: i32 = -2147220448i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DS_E_PROPVERSIONMISMATCH: i32 = -2147220441i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DS_E_PROTOCOLVERSION: i32 = -2147220455i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DS_E_QUERYCANCELED: i32 = -2147220477i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DS_E_QUERYHUNG: i32 = -2147220446i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DS_E_REGISTRY: i32 = -2147220460i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DS_E_SEARCHCATNAMECOLLISION: i32 = -2147220442i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DS_E_SERVERCAPACITY: i32 = -2147220474i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DS_E_SERVERERROR: i32 = -2147220471i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DS_E_SETSTATUSINPROGRESS: i32 = -2147220463i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DS_E_TOOMANYDATASOURCES: i32 = -2147220461i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DS_E_UNKNOWNPARAM: i32 = -2147220450i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DS_E_UNKNOWNREQUEST: i32 = -2147220476i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DS_E_VALUETOOLARGE: i32 = -2147220451i32;
pub const DataLinks: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 570871218, data2: 6593, data3: 4561, data4: [137, 224, 0, 192, 79, 215, 168, 41] };
#[repr(C)]
pub struct DataSource {
    pub base__: ::windows_sys::core::IUnknown,
    pub getDataMember: unsafe extern "system" fn(this: *mut *mut Self, bstrdm: *const u16, riid: *const ::windows_sys::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub getDataMemberName: unsafe extern "system" fn(this: *mut *mut Self, lindex: i32, pbstrdm: *mut *mut u16) -> ::windows_sys::core::HRESULT,
    pub getDataMemberCount: unsafe extern "system" fn(this: *mut *mut Self, plcount: *mut i32) -> ::windows_sys::core::HRESULT,
    pub addDataSourceListener: unsafe extern "system" fn(this: *mut *mut Self, pdsl: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub removeDataSourceListener: unsafe extern "system" fn(this: *mut *mut Self, pdsl: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for DataSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2081422003, data2: 52612, data3: 4560, data4: [148, 154, 0, 160, 201, 17, 16, 237] };
}
#[repr(C)]
pub struct DataSourceListener {
    pub base__: ::windows_sys::core::IUnknown,
    pub dataMemberChanged: unsafe extern "system" fn(this: *mut *mut Self, bstrdm: *const u16) -> ::windows_sys::core::HRESULT,
    pub dataMemberAdded: unsafe extern "system" fn(this: *mut *mut Self, bstrdm: *const u16) -> ::windows_sys::core::HRESULT,
    pub dataMemberRemoved: unsafe extern "system" fn(this: *mut *mut Self, bstrdm: *const u16) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for DataSourceListener {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2081422002, data2: 52612, data3: 4560, data4: [148, 154, 0, 160, 201, 17, 16, 237] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct DataSourceObject {
    pub base__: super::Com::IDispatch,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for DataSourceObject {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 183084260, data2: 6356, data3: 4561, data4: [179, 179, 0, 170, 0, 193, 169, 36] };
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type EBindInfoOptions = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const BIO_BINDER: EBindInfoOptions = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct ERRORINFO {
    pub hrError: ::windows_sys::core::HRESULT,
    pub dwMinor: u32,
    pub clsid: ::windows_sys::core::GUID,
    pub iid: ::windows_sys::core::GUID,
    pub dispid: i32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for ERRORINFO {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for ERRORINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
#[cfg(target_arch = "x86")]
pub struct ERRORINFO {
    pub hrError: ::windows_sys::core::HRESULT,
    pub dwMinor: u32,
    pub clsid: ::windows_sys::core::GUID,
    pub iid: ::windows_sys::core::GUID,
    pub dispid: i32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for ERRORINFO {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for ERRORINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ERROR_FTE: u32 = 13824u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ERROR_FTE_CB: u32 = 51968u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ERROR_FTE_FD: u32 = 64768u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ERROR_SOURCE_CMDLINE: u32 = 5376u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ERROR_SOURCE_COLLATOR: u32 = 1280u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ERROR_SOURCE_CONNMGR: u32 = 1536u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ERROR_SOURCE_CONTENT_SOURCE: u32 = 13312u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ERROR_SOURCE_DATASOURCE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ERROR_SOURCE_DAV: u32 = 8960u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ERROR_SOURCE_EXSTOREPH: u32 = 9984u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ERROR_SOURCE_FLTRDMN: u32 = 9216u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ERROR_SOURCE_GATHERER: u32 = 3328u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ERROR_SOURCE_INDEXER: u32 = 4352u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ERROR_SOURCE_MSS: u32 = 8448u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ERROR_SOURCE_NETWORKING: u32 = 768u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ERROR_SOURCE_NLADMIN: u32 = 6400u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ERROR_SOURCE_NOTESPH: u32 = 9728u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ERROR_SOURCE_OLEDB_BINDER: u32 = 9472u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ERROR_SOURCE_PEOPLE_IMPORT: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ERROR_SOURCE_PROTHNDLR: u32 = 4608u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ERROR_SOURCE_QUERY: u32 = 1792u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ERROR_SOURCE_REMOTE_EXSTOREPH: u32 = 13568u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ERROR_SOURCE_SCHEMA: u32 = 3072u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ERROR_SOURCE_SCRIPTPI: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ERROR_SOURCE_SECURITY: u32 = 5120u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ERROR_SOURCE_SETUP: u32 = 4864u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ERROR_SOURCE_SRCH_SCHEMA_CACHE: u32 = 13056u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ERROR_SOURCE_XML: u32 = 8704u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_AUDIENCECOMPUTATION_CANNOTSTART: i32 = -1073738223i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_AUTOCAT_CANT_CREATE_FILE_SHARE: i32 = -1073738726i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_AUTOCAT_PERFMON: i32 = -1073738753i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_CONFIG_ERROR: i32 = -1073738821i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_CONFIG_SYNTAX: i32 = -2147482604i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_CRAWL_SCHEDULED: i32 = 1073744884i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_DETAILED_FILTERPOOL_ADD_FAILED: i32 = -1073738719i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_DSS_NOT_ENABLED: i32 = -2147476572i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_ENUMERATE_SESSIONS_FAILED: i32 = -1073738720i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_EXCEPTION: i32 = -1073740815i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_FAILED_CREATE_GATHERER_LOG: i32 = -2147480587i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_FAILED_INITIALIZE_CRAWL: i32 = -1073738765i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_FILTERPOOL_ADD_FAILED: i32 = -1073738722i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_FILTERPOOL_DELETE_FAILED: i32 = -1073738721i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_FILTER_HOST_FORCE_TERMINATE: i32 = -2147473624i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_FILTER_HOST_NOT_INITIALIZED: i32 = -1073738724i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_FILTER_HOST_NOT_TERMINATED: i32 = -1073738723i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHERER_DATASOURCE: i32 = -1073738727i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHERER_PERFMON: i32 = -1073738817i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHERSVC_PERFMON: i32 = -1073738818i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_ADVISE_FAILED: i32 = -1073738798i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_APP_INIT_FAILED: i32 = -1073738766i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_AUTODESCENCODE_INVALID: i32 = -2147480592i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_AUTODESCLEN_ADJUSTED: i32 = -2147480603i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_BACKUPAPP_COMPLETE: i32 = 3077i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_BACKUPAPP_ERROR: i32 = -1073738748i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_CANT_CREATE_DOCID: i32 = -1073738793i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_CANT_DELETE_DOCID: i32 = -1073738792i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_CHECKPOINT_CORRUPT: i32 = -1073738732i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_CHECKPOINT_FAILED: i32 = -1073738736i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_CHECKPOINT_FILE_MISSING: i32 = -1073738731i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_CRAWL_IN_PROGRESS: i32 = -2147480609i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_CRAWL_NOT_STARTED: i32 = -2147480625i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_CRAWL_SEED_ERROR: i32 = -2147480624i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_CRAWL_SEED_FAILED: i32 = -2147480612i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_CRAWL_SEED_FAILED_INIT: i32 = -2147480611i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_CRITICAL_ERROR: i32 = -1073738799i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_DAEMON_TERMINATED: i32 = -2147480570i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_DELETING_HISTORY_ITEMS: i32 = -1073738774i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_DIRTY_STARTUP: i32 = -2147480576i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_DISK_FULL: i32 = -2147480594i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_END_ADAPTIVE: i32 = 1073744891i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_END_CRAWL: i32 = 1073744842i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_END_INCREMENTAL: i32 = 1073744871i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_EXCEPTION: i32 = -1073738810i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_FLUSH_FAILED: i32 = -1073738737i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_FROM_NOT_SET: i32 = -1073738776i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_HISTORY_CORRUPTION_DETECTED: i32 = -2147480575i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_INTERNAL: i32 = -1073738804i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_INVALID_NETWORK_ACCESS_ACCOUNT: i32 = -1073738739i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_LOCK_FAILED: i32 = -1073738784i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_NO_CRAWL_SEEDS: i32 = -2147480602i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_NO_SCHEMA: i32 = -2147480593i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_OBJ_INIT_FAILED: i32 = -1073738796i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_PLUGINMGR_INIT_FAILED: i32 = -1073738767i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_PLUGIN_INIT_FAILED: i32 = -1073738795i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_PROTOCOLHANDLER_INIT_FAILED: i32 = -1073738740i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_PROTOCOLHANDLER_LOAD_FAILED: i32 = -1073738741i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_READ_CHECKPOINT_FAILED: i32 = -1073738733i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_RECOVERY_FAILURE: i32 = -1073738222i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_REG_MISSING: i32 = -2147480610i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_RESET_START: i32 = 1073744865i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_RESTOREAPP_COMPLETE: i32 = 3075i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_RESTOREAPP_ERROR: i32 = -1073738750i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_RESTORE_CHECKPOINT_FAILED: i32 = -1073738734i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_RESTORE_COMPLETE: i32 = 3069i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_RESTORE_ERROR: i32 = -1073738754i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_RESUME: i32 = 1073744868i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_SAVE_FAILED: i32 = -1073738735i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_SERVICE_INIT: i32 = -1073738794i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_START_CRAWL: i32 = 1073744843i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_START_CRAWL_IF_RESET: i32 = -2147480595i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_START_PAUSE: i32 = -2147480606i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_STOP_START: i32 = 1073744876i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_SYSTEM_LCID_CHANGED: i32 = -2147480562i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_THROTTLE: i32 = 1073744867i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_GATHER_TRANSACTION_FAIL: i32 = -1073738797i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_HASHMAP_INSERT: i32 = -1073738816i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_HASHMAP_UPDATE: i32 = -1073738811i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_ADD_DSS_DISCONNECT: i32 = -2147476585i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_ADD_DSS_FAILED: i32 = -2147476627i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_ADD_DSS_SUCCEEDED: i32 = 7019i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_BUILD_ENDED: i32 = 1073748873i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_BUILD_FAILED: i32 = -1073734797i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_BUILD_START: i32 = 1073748872i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_CI_LOAD_ERROR: i32 = -1073734785i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_DSS_ALREADY_ADDED: i32 = 1073748870i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_DSS_CONTACT_FAILED: i32 = -1073734800i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_DSS_UNABLE_TO_REMOVE: i32 = -1073734755i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_FAIL_TO_CREATE_PER_USER_CATALOG: i32 = -1073731797i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_FAIL_TO_SET_MAX_JETINSTANCE: i32 = -1073731798i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_FAIL_TO_UNLOAD_PER_USER_CATALOG: i32 = -1073731796i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_INIT_ERROR: i32 = -1073734814i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_INVALID_DIRECTORY: i32 = -1073734813i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_LOAD_FAIL: i32 = -1073734781i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_MISSING_APP_DIRECTORY: i32 = -1073734758i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_NEW_PROJECT: i32 = -1073734754i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_NO_SEARCH_SERVERS: i32 = -2147476630i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_OUT_OF_DATABASE_INSTANCE: i32 = -1073731799i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_PAUSED_FOR_DISKFULL: i32 = -1073734811i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_PERFMON: i32 = -1073734760i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_PROPSTORE_INIT_FAILED: i32 = -1073734787i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_PROP_ABORTED: i32 = 1073748899i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_PROP_COMMITTED: i32 = 1073748898i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_PROP_COMMIT_FAILED: i32 = -1073734747i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_PROP_ERROR: i32 = -1073734812i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_PROP_STARTED: i32 = 1073748841i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_PROP_STATE_CORRUPT: i32 = -1073734780i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_PROP_STOPPED: i32 = -2147476633i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_PROP_SUCCEEDED: i32 = 7016i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_REG_ERROR: i32 = -1073734756i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_REG_MISSING: i32 = -1073734796i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_REMOVED_PROJECT: i32 = -1073734753i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_REMOVE_DSS_FAILED: i32 = -1073734801i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_REMOVE_DSS_SUCCEEDED: i32 = 7020i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_RESET_FOR_CORRUPTION: i32 = -1073734784i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_SCHEMA_COPY_ERROR: i32 = -1073734823i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_SHUTDOWN: i32 = 1073748866i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_STARTED: i32 = 1073748824i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_INDEXER_VERIFY_PROP_ACCOUNT: i32 = -1073734768i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_LEARN_COMPILE_FAILED: i32 = -2147480583i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_LEARN_CREATE_DB_FAILED: i32 = -2147480584i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_LEARN_PROPAGATION_COPY_FAILED: i32 = -2147480585i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_LEARN_PROPAGATION_FAILED: i32 = -2147480582i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_LOCAL_GROUPS_CACHE_FLUSHED: i32 = 1073744920i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_LOCAL_GROUP_NOT_EXPANDED: i32 = 1073744919i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_NOTIFICATION_FAILURE: i32 = -1073738745i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_NOTIFICATION_FAILURE_SCOPE_EXCEEDED_LOGGING: i32 = -2147480568i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_NOTIFICATION_RESTORED: i32 = 1073744905i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_NOTIFICATION_RESTORED_SCOPE_EXCEEDED_LOGGING: i32 = -2147480566i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_NOTIFICATION_THREAD_EXIT_FAILED: i32 = -1073738725i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_OUTOFMEMORY: i32 = -1073740817i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_PERF_COUNTERS_ALREADY_EXISTS: i32 = -2147473626i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_PERF_COUNTERS_NOT_LOADED: i32 = -2147473628i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_PERF_COUNTERS_REGISTRY_TROUBLE: i32 = -2147473627i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_PROTOCOL_HOST_FORCE_TERMINATE: i32 = -2147473625i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_REG_VERSION: i32 = -1073738790i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_SSSEARCH_CREATE_PATH_RULES_FAILED: i32 = -2147482634i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_SSSEARCH_CSM_SAVE_FAILED: i32 = -1073740805i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_SSSEARCH_DATAFILES_MOVE_FAILED: i32 = -1073740808i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_SSSEARCH_DATAFILES_MOVE_ROLLBACK_ERRORS: i32 = -2147482630i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_SSSEARCH_DATAFILES_MOVE_SUCCEEDED: i32 = 1073742841i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_SSSEARCH_DROPPED_EVENTS: i32 = -2147482633i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_SSSEARCH_SETUP_CLEANUP_FAILED: i32 = -1073740813i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_SSSEARCH_SETUP_CLEANUP_STARTED: i32 = -2147482640i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_SSSEARCH_SETUP_CLEANUP_SUCCEEDED: i32 = 1073742834i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_SSSEARCH_SETUP_FAILED: i32 = -1073740818i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_SSSEARCH_SETUP_SUCCEEDED: i32 = 1073742829i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_SSSEARCH_STARTED: i32 = 1073742827i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_SSSEARCH_STARTING_SETUP: i32 = 1073742828i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_SSSEARCH_STOPPED: i32 = 1073742837i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_STS_INIT_SECURITY_FAILED: i32 = -2147480554i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_SYSTEM_EXCEPTION: i32 = -2147482595i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_TRANSACTION_READ: i32 = -1073738809i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_TRANSLOG_APPEND: i32 = -1073738814i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_TRANSLOG_CREATE: i32 = -1073738791i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_TRANSLOG_CREATE_TRX: i32 = -1073738815i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_TRANSLOG_UPDATE: i32 = -1073738813i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_UNPRIVILEGED_SERVICE_ACCOUNT: i32 = -2147482596i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_USING_DIFFERENT_WORD_BREAKER: i32 = -2147480580i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_WARNING_CANNOT_UPGRADE_NOISE_FILE: i32 = -2147473634i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_WARNING_CANNOT_UPGRADE_NOISE_FILES: i32 = -2147473635i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_WBREAKER_NOT_LOADED: i32 = -2147480586i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EVENT_WIN32_ERROR: i32 = -2147473633i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EXCI_E_ACCESS_DENIED: i32 = -2147216990i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EXCI_E_BADCONFIG_OR_ACCESSDENIED: i32 = -2147216988i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EXCI_E_INVALID_ACCOUNT_INFO: i32 = -2147216984i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EXCI_E_INVALID_EXCHANGE_SERVER: i32 = -2147216989i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EXCI_E_INVALID_SERVER_CONFIG: i32 = -2147216991i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EXCI_E_NOT_ADMIN_OR_WRONG_SITE: i32 = -2147216986i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EXCI_E_NO_CONFIG: i32 = -2147216992i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EXCI_E_NO_MAPI: i32 = -2147216985i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EXCI_E_WRONG_SERVER_OR_ACCT: i32 = -2147216987i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EXSTOREPH_E_UNEXPECTED: i32 = -2147211519i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EX_ANY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EX_CMDFATAL: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EX_CONTROL: u32 = 25u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EX_DBCORRUPT: u32 = 23u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EX_DBFATAL: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EX_DEADLOCK: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EX_HARDWARE: u32 = 24u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EX_INFO: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EX_INTOK: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EX_LIMIT: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EX_MAXISEVERITY: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EX_MISSING: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EX_PERMIT: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EX_RESOURCE: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EX_SYNTAX: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EX_TABCORRUPT: u32 = 22u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EX_TYPE: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const EX_USER: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FAIL: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub struct FILTERED_DATA_SOURCES {
    pub pwcsExtension: ::windows_sys::core::PCWSTR,
    pub pwcsMime: ::windows_sys::core::PCWSTR,
    pub pClsid: *const ::windows_sys::core::GUID,
    pub pwcsOverride: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for FILTERED_DATA_SOURCES {}
impl ::core::clone::Clone for FILTERED_DATA_SOURCES {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FLTRDMN_E_CANNOT_DECRYPT_PASSWORD: i32 = -2147212282i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FLTRDMN_E_ENCRYPTED_DOCUMENT: i32 = -2147212283i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FLTRDMN_E_FILTER_INIT_FAILED: i32 = -2147212284i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FLTRDMN_E_QI_FILTER_FAILED: i32 = -2147212286i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FLTRDMN_E_UNEXPECTED: i32 = -2147212287i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type FOLLOW_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FF_INDEXCOMPLEXURLS: FOLLOW_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FF_SUPPRESSINDEXING: FOLLOW_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_ADMIN_BLOB_CORRUPT: i32 = -2147207676i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_AFFINITY_MASK: i32 = -2147207651i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_ALREADY_INITIALIZED: i32 = -2147207604i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_ANOTHER_STATUS_CHANGE_IS_ALREADY_ACTIVE: i32 = -2147207635i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_BATCH_ABORTED: i32 = -2147207636i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_CATALOG_ALREADY_EXISTS: i32 = -2147207656i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_CATALOG_DOES_NOT_EXIST: i32 = -2147207639i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_CB_CBID_OUT_OF_BOUND: i32 = -2147169535i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_CB_NOT_ENOUGH_AVAIL_PHY_MEM: i32 = -2147169534i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_CB_NOT_ENOUGH_OCC_BUFFER: i32 = -2147169533i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_CB_OUT_OF_MEMORY: i32 = -2147169536i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_COM_SIGNATURE_VALIDATION: i32 = -2147207652i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_CORRUPT_GATHERER_HASH_MAP: i32 = -2147207619i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_CORRUPT_PROPERTY_STORE: i32 = -2147207622i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_CORRUPT_WORDLIST: i32 = -2147169532i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_DATATYPE_MISALIGNMENT: i32 = -2147207605i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_DEPENDENT_TRAN_FAILED_TO_PERSIST: i32 = -2147207641i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_DOC_TOO_HUGE: i32 = -2147207606i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_DUPLICATE_OBJECT: i32 = -2147207644i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_ERROR_WRITING_REGISTRY: i32 = -2147207674i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_EXCEEDED_MAX_PLUGINS: i32 = -2147207647i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_FAILED_TO_CREATE_ACCESSOR: i32 = -2147207625i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_FAILURE_TO_POST_SETCOMPLETION_STATUS: i32 = -2147207597i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_FD_DID_NOT_CONNECT: i32 = -2147207660i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_FD_DOC_TIMEOUT: i32 = -2147156733i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_FD_DOC_UNEXPECTED_EXIT: i32 = -2147156731i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_FD_FAILED_TO_LOAD_IFILTER: i32 = -2147156734i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_FD_FILTER_CAUSED_SHARING_VIOLATION: i32 = -2147156725i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_FD_IDLE: i32 = -2147207595i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_FD_IFILTER_INIT_FAILED: i32 = -2147156735i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_FD_NOISE_NO_IPERSISTSTREAM_ON_TEXT_FILTER: i32 = -2147156729i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_FD_NOISE_NO_TEXT_FILTER: i32 = -2147156730i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_FD_NOISE_TEXT_FILTER_INIT_FAILED: i32 = -2147156727i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_FD_NOISE_TEXT_FILTER_LOAD_FAILED: i32 = -2147156728i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_FD_NO_IPERSIST_INTERFACE: i32 = -2147156736i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_FD_OCCURRENCE_OVERFLOW: i32 = -2147156726i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_FD_OWNERSHIP_OBSOLETE: i32 = -2147207650i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_FD_SHUTDOWN: i32 = -2147207640i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_FD_TIMEOUT: i32 = -2147207632i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_FD_UNEXPECTED_EXIT: i32 = -2147156732i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_FD_UNRESPONSIVE: i32 = -2147207594i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_FD_USED_TOO_MUCH_MEMORY: i32 = -2147207603i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_FILTER_SINGLE_THREADED: i32 = -2147207675i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_HIGH_MEMORY_PRESSURE: i32 = -2147207601i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_INVALID_CODEPAGE: i32 = -2147207596i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_INVALID_DOCID: i32 = -2147207663i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_INVALID_ISOLATE_ERROR_BATCH: i32 = -2147207600i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_INVALID_PROG_ID: i32 = -2147207614i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_INVALID_PROJECT_ID: i32 = -2147207598i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_INVALID_PROPERTY: i32 = -2147207630i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_INVALID_TYPE: i32 = -2147207624i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_KEY_NOT_CACHED: i32 = -2147207618i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_LIBRARY_NOT_LOADED: i32 = -2147207627i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_NOT_PROCESSED_DUE_TO_PREVIOUS_ERRORS: i32 = -2147207633i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_NO_MORE_PROPERTIES: i32 = -2147207629i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_NO_PLUGINS: i32 = -2147207638i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_NO_PROPERTY_STORE: i32 = -1073465766i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_OUT_OF_RANGE: i32 = -2147207623i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_PATH_TOO_LONG: i32 = -2147207654i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_PAUSE_EXTERNAL: i32 = -2147207662i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_PERFMON_FULL: i32 = -2147207626i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_PERF_NOT_LOADED: i32 = -2147207611i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_PIPE_DATA_CORRUPTED: i32 = -2147207671i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_PIPE_NOT_CONNECTED: i32 = -2147207677i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_PROGID_REQUIRED: i32 = -2147207658i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_PROJECT_NOT_INITALIZED: i32 = -2147207672i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_PROJECT_SHUTDOWN: i32 = -2147207673i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_PROPERTY_STORE_WORKID_NOTVALID: i32 = -2147207621i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_READONLY_CATALOG: i32 = -2147207612i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_REDUNDANT_TRAN_FAILURE: i32 = -2147207642i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_REJECTED_DUE_TO_PROJECT_STATUS: i32 = -2147207661i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_RESOURCE_SHUTDOWN: i32 = -2147207631i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_RETRY_HUGE_DOC: i32 = -2147207608i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_RETRY_SINGLE_DOC_PER_BATCH: i32 = -2147207599i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_SECRET_NOT_FOUND: i32 = -2147207678i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_SERIAL_STREAM_CORRUPT: i32 = -2147207613i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_STACK_CORRUPTED: i32 = -2147207615i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_STATIC_THREAD_INVALID_ARGUMENTS: i32 = -2147207657i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_UNEXPECTED_EXIT: i32 = -2147207602i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_UNKNOWN_FD_TYPE: i32 = -2147207607i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_UNKNOWN_PLUGIN: i32 = -2147207628i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_UPGRADE_INTERFACE_ALREADY_INSTANTIATED: i32 = -2147207616i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_UPGRADE_INTERFACE_ALREADY_SHUTDOWN: i32 = -2147207617i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_E_URB_TOO_BIG: i32 = -2147207664i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_INVALID_ADMIN_CLIENT: i32 = -2147207653i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_S_BEYOND_QUOTA: i32 = 276002i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_S_CATALOG_BLOB_MISMATCHED: i32 = 276056i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_S_PROPERTY_RESET: i32 = 276057i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_S_PROPERTY_STORE_END_OF_ENUMERATION: i32 = 276028i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_S_READONLY_CATALOG: i32 = 276038i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_S_REDUNDANT: i32 = 276005i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_S_RESOURCES_STARTING_TO_GET_LOW: i32 = 275993i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_S_RESUME: i32 = 276014i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_S_STATUS_CHANGE_REQUEST: i32 = 276011i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const FTE_S_TRY_TO_FLUSH: i32 = 276055i32;
pub const FilterRegistration: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2652330893, data2: 62762, data3: 4568, data4: [185, 165, 80, 80, 84, 80, 48, 48] };
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GENERATE_METHOD_PREFIXMATCH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GENERATE_METHOD_STEMMED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GHTR_E_INSUFFICIENT_DISK_SPACE: i32 = -2147218037i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GHTR_E_LOCAL_SERVER_UNAVAILABLE: i32 = -2147218055i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_ADDLINKS_FAILED_WILL_RETRY_PARENT: i32 = -2147217989i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_APPLICATION_NOT_FOUND: i32 = -2147218079i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_AUTOCAT_UNEXPECTED: i32 = -2147218012i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_BACKUP_VALIDATION_FAIL: i32 = -2147217994i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_BAD_FILTER_DAEMON: i32 = -2147218119i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_BAD_FILTER_HOST: i32 = -2147217993i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_CANNOT_ENABLE_CHECKPOINT: i32 = -2147218002i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_CANNOT_REMOVE_PLUGINMGR: i32 = -2147218078i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_CONFIG_DUP_EXTENSION: i32 = -2147218165i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_CONFIG_DUP_PROJECT: i32 = -2147218166i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_CONTENT_ID_CONFLICT: i32 = -2147218062i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_DIRMON_NOT_INITIALZED: i32 = -2147218019i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_DUPLICATE_OBJECT: i32 = -2147218174i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_DUPLICATE_PROJECT: i32 = -2147218094i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_DUPLICATE_URL: i32 = -2147218163i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_DUP_PROPERTY_MAPPING: i32 = -2147218134i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_EMPTY_DACL: i32 = -2147218006i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_ERROR_INITIALIZING_PERFMON: i32 = -2147218171i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_ERROR_OBJECT_NOT_FOUND: i32 = -2147218170i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_ERROR_WRITING_REGISTRY: i32 = -2147218172i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_FILTERPOOL_NOTFOUND: i32 = -2147217990i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_FILTER_FAULT: i32 = -2147218075i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_FILTER_INIT: i32 = -2147218130i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_FILTER_INTERRUPTED: i32 = -2147218092i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_FILTER_INVALID_MESSAGE: i32 = -2147218158i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_FILTER_NOT_FOUND: i32 = -2147218154i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_FILTER_NO_CODEPAGE: i32 = -2147218123i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_FILTER_NO_MORE_THREADS: i32 = -2147218153i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_FILTER_PROCESS_TERMINATED: i32 = -2147218159i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_FILTER_PROCESS_TERMINATED_QUOTA: i32 = -2147218151i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_FILTER_SINGLE_THREADED: i32 = -2147218069i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_FOLDER_CRAWLED_BY_ANOTHER_WORKSPACE: i32 = -2147218007i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_FORCE_NOTIFICATION_RESET: i32 = -2147218065i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_FROM_NOT_SPECIFIED: i32 = -2147218109i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_IE_OFFLINE: i32 = -2147218120i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_INSUFFICIENT_EXAMPLE_CATEGORIES: i32 = -2147218014i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_INSUFFICIENT_EXAMPLE_DOCUMENTS: i32 = -2147218013i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_INSUFFICIENT_FEATURE_TERMS: i32 = -2147218015i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_INVALIDFUNCTION: i32 = -2147218161i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_INVALID_ACCOUNT: i32 = -2147218132i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_INVALID_ACCOUNT_SYNTAX: i32 = -2147218129i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_INVALID_APPLICATION_NAME: i32 = -2147218077i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_INVALID_CALL_FROM_WBREAKER: i32 = -2147218058i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_INVALID_DIRECTORY: i32 = -2147218093i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_INVALID_EXTENSION: i32 = -2147218107i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_INVALID_GROW_FACTOR: i32 = -2147218106i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_INVALID_HOST_NAME: i32 = -2147218096i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_INVALID_LOG_FILE_NAME: i32 = -2147218103i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_INVALID_MAPPING: i32 = -2147218112i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_INVALID_PATH: i32 = -2147218124i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_INVALID_PATH_EXPRESSION: i32 = -2147218088i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_INVALID_PATH_SPEC: i32 = -2147218016i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_INVALID_PROJECT_NAME: i32 = -2147218142i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_INVALID_PROXY_PORT: i32 = -2147218091i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_INVALID_RESOURCE_ID: i32 = -2147218035i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_INVALID_RETRIES: i32 = -2147218104i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_INVALID_START_ADDRESS: i32 = -2147217998i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_INVALID_START_PAGE: i32 = -2147218095i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_INVALID_START_PAGE_HOST: i32 = -2147218087i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_INVALID_START_PAGE_PATH: i32 = -2147218080i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_INVALID_STREAM_LOGS_COUNT: i32 = -2147218108i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_INVALID_TIME_OUT: i32 = -2147218105i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_JET_BACKUP_ERROR: i32 = -2147218026i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_JET_RESTORE_ERROR: i32 = -2147218025i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_LOCAL_GROUPS_EXPANSION_INTERNAL_ERROR: i32 = -2147216867i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_NAME_TOO_LONG: i32 = -2147218156i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_NESTED_HIERARCHICAL_START_ADDRESSES: i32 = -2147218034i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_NOFILTERSINK: i32 = -2147218160i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_NON_FIXED_DRIVE: i32 = -2147218074i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_NOTIFICATION_FILE_SHARE_INFO_NOT_AVAILABLE: i32 = -2147218040i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_NOTIFICATION_LOCAL_PATH_MUST_USE_FIXED_DRIVE: i32 = -2147218039i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_NOTIFICATION_START_ADDRESS_INVALID: i32 = -2147218042i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_NOTIFICATION_START_PAGE: i32 = -2147218137i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_NOTIFICATION_TYPE_NOT_SUPPORTED: i32 = -2147218041i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_NOTIF_ACCESS_TOKEN_UPDATED: i32 = -2147218020i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_NOTIF_BEING_REMOVED: i32 = -2147218018i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_NOTIF_EXCESSIVE_THROUGHPUT: i32 = -2147218017i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_NO_IDENTITY: i32 = -2147218155i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_NO_PRTCLHNLR: i32 = -2147218121i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_NTF_CLIENT_NOT_SUBSCRIBED: i32 = -1073476167i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_OBJECT_NOT_VALID: i32 = -2147218005i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_OUT_OF_DOC_ID: i32 = -2147218138i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_PIPE_NOT_CONNECTTED: i32 = -2147217996i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_PLUGIN_NOT_REGISTERED: i32 = -2147218021i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_PROJECT_NOT_INITIALIZED: i32 = -2147218149i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_PROPERTIES_EXCEEDED: i32 = -2147218000i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_PROPERTY_LIST_NOT_INITIALIZED: i32 = -2147218057i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_PROXY_NAME: i32 = -2147218127i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_PRT_HNDLR_PROGID_MISSING: i32 = -2147218152i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_RECOVERABLE_EXOLEDB_ERROR: i32 = -2147218060i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_RETRY: i32 = -2147218027i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_SCHEMA_ERRORS_OCCURRED: i32 = -2147218054i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_SCOPES_EXCEEDED: i32 = -2147218001i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_SECRET_NOT_FOUND: i32 = -2147218089i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_SERVER_UNAVAILABLE: i32 = -2147218126i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_SHUTTING_DOWN: i32 = -2147218141i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_SINGLE_THREADED_EMBEDDING: i32 = -2147218011i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_TIMEOUT: i32 = -2147218053i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_TOO_MANY_PLUGINS: i32 = -2147218162i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_UNABLE_TO_READ_EXCHANGE_STORE: i32 = -2147218061i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_UNABLE_TO_READ_REGISTRY: i32 = -2147218173i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_UNKNOWN_PROTOCOL: i32 = -2147218150i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_UNSUPPORTED_PROPERTY_TYPE: i32 = -2147218157i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_URL_EXCLUDED: i32 = -2147218169i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_URL_UNIDENTIFIED: i32 = -2147218067i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_USER_AGENT_NOT_SPECIFIED: i32 = -2147218111i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_E_VALUE_NOT_AVAILABLE: i32 = -2147218139i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_S_BAD_FILE_LINK: i32 = 265580i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_S_CANNOT_FILTER: i32 = 265520i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_S_CANNOT_WORDBREAK: i32 = 265638i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_S_CONFIG_HAS_ACCOUNTS: i32 = 265558i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_S_CRAWL_ADAPTIVE: i32 = 265605i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_S_CRAWL_FULL: i32 = 265603i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_S_CRAWL_INCREMENTAL: i32 = 265604i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_S_CRAWL_SCHEDULED: i32 = 265576i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_S_END_PROCESS_LOOP_NOTIFY_QUEUE: i32 = 265584i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_S_END_STD_CHUNKS: i32 = 265508i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_S_MODIFIED_PARTS: i32 = 265592i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_S_NOT_ALL_PARTS: i32 = 265582i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_S_NO_CRAWL_SEEDS: i32 = 265515i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_S_NO_INDEX: i32 = 265616i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_S_OFFICE_CHILD: i32 = 265626i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_S_PAUSE_REASON_BACKOFF: i32 = 265620i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_S_PAUSE_REASON_EXTERNAL: i32 = 265618i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_S_PAUSE_REASON_PROFILE_IMPORT: i32 = 265651i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_S_PAUSE_REASON_UPGRADING: i32 = 265619i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_S_PROB_NOT_MODIFIED: i32 = 265575i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_S_START_FILTER_FROM_BODY: i32 = 265585i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_S_START_FILTER_FROM_PROTOCOL: i32 = 265578i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_S_STATUS_CHANGE_IGNORED: i32 = 265500i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_S_STATUS_END_CRAWL: i32 = 265501i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_S_STATUS_PAUSE: i32 = 265505i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_S_STATUS_RESET: i32 = 265502i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_S_STATUS_RESUME: i32 = 265504i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_S_STATUS_START: i32 = 265526i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_S_STATUS_STOP: i32 = 265523i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_S_STATUS_THROTTLE: i32 = 265503i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_S_TRANSACTION_IGNORED: i32 = 265577i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const GTHR_S_USE_MIME_FILTER: i32 = 265639i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub struct HITRANGE {
    pub iPosition: u32,
    pub cLength: u32,
}
impl ::core::marker::Copy for HITRANGE {}
impl ::core::clone::Clone for HITRANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IAccessor {
    pub base__: ::windows_sys::core::IUnknown,
    pub AddRefAccessor: unsafe extern "system" fn(this: *mut *mut Self, haccessor: usize, pcrefcount: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateAccessor: unsafe extern "system" fn(this: *mut *mut Self, dwaccessorflags: u32, cbindings: usize, rgbindings: *const DBBINDING, cbrowsize: usize, phaccessor: *mut usize, rgstatus: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateAccessor: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetBindings: unsafe extern "system" fn(this: *mut *mut Self, haccessor: usize, pdwaccessorflags: *mut u32, pcbindings: *mut usize, prgbindings: *mut *mut DBBINDING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetBindings: usize,
    pub ReleaseAccessor: unsafe extern "system" fn(this: *mut *mut Self, haccessor: usize, pcrefcount: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAccessor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878220, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IAlterIndex {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AlterIndex: unsafe extern "system" fn(this: *mut *mut Self, ptableid: *mut super::super::Storage::IndexServer::DBID, pindexid: *mut super::super::Storage::IndexServer::DBID, pnewindexid: *mut super::super::Storage::IndexServer::DBID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AlterIndex: usize,
}
impl ::windows_sys::core::Interface for IAlterIndex {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878246, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IAlterTable {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AlterColumn: unsafe extern "system" fn(this: *mut *mut Self, ptableid: *mut super::super::Storage::IndexServer::DBID, pcolumnid: *mut super::super::Storage::IndexServer::DBID, dwcolumndescflags: u32, pcolumndesc: *mut DBCOLUMNDESC) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AlterColumn: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AlterTable: unsafe extern "system" fn(this: *mut *mut Self, ptableid: *mut super::super::Storage::IndexServer::DBID, pnewtableid: *mut super::super::Storage::IndexServer::DBID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AlterTable: usize,
}
impl ::windows_sys::core::Interface for IAlterTable {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878245, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IBindResource {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Bind: unsafe extern "system" fn(this: *mut *mut Self, punkouter: *mut ::core::ffi::c_void, pwszurl: ::windows_sys::core::PCWSTR, dwbindurlflags: u32, rguid: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, pauthenticate: *mut ::core::ffi::c_void, pimplsession: *mut DBIMPLICITSESSION, pdwbindstatus: *mut u32, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Bind: usize,
}
impl ::windows_sys::core::Interface for IBindResource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878257, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IChapteredRowset {
    pub base__: ::windows_sys::core::IUnknown,
    pub AddRefChapter: unsafe extern "system" fn(this: *mut *mut Self, hchapter: usize, pcrefcount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ReleaseChapter: unsafe extern "system" fn(this: *mut *mut Self, hchapter: usize, pcrefcount: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IChapteredRowset {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878227, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IColumnMapper {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub GetPropInfoFromName: unsafe extern "system" fn(this: *mut *mut Self, wcspropname: ::windows_sys::core::PCWSTR, pppropid: *mut *mut super::super::Storage::IndexServer::DBID, pproptype: *mut u16, puiwidth: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    GetPropInfoFromName: usize,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub GetPropInfoFromId: unsafe extern "system" fn(this: *mut *mut Self, ppropid: *const super::super::Storage::IndexServer::DBID, pwcsname: *mut *mut u16, pproptype: *mut u16, puiwidth: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    GetPropInfoFromId: usize,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub EnumPropInfo: unsafe extern "system" fn(this: *mut *mut Self, ientry: u32, pwcsname: *const *const u16, pppropid: *mut *mut super::super::Storage::IndexServer::DBID, pproptype: *mut u16, puiwidth: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    EnumPropInfo: usize,
    pub IsMapUpToDate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IColumnMapper {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 191095674, data2: 40140, data3: 4560, data4: [188, 219, 0, 128, 95, 204, 206, 4] };
}
#[repr(C)]
pub struct IColumnMapperCreator {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetColumnMapper: unsafe extern "system" fn(this: *mut *mut Self, wcsmachinename: ::windows_sys::core::PCWSTR, wcscatalogname: ::windows_sys::core::PCWSTR, ppcolumnmapper: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IColumnMapperCreator {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 191095675, data2: 40140, data3: 4560, data4: [188, 219, 0, 128, 95, 204, 206, 4] };
}
#[repr(C)]
pub struct IColumnsInfo {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
    pub GetColumnInfo: unsafe extern "system" fn(this: *mut *mut Self, pccolumns: *mut usize, prginfo: *mut *mut DBCOLUMNINFO, ppstringsbuffer: *mut *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com")))]
    GetColumnInfo: usize,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub MapColumnIDs: unsafe extern "system" fn(this: *mut *mut Self, ccolumnids: usize, rgcolumnids: *const super::super::Storage::IndexServer::DBID, rgcolumns: *mut usize) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    MapColumnIDs: usize,
}
impl ::windows_sys::core::Interface for IColumnsInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878097, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IColumnsInfo2 {
    pub base__: IColumnsInfo,
    #[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
    pub GetRestrictedColumnInfo: unsafe extern "system" fn(this: *mut *mut Self, ccolumnidmasks: usize, rgcolumnidmasks: *const super::super::Storage::IndexServer::DBID, dwflags: u32, pccolumns: *mut usize, prgcolumnids: *mut *mut super::super::Storage::IndexServer::DBID, prgcolumninfo: *mut *mut DBCOLUMNINFO, ppstringsbuffer: *mut *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com")))]
    GetRestrictedColumnInfo: usize,
}
impl ::windows_sys::core::Interface for IColumnsInfo2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878264, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IColumnsRowset {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub GetAvailableColumns: unsafe extern "system" fn(this: *mut *mut Self, pcoptcolumns: *mut usize, prgoptcolumns: *mut *mut super::super::Storage::IndexServer::DBID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    GetAvailableColumns: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetColumnsRowset: unsafe extern "system" fn(this: *mut *mut Self, punkouter: *mut ::core::ffi::c_void, coptcolumns: usize, rgoptcolumns: *const super::super::Storage::IndexServer::DBID, riid: *const ::windows_sys::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, ppcolrowset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetColumnsRowset: usize,
}
impl ::windows_sys::core::Interface for IColumnsRowset {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878096, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct ICommand {
    pub base__: ::windows_sys::core::IUnknown,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Execute: unsafe extern "system" fn(this: *mut *mut Self, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows_sys::core::GUID, pparams: *mut DBPARAMS, pcrowsaffected: *mut isize, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDBSession: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICommand {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878179, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct ICommandCost {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetAccumulatedCost: unsafe extern "system" fn(this: *mut *mut Self, pwszrowsetname: ::windows_sys::core::PCWSTR, pccostlimits: *mut u32, prgcostlimits: *mut *mut DBCOST) -> ::windows_sys::core::HRESULT,
    pub GetCostEstimate: unsafe extern "system" fn(this: *mut *mut Self, pwszrowsetname: ::windows_sys::core::PCWSTR, pccostestimates: *mut u32, prgcostestimates: *mut DBCOST) -> ::windows_sys::core::HRESULT,
    pub GetCostGoals: unsafe extern "system" fn(this: *mut *mut Self, pwszrowsetname: ::windows_sys::core::PCWSTR, pccostgoals: *mut u32, prgcostgoals: *mut DBCOST) -> ::windows_sys::core::HRESULT,
    pub GetCostLimits: unsafe extern "system" fn(this: *mut *mut Self, pwszrowsetname: ::windows_sys::core::PCWSTR, pccostlimits: *mut u32, prgcostlimits: *mut DBCOST) -> ::windows_sys::core::HRESULT,
    pub SetCostGoals: unsafe extern "system" fn(this: *mut *mut Self, pwszrowsetname: ::windows_sys::core::PCWSTR, ccostgoals: u32, rgcostgoals: *const DBCOST) -> ::windows_sys::core::HRESULT,
    pub SetCostLimits: unsafe extern "system" fn(this: *mut *mut Self, pwszrowsetname: ::windows_sys::core::PCWSTR, ccostlimits: u32, prgcostlimits: *mut DBCOST, dwexecutionflags: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICommandCost {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878158, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct ICommandPersist {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub DeleteCommand: unsafe extern "system" fn(this: *mut *mut Self, pcommandid: *mut super::super::Storage::IndexServer::DBID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    DeleteCommand: usize,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub GetCurrentCommand: unsafe extern "system" fn(this: *mut *mut Self, ppcommandid: *mut *mut super::super::Storage::IndexServer::DBID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    GetCurrentCommand: usize,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub LoadCommand: unsafe extern "system" fn(this: *mut *mut Self, pcommandid: *mut super::super::Storage::IndexServer::DBID, dwflags: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    LoadCommand: usize,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub SaveCommand: unsafe extern "system" fn(this: *mut *mut Self, pcommandid: *mut super::super::Storage::IndexServer::DBID, dwflags: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    SaveCommand: usize,
}
impl ::windows_sys::core::Interface for ICommandPersist {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878247, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct ICommandPrepare {
    pub base__: ::windows_sys::core::IUnknown,
    pub Prepare: unsafe extern "system" fn(this: *mut *mut Self, cexpectedruns: u32) -> ::windows_sys::core::HRESULT,
    pub Unprepare: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICommandPrepare {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878118, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct ICommandProperties {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperties: unsafe extern "system" fn(this: *mut *mut Self, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetProperties: unsafe extern "system" fn(this: *mut *mut Self, cpropertysets: u32, rgpropertysets: *const DBPROPSET) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetProperties: usize,
}
impl ::windows_sys::core::Interface for ICommandProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878201, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct ICommandStream {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCommandStream: unsafe extern "system" fn(this: *mut *mut Self, piid: *mut ::windows_sys::core::GUID, pguiddialect: *mut ::windows_sys::core::GUID, ppcommandstream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCommandStream: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, rguiddialect: *const ::windows_sys::core::GUID, pcommandstream: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICommandStream {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878271, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct ICommandText {
    pub base__: ICommand,
    pub GetCommandText: unsafe extern "system" fn(this: *mut *mut Self, pguiddialect: *mut ::windows_sys::core::GUID, ppwszcommand: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetCommandText: unsafe extern "system" fn(this: *mut *mut Self, rguiddialect: *const ::windows_sys::core::GUID, pwszcommand: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICommandText {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878119, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct ICommandValidate {
    pub base__: ::windows_sys::core::IUnknown,
    pub ValidateCompletely: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ValidateSyntax: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICommandValidate {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878104, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct ICommandWithParameters {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub GetParameterInfo: unsafe extern "system" fn(this: *mut *mut Self, pcparams: *mut usize, prgparaminfo: *mut *mut DBPARAMINFO, ppnamesbuffer: *mut *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetParameterInfo: usize,
    pub MapParameterNames: unsafe extern "system" fn(this: *mut *mut Self, cparamnames: usize, rgparamnames: *const ::windows_sys::core::PWSTR, rgparamordinals: *mut isize) -> ::windows_sys::core::HRESULT,
    pub SetParameterInfo: unsafe extern "system" fn(this: *mut *mut Self, cparams: usize, rgparamordinals: *const usize, rgparambindinfo: *const DBPARAMBINDINFO) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICommandWithParameters {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878180, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICondition {
    pub base__: super::Com::IPersistStream,
    #[cfg(feature = "Win32_System_Search_Common")]
    pub GetConditionType: unsafe extern "system" fn(this: *mut *mut Self, pnodetype: *mut Common::CONDITION_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Search_Common"))]
    GetConditionType: usize,
    pub GetSubConditions: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
    pub GetComparisonInfo: unsafe extern "system" fn(this: *mut *mut Self, ppszpropertyname: *mut ::windows_sys::core::PWSTR, pcop: *mut Common::CONDITION_OPERATION, ppropvar: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common")))]
    GetComparisonInfo: usize,
    pub GetValueType: unsafe extern "system" fn(this: *mut *mut Self, ppszvaluetypename: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetValueNormalization: unsafe extern "system" fn(this: *mut *mut Self, ppsznormalization: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetInputTerms: unsafe extern "system" fn(this: *mut *mut Self, pppropertyterm: *mut *mut ::core::ffi::c_void, ppoperationterm: *mut *mut ::core::ffi::c_void, ppvalueterm: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppc: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Clone: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICondition {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 264866004, data2: 51509, data3: 19351, data4: [169, 115, 70, 40, 46, 161, 117, 200] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICondition2 {
    pub base__: ICondition,
    pub GetLocale: unsafe extern "system" fn(this: *mut *mut Self, ppszlocalename: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub GetLeafConditionInfo: unsafe extern "system" fn(this: *mut *mut Self, ppropkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pcop: *mut Common::CONDITION_OPERATION, ppropvar: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_PropertiesSystem")))]
    GetLeafConditionInfo: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICondition2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 230196509, data2: 11867, data3: 18411, data4: [146, 8, 210, 140, 50, 90, 1, 215] };
}
#[repr(C)]
pub struct IConditionFactory {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub MakeNot: unsafe extern "system" fn(this: *mut *mut Self, pcsub: *mut ::core::ffi::c_void, fsimplify: super::super::Foundation::BOOL, ppcresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    MakeNot: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Search_Common"))]
    pub MakeAndOr: unsafe extern "system" fn(this: *mut *mut Self, ct: Common::CONDITION_TYPE, peusubs: *mut ::core::ffi::c_void, fsimplify: super::super::Foundation::BOOL, ppcresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Search_Common")))]
    MakeAndOr: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
    pub MakeLeaf: unsafe extern "system" fn(this: *mut *mut Self, pszpropertyname: ::windows_sys::core::PCWSTR, cop: Common::CONDITION_OPERATION, pszvaluetype: ::windows_sys::core::PCWSTR, ppropvar: *const super::Com::StructuredStorage::PROPVARIANT, ppropertynameterm: *mut ::core::ffi::c_void, poperationterm: *mut ::core::ffi::c_void, pvalueterm: *mut ::core::ffi::c_void, fexpand: super::super::Foundation::BOOL, ppcresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common")))]
    MakeLeaf: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Resolve: unsafe extern "system" fn(this: *mut *mut Self, pc: *mut ::core::ffi::c_void, sqro: STRUCTURED_QUERY_RESOLVE_OPTION, pstreferencetime: *const super::super::Foundation::SYSTEMTIME, ppcresolved: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Resolve: usize,
}
impl ::windows_sys::core::Interface for IConditionFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2783961203, data2: 45423, data3: 18255, data4: [159, 62, 159, 139, 73, 122, 62, 8] };
}
#[repr(C)]
pub struct IConditionFactory2 {
    pub base__: IConditionFactory,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateTrueFalse: unsafe extern "system" fn(this: *mut *mut Self, fval: super::super::Foundation::BOOL, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateTrueFalse: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateNegation: unsafe extern "system" fn(this: *mut *mut Self, pcsub: *mut ::core::ffi::c_void, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateNegation: usize,
    #[cfg(all(feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_Common"))]
    pub CreateCompoundFromObjectArray: unsafe extern "system" fn(this: *mut *mut Self, ct: Common::CONDITION_TYPE, poasubs: *mut ::core::ffi::c_void, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_Common")))]
    CreateCompoundFromObjectArray: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search_Common"))]
    pub CreateCompoundFromArray: unsafe extern "system" fn(this: *mut *mut Self, ct: Common::CONDITION_TYPE, ppcondsubs: *const *mut ::core::ffi::c_void, csubs: u32, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Search_Common")))]
    CreateCompoundFromArray: usize,
    #[cfg(all(feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub CreateStringLeaf: unsafe extern "system" fn(this: *mut *mut Self, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, pszvalue: ::windows_sys::core::PCWSTR, pszlocalename: ::windows_sys::core::PCWSTR, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_PropertiesSystem")))]
    CreateStringLeaf: usize,
    #[cfg(all(feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub CreateIntegerLeaf: unsafe extern "system" fn(this: *mut *mut Self, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, lvalue: i32, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_PropertiesSystem")))]
    CreateIntegerLeaf: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub CreateBooleanLeaf: unsafe extern "system" fn(this: *mut *mut Self, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, fvalue: super::super::Foundation::BOOL, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_PropertiesSystem")))]
    CreateBooleanLeaf: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub CreateLeaf: unsafe extern "system" fn(this: *mut *mut Self, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, propvar: *const super::Com::StructuredStorage::PROPVARIANT, pszsemantictype: ::windows_sys::core::PCWSTR, pszlocalename: ::windows_sys::core::PCWSTR, ppropertynameterm: *mut ::core::ffi::c_void, poperationterm: *mut ::core::ffi::c_void, pvalueterm: *mut ::core::ffi::c_void, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_PropertiesSystem")))]
    CreateLeaf: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub ResolveCondition: unsafe extern "system" fn(this: *mut *mut Self, pc: *mut ::core::ffi::c_void, sqro: STRUCTURED_QUERY_RESOLVE_OPTION, pstreferencetime: *const super::super::Foundation::SYSTEMTIME, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    ResolveCondition: usize,
}
impl ::windows_sys::core::Interface for IConditionFactory2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1909596897, data2: 17199, data3: 17054, data4: [140, 19, 182, 218, 253, 229, 7, 122] };
}
#[repr(C)]
pub struct IConditionGenerator {
    pub base__: ::windows_sys::core::IUnknown,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, pschemaprovider: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RecognizeNamedEntities: unsafe extern "system" fn(this: *mut *mut Self, pszinputstring: ::windows_sys::core::PCWSTR, lciduserlocale: u32, ptokencollection: *mut ::core::ffi::c_void, pnamedentities: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Search_Common"))]
    pub GenerateForLeaf: unsafe extern "system" fn(this: *mut *mut Self, pconditionfactory: *mut ::core::ffi::c_void, pszpropertyname: ::windows_sys::core::PCWSTR, cop: Common::CONDITION_OPERATION, pszvaluetype: ::windows_sys::core::PCWSTR, pszvalue: ::windows_sys::core::PCWSTR, pszvalue2: ::windows_sys::core::PCWSTR, ppropertynameterm: *mut ::core::ffi::c_void, poperationterm: *mut ::core::ffi::c_void, pvalueterm: *mut ::core::ffi::c_void, automaticwildcard: super::super::Foundation::BOOL, pnostringquery: *mut super::super::Foundation::BOOL, ppqueryexpression: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Search_Common")))]
    GenerateForLeaf: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub DefaultPhrase: unsafe extern "system" fn(this: *mut *mut Self, pszvaluetype: ::windows_sys::core::PCWSTR, ppropvar: *const super::Com::StructuredStorage::PROPVARIANT, fuseenglish: super::super::Foundation::BOOL, ppszphrase: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    DefaultPhrase: usize,
}
impl ::windows_sys::core::Interface for IConditionGenerator {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2463288408, data2: 17286, data3: 17827, data4: [185, 140, 126, 12, 230, 74, 65, 23] };
}
#[repr(C)]
pub struct IConvertType {
    pub base__: ::windows_sys::core::IUnknown,
    pub CanConvert: unsafe extern "system" fn(this: *mut *mut Self, wfromtype: u16, wtotype: u16, dwconvertflags: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IConvertType {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878216, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct ICreateRow {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateRow: unsafe extern "system" fn(this: *mut *mut Self, punkouter: *mut ::core::ffi::c_void, pwszurl: ::windows_sys::core::PCWSTR, dwbindurlflags: u32, rguid: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, pauthenticate: *mut ::core::ffi::c_void, pimplsession: *mut DBIMPLICITSESSION, pdwbindstatus: *mut u32, ppwsznewurl: *mut ::windows_sys::core::PWSTR, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateRow: usize,
}
impl ::windows_sys::core::Interface for ICreateRow {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878258, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IDBAsynchNotify {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnLowResource: unsafe extern "system" fn(this: *mut *mut Self, dwreserved: usize) -> ::windows_sys::core::HRESULT,
    pub OnProgress: unsafe extern "system" fn(this: *mut *mut Self, hchapter: usize, eoperation: u32, ulprogress: usize, ulprogressmax: usize, easynchphase: u32, pwszstatustext: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub OnStop: unsafe extern "system" fn(this: *mut *mut Self, hchapter: usize, eoperation: u32, hrstatus: ::windows_sys::core::HRESULT, pwszstatustext: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDBAsynchNotify {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878230, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IDBAsynchStatus {
    pub base__: ::windows_sys::core::IUnknown,
    pub Abort: unsafe extern "system" fn(this: *mut *mut Self, hchapter: usize, eoperation: u32) -> ::windows_sys::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, hchapter: usize, eoperation: u32, pulprogress: *mut usize, pulprogressmax: *mut usize, peasynchphase: *mut u32, ppwszstatustext: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDBAsynchStatus {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878229, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IDBBinderProperties {
    pub base__: IDBProperties,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDBBinderProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878259, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IDBCreateCommand {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateCommand: unsafe extern "system" fn(this: *mut *mut Self, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows_sys::core::GUID, ppcommand: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDBCreateCommand {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878109, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IDBCreateSession {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateSession: unsafe extern "system" fn(this: *mut *mut Self, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows_sys::core::GUID, ppdbsession: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDBCreateSession {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878173, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IDBDataSourceAdmin {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateDataSource: unsafe extern "system" fn(this: *mut *mut Self, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows_sys::core::GUID, ppdbsession: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateDataSource: usize,
    pub DestroyDataSource: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetCreationProperties: unsafe extern "system" fn(this: *mut *mut Self, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertyinfosets: *mut u32, prgpropertyinfosets: *mut *mut DBPROPINFOSET, ppdescbuffer: *mut *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetCreationProperties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ModifyDataSource: unsafe extern "system" fn(this: *mut *mut Self, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ModifyDataSource: usize,
}
impl ::windows_sys::core::Interface for IDBDataSourceAdmin {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878202, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IDBInfo {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetKeywords: unsafe extern "system" fn(this: *mut *mut Self, ppwszkeywords: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLiteralInfo: unsafe extern "system" fn(this: *mut *mut Self, cliterals: u32, rgliterals: *const u32, pcliteralinfo: *mut u32, prgliteralinfo: *mut *mut DBLITERALINFO, ppcharbuffer: *mut *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLiteralInfo: usize,
}
impl ::windows_sys::core::Interface for IDBInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878217, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IDBInitialize {
    pub base__: ::windows_sys::core::IUnknown,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Uninitialize: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDBInitialize {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878219, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IDBPromptInitialize {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub PromptDataSource: unsafe extern "system" fn(this: *mut *mut Self, punkouter: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwpromptoptions: u32, csourcetypefilter: u32, rgsourcetypefilter: *const u32, pwszszzproviderfilter: ::windows_sys::core::PCWSTR, riid: *const ::windows_sys::core::GUID, ppdatasource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PromptDataSource: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PromptFileName: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: super::super::Foundation::HWND, dwpromptoptions: u32, pwszinitialdirectory: ::windows_sys::core::PCWSTR, pwszinitialfile: ::windows_sys::core::PCWSTR, ppwszselectedfile: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PromptFileName: usize,
}
impl ::windows_sys::core::Interface for IDBPromptInitialize {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 570870960, data2: 6593, data3: 4561, data4: [137, 224, 0, 192, 79, 215, 168, 41] };
}
#[repr(C)]
pub struct IDBProperties {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperties: unsafe extern "system" fn(this: *mut *mut Self, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetPropertyInfo: unsafe extern "system" fn(this: *mut *mut Self, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertyinfosets: *mut u32, prgpropertyinfosets: *mut *mut DBPROPINFOSET, ppdescbuffer: *mut *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetPropertyInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetProperties: unsafe extern "system" fn(this: *mut *mut Self, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetProperties: usize,
}
impl ::windows_sys::core::Interface for IDBProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878218, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IDBSchemaCommand {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCommand: unsafe extern "system" fn(this: *mut *mut Self, punkouter: *mut ::core::ffi::c_void, rguidschema: *const ::windows_sys::core::GUID, ppcommand: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetSchemas: unsafe extern "system" fn(this: *mut *mut Self, pcschemas: *mut u32, prgschemas: *mut *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDBSchemaCommand {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878160, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IDBSchemaRowset {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetRowset: unsafe extern "system" fn(this: *mut *mut Self, punkouter: *mut ::core::ffi::c_void, rguidschema: *const ::windows_sys::core::GUID, crestrictions: u32, rgrestrictions: *const super::Com::VARIANT, riid: *const ::windows_sys::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetRowset: usize,
    pub GetSchemas: unsafe extern "system" fn(this: *mut *mut Self, pcschemas: *mut u32, prgschemas: *mut *mut ::windows_sys::core::GUID, prgrestrictionsupport: *mut *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDBSchemaRowset {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878203, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IDCInfo {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetInfo: unsafe extern "system" fn(this: *mut *mut Self, cinfo: u32, rgeinfotype: *const u32, prginfo: *mut *mut DCINFO) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetInfo: unsafe extern "system" fn(this: *mut *mut Self, cinfo: u32, rginfo: *const DCINFO) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetInfo: usize,
}
impl ::windows_sys::core::Interface for IDCInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878236, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDENTIFIER_SDK_ERROR: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDENTIFIER_SDK_MASK: u32 = 4026531840u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDS_MON_BUILTIN_PROPERTY: ::windows_sys::core::HRESULT = 264511i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDS_MON_BUILTIN_VIEW: ::windows_sys::core::HRESULT = 264503i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDS_MON_CANNOT_CAST: ::windows_sys::core::HRESULT = 264518i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDS_MON_CANNOT_CONVERT: ::windows_sys::core::HRESULT = 264507i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDS_MON_COLUMN_NOT_DEFINED: ::windows_sys::core::HRESULT = 264502i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDS_MON_DATE_OUT_OF_RANGE: ::windows_sys::core::HRESULT = 264519i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDS_MON_DEFAULT_ERROR: ::windows_sys::core::HRESULT = 264495i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDS_MON_ILLEGAL_PASSTHROUGH: ::windows_sys::core::HRESULT = 264496i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDS_MON_INVALIDSELECT_COALESCE: ::windows_sys::core::HRESULT = 264517i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDS_MON_INVALID_CATALOG: ::windows_sys::core::HRESULT = 264516i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDS_MON_INVALID_IN_GROUP_CLAUSE: ::windows_sys::core::HRESULT = 264520i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDS_MON_MATCH_STRING: ::windows_sys::core::HRESULT = 264513i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDS_MON_NOT_COLUMN_OF_VIEW: ::windows_sys::core::HRESULT = 264510i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDS_MON_ORDINAL_OUT_OF_RANGE: ::windows_sys::core::HRESULT = 264500i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDS_MON_OR_NOT: ::windows_sys::core::HRESULT = 264506i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDS_MON_OUT_OF_MEMORY: ::windows_sys::core::HRESULT = 264504i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDS_MON_OUT_OF_RANGE: ::windows_sys::core::HRESULT = 264508i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDS_MON_PARSE_ERR_1_PARAM: ::windows_sys::core::HRESULT = 264497i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDS_MON_PARSE_ERR_2_PARAM: ::windows_sys::core::HRESULT = 264498i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDS_MON_PROPERTY_NAME_IN_VIEW: ::windows_sys::core::HRESULT = 264514i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDS_MON_RELATIVE_INTERVAL: ::windows_sys::core::HRESULT = 264509i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDS_MON_SELECT_STAR: ::windows_sys::core::HRESULT = 264505i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDS_MON_SEMI_COLON: ::windows_sys::core::HRESULT = 264499i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDS_MON_VIEW_ALREADY_DEFINED: ::windows_sys::core::HRESULT = 264515i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDS_MON_VIEW_NOT_DEFINED: ::windows_sys::core::HRESULT = 264501i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDS_MON_WEIGHT_OUT_OF_RANGE: ::windows_sys::core::HRESULT = 264512i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDX_E_BUILD_IN_PROGRESS: i32 = -2147217147i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDX_E_CATALOG_DISMOUNTED: i32 = -2147217124i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDX_E_CORRUPT_INDEX: i32 = -2147217136i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDX_E_DISKFULL: i32 = -2147217138i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDX_E_DOCUMENT_ABORTED: i32 = -2147217125i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDX_E_DSS_NOT_CONNECTED: i32 = -2147217126i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDX_E_IDXLSTFILE_CORRUPT: i32 = -2147217146i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDX_E_INVALIDTAG: i32 = -2147217151i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDX_E_INVALID_INDEX: i32 = -2147217137i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDX_E_METAFILE_CORRUPT: i32 = -2147217150i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDX_E_NOISELIST_NOTFOUND: i32 = -2147217141i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDX_E_NOT_LOADED: i32 = -2147217129i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDX_E_OBJECT_NOT_FOUND: i32 = -2147217144i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDX_E_PROPSTORE_INIT_FAILED: i32 = -2147217134i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDX_E_PROP_MAJOR_VERSION_MISMATCH: i32 = -2147217128i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDX_E_PROP_MINOR_VERSION_MISMATCH: i32 = -2147217127i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDX_E_PROP_STATE_CORRUPT: i32 = -2147217133i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDX_E_PROP_STOPPED: i32 = -2147217139i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDX_E_REGISTRY_ENTRY: i32 = -2147217145i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDX_E_SEARCH_SERVER_ALREADY_EXISTS: i32 = -2147217148i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDX_E_SEARCH_SERVER_NOT_FOUND: i32 = -2147217143i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDX_E_STEMMER_NOTFOUND: i32 = -2147217140i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDX_E_TOO_MANY_SEARCH_SERVERS: i32 = -2147217149i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDX_E_USE_APPGLOBAL_PROPTABLE: i32 = -2147217120i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDX_E_USE_DEFAULT_CONTENTCLASS: i32 = -2147217121i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDX_E_WB_NOTFOUND: i32 = -2147217142i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDX_S_DSS_NOT_AVAILABLE: i32 = 266525i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDX_S_NO_BUILD_IN_PROGRESS: i32 = 266516i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDX_S_SEARCH_SERVER_ALREADY_EXISTS: i32 = 266517i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const IDX_S_SEARCH_SERVER_DOES_NOT_EXIST: i32 = 266518i32;
#[repr(C)]
pub struct IDataConvert {
    pub base__: ::windows_sys::core::IUnknown,
    pub DataConvert: unsafe extern "system" fn(this: *mut *mut Self, wsrctype: u16, wdsttype: u16, cbsrclength: usize, pcbdstlength: *mut usize, psrc: *const ::core::ffi::c_void, pdst: *mut ::core::ffi::c_void, cbdstmaxlength: usize, dbssrcstatus: u32, pdbsstatus: *mut u32, bprecision: u8, bscale: u8, dwflags: u32) -> ::windows_sys::core::HRESULT,
    pub CanConvert: unsafe extern "system" fn(this: *mut *mut Self, wsrctype: u16, wdsttype: u16) -> ::windows_sys::core::HRESULT,
    pub GetConversionSize: unsafe extern "system" fn(this: *mut *mut Self, wsrctype: u16, wdsttype: u16, pcbsrclength: *const usize, pcbdstlength: *mut usize, psrc: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDataConvert {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878221, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IDataInitialize {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetDataSource: unsafe extern "system" fn(this: *mut *mut Self, punkouter: *mut ::core::ffi::c_void, dwclsctx: u32, pwszinitializationstring: ::windows_sys::core::PCWSTR, riid: *const ::windows_sys::core::GUID, ppdatasource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetInitializationString: unsafe extern "system" fn(this: *mut *mut Self, pdatasource: *mut ::core::ffi::c_void, fincludepassword: u8, ppwszinitstring: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub CreateDBInstance: unsafe extern "system" fn(this: *mut *mut Self, clsidprovider: *const ::windows_sys::core::GUID, punkouter: *mut ::core::ffi::c_void, dwclsctx: u32, pwszreserved: ::windows_sys::core::PCWSTR, riid: *const ::windows_sys::core::GUID, ppdatasource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateDBInstanceEx: unsafe extern "system" fn(this: *mut *mut Self, clsidprovider: *const ::windows_sys::core::GUID, punkouter: *mut ::core::ffi::c_void, dwclsctx: u32, pwszreserved: ::windows_sys::core::PCWSTR, pserverinfo: *const super::Com::COSERVERINFO, cmq: u32, rgmqresults: *mut super::Com::MULTI_QI) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateDBInstanceEx: usize,
    pub LoadStringFromStorage: unsafe extern "system" fn(this: *mut *mut Self, pwszfilename: ::windows_sys::core::PCWSTR, ppwszinitializationstring: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub WriteStringToStorage: unsafe extern "system" fn(this: *mut *mut Self, pwszfilename: ::windows_sys::core::PCWSTR, pwszinitializationstring: ::windows_sys::core::PCWSTR, dwcreationdisposition: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDataInitialize {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 570870961, data2: 6593, data3: 4561, data4: [137, 224, 0, 192, 79, 215, 168, 41] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDataSourceLocator {
    pub base__: super::Com::IDispatch,
    pub hWnd: unsafe extern "system" fn(this: *mut *mut Self, phwndparent: *mut i64) -> ::windows_sys::core::HRESULT,
    pub SethWnd: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: i64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub PromptNew: unsafe extern "system" fn(this: *mut *mut Self, ppadoconnection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PromptNew: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PromptEdit: unsafe extern "system" fn(this: *mut *mut Self, ppadoconnection: *mut *mut ::core::ffi::c_void, pbsuccess: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PromptEdit: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IDataSourceLocator {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 570870962, data2: 6593, data3: 4561, data4: [137, 224, 0, 192, 79, 215, 168, 41] };
}
#[repr(C)]
pub struct IEntity {
    pub base__: ::windows_sys::core::IUnknown,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, ppszname: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub Base: unsafe extern "system" fn(this: *mut *mut Self, pbaseentity: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Relationships: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, prelationships: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetRelationship: unsafe extern "system" fn(this: *mut *mut Self, pszrelationname: ::windows_sys::core::PCWSTR, prelationship: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MetaData: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, pmetadata: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NamedEntities: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, pnamedentities: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetNamedEntity: unsafe extern "system" fn(this: *mut *mut Self, pszvalue: ::windows_sys::core::PCWSTR, ppnamedentity: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DefaultPhrase: unsafe extern "system" fn(this: *mut *mut Self, ppszphrase: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEntity {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 606488721, data2: 59403, data3: 20435, data4: [183, 206, 79, 242, 250, 232, 147, 31] };
}
#[repr(C)]
pub struct IEnumItemProperties {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, rgelt: *mut ITEMPROP, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self, pncount: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumItemProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4146892182, data2: 28093, data3: 4561, data4: [161, 232, 0, 192, 79, 194, 251, 225] };
}
#[repr(C)]
pub struct IEnumSearchRoots {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumSearchRoots {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2872116609, data2: 44160, data3: 4561, data4: [141, 243, 0, 192, 79, 182, 239, 82] };
}
#[repr(C)]
pub struct IEnumSearchScopeRules {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, pprgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumSearchScopeRules {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2872116609, data2: 44160, data3: 4561, data4: [141, 243, 0, 192, 79, 182, 239, 84] };
}
#[repr(C)]
pub struct IEnumSubscription {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, rgelt: *mut ::windows_sys::core::GUID, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self, pncount: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumSubscription {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4146892183, data2: 28093, data3: 4561, data4: [161, 232, 0, 192, 79, 194, 251, 225] };
}
#[repr(C)]
pub struct IErrorLookup {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetErrorDescription: unsafe extern "system" fn(this: *mut *mut Self, hrerror: ::windows_sys::core::HRESULT, dwlookupid: u32, pdispparams: *const super::Com::DISPPARAMS, lcid: u32, pbstrsource: *mut super::super::Foundation::BSTR, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetErrorDescription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHelpInfo: unsafe extern "system" fn(this: *mut *mut Self, hrerror: ::windows_sys::core::HRESULT, dwlookupid: u32, lcid: u32, pbstrhelpfile: *mut super::super::Foundation::BSTR, pdwhelpcontext: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHelpInfo: usize,
    pub ReleaseErrors: unsafe extern "system" fn(this: *mut *mut Self, dwdynamicerrorid: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IErrorLookup {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878182, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IErrorRecords {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddErrorRecord: unsafe extern "system" fn(this: *mut *mut Self, perrorinfo: *const ERRORINFO, dwlookupid: u32, pdispparams: *const super::Com::DISPPARAMS, punkcustomerror: *mut ::core::ffi::c_void, dwdynamicerrorid: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddErrorRecord: usize,
    pub GetBasicErrorInfo: unsafe extern "system" fn(this: *mut *mut Self, ulrecordnum: u32, perrorinfo: *mut ERRORINFO) -> ::windows_sys::core::HRESULT,
    pub GetCustomErrorObject: unsafe extern "system" fn(this: *mut *mut Self, ulrecordnum: u32, riid: *const ::windows_sys::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetErrorInfo: unsafe extern "system" fn(this: *mut *mut Self, ulrecordnum: u32, lcid: u32, pperrorinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetErrorInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetErrorParameters: unsafe extern "system" fn(this: *mut *mut Self, ulrecordnum: u32, pdispparams: *mut super::Com::DISPPARAMS) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetErrorParameters: usize,
    pub GetRecordCount: unsafe extern "system" fn(this: *mut *mut Self, pcrecords: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IErrorRecords {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878183, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IGetDataSource {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetDataSource: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppdatasource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGetDataSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878197, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IGetRow {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetRowFromHROW: unsafe extern "system" fn(this: *mut *mut Self, punkouter: *mut ::core::ffi::c_void, hrow: usize, riid: *const ::windows_sys::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetURLFromHROW: unsafe extern "system" fn(this: *mut *mut Self, hrow: usize, ppwszurl: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGetRow {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878255, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IGetSession {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetSession: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGetSession {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878266, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IGetSourceRow {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetSourceRow: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, pprow: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGetSourceRow {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878267, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IIndexDefinition {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateIndex: unsafe extern "system" fn(this: *mut *mut Self, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID, cindexcolumndescs: usize, rgindexcolumndescs: *const DBINDEXCOLUMNDESC, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, ppindexid: *mut *mut super::super::Storage::IndexServer::DBID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateIndex: usize,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub DropIndex: unsafe extern "system" fn(this: *mut *mut Self, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    DropIndex: usize,
}
impl ::windows_sys::core::Interface for IIndexDefinition {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878184, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IInterval {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetLimits: unsafe extern "system" fn(this: *mut *mut Self, pilklower: *mut INTERVAL_LIMIT_KIND, ppropvarlower: *mut super::Com::StructuredStorage::PROPVARIANT, pilkupper: *mut INTERVAL_LIMIT_KIND, ppropvarupper: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetLimits: usize,
}
impl ::windows_sys::core::Interface for IInterval {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1810933524, data2: 15384, data3: 17163, data4: [139, 93, 131, 177, 194, 52, 211, 219] };
}
#[repr(C)]
pub struct ILoadFilter {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
    pub LoadIFilter: unsafe extern "system" fn(this: *mut *mut Self, pwcspath: ::windows_sys::core::PCWSTR, pfilteredsources: *const FILTERED_DATA_SOURCES, punkouter: *mut ::core::ffi::c_void, fusedefault: super::super::Foundation::BOOL, pfilterclsid: *mut ::windows_sys::core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer")))]
    LoadIFilter: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
    pub LoadIFilterFromStorage: unsafe extern "system" fn(this: *mut *mut Self, pstg: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, pwcsoverride: ::windows_sys::core::PCWSTR, fusedefault: super::super::Foundation::BOOL, pfilterclsid: *mut ::windows_sys::core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage")))]
    LoadIFilterFromStorage: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
    pub LoadIFilterFromStream: unsafe extern "system" fn(this: *mut *mut Self, pstm: *mut ::core::ffi::c_void, pfilteredsources: *const FILTERED_DATA_SOURCES, punkouter: *mut ::core::ffi::c_void, fusedefault: super::super::Foundation::BOOL, pfilterclsid: *mut ::windows_sys::core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com")))]
    LoadIFilterFromStream: usize,
}
impl ::windows_sys::core::Interface for ILoadFilter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3341879074, data2: 44160, data3: 4561, data4: [141, 243, 0, 192, 79, 182, 239, 79] };
}
#[repr(C)]
pub struct ILoadFilterWithPrivateComActivation {
    pub base__: ILoadFilter,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
    pub LoadIFilterWithPrivateComActivation: unsafe extern "system" fn(this: *mut *mut Self, filteredsources: *const FILTERED_DATA_SOURCES, usedefault: super::super::Foundation::BOOL, filterclsid: *mut ::windows_sys::core::GUID, isfilterprivatecomactivated: *mut super::super::Foundation::BOOL, filterobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer")))]
    LoadIFilterWithPrivateComActivation: usize,
}
impl ::windows_sys::core::Interface for ILoadFilterWithPrivateComActivation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1086176564, data2: 30731, data3: 18643, data4: [155, 182, 18, 235, 212, 173, 46, 117] };
}
#[repr(C)]
pub struct IMDDataset {
    pub base__: ::windows_sys::core::IUnknown,
    pub FreeAxisInfo: unsafe extern "system" fn(this: *mut *mut Self, caxes: usize, rgaxisinfo: *mut MDAXISINFO) -> ::windows_sys::core::HRESULT,
    pub GetAxisInfo: unsafe extern "system" fn(this: *mut *mut Self, pcaxes: *mut usize, prgaxisinfo: *mut *mut MDAXISINFO) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetAxisRowset: unsafe extern "system" fn(this: *mut *mut Self, punkouter: *mut ::core::ffi::c_void, iaxis: usize, riid: *const ::windows_sys::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetAxisRowset: usize,
    pub GetCellData: unsafe extern "system" fn(this: *mut *mut Self, haccessor: usize, ulstartcell: usize, ulendcell: usize, pdata: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetSpecification: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppspecification: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMDDataset {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2692533457, data2: 33096, data3: 4560, data4: [135, 187, 0, 192, 79, 195, 57, 66] };
}
#[repr(C)]
pub struct IMDFind {
    pub base__: ::windows_sys::core::IUnknown,
    pub FindCell: unsafe extern "system" fn(this: *mut *mut Self, ulstartingordinal: usize, cmembers: usize, rgpwszmember: *mut ::windows_sys::core::PWSTR, pulcellordinal: *mut usize) -> ::windows_sys::core::HRESULT,
    pub FindTuple: unsafe extern "system" fn(this: *mut *mut Self, ulaxisidentifier: u32, ulstartingordinal: usize, cmembers: usize, rgpwszmember: *mut ::windows_sys::core::PWSTR, pultupleordinal: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMDFind {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2692533458, data2: 33096, data3: 4560, data4: [135, 187, 0, 192, 79, 195, 57, 66] };
}
#[repr(C)]
pub struct IMDRangeRowset {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetRangeRowset: unsafe extern "system" fn(this: *mut *mut Self, punkouter: *mut ::core::ffi::c_void, ulstartcell: usize, ulendcell: usize, riid: *const ::windows_sys::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetRangeRowset: usize,
}
impl ::windows_sys::core::Interface for IMDRangeRowset {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878240, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IMetaData {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetData: unsafe extern "system" fn(this: *mut *mut Self, ppszkey: *mut ::windows_sys::core::PWSTR, ppszvalue: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMetaData {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2013332144, data2: 50235, data3: 18550, data4: [188, 123, 94, 155, 165, 200, 135, 148] };
}
#[repr(C)]
pub struct IMultipleResults {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetResult: unsafe extern "system" fn(this: *mut *mut Self, punkouter: *mut ::core::ffi::c_void, lresultflag: isize, riid: *const ::windows_sys::core::GUID, pcrowsaffected: *mut isize, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMultipleResults {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878224, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct INCREMENTAL_ACCESS_INFO {
    pub dwSize: u32,
    pub ftLastModifiedTime: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INCREMENTAL_ACCESS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INCREMENTAL_ACCESS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const INET_E_AGENT_CACHE_SIZE_EXCEEDED: ::windows_sys::core::HRESULT = -2146693246i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const INET_E_AGENT_CONNECTION_FAILED: ::windows_sys::core::HRESULT = -2146693245i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const INET_E_AGENT_EXCEEDING_CACHE_SIZE: ::windows_sys::core::HRESULT = -2146693232i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const INET_E_AGENT_MAX_SIZE_EXCEEDED: ::windows_sys::core::HRESULT = -2146693248i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const INET_E_SCHEDULED_EXCLUDE_RANGE: ::windows_sys::core::HRESULT = -2146693241i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const INET_E_SCHEDULED_UPDATES_DISABLED: ::windows_sys::core::HRESULT = -2146693244i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const INET_E_SCHEDULED_UPDATES_RESTRICTED: ::windows_sys::core::HRESULT = -2146693243i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const INET_E_SCHEDULED_UPDATE_INTERVAL: ::windows_sys::core::HRESULT = -2146693242i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const INET_S_AGENT_INCREASED_CACHE_SIZE: ::windows_sys::core::HRESULT = 790416i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const INET_S_AGENT_PART_FAIL: ::windows_sys::core::HRESULT = 790401i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type INTERVAL_LIMIT_KIND = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ILK_EXPLICIT_INCLUDED: INTERVAL_LIMIT_KIND = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ILK_EXPLICIT_EXCLUDED: INTERVAL_LIMIT_KIND = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ILK_NEGATIVE_INFINITY: INTERVAL_LIMIT_KIND = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ILK_POSITIVE_INFINITY: INTERVAL_LIMIT_KIND = 3i32;
#[repr(C)]
pub struct INamedEntity {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetValue: unsafe extern "system" fn(this: *mut *mut Self, ppszvalue: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub DefaultPhrase: unsafe extern "system" fn(this: *mut *mut Self, ppszphrase: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for INamedEntity {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2883309745, data2: 32084, data3: 18939, data4: [171, 92, 191, 244, 19, 0, 4, 205] };
}
#[repr(C)]
pub struct INamedEntityCollector {
    pub base__: ::windows_sys::core::IUnknown,
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, beginspan: u32, endspan: u32, beginactual: u32, endactual: u32, ptype: *mut ::core::ffi::c_void, pszvalue: ::windows_sys::core::PCWSTR, certainty: NAMED_ENTITY_CERTAINTY) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for INamedEntityCollector {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2938388726, data2: 35580, data3: 18384, data4: [154, 127, 57, 106, 10, 207, 180, 61] };
}
#[repr(C)]
pub struct IObjectAccessControl {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer"))]
    pub GetObjectAccessRights: unsafe extern "system" fn(this: *mut *mut Self, pobject: *mut SEC_OBJECT, pcaccessentries: *mut u32, prgaccessentries: *mut *mut super::super::Security::Authorization::EXPLICIT_ACCESS_W) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer")))]
    GetObjectAccessRights: usize,
    #[cfg(all(feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer"))]
    pub GetObjectOwner: unsafe extern "system" fn(this: *mut *mut Self, pobject: *mut SEC_OBJECT, ppowner: *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer")))]
    GetObjectOwner: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer"))]
    pub IsObjectAccessAllowed: unsafe extern "system" fn(this: *mut *mut Self, pobject: *mut SEC_OBJECT, paccessentry: *mut super::super::Security::Authorization::EXPLICIT_ACCESS_W, pfresult: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer")))]
    IsObjectAccessAllowed: usize,
    #[cfg(all(feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer"))]
    pub SetObjectAccessRights: unsafe extern "system" fn(this: *mut *mut Self, pobject: *mut SEC_OBJECT, caccessentries: u32, prgaccessentries: *mut super::super::Security::Authorization::EXPLICIT_ACCESS_W) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer")))]
    SetObjectAccessRights: usize,
    #[cfg(all(feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer"))]
    pub SetObjectOwner: unsafe extern "system" fn(this: *mut *mut Self, pobject: *mut SEC_OBJECT, powner: *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer")))]
    SetObjectOwner: usize,
}
impl ::windows_sys::core::Interface for IObjectAccessControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878243, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IOpLockStatus {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub IsOplockValid: unsafe extern "system" fn(this: *mut *mut Self, pfisoplockvalid: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsOplockValid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsOplockBroken: unsafe extern "system" fn(this: *mut *mut Self, pfisoplockbroken: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsOplockBroken: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetOplockEventHandle: unsafe extern "system" fn(this: *mut *mut Self, phoplockev: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetOplockEventHandle: usize,
}
impl ::windows_sys::core::Interface for IOpLockStatus {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3341878877, data2: 44160, data3: 4561, data4: [141, 243, 0, 192, 79, 182, 239, 79] };
}
#[repr(C)]
pub struct IOpenRowset {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OpenRowset: unsafe extern "system" fn(this: *mut *mut Self, punkouter: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID, riid: *const ::windows_sys::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OpenRowset: usize,
}
impl ::windows_sys::core::Interface for IOpenRowset {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878185, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IParentRowset {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetChildRowset: unsafe extern "system" fn(this: *mut *mut Self, punkouter: *mut ::core::ffi::c_void, iordinal: usize, riid: *const ::windows_sys::core::GUID, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IParentRowset {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878250, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IProtocolHandlerSite {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub GetFilter: unsafe extern "system" fn(this: *mut *mut Self, pclsidobj: *mut ::windows_sys::core::GUID, pcwszcontenttype: ::windows_sys::core::PCWSTR, pcwszextension: ::windows_sys::core::PCWSTR, ppfilter: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    GetFilter: usize,
}
impl ::windows_sys::core::Interface for IProtocolHandlerSite {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 191095685, data2: 40140, data3: 4560, data4: [188, 219, 0, 128, 95, 204, 206, 4] };
}
#[repr(C)]
pub struct IProvideMoniker {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub GetMoniker: unsafe extern "system" fn(this: *mut *mut Self, ppimoniker: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetMoniker: usize,
}
impl ::windows_sys::core::Interface for IProvideMoniker {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878157, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IQueryParser {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, pszinputstring: ::windows_sys::core::PCWSTR, pcustomproperties: *mut ::core::ffi::c_void, ppsolution: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Parse: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub SetOption: unsafe extern "system" fn(this: *mut *mut Self, option: STRUCTURED_QUERY_SINGLE_OPTION, poptionvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    SetOption: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetOption: unsafe extern "system" fn(this: *mut *mut Self, option: STRUCTURED_QUERY_SINGLE_OPTION, poptionvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetOption: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub SetMultiOption: unsafe extern "system" fn(this: *mut *mut Self, option: STRUCTURED_QUERY_MULTIOPTION, pszoptionkey: ::windows_sys::core::PCWSTR, poptionvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    SetMultiOption: usize,
    pub GetSchemaProvider: unsafe extern "system" fn(this: *mut *mut Self, ppschemaprovider: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub RestateToString: unsafe extern "system" fn(this: *mut *mut Self, pcondition: *mut ::core::ffi::c_void, fuseenglish: super::super::Foundation::BOOL, ppszquerystring: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    RestateToString: usize,
    pub ParsePropertyValue: unsafe extern "system" fn(this: *mut *mut Self, pszpropertyname: ::windows_sys::core::PCWSTR, pszinputstring: ::windows_sys::core::PCWSTR, ppsolution: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub RestatePropertyValueToString: unsafe extern "system" fn(this: *mut *mut Self, pcondition: *mut ::core::ffi::c_void, fuseenglish: super::super::Foundation::BOOL, ppszpropertyname: *mut ::windows_sys::core::PWSTR, ppszquerystring: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    RestatePropertyValueToString: usize,
}
impl ::windows_sys::core::Interface for IQueryParser {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 784199271, data2: 13573, data3: 17400, data4: [153, 70, 234, 68, 171, 200, 229, 176] };
}
#[repr(C)]
pub struct IQueryParserManager {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateLoadedParser: unsafe extern "system" fn(this: *mut *mut Self, pszcatalog: ::windows_sys::core::PCWSTR, langidforkeywords: u16, riid: *const ::windows_sys::core::GUID, ppqueryparser: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeOptions: unsafe extern "system" fn(this: *mut *mut Self, funderstandnqs: super::super::Foundation::BOOL, fautowildcard: super::super::Foundation::BOOL, pqueryparser: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeOptions: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub SetOption: unsafe extern "system" fn(this: *mut *mut Self, option: QUERY_PARSER_MANAGER_OPTION, poptionvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    SetOption: usize,
}
impl ::windows_sys::core::Interface for IQueryParserManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2826560452, data2: 44919, data3: 17659, data4: [143, 55, 235, 209, 72, 124, 249, 32] };
}
#[repr(C)]
pub struct IQuerySolution {
    pub base__: IConditionFactory,
    #[cfg(feature = "Win32_System_Com")]
    pub GetQuery: unsafe extern "system" fn(this: *mut *mut Self, ppquerynode: *mut *mut ::core::ffi::c_void, ppmaintype: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetQuery: usize,
    pub GetErrors: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppparseerrors: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetLexicalData: unsafe extern "system" fn(this: *mut *mut Self, ppszinputstring: *mut ::windows_sys::core::PWSTR, pptokens: *mut *mut ::core::ffi::c_void, plcid: *mut u32, ppwordbreaker: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IQuerySolution {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3605775979, data2: 35105, data3: 16787, data4: [175, 221, 161, 120, 159, 183, 255, 87] };
}
#[repr(C)]
pub struct IReadData {
    pub base__: ::windows_sys::core::IUnknown,
    pub ReadData: unsafe extern "system" fn(this: *mut *mut Self, hchapter: usize, cbbookmark: usize, pbookmark: *const u8, lrowsoffset: isize, haccessor: usize, crows: isize, pcrowsobtained: *mut usize, ppfixeddata: *mut *mut u8, pcbvariabletotal: *mut usize, ppvariabledata: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    pub ReleaseChapter: unsafe extern "system" fn(this: *mut *mut Self, hchapter: usize) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IReadData {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878186, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IRegisterProvider {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetURLMapping: unsafe extern "system" fn(this: *mut *mut Self, pwszurl: ::windows_sys::core::PCWSTR, dwreserved: usize, pclsidprovider: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SetURLMapping: unsafe extern "system" fn(this: *mut *mut Self, pwszurl: ::windows_sys::core::PCWSTR, dwreserved: usize, rclsidprovider: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub UnregisterProvider: unsafe extern "system" fn(this: *mut *mut Self, pwszurl: ::windows_sys::core::PCWSTR, dwreserved: usize, rclsidprovider: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRegisterProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878265, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IRelationship {
    pub base__: ::windows_sys::core::IUnknown,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, ppszname: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsReal: unsafe extern "system" fn(this: *mut *mut Self, pisreal: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsReal: usize,
    pub Destination: unsafe extern "system" fn(this: *mut *mut Self, pdestinationentity: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MetaData: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, pmetadata: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DefaultPhrase: unsafe extern "system" fn(this: *mut *mut Self, ppszphrase: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRelationship {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 661202955, data2: 20744, data3: 18828, data4: [156, 127, 165, 18, 57, 182, 49, 71] };
}
#[repr(C)]
pub struct IRichChunk {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetData: unsafe extern "system" fn(this: *mut *mut Self, pfirstpos: *mut u32, plength: *mut u32, ppsz: *mut ::windows_sys::core::PWSTR, pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetData: usize,
}
impl ::windows_sys::core::Interface for IRichChunk {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1340012188, data2: 56265, data3: 17742, data4: [153, 16, 179, 79, 60, 100, 181, 16] };
}
#[repr(C)]
pub struct IRow {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub GetColumns: unsafe extern "system" fn(this: *mut *mut Self, ccolumns: usize, rgcolumns: *mut DBCOLUMNACCESS) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    GetColumns: usize,
    pub GetSourceRowset: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, pprowset: *mut *mut ::core::ffi::c_void, phrow: *mut usize) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub Open: unsafe extern "system" fn(this: *mut *mut Self, punkouter: *mut ::core::ffi::c_void, pcolumnid: *const super::super::Storage::IndexServer::DBID, rguidcolumntype: *const ::windows_sys::core::GUID, dwbindflags: u32, riid: *const ::windows_sys::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    Open: usize,
}
impl ::windows_sys::core::Interface for IRow {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878260, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IRowChange {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub SetColumns: unsafe extern "system" fn(this: *mut *mut Self, ccolumns: usize, rgcolumns: *const DBCOLUMNACCESS) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    SetColumns: usize,
}
impl ::windows_sys::core::Interface for IRowChange {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878261, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IRowPosition {
    pub base__: ::windows_sys::core::IUnknown,
    pub ClearRowPosition: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetRowPosition: unsafe extern "system" fn(this: *mut *mut Self, phchapter: *mut usize, phrow: *mut usize, pdwpositionflags: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetRowset: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, prowset: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRowPosition: unsafe extern "system" fn(this: *mut *mut Self, hchapter: usize, hrow: usize, dwpositionflags: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRowPosition {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878228, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IRowPositionChange {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub OnRowPositionChange: unsafe extern "system" fn(this: *mut *mut Self, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnRowPositionChange: usize,
}
impl ::windows_sys::core::Interface for IRowPositionChange {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 160933233, data2: 4718, data3: 4560, data4: [159, 138, 0, 160, 201, 160, 99, 30] };
}
#[repr(C)]
pub struct IRowSchemaChange {
    pub base__: IRowChange,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub DeleteColumns: unsafe extern "system" fn(this: *mut *mut Self, ccolumns: usize, rgcolumnids: *const super::super::Storage::IndexServer::DBID, rgdwstatus: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    DeleteColumns: usize,
    #[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
    pub AddColumns: unsafe extern "system" fn(this: *mut *mut Self, ccolumns: usize, rgnewcolumninfo: *const DBCOLUMNINFO, rgcolumns: *mut DBCOLUMNACCESS) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com")))]
    AddColumns: usize,
}
impl ::windows_sys::core::Interface for IRowSchemaChange {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878254, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IRowset {
    pub base__: ::windows_sys::core::IUnknown,
    pub AddRefRows: unsafe extern "system" fn(this: *mut *mut Self, crows: usize, rghrows: *const usize, rgrefcounts: *mut u32, rgrowstatus: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetData: unsafe extern "system" fn(this: *mut *mut Self, hrow: usize, haccessor: usize, pdata: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetNextRows: unsafe extern "system" fn(this: *mut *mut Self, hreserved: usize, lrowsoffset: isize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> ::windows_sys::core::HRESULT,
    pub ReleaseRows: unsafe extern "system" fn(this: *mut *mut Self, crows: usize, rghrows: *const usize, rgrowoptions: *mut u32, rgrefcounts: *mut u32, rgrowstatus: *mut u32) -> ::windows_sys::core::HRESULT,
    pub RestartPosition: unsafe extern "system" fn(this: *mut *mut Self, hreserved: usize) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRowset {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878204, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IRowsetAsynch {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub RatioFinished: unsafe extern "system" fn(this: *mut *mut Self, puldenominator: *mut usize, pulnumerator: *mut usize, pcrows: *mut usize, pfnewrows: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RatioFinished: usize,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRowsetAsynch {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878095, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IRowsetBookmark {
    pub base__: ::windows_sys::core::IUnknown,
    pub PositionOnBookmark: unsafe extern "system" fn(this: *mut *mut Self, hchapter: usize, cbbookmark: usize, pbookmark: *const u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRowsetBookmark {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878274, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IRowsetChange {
    pub base__: ::windows_sys::core::IUnknown,
    pub DeleteRows: unsafe extern "system" fn(this: *mut *mut Self, hreserved: usize, crows: usize, rghrows: *const usize, rgrowstatus: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetData: unsafe extern "system" fn(this: *mut *mut Self, hrow: usize, haccessor: usize, pdata: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InsertRow: unsafe extern "system" fn(this: *mut *mut Self, hreserved: usize, haccessor: usize, pdata: *mut ::core::ffi::c_void, phrow: *mut usize) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRowsetChange {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878085, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IRowsetChangeExtInfo {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetOriginalRow: unsafe extern "system" fn(this: *mut *mut Self, hreserved: usize, hrow: usize, phroworiginal: *mut usize) -> ::windows_sys::core::HRESULT,
    pub GetPendingColumns: unsafe extern "system" fn(this: *mut *mut Self, hreserved: usize, hrow: usize, ccolumnordinals: u32, rgiordinals: *const u32, rgcolumnstatus: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRowsetChangeExtInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878223, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IRowsetChapterMember {
    pub base__: ::windows_sys::core::IUnknown,
    pub IsRowInChapter: unsafe extern "system" fn(this: *mut *mut Self, hchapter: usize, hrow: usize) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRowsetChapterMember {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878248, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IRowsetCopyRows {
    pub base__: ::windows_sys::core::IUnknown,
    pub CloseSource: unsafe extern "system" fn(this: *mut *mut Self, hsourceid: u16) -> ::windows_sys::core::HRESULT,
    pub CopyByHROWS: unsafe extern "system" fn(this: *mut *mut Self, hsourceid: u16, hreserved: usize, crows: isize, rghrows: *const usize, bflags: u32) -> ::windows_sys::core::HRESULT,
    pub CopyRows: unsafe extern "system" fn(this: *mut *mut Self, hsourceid: u16, hreserved: usize, crows: isize, bflags: u32, pcrowscopied: *mut usize) -> ::windows_sys::core::HRESULT,
    pub DefineSource: unsafe extern "system" fn(this: *mut *mut Self, prowsetsource: *mut ::core::ffi::c_void, ccolids: usize, rgsourcecolumns: *const isize, rgtargetcolumns: *const isize, phsourceid: *mut u16) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRowsetCopyRows {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878187, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IRowsetCurrentIndex {
    pub base__: IRowsetIndex,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub GetIndex: unsafe extern "system" fn(this: *mut *mut Self, ppindexid: *mut *mut super::super::Storage::IndexServer::DBID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    GetIndex: usize,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub SetIndex: unsafe extern "system" fn(this: *mut *mut Self, pindexid: *mut super::super::Storage::IndexServer::DBID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    SetIndex: usize,
}
impl ::windows_sys::core::Interface for IRowsetCurrentIndex {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878269, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IRowsetEvents {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub OnNewItem: unsafe extern "system" fn(this: *mut *mut Self, itemid: *const super::Com::StructuredStorage::PROPVARIANT, newitemstate: ROWSETEVENT_ITEMSTATE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    OnNewItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub OnChangedItem: unsafe extern "system" fn(this: *mut *mut Self, itemid: *const super::Com::StructuredStorage::PROPVARIANT, rowsetitemstate: ROWSETEVENT_ITEMSTATE, changeditemstate: ROWSETEVENT_ITEMSTATE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    OnChangedItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub OnDeletedItem: unsafe extern "system" fn(this: *mut *mut Self, itemid: *const super::Com::StructuredStorage::PROPVARIANT, deleteditemstate: ROWSETEVENT_ITEMSTATE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    OnDeletedItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub OnRowsetEvent: unsafe extern "system" fn(this: *mut *mut Self, eventtype: ROWSETEVENT_TYPE, eventdata: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    OnRowsetEvent: usize,
}
impl ::windows_sys::core::Interface for IRowsetEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 357674661, data2: 23910, data3: 19217, data4: [134, 245, 213, 99, 76, 178, 17, 185] };
}
#[repr(C)]
pub struct IRowsetExactScroll(pub u8);
#[repr(C)]
pub struct IRowsetFastLoad {
    pub base__: ::windows_sys::core::IUnknown,
    pub InsertRow: unsafe extern "system" fn(this: *mut *mut Self, haccessor: usize, pdata: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Commit: unsafe extern "system" fn(this: *mut *mut Self, fdone: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Commit: usize,
}
impl ::windows_sys::core::Interface for IRowsetFastLoad {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1559546387, data2: 61217, data3: 4560, data4: [151, 231, 0, 192, 79, 194, 173, 152] };
}
#[repr(C)]
pub struct IRowsetFind {
    pub base__: ::windows_sys::core::IUnknown,
    pub FindNextRow: unsafe extern "system" fn(this: *mut *mut Self, hchapter: usize, haccessor: usize, pfindvalue: *mut ::core::ffi::c_void, compareop: u32, cbbookmark: usize, pbookmark: *const u8, lrowsoffset: isize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRowsetFind {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878237, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IRowsetIdentity {
    pub base__: ::windows_sys::core::IUnknown,
    pub IsSameRow: unsafe extern "system" fn(this: *mut *mut Self, hthisrow: usize, hthatrow: usize) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRowsetIdentity {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878089, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IRowsetIndex {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetIndexInfo: unsafe extern "system" fn(this: *mut *mut Self, pckeycolumns: *mut usize, prgindexcolumndesc: *mut *mut DBINDEXCOLUMNDESC, pcindexpropertysets: *mut u32, prgindexpropertysets: *mut *mut DBPROPSET) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetIndexInfo: usize,
    pub Seek: unsafe extern "system" fn(this: *mut *mut Self, haccessor: usize, ckeyvalues: usize, pdata: *mut ::core::ffi::c_void, dwseekoptions: u32) -> ::windows_sys::core::HRESULT,
    pub SetRange: unsafe extern "system" fn(this: *mut *mut Self, haccessor: usize, cstartkeycolumns: usize, pstartdata: *mut ::core::ffi::c_void, cendkeycolumns: usize, penddata: *mut ::core::ffi::c_void, dwrangeoptions: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRowsetIndex {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878210, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IRowsetInfo {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperties: unsafe extern "system" fn(this: *mut *mut Self, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperties: usize,
    pub GetReferencedRowset: unsafe extern "system" fn(this: *mut *mut Self, iordinal: usize, riid: *const ::windows_sys::core::GUID, ppreferencedrowset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetSpecification: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppspecification: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRowsetInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878165, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IRowsetKeys {
    pub base__: ::windows_sys::core::IUnknown,
    pub ListKeys: unsafe extern "system" fn(this: *mut *mut Self, pccolumns: *mut usize, prgcolumns: *mut *mut usize) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRowsetKeys {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878098, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IRowsetLocate {
    pub base__: IRowset,
    pub Compare: unsafe extern "system" fn(this: *mut *mut Self, hreserved: usize, cbbookmark1: usize, pbookmark1: *const u8, cbbookmark2: usize, pbookmark2: *const u8, pcomparison: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetRowsAt: unsafe extern "system" fn(this: *mut *mut Self, hreserved1: usize, hreserved2: usize, cbbookmark: usize, pbookmark: *const u8, lrowsoffset: isize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> ::windows_sys::core::HRESULT,
    pub GetRowsByBookmark: unsafe extern "system" fn(this: *mut *mut Self, hreserved: usize, crows: usize, rgcbbookmarks: *const usize, rgpbookmarks: *const *const u8, rghrows: *mut usize, rgrowstatus: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Hash: unsafe extern "system" fn(this: *mut *mut Self, hreserved: usize, cbookmarks: usize, rgcbbookmarks: *const usize, rgpbookmarks: *const *const u8, rghashedvalues: *mut usize, rgbookmarkstatus: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRowsetLocate {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878205, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IRowsetNewRowAfter {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetNewDataAfter: unsafe extern "system" fn(this: *mut *mut Self, hchapter: usize, cbbmprevious: u32, pbmprevious: *const u8, haccessor: usize, pdata: *mut u8, phrow: *mut usize) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRowsetNewRowAfter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878193, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IRowsetNextRowset {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetNextRowset: unsafe extern "system" fn(this: *mut *mut Self, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows_sys::core::GUID, ppnextrowset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRowsetNextRowset {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878194, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IRowsetNotify {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub OnFieldChange: unsafe extern "system" fn(this: *mut *mut Self, prowset: *mut ::core::ffi::c_void, hrow: usize, ccolumns: usize, rgcolumns: *const usize, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnFieldChange: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnRowChange: unsafe extern "system" fn(this: *mut *mut Self, prowset: *mut ::core::ffi::c_void, crows: usize, rghrows: *const usize, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnRowChange: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnRowsetChange: unsafe extern "system" fn(this: *mut *mut Self, prowset: *mut ::core::ffi::c_void, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnRowsetChange: usize,
}
impl ::windows_sys::core::Interface for IRowsetNotify {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878211, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IRowsetPrioritization {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetScopePriority: unsafe extern "system" fn(this: *mut *mut Self, priority: PRIORITY_LEVEL, scopestatisticseventfrequency: u32) -> ::windows_sys::core::HRESULT,
    pub GetScopePriority: unsafe extern "system" fn(this: *mut *mut Self, priority: *mut PRIORITY_LEVEL, scopestatisticseventfrequency: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetScopeStatistics: unsafe extern "system" fn(this: *mut *mut Self, indexeddocumentcount: *mut u32, oustandingaddcount: *mut u32, oustandingmodifycount: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRowsetPrioritization {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1115756114, data2: 1949, data3: 18459, data4: [135, 162, 9, 166, 158, 204, 95, 68] };
}
#[repr(C)]
pub struct IRowsetQueryStatus {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, pdwstatus: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetStatusEx: unsafe extern "system" fn(this: *mut *mut Self, pdwstatus: *mut u32, pcfiltereddocuments: *mut u32, pcdocumentstofilter: *mut u32, pdwratiofinisheddenominator: *mut usize, pdwratiofinishednumerator: *mut usize, cbbmk: usize, pbmk: *const u8, pirowbmk: *mut usize, pcrowstotal: *mut usize) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRowsetQueryStatus {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2813097965, data2: 63703, data3: 4558, data4: [167, 152, 0, 32, 248, 0, 128, 36] };
}
#[repr(C)]
pub struct IRowsetRefresh {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub RefreshVisibleData: unsafe extern "system" fn(this: *mut *mut Self, hchapter: usize, crows: usize, rghrows: *const usize, foverwrite: super::super::Foundation::BOOL, pcrowsrefreshed: *mut usize, prghrowsrefreshed: *mut *mut usize, prgrowstatus: *mut *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RefreshVisibleData: usize,
    pub GetLastVisibleData: unsafe extern "system" fn(this: *mut *mut Self, hrow: usize, haccessor: usize, pdata: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRowsetRefresh {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878249, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IRowsetResynch {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetVisibleData: unsafe extern "system" fn(this: *mut *mut Self, hrow: usize, haccessor: usize, pdata: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ResynchRows: unsafe extern "system" fn(this: *mut *mut Self, crows: usize, rghrows: *const usize, pcrowsresynched: *mut usize, prghrowsresynched: *mut *mut usize, prgrowstatus: *mut *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRowsetResynch {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878212, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IRowsetScroll {
    pub base__: IRowsetLocate,
    pub GetApproximatePosition: unsafe extern "system" fn(this: *mut *mut Self, hreserved: usize, cbbookmark: usize, pbookmark: *const u8, pulposition: *mut usize, pcrows: *mut usize) -> ::windows_sys::core::HRESULT,
    pub GetRowsAtRatio: unsafe extern "system" fn(this: *mut *mut Self, hreserved1: usize, hreserved2: usize, ulnumerator: usize, uldenominator: usize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRowsetScroll {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878206, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IRowsetUpdate {
    pub base__: IRowsetChange,
    pub GetOriginalData: unsafe extern "system" fn(this: *mut *mut Self, hrow: usize, haccessor: usize, pdata: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetPendingRows: unsafe extern "system" fn(this: *mut *mut Self, hreserved: usize, dwrowstatus: u32, pcpendingrows: *mut usize, prgpendingrows: *mut *mut usize, prgpendingstatus: *mut *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetRowStatus: unsafe extern "system" fn(this: *mut *mut Self, hreserved: usize, crows: usize, rghrows: *const usize, rgpendingstatus: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Undo: unsafe extern "system" fn(this: *mut *mut Self, hreserved: usize, crows: usize, rghrows: *const usize, pcrowsundone: *mut usize, prgrowsundone: *mut *mut usize, prgrowstatus: *mut *mut u32) -> ::windows_sys::core::HRESULT,
    pub Update: unsafe extern "system" fn(this: *mut *mut Self, hreserved: usize, crows: usize, rghrows: *const usize, pcrows: *mut usize, prgrows: *mut *mut usize, prgrowstatus: *mut *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRowsetUpdate {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878189, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IRowsetView {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateView: unsafe extern "system" fn(this: *mut *mut Self, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows_sys::core::GUID, ppview: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetView: unsafe extern "system" fn(this: *mut *mut Self, hchapter: usize, riid: *const ::windows_sys::core::GUID, phchaptersource: *mut usize, ppview: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRowsetView {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878233, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IRowsetWatchAll {
    pub base__: ::windows_sys::core::IUnknown,
    pub Acknowledge: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub StopWatching: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRowsetWatchAll {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878195, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IRowsetWatchNotify {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnChange: unsafe extern "system" fn(this: *mut *mut Self, prowset: *mut ::core::ffi::c_void, echangereason: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRowsetWatchNotify {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878148, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IRowsetWatchRegion {
    pub base__: IRowsetWatchAll,
    pub CreateWatchRegion: unsafe extern "system" fn(this: *mut *mut Self, dwwatchmode: u32, phregion: *mut usize) -> ::windows_sys::core::HRESULT,
    pub ChangeWatchMode: unsafe extern "system" fn(this: *mut *mut Self, hregion: usize, dwwatchmode: u32) -> ::windows_sys::core::HRESULT,
    pub DeleteWatchRegion: unsafe extern "system" fn(this: *mut *mut Self, hregion: usize) -> ::windows_sys::core::HRESULT,
    pub GetWatchRegionInfo: unsafe extern "system" fn(this: *mut *mut Self, hregion: usize, pdwwatchmode: *mut u32, phchapter: *mut usize, pcbbookmark: *mut usize, ppbookmark: *mut *mut u8, pcrows: *mut isize) -> ::windows_sys::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self, pcchangesobtained: *mut usize, prgchanges: *mut *mut tagDBROWWATCHRANGE) -> ::windows_sys::core::HRESULT,
    pub ShrinkWatchRegion: unsafe extern "system" fn(this: *mut *mut Self, hregion: usize, hchapter: usize, cbbookmark: usize, pbookmark: *mut u8, crows: isize) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRowsetWatchRegion {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878149, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IRowsetWithParameters {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub GetParameterInfo: unsafe extern "system" fn(this: *mut *mut Self, pcparams: *mut usize, prgparaminfo: *mut *mut DBPARAMINFO, ppnamesbuffer: *mut *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetParameterInfo: usize,
    pub Requery: unsafe extern "system" fn(this: *mut *mut Self, pparams: *mut DBPARAMS, pulerrorparam: *mut u32, phreserved: *mut usize) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRowsetWithParameters {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878190, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct ISQLErrorInfo {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSQLInfo: unsafe extern "system" fn(this: *mut *mut Self, pbstrsqlstate: *mut super::super::Foundation::BSTR, plnativeerror: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSQLInfo: usize,
}
impl ::windows_sys::core::Interface for ISQLErrorInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878196, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct ISQLGetDiagField {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetDiagField: unsafe extern "system" fn(this: *mut *mut Self, pdiaginfo: *mut KAGGETDIAG) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetDiagField: usize,
}
impl ::windows_sys::core::Interface for ISQLGetDiagField {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 579433201, data2: 46591, data3: 4560, data4: [138, 128, 0, 192, 79, 214, 17, 205] };
}
#[repr(C)]
pub struct ISQLRequestDiagFields {
    pub base__: ::windows_sys::core::IUnknown,
    pub RequestDiagFields: unsafe extern "system" fn(this: *mut *mut Self, cdiagfields: u32, rgdiagfields: *const KAGREQDIAG) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISQLRequestDiagFields {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 579433200, data2: 46591, data3: 4560, data4: [138, 128, 0, 192, 79, 214, 17, 205] };
}
#[repr(C)]
pub struct ISQLServerErrorInfo {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetErrorInfo: unsafe extern "system" fn(this: *mut *mut Self, pperrorinfo: *mut *mut tagSSErrorInfo, ppstringsbuffer: *mut *mut u16) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISQLServerErrorInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1559546386, data2: 61217, data3: 4560, data4: [151, 231, 0, 192, 79, 194, 173, 152] };
}
#[repr(C)]
pub struct ISchemaLocalizerSupport {
    pub base__: ::windows_sys::core::IUnknown,
    pub Localize: unsafe extern "system" fn(this: *mut *mut Self, pszglobalstring: ::windows_sys::core::PCWSTR, ppszlocalstring: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISchemaLocalizerSupport {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3393182882, data2: 49086, data3: 20205, data4: [144, 215, 12, 174, 240, 161, 189, 161] };
}
#[repr(C)]
pub struct ISchemaLock {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
    pub GetSchemaLock: unsafe extern "system" fn(this: *mut *mut Self, ptableid: *mut super::super::Storage::IndexServer::DBID, lmmode: u32, phlockhandle: *mut super::super::Foundation::HANDLE, ptableversion: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer")))]
    GetSchemaLock: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReleaseSchemaLock: unsafe extern "system" fn(this: *mut *mut Self, hlockhandle: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReleaseSchemaLock: usize,
}
impl ::windows_sys::core::Interface for ISchemaLock {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1277397499, data2: 9489, data3: 4564, data4: [178, 88, 0, 192, 79, 121, 113, 206] };
}
#[repr(C)]
pub struct ISchemaProvider {
    pub base__: ::windows_sys::core::IUnknown,
    pub Entities: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, pentities: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RootEntity: unsafe extern "system" fn(this: *mut *mut Self, prootentity: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetEntity: unsafe extern "system" fn(this: *mut *mut Self, pszentityname: ::windows_sys::core::PCWSTR, pentity: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MetaData: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, pmetadata: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Localize: unsafe extern "system" fn(this: *mut *mut Self, lcid: u32, pschemalocalizersupport: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SaveBinary: unsafe extern "system" fn(this: *mut *mut Self, pszschemabinarypath: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub LookupAuthoredNamedEntity: unsafe extern "system" fn(this: *mut *mut Self, pentity: *mut ::core::ffi::c_void, pszinputstring: ::windows_sys::core::PCWSTR, ptokencollection: *mut ::core::ffi::c_void, ctokensbegin: u32, pctokenslength: *mut u32, ppszvalue: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISchemaProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2365103051, data2: 14668, data3: 18866, data4: [174, 40, 165, 157, 212, 237, 127, 104] };
}
#[repr(C)]
pub struct IScopedOperations {
    pub base__: IBindResource,
    #[cfg(feature = "Win32_System_Com")]
    pub Copy: unsafe extern "system" fn(this: *mut *mut Self, crows: usize, rgpwszsourceurls: *const ::windows_sys::core::PWSTR, rgpwszdesturls: *const ::windows_sys::core::PWSTR, dwcopyflags: u32, pauthenticate: *mut ::core::ffi::c_void, rgdwstatus: *mut u32, rgpwsznewurls: *mut ::windows_sys::core::PWSTR, ppstringsbuffer: *mut *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Copy: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Move: unsafe extern "system" fn(this: *mut *mut Self, crows: usize, rgpwszsourceurls: *const ::windows_sys::core::PWSTR, rgpwszdesturls: *const ::windows_sys::core::PWSTR, dwmoveflags: u32, pauthenticate: *mut ::core::ffi::c_void, rgdwstatus: *mut u32, rgpwsznewurls: *mut ::windows_sys::core::PWSTR, ppstringsbuffer: *mut *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Move: usize,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self, crows: usize, rgpwszurls: *const ::windows_sys::core::PWSTR, dwdeleteflags: u32, rgdwstatus: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OpenRowset: unsafe extern "system" fn(this: *mut *mut Self, punkouter: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID, riid: *const ::windows_sys::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OpenRowset: usize,
}
impl ::windows_sys::core::Interface for IScopedOperations {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878256, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct ISearchCatalogManager {
    pub base__: ::windows_sys::core::IUnknown,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pszname: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetParameter: unsafe extern "system" fn(this: *mut *mut Self, pszname: ::windows_sys::core::PCWSTR, ppvalue: *mut *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetParameter: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub SetParameter: unsafe extern "system" fn(this: *mut *mut Self, pszname: ::windows_sys::core::PCWSTR, pvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    SetParameter: usize,
    pub GetCatalogStatus: unsafe extern "system" fn(this: *mut *mut Self, pstatus: *mut CatalogStatus, ppausedreason: *mut CatalogPausedReason) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Reindex: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ReindexMatchingURLs: unsafe extern "system" fn(this: *mut *mut Self, pszpattern: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub ReindexSearchRoot: unsafe extern "system" fn(this: *mut *mut Self, pszrooturl: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub SetConnectTimeout: unsafe extern "system" fn(this: *mut *mut Self, dwconnecttimeout: u32) -> ::windows_sys::core::HRESULT,
    pub ConnectTimeout: unsafe extern "system" fn(this: *mut *mut Self, pdwconnecttimeout: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetDataTimeout: unsafe extern "system" fn(this: *mut *mut Self, dwdatatimeout: u32) -> ::windows_sys::core::HRESULT,
    pub DataTimeout: unsafe extern "system" fn(this: *mut *mut Self, pdwdatatimeout: *mut u32) -> ::windows_sys::core::HRESULT,
    pub NumberOfItems: unsafe extern "system" fn(this: *mut *mut Self, plcount: *mut i32) -> ::windows_sys::core::HRESULT,
    pub NumberOfItemsToIndex: unsafe extern "system" fn(this: *mut *mut Self, plincrementalcount: *mut i32, plnotificationqueue: *mut i32, plhighpriorityqueue: *mut i32) -> ::windows_sys::core::HRESULT,
    pub URLBeingIndexed: unsafe extern "system" fn(this: *mut *mut Self, pszurl: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetURLIndexingState: unsafe extern "system" fn(this: *mut *mut Self, pszurl: ::windows_sys::core::PCWSTR, pdwstate: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetPersistentItemsChangedSink: unsafe extern "system" fn(this: *mut *mut Self, ppisearchpersistentitemschangedsink: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RegisterViewForNotification: unsafe extern "system" fn(this: *mut *mut Self, pszview: ::windows_sys::core::PCWSTR, pviewchangedsink: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetItemsChangedSink: unsafe extern "system" fn(this: *mut *mut Self, pisearchnotifyinlinesite: *mut ::core::ffi::c_void, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void, pguidcatalogresetsignature: *mut ::windows_sys::core::GUID, pguidcheckpointsignature: *mut ::windows_sys::core::GUID, pdwlastcheckpointnumber: *mut u32) -> ::windows_sys::core::HRESULT,
    pub UnregisterViewForNotification: unsafe extern "system" fn(this: *mut *mut Self, dwcookie: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetExtensionClusion: unsafe extern "system" fn(this: *mut *mut Self, pszextension: ::windows_sys::core::PCWSTR, fexclude: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetExtensionClusion: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumerateExcludedExtensions: unsafe extern "system" fn(this: *mut *mut Self, ppextensions: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumerateExcludedExtensions: usize,
    pub GetQueryHelper: unsafe extern "system" fn(this: *mut *mut Self, ppsearchqueryhelper: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDiacriticSensitivity: unsafe extern "system" fn(this: *mut *mut Self, fdiacriticsensitive: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDiacriticSensitivity: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DiacriticSensitivity: unsafe extern "system" fn(this: *mut *mut Self, pfdiacriticsensitive: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DiacriticSensitivity: usize,
    pub GetCrawlScopeManager: unsafe extern "system" fn(this: *mut *mut Self, ppcrawlscopemanager: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISearchCatalogManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2872116609, data2: 44160, data3: 4561, data4: [141, 243, 0, 192, 79, 182, 239, 80] };
}
#[repr(C)]
pub struct ISearchCatalogManager2 {
    pub base__: ISearchCatalogManager,
    pub PrioritizeMatchingURLs: unsafe extern "system" fn(this: *mut *mut Self, pszpattern: ::windows_sys::core::PCWSTR, dwprioritizeflags: PRIORITIZE_FLAGS) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISearchCatalogManager2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2059610221, data2: 19741, data3: 18455, data4: [132, 252, 193, 200, 94, 58, 240, 217] };
}
#[repr(C)]
pub struct ISearchCrawlScopeManager {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub AddDefaultScopeRule: unsafe extern "system" fn(this: *mut *mut Self, pszurl: ::windows_sys::core::PCWSTR, finclude: super::super::Foundation::BOOL, ffollowflags: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddDefaultScopeRule: usize,
    pub AddRoot: unsafe extern "system" fn(this: *mut *mut Self, psearchroot: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveRoot: unsafe extern "system" fn(this: *mut *mut Self, pszurl: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub EnumerateRoots: unsafe extern "system" fn(this: *mut *mut Self, ppsearchroots: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddHierarchicalScope: unsafe extern "system" fn(this: *mut *mut Self, pszurl: ::windows_sys::core::PCWSTR, finclude: super::super::Foundation::BOOL, fdefault: super::super::Foundation::BOOL, foverridechildren: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddHierarchicalScope: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AddUserScopeRule: unsafe extern "system" fn(this: *mut *mut Self, pszurl: ::windows_sys::core::PCWSTR, finclude: super::super::Foundation::BOOL, foverridechildren: super::super::Foundation::BOOL, ffollowflags: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddUserScopeRule: usize,
    pub RemoveScopeRule: unsafe extern "system" fn(this: *mut *mut Self, pszrule: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub EnumerateScopeRules: unsafe extern "system" fn(this: *mut *mut Self, ppsearchscoperules: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub HasParentScopeRule: unsafe extern "system" fn(this: *mut *mut Self, pszurl: ::windows_sys::core::PCWSTR, pfhasparentrule: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasParentScopeRule: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub HasChildScopeRule: unsafe extern "system" fn(this: *mut *mut Self, pszurl: ::windows_sys::core::PCWSTR, pfhaschildrule: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasChildScopeRule: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IncludedInCrawlScope: unsafe extern "system" fn(this: *mut *mut Self, pszurl: ::windows_sys::core::PCWSTR, pfisincluded: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IncludedInCrawlScope: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IncludedInCrawlScopeEx: unsafe extern "system" fn(this: *mut *mut Self, pszurl: ::windows_sys::core::PCWSTR, pfisincluded: *mut super::super::Foundation::BOOL, preason: *mut CLUSION_REASON) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IncludedInCrawlScopeEx: usize,
    pub RevertToDefaultScopes: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SaveAll: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetParentScopeVersionId: unsafe extern "system" fn(this: *mut *mut Self, pszurl: ::windows_sys::core::PCWSTR, plscopeid: *mut i32) -> ::windows_sys::core::HRESULT,
    pub RemoveDefaultScopeRule: unsafe extern "system" fn(this: *mut *mut Self, pszurl: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISearchCrawlScopeManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2872116609, data2: 44160, data3: 4561, data4: [141, 243, 0, 192, 79, 182, 239, 85] };
}
#[repr(C)]
pub struct ISearchCrawlScopeManager2 {
    pub base__: ISearchCrawlScopeManager,
    #[cfg(feature = "Win32_Foundation")]
    pub GetVersion: unsafe extern "system" fn(this: *mut *mut Self, plversion: *mut *mut i32, phfilemapping: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetVersion: usize,
}
impl ::windows_sys::core::Interface for ISearchCrawlScopeManager2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1653798829, data2: 19993, data3: 18199, data4: [165, 52, 143, 194, 43, 205, 92, 205] };
}
#[repr(C)]
pub struct ISearchItemsChangedSink {
    pub base__: ::windows_sys::core::IUnknown,
    pub StartedMonitoringScope: unsafe extern "system" fn(this: *mut *mut Self, pszurl: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub StoppedMonitoringScope: unsafe extern "system" fn(this: *mut *mut Self, pszurl: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OnItemsChanged: unsafe extern "system" fn(this: *mut *mut Self, dwnumberofchanges: u32, rgdatachangeentries: *const SEARCH_ITEM_CHANGE, rgdwdocids: *mut u32, rghrcompletioncodes: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnItemsChanged: usize,
}
impl ::windows_sys::core::Interface for ISearchItemsChangedSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2872116609, data2: 44160, data3: 4561, data4: [141, 243, 0, 192, 79, 182, 239, 88] };
}
#[repr(C)]
pub struct ISearchLanguageSupport {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDiacriticSensitivity: unsafe extern "system" fn(this: *mut *mut Self, fdiacriticsensitive: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDiacriticSensitivity: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDiacriticSensitivity: unsafe extern "system" fn(this: *mut *mut Self, pfdiacriticsensitive: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDiacriticSensitivity: usize,
    pub LoadWordBreaker: unsafe extern "system" fn(this: *mut *mut Self, lcid: u32, riid: *const ::windows_sys::core::GUID, ppwordbreaker: *mut *mut ::core::ffi::c_void, plcidused: *mut u32) -> ::windows_sys::core::HRESULT,
    pub LoadStemmer: unsafe extern "system" fn(this: *mut *mut Self, lcid: u32, riid: *const ::windows_sys::core::GUID, ppstemmer: *mut *mut ::core::ffi::c_void, plcidused: *mut u32) -> ::windows_sys::core::HRESULT,
    pub IsPrefixNormalized: unsafe extern "system" fn(this: *mut *mut Self, pwcsquerytoken: ::windows_sys::core::PCWSTR, cwcquerytoken: u32, pwcsdocumenttoken: ::windows_sys::core::PCWSTR, cwcdocumenttoken: u32, pulprefixlength: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISearchLanguageSupport {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 616811434, data2: 60353, data3: 18714, data4: [158, 241, 159, 109, 141, 235, 27, 143] };
}
#[repr(C)]
pub struct ISearchManager {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetIndexerVersionStr: unsafe extern "system" fn(this: *mut *mut Self, ppszversionstring: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetIndexerVersion: unsafe extern "system" fn(this: *mut *mut Self, pdwmajor: *mut u32, pdwminor: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetParameter: unsafe extern "system" fn(this: *mut *mut Self, pszname: ::windows_sys::core::PCWSTR, ppvalue: *mut *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetParameter: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub SetParameter: unsafe extern "system" fn(this: *mut *mut Self, pszname: ::windows_sys::core::PCWSTR, pvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    SetParameter: usize,
    pub ProxyName: unsafe extern "system" fn(this: *mut *mut Self, ppszproxyname: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub BypassList: unsafe extern "system" fn(this: *mut *mut Self, ppszbypasslist: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetProxy: unsafe extern "system" fn(this: *mut *mut Self, suseproxy: PROXY_ACCESS, flocalbypassproxy: super::super::Foundation::BOOL, dwportnumber: u32, pszproxyname: ::windows_sys::core::PCWSTR, pszbypasslist: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetProxy: usize,
    pub GetCatalog: unsafe extern "system" fn(this: *mut *mut Self, pszcatalog: ::windows_sys::core::PCWSTR, ppcatalogmanager: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UserAgent: unsafe extern "system" fn(this: *mut *mut Self, ppszuseragent: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetUserAgent: unsafe extern "system" fn(this: *mut *mut Self, pszuseragent: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub UseProxy: unsafe extern "system" fn(this: *mut *mut Self, puseproxy: *mut PROXY_ACCESS) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LocalBypass: unsafe extern "system" fn(this: *mut *mut Self, pflocalbypass: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LocalBypass: usize,
    pub PortNumber: unsafe extern "system" fn(this: *mut *mut Self, pdwportnumber: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISearchManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2872116609, data2: 44160, data3: 4561, data4: [141, 243, 0, 192, 79, 182, 239, 105] };
}
#[repr(C)]
pub struct ISearchManager2 {
    pub base__: ISearchManager,
    pub CreateCatalog: unsafe extern "system" fn(this: *mut *mut Self, pszcatalog: ::windows_sys::core::PCWSTR, ppcatalogmanager: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DeleteCatalog: unsafe extern "system" fn(this: *mut *mut Self, pszcatalog: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISearchManager2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3685433203, data2: 56089, data3: 19065, data4: [191, 192, 166, 26, 147, 136, 109, 223] };
}
#[repr(C)]
pub struct ISearchNotifyInlineSite {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnItemIndexedStatusChange: unsafe extern "system" fn(this: *mut *mut Self, sipstatus: SEARCH_INDEXING_PHASE, dwnumentries: u32, rgitemstatusentries: *const SEARCH_ITEM_INDEXING_STATUS) -> ::windows_sys::core::HRESULT,
    pub OnCatalogStatusChange: unsafe extern "system" fn(this: *mut *mut Self, guidcatalogresetsignature: *const ::windows_sys::core::GUID, guidcheckpointsignature: *const ::windows_sys::core::GUID, dwlastcheckpointnumber: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISearchNotifyInlineSite {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3044028001, data2: 59228, data3: 19300, data4: [130, 161, 108, 180, 248, 50, 252, 207] };
}
#[repr(C)]
pub struct ISearchPersistentItemsChangedSink {
    pub base__: ::windows_sys::core::IUnknown,
    pub StartedMonitoringScope: unsafe extern "system" fn(this: *mut *mut Self, pszurl: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub StoppedMonitoringScope: unsafe extern "system" fn(this: *mut *mut Self, pszurl: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub OnItemsChanged: unsafe extern "system" fn(this: *mut *mut Self, dwnumberofchanges: u32, datachangeentries: *const SEARCH_ITEM_PERSISTENT_CHANGE, hrcompletioncodes: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISearchPersistentItemsChangedSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2734677915, data2: 18264, data3: 20356, data4: [183, 41, 223, 129, 161, 160, 97, 47] };
}
#[repr(C)]
pub struct ISearchProtocol {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Init: unsafe extern "system" fn(this: *mut *mut Self, ptimeoutinfo: *mut TIMEOUT_INFO, pprotocolhandlersite: *mut ::core::ffi::c_void, pproxyinfo: *mut PROXY_INFO) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Init: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateAccessor: unsafe extern "system" fn(this: *mut *mut Self, pcwszurl: ::windows_sys::core::PCWSTR, pauthenticationinfo: *mut AUTHENTICATION_INFO, pincrementalaccessinfo: *mut INCREMENTAL_ACCESS_INFO, piteminfo: *mut ITEM_INFO, ppaccessor: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateAccessor: usize,
    pub CloseAccessor: unsafe extern "system" fn(this: *mut *mut Self, paccessor: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ShutDown: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISearchProtocol {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3341878970, data2: 44160, data3: 4561, data4: [141, 243, 0, 192, 79, 182, 239, 79] };
}
#[repr(C)]
pub struct ISearchProtocol2 {
    pub base__: ISearchProtocol,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateAccessorEx: unsafe extern "system" fn(this: *mut *mut Self, pcwszurl: ::windows_sys::core::PCWSTR, pauthenticationinfo: *mut AUTHENTICATION_INFO, pincrementalaccessinfo: *mut INCREMENTAL_ACCESS_INFO, piteminfo: *mut ITEM_INFO, puserdata: *const super::Com::BLOB, ppaccessor: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateAccessorEx: usize,
}
impl ::windows_sys::core::Interface for ISearchProtocol2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2005528754, data2: 46514, data3: 18210, data4: [139, 101, 93, 189, 21, 6, 151, 169] };
}
#[repr(C)]
pub struct ISearchProtocolThreadContext {
    pub base__: ::windows_sys::core::IUnknown,
    pub ThreadInit: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ThreadShutdown: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ThreadIdle: unsafe extern "system" fn(this: *mut *mut Self, dwtimeelaspedsincelastcallinms: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISearchProtocolThreadContext {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3341879009, data2: 44160, data3: 4561, data4: [141, 243, 0, 192, 79, 182, 239, 79] };
}
#[repr(C)]
pub struct ISearchQueryHelper {
    pub base__: ::windows_sys::core::IUnknown,
    pub ConnectionString: unsafe extern "system" fn(this: *mut *mut Self, pszconnectionstring: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetQueryContentLocale: unsafe extern "system" fn(this: *mut *mut Self, lcid: u32) -> ::windows_sys::core::HRESULT,
    pub QueryContentLocale: unsafe extern "system" fn(this: *mut *mut Self, plcid: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetQueryKeywordLocale: unsafe extern "system" fn(this: *mut *mut Self, lcid: u32) -> ::windows_sys::core::HRESULT,
    pub QueryKeywordLocale: unsafe extern "system" fn(this: *mut *mut Self, plcid: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetQueryTermExpansion: unsafe extern "system" fn(this: *mut *mut Self, expandterms: SEARCH_TERM_EXPANSION) -> ::windows_sys::core::HRESULT,
    pub QueryTermExpansion: unsafe extern "system" fn(this: *mut *mut Self, pexpandterms: *mut SEARCH_TERM_EXPANSION) -> ::windows_sys::core::HRESULT,
    pub SetQuerySyntax: unsafe extern "system" fn(this: *mut *mut Self, querysyntax: SEARCH_QUERY_SYNTAX) -> ::windows_sys::core::HRESULT,
    pub QuerySyntax: unsafe extern "system" fn(this: *mut *mut Self, pquerysyntax: *mut SEARCH_QUERY_SYNTAX) -> ::windows_sys::core::HRESULT,
    pub SetQueryContentProperties: unsafe extern "system" fn(this: *mut *mut Self, pszcontentproperties: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub QueryContentProperties: unsafe extern "system" fn(this: *mut *mut Self, ppszcontentproperties: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetQuerySelectColumns: unsafe extern "system" fn(this: *mut *mut Self, pszselectcolumns: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub QuerySelectColumns: unsafe extern "system" fn(this: *mut *mut Self, ppszselectcolumns: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetQueryWhereRestrictions: unsafe extern "system" fn(this: *mut *mut Self, pszrestrictions: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub QueryWhereRestrictions: unsafe extern "system" fn(this: *mut *mut Self, ppszrestrictions: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetQuerySorting: unsafe extern "system" fn(this: *mut *mut Self, pszsorting: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub QuerySorting: unsafe extern "system" fn(this: *mut *mut Self, ppszsorting: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GenerateSQLFromUserQuery: unsafe extern "system" fn(this: *mut *mut Self, pszquery: ::windows_sys::core::PCWSTR, ppszsql: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub WriteProperties: unsafe extern "system" fn(this: *mut *mut Self, itemid: i32, dwnumberofcolumns: u32, pcolumns: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalues: *const SEARCH_COLUMN_PROPERTIES, pftgathermodifiedtime: *const super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    WriteProperties: usize,
    pub SetQueryMaxResults: unsafe extern "system" fn(this: *mut *mut Self, cmaxresults: i32) -> ::windows_sys::core::HRESULT,
    pub QueryMaxResults: unsafe extern "system" fn(this: *mut *mut Self, pcmaxresults: *mut i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISearchQueryHelper {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2872116609, data2: 44160, data3: 4561, data4: [141, 243, 0, 192, 79, 182, 239, 99] };
}
#[repr(C)]
pub struct ISearchQueryHits {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub Init: unsafe extern "system" fn(this: *mut *mut Self, pflt: *mut ::core::ffi::c_void, ulflags: u32) -> i32,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    Init: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub NextHitMoniker: unsafe extern "system" fn(this: *mut *mut Self, pcmnk: *mut u32, papmnk: *mut *mut *mut ::core::ffi::c_void) -> i32,
    #[cfg(not(feature = "Win32_System_Com"))]
    NextHitMoniker: usize,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub NextHitOffset: unsafe extern "system" fn(this: *mut *mut Self, pcregion: *mut u32, paregion: *mut *mut super::super::Storage::IndexServer::FILTERREGION) -> i32,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    NextHitOffset: usize,
}
impl ::windows_sys::core::Interface for ISearchQueryHits {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3985434592, data2: 4204, data3: 4558, data4: [132, 226, 0, 170, 0, 75, 153, 134] };
}
#[repr(C)]
pub struct ISearchRoot {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetSchedule: unsafe extern "system" fn(this: *mut *mut Self, psztaskarg: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub Schedule: unsafe extern "system" fn(this: *mut *mut Self, ppsztaskarg: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetRootURL: unsafe extern "system" fn(this: *mut *mut Self, pszurl: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub RootURL: unsafe extern "system" fn(this: *mut *mut Self, ppszurl: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIsHierarchical: unsafe extern "system" fn(this: *mut *mut Self, fishierarchical: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIsHierarchical: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsHierarchical: unsafe extern "system" fn(this: *mut *mut Self, pfishierarchical: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsHierarchical: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetProvidesNotifications: unsafe extern "system" fn(this: *mut *mut Self, fprovidesnotifications: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetProvidesNotifications: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ProvidesNotifications: unsafe extern "system" fn(this: *mut *mut Self, pfprovidesnotifications: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ProvidesNotifications: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUseNotificationsOnly: unsafe extern "system" fn(this: *mut *mut Self, fusenotificationsonly: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUseNotificationsOnly: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UseNotificationsOnly: unsafe extern "system" fn(this: *mut *mut Self, pfusenotificationsonly: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UseNotificationsOnly: usize,
    pub SetEnumerationDepth: unsafe extern "system" fn(this: *mut *mut Self, dwdepth: u32) -> ::windows_sys::core::HRESULT,
    pub EnumerationDepth: unsafe extern "system" fn(this: *mut *mut Self, pdwdepth: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetHostDepth: unsafe extern "system" fn(this: *mut *mut Self, dwdepth: u32) -> ::windows_sys::core::HRESULT,
    pub HostDepth: unsafe extern "system" fn(this: *mut *mut Self, pdwdepth: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFollowDirectories: unsafe extern "system" fn(this: *mut *mut Self, ffollowdirectories: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFollowDirectories: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FollowDirectories: unsafe extern "system" fn(this: *mut *mut Self, pffollowdirectories: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FollowDirectories: usize,
    pub SetAuthenticationType: unsafe extern "system" fn(this: *mut *mut Self, authtype: AUTH_TYPE) -> ::windows_sys::core::HRESULT,
    pub AuthenticationType: unsafe extern "system" fn(this: *mut *mut Self, pauthtype: *mut AUTH_TYPE) -> ::windows_sys::core::HRESULT,
    pub SetUser: unsafe extern "system" fn(this: *mut *mut Self, pszuser: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub User: unsafe extern "system" fn(this: *mut *mut Self, ppszuser: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetPassword: unsafe extern "system" fn(this: *mut *mut Self, pszpassword: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub Password: unsafe extern "system" fn(this: *mut *mut Self, ppszpassword: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISearchRoot {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 79793359, data2: 8023, data3: 19645, data4: [136, 204, 57, 0, 245, 25, 92, 227] };
}
#[repr(C)]
pub struct ISearchScopeRule {
    pub base__: ::windows_sys::core::IUnknown,
    pub PatternOrURL: unsafe extern "system" fn(this: *mut *mut Self, ppszpatternorurl: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsIncluded: unsafe extern "system" fn(this: *mut *mut Self, pfisincluded: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsIncluded: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsDefault: unsafe extern "system" fn(this: *mut *mut Self, pfisdefault: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsDefault: usize,
    pub FollowFlags: unsafe extern "system" fn(this: *mut *mut Self, pfollowflags: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISearchScopeRule {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2872116609, data2: 44160, data3: 4561, data4: [141, 243, 0, 192, 79, 182, 239, 83] };
}
#[repr(C)]
pub struct ISearchViewChangedSink {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnChange: unsafe extern "system" fn(this: *mut *mut Self, pdwdocid: *const i32, pchange: *const SEARCH_ITEM_CHANGE, pfinview: *const super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnChange: usize,
}
impl ::windows_sys::core::Interface for ISearchViewChangedSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2872116609, data2: 44160, data3: 4561, data4: [141, 243, 0, 192, 79, 182, 239, 101] };
}
#[repr(C)]
pub struct ISecurityInfo {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Security_Authorization")]
    pub GetCurrentTrustee: unsafe extern "system" fn(this: *mut *mut Self, pptrustee: *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Security_Authorization"))]
    GetCurrentTrustee: usize,
    pub GetObjectTypes: unsafe extern "system" fn(this: *mut *mut Self, cobjecttypes: *mut u32, rgobjecttypes: *mut *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetPermissions: unsafe extern "system" fn(this: *mut *mut Self, objecttype: ::windows_sys::core::GUID, ppermissions: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISecurityInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878244, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IService {
    pub base__: ::windows_sys::core::IUnknown,
    pub InvokeService: unsafe extern "system" fn(this: *mut *mut Self, punkinner: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IService {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 102829704, data2: 501, data3: 4561, data4: [181, 18, 0, 128, 199, 129, 195, 132] };
}
#[repr(C)]
pub struct ISessionProperties {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperties: unsafe extern "system" fn(this: *mut *mut Self, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetProperties: unsafe extern "system" fn(this: *mut *mut Self, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetProperties: usize,
}
impl ::windows_sys::core::Interface for ISessionProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878213, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct ISimpleCommandCreator {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateICommand: unsafe extern "system" fn(this: *mut *mut Self, ppiunknown: *mut *mut ::core::ffi::c_void, pouterunk: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VerifyCatalog: unsafe extern "system" fn(this: *mut *mut Self, pwszmachine: ::windows_sys::core::PCWSTR, pwszcatalogname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetDefaultCatalog: unsafe extern "system" fn(this: *mut *mut Self, pwszcatalogname: ::windows_sys::core::PCWSTR, cwcin: u32, pcwcout: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISimpleCommandCreator {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1580473015, data2: 720, data3: 4561, data4: [144, 12, 0, 160, 201, 6, 55, 150] };
}
#[repr(C)]
pub struct ISourcesRowset {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetSourcesRowset: unsafe extern "system" fn(this: *mut *mut Self, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows_sys::core::GUID, cpropertysets: u32, rgproperties: *mut DBPROPSET, ppsourcesrowset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetSourcesRowset: usize,
}
impl ::windows_sys::core::Interface for ISourcesRowset {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878110, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IStemmer {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Init: unsafe extern "system" fn(this: *mut *mut Self, ulmaxtokensize: u32, pflicense: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Init: usize,
    pub GenerateWordForms: unsafe extern "system" fn(this: *mut *mut Self, pwcinbuf: ::windows_sys::core::PCWSTR, cwc: u32, pstemsink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetLicenseToUse: unsafe extern "system" fn(this: *mut *mut Self, ppwcslicense: *const *const u16) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStemmer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4022006080, data2: 32578, data3: 4558, data4: [190, 87, 0, 170, 0, 81, 254, 32] };
}
#[repr(C)]
pub struct ISubscriptionItem {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCookie: unsafe extern "system" fn(this: *mut *mut Self, pcookie: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetSubscriptionItemInfo: unsafe extern "system" fn(this: *mut *mut Self, psubscriptioniteminfo: *mut SUBSCRIPTIONITEMINFO) -> ::windows_sys::core::HRESULT,
    pub SetSubscriptionItemInfo: unsafe extern "system" fn(this: *mut *mut Self, psubscriptioniteminfo: *const SUBSCRIPTIONITEMINFO) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReadProperties: unsafe extern "system" fn(this: *mut *mut Self, ncount: u32, rgwszname: *const ::windows_sys::core::PWSTR, rgvalue: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReadProperties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub WriteProperties: unsafe extern "system" fn(this: *mut *mut Self, ncount: u32, rgwszname: *const ::windows_sys::core::PWSTR, rgvalue: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    WriteProperties: usize,
    pub EnumProperties: unsafe extern "system" fn(this: *mut *mut Self, ppenumitemproperties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NotifyChanged: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISubscriptionItem {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2843040248, data2: 27722, data3: 4561, data4: [161, 232, 0, 192, 79, 194, 251, 225] };
}
#[repr(C)]
pub struct ISubscriptionMgr {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteSubscription: unsafe extern "system" fn(this: *mut *mut Self, pwszurl: ::windows_sys::core::PCWSTR, hwnd: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteSubscription: usize,
    pub UpdateSubscription: unsafe extern "system" fn(this: *mut *mut Self, pwszurl: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub UpdateAll: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsSubscribed: unsafe extern "system" fn(this: *mut *mut Self, pwszurl: ::windows_sys::core::PCWSTR, pfsubscribed: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsSubscribed: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSubscriptionInfo: unsafe extern "system" fn(this: *mut *mut Self, pwszurl: ::windows_sys::core::PCWSTR, pinfo: *mut SUBSCRIPTIONINFO) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSubscriptionInfo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDefaultInfo: unsafe extern "system" fn(this: *mut *mut Self, subtype: SUBSCRIPTIONTYPE, pinfo: *mut SUBSCRIPTIONINFO) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDefaultInfo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowSubscriptionProperties: unsafe extern "system" fn(this: *mut *mut Self, pwszurl: ::windows_sys::core::PCWSTR, hwnd: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowSubscriptionProperties: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateSubscription: unsafe extern "system" fn(this: *mut *mut Self, hwnd: super::super::Foundation::HWND, pwszurl: ::windows_sys::core::PCWSTR, pwszfriendlyname: ::windows_sys::core::PCWSTR, dwflags: u32, substype: SUBSCRIPTIONTYPE, pinfo: *mut SUBSCRIPTIONINFO) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateSubscription: usize,
}
impl ::windows_sys::core::Interface for ISubscriptionMgr {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 140489408, data2: 3576, data3: 4561, data4: [143, 75, 0, 160, 201, 5, 65, 63] };
}
#[repr(C)]
pub struct ISubscriptionMgr2 {
    pub base__: ISubscriptionMgr,
    pub GetItemFromURL: unsafe extern "system" fn(this: *mut *mut Self, pwszurl: ::windows_sys::core::PCWSTR, ppsubscriptionitem: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetItemFromCookie: unsafe extern "system" fn(this: *mut *mut Self, psubscriptioncookie: *const ::windows_sys::core::GUID, ppsubscriptionitem: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetSubscriptionRunState: unsafe extern "system" fn(this: *mut *mut Self, dwnumcookies: u32, pcookies: *const ::windows_sys::core::GUID, pdwrunstate: *mut u32) -> ::windows_sys::core::HRESULT,
    pub EnumSubscriptions: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, ppenumsubscriptions: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UpdateItems: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, dwnumcookies: u32, pcookies: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub AbortItems: unsafe extern "system" fn(this: *mut *mut Self, dwnumcookies: u32, pcookies: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub AbortAll: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISubscriptionMgr2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1632354928, data2: 44767, data3: 4561, data4: [161, 249, 0, 192, 79, 194, 251, 225] };
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct ITEMPROP {
    pub variantValue: super::Com::VARIANT,
    pub pwszName: ::windows_sys::core::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for ITEMPROP {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for ITEMPROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub struct ITEM_INFO {
    pub dwSize: u32,
    pub pcwszFromEMail: ::windows_sys::core::PCWSTR,
    pub pcwszApplicationName: ::windows_sys::core::PCWSTR,
    pub pcwszCatalogName: ::windows_sys::core::PCWSTR,
    pub pcwszContentClass: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for ITEM_INFO {}
impl ::core::clone::Clone for ITEM_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ITableCreation {
    pub base__: ITableDefinition,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetTableDefinition: unsafe extern "system" fn(this: *mut *mut Self, ptableid: *const super::super::Storage::IndexServer::DBID, pccolumndescs: *mut usize, prgcolumndescs: *mut *mut DBCOLUMNDESC, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET, pcconstraintdescs: *mut u32, prgconstraintdescs: *mut *mut DBCONSTRAINTDESC, ppwszstringbuffer: *mut *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetTableDefinition: usize,
}
impl ::windows_sys::core::Interface for ITableCreation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878268, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct ITableDefinition {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateTable: unsafe extern "system" fn(this: *mut *mut Self, punkouter: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, ccolumndescs: usize, rgcolumndescs: *const DBCOLUMNDESC, riid: *const ::windows_sys::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pptableid: *mut *mut super::super::Storage::IndexServer::DBID, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateTable: usize,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub DropTable: unsafe extern "system" fn(this: *mut *mut Self, ptableid: *const super::super::Storage::IndexServer::DBID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    DropTable: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddColumn: unsafe extern "system" fn(this: *mut *mut Self, ptableid: *const super::super::Storage::IndexServer::DBID, pcolumndesc: *const DBCOLUMNDESC, ppcolumnid: *mut *mut super::super::Storage::IndexServer::DBID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddColumn: usize,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub DropColumn: unsafe extern "system" fn(this: *mut *mut Self, ptableid: *const super::super::Storage::IndexServer::DBID, pcolumnid: *const super::super::Storage::IndexServer::DBID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    DropColumn: usize,
}
impl ::windows_sys::core::Interface for ITableDefinition {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878214, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct ITableDefinitionWithConstraints {
    pub base__: ITableCreation,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddConstraint: unsafe extern "system" fn(this: *mut *mut Self, ptableid: *mut super::super::Storage::IndexServer::DBID, pconstraintdesc: *mut DBCONSTRAINTDESC) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddConstraint: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateTableWithConstraints: unsafe extern "system" fn(this: *mut *mut Self, punkouter: *mut ::core::ffi::c_void, ptableid: *mut super::super::Storage::IndexServer::DBID, ccolumndescs: usize, rgcolumndescs: *mut DBCOLUMNDESC, cconstraintdescs: u32, rgconstraintdescs: *mut DBCONSTRAINTDESC, riid: *const ::windows_sys::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pptableid: *mut *mut super::super::Storage::IndexServer::DBID, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateTableWithConstraints: usize,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub DropConstraint: unsafe extern "system" fn(this: *mut *mut Self, ptableid: *mut super::super::Storage::IndexServer::DBID, pconstraintid: *mut super::super::Storage::IndexServer::DBID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    DropConstraint: usize,
}
impl ::windows_sys::core::Interface for ITableDefinitionWithConstraints {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878251, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct ITableRename {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub RenameColumn: unsafe extern "system" fn(this: *mut *mut Self, ptableid: *mut super::super::Storage::IndexServer::DBID, poldcolumnid: *mut super::super::Storage::IndexServer::DBID, pnewcolumnid: *mut super::super::Storage::IndexServer::DBID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    RenameColumn: usize,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub RenameTable: unsafe extern "system" fn(this: *mut *mut Self, poldtableid: *mut super::super::Storage::IndexServer::DBID, poldindexid: *mut super::super::Storage::IndexServer::DBID, pnewtableid: *mut super::super::Storage::IndexServer::DBID, pnewindexid: *mut super::super::Storage::IndexServer::DBID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    RenameTable: usize,
}
impl ::windows_sys::core::Interface for ITableRename {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878199, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct ITokenCollection {
    pub base__: ::windows_sys::core::IUnknown,
    pub NumberOfTokens: unsafe extern "system" fn(this: *mut *mut Self, pcount: *const u32) -> ::windows_sys::core::HRESULT,
    pub GetToken: unsafe extern "system" fn(this: *mut *mut Self, i: u32, pbegin: *mut u32, plength: *mut u32, ppsz: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITokenCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 584627442, data2: 62839, data3: 19163, data4: [163, 53, 194, 174, 136, 65, 111, 171] };
}
#[repr(C)]
pub struct ITransactionJoin {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub GetOptionsObject: unsafe extern "system" fn(this: *mut *mut Self, ppoptions: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_DistributedTransactionCoordinator"))]
    GetOptionsObject: usize,
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub JoinTransaction: unsafe extern "system" fn(this: *mut *mut Self, punktransactioncoord: *mut ::core::ffi::c_void, isolevel: i32, isoflags: u32, potheroptions: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_DistributedTransactionCoordinator"))]
    JoinTransaction: usize,
}
impl ::windows_sys::core::Interface for ITransactionJoin {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878174, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
#[repr(C)]
pub struct ITransactionLocal {
    pub base__: super::DistributedTransactionCoordinator::ITransaction,
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub GetOptionsObject: unsafe extern "system" fn(this: *mut *mut Self, ppoptions: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_DistributedTransactionCoordinator"))]
    GetOptionsObject: usize,
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub StartTransaction: unsafe extern "system" fn(this: *mut *mut Self, isolevel: i32, isoflags: u32, potheroptions: *mut ::core::ffi::c_void, pultransactionlevel: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_DistributedTransactionCoordinator"))]
    StartTransaction: usize,
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl ::windows_sys::core::Interface for ITransactionLocal {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878175, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct ITransactionObject {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub GetTransactionObject: unsafe extern "system" fn(this: *mut *mut Self, ultransactionlevel: u32, pptransactionobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_DistributedTransactionCoordinator"))]
    GetTransactionObject: usize,
}
impl ::windows_sys::core::Interface for ITransactionObject {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878176, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct ITrusteeAdmin {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Security_Authorization")]
    pub CompareTrustees: unsafe extern "system" fn(this: *mut *mut Self, ptrustee1: *mut super::super::Security::Authorization::TRUSTEE_W, ptrustee2: *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Security_Authorization"))]
    CompareTrustees: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateTrustee: unsafe extern "system" fn(this: *mut *mut Self, ptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateTrustee: usize,
    #[cfg(feature = "Win32_Security_Authorization")]
    pub DeleteTrustee: unsafe extern "system" fn(this: *mut *mut Self, ptrustee: *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Security_Authorization"))]
    DeleteTrustee: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetTrusteeProperties: unsafe extern "system" fn(this: *mut *mut Self, ptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetTrusteeProperties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetTrusteeProperties: unsafe extern "system" fn(this: *mut *mut Self, ptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetTrusteeProperties: usize,
}
impl ::windows_sys::core::Interface for ITrusteeAdmin {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878241, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct ITrusteeGroupAdmin {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Security_Authorization")]
    pub AddMember: unsafe extern "system" fn(this: *mut *mut Self, pmembershiptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, pmembertrustee: *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Security_Authorization"))]
    AddMember: usize,
    #[cfg(feature = "Win32_Security_Authorization")]
    pub DeleteMember: unsafe extern "system" fn(this: *mut *mut Self, pmembershiptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, pmembertrustee: *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Security_Authorization"))]
    DeleteMember: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization"))]
    pub IsMember: unsafe extern "system" fn(this: *mut *mut Self, pmembershiptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, pmembertrustee: *mut super::super::Security::Authorization::TRUSTEE_W, pfstatus: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization")))]
    IsMember: usize,
    #[cfg(feature = "Win32_Security_Authorization")]
    pub GetMembers: unsafe extern "system" fn(this: *mut *mut Self, pmembershiptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, pcmembers: *mut u32, prgmembers: *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Security_Authorization"))]
    GetMembers: usize,
    #[cfg(feature = "Win32_Security_Authorization")]
    pub GetMemberships: unsafe extern "system" fn(this: *mut *mut Self, ptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, pcmemberships: *mut u32, prgmemberships: *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Security_Authorization"))]
    GetMemberships: usize,
}
impl ::windows_sys::core::Interface for ITrusteeGroupAdmin {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878242, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IUMS {
    pub SqlUmsSuspend: unsafe extern "system" fn(this: *mut *mut Self, ticks: u32),
    pub SqlUmsYield: unsafe extern "system" fn(this: *mut *mut Self, ticks: u32),
    pub SqlUmsSwitchPremptive: unsafe extern "system" fn(this: *mut *mut Self),
    pub SqlUmsSwitchNonPremptive: unsafe extern "system" fn(this: *mut *mut Self),
    #[cfg(feature = "Win32_Foundation")]
    pub SqlUmsFIsPremptive: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    SqlUmsFIsPremptive: usize,
}
impl ::windows_sys::core::Interface for IUMS {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::zeroed();
}
#[repr(C)]
pub struct IUMSInitialize {
    pub base__: ::windows_sys::core::IUnknown,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, pums: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUMSInitialize {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1559546388, data2: 61217, data3: 4560, data4: [151, 231, 0, 192, 79, 194, 173, 152] };
}
#[repr(C)]
pub struct IUrlAccessor {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub AddRequestParameter: unsafe extern "system" fn(this: *mut *mut Self, pspec: *const super::Com::StructuredStorage::PROPSPEC, pvar: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    AddRequestParameter: usize,
    pub GetDocFormat: unsafe extern "system" fn(this: *mut *mut Self, wszdocformat: ::windows_sys::core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetCLSID: unsafe extern "system" fn(this: *mut *mut Self, pclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetHost: unsafe extern "system" fn(this: *mut *mut Self, wszhost: ::windows_sys::core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows_sys::core::HRESULT,
    pub IsDirectory: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetSize: unsafe extern "system" fn(this: *mut *mut Self, pllsize: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLastModified: unsafe extern "system" fn(this: *mut *mut Self, pftlastmodified: *mut super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLastModified: usize,
    pub GetFileName: unsafe extern "system" fn(this: *mut *mut Self, wszfilename: ::windows_sys::core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetSecurityDescriptor: unsafe extern "system" fn(this: *mut *mut Self, psd: *mut u8, dwsize: u32, pdwlength: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetRedirectedURL: unsafe extern "system" fn(this: *mut *mut Self, wszredirectedurl: ::windows_sys::core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetSecurityProvider: unsafe extern "system" fn(this: *mut *mut Self, pspclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub BindToStream: unsafe extern "system" fn(this: *mut *mut Self, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BindToStream: usize,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub BindToFilter: unsafe extern "system" fn(this: *mut *mut Self, ppfilter: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    BindToFilter: usize,
}
impl ::windows_sys::core::Interface for IUrlAccessor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 191095576, data2: 40140, data3: 4560, data4: [188, 219, 0, 128, 95, 204, 206, 4] };
}
#[repr(C)]
pub struct IUrlAccessor2 {
    pub base__: IUrlAccessor,
    pub GetDisplayUrl: unsafe extern "system" fn(this: *mut *mut Self, wszdocurl: ::windows_sys::core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows_sys::core::HRESULT,
    pub IsDocument: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetCodePage: unsafe extern "system" fn(this: *mut *mut Self, wszcodepage: ::windows_sys::core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUrlAccessor2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3341879092, data2: 44160, data3: 4561, data4: [141, 243, 0, 192, 79, 182, 239, 79] };
}
#[repr(C)]
pub struct IUrlAccessor3 {
    pub base__: IUrlAccessor2,
    #[cfg(feature = "Win32_System_Com")]
    pub GetImpersonationSidBlobs: unsafe extern "system" fn(this: *mut *mut Self, pcwszurl: ::windows_sys::core::PCWSTR, pcsidcount: *mut u32, ppsidblobs: *mut *mut super::Com::BLOB) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetImpersonationSidBlobs: usize,
}
impl ::windows_sys::core::Interface for IUrlAccessor3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1874620421, data2: 1109, data3: 18548, data4: [184, 255, 116, 57, 69, 2, 65, 163] };
}
#[repr(C)]
pub struct IUrlAccessor4 {
    pub base__: IUrlAccessor3,
    #[cfg(feature = "Win32_Foundation")]
    pub ShouldIndexItemContent: unsafe extern "system" fn(this: *mut *mut Self, pfindexcontent: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShouldIndexItemContent: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub ShouldIndexProperty: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfindexproperty: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem")))]
    ShouldIndexProperty: usize,
}
impl ::windows_sys::core::Interface for IUrlAccessor4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1556418625, data2: 51410, data3: 16855, data4: [188, 163, 158, 158, 40, 98, 151, 220] };
}
#[repr(C)]
pub struct IViewChapter {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetSpecification: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OpenViewChapter: unsafe extern "system" fn(this: *mut *mut Self, hsource: usize, phviewchapter: *mut usize) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IViewChapter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878232, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IViewFilter {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetFilter: unsafe extern "system" fn(this: *mut *mut Self, haccessor: usize, pcrows: *mut usize, pcompareops: *mut *mut u32, pcriteriadata: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFilterBindings: unsafe extern "system" fn(this: *mut *mut Self, pcbindings: *mut usize, prgbindings: *mut *mut DBBINDING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFilterBindings: usize,
    pub SetFilter: unsafe extern "system" fn(this: *mut *mut Self, haccessor: usize, crows: usize, compareops: *const u32, pcriteriadata: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IViewFilter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878235, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IViewRowset {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetSpecification: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OpenViewRowset: unsafe extern "system" fn(this: *mut *mut Self, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows_sys::core::GUID, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IViewRowset {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878231, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IViewSort {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetSortOrder: unsafe extern "system" fn(this: *mut *mut Self, pcvalues: *mut usize, prgcolumns: *mut *mut usize, prgorders: *mut *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetSortOrder: unsafe extern "system" fn(this: *mut *mut Self, cvalues: usize, rgcolumns: *const usize, rgorders: *const u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IViewSort {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 208878234, data2: 10780, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
}
#[repr(C)]
pub struct IWordBreaker {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Init: unsafe extern "system" fn(this: *mut *mut Self, fquery: super::super::Foundation::BOOL, ulmaxtokensize: u32, pflicense: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Init: usize,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub BreakText: unsafe extern "system" fn(this: *mut *mut Self, ptextsource: *mut TEXT_SOURCE, pwordsink: *mut ::core::ffi::c_void, pphrasesink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    BreakText: usize,
    pub ComposePhrase: unsafe extern "system" fn(this: *mut *mut Self, pwcnoun: ::windows_sys::core::PCWSTR, cwcnoun: u32, pwcmodifier: ::windows_sys::core::PCWSTR, cwcmodifier: u32, ulattachmenttype: u32, pwcphrase: ::windows_sys::core::PCWSTR, pcwcphrase: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetLicenseToUse: unsafe extern "system" fn(this: *mut *mut Self, ppwcslicense: *const *const u16) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWordBreaker {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3577041608, data2: 30691, data3: 4122, data4: [181, 82, 8, 0, 43, 51, 176, 230] };
}
#[repr(C)]
pub struct IWordFormSink {
    pub base__: ::windows_sys::core::IUnknown,
    pub PutAltWord: unsafe extern "system" fn(this: *mut *mut Self, pwcinbuf: ::windows_sys::core::PCWSTR, cwc: u32) -> ::windows_sys::core::HRESULT,
    pub PutWord: unsafe extern "system" fn(this: *mut *mut Self, pwcinbuf: ::windows_sys::core::PCWSTR, cwc: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWordFormSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4269261616, data2: 32578, data3: 4558, data4: [190, 87, 0, 170, 0, 81, 254, 32] };
}
#[repr(C)]
pub struct IWordSink {
    pub base__: ::windows_sys::core::IUnknown,
    pub PutWord: unsafe extern "system" fn(this: *mut *mut Self, cwc: u32, pwcinbuf: ::windows_sys::core::PCWSTR, cwcsrclen: u32, cwcsrcpos: u32) -> ::windows_sys::core::HRESULT,
    pub PutAltWord: unsafe extern "system" fn(this: *mut *mut Self, cwc: u32, pwcinbuf: ::windows_sys::core::PCWSTR, cwcsrclen: u32, cwcsrcpos: u32) -> ::windows_sys::core::HRESULT,
    pub StartAltPhrase: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub EndAltPhrase: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub PutBreak: unsafe extern "system" fn(this: *mut *mut Self, breaktype: super::super::Storage::IndexServer::WORDREP_BREAK_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    PutBreak: usize,
}
impl ::windows_sys::core::Interface for IWordSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3432018004, data2: 49240, data3: 4122, data4: [181, 84, 8, 0, 43, 51, 176, 230] };
}
pub const Interval: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3646363423, data2: 19449, data3: 19938, data4: [188, 213, 199, 10, 124, 165, 88, 54] };
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const JET_GET_PROP_STORE_ERROR: i32 = -1073732822i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const JET_INIT_ERROR: i32 = -1073732824i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const JET_MULTIINSTANCE_DISABLED: i32 = -2147474645i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const JET_NEW_PROP_STORE_ERROR: i32 = -1073732823i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const JPS_E_CATALOG_DECSRIPTION_MISSING: i32 = -2147217023i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const JPS_E_INSUFFICIENT_DATABASE_RESOURCES: i32 = -2147217019i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const JPS_E_INSUFFICIENT_DATABASE_SESSIONS: i32 = -2147217020i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const JPS_E_INSUFFICIENT_VERSION_STORAGE: i32 = -2147217021i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const JPS_E_JET_ERR: i32 = -2147217025i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const JPS_E_MISSING_INFORMATION: i32 = -2147217022i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const JPS_E_PROPAGATION_CORRUPTION: i32 = -2147217016i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const JPS_E_PROPAGATION_FILE: i32 = -2147217017i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const JPS_E_PROPAGATION_VERSION_MISMATCH: i32 = -2147217015i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const JPS_E_SCHEMA_ERROR: i32 = -2147217018i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const JPS_E_SHARING_VIOLATION: i32 = -2147217014i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const JPS_S_DUPLICATE_DOC_DETECTED: i32 = 266624i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct KAGGETDIAG {
    pub ulSize: u32,
    pub vDiagInfo: super::Com::VARIANT,
    pub sDiagField: i16,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for KAGGETDIAG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for KAGGETDIAG {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROPVAL_CONCUR_LOCK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROPVAL_CONCUR_READ_ONLY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROPVAL_CONCUR_ROWVER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROPVAL_CONCUR_VALUES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROP_ACCESSIBLEPROCEDURES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROP_ACCESSIBLETABLES: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROP_ACTIVESTATEMENTS: u32 = 24u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROP_AUTH_SERVERINTEGRATED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROP_AUTH_TRUSTEDCONNECTION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROP_BLOBSONFOCURSOR: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROP_CONCURRENCY: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROP_CURSOR: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROP_DRIVERNAME: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROP_DRIVERODBCVER: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROP_DRIVERVER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROP_FILEUSAGE: u32 = 23u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROP_FORCENOPARAMETERREBIND: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROP_FORCENOPREPARE: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROP_FORCENOREEXECUTE: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROP_FORCESSFIREHOSEMODE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROP_INCLUDENONEXACT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROP_IRowsetChangeExtInfo: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROP_LIKEESCAPECLAUSE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROP_MARSHALLABLE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROP_MAXCOLUMNSINGROUPBY: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROP_MAXCOLUMNSININDEX: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROP_MAXCOLUMNSINORDERBY: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROP_MAXCOLUMNSINSELECT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROP_MAXCOLUMNSINTABLE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROP_NUMERICFUNCTIONS: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROP_ODBCSQLCONFORMANCE: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROP_ODBCSQLOPTIEF: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROP_OJCAPABILITY: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROP_OUTERJOINS: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROP_POSITIONONNEWROW: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROP_PROCEDURES: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROP_QUERYBASEDUPDATES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROP_SPECIALCHARACTERS: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROP_STRINGFUNCTIONS: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROP_SYSTEMFUNCTIONS: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGPROP_TIMEDATEFUNCTIONS: u32 = 22u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub struct KAGREQDIAG {
    pub ulDiagFlags: u32,
    pub vt: u16,
    pub sDiagField: i16,
}
impl ::core::marker::Copy for KAGREQDIAG {}
impl ::core::clone::Clone for KAGREQDIAG {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type KAGREQDIAGFLAGSENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGREQDIAGFLAGS_HEADER: KAGREQDIAGFLAGSENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const KAGREQDIAGFLAGS_RECORD: KAGREQDIAGFLAGSENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type LOCKMODEENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const LOCKMODE_INVALID: LOCKMODEENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const LOCKMODE_EXCLUSIVE: LOCKMODEENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const LOCKMODE_SHARED: LOCKMODEENUM = 2i32;
pub const LeafCondition: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1391549577, data2: 23063, data3: 18657, data4: [187, 205, 70, 163, 248, 156, 124, 194] };
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MAXNAME: u32 = 129u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MAXNUMERICLEN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MAXUSEVERITY: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MAX_QUERY_RANK: u32 = 1000u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct MDAXISINFO {
    pub cbSize: usize,
    pub iAxis: usize,
    pub cDimensions: usize,
    pub cCoordinates: usize,
    pub rgcColumns: *mut usize,
    pub rgpwszDimensionNames: *mut ::windows_sys::core::PWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for MDAXISINFO {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for MDAXISINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
#[cfg(target_arch = "x86")]
pub struct MDAXISINFO {
    pub cbSize: usize,
    pub iAxis: usize,
    pub cDimensions: usize,
    pub cCoordinates: usize,
    pub rgcColumns: *mut usize,
    pub rgpwszDimensionNames: *mut ::windows_sys::core::PWSTR,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for MDAXISINFO {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for MDAXISINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDAXIS_CHAPTERS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDAXIS_COLUMNS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDAXIS_PAGES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDAXIS_ROWS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDAXIS_SECTIONS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDAXIS_SLICERS: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDDISPINFO_DRILLED_DOWN: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDDISPINFO_PARENT_SAME_AS_PREV: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDFF_BOLD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDFF_ITALIC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDFF_STRIKEOUT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDFF_UNDERLINE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDLEVEL_TYPE_ALL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDLEVEL_TYPE_CALCULATED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDLEVEL_TYPE_REGULAR: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDLEVEL_TYPE_RESERVED1: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDLEVEL_TYPE_TIME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDLEVEL_TYPE_TIME_DAYS: u32 = 516u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDLEVEL_TYPE_TIME_HALF_YEAR: u32 = 36u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDLEVEL_TYPE_TIME_HOURS: u32 = 772u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDLEVEL_TYPE_TIME_MINUTES: u32 = 1028u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDLEVEL_TYPE_TIME_MONTHS: u32 = 132u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDLEVEL_TYPE_TIME_QUARTERS: u32 = 68u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDLEVEL_TYPE_TIME_SECONDS: u32 = 2052u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDLEVEL_TYPE_TIME_UNDEFINED: u32 = 4100u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDLEVEL_TYPE_TIME_WEEKS: u32 = 260u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDLEVEL_TYPE_TIME_YEARS: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDLEVEL_TYPE_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDMEASURE_AGGR_AVG: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDMEASURE_AGGR_CALCULATED: u32 = 127u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDMEASURE_AGGR_COUNT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDMEASURE_AGGR_MAX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDMEASURE_AGGR_MIN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDMEASURE_AGGR_STD: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDMEASURE_AGGR_SUM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDMEASURE_AGGR_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDMEASURE_AGGR_VAR: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDMEMBER_TYPE_ALL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDMEMBER_TYPE_FORMULA: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDMEMBER_TYPE_MEASURE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDMEMBER_TYPE_REGULAR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDMEMBER_TYPE_RESERVE1: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDMEMBER_TYPE_RESERVE2: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDMEMBER_TYPE_RESERVE3: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDMEMBER_TYPE_RESERVE4: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDMEMBER_TYPE_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_AU_UNCHANGED: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_AU_UNKNOWN: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_AU_UNSUPPORTED: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_FS_FULL_SUPPORT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_FS_GENERATED_COLUMN: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_FS_GENERATED_DIMENSION: i32 = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_FS_NO_SUPPORT: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MC_SEARCHEDCASE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MC_SINGLECASE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MD_AFTER: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MD_BEFORE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MD_SELF: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MF_CREATE_CALCMEMBERS: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MF_CREATE_NAMEDSETS: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MF_SCOPE_GLOBAL: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MF_SCOPE_SESSION: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MF_WITH_CALCMEMBERS: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MF_WITH_NAMEDSETS: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MJC_IMPLICITCUBE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MJC_MULTICUBES: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MJC_SINGLECUBE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MMF_CLOSINGPERIOD: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MMF_COUSIN: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MMF_OPENINGPERIOD: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MMF_PARALLELPERIOD: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MNF_AGGREGATE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MNF_CORRELATION: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MNF_COVARIANCE: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MNF_DRILLDOWNLEVEL: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MNF_DRILLDOWNLEVELBOTTOM: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MNF_DRILLDOWNLEVELTOP: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MNF_DRILLDOWNMEMBERBOTTOM: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MNF_DRILLDOWNMEMBERTOP: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MNF_DRILLUPLEVEL: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MNF_DRILLUPMEMBER: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MNF_LINREG2: i32 = 512i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MNF_LINREGPOINT: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MNF_LINREGSLOPE: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MNF_LINREGVARIANCE: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MNF_MEDIAN: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MNF_RANK: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MNF_STDDEV: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MNF_VAR: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MOQ_CATALOG_CUBE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MOQ_CUBE_DIM: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MOQ_DATASOURCE_CUBE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MOQ_DIMHIER_LEVEL: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MOQ_DIMHIER_MEMBER: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MOQ_DIM_HIER: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MOQ_LEVEL_MEMBER: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MOQ_MEMBER_MEMBER: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MOQ_OUTERREFERENCE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MOQ_SCHEMA_CUBE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MSC_GREATERTHAN: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MSC_GREATERTHANEQUAL: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MSC_LESSTHAN: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MSC_LESSTHANEQUAL: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MSF_BOTTOMPERCENT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MSF_BOTTOMSUM: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MSF_DRILLDOWNLEVEL: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MSF_DRILLDOWNLEVELBOTTOM: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MSF_DRILLDOWNLEVELTOP: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MSF_DRILLDOWNMEMBBER: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MSF_DRILLDOWNMEMBERBOTTOM: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MSF_DRILLDOWNMEMBERTOP: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MSF_DRILLUPLEVEL: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MSF_DRILLUPMEMBER: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MSF_LASTPERIODS: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MSF_MTD: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MSF_PERIODSTODATE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MSF_QTD: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MSF_TOGGLEDRILLSTATE: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MSF_TOPPERCENT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MSF_TOPSUM: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MSF_WTD: i32 = 512i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MSF_YTD: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MS_MULTIPLETUPLES: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_MS_SINGLETUPLE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_NL_NAMEDLEVELS: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_NL_NUMBEREDLEVELS: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_NL_SCHEMAONLY: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_NME_ALLDIMENSIONS: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_NME_MEASURESONLY: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_RR_NORANGEROWSET: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_RR_READONLY: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_RR_UPDATE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_VISUAL_MODE_DEFAULT: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_VISUAL_MODE_VISUAL: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROPVAL_VISUAL_MODE_VISUAL_OFF: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROP_CELL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDPROP_MEMBER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDTREEOP_ANCESTORS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDTREEOP_CHILDREN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDTREEOP_DESCENDANTS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDTREEOP_PARENT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDTREEOP_SELF: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MDTREEOP_SIBLINGS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MD_DIMTYPE_MEASURE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MD_DIMTYPE_OTHER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MD_DIMTYPE_TIME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MD_DIMTYPE_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MD_E_BADCOORDINATE: ::windows_sys::core::HRESULT = -2147217822i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MD_E_BADTUPLE: ::windows_sys::core::HRESULT = -2147217823i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MD_E_INVALIDAXIS: ::windows_sys::core::HRESULT = -2147217821i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MD_E_INVALIDCELLRANGE: ::windows_sys::core::HRESULT = -2147217820i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MINFATALERR: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MIN_USER_DATATYPE: u32 = 256u32;
pub const MSDAINITIALIZE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 570871216, data2: 6593, data3: 4561, data4: [137, 224, 0, 192, 79, 215, 168, 41] };
pub const MSDAORA: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3905703102, data2: 65023, data3: 4560, data4: [184, 101, 0, 160, 201, 8, 28, 29] };
pub const MSDAORA8: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2131141491, data2: 56682, data3: 17371, data4: [180, 224, 31, 193, 33, 229, 230, 43] };
pub const MSDAORA8_ERROR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2131141492, data2: 56682, data3: 17371, data4: [180, 224, 31, 193, 33, 229, 230, 43] };
pub const MSDAORA_ERROR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3905703103, data2: 65023, data3: 4560, data4: [184, 101, 0, 160, 201, 8, 28, 29] };
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type MSDSDBINITPROPENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_MSDS_DBINIT_DATAPROVIDER: MSDSDBINITPROPENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type MSDSSESSIONPROPENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const DBPROP_MSDS_SESS_UNIQUENAMES: MSDSSESSIONPROPENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MSG_CI_CORRUPT_INDEX_COMPONENT: ::windows_sys::core::HRESULT = 1073745962i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MSG_CI_CREATE_SEVER_ITEM_FAILED: ::windows_sys::core::HRESULT = -2147479480i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MSG_CI_MASTER_MERGE_ABORTED: ::windows_sys::core::HRESULT = 1073745928i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MSG_CI_MASTER_MERGE_ABORTED_LOW_DISK: ::windows_sys::core::HRESULT = 1073745987i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MSG_CI_MASTER_MERGE_CANT_RESTART: ::windows_sys::core::HRESULT = -1073737718i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MSG_CI_MASTER_MERGE_CANT_START: ::windows_sys::core::HRESULT = -1073737719i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MSG_CI_MASTER_MERGE_COMPLETED: ::windows_sys::core::HRESULT = 1073745927i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MSG_CI_MASTER_MERGE_REASON_EXPECTED_DOCS: ::windows_sys::core::HRESULT = 1073745990i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MSG_CI_MASTER_MERGE_REASON_EXTERNAL: ::windows_sys::core::HRESULT = 1073745988i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MSG_CI_MASTER_MERGE_REASON_INDEX_LIMIT: ::windows_sys::core::HRESULT = 1073745989i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MSG_CI_MASTER_MERGE_REASON_NUMBER: ::windows_sys::core::HRESULT = 1073745991i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MSG_CI_MASTER_MERGE_RESTARTED: ::windows_sys::core::HRESULT = 1073745945i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MSG_CI_MASTER_MERGE_STARTED: ::windows_sys::core::HRESULT = 1073745926i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MSG_TEST_MESSAGE: i32 = 1074008064i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MSS_E_APPALREADYEXISTS: i32 = -2147213054i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MSS_E_APPNOTFOUND: i32 = -2147213055i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MSS_E_CATALOGALREADYEXISTS: i32 = -2147213050i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MSS_E_CATALOGNOTFOUND: i32 = -2147213053i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MSS_E_CATALOGSTOPPING: i32 = -2147213052i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MSS_E_INVALIDAPPNAME: i32 = -2147213056i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MSS_E_UNICODEFILEHEADERMISSING: i32 = -2147213051i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const MS_PERSIST_PROGID: &str = "MSPersist";
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type NAMED_ENTITY_CERTAINTY = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const NEC_LOW: NAMED_ENTITY_CERTAINTY = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const NEC_MEDIUM: NAMED_ENTITY_CERTAINTY = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const NEC_HIGH: NAMED_ENTITY_CERTAINTY = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
pub struct NATLANGUAGERESTRICTION {
    pub prop: super::super::Storage::IndexServer::FULLPROPSPEC,
    pub pwcsPhrase: ::windows_sys::core::PWSTR,
    pub lcid: u32,
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for NATLANGUAGERESTRICTION {}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for NATLANGUAGERESTRICTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const NET_E_DISCONNECTED: i32 = -2147220733i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const NET_E_GENERAL: i32 = -2147220736i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const NET_E_INVALIDPARAMS: i32 = -2147220728i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const NET_E_OPERATIONINPROGRESS: i32 = -2147220727i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const NLADMIN_E_BUILD_CATALOG_NOT_INITIALIZED: i32 = -2147215100i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const NLADMIN_E_DUPLICATE_CATALOG: i32 = -2147215103i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const NLADMIN_E_FAILED_TO_GIVE_ACCOUNT_PRIVILEGE: i32 = -2147215101i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const NLADMIN_S_NOT_ALL_BUILD_CATALOGS_INITIALIZED: i32 = 268546i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
pub struct NODERESTRICTION {
    pub cRes: u32,
    pub paRes: *mut *mut RESTRICTION,
    pub reserved: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for NODERESTRICTION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for NODERESTRICTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const NOTESPH_E_ATTACHMENTS: i32 = -2147211770i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const NOTESPH_E_DB_ACCESS_DENIED: i32 = -2147211768i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const NOTESPH_E_FAIL: i32 = -2147211759i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const NOTESPH_E_ITEM_NOT_FOUND: i32 = -2147211772i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const NOTESPH_E_NOTESSETUP_ID_MAPPING_ERROR: i32 = -2147211767i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const NOTESPH_E_NO_NTID: i32 = -2147211769i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const NOTESPH_E_SERVER_CONFIG: i32 = -2147211771i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const NOTESPH_E_UNEXPECTED_STATE: i32 = -2147211775i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const NOTESPH_E_UNSUPPORTED_CONTENT_FIELD_TYPE: i32 = -2147211773i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const NOTESPH_S_IGNORE_ID: i32 = 271874i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const NOTESPH_S_LISTKNOWNFIELDS: i32 = 271888i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
pub struct NOTRESTRICTION {
    pub pRes: *mut RESTRICTION,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for NOTRESTRICTION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for NOTRESTRICTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const NOT_N_PARSE_ERROR: ::windows_sys::core::HRESULT = 526638i32;
pub const NegationCondition: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2380908364, data2: 24666, data3: 19149, data4: [190, 227, 43, 34, 42, 162, 210, 61] };
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const OCC_INVALID: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBCVER: u32 = 896u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_ADD_DSN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_ADD_SYS_DSN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_BOTH_DSN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_CONFIG_DRIVER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_CONFIG_DRIVER_MAX: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_CONFIG_DSN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_CONFIG_SYS_DSN: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_ERROR_COMPONENT_NOT_FOUND: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_ERROR_CREATE_DSN_FAILED: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_ERROR_GENERAL_ERR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_ERROR_INVALID_BUFF_LEN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_ERROR_INVALID_DSN: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_ERROR_INVALID_HWND: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_ERROR_INVALID_INF: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_ERROR_INVALID_KEYWORD_VALUE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_ERROR_INVALID_LOG_FILE: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_ERROR_INVALID_NAME: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_ERROR_INVALID_PARAM_SEQUENCE: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_ERROR_INVALID_PATH: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_ERROR_INVALID_REQUEST_TYPE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_ERROR_INVALID_STR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_ERROR_LOAD_LIB_FAILED: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_ERROR_MAX: u32 = 23u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_ERROR_NOTRANINFO: u32 = 23u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_ERROR_OUTPUT_STRING_TRUNCATED: u32 = 22u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_ERROR_OUT_OF_MEM: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_ERROR_REMOVE_DSN_FAILED: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_ERROR_REQUEST_FAILED: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_ERROR_USAGE_UPDATE_FAILED: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_ERROR_USER_CANCELED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_ERROR_WRITING_SYSINFO_FAILED: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_INSTALL_COMPLETE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_INSTALL_DRIVER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_INSTALL_INQUIRY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_REMOVE_DEFAULT_DSN: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_REMOVE_DRIVER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_REMOVE_DSN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_REMOVE_SYS_DSN: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_SYSTEM_DSN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_USER_DSN: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub struct ODBC_VS_ARGS {
    pub pguidEvent: *const ::windows_sys::core::GUID,
    pub dwFlags: u32,
    pub Anonymous1: ODBC_VS_ARGS_0,
    pub Anonymous2: ODBC_VS_ARGS_1,
    pub RetCode: i16,
}
impl ::core::marker::Copy for ODBC_VS_ARGS {}
impl ::core::clone::Clone for ODBC_VS_ARGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub union ODBC_VS_ARGS_0 {
    pub wszArg: ::windows_sys::core::PWSTR,
    pub szArg: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for ODBC_VS_ARGS_0 {}
impl ::core::clone::Clone for ODBC_VS_ARGS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub union ODBC_VS_ARGS_1 {
    pub wszCorrelation: ::windows_sys::core::PWSTR,
    pub szCorrelation: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for ODBC_VS_ARGS_1 {}
impl ::core::clone::Clone for ODBC_VS_ARGS_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_VS_FLAG_RETCODE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_VS_FLAG_STOP: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_VS_FLAG_UNICODE_ARG: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ODBC_VS_FLAG_UNICODE_COR: i32 = 2i32;
#[repr(C)]
pub struct OLEDBSimpleProvider {
    pub base__: ::windows_sys::core::IUnknown,
    pub getRowCount: unsafe extern "system" fn(this: *mut *mut Self, pcrows: *mut isize) -> ::windows_sys::core::HRESULT,
    pub getColumnCount: unsafe extern "system" fn(this: *mut *mut Self, pccolumns: *mut isize) -> ::windows_sys::core::HRESULT,
    pub getRWStatus: unsafe extern "system" fn(this: *mut *mut Self, irow: isize, icolumn: isize, prwstatus: *mut OSPRW) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub getVariant: unsafe extern "system" fn(this: *mut *mut Self, irow: isize, icolumn: isize, format: OSPFORMAT, pvar: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    getVariant: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub setVariant: unsafe extern "system" fn(this: *mut *mut Self, irow: isize, icolumn: isize, format: OSPFORMAT, var: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    setVariant: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub getLocale: unsafe extern "system" fn(this: *mut *mut Self, pbstrlocale: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    getLocale: usize,
    pub deleteRows: unsafe extern "system" fn(this: *mut *mut Self, irow: isize, crows: isize, pcrowsdeleted: *mut isize) -> ::windows_sys::core::HRESULT,
    pub insertRows: unsafe extern "system" fn(this: *mut *mut Self, irow: isize, crows: isize, pcrowsinserted: *mut isize) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub find: unsafe extern "system" fn(this: *mut *mut Self, irowstart: isize, icolumn: isize, val: ::core::mem::ManuallyDrop<super::Com::VARIANT>, findflags: OSPFIND, comptype: OSPCOMP, pirowfound: *mut isize) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    find: usize,
    pub addOLEDBSimpleProviderListener: unsafe extern "system" fn(this: *mut *mut Self, pospilistener: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub removeOLEDBSimpleProviderListener: unsafe extern "system" fn(this: *mut *mut Self, pospilistener: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub isAsync: unsafe extern "system" fn(this: *mut *mut Self, pbasynch: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    isAsync: usize,
    pub getEstimatedRows: unsafe extern "system" fn(this: *mut *mut Self, pirows: *mut isize) -> ::windows_sys::core::HRESULT,
    pub stopTransfer: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for OLEDBSimpleProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3772936384, data2: 49342, data3: 4560, data4: [143, 228, 0, 160, 201, 10, 99, 65] };
}
#[repr(C)]
pub struct OLEDBSimpleProviderListener {
    pub base__: ::windows_sys::core::IUnknown,
    pub aboutToChangeCell: unsafe extern "system" fn(this: *mut *mut Self, irow: isize, icolumn: isize) -> ::windows_sys::core::HRESULT,
    pub cellChanged: unsafe extern "system" fn(this: *mut *mut Self, irow: isize, icolumn: isize) -> ::windows_sys::core::HRESULT,
    pub aboutToDeleteRows: unsafe extern "system" fn(this: *mut *mut Self, irow: isize, crows: isize) -> ::windows_sys::core::HRESULT,
    pub deletedRows: unsafe extern "system" fn(this: *mut *mut Self, irow: isize, crows: isize) -> ::windows_sys::core::HRESULT,
    pub aboutToInsertRows: unsafe extern "system" fn(this: *mut *mut Self, irow: isize, crows: isize) -> ::windows_sys::core::HRESULT,
    pub insertedRows: unsafe extern "system" fn(this: *mut *mut Self, irow: isize, crows: isize) -> ::windows_sys::core::HRESULT,
    pub rowsAvailable: unsafe extern "system" fn(this: *mut *mut Self, irow: isize, crows: isize) -> ::windows_sys::core::HRESULT,
    pub transferComplete: unsafe extern "system" fn(this: *mut *mut Self, xfer: OSPXFER) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for OLEDBSimpleProviderListener {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3772936385, data2: 49342, data3: 4560, data4: [143, 228, 0, 160, 201, 10, 99, 65] };
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const OLEDBVER: u32 = 624u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const OLEDB_BINDER_CUSTOM_ERROR: i32 = -2147212032i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type OSPCOMP = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const OSPCOMP_EQ: OSPCOMP = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const OSPCOMP_DEFAULT: OSPCOMP = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const OSPCOMP_LT: OSPCOMP = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const OSPCOMP_LE: OSPCOMP = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const OSPCOMP_GE: OSPCOMP = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const OSPCOMP_GT: OSPCOMP = 5i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const OSPCOMP_NE: OSPCOMP = 6i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type OSPFIND = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const OSPFIND_DEFAULT: OSPFIND = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const OSPFIND_UP: OSPFIND = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const OSPFIND_CASESENSITIVE: OSPFIND = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const OSPFIND_UPCASESENSITIVE: OSPFIND = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type OSPFORMAT = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const OSPFORMAT_RAW: OSPFORMAT = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const OSPFORMAT_DEFAULT: OSPFORMAT = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const OSPFORMAT_FORMATTED: OSPFORMAT = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const OSPFORMAT_HTML: OSPFORMAT = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type OSPRW = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const OSPRW_DEFAULT: OSPRW = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const OSPRW_READONLY: OSPRW = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const OSPRW_READWRITE: OSPRW = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const OSPRW_MIXED: OSPRW = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type OSPXFER = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const OSPXFER_COMPLETE: OSPXFER = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const OSPXFER_ABORT: OSPXFER = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const OSPXFER_ERROR: OSPXFER = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const OSP_IndexLabel: u32 = 0u32;
pub const PDPO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3434409056, data2: 47580, data3: 4561, data4: [172, 128, 0, 160, 201, 3, 72, 115] };
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PEOPLE_IMPORT_E_CANONICALURL_TOOLONG: i32 = -2147205110i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PEOPLE_IMPORT_E_DATATYPENOTSUPPORTED: i32 = -2147205115i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PEOPLE_IMPORT_E_DBCONNFAIL: i32 = -2147205120i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PEOPLE_IMPORT_E_DC_NOT_AVAILABLE: i32 = -2147205108i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PEOPLE_IMPORT_E_DIRSYNC_NOTREFRESHED: i32 = -2147205103i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PEOPLE_IMPORT_E_DIRSYNC_ZERO_COOKIE: i32 = -2147205112i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PEOPLE_IMPORT_E_DOMAIN_DISCOVER_FAILED: i32 = -2147205107i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PEOPLE_IMPORT_E_DOMAIN_REMOVED: i32 = -2147205105i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PEOPLE_IMPORT_E_ENUM_ACCESSDENIED: i32 = -2147205104i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PEOPLE_IMPORT_E_FAILTOGETDSDEF: i32 = -2147205118i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PEOPLE_IMPORT_E_FAILTOGETDSMAPPING: i32 = -2147205116i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PEOPLE_IMPORT_E_FAILTOGETLCID: i32 = -2147205106i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PEOPLE_IMPORT_E_LDAPPATH_TOOLONG: i32 = -2147205111i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PEOPLE_IMPORT_E_NOCASTINGSUPPORTED: i32 = -2147205114i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PEOPLE_IMPORT_E_UPDATE_DIRSYNC_COOKIE: i32 = -2147205113i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PEOPLE_IMPORT_E_USERNAME_NOTRESOLVED: i32 = -2147205109i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PEOPLE_IMPORT_NODSDEFINED: i32 = -2147205119i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PEOPLE_IMPORT_NOMAPPINGDEFINED: i32 = -2147205117i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type PFNFILLTEXTBUFFER = ::core::option::Option<unsafe extern "system" fn(ptextsource: *mut TEXT_SOURCE) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PRAll: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PRAllBits: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PRAny: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type PRIORITIZE_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PRIORITIZE_FLAG_RETRYFAILEDITEMS: PRIORITIZE_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PRIORITIZE_FLAG_IGNOREFAILURECOUNT: PRIORITIZE_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type PRIORITY_LEVEL = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PRIORITY_LEVEL_FOREGROUND: PRIORITY_LEVEL = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PRIORITY_LEVEL_HIGH: PRIORITY_LEVEL = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PRIORITY_LEVEL_LOW: PRIORITY_LEVEL = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PRIORITY_LEVEL_DEFAULT: PRIORITY_LEVEL = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PROGID_MSPersist_Version_W: &str = "MSPersist.1";
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PROGID_MSPersist_W: &str = "MSPersist";
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
pub struct PROPERTYRESTRICTION {
    pub rel: u32,
    pub prop: super::super::Storage::IndexServer::FULLPROPSPEC,
    pub prval: super::Com::StructuredStorage::PROPVARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for PROPERTYRESTRICTION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for PROPERTYRESTRICTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PROPID_DBBMK_BOOKMARK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PROPID_DBBMK_CHAPTER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PROPID_DBSELF_SELF: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type PROXY_ACCESS = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PROXY_ACCESS_PRECONFIG: PROXY_ACCESS = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PROXY_ACCESS_DIRECT: PROXY_ACCESS = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PROXY_ACCESS_PROXY: PROXY_ACCESS = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PROXY_INFO {
    pub dwSize: u32,
    pub pcwszUserAgent: ::windows_sys::core::PCWSTR,
    pub paUseProxy: PROXY_ACCESS,
    pub fLocalBypass: super::super::Foundation::BOOL,
    pub dwPortNumber: u32,
    pub pcwszProxyName: ::windows_sys::core::PCWSTR,
    pub pcwszBypassList: ::windows_sys::core::PCWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROXY_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROXY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PRRE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PRSomeBits: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PRTH_E_CANT_TRANSFORM_DENIED_ACE: i32 = -2147216881i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PRTH_E_CANT_TRANSFORM_EXTERNAL_ACL: i32 = -2147216882i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PRTH_E_DATABASE_OPEN_ERROR: i32 = -2147216875i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PRTH_E_HTTPS_CERTIFICATE_ERROR: i32 = -2147216861i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PRTH_E_HTTPS_REQUIRE_CERTIFICATE: i32 = -2147216860i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PRTH_E_INIT_FAILED: i32 = -2147216872i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PRTH_E_INTERNAL_ERROR: i32 = -2147216892i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PRTH_E_LOAD_FAILED: i32 = -2147216873i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PRTH_E_MIME_EXCLUDED: i32 = -2147216883i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PRTH_E_NO_PROPERTY: i32 = -2147216877i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PRTH_E_OPLOCK_BROKEN: i32 = -2147216874i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PRTH_E_RETRY: i32 = -2147216885i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PRTH_E_TRUNCATED: i32 = -2147216870i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PRTH_E_VOLUME_MOUNT_POINT: i32 = -2147216871i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PRTH_E_WININET: i32 = -2147216886i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PRTH_S_MAX_DOWNLOAD: i32 = 266764i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PRTH_S_MAX_GROWTH: i32 = 266761i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PRTH_S_TRY_IMPERSONATING: i32 = 266789i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PRTH_S_USE_ROSEBUD: i32 = 266772i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const PWPROP_OSPVALUE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QRY_E_COLUMNNOTSEARCHABLE: i32 = -2147219700i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QRY_E_COLUMNNOTSORTABLE: i32 = -2147219701i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QRY_E_ENGINEFAILED: i32 = -2147219693i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QRY_E_INFIXWILDCARD: i32 = -2147219696i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QRY_E_INVALIDCATALOG: i32 = -2147219687i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QRY_E_INVALIDCOLUMN: i32 = -2147219699i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QRY_E_INVALIDINTERVAL: i32 = -2147219682i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QRY_E_INVALIDPATH: i32 = -2147219684i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QRY_E_INVALIDSCOPES: i32 = -2147219688i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QRY_E_LMNOTINITIALIZED: i32 = -2147219683i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QRY_E_NOCOLUMNS: i32 = -2147219689i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QRY_E_NODATASOURCES: i32 = -2147219703i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QRY_E_NOLOGMANAGER: i32 = -2147219681i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QRY_E_NULLQUERY: i32 = -2147219691i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QRY_E_PREFIXWILDCARD: i32 = -2147219697i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QRY_E_QUERYCORRUPT: i32 = -2147219698i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QRY_E_QUERYSYNTAX: i32 = -2147219711i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QRY_E_SCOPECARDINALIDY: i32 = -2147219686i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QRY_E_SEARCHTOOBIG: i32 = -2147219692i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QRY_E_STARTHITTOBIG: i32 = -2147219705i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QRY_E_TIMEOUT: i32 = -2147219702i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QRY_E_TOOMANYCOLUMNS: i32 = -2147219707i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QRY_E_TOOMANYDATABASES: i32 = -2147219706i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QRY_E_TOOMANYQUERYTERMS: i32 = -2147219704i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QRY_E_TYPEMISMATCH: i32 = -2147219710i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QRY_E_UNEXPECTED: i32 = -2147219685i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QRY_E_UNHANDLEDTYPE: i32 = -2147219709i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QRY_E_WILDCARDPREFIXLENGTH: i32 = -2147219695i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QRY_S_INEXACTRESULTS: i32 = 263958i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QRY_S_NOROWSFOUND: i32 = 263940i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QRY_S_TERMIGNORED: i32 = 263954i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QUERY_E_AGGREGATE_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2147215847i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QUERY_E_ALLNOISE_AND_NO_RELDOC: ::windows_sys::core::HRESULT = -2147215859i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QUERY_E_ALLNOISE_AND_NO_RELPROP: ::windows_sys::core::HRESULT = -2147215857i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QUERY_E_DUPLICATE_RANGE_NAME: ::windows_sys::core::HRESULT = -2147215845i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QUERY_E_INCORRECT_VERSION: ::windows_sys::core::HRESULT = -2147215852i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QUERY_E_INVALIDCOALESCE: ::windows_sys::core::HRESULT = -2147215849i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QUERY_E_INVALIDSCOPE_COALESCE: ::windows_sys::core::HRESULT = -2147215851i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QUERY_E_INVALIDSORT_COALESCE: ::windows_sys::core::HRESULT = -2147215850i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QUERY_E_INVALID_DOCUMENT_IDENTIFIER: ::windows_sys::core::HRESULT = -2147215853i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QUERY_E_NO_RELDOC: ::windows_sys::core::HRESULT = -2147215858i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QUERY_E_NO_RELPROP: ::windows_sys::core::HRESULT = -2147215856i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QUERY_E_RELDOC_SYNTAX_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2147215854i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QUERY_E_REPEATED_RELDOC: ::windows_sys::core::HRESULT = -2147215855i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QUERY_E_TOP_LEVEL_IN_GROUP: ::windows_sys::core::HRESULT = -2147215846i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QUERY_E_UPGRADEINPROGRESS: ::windows_sys::core::HRESULT = -2147215848i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type QUERY_PARSER_MANAGER_OPTION = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QPMO_SCHEMA_BINARY_NAME: QUERY_PARSER_MANAGER_OPTION = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QPMO_PRELOCALIZED_SCHEMA_BINARY_PATH: QUERY_PARSER_MANAGER_OPTION = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QPMO_UNLOCALIZED_SCHEMA_BINARY_PATH: QUERY_PARSER_MANAGER_OPTION = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QPMO_LOCALIZED_SCHEMA_BINARY_PATH: QUERY_PARSER_MANAGER_OPTION = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QPMO_APPEND_LCID_TO_LOCALIZED_PATH: QUERY_PARSER_MANAGER_OPTION = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QPMO_LOCALIZER_SUPPORT: QUERY_PARSER_MANAGER_OPTION = 5i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QUERY_SORTDEFAULT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QUERY_SORTXASCEND: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QUERY_SORTXDESCEND: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const QUERY_VALIDBITS: u32 = 3u32;
pub const QueryParser: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3073347544, data2: 4011, data3: 19929, data4: [189, 191, 36, 90, 108, 225, 72, 91] };
pub const QueryParserManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1351136154, data2: 10676, data3: 19869, data4: [130, 69, 78, 226, 137, 34, 47, 102] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub struct RANGECATEGORIZE {
    pub cRange: u32,
    pub aRangeBegin: *mut super::Com::StructuredStorage::PROPVARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for RANGECATEGORIZE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for RANGECATEGORIZE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
pub struct RESTRICTION {
    pub rt: u32,
    pub weight: u32,
    pub res: RESTRICTION_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for RESTRICTION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for RESTRICTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
pub union RESTRICTION_0 {
    pub ar: NODERESTRICTION,
    pub orRestriction: NODERESTRICTION,
    pub pxr: NODERESTRICTION,
    pub vr: VECTORRESTRICTION,
    pub nr: NOTRESTRICTION,
    pub cr: CONTENTRESTRICTION,
    pub nlr: NATLANGUAGERESTRICTION,
    pub pr: PROPERTYRESTRICTION,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for RESTRICTION_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for RESTRICTION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const REXSPH_E_DUPLICATE_PROPERTY: i32 = -2147207927i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const REXSPH_E_INVALID_CALL: i32 = -2147207936i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const REXSPH_E_MULTIPLE_REDIRECT: i32 = -2147207933i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const REXSPH_E_NO_PROPERTY_ON_ROW: i32 = -2147207932i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const REXSPH_E_REDIRECT_ON_SECURITY_UPDATE: i32 = -2147207934i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const REXSPH_E_TYPE_MISMATCH_ON_READ: i32 = -2147207931i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const REXSPH_E_UNEXPECTED_DATA_STATUS: i32 = -2147207930i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const REXSPH_E_UNEXPECTED_FILTER_STATE: i32 = -2147207928i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const REXSPH_E_UNKNOWN_DATA_TYPE: i32 = -2147207929i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const REXSPH_S_REDIRECTED: i32 = 275713i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
pub struct RMTPACK {
    pub pISeqStream: *mut *mut *mut *mut super::Com::ISequentialStream,
    pub cbData: u32,
    pub cBSTR: u32,
    pub rgBSTR: *mut super::super::Foundation::BSTR,
    pub cVARIANT: u32,
    pub rgVARIANT: *mut super::Com::VARIANT,
    pub cIDISPATCH: u32,
    pub rgIDISPATCH: *mut *mut *mut super::Com::IDispatch,
    pub cIUNKNOWN: u32,
    pub rgIUNKNOWN: *mut *mut *mut ::windows_sys::core::IUnknown,
    pub cPROPVARIANT: u32,
    pub rgPROPVARIANT: *mut super::Com::StructuredStorage::PROPVARIANT,
    pub cArray: u32,
    pub rgArray: *mut super::Com::VARIANT,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for RMTPACK {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for RMTPACK {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
pub struct RMTPACK {
    pub pISeqStream: *mut *mut *mut *mut super::Com::ISequentialStream,
    pub cbData: u32,
    pub cBSTR: u32,
    pub rgBSTR: *mut super::super::Foundation::BSTR,
    pub cVARIANT: u32,
    pub rgVARIANT: *mut super::Com::VARIANT,
    pub cIDISPATCH: u32,
    pub rgIDISPATCH: *mut *mut *mut super::Com::IDispatch,
    pub cIUNKNOWN: u32,
    pub rgIUNKNOWN: *mut *mut *mut ::windows_sys::core::IUnknown,
    pub cPROPVARIANT: u32,
    pub rgPROPVARIANT: *mut super::Com::StructuredStorage::PROPVARIANT,
    pub cArray: u32,
    pub rgArray: *mut super::Com::VARIANT,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for RMTPACK {}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for RMTPACK {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type ROWSETEVENT_ITEMSTATE = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ROWSETEVENT_ITEMSTATE_NOTINROWSET: ROWSETEVENT_ITEMSTATE = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ROWSETEVENT_ITEMSTATE_INROWSET: ROWSETEVENT_ITEMSTATE = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ROWSETEVENT_ITEMSTATE_UNKNOWN: ROWSETEVENT_ITEMSTATE = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type ROWSETEVENT_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ROWSETEVENT_TYPE_DATAEXPIRED: ROWSETEVENT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ROWSETEVENT_TYPE_FOREGROUNDLOST: ROWSETEVENT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const ROWSETEVENT_TYPE_SCOPESTATISTICS: ROWSETEVENT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const RS_COMPLETED: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const RS_MAYBOTHERUSER: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const RS_READY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const RS_SUSPENDED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const RS_SUSPENDONIDLE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const RS_UPDATING: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const RTAnd: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const RTContent: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const RTNatLanguage: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const RTNone: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const RTNot: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const RTOr: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const RTProperty: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const RTProximity: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const RTVector: u32 = 7u32;
pub const RootBinder: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4279572514, data2: 45247, data3: 4561, data4: [168, 13, 0, 0, 0, 0, 0, 0] };
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SCHEMA_E_ADDSTOPWORDS: i32 = -2147218420i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SCHEMA_E_BADATTRIBUTE: i32 = -2147218412i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SCHEMA_E_BADCOLUMNNAME: i32 = -2147218414i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SCHEMA_E_BADFILENAME: i32 = -2147218411i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SCHEMA_E_BADPROPPID: i32 = -2147218413i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SCHEMA_E_BADPROPSPEC: i32 = -2147218417i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SCHEMA_E_CANNOTCREATEFILE: i32 = -2147218426i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SCHEMA_E_CANNOTCREATENOISEWORDFILE: i32 = -2147218421i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SCHEMA_E_CANNOTWRITEFILE: i32 = -2147218425i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SCHEMA_E_DUPLICATENOISE: i32 = -2147218409i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SCHEMA_E_EMPTYFILE: i32 = -2147218424i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SCHEMA_E_FILECHANGED: i32 = -2147218415i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SCHEMA_E_FILENOTFOUND: i32 = -2147218430i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SCHEMA_E_INVALIDDATATYPE: i32 = -2147218422i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SCHEMA_E_INVALIDFILETYPE: i32 = -2147218423i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SCHEMA_E_INVALIDVALUE: i32 = -2147218418i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SCHEMA_E_LOAD_SPECIAL: i32 = -2147218431i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SCHEMA_E_NAMEEXISTS: i32 = -2147218419i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SCHEMA_E_NESTEDTAG: i32 = -2147218429i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SCHEMA_E_NOMORECOLUMNS: i32 = -2147218416i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SCHEMA_E_PROPEXISTS: i32 = -2147218410i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SCHEMA_E_UNEXPECTEDTAG: i32 = -2147218428i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SCHEMA_E_VERSIONMISMATCH: i32 = -2147218427i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SCRIPTPI_E_ALREADY_COMPLETED: i32 = -2147213307i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SCRIPTPI_E_CANNOT_ALTER_CHUNK: i32 = -2147213308i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SCRIPTPI_E_CHUNK_NOT_TEXT: i32 = -2147213312i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SCRIPTPI_E_CHUNK_NOT_VALUE: i32 = -2147213309i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SCRIPTPI_E_PID_NOT_NAME: i32 = -2147213311i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SCRIPTPI_E_PID_NOT_NUMERIC: i32 = -2147213310i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub struct SEARCH_COLUMN_PROPERTIES {
    pub Value: super::Com::StructuredStorage::PROPVARIANT,
    pub lcid: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for SEARCH_COLUMN_PROPERTIES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for SEARCH_COLUMN_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type SEARCH_INDEXING_PHASE = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SEARCH_INDEXING_PHASE_GATHERER: SEARCH_INDEXING_PHASE = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SEARCH_INDEXING_PHASE_QUERYABLE: SEARCH_INDEXING_PHASE = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SEARCH_INDEXING_PHASE_PERSISTED: SEARCH_INDEXING_PHASE = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct SEARCH_ITEM_CHANGE {
    pub Change: SEARCH_KIND_OF_CHANGE,
    pub Priority: SEARCH_NOTIFICATION_PRIORITY,
    pub pUserData: *mut super::Com::BLOB,
    pub lpwszURL: ::windows_sys::core::PWSTR,
    pub lpwszOldURL: ::windows_sys::core::PWSTR,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for SEARCH_ITEM_CHANGE {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for SEARCH_ITEM_CHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub struct SEARCH_ITEM_INDEXING_STATUS {
    pub dwDocID: u32,
    pub hrIndexingStatus: ::windows_sys::core::HRESULT,
}
impl ::core::marker::Copy for SEARCH_ITEM_INDEXING_STATUS {}
impl ::core::clone::Clone for SEARCH_ITEM_INDEXING_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub struct SEARCH_ITEM_PERSISTENT_CHANGE {
    pub Change: SEARCH_KIND_OF_CHANGE,
    pub URL: ::windows_sys::core::PWSTR,
    pub OldURL: ::windows_sys::core::PWSTR,
    pub Priority: SEARCH_NOTIFICATION_PRIORITY,
}
impl ::core::marker::Copy for SEARCH_ITEM_PERSISTENT_CHANGE {}
impl ::core::clone::Clone for SEARCH_ITEM_PERSISTENT_CHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type SEARCH_KIND_OF_CHANGE = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SEARCH_CHANGE_ADD: SEARCH_KIND_OF_CHANGE = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SEARCH_CHANGE_DELETE: SEARCH_KIND_OF_CHANGE = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SEARCH_CHANGE_MODIFY: SEARCH_KIND_OF_CHANGE = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SEARCH_CHANGE_MOVE_RENAME: SEARCH_KIND_OF_CHANGE = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SEARCH_CHANGE_SEMANTICS_DIRECTORY: SEARCH_KIND_OF_CHANGE = 262144i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SEARCH_CHANGE_SEMANTICS_SHALLOW: SEARCH_KIND_OF_CHANGE = 524288i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SEARCH_CHANGE_SEMANTICS_UPDATE_SECURITY: SEARCH_KIND_OF_CHANGE = 4194304i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type SEARCH_NOTIFICATION_PRIORITY = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SEARCH_NORMAL_PRIORITY: SEARCH_NOTIFICATION_PRIORITY = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SEARCH_HIGH_PRIORITY: SEARCH_NOTIFICATION_PRIORITY = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type SEARCH_QUERY_SYNTAX = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SEARCH_NO_QUERY_SYNTAX: SEARCH_QUERY_SYNTAX = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SEARCH_ADVANCED_QUERY_SYNTAX: SEARCH_QUERY_SYNTAX = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SEARCH_NATURAL_QUERY_SYNTAX: SEARCH_QUERY_SYNTAX = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type SEARCH_TERM_EXPANSION = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SEARCH_TERM_NO_EXPANSION: SEARCH_TERM_EXPANSION = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SEARCH_TERM_PREFIX_ALL: SEARCH_TERM_EXPANSION = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SEARCH_TERM_STEM_ALL: SEARCH_TERM_EXPANSION = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SEC_E_ACCESSDENIED: i32 = -2147216129i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SEC_E_BADTRUSTEEID: ::windows_sys::core::HRESULT = -2147217814i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SEC_E_INITFAILED: i32 = -2147216383i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SEC_E_INVALIDACCESSENTRY: ::windows_sys::core::HRESULT = -2147217807i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SEC_E_INVALIDACCESSENTRYLIST: ::windows_sys::core::HRESULT = -2147217809i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SEC_E_INVALIDCONTEXT: i32 = -2147216381i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SEC_E_INVALIDOBJECT: ::windows_sys::core::HRESULT = -2147217811i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SEC_E_INVALIDOWNER: ::windows_sys::core::HRESULT = -2147217808i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SEC_E_NOMEMBERSHIPSUPPORT: ::windows_sys::core::HRESULT = -2147217812i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SEC_E_NOOWNER: ::windows_sys::core::HRESULT = -2147217810i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SEC_E_NOTINITIALIZED: i32 = -2147216382i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SEC_E_NOTRUSTEEID: ::windows_sys::core::HRESULT = -2147217813i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SEC_E_PERMISSIONDENIED: i32 = -2147217911i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Storage_IndexServer\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Storage_IndexServer")]
pub struct SEC_OBJECT {
    pub cObjects: u32,
    pub prgObjects: *mut SEC_OBJECT_ELEMENT,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ::core::marker::Copy for SEC_OBJECT {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ::core::clone::Clone for SEC_OBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Storage_IndexServer\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Storage_IndexServer")]
pub struct SEC_OBJECT {
    pub cObjects: u32,
    pub prgObjects: *mut SEC_OBJECT_ELEMENT,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ::core::marker::Copy for SEC_OBJECT {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ::core::clone::Clone for SEC_OBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Storage_IndexServer\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Storage_IndexServer")]
pub struct SEC_OBJECT_ELEMENT {
    pub guidObjectType: ::windows_sys::core::GUID,
    pub ObjectID: super::super::Storage::IndexServer::DBID,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ::core::marker::Copy for SEC_OBJECT_ELEMENT {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ::core::clone::Clone for SEC_OBJECT_ELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Storage_IndexServer\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Storage_IndexServer")]
pub struct SEC_OBJECT_ELEMENT {
    pub guidObjectType: ::windows_sys::core::GUID,
    pub ObjectID: super::super::Storage::IndexServer::DBID,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ::core::marker::Copy for SEC_OBJECT_ELEMENT {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ::core::clone::Clone for SEC_OBJECT_ELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SI_TEMPORARY: u32 = 2147483648u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
pub struct SORTKEY {
    pub propColumn: super::super::Storage::IndexServer::FULLPROPSPEC,
    pub dwOrder: u32,
    pub locale: u32,
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for SORTKEY {}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for SORTKEY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
pub struct SORTSET {
    pub cCol: u32,
    pub aCol: *mut SORTKEY,
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for SORTSET {}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for SORTSET {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SPS_WS_ERROR: i32 = -2147211753i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLAOPANY: u32 = 83u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLAOPAVG: u32 = 79u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLAOPCNT: u32 = 75u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLAOPMAX: u32 = 82u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLAOPMIN: u32 = 81u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLAOPNOOP: u32 = 86u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLAOPSTDEV: u32 = 48u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLAOPSTDEVP: u32 = 49u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLAOPSUM: u32 = 77u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLAOPVAR: u32 = 50u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLAOPVARP: u32 = 51u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLBIGBINARY: u32 = 173u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLBIGCHAR: u32 = 175u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLBIGVARBINARY: u32 = 165u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLBIGVARCHAR: u32 = 167u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLBINARY: u32 = 45u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLBIT: u32 = 50u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLBITN: u32 = 104u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLCHARACTER: u32 = 47u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLDATETIM4: u32 = 58u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLDATETIME: u32 = 61u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLDATETIMN: u32 = 111u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLDECIMAL: u32 = 106u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLDECIMALN: u32 = 106u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLFLT4: u32 = 59u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLFLT8: u32 = 62u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLFLTN: u32 = 109u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLIMAGE: u32 = 34u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLINT1: u32 = 48u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLINT2: u32 = 52u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLINT4: u32 = 56u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLINT8: u32 = 127u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type SQLINTERVAL = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_IS_YEAR: SQLINTERVAL = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_IS_MONTH: SQLINTERVAL = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_IS_DAY: SQLINTERVAL = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_IS_HOUR: SQLINTERVAL = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_IS_MINUTE: SQLINTERVAL = 5i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_IS_SECOND: SQLINTERVAL = 6i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_IS_YEAR_TO_MONTH: SQLINTERVAL = 7i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_IS_DAY_TO_HOUR: SQLINTERVAL = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_IS_DAY_TO_MINUTE: SQLINTERVAL = 9i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_IS_DAY_TO_SECOND: SQLINTERVAL = 10i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_IS_HOUR_TO_MINUTE: SQLINTERVAL = 11i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_IS_HOUR_TO_SECOND: SQLINTERVAL = 12i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_IS_MINUTE_TO_SECOND: SQLINTERVAL = 13i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLINTN: u32 = 38u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLMONEY: u32 = 60u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLMONEY4: u32 = 122u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLMONEYN: u32 = 110u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLNCHAR: u32 = 239u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLNTEXT: u32 = 99u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLNUMERIC: u32 = 108u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLNUMERICN: u32 = 108u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLNVARCHAR: u32 = 231u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLTEXT: u32 = 35u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLUNIQUEID: u32 = 36u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLVARBINARY: u32 = 37u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLVARCHAR: u32 = 39u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type SQLVARENUM = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const VT_SS_EMPTY: SQLVARENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const VT_SS_NULL: SQLVARENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const VT_SS_UI1: SQLVARENUM = 17i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const VT_SS_I2: SQLVARENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const VT_SS_I4: SQLVARENUM = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const VT_SS_I8: SQLVARENUM = 20i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const VT_SS_R4: SQLVARENUM = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const VT_SS_R8: SQLVARENUM = 5i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const VT_SS_MONEY: SQLVARENUM = 6i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const VT_SS_SMALLMONEY: SQLVARENUM = 200i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const VT_SS_WSTRING: SQLVARENUM = 201i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const VT_SS_WVARSTRING: SQLVARENUM = 202i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const VT_SS_STRING: SQLVARENUM = 203i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const VT_SS_VARSTRING: SQLVARENUM = 204i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const VT_SS_BIT: SQLVARENUM = 11i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const VT_SS_GUID: SQLVARENUM = 72i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const VT_SS_NUMERIC: SQLVARENUM = 131i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const VT_SS_DECIMAL: SQLVARENUM = 205i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const VT_SS_DATETIME: SQLVARENUM = 135i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const VT_SS_SMALLDATETIME: SQLVARENUM = 206i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const VT_SS_BINARY: SQLVARENUM = 207i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const VT_SS_VARBINARY: SQLVARENUM = 208i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const VT_SS_UNKNOWN: SQLVARENUM = 209i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLVARIANT: u32 = 98u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AA_FALSE: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AA_TRUE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ACCESSIBLE_PROCEDURES: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ACCESSIBLE_TABLES: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ACCESS_MODE: u32 = 101u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ACTIVE_CONNECTIONS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ACTIVE_ENVIRONMENTS: u32 = 116u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ACTIVE_STATEMENTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ADD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AD_ADD_CONSTRAINT_DEFERRABLE: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AD_ADD_CONSTRAINT_INITIALLY_DEFERRED: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AD_ADD_CONSTRAINT_INITIALLY_IMMEDIATE: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AD_ADD_CONSTRAINT_NON_DEFERRABLE: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AD_ADD_DOMAIN_CONSTRAINT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AD_ADD_DOMAIN_DEFAULT: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AD_CONSTRAINT_NAME_DEFINITION: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AD_DEFAULT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AD_DROP_DOMAIN_CONSTRAINT: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AD_DROP_DOMAIN_DEFAULT: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AD_OFF: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AD_ON: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AF_ALL: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AF_AVG: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AF_COUNT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AF_DISTINCT: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AF_MAX: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AF_MIN: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AF_SUM: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AGGREGATE_FUNCTIONS: u32 = 169u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ALL_CATALOGS: &str = "%";
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ALL_EXCEPT_LIKE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ALL_SCHEMAS: &str = "%";
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ALL_TABLE_TYPES: &str = "%";
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ALL_TYPES: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ALTER_DOMAIN: u32 = 117u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ALTER_TABLE: u32 = 86u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AM_CONNECTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AM_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AM_STATEMENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AO_DEFAULT: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AO_OFF: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AO_ON: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_APD_TYPE: i32 = -100i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_ALL_FUNCTIONS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_LOADBYORDINAL: u32 = 199u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_ODBC3_ALL_FUNCTIONS: u32 = 999u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_ODBC3_ALL_FUNCTIONS_SIZE: u32 = 250u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLALLOCCONNECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLALLOCENV: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLALLOCHANDLE: u32 = 1001u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLALLOCHANDLESTD: u32 = 73u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLALLOCSTMT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLBINDCOL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLBINDPARAM: u32 = 1002u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLBINDPARAMETER: u32 = 72u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLBROWSECONNECT: u32 = 55u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLBULKOPERATIONS: u32 = 24u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLCANCEL: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLCANCELHANDLE: u32 = 1550u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLCLOSECURSOR: u32 = 1003u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLCOLATTRIBUTE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLCOLATTRIBUTES: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLCOLUMNPRIVILEGES: u32 = 56u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLCOLUMNS: u32 = 40u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLCOMPLETEASYNC: u32 = 1551u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLCONNECT: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLCOPYDESC: u32 = 1004u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLDATASOURCES: u32 = 57u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLDESCRIBECOL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLDESCRIBEPARAM: u32 = 58u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLDISCONNECT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLDRIVERCONNECT: u32 = 41u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLDRIVERS: u32 = 71u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLENDTRAN: u32 = 1005u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLERROR: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLEXECDIRECT: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLEXECUTE: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLEXTENDEDFETCH: u32 = 59u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLFETCH: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLFETCHSCROLL: u32 = 1021u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLFOREIGNKEYS: u32 = 60u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLFREECONNECT: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLFREEENV: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLFREEHANDLE: u32 = 1006u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLFREESTMT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLGETCONNECTATTR: u32 = 1007u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLGETCONNECTOPTION: u32 = 42u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLGETCURSORNAME: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLGETDATA: u32 = 43u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLGETDESCFIELD: u32 = 1008u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLGETDESCREC: u32 = 1009u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLGETDIAGFIELD: u32 = 1010u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLGETDIAGREC: u32 = 1011u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLGETENVATTR: u32 = 1012u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLGETFUNCTIONS: u32 = 44u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLGETINFO: u32 = 45u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLGETSTMTATTR: u32 = 1014u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLGETSTMTOPTION: u32 = 46u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLGETTYPEINFO: u32 = 47u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLMORERESULTS: u32 = 61u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLNATIVESQL: u32 = 62u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLNUMPARAMS: u32 = 63u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLNUMRESULTCOLS: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLPARAMDATA: u32 = 48u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLPARAMOPTIONS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLPREPARE: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLPRIMARYKEYS: u32 = 65u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLPRIVATEDRIVERS: u32 = 79u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLPROCEDURECOLUMNS: u32 = 66u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLPROCEDURES: u32 = 67u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLPUTDATA: u32 = 49u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLROWCOUNT: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLSETCONNECTATTR: u32 = 1016u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLSETCONNECTOPTION: u32 = 50u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLSETCURSORNAME: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLSETDESCFIELD: u32 = 1017u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLSETDESCREC: u32 = 1018u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLSETENVATTR: u32 = 1019u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLSETPARAM: u32 = 22u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLSETPOS: u32 = 68u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLSETSCROLLOPTIONS: u32 = 69u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLSETSTMTATTR: u32 = 1020u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLSETSTMTOPTION: u32 = 51u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLSPECIALCOLUMNS: u32 = 52u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLSTATISTICS: u32 = 53u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLTABLEPRIVILEGES: u32 = 70u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLTABLES: u32 = 54u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_API_SQLTRANSACT: u32 = 23u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ARD_TYPE: i32 = -99i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ASYNC_DBC_CAPABLE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ASYNC_DBC_ENABLE_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ASYNC_DBC_ENABLE_OFF: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ASYNC_DBC_ENABLE_ON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ASYNC_DBC_FUNCTIONS: u32 = 10023u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ASYNC_DBC_NOT_CAPABLE: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ASYNC_ENABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ASYNC_ENABLE_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ASYNC_ENABLE_OFF: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ASYNC_ENABLE_ON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ASYNC_MODE: u32 = 10021u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ASYNC_NOTIFICATION: u32 = 10025u32;
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SQL_ASYNC_NOTIFICATION_CALLBACK = ::core::option::Option<unsafe extern "system" fn(pcontext: *const ::core::ffi::c_void, flast: super::super::Foundation::BOOL) -> i16>;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ASYNC_NOTIFICATION_CAPABLE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ASYNC_NOTIFICATION_NOT_CAPABLE: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_ACCESS_MODE: u32 = 101u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_ANSI_APP: u32 = 115u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_APPLICATION_KEY: u32 = 203u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_APP_PARAM_DESC: u32 = 10011u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_APP_ROW_DESC: u32 = 10010u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_ASYNC_DBC_EVENT: u32 = 119u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_ASYNC_DBC_FUNCTIONS_ENABLE: u32 = 117u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_ASYNC_DBC_NOTIFICATION_CALLBACK: u32 = 120u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_ASYNC_DBC_NOTIFICATION_CONTEXT: u32 = 121u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_ASYNC_ENABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_ASYNC_STMT_EVENT: u32 = 29u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_ASYNC_STMT_NOTIFICATION_CALLBACK: u32 = 30u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_ASYNC_STMT_NOTIFICATION_CONTEXT: u32 = 31u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_AUTOCOMMIT: u32 = 102u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_AUTO_IPD: u32 = 10001u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_CONCURRENCY: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_CONNECTION_DEAD: u32 = 1209u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_CONNECTION_POOLING: u32 = 201u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_CONNECTION_TIMEOUT: u32 = 113u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_CP_MATCH: u32 = 202u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_CURRENT_CATALOG: u32 = 109u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_CURSOR_SCROLLABLE: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_CURSOR_SENSITIVITY: i32 = -2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_CURSOR_TYPE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_DBC_INFO_TOKEN: u32 = 118u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_DISCONNECT_BEHAVIOR: u32 = 114u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_ENABLE_AUTO_IPD: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_ENLIST_IN_DTC: u32 = 1207u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_ENLIST_IN_XA: u32 = 1208u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_FETCH_BOOKMARK_PTR: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_IMP_PARAM_DESC: u32 = 10013u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_IMP_ROW_DESC: u32 = 10012u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_KEYSET_SIZE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_LOGIN_TIMEOUT: u32 = 103u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_MAX_LENGTH: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_MAX_ROWS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_METADATA_ID: u32 = 10014u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_NOSCAN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_ODBC_CURSORS: u32 = 110u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_ODBC_VERSION: u32 = 200u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_OUTPUT_NTS: u32 = 10001u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_PACKET_SIZE: u32 = 112u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_PARAMSET_SIZE: u32 = 22u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_PARAMS_PROCESSED_PTR: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_PARAM_BIND_OFFSET_PTR: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_PARAM_BIND_TYPE: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_PARAM_OPERATION_PTR: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_PARAM_STATUS_PTR: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_QUERY_TIMEOUT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_QUIET_MODE: u32 = 111u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_READONLY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_READWRITE_UNKNOWN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_RESET_CONNECTION: u32 = 116u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_RETRIEVE_DATA: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_ROWS_FETCHED_PTR: u32 = 26u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_ROW_ARRAY_SIZE: u32 = 27u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_ROW_BIND_OFFSET_PTR: u32 = 23u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_ROW_BIND_TYPE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_ROW_NUMBER: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_ROW_OPERATION_PTR: u32 = 24u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_ROW_STATUS_PTR: u32 = 25u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_SIMULATE_CURSOR: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_TRACE: u32 = 104u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_TRACEFILE: u32 = 105u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_TRANSLATE_LIB: u32 = 106u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_TRANSLATE_OPTION: u32 = 107u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_TXN_ISOLATION: u32 = 108u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_USE_BOOKMARKS: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ATTR_WRITE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AT_ADD_COLUMN: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AT_ADD_COLUMN_COLLATION: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AT_ADD_COLUMN_DEFAULT: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AT_ADD_COLUMN_SINGLE: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AT_ADD_CONSTRAINT: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AT_ADD_TABLE_CONSTRAINT: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AT_CONSTRAINT_DEFERRABLE: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AT_CONSTRAINT_INITIALLY_DEFERRED: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AT_CONSTRAINT_INITIALLY_IMMEDIATE: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AT_CONSTRAINT_NAME_DEFINITION: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AT_CONSTRAINT_NON_DEFERRABLE: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AT_DROP_COLUMN: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AT_DROP_COLUMN_CASCADE: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AT_DROP_COLUMN_DEFAULT: i32 = 512i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AT_DROP_COLUMN_RESTRICT: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AT_DROP_TABLE_CONSTRAINT_CASCADE: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AT_DROP_TABLE_CONSTRAINT_RESTRICT: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AT_SET_COLUMN_DEFAULT: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AUTOCOMMIT: u32 = 102u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AUTOCOMMIT_DEFAULT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AUTOCOMMIT_OFF: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_AUTOCOMMIT_ON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_BATCH_ROW_COUNT: u32 = 120u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_BATCH_SUPPORT: u32 = 121u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_BCP_DEFAULT: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_BCP_OFF: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_BCP_ON: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_BEST_ROWID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_BIGINT: i32 = -5i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_BINARY: i32 = -2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_BIND_BY_COLUMN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_BIND_TYPE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_BIND_TYPE_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_BIT: i32 = -7i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_BOOKMARK_PERSISTENCE: u32 = 82u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_BP_CLOSE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_BP_DELETE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_BP_DROP: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_BP_OTHER_HSTMT: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_BP_SCROLL: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_BP_TRANSACTION: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_BP_UPDATE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_BRC_EXPLICIT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_BRC_PROCEDURES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_BRC_ROLLED_UP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_BS_ROW_COUNT_EXPLICIT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_BS_ROW_COUNT_PROC: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_BS_SELECT_EXPLICIT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_BS_SELECT_PROC: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA1_ABSOLUTE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA1_BOOKMARK: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA1_BULK_ADD: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA1_BULK_DELETE_BY_BOOKMARK: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA1_BULK_FETCH_BY_BOOKMARK: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA1_BULK_UPDATE_BY_BOOKMARK: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA1_LOCK_EXCLUSIVE: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA1_LOCK_NO_CHANGE: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA1_LOCK_UNLOCK: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA1_NEXT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA1_POSITIONED_DELETE: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA1_POSITIONED_UPDATE: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA1_POS_DELETE: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA1_POS_POSITION: i32 = 512i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA1_POS_REFRESH: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA1_POS_UPDATE: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA1_RELATIVE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA1_SELECT_FOR_UPDATE: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA2_CRC_APPROXIMATE: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA2_CRC_EXACT: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA2_LOCK_CONCURRENCY: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA2_MAX_ROWS_CATALOG: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA2_MAX_ROWS_DELETE: i32 = 512i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA2_MAX_ROWS_INSERT: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA2_MAX_ROWS_SELECT: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA2_MAX_ROWS_UPDATE: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA2_OPT_ROWVER_CONCURRENCY: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA2_OPT_VALUES_CONCURRENCY: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA2_READ_ONLY_CONCURRENCY: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA2_SENSITIVITY_ADDITIONS: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA2_SENSITIVITY_DELETIONS: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA2_SENSITIVITY_UPDATES: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA2_SIMULATE_NON_UNIQUE: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA2_SIMULATE_TRY_UNIQUE: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA2_SIMULATE_UNIQUE: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CACHE_DATA_NO: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CACHE_DATA_YES: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CASCADE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CATALOG_LOCATION: u32 = 114u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CATALOG_NAME: u32 = 10003u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CATALOG_NAME_SEPARATOR: u32 = 41u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CATALOG_TERM: u32 = 42u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CATALOG_USAGE: u32 = 92u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA_CONSTRAINT_DEFERRABLE: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA_CONSTRAINT_INITIALLY_DEFERRED: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA_CONSTRAINT_INITIALLY_IMMEDIATE: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA_CONSTRAINT_NON_DEFERRABLE: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA_CREATE_ASSERTION: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA_SS_BASE: u32 = 1200u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA_SS_COLUMN_COLLATION: u32 = 1214u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA_SS_COLUMN_HIDDEN: u32 = 1211u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA_SS_COLUMN_ID: u32 = 1208u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA_SS_COLUMN_KEY: u32 = 1212u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA_SS_COLUMN_OP: u32 = 1209u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA_SS_COLUMN_ORDER: u32 = 1203u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA_SS_COLUMN_SIZE: u32 = 1210u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA_SS_COLUMN_SSTYPE: u32 = 1200u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA_SS_COLUMN_UTYPE: u32 = 1201u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA_SS_COLUMN_VARYLEN: u32 = 1204u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA_SS_COMPUTE_BYLIST: u32 = 1207u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA_SS_COMPUTE_ID: u32 = 1206u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA_SS_MAX_USED: u32 = 1218u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA_SS_NUM_COMPUTES: u32 = 1205u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA_SS_NUM_ORDERS: u32 = 1202u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA_SS_VARIANT_SERVER_TYPE: u32 = 1217u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA_SS_VARIANT_SQL_TYPE: u32 = 1216u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CA_SS_VARIANT_TYPE: u32 = 1215u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CB_CLOSE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CB_DELETE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CB_NON_NULL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CB_NULL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CB_PRESERVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CCOL_CREATE_COLLATION: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CCS_COLLATE_CLAUSE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CCS_CREATE_CHARACTER_SET: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CCS_LIMITED_COLLATION: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CC_CLOSE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CC_DELETE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CC_PRESERVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CDO_COLLATION: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CDO_CONSTRAINT: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CDO_CONSTRAINT_DEFERRABLE: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CDO_CONSTRAINT_INITIALLY_DEFERRED: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CDO_CONSTRAINT_INITIALLY_IMMEDIATE: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CDO_CONSTRAINT_NAME_DEFINITION: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CDO_CONSTRAINT_NON_DEFERRABLE: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CDO_CREATE_DOMAIN: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CDO_DEFAULT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CD_FALSE: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CD_TRUE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CHAR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CLOSE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CL_END: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CL_START: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CN_ANY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CN_DEFAULT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CN_DIFFERENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CN_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CN_OFF: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CN_ON: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CODE_DATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CODE_DAY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CODE_DAY_TO_HOUR: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CODE_DAY_TO_MINUTE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CODE_DAY_TO_SECOND: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CODE_HOUR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CODE_HOUR_TO_MINUTE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CODE_HOUR_TO_SECOND: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CODE_MINUTE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CODE_MINUTE_TO_SECOND: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CODE_MONTH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CODE_SECOND: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CODE_TIME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CODE_TIMESTAMP: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CODE_YEAR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CODE_YEAR_TO_MONTH: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COLATT_OPT_MAX: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COLATT_OPT_MIN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COLLATION_SEQ: u32 = 10004u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COLUMN_ALIAS: u32 = 87u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COLUMN_AUTO_INCREMENT: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COLUMN_CASE_SENSITIVE: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COLUMN_COUNT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COLUMN_DISPLAY_SIZE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COLUMN_DRIVER_START: u32 = 1000u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COLUMN_IGNORE: i32 = -6i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COLUMN_LABEL: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COLUMN_LENGTH: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COLUMN_MONEY: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COLUMN_NAME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COLUMN_NULLABLE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COLUMN_NUMBER_UNKNOWN: i32 = -2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COLUMN_OWNER_NAME: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COLUMN_PRECISION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COLUMN_QUALIFIER_NAME: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COLUMN_SCALE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COLUMN_SEARCHABLE: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COLUMN_TABLE_NAME: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COLUMN_TYPE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COLUMN_TYPE_NAME: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COLUMN_UNSIGNED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COLUMN_UPDATABLE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COMMIT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONCAT_NULL_BEHAVIOR: u32 = 22u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONCURRENCY: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONCUR_DEFAULT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONCUR_LOCK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONCUR_READ_ONLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONCUR_ROWVER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONCUR_TIMESTAMP: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONCUR_VALUES: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONNECT_OPT_DRVR_START: u32 = 1000u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONN_OPT_MAX: u32 = 112u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONN_OPT_MIN: u32 = 101u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONN_POOL_RATING_BEST: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONN_POOL_RATING_GOOD_ENOUGH: u32 = 99u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONN_POOL_RATING_USELESS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONVERT_BIGINT: u32 = 53u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONVERT_BINARY: u32 = 54u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONVERT_BIT: u32 = 55u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONVERT_CHAR: u32 = 56u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONVERT_DATE: u32 = 57u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONVERT_DECIMAL: u32 = 58u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONVERT_DOUBLE: u32 = 59u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONVERT_FLOAT: u32 = 60u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONVERT_FUNCTIONS: u32 = 48u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONVERT_GUID: u32 = 173u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONVERT_INTEGER: u32 = 61u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONVERT_INTERVAL_DAY_TIME: u32 = 123u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONVERT_INTERVAL_YEAR_MONTH: u32 = 124u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONVERT_LONGVARBINARY: u32 = 71u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONVERT_LONGVARCHAR: u32 = 62u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONVERT_NUMERIC: u32 = 63u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONVERT_REAL: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONVERT_SMALLINT: u32 = 65u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONVERT_TIME: u32 = 66u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONVERT_TIMESTAMP: u32 = 67u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONVERT_TINYINT: u32 = 68u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONVERT_VARBINARY: u32 = 69u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONVERT_VARCHAR: u32 = 70u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONVERT_WCHAR: u32 = 122u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONVERT_WLONGVARCHAR: u32 = 125u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CONVERT_WVARCHAR: u32 = 126u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COPT_SS_ANSI_NPW: u32 = 1218u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COPT_SS_ANSI_OEM: u32 = 1206u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COPT_SS_ATTACHDBFILENAME: u32 = 1221u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COPT_SS_BASE: u32 = 1200u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COPT_SS_BASE_EX: u32 = 1240u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COPT_SS_BCP: u32 = 1219u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COPT_SS_BROWSE_CACHE_DATA: u32 = 1245u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COPT_SS_BROWSE_CONNECT: u32 = 1241u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COPT_SS_BROWSE_SERVER: u32 = 1242u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COPT_SS_CONCAT_NULL: u32 = 1222u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COPT_SS_CONNECTION_DEAD: u32 = 1244u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COPT_SS_ENCRYPT: u32 = 1223u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COPT_SS_EX_MAX_USED: u32 = 1246u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COPT_SS_FALLBACK_CONNECT: u32 = 1210u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COPT_SS_INTEGRATED_SECURITY: u32 = 1203u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COPT_SS_MAX_USED: u32 = 1223u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COPT_SS_PERF_DATA: u32 = 1211u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COPT_SS_PERF_DATA_LOG: u32 = 1212u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COPT_SS_PERF_DATA_LOG_NOW: u32 = 1216u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COPT_SS_PERF_QUERY: u32 = 1215u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COPT_SS_PERF_QUERY_INTERVAL: u32 = 1213u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COPT_SS_PERF_QUERY_LOG: u32 = 1214u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COPT_SS_PRESERVE_CURSORS: u32 = 1204u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COPT_SS_QUOTED_IDENT: u32 = 1217u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COPT_SS_REMOTE_PWD: u32 = 1201u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COPT_SS_RESET_CONNECTION: u32 = 1246u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COPT_SS_TRANSLATE: u32 = 1220u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COPT_SS_USER_DATA: u32 = 1205u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COPT_SS_USE_PROC_FOR_PREP: u32 = 1202u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_COPT_SS_WARN_ON_CP_ERROR: u32 = 1243u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CORRELATION_NAME: u32 = 74u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CO_AF: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CO_DEFAULT: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CO_FFO: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CO_FIREHOSE_AF: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CO_OFF: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CP_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CP_DRIVER_AWARE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CP_MATCH_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CP_OFF: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CP_ONE_PER_DRIVER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CP_ONE_PER_HENV: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CP_RELAXED_MATCH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CP_STRICT_MATCH: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CREATE_ASSERTION: u32 = 127u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CREATE_CHARACTER_SET: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CREATE_COLLATION: u32 = 129u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CREATE_DOMAIN: u32 = 130u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CREATE_SCHEMA: u32 = 131u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CREATE_TABLE: u32 = 132u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CREATE_TRANSLATION: u32 = 133u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CREATE_VIEW: u32 = 134u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CR_CLOSE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CR_DELETE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CR_PRESERVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CS_AUTHORIZATION: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CS_CREATE_SCHEMA: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CS_DEFAULT_CHARACTER_SET: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CTR_CREATE_TRANSLATION: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CT_COLUMN_COLLATION: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CT_COLUMN_CONSTRAINT: i32 = 512i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CT_COLUMN_DEFAULT: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CT_COMMIT_DELETE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CT_COMMIT_PRESERVE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CT_CONSTRAINT_DEFERRABLE: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CT_CONSTRAINT_INITIALLY_DEFERRED: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CT_CONSTRAINT_INITIALLY_IMMEDIATE: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CT_CONSTRAINT_NAME_DEFINITION: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CT_CONSTRAINT_NON_DEFERRABLE: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CT_CREATE_TABLE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CT_GLOBAL_TEMPORARY: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CT_LOCAL_TEMPORARY: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CT_TABLE_CONSTRAINT: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CURRENT_QUALIFIER: u32 = 109u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CURSOR_COMMIT_BEHAVIOR: u32 = 23u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CURSOR_DYNAMIC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CURSOR_FAST_FORWARD_ONLY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CURSOR_FORWARD_ONLY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CURSOR_KEYSET_DRIVEN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CURSOR_ROLLBACK_BEHAVIOR: u32 = 24u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CURSOR_SENSITIVITY: u32 = 10001u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CURSOR_STATIC: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CURSOR_TYPE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CURSOR_TYPE_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CUR_DEFAULT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CUR_USE_DRIVER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CUR_USE_IF_NEEDED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CUR_USE_ODBC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CU_DML_STATEMENTS: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CU_INDEX_DEFINITION: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CU_PRIVILEGE_DEFINITION: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CU_PROCEDURE_INVOCATION: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CU_TABLE_DEFINITION: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CVT_BIGINT: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CVT_BINARY: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CVT_BIT: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CVT_CHAR: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CVT_DATE: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CVT_DECIMAL: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CVT_DOUBLE: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CVT_FLOAT: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CVT_GUID: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CVT_INTEGER: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CVT_INTERVAL_DAY_TIME: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CVT_INTERVAL_YEAR_MONTH: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CVT_LONGVARBINARY: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CVT_LONGVARCHAR: i32 = 512i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CVT_NUMERIC: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CVT_REAL: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CVT_SMALLINT: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CVT_TIME: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CVT_TIMESTAMP: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CVT_TINYINT: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CVT_VARBINARY: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CVT_VARCHAR: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CVT_WCHAR: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CVT_WLONGVARCHAR: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CVT_WVARCHAR: i32 = 8388608i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CV_CASCADED: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CV_CHECK_OPTION: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CV_CREATE_VIEW: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_CV_LOCAL: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_C_BINARY: i32 = -2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_C_BIT: i32 = -7i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_C_CHAR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_C_DATE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_C_DEFAULT: u32 = 99u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_C_DOUBLE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_C_FLOAT: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_C_GUID: i32 = -11i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_C_INTERVAL_DAY: i32 = -83i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_C_INTERVAL_DAY_TO_HOUR: i32 = -87i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_C_INTERVAL_DAY_TO_MINUTE: i32 = -88i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_C_INTERVAL_DAY_TO_SECOND: i32 = -89i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_C_INTERVAL_HOUR: i32 = -84i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_C_INTERVAL_HOUR_TO_MINUTE: i32 = -90i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_C_INTERVAL_HOUR_TO_SECOND: i32 = -91i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_C_INTERVAL_MINUTE: i32 = -85i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_C_INTERVAL_MINUTE_TO_SECOND: i32 = -92i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_C_INTERVAL_MONTH: i32 = -81i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_C_INTERVAL_SECOND: i32 = -86i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_C_INTERVAL_YEAR: i32 = -80i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_C_INTERVAL_YEAR_TO_MONTH: i32 = -82i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_C_LONG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_C_NUMERIC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_C_SHORT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_C_TCHAR: i32 = -8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_C_TIME: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_C_TIMESTAMP: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_C_TINYINT: i32 = -6i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_C_TYPE_DATE: u32 = 91u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_C_TYPE_TIME: u32 = 92u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_C_TYPE_TIMESTAMP: u32 = 93u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_C_VARBOOKMARK: i32 = -2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_C_WCHAR: i32 = -8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DATABASE_NAME: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DATA_AT_EXEC: i32 = -2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DATA_SOURCE_NAME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DATA_SOURCE_READ_ONLY: u32 = 25u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DATE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DATETIME: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DATETIME_LITERALS: u32 = 119u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DATE_LEN: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DAY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DAY_TO_HOUR: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DAY_TO_MINUTE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DAY_TO_SECOND: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DA_DROP_ASSERTION: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DBMS_NAME: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DBMS_VER: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DB_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DB_DISCONNECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DB_RETURN_TO_POOL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DCS_DROP_CHARACTER_SET: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DC_DROP_COLLATION: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DDL_INDEX: u32 = 170u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DD_CASCADE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DD_DROP_DOMAIN: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DD_RESTRICT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DECIMAL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DEFAULT: u32 = 99u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DEFAULT_PARAM: i32 = -5i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DEFAULT_TXN_ISOLATION: u32 = 26u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DELETE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DELETE_BY_BOOKMARK: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DESCRIBE_PARAMETER: u32 = 10002u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DESC_ALLOC_AUTO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DESC_ALLOC_TYPE: u32 = 1099u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DESC_ALLOC_USER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DESC_ARRAY_SIZE: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DESC_ARRAY_STATUS_PTR: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DESC_BASE_COLUMN_NAME: u32 = 22u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DESC_BASE_TABLE_NAME: u32 = 23u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DESC_BIND_OFFSET_PTR: u32 = 24u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DESC_BIND_TYPE: u32 = 25u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DESC_COUNT: u32 = 1001u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DESC_DATA_PTR: u32 = 1010u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DESC_DATETIME_INTERVAL_CODE: u32 = 1007u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DESC_DATETIME_INTERVAL_PRECISION: u32 = 26u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DESC_INDICATOR_PTR: u32 = 1009u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DESC_LENGTH: u32 = 1003u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DESC_LITERAL_PREFIX: u32 = 27u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DESC_LITERAL_SUFFIX: u32 = 28u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DESC_LOCAL_TYPE_NAME: u32 = 29u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DESC_MAXIMUM_SCALE: u32 = 30u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DESC_MINIMUM_SCALE: u32 = 31u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DESC_NAME: u32 = 1011u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DESC_NULLABLE: u32 = 1008u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DESC_NUM_PREC_RADIX: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DESC_OCTET_LENGTH: u32 = 1013u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DESC_OCTET_LENGTH_PTR: u32 = 1004u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DESC_PARAMETER_TYPE: u32 = 33u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DESC_PRECISION: u32 = 1005u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DESC_ROWS_PROCESSED_PTR: u32 = 34u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DESC_ROWVER: u32 = 35u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DESC_SCALE: u32 = 1006u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DESC_TYPE: u32 = 1002u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DESC_UNNAMED: u32 = 1012u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_ALTER_DOMAIN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_ALTER_TABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_CALL: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_CLASS_ORIGIN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_COLUMN_NUMBER: i32 = -1247i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_CONNECTION_NAME: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_CREATE_ASSERTION: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_CREATE_CHARACTER_SET: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_CREATE_COLLATION: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_CREATE_DOMAIN: u32 = 23u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_CREATE_INDEX: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_CREATE_SCHEMA: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_CREATE_TABLE: u32 = 77u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_CREATE_TRANSLATION: u32 = 79u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_CREATE_VIEW: u32 = 84u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_CURSOR_ROW_COUNT: i32 = -1249i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_DELETE_WHERE: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_DFC_SS_BASE: i32 = -200i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_DROP_ASSERTION: u32 = 24u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_DROP_CHARACTER_SET: u32 = 25u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_DROP_COLLATION: u32 = 26u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_DROP_DOMAIN: u32 = 27u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_DROP_INDEX: i32 = -2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_DROP_SCHEMA: u32 = 31u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_DROP_TABLE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_DROP_TRANSLATION: u32 = 33u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_DROP_VIEW: u32 = 36u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_DYNAMIC_DELETE_CURSOR: u32 = 38u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_DYNAMIC_FUNCTION: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_DYNAMIC_FUNCTION_CODE: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_DYNAMIC_UPDATE_CURSOR: u32 = 81u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_GRANT: u32 = 48u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_INSERT: u32 = 50u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_MESSAGE_TEXT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_NATIVE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_NUMBER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_RETURNCODE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_REVOKE: u32 = 59u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_ROW_COUNT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_ROW_NUMBER: i32 = -1248i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_SELECT_CURSOR: u32 = 85u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_SERVER_NAME: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_SQLSTATE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_SS_BASE: i32 = -1150i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_SS_MSGSTATE: i32 = -1150i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_SUBCLASS_ORIGIN: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_UNKNOWN_STATEMENT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DIAG_UPDATE_WHERE: u32 = 82u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DI_CREATE_INDEX: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DI_DROP_INDEX: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DL_SQL92_DATE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DL_SQL92_INTERVAL_DAY: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DL_SQL92_INTERVAL_DAY_TO_HOUR: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DL_SQL92_INTERVAL_DAY_TO_MINUTE: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DL_SQL92_INTERVAL_DAY_TO_SECOND: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DL_SQL92_INTERVAL_HOUR: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DL_SQL92_INTERVAL_HOUR_TO_MINUTE: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DL_SQL92_INTERVAL_HOUR_TO_SECOND: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DL_SQL92_INTERVAL_MINUTE: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DL_SQL92_INTERVAL_MINUTE_TO_SECOND: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DL_SQL92_INTERVAL_MONTH: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DL_SQL92_INTERVAL_SECOND: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DL_SQL92_INTERVAL_YEAR: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DL_SQL92_INTERVAL_YEAR_TO_MONTH: i32 = 512i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DL_SQL92_TIME: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DL_SQL92_TIMESTAMP: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DM_VER: u32 = 171u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DOUBLE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DP_OFF: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DP_ON: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DRIVER_AWARE_POOLING_CAPABLE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DRIVER_AWARE_POOLING_NOT_CAPABLE: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DRIVER_AWARE_POOLING_SUPPORTED: u32 = 10024u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DRIVER_COMPLETE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DRIVER_COMPLETE_REQUIRED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DRIVER_CONN_ATTR_BASE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DRIVER_C_TYPE_BASE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DRIVER_DESC_FIELD_BASE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DRIVER_DIAG_FIELD_BASE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DRIVER_HDBC: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DRIVER_HDESC: u32 = 135u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DRIVER_HENV: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DRIVER_HLIB: u32 = 76u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DRIVER_HSTMT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DRIVER_INFO_TYPE_BASE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DRIVER_NAME: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DRIVER_NOPROMPT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DRIVER_ODBC_VER: u32 = 77u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DRIVER_PROMPT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DRIVER_SQL_TYPE_BASE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DRIVER_STMT_ATTR_BASE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DRIVER_VER: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DROP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DROP_ASSERTION: u32 = 136u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DROP_CHARACTER_SET: u32 = 137u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DROP_COLLATION: u32 = 138u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DROP_DOMAIN: u32 = 139u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DROP_SCHEMA: u32 = 140u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DROP_TABLE: u32 = 141u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DROP_TRANSLATION: u32 = 142u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DROP_VIEW: u32 = 143u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DS_CASCADE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DS_DROP_SCHEMA: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DS_RESTRICT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DTC_DONE: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DTC_ENLIST_EXPENSIVE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DTC_TRANSITION_COST: u32 = 1750u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DTC_UNENLIST_EXPENSIVE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DTR_DROP_TRANSLATION: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DT_CASCADE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DT_DROP_TABLE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DT_RESTRICT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DV_CASCADE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DV_DROP_VIEW: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DV_RESTRICT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DYNAMIC_CURSOR_ATTRIBUTES1: u32 = 144u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_DYNAMIC_CURSOR_ATTRIBUTES2: u32 = 145u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ENSURE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ENTIRE_ROWSET: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_EN_OFF: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_EN_ON: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ERROR: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_EXPRESSIONS_IN_ORDERBY: u32 = 27u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_EXT_API_LAST: u32 = 72u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_EXT_API_START: u32 = 40u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FALSE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FAST_CONNECT: u32 = 1200u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FB_DEFAULT: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FB_OFF: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FB_ON: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FC_DEFAULT: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FC_OFF: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FC_ON: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FD_FETCH_ABSOLUTE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FD_FETCH_BOOKMARK: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FD_FETCH_FIRST: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FD_FETCH_LAST: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FD_FETCH_NEXT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FD_FETCH_PREV: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FD_FETCH_PRIOR: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FD_FETCH_RELATIVE: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FD_FETCH_RESUME: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FETCH_ABSOLUTE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FETCH_BOOKMARK: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FETCH_BY_BOOKMARK: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FETCH_DIRECTION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FETCH_FIRST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FETCH_FIRST_SYSTEM: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FETCH_FIRST_USER: u32 = 31u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FETCH_LAST: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FETCH_NEXT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FETCH_PREV: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FETCH_PRIOR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FETCH_RELATIVE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FETCH_RESUME: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FILE_CATALOG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FILE_NOT_SUPPORTED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FILE_QUALIFIER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FILE_TABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FILE_USAGE: u32 = 84u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FLOAT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_CVT_CAST: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_CVT_CONVERT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_NUM_ABS: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_NUM_ACOS: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_NUM_ASIN: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_NUM_ATAN: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_NUM_ATAN2: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_NUM_CEILING: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_NUM_COS: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_NUM_COT: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_NUM_DEGREES: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_NUM_EXP: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_NUM_FLOOR: i32 = 512i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_NUM_LOG: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_NUM_LOG10: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_NUM_MOD: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_NUM_PI: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_NUM_POWER: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_NUM_RADIANS: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_NUM_RAND: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_NUM_ROUND: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_NUM_SIGN: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_NUM_SIN: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_NUM_SQRT: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_NUM_TAN: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_NUM_TRUNCATE: i32 = 8388608i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_STR_ASCII: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_STR_BIT_LENGTH: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_STR_CHAR: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_STR_CHARACTER_LENGTH: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_STR_CHAR_LENGTH: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_STR_CONCAT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_STR_DIFFERENCE: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_STR_INSERT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_STR_LCASE: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_STR_LEFT: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_STR_LENGTH: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_STR_LOCATE: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_STR_LOCATE_2: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_STR_LTRIM: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_STR_OCTET_LENGTH: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_STR_POSITION: i32 = 8388608i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_STR_REPEAT: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_STR_REPLACE: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_STR_RIGHT: i32 = 512i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_STR_RTRIM: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_STR_SOUNDEX: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_STR_SPACE: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_STR_SUBSTRING: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_STR_UCASE: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_SYS_DBNAME: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_SYS_IFNULL: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_SYS_USERNAME: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_TD_CURDATE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_TD_CURRENT_DATE: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_TD_CURRENT_TIME: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_TD_CURRENT_TIMESTAMP: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_TD_CURTIME: i32 = 512i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_TD_DAYNAME: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_TD_DAYOFMONTH: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_TD_DAYOFWEEK: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_TD_DAYOFYEAR: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_TD_EXTRACT: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_TD_HOUR: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_TD_MINUTE: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_TD_MONTH: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_TD_MONTHNAME: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_TD_NOW: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_TD_QUARTER: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_TD_SECOND: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_TD_TIMESTAMPADD: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_TD_TIMESTAMPDIFF: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_TD_WEEK: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_TD_YEAR: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_TSI_DAY: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_TSI_FRAC_SECOND: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_TSI_HOUR: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_TSI_MINUTE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_TSI_MONTH: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_TSI_QUARTER: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_TSI_SECOND: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_TSI_WEEK: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FN_TSI_YEAR: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FORWARD_ONLY_CURSOR_ATTRIBUTES1: u32 = 146u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_FORWARD_ONLY_CURSOR_ATTRIBUTES2: u32 = 147u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_GB_COLLATE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_GB_GROUP_BY_CONTAINS_SELECT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_GB_GROUP_BY_EQUALS_SELECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_GB_NOT_SUPPORTED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_GB_NO_RELATION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_GD_ANY_COLUMN: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_GD_ANY_ORDER: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_GD_BLOCK: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_GD_BOUND: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_GD_OUTPUT_PARAMS: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_GETDATA_EXTENSIONS: u32 = 81u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_GET_BOOKMARK: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_GROUP_BY: u32 = 88u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_GUID: i32 = -11i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_HANDLE_DBC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_HANDLE_DBC_INFO_TOKEN: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_HANDLE_DESC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_HANDLE_ENV: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_HANDLE_SENV: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_HANDLE_STMT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_HC_DEFAULT: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_HC_OFF: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_HC_ON: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_HOUR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_HOUR_TO_MINUTE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_HOUR_TO_SECOND: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_IC_LOWER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_IC_MIXED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_IC_SENSITIVE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_IC_UPPER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_IDENTIFIER_CASE: u32 = 28u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_IDENTIFIER_QUOTE_CHAR: u32 = 29u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_IGNORE: i32 = -6i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_IK_ASC: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_IK_DESC: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_IK_NONE: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_INDEX_ALL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_INDEX_CLUSTERED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_INDEX_HASHED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_INDEX_KEYWORDS: u32 = 148u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_INDEX_OTHER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_INDEX_UNIQUE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_INFO_DRIVER_START: u32 = 1000u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_INFO_FIRST: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_INFO_LAST: u32 = 114u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_INFO_SCHEMA_VIEWS: u32 = 149u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_INFO_SS_FIRST: u32 = 1199u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_INFO_SS_MAX_USED: u32 = 1200u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_INFO_SS_NETLIB_NAME: u32 = 1199u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_INFO_SS_NETLIB_NAMEA: u32 = 1200u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_INFO_SS_NETLIB_NAMEW: u32 = 1199u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_INITIALLY_DEFERRED: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_INITIALLY_IMMEDIATE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_INSENSITIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_INSERT_STATEMENT: u32 = 172u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_INTEGER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_INTEGRATED_SECURITY: u32 = 1203u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_INTEGRITY: u32 = 73u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_INTERVAL: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_INTERVAL_DAY: i32 = -83i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_INTERVAL_DAY_TO_HOUR: i32 = -87i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_INTERVAL_DAY_TO_MINUTE: i32 = -88i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_INTERVAL_DAY_TO_SECOND: i32 = -89i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_INTERVAL_HOUR: i32 = -84i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_INTERVAL_HOUR_TO_MINUTE: i32 = -90i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_INTERVAL_HOUR_TO_SECOND: i32 = -91i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_INTERVAL_MINUTE: i32 = -85i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_INTERVAL_MINUTE_TO_SECOND: i32 = -92i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_INTERVAL_MONTH: i32 = -81i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_INTERVAL_SECOND: i32 = -86i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub struct SQL_INTERVAL_STRUCT {
    pub interval_type: SQLINTERVAL,
    pub interval_sign: i16,
    pub intval: SQL_INTERVAL_STRUCT_0,
}
impl ::core::marker::Copy for SQL_INTERVAL_STRUCT {}
impl ::core::clone::Clone for SQL_INTERVAL_STRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub union SQL_INTERVAL_STRUCT_0 {
    pub year_month: tagSQL_YEAR_MONTH,
    pub day_second: tagSQL_DAY_SECOND,
}
impl ::core::marker::Copy for SQL_INTERVAL_STRUCT_0 {}
impl ::core::clone::Clone for SQL_INTERVAL_STRUCT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_INTERVAL_YEAR: i32 = -80i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_INTERVAL_YEAR_TO_MONTH: i32 = -82i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_INVALID_HANDLE: i32 = -2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ISV_ASSERTIONS: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ISV_CHARACTER_SETS: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ISV_CHECK_CONSTRAINTS: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ISV_COLLATIONS: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ISV_COLUMNS: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ISV_COLUMN_DOMAIN_USAGE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ISV_COLUMN_PRIVILEGES: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ISV_CONSTRAINT_COLUMN_USAGE: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ISV_CONSTRAINT_TABLE_USAGE: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ISV_DOMAINS: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ISV_DOMAIN_CONSTRAINTS: i32 = 512i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ISV_KEY_COLUMN_USAGE: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ISV_REFERENTIAL_CONSTRAINTS: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ISV_SCHEMATA: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ISV_SQL_LANGUAGES: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ISV_TABLES: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ISV_TABLE_CONSTRAINTS: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ISV_TABLE_PRIVILEGES: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ISV_TRANSLATIONS: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ISV_USAGE_PRIVILEGES: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ISV_VIEWS: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ISV_VIEW_COLUMN_USAGE: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ISV_VIEW_TABLE_USAGE: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_IS_DEFAULT: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_IS_INSERT_LITERALS: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_IS_INSERT_SEARCHED: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_IS_INTEGER: i32 = -6i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_IS_OFF: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_IS_ON: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_IS_POINTER: i32 = -4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_IS_SELECT_INTO: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_IS_SMALLINT: i32 = -8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_IS_UINTEGER: i32 = -5i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_IS_USMALLINT: i32 = -7i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_KEYSET_CURSOR_ATTRIBUTES1: u32 = 150u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_KEYSET_CURSOR_ATTRIBUTES2: u32 = 151u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_KEYSET_SIZE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_KEYSET_SIZE_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_KEYWORDS: u32 = 89u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_LCK_EXCLUSIVE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_LCK_NO_CHANGE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_LCK_UNLOCK: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_LEN_BINARY_ATTR_OFFSET: i32 = -100i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_LEN_DATA_AT_EXEC_OFFSET: i32 = -100i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_LIKE_ESCAPE_CLAUSE: u32 = 113u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_LIKE_ONLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_LOCK_EXCLUSIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_LOCK_NO_CHANGE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_LOCK_TYPES: u32 = 78u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_LOCK_UNLOCK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_LOGIN_TIMEOUT: u32 = 103u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_LOGIN_TIMEOUT_DEFAULT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_LONGVARBINARY: i32 = -4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_LONGVARCHAR: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAXIMUM_CATALOG_NAME_LENGTH: u32 = 34u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAXIMUM_COLUMNS_IN_GROUP_BY: u32 = 97u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAXIMUM_COLUMNS_IN_INDEX: u32 = 98u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAXIMUM_COLUMNS_IN_ORDER_BY: u32 = 99u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAXIMUM_COLUMNS_IN_SELECT: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAXIMUM_COLUMN_NAME_LENGTH: u32 = 30u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAXIMUM_CONCURRENT_ACTIVITIES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAXIMUM_CURSOR_NAME_LENGTH: u32 = 31u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAXIMUM_DRIVER_CONNECTIONS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAXIMUM_IDENTIFIER_LENGTH: u32 = 10005u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAXIMUM_INDEX_SIZE: u32 = 102u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAXIMUM_ROW_SIZE: u32 = 104u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAXIMUM_SCHEMA_NAME_LENGTH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAXIMUM_STATEMENT_LENGTH: u32 = 105u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAXIMUM_TABLES_IN_SELECT: u32 = 106u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAXIMUM_USER_NAME_LENGTH: u32 = 107u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAX_ASYNC_CONCURRENT_STATEMENTS: u32 = 10022u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAX_BINARY_LITERAL_LEN: u32 = 112u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAX_CATALOG_NAME_LEN: u32 = 34u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAX_CHAR_LITERAL_LEN: u32 = 108u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAX_COLUMNS_IN_GROUP_BY: u32 = 97u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAX_COLUMNS_IN_INDEX: u32 = 98u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAX_COLUMNS_IN_ORDER_BY: u32 = 99u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAX_COLUMNS_IN_SELECT: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAX_COLUMNS_IN_TABLE: u32 = 101u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAX_COLUMN_NAME_LEN: u32 = 30u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAX_CONCURRENT_ACTIVITIES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAX_CURSOR_NAME_LEN: u32 = 31u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAX_DRIVER_CONNECTIONS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAX_DSN_LENGTH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAX_IDENTIFIER_LEN: u32 = 10005u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAX_INDEX_SIZE: u32 = 102u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAX_LENGTH: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAX_LENGTH_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAX_MESSAGE_LENGTH: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAX_NUMERIC_LEN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAX_OPTION_STRING_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAX_OWNER_NAME_LEN: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAX_PROCEDURE_NAME_LEN: u32 = 33u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAX_QUALIFIER_NAME_LEN: u32 = 34u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAX_ROWS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAX_ROWS_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAX_ROW_SIZE: u32 = 104u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAX_ROW_SIZE_INCLUDES_LONG: u32 = 103u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAX_SCHEMA_NAME_LEN: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAX_SQLSERVERNAME: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAX_STATEMENT_LEN: u32 = 105u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAX_TABLES_IN_SELECT: u32 = 106u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAX_TABLE_NAME_LEN: u32 = 35u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MAX_USER_NAME_LEN: u32 = 107u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MINUTE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MINUTE_TO_SECOND: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MODE_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MODE_READ_ONLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MODE_READ_WRITE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MONTH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MORE_INFO_NO: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MORE_INFO_YES: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MULTIPLE_ACTIVE_TXN: u32 = 37u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_MULT_RESULT_SETS: u32 = 36u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NAMED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NB_DEFAULT: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NB_OFF: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NB_ON: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NC_END: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NC_HIGH: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NC_LOW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NC_OFF: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NC_ON: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NC_START: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NEED_DATA: u32 = 99u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NEED_LONG_DATA_LEN: u32 = 111u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NNC_NON_NULL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NNC_NULL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NONSCROLLABLE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NON_NULLABLE_COLUMNS: u32 = 75u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NOSCAN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NOSCAN_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NOSCAN_OFF: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NOSCAN_ON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NOT_DEFERRABLE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NO_ACTION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NO_COLUMN_NUMBER: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NO_DATA: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NO_DATA_FOUND: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NO_NULLS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NO_ROW_NUMBER: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NO_TOTAL: i32 = -4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NTS: i32 = -3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NTSL: i32 = -3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NULLABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NULLABLE_UNKNOWN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NULL_COLLATION: u32 = 85u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NULL_DATA: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NULL_HANDLE: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NULL_HDBC: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NULL_HDESC: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NULL_HENV: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NULL_HSTMT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NUMERIC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NUMERIC_FUNCTIONS: u32 = 49u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub struct SQL_NUMERIC_STRUCT {
    pub precision: u8,
    pub scale: i8,
    pub sign: u8,
    pub val: [u8; 16],
}
impl ::core::marker::Copy for SQL_NUMERIC_STRUCT {}
impl ::core::clone::Clone for SQL_NUMERIC_STRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_NUM_FUNCTIONS: u32 = 23u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_OAC_LEVEL1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_OAC_LEVEL2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_OAC_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ODBC_API_CONFORMANCE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ODBC_CURSORS: u32 = 110u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ODBC_INTERFACE_CONFORMANCE: u32 = 152u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ODBC_KEYWORDS : & str = "ABSOLUTE,ACTION,ADA,ADD,ALL,ALLOCATE,ALTER,AND,ANY,ARE,AS,ASC,ASSERTION,AT,AUTHORIZATION,AVG,BEGIN,BETWEEN,BIT,BIT_LENGTH,BOTH,BY,CASCADE,CASCADED,CASE,CAST,CATALOG,CHAR,CHAR_LENGTH,CHARACTER,CHARACTER_LENGTH,CHECK,CLOSE,COALESCE,COLLATE,COLLATION,COLUMN,COMMIT,CONNECT,CONNECTION,CONSTRAINT,CONSTRAINTS,CONTINUE,CONVERT,CORRESPONDING,COUNT,CREATE,CROSS,CURRENT,CURRENT_DATE,CURRENT_TIME,CURRENT_TIMESTAMP,CURRENT_USER,CURSOR,DATE,DAY,DEALLOCATE,DEC,DECIMAL,DECLARE,DEFAULT,DEFERRABLE,DEFERRED,DELETE,DESC,DESCRIBE,DESCRIPTOR,DIAGNOSTICS,DISCONNECT,DISTINCT,DOMAIN,DOUBLE,DROP,ELSE,END,END-EXEC,ESCAPE,EXCEPT,EXCEPTION,EXEC,EXECUTE,EXISTS,EXTERNAL,EXTRACT,FALSE,FETCH,FIRST,FLOAT,FOR,FOREIGN,FORTRAN,FOUND,FROM,FULL,GET,GLOBAL,GO,GOTO,GRANT,GROUP,HAVING,HOUR,IDENTITY,IMMEDIATE,IN,INCLUDE,INDEX,INDICATOR,INITIALLY,INNER,INPUT,INSENSITIVE,INSERT,INT,INTEGER,INTERSECT,INTERVAL,INTO,IS,ISOLATION,JOIN,KEY,LANGUAGE,LAST,LEADING,LEFT,LEVEL,LIKE,LOCAL,LOWER,MATCH,MAX,MIN,MINUTE,MODULE,MONTH,NAMES,NATIONAL,NATURAL,NCHAR,NEXT,NO,NONE,NOT,NULL,NULLIF,NUMERIC,OCTET_LENGTH,OF,ON,ONLY,OPEN,OPTION,OR,ORDER,OUTER,OUTPUT,OVERLAPS,PAD,PARTIAL,PASCAL,PLI,POSITION,PRECISION,PREPARE,PRESERVE,PRIMARY,PRIOR,PRIVILEGES,PROCEDURE,PUBLIC,READ,REAL,REFERENCES,RELATIVE,RESTRICT,REVOKE,RIGHT,ROLLBACK,ROWSSCHEMA,SCROLL,SECOND,SECTION,SELECT,SESSION,SESSION_USER,SET,SIZE,SMALLINT,SOME,SPACE,SQL,SQLCA,SQLCODE,SQLERROR,SQLSTATE,SQLWARNING,SUBSTRING,SUM,SYSTEM_USER,TABLE,TEMPORARY,THEN,TIME,TIMESTAMP,TIMEZONE_HOUR,TIMEZONE_MINUTE,TO,TRAILING,TRANSACTION,TRANSLATE,TRANSLATION,TRIM,TRUE,UNION,UNIQUE,UNKNOWN,UPDATE,UPPER,USAGE,USER,USING,VALUE,VALUES,VARCHAR,VARYING,VIEW,WHEN,WHENEVER,WHERE,WITH,WORK,WRITE,YEAR,ZONE" ;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ODBC_SAG_CLI_CONFORMANCE: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ODBC_SQL_CONFORMANCE: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ODBC_SQL_OPT_IEF: u32 = 73u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ODBC_VER: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_OIC_CORE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_OIC_LEVEL1: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_OIC_LEVEL2: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_OJ_ALL_COMPARISON_OPS: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_OJ_CAPABILITIES: u32 = 115u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_OJ_FULL: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_OJ_INNER: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_OJ_LEFT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_OJ_NESTED: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_OJ_NOT_ORDERED: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_OJ_RIGHT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_OPT_TRACE: u32 = 104u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_OPT_TRACEFILE: u32 = 105u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_OPT_TRACE_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_OPT_TRACE_FILE_DEFAULT: &str = "\\SQL.LOG";
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_OPT_TRACE_OFF: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_OPT_TRACE_ON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ORDER_BY_COLUMNS_IN_SELECT: u32 = 90u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_OSCC_COMPLIANT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_OSCC_NOT_COMPLIANT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_OSC_CORE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_OSC_EXTENDED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_OSC_MINIMUM: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_OUTER_JOINS: u32 = 38u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_OUTER_JOIN_CAPABILITIES: u32 = 115u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_OU_DML_STATEMENTS: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_OU_INDEX_DEFINITION: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_OU_PRIVILEGE_DEFINITION: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_OU_PROCEDURE_INVOCATION: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_OU_TABLE_DEFINITION: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_OV_ODBC2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_OV_ODBC3: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_OV_ODBC3_80: u32 = 380u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_OWNER_TERM: u32 = 39u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_OWNER_USAGE: u32 = 91u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PACKET_SIZE: u32 = 112u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PARAM_ARRAY_ROW_COUNTS: u32 = 153u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PARAM_ARRAY_SELECTS: u32 = 154u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PARAM_BIND_BY_COLUMN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PARAM_BIND_TYPE_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PARAM_DATA_AVAILABLE: u32 = 101u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PARAM_DIAG_UNAVAILABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PARAM_ERROR: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PARAM_IGNORE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PARAM_INPUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PARAM_INPUT_OUTPUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PARAM_INPUT_OUTPUT_STREAM: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PARAM_OUTPUT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PARAM_OUTPUT_STREAM: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PARAM_PROCEED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PARAM_SUCCESS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PARAM_SUCCESS_WITH_INFO: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PARAM_TYPE_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PARAM_UNUSED: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PARC_BATCH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PARC_NO_BATCH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PAS_BATCH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PAS_NO_BATCH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PAS_NO_SELECT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PC_DEFAULT: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PC_NON_PSEUDO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PC_NOT_PSEUDO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PC_OFF: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PC_ON: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PC_PSEUDO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PC_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PERF_START: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PERF_STOP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_POSITION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_POSITIONED_STATEMENTS: u32 = 80u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_POS_ADD: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_POS_DELETE: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_POS_OPERATIONS: u32 = 79u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_POS_POSITION: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_POS_REFRESH: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_POS_UPDATE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PRED_BASIC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PRED_CHAR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PRED_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PRED_SEARCHABLE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PRESERVE_CURSORS: u32 = 1204u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PROCEDURES: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PROCEDURE_TERM: u32 = 40u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PS_POSITIONED_DELETE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PS_POSITIONED_UPDATE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PS_SELECT_FOR_UPDATE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PT_FUNCTION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PT_PROCEDURE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_PT_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_QI_DEFAULT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_QI_OFF: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_QI_ON: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_QL_END: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_QL_START: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_QUALIFIER_LOCATION: u32 = 114u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_QUALIFIER_NAME_SEPARATOR: u32 = 41u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_QUALIFIER_TERM: u32 = 42u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_QUALIFIER_USAGE: u32 = 92u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_QUERY_TIMEOUT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_QUERY_TIMEOUT_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_QUICK: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_QUIET_MODE: u32 = 111u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_QUOTED_IDENTIFIER_CASE: u32 = 93u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_QU_DML_STATEMENTS: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_QU_INDEX_DEFINITION: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_QU_PRIVILEGE_DEFINITION: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_QU_PROCEDURE_INVOCATION: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_QU_TABLE_DEFINITION: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_RD_DEFAULT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_RD_OFF: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_RD_ON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_REAL: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_REFRESH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_REMOTE_PWD: u32 = 1201u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_RESET_CONNECTION_YES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_RESET_PARAMS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_RESET_YES: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_RESTRICT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_RESULT_COL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_RETRIEVE_DATA: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_RETURN_VALUE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_RE_DEFAULT: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_RE_OFF: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_RE_ON: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ROLLBACK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ROWSET_SIZE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ROWSET_SIZE_DEFAULT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ROWVER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ROW_ADDED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ROW_DELETED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ROW_ERROR: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ROW_IDENTIFIER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ROW_IGNORE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ROW_NOROW: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ROW_NUMBER: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ROW_NUMBER_UNKNOWN: i32 = -2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ROW_PROCEED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ROW_SUCCESS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ROW_SUCCESS_WITH_INFO: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ROW_UPDATED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_ROW_UPDATES: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SCCO_LOCK: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SCCO_OPT_ROWVER: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SCCO_OPT_TIMESTAMP: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SCCO_OPT_VALUES: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SCCO_READ_ONLY: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SCC_ISO92_CLI: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SCC_XOPEN_CLI_VERSION1: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SCHEMA_TERM: u32 = 39u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SCHEMA_USAGE: u32 = 91u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SCOPE_CURROW: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SCOPE_SESSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SCOPE_TRANSACTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SCROLLABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SCROLL_CONCURRENCY: u32 = 43u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SCROLL_DYNAMIC: i32 = -2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SCROLL_FORWARD_ONLY: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SCROLL_KEYSET_DRIVEN: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SCROLL_OPTIONS: u32 = 44u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SCROLL_STATIC: i32 = -3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SC_FIPS127_2_TRANSITIONAL: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SC_NON_UNIQUE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SC_SQL92_ENTRY: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SC_SQL92_FULL: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SC_SQL92_INTERMEDIATE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SC_TRY_UNIQUE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SC_UNIQUE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SDF_CURRENT_DATE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SDF_CURRENT_TIME: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SDF_CURRENT_TIMESTAMP: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SEARCHABLE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SEARCH_PATTERN_ESCAPE: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SECOND: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SENSITIVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SERVER_NAME: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SETPARAM_VALUE_MAX: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SETPOS_MAX_LOCK_VALUE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SETPOS_MAX_OPTION_VALUE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SET_DEFAULT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SET_NULL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SFKD_CASCADE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SFKD_NO_ACTION: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SFKD_SET_DEFAULT: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SFKD_SET_NULL: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SFKU_CASCADE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SFKU_NO_ACTION: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SFKU_SET_DEFAULT: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SFKU_SET_NULL: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SG_DELETE_TABLE: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SG_INSERT_COLUMN: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SG_INSERT_TABLE: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SG_REFERENCES_COLUMN: i32 = 512i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SG_REFERENCES_TABLE: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SG_SELECT_TABLE: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SG_UPDATE_COLUMN: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SG_UPDATE_TABLE: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SG_USAGE_ON_CHARACTER_SET: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SG_USAGE_ON_COLLATION: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SG_USAGE_ON_DOMAIN: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SG_USAGE_ON_TRANSLATION: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SG_WITH_GRANT_OPTION: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SIGNED_OFFSET: i32 = -20i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SIMULATE_CURSOR: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SMALLINT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SNVF_BIT_LENGTH: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SNVF_CHARACTER_LENGTH: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SNVF_CHAR_LENGTH: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SNVF_EXTRACT: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SNVF_OCTET_LENGTH: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SNVF_POSITION: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SOPT_SS_BASE: u32 = 1225u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SOPT_SS_CURRENT_COMMAND: u32 = 1226u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SOPT_SS_CURSOR_OPTIONS: u32 = 1230u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SOPT_SS_DEFER_PREPARE: u32 = 1232u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SOPT_SS_HIDDEN_COLUMNS: u32 = 1227u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SOPT_SS_MAX_USED: u32 = 1232u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SOPT_SS_NOBROWSETABLE: u32 = 1228u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SOPT_SS_NOCOUNT_STATUS: u32 = 1231u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SOPT_SS_REGIONALIZE: u32 = 1229u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SOPT_SS_TEXTPTR_LOGGING: u32 = 1225u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SO_DYNAMIC: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SO_FORWARD_ONLY: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SO_KEYSET_DRIVEN: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SO_MIXED: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SO_STATIC: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SPECIAL_CHARACTERS: u32 = 94u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SPEC_MAJOR: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SPEC_MINOR: u32 = 80u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SPEC_STRING: &str = "03.80";
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SP_BETWEEN: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SP_COMPARISON: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SP_EXISTS: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SP_IN: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SP_ISNOTNULL: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SP_ISNULL: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SP_LIKE: i32 = 512i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SP_MATCH_FULL: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SP_MATCH_PARTIAL: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SP_MATCH_UNIQUE_FULL: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SP_MATCH_UNIQUE_PARTIAL: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SP_OVERLAPS: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SP_QUANTIFIED_COMPARISON: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SP_UNIQUE: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SQL92_DATETIME_FUNCTIONS: u32 = 155u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SQL92_FOREIGN_KEY_DELETE_RULE: u32 = 156u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SQL92_FOREIGN_KEY_UPDATE_RULE: u32 = 157u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SQL92_GRANT: u32 = 158u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SQL92_NUMERIC_VALUE_FUNCTIONS: u32 = 159u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SQL92_PREDICATES: u32 = 160u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SQL92_RELATIONAL_JOIN_OPERATORS: u32 = 161u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SQL92_REVOKE: u32 = 162u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SQL92_ROW_VALUE_CONSTRUCTOR: u32 = 163u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SQL92_STRING_FUNCTIONS: u32 = 164u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SQL92_VALUE_EXPRESSIONS: u32 = 165u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SQLSTATE_SIZE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SQLSTATE_SIZEW: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SQL_CONFORMANCE: u32 = 118u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SQ_COMPARISON: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SQ_CORRELATED_SUBQUERIES: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SQ_EXISTS: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SQ_IN: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SQ_QUANTIFIED: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SRJO_CORRESPONDING_CLAUSE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SRJO_CROSS_JOIN: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SRJO_EXCEPT_JOIN: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SRJO_FULL_OUTER_JOIN: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SRJO_INNER_JOIN: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SRJO_INTERSECT_JOIN: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SRJO_LEFT_OUTER_JOIN: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SRJO_NATURAL_JOIN: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SRJO_RIGHT_OUTER_JOIN: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SRJO_UNION_JOIN: i32 = 512i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SRVC_DEFAULT: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SRVC_NULL: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SRVC_ROW_SUBQUERY: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SRVC_VALUE_EXPRESSION: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SR_CASCADE: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SR_DELETE_TABLE: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SR_GRANT_OPTION_FOR: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SR_INSERT_COLUMN: i32 = 512i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SR_INSERT_TABLE: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SR_REFERENCES_COLUMN: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SR_REFERENCES_TABLE: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SR_RESTRICT: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SR_SELECT_TABLE: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SR_UPDATE_COLUMN: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SR_UPDATE_TABLE: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SR_USAGE_ON_CHARACTER_SET: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SR_USAGE_ON_COLLATION: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SR_USAGE_ON_DOMAIN: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SR_USAGE_ON_TRANSLATION: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SSF_CONVERT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SSF_LOWER: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SSF_SUBSTRING: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SSF_TRANSLATE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SSF_TRIM_BOTH: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SSF_TRIM_LEADING: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SSF_TRIM_TRAILING: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SSF_UPPER: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SS_ADDITIONS: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SS_DELETIONS: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SS_DL_DEFAULT: &str = "STATS.LOG";
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SS_QI_DEFAULT: u32 = 30000u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SS_QL_DEFAULT: &str = "QUERY.LOG";
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SS_UPDATES: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SS_VARIANT: i32 = -150i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_STANDARD_CLI_CONFORMANCE: u32 = 166u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_STATIC_CURSOR_ATTRIBUTES1: u32 = 167u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_STATIC_CURSOR_ATTRIBUTES2: u32 = 168u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_STATIC_SENSITIVITY: u32 = 83u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_STILL_EXECUTING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_STMT_OPT_MAX: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_STMT_OPT_MIN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_STRING_FUNCTIONS: u32 = 50u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SUBQUERIES: u32 = 95u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SUCCESS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SUCCESS_WITH_INFO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SU_DML_STATEMENTS: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SU_INDEX_DEFINITION: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SU_PRIVILEGE_DEFINITION: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SU_PROCEDURE_INVOCATION: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SU_TABLE_DEFINITION: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SVE_CASE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SVE_CAST: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SVE_COALESCE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SVE_NULLIF: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_SYSTEM_FUNCTIONS: u32 = 51u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TABLE_STAT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TABLE_TERM: u32 = 45u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TC_ALL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TC_DDL_COMMIT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TC_DDL_IGNORE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TC_DML: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TC_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TEXTPTR_LOGGING: u32 = 1225u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TIME: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TIMEDATE_ADD_INTERVALS: u32 = 109u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TIMEDATE_DIFF_INTERVALS: u32 = 110u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TIMEDATE_FUNCTIONS: u32 = 52u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TIMESTAMP: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TIMESTAMP_LEN: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TIME_LEN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TINYINT: i32 = -6i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TL_DEFAULT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TL_OFF: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TL_ON: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TRANSACTION_CAPABLE: u32 = 46u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TRANSACTION_ISOLATION_OPTION: u32 = 72u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TRANSACTION_READ_COMMITTED: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TRANSACTION_READ_UNCOMMITTED: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TRANSACTION_REPEATABLE_READ: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TRANSACTION_SERIALIZABLE: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TRANSLATE_DLL: u32 = 106u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TRANSLATE_OPTION: u32 = 107u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TRUE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TXN_CAPABLE: u32 = 46u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TXN_ISOLATION: u32 = 108u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TXN_ISOLATION_OPTION: u32 = 72u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TXN_READ_COMMITTED: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TXN_READ_UNCOMMITTED: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TXN_REPEATABLE_READ: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TXN_SERIALIZABLE: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TXN_VERSIONING: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TYPE_DATE: u32 = 91u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TYPE_DRIVER_END: i32 = -97i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TYPE_DRIVER_START: i32 = -80i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TYPE_MAX: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TYPE_MIN: i32 = -7i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TYPE_NULL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TYPE_TIME: u32 = 92u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_TYPE_TIMESTAMP: u32 = 93u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_UB_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_UB_FIXED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_UB_OFF: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_UB_ON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_UB_VARIABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_UNBIND: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_UNICODE: i32 = -95i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_UNICODE_CHAR: i32 = -95i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_UNICODE_LONGVARCHAR: i32 = -97i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_UNICODE_VARCHAR: i32 = -96i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_UNION: u32 = 96u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_UNION_STATEMENT: u32 = 96u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_UNKNOWN_TYPE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_UNNAMED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_UNSEARCHABLE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_UNSIGNED_OFFSET: i32 = -22i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_UNSPECIFIED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_UPDATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_UPDATE_BY_BOOKMARK: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_UP_DEFAULT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_UP_OFF: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_UP_ON: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_UP_ON_DROP: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_USER_NAME: u32 = 47u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_USE_BOOKMARKS: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_USE_PROCEDURE_FOR_PREPARE: u32 = 1202u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_US_UNION: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_US_UNION_ALL: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_U_UNION: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_U_UNION_ALL: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_VARBINARY: i32 = -3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_VARCHAR: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_VARLEN_DATA: i32 = -10i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_WARN_NO: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_WARN_YES: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_WCHAR: i32 = -8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_WLONGVARCHAR: i32 = -10i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_WVARCHAR: i32 = -9i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_XL_DEFAULT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_XL_OFF: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_XL_ON: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_XOPEN_CLI_YEAR: u32 = 10000u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_YEAR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQL_YEAR_TO_MONTH: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLudtBINARY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLudtBIT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLudtBITN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLudtCHAR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLudtDATETIM4: u32 = 22u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLudtDATETIME: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLudtDATETIMN: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLudtDECML: u32 = 24u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLudtDECMLN: u32 = 26u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLudtFLT4: u32 = 23u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLudtFLT8: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLudtFLTN: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLudtIMAGE: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLudtINT1: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLudtINT2: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLudtINT4: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLudtINTN: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLudtMONEY: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLudtMONEY4: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLudtMONEYN: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLudtNUM: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLudtNUMN: u32 = 25u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLudtSYSNAME: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLudtTEXT: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLudtTIMESTAMP: u32 = 80u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLudtUNIQUEIDENTIFIER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLudtVARBINARY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQLudtVARCHAR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SRCH_SCHEMA_CACHE_E_UNEXPECTED: i32 = -2147208447i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROPVAL_COMMANDTYPE_BULKLOAD: u32 = 22u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROPVAL_COMMANDTYPE_REGULAR: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROPVAL_USEPROCFORPREP_OFF: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROPVAL_USEPROCFORPREP_ON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROPVAL_USEPROCFORPREP_ON_DROP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROP_ALLOWNATIVEVARIANT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROP_AUTH_REPL_SERVER_NAME: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROP_CHARACTERSET: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROP_COLUMNLEVELCOLLATION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROP_COL_COLLATIONNAME: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROP_CURRENTCOLLATION: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROP_CURSORAUTOFETCH: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROP_DEFERPREPARE: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROP_ENABLEFASTLOAD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROP_FASTLOADKEEPIDENTITY: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROP_FASTLOADKEEPNULLS: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROP_FASTLOADOPTIONS: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROP_INIT_APPNAME: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROP_INIT_AUTOTRANSLATE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROP_INIT_CURRENTLANGUAGE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROP_INIT_ENCRYPT: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROP_INIT_FILENAME: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROP_INIT_NETWORKADDRESS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROP_INIT_NETWORKLIBRARY: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROP_INIT_PACKETSIZE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROP_INIT_TAGCOLUMNCOLLATION: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROP_INIT_USEPROCFORPREP: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROP_INIT_WSID: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROP_IRowsetFastLoad: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROP_MAXBLOBLENGTH: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROP_QUOTEDCATALOGNAMES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROP_SORTORDER: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROP_SQLXMLXPROGID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROP_STREAM_BASEPATH: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROP_STREAM_COMMANDTYPE: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROP_STREAM_CONTENTTYPE: u32 = 23u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROP_STREAM_FLAGS: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROP_STREAM_MAPPINGSCHEMA: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROP_STREAM_XMLROOT: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROP_STREAM_XSL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROP_UNICODECOMPARISONSTYLE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SSPROP_UNICODELCID: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct SSVARIANT {
    pub vt: u16,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub Anonymous: SSVARIANT_0,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for SSVARIANT {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for SSVARIANT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub union SSVARIANT_0 {
    pub bTinyIntVal: u8,
    pub sShortIntVal: i16,
    pub lIntVal: i32,
    pub llBigIntVal: i64,
    pub fltRealVal: f32,
    pub dblFloatVal: f64,
    pub cyMoneyVal: super::Com::CY,
    pub NCharVal: SSVARIANT_0_3,
    pub CharVal: SSVARIANT_0_2,
    pub fBitVal: i16,
    pub rgbGuidVal: [u8; 16],
    pub numNumericVal: DB_NUMERIC,
    pub BinaryVal: SSVARIANT_0_1,
    pub tsDateTimeVal: DBTIMESTAMP,
    pub UnknownType: SSVARIANT_0_4,
    pub BLOBType: SSVARIANT_0_0,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for SSVARIANT_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for SSVARIANT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct SSVARIANT_0_0 {
    pub dbobj: DBOBJECT,
    pub pUnk: *mut *mut *mut *mut ::windows_sys::core::IUnknown,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for SSVARIANT_0_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for SSVARIANT_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct SSVARIANT_0_1 {
    pub sActualLength: i16,
    pub sMaxLength: i16,
    pub prgbBinaryVal: *mut u8,
    pub dwReserved: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for SSVARIANT_0_1 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for SSVARIANT_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct SSVARIANT_0_2 {
    pub sActualLength: i16,
    pub sMaxLength: i16,
    pub pchCharVal: ::windows_sys::core::PSTR,
    pub rgbReserved: [u8; 5],
    pub dwReserved: u32,
    pub pwchReserved: ::windows_sys::core::PWSTR,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for SSVARIANT_0_2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for SSVARIANT_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct SSVARIANT_0_3 {
    pub sActualLength: i16,
    pub sMaxLength: i16,
    pub pwchNCharVal: ::windows_sys::core::PWSTR,
    pub rgbReserved: [u8; 5],
    pub dwReserved: u32,
    pub pwchReserved: ::windows_sys::core::PWSTR,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for SSVARIANT_0_3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for SSVARIANT_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct SSVARIANT_0_4 {
    pub dwActualLength: u32,
    pub rgMetadata: [u8; 16],
    pub pUnknownData: *mut u8,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for SSVARIANT_0_4 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for SSVARIANT_0_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const STD_BOOKMARKLENGTH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const STGM_COLLECTION: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const STGM_OPEN: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const STGM_OUTPUT: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const STGM_RECURSIVE: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const STGM_STRICTOPEN: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const STREAM_FLAGS_DISALLOW_ABSOLUTE_PATH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const STREAM_FLAGS_DISALLOW_QUERY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const STREAM_FLAGS_DISALLOW_UPDATEGRAMS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const STREAM_FLAGS_DISALLOW_URL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const STREAM_FLAGS_DONTCACHEMAPPINGSCHEMA: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const STREAM_FLAGS_DONTCACHETEMPLATE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const STREAM_FLAGS_DONTCACHEXSL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const STREAM_FLAGS_RESERVED: u32 = 4294901760u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type STRUCTURED_QUERY_MULTIOPTION = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQMO_VIRTUAL_PROPERTY: STRUCTURED_QUERY_MULTIOPTION = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQMO_DEFAULT_PROPERTY: STRUCTURED_QUERY_MULTIOPTION = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQMO_GENERATOR_FOR_TYPE: STRUCTURED_QUERY_MULTIOPTION = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQMO_MAP_PROPERTY: STRUCTURED_QUERY_MULTIOPTION = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type STRUCTURED_QUERY_PARSE_ERROR = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQPE_NONE: STRUCTURED_QUERY_PARSE_ERROR = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQPE_EXTRA_OPENING_PARENTHESIS: STRUCTURED_QUERY_PARSE_ERROR = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQPE_EXTRA_CLOSING_PARENTHESIS: STRUCTURED_QUERY_PARSE_ERROR = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQPE_IGNORED_MODIFIER: STRUCTURED_QUERY_PARSE_ERROR = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQPE_IGNORED_CONNECTOR: STRUCTURED_QUERY_PARSE_ERROR = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQPE_IGNORED_KEYWORD: STRUCTURED_QUERY_PARSE_ERROR = 5i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQPE_UNHANDLED: STRUCTURED_QUERY_PARSE_ERROR = 6i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type STRUCTURED_QUERY_RESOLVE_OPTION = u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQRO_DEFAULT: STRUCTURED_QUERY_RESOLVE_OPTION = 0u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQRO_DONT_RESOLVE_DATETIME: STRUCTURED_QUERY_RESOLVE_OPTION = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQRO_ALWAYS_ONE_INTERVAL: STRUCTURED_QUERY_RESOLVE_OPTION = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQRO_DONT_SIMPLIFY_CONDITION_TREES: STRUCTURED_QUERY_RESOLVE_OPTION = 4u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQRO_DONT_MAP_RELATIONS: STRUCTURED_QUERY_RESOLVE_OPTION = 8u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQRO_DONT_RESOLVE_RANGES: STRUCTURED_QUERY_RESOLVE_OPTION = 16u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQRO_DONT_REMOVE_UNRESTRICTED_KEYWORDS: STRUCTURED_QUERY_RESOLVE_OPTION = 32u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQRO_DONT_SPLIT_WORDS: STRUCTURED_QUERY_RESOLVE_OPTION = 64u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQRO_IGNORE_PHRASE_ORDER: STRUCTURED_QUERY_RESOLVE_OPTION = 128u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQRO_ADD_VALUE_TYPE_FOR_PLAIN_VALUES: STRUCTURED_QUERY_RESOLVE_OPTION = 256u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQRO_ADD_ROBUST_ITEM_NAME: STRUCTURED_QUERY_RESOLVE_OPTION = 512u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type STRUCTURED_QUERY_SINGLE_OPTION = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQSO_SCHEMA: STRUCTURED_QUERY_SINGLE_OPTION = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQSO_LOCALE_WORD_BREAKING: STRUCTURED_QUERY_SINGLE_OPTION = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQSO_WORD_BREAKER: STRUCTURED_QUERY_SINGLE_OPTION = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQSO_NATURAL_SYNTAX: STRUCTURED_QUERY_SINGLE_OPTION = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQSO_AUTOMATIC_WILDCARD: STRUCTURED_QUERY_SINGLE_OPTION = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQSO_TRACE_LEVEL: STRUCTURED_QUERY_SINGLE_OPTION = 5i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQSO_LANGUAGE_KEYWORDS: STRUCTURED_QUERY_SINGLE_OPTION = 6i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQSO_SYNTAX: STRUCTURED_QUERY_SINGLE_OPTION = 7i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQSO_TIME_ZONE: STRUCTURED_QUERY_SINGLE_OPTION = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQSO_IMPLICIT_CONNECTOR: STRUCTURED_QUERY_SINGLE_OPTION = 9i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQSO_CONNECTOR_CASE: STRUCTURED_QUERY_SINGLE_OPTION = 10i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type STRUCTURED_QUERY_SYNTAX = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQS_NO_SYNTAX: STRUCTURED_QUERY_SYNTAX = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQS_ADVANCED_QUERY_SYNTAX: STRUCTURED_QUERY_SYNTAX = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SQS_NATURAL_QUERY_SYNTAX: STRUCTURED_QUERY_SYNTAX = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const STS_ABORTXMLPARSE: i32 = -2147211756i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const STS_WS_ERROR: i32 = -2147211754i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SUBSCRIPTIONINFO {
    pub cbSize: u32,
    pub fUpdateFlags: u32,
    pub schedule: SUBSCRIPTIONSCHEDULE,
    pub customGroupCookie: ::windows_sys::core::GUID,
    pub pTrigger: *mut ::core::ffi::c_void,
    pub dwRecurseLevels: u32,
    pub fWebcrawlerFlags: u32,
    pub bMailNotification: super::super::Foundation::BOOL,
    pub bGleam: super::super::Foundation::BOOL,
    pub bChangesOnly: super::super::Foundation::BOOL,
    pub bNeedPassword: super::super::Foundation::BOOL,
    pub fChannelFlags: u32,
    pub bstrUserName: super::super::Foundation::BSTR,
    pub bstrPassword: super::super::Foundation::BSTR,
    pub bstrFriendlyName: super::super::Foundation::BSTR,
    pub dwMaxSizeKB: u32,
    pub subType: SUBSCRIPTIONTYPE,
    pub fTaskFlags: u32,
    pub dwReserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SUBSCRIPTIONINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SUBSCRIPTIONINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type SUBSCRIPTIONINFOFLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SUBSINFO_SCHEDULE: SUBSCRIPTIONINFOFLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SUBSINFO_RECURSE: SUBSCRIPTIONINFOFLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SUBSINFO_WEBCRAWL: SUBSCRIPTIONINFOFLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SUBSINFO_MAILNOT: SUBSCRIPTIONINFOFLAGS = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SUBSINFO_MAXSIZEKB: SUBSCRIPTIONINFOFLAGS = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SUBSINFO_USER: SUBSCRIPTIONINFOFLAGS = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SUBSINFO_PASSWORD: SUBSCRIPTIONINFOFLAGS = 64i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SUBSINFO_TASKFLAGS: SUBSCRIPTIONINFOFLAGS = 256i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SUBSINFO_GLEAM: SUBSCRIPTIONINFOFLAGS = 512i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SUBSINFO_CHANGESONLY: SUBSCRIPTIONINFOFLAGS = 1024i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SUBSINFO_CHANNELFLAGS: SUBSCRIPTIONINFOFLAGS = 2048i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SUBSINFO_FRIENDLYNAME: SUBSCRIPTIONINFOFLAGS = 8192i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SUBSINFO_NEEDPASSWORD: SUBSCRIPTIONINFOFLAGS = 16384i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SUBSINFO_TYPE: SUBSCRIPTIONINFOFLAGS = 32768i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub struct SUBSCRIPTIONITEMINFO {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub dwPriority: u32,
    pub ScheduleGroup: ::windows_sys::core::GUID,
    pub clsidAgent: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for SUBSCRIPTIONITEMINFO {}
impl ::core::clone::Clone for SUBSCRIPTIONITEMINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type SUBSCRIPTIONSCHEDULE = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SUBSSCHED_AUTO: SUBSCRIPTIONSCHEDULE = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SUBSSCHED_DAILY: SUBSCRIPTIONSCHEDULE = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SUBSSCHED_WEEKLY: SUBSCRIPTIONSCHEDULE = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SUBSSCHED_CUSTOM: SUBSCRIPTIONSCHEDULE = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SUBSSCHED_MANUAL: SUBSCRIPTIONSCHEDULE = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type SUBSCRIPTIONTYPE = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SUBSTYPE_URL: SUBSCRIPTIONTYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SUBSTYPE_CHANNEL: SUBSCRIPTIONTYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SUBSTYPE_DESKTOPURL: SUBSCRIPTIONTYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SUBSTYPE_EXTERNAL: SUBSCRIPTIONTYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SUBSTYPE_DESKTOPCHANNEL: SUBSCRIPTIONTYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SUBSINFO_ALLFLAGS: u32 = 61311u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SUBSMGRENUM_MASK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SUBSMGRENUM_TEMP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SUBSMGRUPDATE_MASK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SUBSMGRUPDATE_MINIMIZE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SUCCEED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SUCCEED_ABORT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const SUCCEED_ASYNC: u32 = 3u32;
pub const SubscriptionMgr: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2881368528, data2: 28078, data3: 4560, data4: [190, 202, 0, 192, 79, 217, 64, 190] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub struct TEXT_SOURCE {
    pub pfnFillTextBuffer: PFNFILLTEXTBUFFER,
    pub awcBuffer: ::windows_sys::core::PCWSTR,
    pub iEnd: u32,
    pub iCur: u32,
}
impl ::core::marker::Copy for TEXT_SOURCE {}
impl ::core::clone::Clone for TEXT_SOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub struct TIMEOUT_INFO {
    pub dwSize: u32,
    pub dwConnectTimeout: u32,
    pub dwDataTimeout: u32,
}
impl ::core::marker::Copy for TIMEOUT_INFO {}
impl ::core::clone::Clone for TIMEOUT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub struct TIMESTAMP_STRUCT {
    pub year: i16,
    pub month: u16,
    pub day: u16,
    pub hour: u16,
    pub minute: u16,
    pub second: u16,
    pub fraction: u32,
}
impl ::core::marker::Copy for TIMESTAMP_STRUCT {}
impl ::core::clone::Clone for TIMESTAMP_STRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub struct TIME_STRUCT {
    pub hour: u16,
    pub minute: u16,
    pub second: u16,
}
impl ::core::marker::Copy for TIME_STRUCT {}
impl ::core::clone::Clone for TIME_STRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const TRACE_ON: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const TRACE_VERSION: u32 = 1000u32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const TRACE_VS_EVENT_ON: i32 = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
pub struct VECTORRESTRICTION {
    pub Node: NODERESTRICTION,
    pub RankMethod: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for VECTORRESTRICTION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for VECTORRESTRICTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub type WEBCRAWL_RECURSEFLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const WEBCRAWL_DONT_MAKE_STICKY: WEBCRAWL_RECURSEFLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const WEBCRAWL_GET_IMAGES: WEBCRAWL_RECURSEFLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const WEBCRAWL_GET_VIDEOS: WEBCRAWL_RECURSEFLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const WEBCRAWL_GET_BGSOUNDS: WEBCRAWL_RECURSEFLAGS = 8i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const WEBCRAWL_GET_CONTROLS: WEBCRAWL_RECURSEFLAGS = 16i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const WEBCRAWL_LINKS_ELSEWHERE: WEBCRAWL_RECURSEFLAGS = 32i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const WEBCRAWL_IGNORE_ROBOTSTXT: WEBCRAWL_RECURSEFLAGS = 128i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const WEBCRAWL_ONLY_LINKS_TO_HTML: WEBCRAWL_RECURSEFLAGS = 256i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const XML_E_BADSXQL: i32 = -2147212799i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const XML_E_NODEFAULTNS: i32 = -2147212800i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const _MAPI_E_ACCOUNT_DISABLED: i32 = -2147221212i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const _MAPI_E_BAD_CHARWIDTH: i32 = -2147221245i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const _MAPI_E_BAD_COLUMN: i32 = -2147221224i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const _MAPI_E_BUSY: i32 = -2147221237i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const _MAPI_E_COMPUTED: i32 = -2147221222i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const _MAPI_E_CORRUPT_DATA: i32 = -2147221221i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const _MAPI_E_DISK_ERROR: i32 = -2147221226i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const _MAPI_E_END_OF_SESSION: i32 = -2147220992i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const _MAPI_E_EXTENDED_ERROR: i32 = -2147221223i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const _MAPI_E_FAILONEPROVIDER: i32 = -2147221219i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const _MAPI_E_INVALID_ACCESS_TIME: i32 = -2147221213i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const _MAPI_E_INVALID_ENTRYID: i32 = -2147221241i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const _MAPI_E_INVALID_OBJECT: i32 = -2147221240i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const _MAPI_E_INVALID_WORKSTATION_ACCOUNT: i32 = -2147221214i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const _MAPI_E_LOGON_FAILED: i32 = -2147221231i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const _MAPI_E_MISSING_REQUIRED_COLUMN: i32 = -2147220990i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const _MAPI_E_NETWORK_ERROR: i32 = -2147221227i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const _MAPI_E_NOT_ENOUGH_DISK: i32 = -2147221235i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const _MAPI_E_NOT_ENOUGH_RESOURCES: i32 = -2147221234i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const _MAPI_E_NOT_FOUND: i32 = -2147221233i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const _MAPI_E_NO_SUPPORT: i32 = -2147221246i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const _MAPI_E_OBJECT_CHANGED: i32 = -2147221239i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const _MAPI_E_OBJECT_DELETED: i32 = -2147221238i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const _MAPI_E_PASSWORD_CHANGE_REQUIRED: i32 = -2147221216i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const _MAPI_E_PASSWORD_EXPIRED: i32 = -2147221215i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const _MAPI_E_SESSION_LIMIT: i32 = -2147221230i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const _MAPI_E_STRING_TOO_LONG: i32 = -2147221243i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const _MAPI_E_TOO_COMPLEX: i32 = -2147221225i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const _MAPI_E_UNABLE_TO_ABORT: i32 = -2147221228i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const _MAPI_E_UNCONFIGURED: i32 = -2147221220i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const _MAPI_E_UNKNOWN_CPID: i32 = -2147221218i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const _MAPI_E_UNKNOWN_ENTRYID: i32 = -2147220991i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const _MAPI_E_UNKNOWN_FLAGS: i32 = -2147221242i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const _MAPI_E_UNKNOWN_LCID: i32 = -2147221217i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const _MAPI_E_USER_CANCEL: i32 = -2147221229i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const _MAPI_E_VERSION: i32 = -2147221232i32;
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub const _MAPI_W_NO_SERVICE: i32 = 262659i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub struct dbdatetime {
    pub dtdays: i32,
    pub dttime: u32,
}
impl ::core::marker::Copy for dbdatetime {}
impl ::core::clone::Clone for dbdatetime {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub struct dbdatetime4 {
    pub numdays: u16,
    pub nummins: u16,
}
impl ::core::marker::Copy for dbdatetime4 {}
impl ::core::clone::Clone for dbdatetime4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub struct dbmoney {
    pub mnyhigh: i32,
    pub mnylow: u32,
}
impl ::core::marker::Copy for dbmoney {}
impl ::core::clone::Clone for dbmoney {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub struct dbvarybin {
    pub len: i16,
    pub array: [u8; 8001],
}
impl ::core::marker::Copy for dbvarybin {}
impl ::core::clone::Clone for dbvarybin {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub struct dbvarychar {
    pub len: i16,
    pub str: [i8; 8001],
}
impl ::core::marker::Copy for dbvarychar {}
impl ::core::clone::Clone for dbvarychar {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub struct sqlperf {
    pub TimerResolution: u32,
    pub SQLidu: u32,
    pub SQLiduRows: u32,
    pub SQLSelects: u32,
    pub SQLSelectRows: u32,
    pub Transactions: u32,
    pub SQLPrepares: u32,
    pub ExecDirects: u32,
    pub SQLExecutes: u32,
    pub CursorOpens: u32,
    pub CursorSize: u32,
    pub CursorUsed: u32,
    pub PercentCursorUsed: f64,
    pub AvgFetchTime: f64,
    pub AvgCursorSize: f64,
    pub AvgCursorUsed: f64,
    pub SQLFetchTime: u32,
    pub SQLFetchCount: u32,
    pub CurrentStmtCount: u32,
    pub MaxOpenStmt: u32,
    pub SumOpenStmt: u32,
    pub CurrentConnectionCount: u32,
    pub MaxConnectionsOpened: u32,
    pub SumConnectionsOpened: u32,
    pub SumConnectiontime: u32,
    pub AvgTimeOpened: f64,
    pub ServerRndTrips: u32,
    pub BuffersSent: u32,
    pub BuffersRec: u32,
    pub BytesSent: u32,
    pub BytesRec: u32,
    pub msExecutionTime: u32,
    pub msNetWorkServerTime: u32,
}
impl ::core::marker::Copy for sqlperf {}
impl ::core::clone::Clone for sqlperf {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct tagDBROWWATCHRANGE {
    pub hRegion: usize,
    pub eChangeKind: u32,
    pub hRow: usize,
    pub iRow: usize,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for tagDBROWWATCHRANGE {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for tagDBROWWATCHRANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
#[cfg(target_arch = "x86")]
pub struct tagDBROWWATCHRANGE {
    pub hRegion: usize,
    pub eChangeKind: u32,
    pub hRow: usize,
    pub iRow: usize,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for tagDBROWWATCHRANGE {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for tagDBROWWATCHRANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub struct tagSQL_DAY_SECOND {
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub fraction: u32,
}
impl ::core::marker::Copy for tagSQL_DAY_SECOND {}
impl ::core::clone::Clone for tagSQL_DAY_SECOND {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub struct tagSQL_YEAR_MONTH {
    pub year: u32,
    pub month: u32,
}
impl ::core::marker::Copy for tagSQL_YEAR_MONTH {}
impl ::core::clone::Clone for tagSQL_YEAR_MONTH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Search\"`*"]
pub struct tagSSErrorInfo {
    pub pwszMessage: ::windows_sys::core::PWSTR,
    pub pwszServer: ::windows_sys::core::PWSTR,
    pub pwszProcedure: ::windows_sys::core::PWSTR,
    pub lNative: i32,
    pub bState: u8,
    pub bClass: u8,
    pub wLineNumber: u16,
}
impl ::core::marker::Copy for tagSSErrorInfo {}
impl ::core::clone::Clone for tagSSErrorInfo {
    fn clone(&self) -> Self {
        *self
    }
}
